# Syn builder
Builder functions for `syn` structures and enums to ease the generation of Rust code.

Note: only syn structures are used, there no intermediate structs created.

## Usage
1. Add to your Cargo.toml file
```toml
[dependencies]
syn_builder = "0.1.0"
```

2. Import builder functions and create syn objects
```rust
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn_builder::*;

fn main() {
    //generate code using the builder api
    let code = item_enum("my_enum").variants([
        variant("A").fields(
            fields_unamed([field(type_path("A")), field(type_path("B"))]),
        ),
        variant("B"),
        variant("C").fields(
            fields_named([
                field(type_path("A")).ident("other"),
                field(type_path("B")).ident("one"),
            ]),
        ),
    ]);

    let mut token_stream = TokenStream::new();
    code.to_tokens(&mut token_stream);

    println!("{token_stream}");
}
```

## Alternatives
- **[quote](https://github.com/dtolnay/quote)** - generate syn structs by writing Rust code and using variable interpolation
