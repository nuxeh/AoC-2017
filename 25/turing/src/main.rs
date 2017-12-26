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

	fn read(&self) -> bool {
		self.vec[self.pos]
	}

	fn write(&mut self, v: bool) {
		self.vec[self.pos] = v;
	}

	fn checksum(&self) -> u64 {
		self.vec.iter().fold(0, |acc,&a| if a {acc + 1} else {acc})
	}
}

fn sm () {

}

fn sm_test(state: &mut u8, tape: &mut Tape) {
	match *state {
		0 => {
			match tape.read() {
				false => {
					tape.write(true);
					tape.move_right();
					*state = 1;
				}
				true  => {
					tape.write(false);
					tape.move_left();
					*state = 1;
				}
			}
		}

		1 => {
			match tape.read() {
				false => {
					tape.write(true);
					tape.move_left();
					*state = 1;
				}
				true  => {
					tape.write(true);
					tape.move_right();
					*state = 1;
				}
			}
		}

		_ => {println!("Unknown state!");}
	}
}

fn main() {
//	read_stdin();

	let mut tape = Tape::new();

	part1(&mut tape);
	part2();
}

fn part1(mut tape: &mut Tape) {

	let mut state = 0;

	for _ in 0..6 {
		sm_test(&mut state, &mut tape);
		println!("{:?}", tape.vec);
	}

	println!("checksum; {}", tape.checksum());
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
