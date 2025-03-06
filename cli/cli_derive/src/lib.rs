use std::collections::HashMap;

use syn::DeriveInput;

// TODO:
// 1. Check the number of args is what we expect (positional, named, flag)
// 2. Add support for both positional and named arguments
// 3. Turn positional or named into it's own type rather than just comparing it to a string?
// 4. Allow for args which there can be multiple e.g. arg_name... https://stackoverflow.com/questions/9725675/is-there-a-standard-format-for-command-line-shell-help-text
// 5. Create my own custom data type ArgType to distinguish between an argument and option more effectively.

// enum ArgType {
//     Argument {
//         name: String,
//         description: String,
//     },
//     Optional {
//         long_form: String,
//         short_form: String,
//         description: String,
//     },
// }

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(args))]
struct CliParserStructAttributes {
    prog_name: String,
    description: String,
}

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(args))]
struct CliParserFieldAttributes {
    #[deluxe(default = "argument".to_string())]
    arg_type: String,
    #[deluxe(default = "".to_string())]
    name: String,

    #[deluxe(default = "".to_string())]
    long_form: String,
    #[deluxe(default = "".to_string())]
    short_form: String,
    #[deluxe(default = "".to_string())]
    description: String,
}

fn extract_cli_parser_field_attributes(
    ast: &mut DeriveInput,
) -> deluxe::Result<HashMap<String, CliParserFieldAttributes>> {
    let mut field_attrs = HashMap::new();

    // Extract our field attributes from our input
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

    // get our struct attributes
    let CliParserStructAttributes {
        prog_name,
        description,
    } = deluxe::extract_attributes(&mut ast)?;

    // extract field attributes
    let field_attrs: HashMap<String, CliParserFieldAttributes> =
        extract_cli_parser_field_attributes(&mut ast)?;

    let mut arguments: HashMap<String, CliParserFieldAttributes> = HashMap::new();
    let mut options: HashMap<String, CliParserFieldAttributes> = HashMap::new();

    for (field, attr) in field_attrs {
        if attr.arg_type == "argument" {
            arguments.insert(field, attr); // Add to arguments map
        } else {
            options.insert(field, attr); // Add to options map
        }
    }

    // define implementation variables
    let ident = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    let args_list: String = arguments
        .iter()
        .map(|(_, attr)| attr.name.clone()) // Map to the name field
        .collect::<Vec<String>>()
        .join(" "); // Join the names with spaces
    let args_desc: String = arguments
        .iter()
        .map(|(_, attr)| format!("{} {}", attr.name.clone(), attr.description.clone())) // Map to the name field
        .collect::<Vec<String>>()
        .join("\n"); // Join the names with spaces

    let option_desc: String = options
        .iter()
        .map(|(_, attr)| {
            format!(
                "{}, {}\t{}",
                attr.short_form.clone(),
                attr.long_form.clone(),
                attr.description.clone()
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let usage_string = format!(
        "Usage: {prog_name} [options] {args_list}\n\n{description}\n\nArguments:\n{args_desc}\n\nOptions:\n{option_desc}"
    );

    // generate
    Ok(quote::quote! {
        impl #impl_generics CliParser for #ident #type_generics #where_clause {
            fn get_usage() -> &'static str {
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
