use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;
use std::iter::Iterator;

fn main() {
//	let filename = "input.txt";
	let filename = "test_f.txt";

	let mut f = File::open(filename).expect("file not found");

	let mut contents = String::new();
	f.read_to_string(&mut contents)
		.expect("something went wrong reading the file");

	/* TODO pass reference to string to function? */
	let split = contents.split(|c| c == '\n');

//	let mut elems = HashMap::new();
	let mut weights = HashMap::new();

//	let mut vec = Vec::new();
	for s in split {
		let words = s.split(|c| c == ',');

//		let n = words.nth(1).unwrap().parse::<u32>();
//		match n {
//			Ok(n)     => {vec.push(n)}
//			Err(_err) => {continue}
//		}

		println!("{}", s);
		for (i, w) in words.enumerate() {
			let mut name = "";
			let mut weight = 0;
			match i {
				0 => { name = w; println!("name: {}", name); }
				1 => {
					weight = w.parse::<u32>().unwrap();
					weights.insert(name, weight);
				}
//				2 => {
				_ => { continue; }
			}
			println!("{}", w);
		}

	}

//	println!("initial array: {:?}", vec);
	println!("weights: {:?}", weights);

}
