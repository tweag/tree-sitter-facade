#[cfg(not(target_arch = "wasm32"))]
mod native {
    use crate::{node::Node, query::Query, query_capture::QueryCapture};
    use std::{borrow::Cow, convert::TryFrom};

    pub struct QueryMatch<'tree> {
        pub(crate) inner: tree_sitter::QueryMatch<'tree, 'tree>,
    }

    impl<'tree> QueryMatch<'tree> {
        #[inline]
        pub fn pattern_index(&self) -> usize {
            self.inner.pattern_index
        }

        #[inline]
        pub fn captures(&self) -> impl ExactSizeIterator<Item = QueryCapture<'tree>> {
            self.inner.captures.iter().map(Into::into)
        }
    }

    impl<'tree> From<tree_sitter::QueryMatch<'tree, 'tree>> for QueryMatch<'tree> {
        #[inline]
        fn from(inner: tree_sitter::QueryMatch<'tree, 'tree>) -> Self {
            Self { inner }
        }
    }

    impl<'tree> std::panic::RefUnwindSafe for QueryMatch<'tree> {}

    impl<'tree> Unpin for QueryMatch<'tree> {}

    impl<'tree> std::panic::UnwindSafe for QueryMatch<'tree> {}
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm {
    use crate::{node::Node, query::Query, query_capture::QueryCapture};
    use std::borrow::Cow;
    use wasm_bindgen::JsCast;

    #[derive(Clone)]
    pub struct QueryMatch<'tree> {
        pub(crate) inner: web_tree_sitter::QueryMatch,
        pub(crate) phantom: std::marker::PhantomData<&'tree ()>,
    }

    impl<'tree> QueryMatch<'tree> {
        #[inline]
        pub fn pattern_index(&self) -> usize {
            self.inner.pattern()
        }

        #[inline]
        pub fn captures(&self) -> impl ExactSizeIterator<Item = QueryCapture<'tree>> + 'tree {
            self.inner.captures().into_vec().into_iter().map(|value| {
                let s = format!("Query match capture: {:?}", value);
                //web_sys::console::log_1(&s.into());
                value.unchecked_into::<web_tree_sitter::QueryCapture>().into()
            })
        }
    }

    impl<'tree> From<web_tree_sitter::QueryMatch> for QueryMatch<'tree> {
        #[inline]
        fn from(inner: web_tree_sitter::QueryMatch) -> Self {
            let phantom = std::marker::PhantomData;
            Self { inner, phantom }
        }
    }

    impl<'tree> std::panic::RefUnwindSafe for QueryMatch<'tree> {}

    impl<'tree> Unpin for QueryMatch<'tree> {}

    impl<'tree> std::panic::UnwindSafe for QueryMatch<'tree> {}
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
