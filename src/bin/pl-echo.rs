use cli::CliParser;

#[derive(CliParser)]
#[args(
    prog_name = "pl-echo",
    description = "Display a line of text to standard output"
)]
struct EchoCli {
    #[args(
        arg_type = "argument",
        name = "string",
        description = "the text to display"
    )]
    string: String,

    #[args(
        arg_type = "option",
        long_form = "--help",
        short_form = "-h",
        description = "this usage string"
    )]
    help: String,
}

#[test]
fn test_func() {
    println!("Hello Test World!");
}

fn main() {
    println!("{}", EchoCli::get_usage());
    //println!("{:?}", args.text);
    //println!("{:?}", args.more_text);
}
