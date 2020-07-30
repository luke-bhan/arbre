use std::path::Path;
use std::path::PathBuf;
use std::fs;

use crate::lockfile::LockFile;

pub struct Refs<'a> {
    path: &'a Path,
}

impl Refs<'_> {
    pub fn new(path: &Path) -> Refs {
        Refs{path}
    }

    pub fn update_head(&self, oid: String) {
        let mut lockfile = LockFile::new(PathBuf::from(self.head_path()));

        lockfile.hold_for_update();
        lockfile.write(oid);
        lockfile.write("\n".to_string());
        lockfile.commit();
    }

    fn head_path(&self) -> PathBuf {
        self.path.join("HEAD")
    }

    pub fn read_head(&self) -> String {
        if self.head_path().exists() {
            fs::read_to_string(&self.head_path()).unwrap()
        }
        else {
            "".to_string()
        }
    }
}
