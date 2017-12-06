use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
	let filename = "input.txt";

	let mut f = File::open(filename).expect("file not found");

	let mut contents = String::new();
	f.read_to_string(&mut contents)
		.expect("something went wrong reading the file");

	/* TODO pass reference to string to function? */
	let split = contents.split("\t");

	let mut vec = Vec::new();
	for s in split {
		if s.len() != 0 && s != "\n" {
			let n = s.parse::<i32>();
			match n {
				Ok(n)    => {vec.push(n)}
				Err(err) => {continue}
			}
		}
	}

	println!("initial array: {:?}", vec);

	

}
