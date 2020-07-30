const REGULAR_MODE: &str = "100644";
const EXECUTABLE_MODE: &str = "100755";

pub struct Entry {
    pub name: String,
    pub oid: String,
    pub stat: String,
}

impl Entry {
    pub fn new(name: String, oid: String, stat: String) -> Entry {
        Entry{name, oid, stat}
    }

    pub fn mode(&self) -> String {
        match self.stat.as_ref() {
            "33261" => EXECUTABLE_MODE.to_string(),
            _ => REGULAR_MODE.to_string(),
        }
    }
}
