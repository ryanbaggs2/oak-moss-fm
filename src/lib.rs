//! A simple file manager.

use std::{fs, io};
use std::ffi::{OsString};

pub fn working_dir_entry_names_sorted() -> Result<Vec<OsString>, io::Error> {
    entry_names_sorted(".")
}

pub fn entry_names_sorted(path: &str) -> Result<Vec<OsString>, io::Error> {
    let mut entry_names = fs::read_dir(path)?
        .map(|result| result.map(|entry| entry.file_name()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entry_names.sort();

    Ok(entry_names)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entries_ok() {
        let result = entry_names_sorted("./testing/");
        assert!(result.is_ok());
    }

    #[test]
    fn entries_sorted() {
        let result = entry_names_sorted("./testing/").expect("io error");
        assert_eq!(result[0], OsString::from("a.txt"));
        assert_eq!(result[1], OsString::from("b"));
        assert_eq!(result[2], OsString::from("c.txt"));
    }
}
