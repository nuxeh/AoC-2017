use std::env;
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

//	vec.push(15);

	println!("initial array: {:?}", vec);

	let mut finished = false;
	let mut seen = HashMap::new();
	let mut count = 0;
	let mut count_1 = 0;
	let mut count_2 = 0;

	while finished == false {

		let m = find_max(&vec);
//		println!("max -> {}", m);

		let h = get_hash(&vec);
//		println!("hash -> {}", h);

		seen.entry(h.clone()).or_insert(0);
		if let Some(x) = seen.get_mut(&h) {
			if *x > 0 && count_1 == 0 {
				count_1 = count;
			}
			if *x > 1 {
				count_2 = count;
				break;
			}
			*x = *x + 1;
		}

//		println!("{:?}", seen);

		realloc(m, &mut vec);
//		println!("{:?}", vec);

		count += 1;
	}

	println!("{}", count_1);
	println!("{}", count_2 - count_1);

}

/* // HOW?!
fn process(vec: &mut Vec<u32>) {
	let m = find_max(&vec);
	println!("max -> {}", m);

	let h = get_hash(&vec);
	println!("hash -> {}", h);

	realloc(m, &mut vec);
	println!("{:?}", vec);
}
*/

fn realloc (start: usize, vec: &mut Vec<u32>) {
	let mut i = start as u32;
	let n = vec[start];

	vec[start] = 0;

	for _ in 0..n {
		i += 1;
		let index = i as usize % vec.len();

//		println!("{}", vec[index]);

		vec[index] += 1;
	}
}

fn get_hash (vec: &Vec<u32>) -> String {
	let mut s = String::new();

	for v in vec {
		s = format!("{}{}", s, v);
	}

	s
}

fn find_max (vec: &Vec<u32>) -> usize {
	let mut max_val: u32 = 0;
	let mut max: usize = 0;

	for (i, v) in vec.iter().enumerate() {
//		println!("{}", i);
		if v > &max_val { max_val = *v; max = i}
	}

	max as usize
}
