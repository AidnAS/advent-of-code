use crate::utils::data_file_reader::data_file_reader;

pub fn solvePart1(date: &str, use_test_data: bool) {
	println!("day 02, Part 1: ");

	// create a data file reader and read the file.
	match data_file_reader(date, use_test_data, 1, false) {
		Ok(lines) => {
			for line in lines {
				
			}
			
			// count the total
			let mut total: i32 = 0;

			let mut outputString: String = String::new();
			if (use_test_data) {
				outputString.push_str("Total [using test data]");
			} else {
				outputString.push_str("Total [using puzzle data]");
			}
			println!("{}: {}", outputString, total);

		}
		Err(_) => {
			println!("Error reading file");
		}
	}
}

pub fn solvePart2(date: &str, use_test_data: bool) {
	println!("day 02, Part 2: ");

	// create a data file reader and read the file.
	match data_file_reader(date, use_test_data, 1, false) {
		Ok(lines) => {
			for line in lines {
				
			}

			// count the total
			let mut total: i32 = 0;

			let mut outputString: String = String::new();
			if (use_test_data) {
				outputString.push_str("Total [using test data]");
			} else {
				outputString.push_str("Total [using puzzle data]");
			}
			println!("{}: {}", outputString, total);

		}
		Err(_) => {
			println!("Error reading file");
		}
	}
}
