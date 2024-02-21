use std::env;

fn main() {
    let path_list = match env::var_os("PATH") {
        Some(pl) => pl,
        None => {
            eprintln!("PATH environmental variable is not set");
            return;
        }
    };

    for path in env::split_paths(&path_list) {
        println!("{}", path.display());
    }
}
