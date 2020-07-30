use std::path::Path;
use std::path::PathBuf;
use std::fs;
use std::fs::metadata;

#[derive(Copy, Clone)]
pub struct Workspace<'a> {
    path: &'a Path,
}

impl Workspace<'_> {
    pub fn new(path: &Path) -> Workspace {
        Workspace{path}
    }

    pub fn list_files(&self, path: Option<&Path>) -> Vec<PathBuf> {
        let paths = fs::read_dir(&path.unwrap_or(self.path)).unwrap();
        let mut res: Vec<PathBuf> = Vec::new();
        let ignore = [PathBuf::from(".git"), PathBuf::from("."), PathBuf::from("..")];
        for item in paths {
            let path_unwrap = item.unwrap().path();
            let metadata = metadata(&path_unwrap).unwrap();
            if metadata.is_dir() {
                res.append(&mut self.list_files(Some(&path_unwrap)));
            }
            else {
                let strip = path_unwrap.strip_prefix(&self.path).unwrap().to_path_buf();
                if ignore.iter().filter(|p| **p == strip ).count() == 0{
                    res.push(strip.to_path_buf());
                }
            }
        }
        res
    }

    pub fn read_file(&self, path: &Path) -> String {
        print!("{}", path.to_str().unwrap());
        fs::read_to_string(&self.path.join(path)).unwrap()
    }

    pub fn stat_file(path: &Path) -> String {
        let metadata = fs::metadata(path).unwrap();
        let debug = format!("{:?}", metadata.file_type());
        debug.split(' ').map(|e|{
            if e.chars().next().unwrap().is_digit(10){
                return e;
            }
            else {
                return "";
        }}).collect()
    }
}
