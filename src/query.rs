#[cfg(not(target_arch = "wasm32"))]
mod native {
    use crate::{
        error::QueryError, language::Language, node::Node, query_capture::QueryCapture, query_match::QueryMatch,
    };

    pub struct Query {
        pub inner: tree_sitter::Query,
        pub cursor: tree_sitter::QueryCursor,
    }

    impl Query {
        #[inline]
        pub fn new(language: &Language, source: &str) -> Result<Self, QueryError> {
            let inner = tree_sitter::Query::new(language.inner, source)?;
            let mut cursor = tree_sitter::QueryCursor::new();
            Ok(Self { inner, cursor })
        }

        #[inline]
        pub fn matches<'a, 'tree: 'a>(
            &'a mut self,
            node: Node<'tree>,
            source: &'a [u8],
        ) -> impl Iterator<Item = QueryMatch<'a>> + 'a {
            // for m in self.cursor.matches(&self.inner, node.inner, source) {
            //     let a = m;
            //     let b = a;
            // }

            // let raw_matches: Vec<_> = self.cursor.matches(&self.inner, node.inner, source).collect();

            self.cursor.matches(&self.inner, node.inner, source).map(Into::into)
        }

        #[inline]
        pub fn capture_names(&self) -> Vec<String> {
            let names: Vec<_> = self.inner.capture_names().iter().map(|s| s.clone()).collect();
            names
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
            let mut cursor = tree_sitter::QueryCursor::new();
            Self { inner, cursor }
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
    use crate::{error::QueryError, language::Language, node::Node, query_match::QueryMatch};
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
            &'a mut self,
            node: Node<'tree>,
            source: &'a [u8],
        ) -> impl ExactSizeIterator<Item = QueryMatch<'tree>> + 'tree {
            self.inner
                .matches(&node.inner, None, None)
                .into_vec()
                .into_iter()
                .map(|value| {
                    let s = format!("Query match: {:?}", value);
                    //web_sys::console::log_1(&s.into());

                    value.unchecked_into::<web_tree_sitter::QueryMatch>().into()
                })
        }

        #[inline]
        pub fn capture_names(&self) -> Vec<String> {
            vec![]
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
