use darling::FromDeriveInput;
use proc_macro::TokenStream;

#[derive(darling::FromDeriveInput, Debug)]
#[darling(supports(struct_named))]
struct ConfigInputReceiver {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<(), ConfigFieldReceiver>,
}

#[derive(darling::FromField, Debug)]
struct ConfigFieldReceiver {
    ident: Option<syn::Ident>,
}

pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let ConfigInputReceiver {
        ident,
        generics,
        data,
        ..
    } = match ConfigInputReceiver::from_derive_input(&derive_input) {
        Ok(receiver) => receiver,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let fields = data
        .take_struct()
        .expect("Should always be struct with named fields")
        .fields;

    let field = fields.iter().filter_map(|f| f.ident.as_ref());

    (quote::quote! {
        impl #generics config::Source for #ident #generics {
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

        impl #generics From<#ident #generics> for config::ValueKind {
            fn from(value: #ident #generics) -> Self {
                use config::Source;

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
