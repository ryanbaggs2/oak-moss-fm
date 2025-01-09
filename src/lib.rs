//! A simple file manager, will later include tags to search for related
//! files and directories, color labels, and other metadata.

use std::{fs, io};
use std::ffi::{OsString};
use std::fs::DirBuilder;

/// Returns names of entries in the current working directory.
///
/// May return an io Error.
pub fn working_dir_sorted_entries() -> io::Result<Vec<OsString>> {
    sorted_entries(".")
}

/// Returns names of entries within the specified path.
///
/// May return an io Error.
pub fn sorted_entries(path: &str) -> io::Result<Vec<OsString>> {
    let mut entry_names = fs::read_dir(path)?
        .map(|result| result.map(|entry| entry.file_name()))
        .collect::<io::Result<Vec<_>>>()?;

    entry_names.sort();

    Ok(entry_names)
}

/// Creates the specified directory with default builder options.
///
/// If the directory already exists, an error is returned.
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
        let result = sorted_entries("./testing/");
        assert!(result.is_ok());
    }

    #[test]
    fn entries_sorted() {
        let result = sorted_entries("./testing/").expect("io error");
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
