#[cfg(not(target_arch = "wasm32"))]
mod native {
    use crate::{node::Node, Query};
    use std::{borrow::Cow, convert::TryFrom};

    #[derive(Clone)]
    pub struct QueryCapture<'a> {
        pub(crate) inner: tree_sitter::QueryCapture<'a>,
    }

    impl<'a> QueryCapture<'a> {
        #[inline]
        pub fn node(&self) -> Node {
            self.inner.node.into()
        }

        #[inline]
        pub fn utf8_name<'s>(&self, capture_names: &'s Vec<String>) -> Cow<'s, str> {
            let index: usize = self.inner.index as usize;
            Cow::Borrowed(capture_names[index].as_str())
        }

        #[inline]
        pub fn utf8_text<'s>(&self, capture_names: &'s Vec<String>) -> Option<Cow<'s, str>> {
            None
        }
    }

    impl<'a> From<&tree_sitter::QueryCapture<'a>> for QueryCapture<'a> {
        #[inline]
        fn from(inner: &tree_sitter::QueryCapture<'a>) -> Self {
            Self { inner: *inner }
        }
    }

    impl<'tree> From<tree_sitter::QueryCapture<'tree>> for QueryCapture<'tree> {
        #[inline]
        fn from(inner: tree_sitter::QueryCapture<'tree>) -> Self {
            Self { inner }
        }
    }

    impl<'a> std::panic::RefUnwindSafe for QueryCapture<'a> {}

    impl<'a> Unpin for QueryCapture<'a> {}

    impl<'a> std::panic::UnwindSafe for QueryCapture<'a> {}
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm {
    use crate::node::Node;
    use std::borrow::Cow;

    #[derive(Clone)]
    pub struct QueryCapture<'a> {
        pub(crate) inner: web_tree_sitter::QueryCapture,
        pub(crate) phantom: std::marker::PhantomData<&'a ()>,
    }

    impl<'a> QueryCapture<'a> {
        #[inline]
        pub fn node(&self) -> Node {
            self.inner.node().into()
        }

        #[inline]
        pub fn utf8_name<'s>(&self, capture_names: &'s Vec<String>) -> Cow<str> {
            Cow::Owned(self.inner.name().as_string().unwrap())
        }

        #[inline]
        pub fn utf8_text<'s>(&self, capture_names: &'s Vec<String>) -> Option<Cow<str>> {
            self.inner.text().map(|t| Cow::Owned(t.as_string().unwrap()))
        }
    }

    impl<'a> From<web_tree_sitter::QueryCapture> for QueryCapture<'a> {
        #[inline]
        fn from(inner: web_tree_sitter::QueryCapture) -> Self {
            let phantom = std::marker::PhantomData;
            Self { inner, phantom }
        }
    }

    impl<'a> std::panic::RefUnwindSafe for QueryCapture<'a> {}

    impl<'a> Unpin for QueryCapture<'a> {}

    impl<'a> std::panic::UnwindSafe for QueryCapture<'a> {}
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
