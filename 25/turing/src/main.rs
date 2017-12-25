/* Advent of code */
/* day 25 */

use std::io;
use std::io::BufRead;

struct Tape {
	start: usize,
	end: usize,
	vec: Vec<bool>
}

impl Tape {
	fn new() -> Tape {Tape {start: 0, end: 0, vec: vec![false]}}

	fn move_left(&mut self) {

	}
}


fn main () {
	read_stdin();

	let mut tape = Tape::new();

	part1();
	part2();
}

fn part1() {

}

fn part2() {

}

fn read_stdin() {
	let std = io::stdin();

	for l in std.lock().lines() {
		let l = l.unwrap();
		println!("{}", l);

	}
}
