/* Advent of code */
/* day 20 */

use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct I {
	p: (u32, u32, u32),
	v: (u32, u32, u32),
	a: (u32, u32, u32)
}

fn main () {
	part1();
	part2();
}

fn read_stdin() {

	let stdin = io::stdin();

	let mut map: Vec<Vec<&str>> = vec![];

	for line in stdin.lock().lines() {
		let l = line.unwrap().to_string();
		let split: Vec<_> = l.split("").collect();
	}

}

fn part1() {

}

fn part2() {

}
