use std::io::{stdin, Read};

fn main() {
	let mut input = String::new();

	stdin().read_to_string(&mut input).expect("Couldn’t read input");

	let sum: i32 = input.split_whitespace()
		.map(|i| i.parse::<i32>().unwrap())
		.sum();

  println!("{}", sum)
}
