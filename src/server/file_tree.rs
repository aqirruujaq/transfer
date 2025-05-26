use std::{fs, io, path::Path};

// In order to use this program as soon as possible,
// currently only read the contents of a file in a fixed location
pub struct FileTree {
    file_name: Vec<String>,
    path_name: String,
}

impl FileTree {
    pub fn new<P>(path: P) -> Result<FileTree, io::Error>
    where
        P: AsRef<Path>,
    {
        let path_name = path.as_ref().to_string_lossy().to_string();
        let dir = fs::read_dir(path)?;
        let file_name: Result<Vec<String>, io::Error> = dir
            .map(|res| res.map(|dir| dir.file_name().display().to_string()))
            .collect();
        Ok(FileTree {
            file_name: file_name?,
            path_name,
        })
    }

    pub fn file_names(&self) -> &Vec<String> {
        &self.file_name
    }
}

impl Default for FileTree {
    fn default() -> Self {
        FileTree::new("res").unwrap_or_else(|e| panic!("{e}"))
    }
}
