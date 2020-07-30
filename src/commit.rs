use crate::author::Author;
use std::fmt;

pub struct Commit {
    parent: String,
    tree_oid: String,
    author: Author,
    message: String,
}

impl Commit {
    pub fn new(parent: String, tree_oid: String, author: Author, message: String) -> Commit {
        Commit{parent, tree_oid, author, message}
    }
}

impl fmt::Display for Commit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let parent_string = match self.parent.is_empty() {
            true => "".to_string(),
            false => format!("parent {}\n", self.parent),
        };
        write!(f, "tree {}\n{}author {}\ncommitter {}\n\n{}", self.tree_oid.to_string(), parent_string, self.author.to_string(), self.author.to_string(), self.message)
    }
}


