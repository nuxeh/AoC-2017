/* Advent of code */
/* day 21 */

use std::io;
use std::io::BufRead;
use std::fmt;

#[derive(Clone)]
struct Pic {
	b: Vec<bool>,
//	r: Vec<Vec<bool>>,
	w: usize,
	h: usize,
}

impl fmt::Debug for Pic {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let _ = write!(f, "\n");
		for line in self.b.chunks(self.w) {
			let mut l_s: String = "".to_string();
			for l_e in line.iter() {
				if *l_e {l_s += "#";} else {l_s += ".";}
			}
			let _ = write!(f, "{}\n", l_s);
		}
		write!(f, "w = {} h = {}", self.w, self.h)
	}
}

impl Pic {

	fn new_from_string(s: String) -> Pic {
		let w;
		let h;
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

	fn chunks() {}

	fn flip(&self, n: u8) -> Pic {
		let mut b: Vec<bool> = vec![];

		match n {
			0 => {b = self.b
				.clone()
				.chunks_mut(self.w)
				.fold(vec![],
				|mut acc, s| {
					s.reverse();
					acc.extend_from_slice(s); acc});
			}
			1 => {b = self.b
				.chunks(self.w)
				.rev()
				.fold(vec![],
				|mut acc, s| {acc.extend_from_slice(s); acc});
			}
			_ => {}
		}

		Pic {w: self.w, h: self.h, b: b}
	}

	fn rotate(&self, n: u8) -> Pic {

		if self.w != self.h {panic!("Image must be square")}

		/* create an empty destination vector */
		let mut b: Vec<bool> = vec![false; self.w * self.h];

		const C: [f32; 3] = [0.0, -1.0,  0.0];
		const S: [f32; 3] = [1.0,  0.0, -1.0];

		let centre: (f32, f32) = ((self.h as f32 - 1.0) / 2.0,
					  (self.w as f32 - 1.0) / 2.0);

		for y in 0..self.w {
			for x in 0..self.h {
				let nu = n as usize;
				let x2 = C[nu] * (x as f32 - centre.0)
					 - S[nu] * (y as f32 - centre.1)
					 + centre.0;
				let y2 = S[nu] * (x as f32 - centre.0)
					 + C[nu] * (y as f32 - centre.1)
					 + centre.1;

//				println!("{},{} -> {},{}", x, y, x2, y2);

				b[y * self.w + x] = self.b[
					(y2 * self.w as f32 + x2) as usize
				];
			}
		}

		Pic {w: self.w, h: self.h, b: b}

	}

	fn equal(&self, other: Pic) -> bool {
		let mut t;

		/* check all reflections */
		for i in 0..2 {
			t = other.flip(i);
			if t.b == self.b {return true}
		}

		/* check all rotations */
		for i in 0..3 {
			t = other.rotate(i);
			if t.b == self.b {return true}
		}

		false
	}

	/* split picture into a 2D array of sub pictures */
	fn split(&self) -> Vec<Vec<Pic>> {
		let mut v: Vec<Vec<Pic>> = vec![];
		let sw;

		if ! self.w == self.h {
			panic!("non-square pictures can't be split");
		}

		if self.w % 2 == 0 {
			sw = 2
		} else if self.w % 3 == 0 {
			sw = 3;
		} else {
			panic!("unusual dimension :P");
		}

		let offset = self.w / sw;

		for block in self.b.chunks(self.w * sw) {
			let mut v_row = vec![];
			let mut bs: Vec<Vec<bool>> = vec![vec![]; offset];

			for (s, sec) in block.chunks(sw).enumerate() {
				bs[s % offset].extend_from_slice(sec);
			}

			for b in bs {
				v_row.push(Pic {w: sw, h: sw, b: b});
			}

			v.push(v_row);
		}

		v
	}

	/* set data for picture from an array of subpictures */
	fn combine(&mut self, v: &Vec<Vec<Pic>>) {
		let mut b_new: Vec<bool> = vec![];

		for (y, row) in v.iter().enumerate() {
			for (x, sp) in row.iter().enumerate() {

			}
		}

		self.b = b_new;
	}
}

fn main () {

	/*
	let pic = Pic::new_from_string(".#./..#/###".to_string());
	println!("{:?}", pic);
	println!("{:?}", pic.flip(1));
	println!("{:?}", pic.flip(0));
	println!("{:?}", pic.equal(pic.flip(0)));
	for i in 0..3 {
		println!("{:?}", pic.rotate(i));
		println!("{:?}", pic.equal(pic.rotate(i)));
	}
	let pic2 = Pic::new_from_string(".#/#.".to_string());
	for i in 0..3 {
		println!("{:?}", pic2.rotate(i));
	}
	let pic3 = Pic::new_from_string(".#.#/#.#./.#.#/#.#.".to_string());
	for i in 0..3 {
		println!("{:?}", pic3.rotate(i));
	}
	*/

	let pic3 = Pic::new_from_string("#..#/..../..../#..#".to_string());
	println!("{:?}", pic3);
	println!("{:?}", pic3.split());

	/*
	let rs: Vec<(Pic, Pic)> = read_stdin();

	println!("{:?}", rs);

	let mut rs1 = rs.clone();
	*/

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

		v.push((Pic::new_from_string(s[0].to_string()),
			Pic::new_from_string(s[1].to_string())));
	}

	v
}
