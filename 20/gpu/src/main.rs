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
	x: u64,
	y: u64,
	z: u64
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
		let l = line.unwrap().to_string();
		let s: Vec<_> = l.split(", ").collect();
		for j in s {
			println!("{:?}", j);
		}
//		let s2: Vec<_> = 
	}

}

fn part1() {

}

fn part2() {

}
