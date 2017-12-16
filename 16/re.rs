use std::io;
use std::io::BufRead;

fn main () {

	let stdin = io::stdin();

	let mut moves = vec![];

	for line in stdin.lock().lines() {
		let line = line.unwrap();
		let split: Vec<_> = line.split(",").collect();

		moves.push(split[0].to_owned());
//		let string = split[0].clone();

//		let s = split[0].to_owned();

		moves.push("cheese".to_string());

//		let mv = split[0];
//		{
//			moves.push(mv);
//		}

		// ints?
	}
}

