//! [![github]](https://github.com/chesedo/tokenstream2-tmpl)&ensp;[![crates-io]](https://crates.io/crates/tokenstream2-tmpl)&ensp;[![docs-rs]](https://docs.rs/tokenstream2-tmpl)&ensp;[![workflow]](https://github.com/chesedo/tokenstream2-tmpl/actions?query=workflow%3ARust)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//! [workflow]: https://img.shields.io/github/workflow/status/chesedo/tokenstream2-tmpl/Rust?color=green&label=&labelColor=555555&logo=github%20actions&logoColor=white&style=for-the-badge
//!
//! This crate is meant to be a complement to [quote]. Where as [quote] does quasi-quote interpolations at
//! compile-time, this crate does them at run-time. This is handy for macros receiving templates from client code with
//! markers to be replaced when the macro is run.
//!
//! [quote]: https://github.com/dtolnay/quote
//!
//! # Examples
//! ```
//! use proc_macro2::TokenStream;
//! use tokenstream2_tmpl::interpolate;
//! use quote::ToTokens;
//! use std::collections::HashMap;
//! use syn::{Ident, parse_str};
//!
//! let input: TokenStream = parse_str("let NAME: int = 5;")?;
//! let expected: TokenStream = parse_str("let age: int = 5;")?;
//!
//! let mut replacements: HashMap<&str, &dyn ToTokens> = HashMap::new();
//! let ident = parse_str::<Ident>("age")?;
//! replacements.insert("NAME", &ident);
//!
//! let output = interpolate(input, &replacements);
//! assert_eq!(
//!     format!("{}", output),
//!     format!("{}", expected)
//! );
//!
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! Here `input` might be some input to a macro that functions as a template. [quote] would have tried to expand `NAME`
//! at the macro's compile-time. [tokenstream2-tmpl] will expand it at the macro's run-time.
//!
//! [tokenstream2-tmpl]: https://gitlab.com/chesedo/tokenstream2-tmpl
//!
//! ```
//! extern crate proc_macro;
//! use proc_macro2::TokenStream;
//! use std::collections::HashMap;
//! use syn::{Ident, parse::{Parse, ParseStream, Result}, parse_macro_input, punctuated::Punctuated, Token};
//! use tokenstream2_tmpl::{Interpolate, interpolate};
//! use quote::ToTokens;
//!
//! /// Create a token for macro using [syn](syn)
//! /// Type that holds a key and the value it maps to.
//! /// An acceptable stream will have the following form:
//! /// ```text
//! /// key => value
//! /// ```
//! struct KeyValue {
//!     pub key: Ident,
//!     pub arrow_token: Token![=>],
//!     pub value: Ident,
//! }
//!
//! /// Make KeyValue parsable from a token stream
//! impl Parse for KeyValue {
//!     fn parse(input: ParseStream) -> Result<Self> {
//!         Ok(KeyValue {
//!             key: input.parse()?,
//!             arrow_token: input.parse()?,
//!             value: input.parse()?,
//!         })
//!     }
//! }
//!
//! /// Make KeyValue interpolatible
//! impl Interpolate for KeyValue {
//!     fn interpolate(&self, stream: TokenStream) -> TokenStream {
//!         let mut replacements: HashMap<_, &dyn ToTokens> = HashMap::new();
//!
//!         // Replace each "KEY" with the key
//!         replacements.insert("KEY", &self.key);
//!
//!         // Replace each "VALUE" with the value
//!         replacements.insert("VALUE", &self.value);
//!
//!         interpolate(stream, &replacements)
//!     }
//! }
//!
//! /// Macro to take a list of key-values with a template to expand each key-value
//! # const IGNORE: &str = stringify! {
//! #[proc_macro_attribute]
//! # };
//! pub fn map(tokens: proc_macro::TokenStream, template: proc_macro::TokenStream) -> proc_macro::TokenStream {
//!     // Parse a comma separated list of key-values
//!     let maps =
//!         parse_macro_input!(tokens with Punctuated::<KeyValue, Token![,]>::parse_terminated);
//!
//!     maps.interpolate(template.into()).into()
//! }
//!
//! pub fn main() {
//! # const IGNORE: &str = stringify! {
//!     #[map(
//!         usize => 10,
//!         isize => -2,
//!         bool => false,
//!     )]
//!     let _: KEY = VALUE;
//! # };
//!     // Output:
//!     // let _: usize = 10;
//!     // let _: isize = -2;
//!     // let _: bool = false;
//! }
//! ```

use proc_macro2::{Group, TokenStream, TokenTree};
use quote::{ToTokens, TokenStreamExt};
use std::collections::HashMap;
use syn::punctuated::Punctuated;

/// Trait for tokens that can replace interpolation markers
pub trait Interpolate {
    /// Take a token stream and replace interpolation markers with their actual values into a new stream
    /// using [interpolate](interpolate)
    fn interpolate(&self, stream: TokenStream) -> TokenStream;
}

/// Make a Punctuated list interpolatible if it holds interpolatible types
impl<T: Interpolate, P> Interpolate for Punctuated<T, P> {
    fn interpolate(&self, stream: TokenStream) -> TokenStream {
        self.iter()
            .fold(TokenStream::new(), |mut implementations, t| {
                implementations.extend(t.interpolate(stream.clone()));
                implementations
            })
    }
}

/// Replace the interpolation markers in a token stream with a specific text.
/// See this [crate's](crate) documentation for an example on how to use this.
pub fn interpolate(
    stream: TokenStream,
    replacements: &HashMap<&str, &dyn ToTokens>,
) -> TokenStream {
    let mut new = TokenStream::new();

    // Loop over each token in the stream
    // `Literal`, `Punct`, and `Group` are kept as is
    for token in stream.into_iter() {
        match token {
            TokenTree::Literal(literal) => new.append(literal),
            TokenTree::Punct(punct) => new.append(punct),
            TokenTree::Group(group) => {
                // Recursively interpolate the stream in group
                let mut new_group =
                    Group::new(group.delimiter(), interpolate(group.stream(), replacements));
                new_group.set_span(group.span());

                new.append(new_group);
            }
            TokenTree::Ident(ident) => {
                let ident_str: &str = &ident.to_string();

                // Check if identifier is in the replacement set
                if let Some(value) = replacements.get(ident_str) {
                    // Replace with replacement value
                    value.to_tokens(&mut new);

                    continue;
                }

                // Identifier did not match, so copy as is
                new.append(ident);
            }
        }
    }

    new
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use quote::quote;
    use syn::{parse_str, Ident, Token, Type};

    type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn complete_replacements() -> Result {
        let input = quote! {
            let VAR: TRAIT = if true {
                CONCRETE{}
            } else {
                Alternative{}
            }
        };

        let expected = quote! {
            let var: abstract_type = if true {
                concrete{}
            } else {
                Alternative{}
            }
        };

        let mut r: HashMap<&str, &dyn ToTokens> = HashMap::new();
        let v: Ident = parse_str("var")?;
        let a: Type = parse_str("abstract_type")?;
        let c: Type = parse_str("concrete")?;

        r.insert("VAR", &v);
        r.insert("TRAIT", &a);
        r.insert("CONCRETE", &c);

        assert_eq!(
            format!("{}", &interpolate(input, &r)),
            format!("{}", expected)
        );

        Ok(())
    }

    /// Partial replacements should preverse the uninterpolated identifiers
    #[test]
    fn partial_replacements() -> Result {
        let input: TokenStream = parse_str("let a: TRAIT = OTHER;")?;
        let expected: TokenStream = parse_str("let a: Display = OTHER;")?;

        let mut r: HashMap<&str, &dyn ToTokens> = HashMap::new();
        let t: Type = parse_str("Display")?;
        r.insert("TRAIT", &t);

        assert_eq!(
            format!("{}", interpolate(input, &r)),
            format!("{}", expected)
        );

        Ok(())
    }

    /// Test the interpolation of Punctuated items
    #[test]
    fn interpolate_on_punctuated() -> Result {
        #[allow(dead_code)]
        pub struct TraitSpecifier {
            pub abstract_trait: Type,
            pub arrow_token: Token![=>],
            pub concrete: Type,
        }

        /// Make TraitSpecifier interpolatible
        impl Interpolate for TraitSpecifier {
            fn interpolate(&self, stream: TokenStream) -> TokenStream {
                let mut replacements: HashMap<_, &dyn ToTokens> = HashMap::new();

                // Replace each "TRAIT" with the absract trait
                replacements.insert("TRAIT", &self.abstract_trait);

                // Replace each "CONCRETE" with the concrete type
                replacements.insert("CONCRETE", &self.concrete);

                interpolate(stream, &replacements)
            }
        }
        let mut traits: Punctuated<TraitSpecifier, Token![,]> = Punctuated::new();

        traits.push(TraitSpecifier {
            abstract_trait: parse_str("IButton")?,
            arrow_token: Default::default(),
            concrete: parse_str("BigButton")?,
        });
        traits.push(TraitSpecifier {
            abstract_trait: parse_str("IWindow")?,
            arrow_token: Default::default(),
            concrete: parse_str("MinimalWindow")?,
        });

        let input = quote! {
            let _: TRAIT = CONCRETE{};
        };
        let expected = quote! {
            let _: IButton = BigButton{};
            let _: IWindow = MinimalWindow{};
        };

        assert_eq!(
            format!("{}", traits.interpolate(input)),
            format!("{}", expected)
        );

        Ok(())
    }
}
