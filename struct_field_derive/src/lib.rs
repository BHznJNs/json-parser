use proc_macro::TokenStream;

#[proc_macro_derive(ListedFieldNames)]
pub fn resolve_struct_field_names(input: TokenStream) -> TokenStream {
    fn field_ident_resolver(field: &syn::Field) -> &'static str {
        let field_string = field.ident.as_ref().unwrap().to_string();
        let leaked: &mut str = Box::leak(field_string.into_boxed_str());
        return leaked;
    }

    let ast = syn::parse::<syn::ItemStruct>(input).expect("failed to parse input");
    let field_names = ast.fields
        .iter()
        .map(field_ident_resolver)
        .collect::<Vec<&'static str>>();

    let type_name = &ast.ident;
    let output_impl = quote::quote! {
        impl ListedFieldNames for #type_name {
            fn fields() -> Vec<&'static str> {
                return vec![#(#field_names),*];
            }
        }
    };
    return output_impl.into();
}
