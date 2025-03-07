use clap::Parser;

#[derive(Parser)]
#[command(version="1.0", about = "Display a line of text", long_about = None)]
struct Args {
    /// Output text
    string: Option<Vec<String>>,

    /// Do not output the trailing newline
    #[arg(short)]
    n: bool,
}

fn main() {
    let args = Args::parse();

    let output_string = args.string.unwrap_or(vec!["".to_string()]);

    print!("{}", output_string.join(" "));

    if !args.n {
        print!("\n")
    }
}

#[test]
fn test_func() {}
