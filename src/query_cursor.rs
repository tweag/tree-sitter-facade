#[cfg(not(target_arch = "wasm32"))]
mod native {
    pub struct QueryCursor {
        pub(crate) inner: tree_sitter::QueryCursor,
    }

    impl QueryCursor {
        #[inline]
        pub fn new() -> Self {
            let inner = tree_sitter::QueryCursor::new();
            Self { inner }
        }

        // #[inline]
        // pub fn matches<'a, 'tree: 'a>(
        //     &'a mut self,
        //     query: &'a Query,
        //     node: Node<'tree>,
        //     source: &'a [u8],
        // ) -> Vec<QueryMatch> {
        //     let matches = self
        //         .inner
        //         .matches(&query.inner, node.inner, source)
        //         .map(Into::into)
        //         .collect();

        //     matches
        // }
    }

    impl Default for QueryCursor {
        fn default() -> Self {
            Self::new()
        }
    }

    impl From<tree_sitter::QueryCursor> for QueryCursor {
        #[inline]
        fn from(inner: tree_sitter::QueryCursor) -> Self {
            Self { inner }
        }
    }

    impl std::panic::RefUnwindSafe for QueryCursor {}

    impl Unpin for QueryCursor {}

    impl std::panic::UnwindSafe for QueryCursor {}
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm {
    #[derive(Clone)]
    pub struct QueryCursor {}

    impl QueryCursor {
        #[inline]
        pub fn new() -> Self {
            Self {}
        }

        // #[inline]
        // pub fn matches<'a, 'tree: 'a>(
        //     &'a mut self,
        //     query: &'a Query,
        //     node: Node<'tree>,
        //     source: &'a [u8],
        // ) -> Vec<QueryMatch> {
        //     let matches = self
        //         .inner
        //         .matches(&query.inner, node.inner, source)
        //         .into_iter()
        //         .map(Into::into)
        //         .collect();

        //     matches
        // }
    }

    // impl From<web_tree_sitter::QueryCursor> for QueryCursor {
    //     #[inline]
    //     fn from(inner: web_tree_sitter::QueryCursor) -> Self {
    //         Self { inner }
    //     }
    // }

    impl std::panic::RefUnwindSafe for QueryCursor {}

    impl Unpin for QueryCursor {}

    impl std::panic::UnwindSafe for QueryCursor {}
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
