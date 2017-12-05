use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
	let filename = "input.txt";
//	let filename = "test.txt";

	println!("In file {}", filename);

	let mut f = File::open(filename).expect("file not found");

	let mut contents = String::new();
	f.read_to_string(&mut contents)
		.expect("something went wrong reading the file");

	/* TODO pass reference to string to function? */
	let split = contents.split("\n");

	let mut vec = Vec::new();
	for s in split {
		if s.len() != 0 {
			let n = s.parse::<i32>().unwrap();
//		match s.parse::<i32>() {
//			Ok(n) => {println!("{}", n);},//vec.push(n);}
//			Err(err) => {println!("err");}
//		}

//		println!("{}", n);
		vec.push(n);
		}
	}

	println!("{:?}", vec);

	let mut pc: i32 = 0;
	let end = vec.len() as i32;
	let mut ic = 0;

	while (pc < end) && (pc >= 0)
	{
		let j = vec[pc as usize];

		if j >= 3 {
			vec[pc as usize] -= 1;
		} else {
			vec[pc as usize] += 1;
		}

		pc = pc + j;

		ic += 1;

//		println!("{:?}", vec);
	}
	println!("{} steps", ic);
}
