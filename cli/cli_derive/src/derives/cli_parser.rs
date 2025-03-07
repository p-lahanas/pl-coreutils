use std::{iter::Peekable, path::Iter};

use syn::DeriveInput;

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(args))]
struct CliParserStructAttributes {
    cmd_name: String,
    description: String,
}

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(args))]
struct CliParserFieldAttributes {
    #[deluxe(default = "argument".to_string())]
    arg_type: String,
    #[deluxe(default = "".to_string())]
    name: String,

    paramc: Option<u32>, // the number of tokens expected for this argument/option

    #[deluxe(default = "".to_string())]
    long_form: String,
    #[deluxe(default = "".to_string())]
    short_form: String,
    #[deluxe(default = "".to_string())]
    description: String,
}

#[derive(PartialEq)]
struct Argument {
    field: syn::Field, // name of the field in the struct
    arg_name: String,
    paramc: u32,
    description: String,
}

#[derive(PartialEq)]
struct OptionArg {
    field: syn::Field, // name of the field in the struct
    paramc: u32,
    long_form: String,
    short_form: String,
    description: String,
}
struct CliCommand {
    name: String,
    description: String,
    arguments: Vec<Argument>,
    options: Vec<OptionArg>,
}

fn extract_cli_parser_fields(
    ast: &mut DeriveInput,
) -> deluxe::Result<(Vec<Argument>, Vec<OptionArg>)> {
    let mut args = Vec::new();
    let mut opts = Vec::new();

    // Extract our field attributes from our input
    if let syn::Data::Struct(s) = &mut ast.data {
        for field in s.fields.iter_mut() {
            let attrs: CliParserFieldAttributes = deluxe::extract_attributes(field)?;
            if attrs.arg_type == "option" {
                opts.push(OptionArg {
                    field: field.clone(),
                    paramc: attrs.paramc.unwrap_or(0),
                    long_form: attrs.long_form,
                    short_form: attrs.short_form,
                    description: attrs.description,
                });
            } else {
                // Default is argument
                args.push(Argument {
                    field: field.clone(),
                    arg_name: attrs.name,
                    paramc: attrs.paramc.unwrap_or(1),
                    description: attrs.description,
                });
            }
        }
    }

    Ok((args, opts))
}

pub fn impl_cli_derive_macro(
    item: proc_macro2::TokenStream,
) -> deluxe::Result<proc_macro2::TokenStream> {
    let mut ast: DeriveInput = syn::parse2(item)?;

    // Populate the model for our command
    let cmd_attr: CliParserStructAttributes = deluxe::extract_attributes(&mut ast)?;
    let fields = extract_cli_parser_fields(&mut ast)?;

    let cmd = CliCommand {
        name: cmd_attr.cmd_name.clone(),
        description: cmd_attr.description.clone(),
        arguments: fields.0,
        options: fields.1,
    };

    // define implementation variables
    let ident = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    let usage_string = generate_usage_string(&cmd);

    // let parse_code = generate_parse_function(&cmd.arguments, &cmd.options);

    // generate
    Ok(quote::quote! {
        impl #impl_generics cli::CliParser for #ident #type_generics #where_clause {
            fn usage() {
                std::println!("{}", Self::get_usage())
            }
            fn get_usage() -> &'static str {
                #usage_string
            }
            // fn parse() -> Self {
            //     #parse_code
            // }
        }
    })
}

fn parse_optionarg(
    arg_in: &String,
    cmd: &CliCommand,
    parsed_options: &mut std::collections::HashMap<String, String>,
    args_in: &mut Peekable<std::env::ArgsOs>,
) {
    let mut option_arg: Option<&OptionArg> = None;
    // Find which optionarg we are referencing
    for opt in &cmd.options {
        if opt.short_form == *arg_in || opt.long_form == *arg_in {
            option_arg = Some(opt);
        }
    }
    if option_arg == None {
        panic!("Invalid option {}", arg_in);
    }
}

fn parse_tokens(cmd: &CliCommand) {
    let parsed_args: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let parsed_options: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();

    let args = cmd.arguments.iter().peekable();
    let mut args_in = std::env::args_os().skip(1).peekable();

    while let Some(arg_in) = args_in.next() {
        let arg_in = arg_in.to_string_lossy().into_owned();
        // Check for option
        if arg_in.starts_with("--") {
            // Check if it's
        }
    }

    // while Some(in_arg) = in_args.next() {
    //     if in_arg.starts_with("--") {
    //         let key = arg.trim_start_matches("--").to_string();
    //         if let Some(next) = args.peek() {
    //             if !next.starts_with("--") {
    //                 named_args.insert(key.clone(), args.next().unwrap().to_string());
    //             } else {
    //                 flags.insert(key);
    //             }
    //         } else {
    //             flags.insert(key);
    //         }
    //     } else {
    //         positional_args.push(arg.to_string());
    //     }
    // }
}

fn generate_usage_string(cmd: &CliCommand) -> String {
    let description = &cmd.description;
    let cmd_name = &cmd.name;

    let args_list: String = cmd
        .arguments
        .iter()
        .map(|f| f.arg_name.clone())
        .collect::<Vec<String>>()
        .join(" ");

    let args_desc: String = cmd
        .arguments
        .iter()
        .map(|f| format!("{}\t{}", f.arg_name.clone(), f.description.clone()))
        .collect::<Vec<String>>()
        .join("\n");

    let option_desc: String = cmd
        .options
        .iter()
        .map(|f| {
            format!(
                "{}, {}\t{}",
                f.short_form.clone(),
                f.long_form.clone(),
                f.description.clone()
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        "{description}\n\nUsage: {cmd_name} [options] {args_list}\n\nArguments:\n{args_desc}\n\nOptions:\n{option_desc}"
    )
}
