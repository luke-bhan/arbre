use std::env;
use std::path::Path;
use std::fs;

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
            } else {
                print!("No command named {}", cmd);
            }
        },
        _ => print!("Please provide a command"),
    };
    Ok(())
}


    



