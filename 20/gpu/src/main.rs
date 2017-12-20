/* Advent of code */
/* day 20 */

use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct I {
	p: Xyz,
	v: Xyz,
	a: Xyz
}

#[derive(Debug)]
struct Xyz {
	x: i64,
	y: i64,
	z: i64
}

fn main () {

	let mut inst: Vec<I> = vec![];
	read_stdin(&mut inst);


	part1();
	part2();
}

/* p=<474,3043,412>, v=<66,437,59>, a=<-6,-24,-4> */

fn read_stdin(v: &mut Vec<I>) {

	let stdin = io::stdin();

	let mut map: Vec<Vec<&str>> = vec![];

	for line in stdin.lock().lines() {
		let mut i: I = I {
			a: Xyz {x: 0, y: 0, z: 0},
			p: Xyz {x: 0, y: 0, z: 0},
			v: Xyz {x: 0, y: 0, z: 0}};

		let l = line.unwrap().to_string();
		let s: Vec<_> = l.split(", ").collect();

		for (n, j) in s.iter().enumerate() {
			println!("{:?}", j);
			let s2: Vec<_> = j.split(|c| c == '<' ||
						     c == '>' ||
						     c == ',').collect();
			println!("{:?}", s2);

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
	}

}

fn part1() {

}

fn part2() {

}
