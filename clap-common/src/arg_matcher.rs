use std::ffi::OsString;
use std::path::{Path, PathBuf};
use clap::ArgMatches;

pub trait ArgMatchesFiles {
    fn get_file(&self, name: &str, display: &str) -> Result<PathBuf, String>;
    fn get_output_file(
        &self,
        name: &str,
        display: &str,
        default: PathBuf,
    ) -> Result<PathBuf, String>;
}

impl ArgMatchesFiles for ArgMatches {
    fn get_file(&self, name: &str, display: &str) -> Result<PathBuf, String> {
        let file = self
            .get_one::<PathBuf>(name)
            .cloned()
            .unwrap_or_else(|| panic!("{display} must be provided"));
        if !file.is_file() {
            return Err(format!("{display} {} is not a file", file.display()));
        }
        Ok(file)
    }

    fn get_output_file(
        &self,
        name: &str,
        display: &str,
        default: PathBuf,
    ) -> Result<PathBuf, String> {
        let file = self.get_one::<PathBuf>(name).cloned().unwrap_or(default);
        if file.is_dir() {
            return Err(format!("{display} {} is a directory", file.display()));
        }
        Ok(file)
    }
}

pub fn create_output_default(input: &Path, ext: &str, display: &str) -> PathBuf {
    let (filename, path) = split_file(input, display);
    let mut default_output = path;
    default_output.push(filename);
    if !default_output.set_extension(ext) {
        panic!("Unable to set output extension, please raise a bug");
    }
    default_output
}

fn split_file(file: &Path, display: &str) -> (OsString, PathBuf) {
    let filename = file
        .file_stem()
        .unwrap_or_else(|| panic!("{display} {} has invalid path/filename", file.display()));
    let path = file
        .parent()
        .unwrap_or_else(|| panic!("{display} {} has invalid path/location", file.display()));
    (filename.to_os_string(), path.to_path_buf())
}
