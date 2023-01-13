#[macro_export]
macro_rules! arg_input_file {
    ($text: literal) => {
        clap::arg!([INPUT_FILE] $text)
            .required(true)
            .value_hint(clap::ValueHint::FilePath)
            .value_parser(clap::value_parser!(std::path::PathBuf))
    };
}

#[macro_export]
macro_rules! arg_output_file {
    () => {
        clap::arg!(-o --output [FILE] "Output file (defaults to input dir)")
                .value_parser(clap::value_parser!(std::path::PathBuf))
                .value_hint(clap::ValueHint::FilePath)
    };
}

#[macro_export]
macro_rules! arg_log_level {
    () => {
         clap::arg!(-l --level [LevelFilter] "Logging level")
                .value_parser($crate::level_filter_parser::LevelFilterParser {})
                .default_value("warn")
    };
}

#[macro_export]
macro_rules! arg_check_level {
    ($long: tt, $short: tt, $help: literal, $default: literal) => {
         clap::arg!(-$short --$long [CheckLevel] $help)
                .value_parser($crate::check_level_parser::CheckLevelParser {})
                .default_value($default)
    };
}