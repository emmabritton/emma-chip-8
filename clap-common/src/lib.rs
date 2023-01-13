pub mod arg_matcher;
pub mod check_level_parser;
pub mod level_filter_parser;
pub mod macros;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CheckLevel {
    Off,
    Warn,
    Error
}