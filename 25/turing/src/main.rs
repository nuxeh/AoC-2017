/* Advent of code */
/* day 25 */

use std::io;
use std::io::BufRead;

struct Tape {
	end: usize,
	pos: usize,
	vec: Vec<bool>
}

impl Tape {
	fn new() -> Tape {Tape {end: 0, pos: 0, vec: vec![false]}}

	fn move_right(&mut self) {
		self.pos += 1;
		if self.pos > self.end {
			self.vec.push(false);
			self.end += 1;
		}
	}

	fn move_left(&mut self) {
		if self.pos > 0 {
			self.pos -= 1;
		} else {
			self.vec.insert(0, false);
			self.end += 1;
		}
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
