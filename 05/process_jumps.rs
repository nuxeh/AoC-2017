use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
	let filename = "input.txt";

	println!("In file {}", filename);

	let mut f = File::open(filename).expect("file not found");

	let mut contents = String::new();
	f.read_to_string(&mut contents)
		.expect("something went wrong reading the file");

	/* TODO pass reference to string to function? */
	let mut split = contents.split("\n");

	let mut vec = Vec::new();
	for s in split {
		match s.parse::<i32>() {
			Ok(n) => {println!("{}", n);},//vec.push(n);}
			Err(err) => {println!("err");}
		}

		println!("{}", n);
		vec.push(n);
	}

	println!("{:?}", vec);
}
