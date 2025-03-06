use cli::CliParser;

#[derive(CliParser, Debug)]
struct Args {
    //#[arg(positional)]
    first_name: String,

    //#[arg(positional)]
    last_name: String,
    //#[arg(named)]
    age: u32,
    // //#[arg(named)]
    // city: Option<String>,
}

fn main() {
    let x = Args::parse();
    println!("{:?}", &x);
}
