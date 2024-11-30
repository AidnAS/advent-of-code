use std::env;
use std::fs;
use std::io::{self, Read};


pub fn data_file_reader(date: &str, use_test_data: bool, part: u8, verbose: bool) -> Result<Vec<String>, io::Error> {
	let current_dir = env::current_dir()?;
	let parent_dir = current_dir.parent().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Parent directory not found"))?;

	if verbose {
		println!("current_dir: {:?}", current_dir);
		println!("parent_dir: {:?}", parent_dir);
	}
	

	let filepath = if use_test_data {
		parent_dir.join(format!("common/data/{}-part{}-test-data.txt", date, part))
	} else {
		parent_dir.join(format!("common/data/{}-data.txt", date))
	};

	if verbose {
		println!("filepath: {:?}", filepath);
	}

	match fs::metadata(&filepath) {
		Ok(metadata) => {
			if metadata.is_file() {
				let mut lines = Vec::new();
				let file_contents = fs::read_to_string(&filepath)?;

				for line in file_contents.lines() {
					lines.push(line.to_string());
				}

				Ok(lines)
			
			} else {
				println!("Path exists, but it's not a file");
				Err(io::Error::new(io::ErrorKind::Other, "Path exists, but it's not a file"))
			}
		}
		Err(_) => {
			println!("File does not exist");
			Err(io::Error::new(io::ErrorKind::NotFound, "File does not exist"))
		}
	}
}
