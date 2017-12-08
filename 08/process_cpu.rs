use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;
use std::iter::Iterator;

fn main() {
//	let filename = "input.txt";
	let filename = "test.txt";

	let mut f = File::open(filename).expect("file not found");

	let mut contents = String::new();
	f.read_to_string(&mut contents)
		.expect("something went wrong reading the file");

	/* TODO pass reference to string to function? */
	let split = contents.split(|c| c == '\n');

//	let mut elems = HashMap::new();
	let mut r = HashMap::new();

//	let mut vec = Vec::new();
	for s in split {
		let words = s.split(|c| c == ' ');

//		let n = words.nth(1).unwrap().parse::<u32>();
//		match n {
//			Ok(n)     => {vec.push(n)}
//			Err(_err) => {continue}
//		}

		println!("{}", s);

		let mut reg = "";
		let mut oper;
		let mut diff;
		let mut comp1 = "";
		let mut comp2;
		let mut comp_op;

		for (i, w) in words.enumerate() {

			match i {
				0 => { reg  = w; },
				1 => { oper = w; },
				2 => { diff = w.parse::<i32>().unwrap(); },
				4 => { comp1 = w; },
				5 => { comp_op = w; },
				6 => { comp2 = w.parse::<i32>().unwrap(); },
				_ => {}
			}

			println!("{}", w);
		}

		/* create registers if needed */
		r.entry(reg.clone()).or_insert(0);
		r.entry(comp1.clone()).or_insert(0);

		let Some(comp1_v) = &r.get(&comp1.clone()).unwrap();

		if let Some(x) = r.get_mut(&reg) {

			match oper {
				"dec" => {}
				"inc" => {}
				_     => { panic!("bad input"); }
			}

//			if *x > 0 && count_1 == 0 {
//				count_1 = count;
//			}
//			if *x > 1 {
//				count_2 = count;
//				break;
//			}
//			*x = *x + 1;
		}

	}

//	println!("initial array: {:?}", vec);
//	println!("weights: {:?}", weights);

}
