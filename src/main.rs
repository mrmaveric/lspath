use std::env;

fn main() {
	let paths = env::var_os("PATH").unwrap();
	for path in env::split_paths(&paths) 
	{
		println!("{}", path.display());
	}
}
