/* Advent of code */
/* day 21 */

use std::io;
use std::io::BufRead;

#[derive(Debug)]
#[derive(Clone)]
struct Pic {
	b: Vec<u32>,
	w: usize,
	h: usize,
}


fn main () {

	let rs: Vec<P> = read_stdin();

//	println!("{:?}", rs);

	let mut rs1 = rs.clone();

//	part1(&mut rs1);
}

fn part1() {

}

fn part2() {

}

fn read_stdin() -> Vec<P> {

	let stdin = io::stdin();
	let mut v: Vec<P> = vec![];

	for line in stdin.lock().lines() {
		let mut i: P = P {
			a: Xyz {x: 0, y: 0, z: 0},
			p: Xyz {x: 0, y: 0, z: 0},
			v: Xyz {x: 0, y: 0, z: 0},
			collided: false};

		let l = line.unwrap().to_string();
		let s: Vec<_> = l.split(", ").collect();

		for (n, j) in s.iter().enumerate() {
			let s2: Vec<_> = j.split(|c| c == '<' ||
						     c == '>' ||
						     c == ',').collect();

			let x: i64 = s2[1].parse().unwrap();
			let y: i64 = s2[2].parse().unwrap();
			let z: i64 = s2[3].parse().unwrap();

			match n {
				0 => i.p = Xyz {x: x, y: y, z: z},
				1 => i.v = Xyz {x: x, y: y, z: z},
				2 => i.a = Xyz {x: x, y: y, z: z},
				_ => {}
			}
		}

		println!("{:?}", i);
		v.push(i);
	}

	v
}
