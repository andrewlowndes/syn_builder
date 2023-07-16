use proc_macro2::TokenStream;
use quote::ToTokens;
use syn_builder::*;

fn main() {
    //generate code using the builder api
    let code = item_enum("my_enum").variants([
        variant("A").fields(fields_unamed([
            field(type_path("A")),
            field(type_path("B")),
        ])),
        variant("B"),
        variant("C").fields(fields_named([
            field(type_path("A")).ident("other"),
            field(type_path("B")).ident("one"),
        ])),
    ]);

    let mut token_stream = TokenStream::new();
    code.to_tokens(&mut token_stream);

    println!("{token_stream}");
}
