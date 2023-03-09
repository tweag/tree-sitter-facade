#[cfg(not(target_arch = "wasm32"))]
mod native {
    use crate::{
        error::QueryError, language::Language, node::Node, query_cursor::QueryCursor, query_match::QueryMatch,
        query_predicate::QueryPredicate,
    };

    pub struct Query {
        pub inner: tree_sitter::Query,
        // pub cursor: tree_sitter::QueryCursor,
    }

    impl Query {
        #[inline]
        pub fn new(language: &Language, source: &str) -> Result<Self, QueryError> {
            let inner = tree_sitter::Query::new(language.inner, source)?;
            // let cursor = tree_sitter::QueryCursor::new();
            Ok(Self { inner })
        }

        #[inline]
        pub fn matches<'a, 'tree: 'a>(
            &'a self,
            node: Node<'tree>,
            source: &'a [u8],
            cursor: &'a mut QueryCursor,
        ) -> impl Iterator<Item = QueryMatch<'a>> + 'a {
            cursor.inner.matches(&self.inner, node.inner, source).map(Into::into)
        }

        #[inline]
        pub fn capture_names(&self) -> Vec<String> {
            let names: Vec<_> = self.inner.capture_names().iter().cloned().collect();
            names
        }

        #[inline]
        pub fn general_predicates(&self, index: u32) -> Vec<QueryPredicate> {
            let index = index as usize;
            self.inner.general_predicates(index).iter().map(Into::into).collect()
        }
    }

    impl std::fmt::Debug for Query {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            std::fmt::Debug::fmt(&self.inner, fmt)
        }
    }

    impl From<tree_sitter::Query> for Query {
        #[inline]
        fn from(inner: tree_sitter::Query) -> Self {
            // let cursor = tree_sitter::QueryCursor::new();
            Self { inner }
        }
    }

    impl std::panic::RefUnwindSafe for Query {}

    unsafe impl Send for Query {}

    unsafe impl Sync for Query {}

    impl Unpin for Query {}

    impl std::panic::UnwindSafe for Query {}
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm {
    use crate::{
        error::QueryError, language::Language, node::Node, query_cursor::QueryCursor, query_match::QueryMatch,
        query_predicate::QueryPredicate,
    };
    use wasm_bindgen::JsCast;

    pub struct Query {
        pub(crate) inner: web_tree_sitter::Query,
    }

    impl Query {
        #[inline]
        pub fn new(language: &Language, source: &str) -> Result<Self, QueryError> {
            let inner = language.inner.query(&source.into())?;
            Ok(Self { inner })
        }

        #[inline]
        pub fn matches<'a, 'tree: 'a>(
            &'a self,
            node: Node<'tree>,
            _source: &'a [u8],
            _cursor: &'a mut QueryCursor,
        ) -> impl ExactSizeIterator<Item = QueryMatch<'tree>> + 'tree {
            self.inner
                .matches(&node.inner, None, None)
                .into_vec()
                .into_iter()
                .map(|value| {
                    let _s = format!("Query match: {:?}", value);
                    //web_sys::console::log_1(&_s.into());

                    value.unchecked_into::<web_tree_sitter::QueryMatch>().into()
                })
        }

        #[inline]
        pub fn capture_names(&self) -> Vec<String> {
            vec![]
        }

        #[inline]
        pub fn general_predicates(&self, index: u32) -> Vec<QueryPredicate> {
            let predicates: Vec<_> = self
                .inner
                .predicates_for_pattern(index)
                .into_vec()
                .into_iter()
                .map(|value| {
                    let _s = format!("Query predicate: {:?}", value);
                    //web_sys::console::log_1(&_s.into());

                    value.unchecked_into::<web_tree_sitter::QueryPredicate>().into()
                })
                .collect();

            predicates
        }
    }

    impl std::fmt::Debug for Query {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            std::fmt::Debug::fmt(&self.inner, fmt)
        }
    }

    impl Drop for Query {
        #[inline]
        fn drop(&mut self) {
            self.inner.delete();
        }
    }

    impl From<web_tree_sitter::Query> for Query {
        #[inline]
        fn from(inner: web_tree_sitter::Query) -> Self {
            Self { inner }
        }
    }

    impl std::panic::RefUnwindSafe for Query {}

    unsafe impl Send for Query {}

    unsafe impl Sync for Query {}

    impl Unpin for Query {}

    impl std::panic::UnwindSafe for Query {}
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
