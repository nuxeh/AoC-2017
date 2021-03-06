/* Advent of code */
/* day 21 */

use std::io;
use std::io::BufRead;
use std::fmt;

#[derive(Clone)]
struct Pic {
	b: Vec<bool>,
	w: usize,
	h: usize,
	t: Vec<Pic>,
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

	fn new() -> Pic {
		Pic {w: 0, h: 0, b: vec![], t: vec![]}
	}

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

		Pic {w: w, h: h, b: b, t: vec![]}
	}

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

		Pic {w: self.w, h: self.h, b: b, t: vec![]}
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

		Pic {w: self.w, h: self.h, b: b, t: vec![]}

	}

	fn equal(&self, other: Pic) -> bool {
		let mut t;
		let mut u;

//		println!("self: {:?}", self);
//		println!("other: {:?}", other);

		/* check original representation */
		if self.b == other.b {return true}

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

		/* check all combinations */
		for i in 0..3 {
			t = other.rotate(i);
			for i in 0..2 {
				u = t.flip(i);
				if u.b == self.b {return true}
			}
		}

		false
	}

	fn calculate_transformations(&mut self) {
		let mut ts: Vec<Pic> = vec![];
		let mut t;
		let mut u;

		/* check all reflections */
		for i in 0..2 {
			t = self.flip(i);
			ts.push(t);
		}

		/* check all rotations */
		for i in 0..3 {
			t = self.rotate(i);
			ts.push(t);
		}

		/* check all combinations */
		for i in 0..3 {
			t = self.rotate(i);
			for i in 0..2 {
				u = t.flip(i);
				ts.push(u);
			}
		}

		self.t = ts;
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
				v_row.push(Pic {w: sw, h: sw, b: b, t: vec![]});
			}

			v.push(v_row);
		}

		v
	}

	/* set data for picture from an array of subpictures */
	fn combine(&mut self, v: &Vec<Vec<Pic>>) {
		let ny = v.len();
		let nx = v[0].len();
		let sw = v[0][0].w;
		let sh = v[0][0].h;

		let w = nx * sw;
		let h = ny * sh;

		let mut b_new: Vec<bool> = vec![false; w * h];

		for (y, row) in v.iter().enumerate() {
			for (x, sp) in row.iter().enumerate() {
				for (sy, sr) in sp.b.chunks(sp.w).enumerate() {
					for (sx, sc) in sr.iter().enumerate() {
						let dy = (y * sp.w) + sy;
						let dx = (x * sp.h) + sx;
						let i = dy * w + dx;
						b_new[i] = *sc;
//						println!("{:?}", Pic { b: b_new.clone(), w: w, h: h});
					}
				}
			}
		}

		self.w = w;
		self.h = h;
		self.b = b_new;
	}
	/* needs some thought about a proper API implementation */

	fn count_on(&self) -> u32 {
		let count: usize = self.b.iter().filter(|&&v| v == true).count();

		count as u32
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

	/*
	let pic3 = Pic::new_from_string("#..#/..../..../#..#".to_string());
	let mut pic4 = Pic::new();
	println!("{:?}", pic3);
	println!("{:?}", pic3.split());
	pic4.combine(&pic3.split());
	println!("{:?}", pic4);
	*/

	let rs: Vec<(Pic, Pic)> = read_stdin();

	/*
	for rule in rs.clone().iter() {

		println!("{:?}", rule.0);
		for i in 0..3 {
			println!("{:?}", rule.0.rotate(i));
		}
		println!("{:?}", rule.1);
		for i in 0..3 {
			println!("{:?}", rule.1.rotate(i));
		}

		println!("-----------");
	}

	println!("{:?}\n-----------", rs);
	*/

	/*
	let mut rs1 = rs.clone();
	*/

	part1(&rs);
	part2(&rs);
}

fn part1(rules: &Vec<(Pic, Pic)>) {
	println!("Part 1 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
	compute(rules, 5);
}

fn part2(rules: &Vec<(Pic, Pic)>) {
	println!("Part 2 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
	compute(rules, 18);
}

fn compute(rules: &Vec<(Pic, Pic)>, n: u32) {

	let mut pic = Pic::new_from_string(".#./..#/###".to_string());
	println!("{:?}", pic);

	for _ in 0..n {
		let mut subs = pic.split();

		for sub_row in subs.iter_mut() {
			for sub in sub_row.iter_mut() {
				for rule in rules {
					if sub.equal(rule.0.clone()) {
						*sub = rule.1.clone();
//						println!("matched: {:?}", rule.0);
						break;
					}
				}
			}
		}

		pic.combine(&subs);
		println!("{:?}", pic);
		println!("count: {}", pic.count_on());
	}
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
