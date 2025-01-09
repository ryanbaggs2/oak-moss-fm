//! A simple file manager.

use std::{fs, io};
use std::ffi::{OsString};
use std::fs::DirBuilder;

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

/// Creates the specified directory with default builder options.
///
/// If the directory exists, an error is returned.
pub fn create_dir(path: &str) -> io::Result<()> {
    DirBuilder::new().create(path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::remove_dir;
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

    #[test]
    fn create_dir_works() {
        let dir = "taco";
        let result = create_dir(dir);
        remove_dir(dir).expect("Created and removed directory for test");

        assert!(result.is_ok());
    }

    #[test]
    fn create_existing_dir() {
        let dir = "tuesday";
        DirBuilder::new()
            .create(dir)
            .expect("Create a directory and removed for test");
        let result = create_dir(dir);
        remove_dir(dir).expect("Create a directory and removed for test");

        assert!(result.is_err());
    }
}
