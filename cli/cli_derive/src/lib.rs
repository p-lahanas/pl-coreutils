use std::collections::HashMap;

use syn::DeriveInput;

// TODO:
// 1. Check the number of args is what we expect (positional, named, flag)
// 2. Add support for both positional and named arguments
// 3. Turn positional or named into it's own type rather than just comparing it to a string?

// enum ArgType {
//     Positional,
//     Named,
// }

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(args))]
struct CliParserFieldAttributes {
    arg_type: String,
}

fn extract_cli_parser_field_attributes(
    ast: &mut DeriveInput,
) -> deluxe::Result<HashMap<String, CliParserFieldAttributes>> {
    let mut field_attrs = HashMap::new();

    // Extact our field attributes from our input
    if let syn::Data::Struct(s) = &mut ast.data {
        for field in s.fields.iter_mut() {
            let field_name = field.ident.as_ref().unwrap().to_string();
            let attrs: CliParserFieldAttributes = deluxe::extract_attributes(field)?;
            field_attrs.insert(field_name, attrs);
        }
    }

    Ok(field_attrs)
}

fn impl_cli_derive_macro(
    item: proc_macro2::TokenStream,
) -> deluxe::Result<proc_macro2::TokenStream> {
    // parse
    let mut ast: DeriveInput = syn::parse2(item)?;

    // extract field attributes
    let field_attrs: HashMap<String, CliParserFieldAttributes> =
        extract_cli_parser_field_attributes(&mut ast)?;

    // define implementation variables
    let ident = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    let positional_fields: Vec<_> = field_attrs
        .iter()
        .filter(|(_, attrs)| attrs.arg_type == "positional")
        .collect();
    let named_fields: Vec<_> = field_attrs
        .iter()
        .filter(|(_, attrs)| attrs.arg_type == "named")
        .collect();

    let positional_usage = positional_fields
        .iter()
        .map(|(f, a)| format!("<{}>", f))
        .collect::<Vec<_>>()
        .join(" ");

    let named_usage = named_fields
        .iter()
        .map(|(f, _)| {
            let field_name = f;
            format!("[--{} {}]", field_name, field_name)
        })
        .collect::<Vec<_>>()
        .join(" ");

    let usage_string = format!(
        "{} usage: {} {}",
        ident.to_string(),
        positional_usage,
        named_usage
    );

    // generate
    Ok(quote::quote! {
        impl #impl_generics CliParser for #ident #type_generics #where_clause {
            fn usage() -> &'static str {
                #usage_string
            }
        }
    })
}

#[proc_macro_derive(CliParser, attributes(args))]
pub fn cli_derive_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Build the CliParser implementation
    impl_cli_derive_macro(item.into()).unwrap().into()
}
