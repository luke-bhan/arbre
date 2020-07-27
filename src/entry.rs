pub struct Entry {
    pub name: String,
    pub oid: String,
}

impl Entry {
    pub fn new(name: String, oid: String) -> Entry {
        Entry{name, oid}
    }
}
