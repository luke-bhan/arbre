use std::fmt;
use crate::tree::Tree;
use crate::author::Author;
use crate::commit::Commit;

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
        Blob::new(tree.to_string())
    }
}

impl From<Author> for Blob {
    fn from(author: Author) -> Self {
        Blob::new(author.to_string())
    }
}

impl From<Commit> for Blob {
    fn from(commit: Commit) -> Self {
        Blob::new(commit.to_string())
    }
}
