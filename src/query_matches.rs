// #[cfg(not(target_arch = "wasm32"))]
// mod native {
//     use crate::{node::Node, Query, QueryMatch};
//     use std::{borrow::Cow, convert::TryFrom};

//     pub struct QueryMatches<'a, 'tree> {
//         pub(crate) inner: tree_sitter::QueryMatches<'a, 'tree, &'a [u8]>,
//     }

//     impl<'a, 'tree> Iterator for QueryMatches<'a, 'tree> {
//         type Item = QueryMatch<'a, 'tree>;

//         fn next(&mut self) -> Option<Self::Item> {
//             self.inner.next().map(Into::into)
//         }
//     }

//     impl<'a, 'tree> QueryMatches<'a, 'tree> {}

//     impl<'a, 'tree> From<tree_sitter::QueryMatches<'a, 'tree, &'a [u8]>> for QueryMatches<'a, 'tree> {
//         #[inline]
//         fn from(inner: tree_sitter::QueryMatches<'a, 'tree, &'a [u8]>) -> Self {
//             Self { inner }
//         }
//     }

//     impl<'a, 'tree> std::panic::RefUnwindSafe for QueryMatches<'a, 'tree> {}

//     impl<'a, 'tree> Unpin for QueryMatches<'a, 'tree> {}

//     impl<'a, 'tree> std::panic::UnwindSafe for QueryMatches<'a, 'tree> {}
// }

// #[cfg(not(target_arch = "wasm32"))]
// pub use native::*;

// #[cfg(target_arch = "wasm32")]
// mod wasm {
//     use crate::node::Node;
//     use std::borrow::Cow;

//     #[derive(Clone)]
//     pub struct QueryMatches {
//         pub(crate) inner: Vec<web_tree_sitter::QueryMatch>,
//     }

//     impl QueryMatches {
//         #[inline]
//         pub fn field_id(&self) -> Option<u16> {
//             self.inner.current_field_id()
//         }

//         #[inline]
//         pub fn field_name(&self) -> Option<Cow<str>> {
//             self.inner
//                 .current_field_name()
//                 .map(|name| From::<String>::from(name.into()))
//         }

//         #[inline]
//         pub fn goto_first_child(&mut self) -> bool {
//             self.inner.goto_first_child()
//         }

//         #[inline]
//         pub fn goto_next_sibling(&mut self) -> bool {
//             self.inner.goto_next_sibling()
//         }

//         #[inline]
//         pub fn goto_parent(&mut self) -> bool {
//             self.inner.goto_parent()
//         }

//         #[inline]
//         pub fn node(&self) -> Node<'a> {
//             let inner = self.inner.current_node();
//             let phantom = std::marker::PhantomData;
//             Node { inner, phantom }
//         }

//         #[inline]
//         pub fn reset(&mut self, node: Node<'a>) {
//             self.inner.reset(&node.inner);
//         }
//     }

//     impl<'a, 'tree> From<web_tree_sitter::QueryMatches<'a, 'tree, &'a [u8]>> for QueryMatches<'a, 'tree> {
//         #[inline]
//         fn from(inner: web_tree_sitter::QueryMatches<'a, 'tree, &'a [u8]>) -> Self {
//             Self { inner }
//         }
//     }

//     impl<'a, 'tree> std::panic::RefUnwindSafe for QueryMatches<'a, 'tree> {}

//     impl<'a, 'tree> Unpin for QueryMatches<'a, 'tree> {}

//     impl<'a, 'tree> std::panic::UnwindSafe for QueryMatches {}
// }

// #[cfg(target_arch = "wasm32")]
// pub use wasm::*;
