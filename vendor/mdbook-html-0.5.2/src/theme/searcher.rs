//! Theme dependencies for in-browser search. Not included in mdbook when
//! the "search" cargo feature is disabled.

pub(crate) static JS: &[u8] = include_bytes!("../../front-end/searcher/searcher.js");
