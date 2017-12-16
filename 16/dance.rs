use std::io;
use std::io::BufRead;

fn main () {

	let mut c: u8 = 64;
	let mut a: Vec<_> = vec![0; 16]
		.iter().map(|_| {c += 1; c as char}).collect();
	println!("{:?}", a);


	let stdin = io::stdin();

	for line in stdin.lock().lines() {
		let line = line.unwrap();
		let split: Vec<_> = line.split(",").collect();

		for m in split {
			println!("{:?}", m);
		}
	}
}
