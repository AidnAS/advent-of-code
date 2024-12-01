
mod puzzles {
	pub mod boilerplate;
	pub mod dec01;
}

mod utils {
	pub mod data_file_reader;
}

use chrono::Local;

// use puzzles::boilerplate;
use puzzles::dec01;

fn main() {

	let now = Local::now();
	let current_date = now.format("%d-%m-%Y").to_string();

	// test boilerplate
	// let test_date = "01-12-2000";
	// boilerplate::solvePart1(test_date, true);
	// boilerplate::solvePart2(test_date, true);

	// run puzzle solvers
	// use_test_data
	dec01::solvePart1(&current_date, false); // ans: 3508942
	dec01::solvePart2(&current_date, false); // ans: 26593248
}