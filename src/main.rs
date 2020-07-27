use std::env;
use std::path::Path;
use std::fs;

mod workspace;
use workspace::Workspace;

mod blob;
use blob::Blob;

mod database;
use database::Database;

mod tree;
use tree::Tree;

mod entry;
use entry::Entry;

pub fn init(path: &Path) {
    let git_path = path.join(".git");

    let dirs = vec!["objects", "refs"];
    for name in dirs.iter() {
        fs::create_dir_all(git_path.join(name)).unwrap();
    }
    print!("Initialized empty lit repository in {}", git_path.display());
}

fn main() -> std::io::Result<()> {
    // get command line argument for path
    let args: Vec<String> = env::args().collect();

    // if empty use working directory as default path
    match args.get(1) {
        Some(cmd) => {
            if String::from("init") == cmd.to_string() {
                match args.get(2) {
                    Some(dir) => init(&Path::new(dir).canonicalize().unwrap()),
                    None => init(&env::current_dir()?),
                };
            }
            else if String::from("commit") == cmd.to_string() {
                let root_path = env::current_dir()?;
                let git_path = root_path.join(".git");
                let db_path = git_path.join("objects");
                let ws = Workspace::new(&root_path);
                let db = Database::new(db_path.as_path());
                let mut entries: Vec<Entry> = Vec::new();

                for file in ws.list_files().iter() {
                    let data = ws.read_file(file);
                    let blob = Blob::new(data);
                    let oid = db.store(blob, "blob".to_string());

                    entries.push(Entry::new(file.as_path().to_str().unwrap().to_string(), oid));
                }

                let tree = Tree::new(entries);
                db.store(Blob::from(tree), "tree".to_string());
            }
            else {
                print!("No command named {}", cmd);
            }
        },
        _ => print!("Please provide a command"),
    };
    Ok(())
}
