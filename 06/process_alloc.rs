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

	println!("{:?}", vec);

//	let mut pc: i32 = 0;
//	let end = vec.len() as i32;
//	let mut ic = 0;
//
//	while (pc < end) && (pc >= 0)
//	{
//		let pc_u = pc as usize;
//		let j = vec[pc_u];
//
//		if j >= 3 {
//			vec[pc_u] -= 1;
//		} else {
//			vec[pc_u] += 1;
//		}
//
//		pc = pc + j;
//
//		ic += 1;
//
////		println!("{:?}", vec);
//	}
//	println!("{} steps", ic);
}
