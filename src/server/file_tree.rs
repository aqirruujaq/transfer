use std::{
    collections::HashMap,
    fs::{read_dir, File},
    io,
    path::Path,
 sync::Arc,
};

// In order to use this program as soon as possible,
// currently only read the contents of a file in a fixed location
pub struct FileTree {
    files: HashMap<Arc<str>, File>,
}

impl FileTree {
    fn new<P>(path: P) -> Result<FileTree, io::Error>
    where
        P: AsRef<Path>,
    {
        let dir = read_dir(path)?;
        let mut files = HashMap::new();
        for file in dir {
            let file = file?;
            if file.file_type()?.is_file() {
                files.insert(
                    Arc::from(file.file_name().into_string().unwrap()),
                    File::open(file.path())?,
                );
            }
        }
        Ok(FileTree { files })
    }

    pub fn file_names(&self) -> impl Iterator<Item = &Arc<str>> {
        self.files.keys()
    }

    pub fn get_file(&self, name: &str) -> Option<&File> {
        self.files.get(name)
    }
}

impl Default for FileTree {
    fn default() -> Self {
        FileTree::new("res").unwrap_or_else(|e| panic!("{e}"))
    }
}
