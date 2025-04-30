use std::env;
use std::fs;
use std::path::PathBuf;


pub fn run() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();
//	let current_path = env::current_dir()?;
	let mut paths = Vec::new();
	if args.len() <= 1 {
		paths.push(env::current_dir()?);
	}
	else {
		for i in 0..args.len() {
			if i == 0 {
				continue;
			}
			let mut path = PathBuf::new();
			path.push(&args[i]);
			paths.push(path);
		}
	}
	
	for path in &paths {
		if paths.len() > 1 {
			let disp = path.to_str().unwrap();
			println!("-------{disp}-------");
		}
		let current_dir = fs::read_dir(path)?;
		for entry in current_dir {
			let entry = entry?;
			let path = entry.path();
			println!("{}" ,path.display())
		}
		println!("");
	}
	Ok(())
}