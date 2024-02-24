use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemStruct, Lit, Meta, NestedMeta};

#[proc_macro_attribute]
pub fn derive_path(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let mut path = String::new();
    // Parse named arguments
    for arg in args {
        if let NestedMeta::Meta(Meta::NameValue(nv)) = arg {
            if nv.path.is_ident("path") {
                if let Lit::Str(lit) = &nv.lit {
                    path = lit.value();
                }
            }
        }
    }



    let input = parse_macro_input!(input as ItemStruct);
    let struct_name = &input.ident;

    let expanded = quote! {
        impl path_resolver::path_trait::PathResolver for #struct_name {
            fn resolve_path(&self, vars_map: std::collections::HashMap<String,String>) -> String {
                let mut path = #path.to_string();
                let vars = path_resolver::path_trait::extract_values(&path);
                for var in &vars {
                    let var = var.to_string();
                    match vars_map.get(&var){
                        Some(value) => path = path.replace(&format!("{{{{ {} }}}}", var), value),
                        None => panic!("{} not found in vars_map {:?}", var, vars_map),
                    }
                }
                path
            }
        }
        #input
    };

    TokenStream::from(expanded)
}