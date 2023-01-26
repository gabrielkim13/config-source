use proc_macro::TokenStream;

pub fn config_source(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let field = match &derive_input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fields),
            ..
        }) => fields.named.iter().map(|field| &field.ident),
        _ => panic!("Expected a struct with named fields"),
    };

    let struct_name = &derive_input.ident;

    (quote::quote! {
        use config::Source;

        impl config::Source for #struct_name {
            fn clone_into_box(&self) -> Box<dyn config::Source + Send + Sync> {
                Box::new((*self).clone())
            }

            fn collect(
                &self,
            ) -> Result<std::collections::HashMap<String, config::Value>, config::ConfigError> {
                Ok(std::collections::HashMap::from([
                    #(
                        (
                            stringify!(#field).to_string(),
                            config::Value::new(None, self.#field.clone()),
                        ),
                    )*
                ]))
            }
        }

        impl From<#struct_name> for config::ValueKind {
            fn from(value: #struct_name) -> Self {
                if let Ok(table) = value.collect() {
                    config::ValueKind::Table(table)
                } else {
                    unreachable!()
                }
            }
        }
    })
    .into()
}
