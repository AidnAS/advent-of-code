
mod puzzles {
	pub mod boilerplate;
}

mod utils {
	pub mod data_file_reader;
}

use chrono::Local;

use puzzles::boilerplate;


fn main() {

	let now = Local::now();
	let formatted_date = now.format("%d-%m-%Y").to_string();

	// test boilerplate
	let test_date = "01-12-2000";
	boilerplate::solvePart1(test_date, true);
	boilerplate::solvePart2(test_date, true);

	// run puzzle solvers
	// use_test_data
	// day_01(formatted_date, true);
}