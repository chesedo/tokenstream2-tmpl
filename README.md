# tokenstream2-tmpl

[![github]](https://github.com/chesedo/tokenstream2-tmpl)&ensp;[![crates-io]](https://crates.io/crates/tokenstream2-tmpl)&ensp;[![docs-rs]](https://docs.rs/tokenstream2-tmpl)&ensp;[![workflow]](https://github.com/chesedo/tokenstream2-tmpl/actions?query=workflow%3ARust)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
[workflow]: https://img.shields.io/github/workflow/status/chesedo/tokenstream2-tmpl/Rust?color=green&label=&labelColor=555555&logo=github%20actions&logoColor=white&style=for-the-badge

This crate is meant to be a complement to [quote]. Where as [quote] does quasi-quote interpolations at
compile-time, this crate does them at run-time. This is handy for macros receiving templates from client code with
markers to be replaced when the macro is run.

[quote]: https://github.com/dtolnay/quote

## Examples
```rust
use proc_macro2::TokenStream;
use tokenstream2-tmpl::interpolate;
use quote::ToTokens;
use std::collections::HashMap;
use syn::{Ident, parse_str};

let input: TokenStream = parse_str("let NAME: int = 5;")?;
let expected: TokenStream = parse_str("let age: int = 5;")?;

let mut replacements: HashMap<&str, &dyn ToTokens> = HashMap::new();
let ident = parse_str::<Ident>("age")?;
replacements.insert("NAME", &ident);

let output = interpolate(input, &replacements);
assert_eq!(
    format!("{}", output),
    format!("{}", expected)
);

```

Here `input` might be some input to a macro that functions as a template. [quote] would have tried to expand `NAME`
at the macro's compile-time. [tokenstream2-tmpl] will expand it at the macro's run-time.

[tokenstream2-tmpl]: https://gitlab.com/chesedo/tokenstream2-tmpl

```rust
extern crate proc_macro;
use proc_macro2::TokenStream;
use std::collections::HashMap;
use syn::{Ident, parse::{Parse, ParseStream, Result}, parse_macro_input, punctuated::Punctuated, Token};
use tokenstream2-tmpl::{Interpolate, interpolate};
use quote::ToTokens;

/// Create a token for macro using [syn](syn)
/// Type that holds a key and the value it maps to.
/// An acceptable stream will have the following form:
/// ```text
/// key => value
/// ```
struct KeyValue {
    pub key: Ident,
    pub arrow_token: Token![=>],
    pub value: Ident,
}

/// Make KeyValue parsable from a token stream
impl Parse for KeyValue {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(KeyValue {
            key: input.parse()?,
            arrow_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

/// Make KeyValue interpolatible
impl Interpolate for KeyValue {
    fn interpolate(&self, stream: TokenStream) -> TokenStream {
        let mut replacements: HashMap<_, &dyn ToTokens> = HashMap::new();

        // Replace each "KEY" with the key
        replacements.insert("KEY", &self.key);

        // Replace each "VALUE" with the value
        replacements.insert("VALUE", &self.value);

        interpolate(stream, &replacements)
    }
}

/// Macro to take a list of key-values with a template to expand each key-value
#[proc_macro_attribute]
pub fn map(tokens: proc_macro::TokenStream, template: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse a comma separated list of key-values
    let maps =
        parse_macro_input!(tokens with Punctuated::<KeyValue, Token![,]>::parse_terminated);

    maps.interpolate(template.into()).into()
}

pub fn main() {
    #[map(
        usize => 10,
        isize => -2,
        bool => false,
    )]
    let _: KEY = VALUE;
    // Output:
    // let _: usize = 10;
    // let _: isize = -2;
    // let _: bool = false;
}
```

License: MIT
