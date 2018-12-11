use std::env;

static VERSION: &str = "0.1.1";

fn main() {
	// Pointless command line arguements code
	let args:Vec<String> = env::args().collect();

	for arg in args.iter().skip(1) {
		//if index == 0 {continue}
		match arg as &str {
			"-v"|"-V" => {println!("Version: {}", VERSION); return},
			"-h"|"-H" => {println!("Usage: {} [-h help][-v version]", args[0]); return},
			_ => (),
		}
	}
	

	// The actual code
	for path in env::split_paths(&(env::var_os("PATH").unwrap()))
	{
		println!("{}", path.display());
	}
}
