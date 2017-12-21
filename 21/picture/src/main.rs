/* Advent of code */
/* day 21 */

use std::io;
use std::io::BufRead;
use std::fmt;

#[derive(Clone)]
struct Pic {
	b: Vec<bool>,
	r: Vec<Vec<bool>>,
	w: usize,
	h: usize,
}

impl fmt::Debug for Pic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	for y in 0..self.h {
	       write!(f, "Point {{ x: {}, y: {} }}", self.w, self.h)
	}
    }
	Ok(1)
}

impl Pic {

	fn new_from_string(s: &String) -> Pic {
		let mut w;
		let mut h;

		let lines = s.split("/").collect();
		for l in lines {

		}

		w = lines[0].len();
		h = lines.len();

		Pic {w: w, h: h, b: }
	}

}

fn main () {

	let rs: Vec<(Pic, Pic)> = read_stdin();

//	println!("{:?}", rs);

	let mut rs1 = rs.clone();

//	part1(&mut rs1);
}

fn part1() {

}

fn part2() {

}

fn read_stdin() -> Vec<(Pic, Pic)> {

	let stdin = io::stdin();
	let mut v: Vec<(Pic, Pic)> = vec![];

	for line in stdin.lock().lines() {
//		let mut i: P = P {
//			a: Xyz {x: 0, y: 0, z: 0},
//			p: Xyz {x: 0, y: 0, z: 0},
//			v: Xyz {x: 0, y: 0, z: 0},
//			collided: false};
//
//		let l = line.unwrap().to_string();
//		let s: Vec<_> = l.split(", ").collect();
//
//		for (n, j) in s.iter().enumerate() {
//			let s2: Vec<_> = j.split(|c| c == '<' ||
//						     c == '>' ||
//						     c == ',').collect();
//
//			let x: i64 = s2[1].parse().unwrap();
//			let y: i64 = s2[2].parse().unwrap();
//			let z: i64 = s2[3].parse().unwrap();
//
//			match n {
//				0 => i.p = Xyz {x: x, y: y, z: z},
//				1 => i.v = Xyz {x: x, y: y, z: z},
//				2 => i.a = Xyz {x: x, y: y, z: z},
//				_ => {}
//			}
//		}
//
//		println!("{:?}", i);
//		v.push(i);
	}

	v
}
