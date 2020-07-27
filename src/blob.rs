use std::fmt;
use crate::tree::Tree;

pub struct Blob {
    data: String,
}

impl Blob {
    pub fn new(data: String) -> Blob {
        Blob{data}
    }
}

impl fmt::Display for Blob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl From<Tree> for Blob {
    fn from(tree: Tree) -> Self {
        print!("{}", tree.to_string());
        Blob::new(tree.to_string())
    }
}

