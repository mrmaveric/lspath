use std::env;

static VERSION: &str = "0.1.1";

fn main() {
	// Pointless command line arguements code
	for arg in env::args().skip(1) {
		match arg.as_str() {
			"-v"|"-V" => {println!("Version: {}", VERSION); return},
			"-h"|"-H" => {println!("Usage: {} [-h help][-v version]", env::args().nth(0).unwrap()); return},
			_ => {println!("Unknown arguement: {}", arg); return},
		}
	}
	

	// The actual code
	for path in env::split_paths(&(env::var_os("PATH").unwrap()))
	{
		println!("{}", path.display());
	}
}
