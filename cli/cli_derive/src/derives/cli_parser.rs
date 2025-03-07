use std::collections::HashMap;

use syn::DeriveInput;

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

struct CliCommand {
    cmd_attrs: CliParserStructAttributes,
    arg_attrs: HashMap<String, CliParserFieldAttributes>,
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

pub fn impl_cli_derive_macro(
    item: proc_macro2::TokenStream,
) -> deluxe::Result<proc_macro2::TokenStream> {
    let mut ast: DeriveInput = syn::parse2(item)?;

    // Populate the model for our command
    let cmd = CliCommand {
        cmd_attrs: deluxe::extract_attributes(&mut ast)?,
        arg_attrs: extract_cli_parser_field_attributes(&mut ast)?,
    };

    // define implementation variables
    let ident = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    let usage_string = generate_usage_string(&cmd);

    // generate
    Ok(quote::quote! {
        impl #impl_generics cli::CliParser for #ident #type_generics #where_clause {
            fn usage() {
                std::println!("{}", Self::get_usage())
            }
            fn get_usage() -> &'static str {
                #usage_string
            }
        }
    })
}

fn generate_usage_string(cmd: &CliCommand) -> String {
    let description = &cmd.cmd_attrs.description;
    let prog_name = &cmd.cmd_attrs.prog_name;

    let args_list: String = cmd
        .arg_attrs
        .iter()
        .map(|(_, attr)| {
            if attr.arg_type == "argument" {
                attr.name.clone()
            } else {
                "".to_string()
            }
        }) // Map to the name field
        .collect::<Vec<String>>()
        .join(" "); // Join the names with spaces
    let args_desc: String = cmd
        .arg_attrs
        .iter()
        .map(|(_, attr)| {
            if attr.arg_type == "argument" {
                format!("{} {}", attr.name.clone(), attr.description.clone())
            } else {
                "".to_string()
            }
        }) // Map to the name field
        .collect::<Vec<String>>()
        .join("\n"); // Join the names with spaces

    let option_desc: String = cmd
        .arg_attrs
        .iter()
        .map(|(_, attr)| {
            if attr.arg_type == "optional" {
                format!(
                    "{}, {}\t{}",
                    attr.short_form.clone(),
                    attr.long_form.clone(),
                    attr.description.clone()
                )
            } else {
                "".to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        "{description}\n\nUsage: {prog_name} [options] {args_list}\n\nArguments:\n{args_desc}\n\nOptions:\n{option_desc}"
    )
}
