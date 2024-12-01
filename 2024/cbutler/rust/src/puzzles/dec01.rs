use crate::utils::data_file_reader::data_file_reader;

pub fn solvePart1(date: &str, use_test_data: bool) {
	println!("day 01, Part 1: Id lists total distance");

	// create two vectors of Ids
	let mut firstIds: Vec<i32> = Vec::new();
	let mut secondIds: Vec<i32> = Vec::new();

	// create a data file reader and read the file.
	match data_file_reader(date, use_test_data, 1, false) {
		Ok(lines) => {
			for line in lines {
				let parts: Vec<&str> = line.split_whitespace().collect();
				if parts.len() >= 2 {
					let firstId: i32 = parts[0].parse().unwrap_or(0);
					let secondId: i32 = parts[1].parse().unwrap_or(0);
					firstIds.push(firstId);
					secondIds.push(secondId);
				}
			}
			// sort vectors
			firstIds.sort();
			secondIds.sort();

			// compare both lists and count
			if firstIds.len() != secondIds.len() {
				println!("Error: the two vectors are not the same size");
				return;
			}

			let mut diffs: Vec<i32> = Vec::new();

			for i in 0..firstIds.len() {
				let mut diff = firstIds[i] - secondIds[i];
				if secondIds[i] > firstIds[i] {
					diff = secondIds[i] - firstIds[i];
				}
				diffs.push(diff);
			}

			// count the total
			let mut total: i32 = 0;

			for diff in diffs {
				total += diff;
			}

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
	println!("day 01, Part 2: Id lists similarity score");

	// create two vectors of Ids
	let mut firstIds: Vec<i32> = Vec::new();
	let mut secondIds: Vec<i32> = Vec::new();

	// create a data file reader and read the file.
	match data_file_reader(date, use_test_data, 1, false) {
		Ok(lines) => {
			for line in lines {
				let parts: Vec<&str> = line.split_whitespace().collect();
				if parts.len() >= 2 {
					let firstId: i32 = parts[0].parse().unwrap_or(0);
					let secondId: i32 = parts[1].parse().unwrap_or(0);
					firstIds.push(firstId);
					secondIds.push(secondId);
				}
			}

			// compare both lists and count
			if firstIds.len() != secondIds.len() {
				println!("Error: the two vectors are not the same size");
				return;
			}

			// find the similarity scores for each id in the first list
			let mut scores: Vec<i32> = Vec::new();

			for i in 0..firstIds.len() {
				let mut count: i32 = 0;
				for j in 0..secondIds.len() {
					if firstIds[i] == secondIds[j] {
						count += 1;
					}
				}
				scores.push(firstIds[i] * count);
			}

			// count the total
			let mut total: i32 = 0;

			for score in scores {
				total += score;
			}

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
