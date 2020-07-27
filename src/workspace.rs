use std::path::Path;
use std::path::PathBuf;
use std::fs;

#[derive(Copy, Clone)]
pub struct Workspace<'a> {
    path: &'a Path,
}

impl Workspace<'_> {
    pub fn new(path: &Path) -> Workspace {
        Workspace{path}
    }

    pub fn list_files(self) -> Vec<PathBuf> {
        let paths = fs::read_dir(&self.path).unwrap();
        let mut res: Vec<PathBuf> = Vec::new();
        let ignore = [PathBuf::from(".git"), PathBuf::from("."), PathBuf::from("..")];
        for item in paths {
            let strip = item.unwrap().path().strip_prefix(&self.path).unwrap().to_path_buf();
            if ignore.iter().filter(|p| **p == strip ).count() == 0{

            res.push(strip);
            }
        }
        res
    }

    pub fn read_file(self, path: &Path) -> String {
        fs::read_to_string(&self.path.join(path)).unwrap()
    }
}
