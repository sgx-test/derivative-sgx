//! Contains some standard paths.
use std::prelude::v1::*;
/// Return the path of the `discriminant` function, that is `::std::mem::discriminant`.
pub fn discriminant_path() -> syn::Path {
    if cfg!(feature = "use_core") {
        parse_quote!(::core::mem::discriminant)
    } else {
        parse_quote!(::std::mem::discriminant)
    }
}