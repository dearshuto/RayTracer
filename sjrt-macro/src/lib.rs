use proc_macro::TokenStream;
use quote::quote;
use syn::{ext::IdentExt, parse_macro_input, DeriveInput};

#[proc_macro_derive(Immutable)]
pub fn immutable_device(input: TokenStream) -> TokenStream {
    // 構造体じゃなければエラー
    let input = &parse_macro_input!(input as DeriveInput);
    let syn::Data::Struct(struct_data) = &input.data else {
        // TODO
        panic!();
    };

    let mut with_fields = Vec::default();
    for (target_index, target_field) in struct_data.fields.iter().enumerate() {
        let mut init_fields = Vec::default();

        for (with_index, with) in struct_data.fields.iter().enumerate() {
            if target_index == with_index {
                continue;
            }

            // Self {
            //    value: self.value // ⇦ ここを生成
            // }
            let name_info = with.ident.as_ref().unwrap();
            init_fields.push(quote! {
            #name_info: self.#name_info,
                });
        }

        // 関数本体の生成
        // fieldの型情報
        let ty = &target_field.ty;
        let name_info = target_field.ident.as_ref().unwrap();

        // with_name のようにフィールド名に with_ をつけた名称を生成
        let generated_method_name: proc_macro2::TokenStream =
            format!("with_{}", name_info.unraw().to_string())
                .parse()
                .unwrap();
        with_fields.push(quote! {
            pub fn #generated_method_name(self, value: #ty) -> Self {
                Self {
                    #name_info: value,
                    #(#init_fields)*
                }
            }
        });
    }

    // 構造体名
    let struct_name = &input.ident;

    // generics, where句の情報
    let (impl_generics, _, where_clause) = &input.generics.split_for_impl();

    let expanded = quote! {
    impl #impl_generics #struct_name #impl_generics #where_clause  {
        #(#with_fields)*
    }
     };

    TokenStream::from(expanded)
}
