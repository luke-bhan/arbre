use crate::author::Author;
use std::fmt;

pub struct Commit {
    tree_oid: String,
    author: Author,
    message: String,
}

impl Commit {
    pub fn new(tree_oid: String, author: Author, message: String) -> Commit {
        Commit{tree_oid, author, message}
    }
}

impl fmt::Display for Commit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tree {}\nauthor {}\ncommitter {}\n\n{}", self.tree_oid.to_string(), self.author.to_string(), self.author.to_string(), self.message)
    }
}


