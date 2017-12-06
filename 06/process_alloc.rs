use std::env;
use std::fs::File;
use std::io::prelude::*;

use std::iter::Iterator;

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
			let n = s.parse::<u32>();
			match n {
				Ok(n)    => {vec.push(n)}
				Err(err) => {continue}
			}
		}
	}

//	vec.push(15);

	println!("initial array: {:?}", vec);

	let m = find_max(&vec);
	println!("max -> {}", m);

	let h = get_hash(vec);
	println!("hash -> {}", h);
}

fn get_hash (vec: Vec<u32>) -> String {
	let mut s = String::new();

	for v in vec {
		format!("{}{}", s, v);
	}

	s
}

fn find_max (vec: &Vec<u32>) -> usize {
	let mut max_val: u32 = 0;
	let mut max: usize = 0;
	let mut i = 0;

//	for (i, v) in vec.enumerate() {
	for v in vec.iter() {
		println!("{}", i);
//		if v > max_val { max_val = v; max = i}
		if v > &max_val { max_val = *v; max = i}
		i += 1;
	}

	max as usize
}
