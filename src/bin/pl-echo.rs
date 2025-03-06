use cli::CliParser;

#[derive(CliParser)]
struct EchoCli {
    #[args(arg_type = "positional")]
    string: String,

    #[args(arg_type = "named")]
    help: String,
}

#[test]
fn test_func() {
    println!("Hello Test World!");
}

fn main() {
    println!("{}", EchoCli::usage());
    //println!("{:?}", args.text);
    //println!("{:?}", args.more_text);
}
