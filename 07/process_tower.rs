use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;
use std::iter::Iterator;

fn main() {
	let filename = "input.txt";
//	let filename = "test.txt";

	let mut f = File::open(filename).expect("file not found");

	let mut contents = String::new();
	f.read_to_string(&mut contents)
		.expect("something went wrong reading the file");

	/* TODO pass reference to string to function? */
	let split = contents.split(|c| c == '\n' || c == '\t');

	let mut vec = Vec::new();
	for s in split {
		let n = s.parse::<u32>();
		match n {
			Ok(n)     => {vec.push(n)}
			Err(_err) => {continue}
		}
	}

	println!("initial array: {:?}", vec);

}
