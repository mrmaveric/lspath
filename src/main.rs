use std::env;

const VERSION: &str = "0.1.2";

fn main() {
    // Pointless command line arguements code
    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-v" | "-V" => {
                println!("Version: {}", VERSION);
                return;
            }
            "-h" | "-H" => {
                println!(
                    "Usage: {} [-h help][-v version]",
                    env::args().nth(0).unwrap()
                );
                return;
            }
            _ => {
                println!("Unknown arguement: {}", arg);
                return;
            }
        }
    }

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
