use aoc::cli;
use aoc::solver;

fn main() {
    let args = cli::ParsedArguments::new();
    let problems = args.problems();

    for (day, part) in problems {
        let function = cli::get_function(day, part);
        let result = solver::solve(day, part, function);
        println!("{}", result)
    }
}
