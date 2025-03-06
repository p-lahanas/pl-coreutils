pub use cli_derive::CliParser;

pub trait CliParser {
    fn get_usage() -> &'static str;
    //fn parse() -> Self;
}
