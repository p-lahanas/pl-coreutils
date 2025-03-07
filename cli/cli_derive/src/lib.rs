#![forbid(unsafe_code)]

mod derives;

#[proc_macro_derive(CliParser, attributes(args))]
pub fn cli_derive_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Build the CliParser implementation
    derives::impl_cli_derive_macro(item.into()).unwrap().into()
}

// TODO:
// 1. Check the number of args is what we expect (positional, named, flag)
// 2. Add support for both positional and named arguments
// 3. Turn positional or named into it's own type rather than just comparing it to a string?
// 4. Allow for args which there can be multiple e.g. arg_name... https://stackoverflow.com/questions/9725675/is-there-a-standard-format-for-command-line-shell-help-text
// 5. Create my own custom data type ArgType to distinguish between an argument and option more effectively.
