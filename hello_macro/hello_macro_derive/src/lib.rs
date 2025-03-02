use proc_macro::TokenStream;
use quote::quote;
// syn - разбирает код на rust в структуру данных которую мы можем менять
// пример структуры разобранного кода:
// DeriveInput {
//     // --snip--
//     ident: Ident {
//         ident: "Pancakes",
//         span: #0 bytes(95..103)
//     },
//     data: Struct(
//         DataStruct {
//             struct_token: Struct,
//             fields: Unit,
//             semi_token: Some(
//                 Semi
//             )
//         }
//     )
// }

// quote - собирает обратно из структуры даннх в код на Rust
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

// Еще один типа макроса - атрибутный. Он используется самостоятельно и отдельно от derive
// может использоваться не только со структурами и перечислениями но и с функциями либо другими типами
// attr - аттрибут который прикрепляется, item - итем к которому прикрепляется аттрибут (на примере это функция)

// пример макроса:
// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}

// пример вызова:
// #[route(GET, "/")]
// fn index() {}


// третий тип макросов - похожие на функции
// можно объединить только с синтаксисом подобного сопоставлению
// пример макроса:
// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {}

// пример вызова:
// let sql = sql!(SELECT * FROM posts WHERE id=1);
