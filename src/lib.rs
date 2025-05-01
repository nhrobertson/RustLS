use std::env;
use std::fs;
use std::path::PathBuf;
use colored::Colorize;
use colored::ColoredString;


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
		let dir = fs::read_dir(path)?;
		for entry in dir {
			let entry = entry?;
			let path = entry.path();
			//Implement Stylization for path
			let a_str = style_output(&path);
			print!("{}   " , a_str);
		}
		println!("");
	}
	Ok(())
}

fn style_output(path: &PathBuf) -> ColoredString {
	let path_str = path.to_str().unwrap();
	let md = fs::metadata(path_str).unwrap();
	let name = path.file_name().unwrap().to_str().unwrap();
	if md.is_dir() {
		name.green().bold()
	}
	else {
		name.cyan()
	}
}