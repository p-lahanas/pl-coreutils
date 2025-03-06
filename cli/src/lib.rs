pub use cli_derive::CliParser;

pub trait CliParser {
    fn usage() -> &'static str;
    //fn parse() -> Self;
}
