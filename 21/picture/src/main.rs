/* Advent of code */
/* day 21 */

use std::io;
use std::io::BufRead;
use std::fmt;

#[derive(Clone)]
//#[derive(Debug)]
struct Pic {
	b: Vec<bool>,
//	r: Vec<Vec<bool>>,
	w: usize,
	h: usize,
}

impl fmt::Debug for Pic {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "\n");
		for line in self.b.chunks(self.w) {
			let mut l_s: String = "".to_string();
			for l_e in line.iter() {
				if *l_e {l_s += "#";} else {l_s += ".";}
			}
			write!(f, "{}\n", l_s);
		}
		write!(f, "w = {} h = {}", self.w, self.h)
	}
}

impl Pic {

	fn new_from_string(s: String) -> Pic {
		let mut w;
		let mut h;
		let mut b = vec![];

		let lines: Vec<_> = s.split("/").collect();

		w = lines[0].len();
		h = lines.len();

		for l in lines {
			for c in l
			.chars()
			.map(|c| if c == '#' {true} else {false}) {
				b.push(c);
			}
		}

		Pic {w: w, h: h, b: b}
	}

}

fn main () {

	let rs: Vec<(Pic, Pic)> = read_stdin();

	println!("{:?}", rs);

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
		let l = line.unwrap().to_string();
		let s: Vec<_> = l.split(" => ").collect();

		println!("{} {:?}", l, v);
		v.push((Pic::new_from_string(s[0].to_string()),
			Pic::new_from_string(s[1].to_string())));
	}

	v
}
