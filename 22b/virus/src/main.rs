/* Advent of code */
/* day 22 */

use std::io;
use std::io::BufRead;
use std::collections::HashMap;

/*
	0: Clean
	1: Weakened
	2: Infected
	3: Flagged
*/

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Debug)]
struct Xy {
	x: i64, // infinite, yeah!
	y: i64
}

#[derive(Debug)]
struct Map {
	pos: Xy,
	w: usize,
	h: usize,
	tl: Xy,
	br: Xy,
	infection_count: u64,
	map: HashMap<Xy, u8>
}

impl Map {
	fn new() -> Map {
		Map {
			w: 0, h: 0,
			tl: Xy {x: 0, y: 0},
			br: Xy {x: 0, y: 0},
			pos: Xy {x: 0, y: 0},
			infection_count: 0,
			map: HashMap::<Xy, u8>::new()
		}
	}

	fn in_map(&self, p: &Xy) -> bool {

		if p.x < self.tl.x {return false;}
		if p.y < self.tl.y {return false;}
		if p.x > self.br.x {return false;}
		if p.y > self.br.y {return false;}

		true
	}

	fn add(&mut self, p: Xy, v: u8) {

		let mut inside = true;

		/* extend map extents */
		if p.x < self.tl.x {self.tl.x = p.x; inside = false}
		if p.y < self.tl.y {self.tl.y = p.y; inside = false}
		if p.x > self.br.x {self.br.x = p.x; inside = false}
		if p.y > self.br.y {self.br.y = p.y; inside = false}

		self.map.insert(p, v);

		/* add padding */
		if ! inside {
			for y in self.tl.y .. self.br.y + 1 {
				let y = y as i64;
				for x in self.tl.x .. self.br.x + 1 {
					let x = x as i64;
					self.map.entry(Xy {x: x, y: y}).or_insert(0);
				}
			}
		}
	}

	fn move_one(&mut self, d: u8) {

		let p = self.pos.clone();
		let v = self.get();
		let vn = (v + 1) % 4;
		self.add(p, vn);

		if vn == 2 {self.infection_count += 1;}

		match d {
			0 => {self.pos.y -= 1;},
			1 => {self.pos.x += 1;},
			2 => {self.pos.y += 1;},
			3 => {self.pos.x -= 1;},
			_ => {}
		}
	}

//	fn get(&self) -> Result<bool, &'static str> {

	fn get(&mut self) -> u8 {

		let p = self.pos.clone();

		if ! self.in_map(&self.pos) {
			self.add(p, 0);
		}

		self.map[&self.pos]
	}
}

fn main () {
	let mut map = Map::new();
	read_stdin(&mut map);

//	println!("{:?}", map);

	print_map(&map);

	part1(&mut map);
	part2();
}

fn turn_right(d: &mut u8) {*d = (*d + 1) % 4}
fn turn_left (d: &mut u8) {if *d == 0 {*d = 3} else {*d = *d - 1}}
fn reverse(d: &mut u8) {*d = (*d + 2) % 4}

fn part1(m: &mut Map) {
	let mut dir = 0;

	for i in 0..10000000 {
		match m.get() {
			0 => {turn_left(&mut dir)}
			2 => {turn_right(&mut dir)}
			3 => {reverse(&mut dir)}
			_ => {}
		}
		m.move_one(dir);
//		print_map(m);
//		println!("{}", i);
	}
//	print_map(m);
	println!("infection count: {}", m.infection_count);
}

fn part2() {

}

fn print_map(m: &Map) {

//	println!("tl = {:#?}", m.tl);
//	println!("br = {:#?}", m.br);

	println!("");

	for y in m.tl.y .. m.br.y + 1 {
		let y = y as i64;
		for x in m.tl.x .. m.br.x + 1 {
			let x = x as i64;
			let v = m.map[&Xy {x: x, y: y}];

			let mut e1 = ' ';
			let mut e2 = e1;
			if x == m.pos.x && y == m.pos.y {
				e1 = '['; e2 = ']';
			}

			match v {
				0 => {print!("{}.{}", e1, e2);}
				1 => {print!("{}W{}", e1, e2);}
				2 => {print!("{}#{}", e1, e2);}
				3 => {print!("{}F{}", e1, e2);}
				_ => {}
			}
		}
		print!("\n");
	}
}

fn read_stdin(m: &mut Map) {

	let stdin = io::stdin();
	let all_lines: Vec<_> = stdin.lock().lines().map(|v| v.unwrap()).collect();

	let mut w = 0;
	let h = all_lines.len();
	let y0 = ((h - 1) / 2) as i64;

	for (y, line) in all_lines.iter().enumerate() {
		let l = line.to_string();
		w = l.chars().count();

		let x0 = ((w - 1) / 2) as i64;
		let y = y as i64;

		for (x, c) in l.chars().enumerate() {
			let x = x as i64;
			match c {
				'.' => {m.add(Xy {x: x - x0, y: y - y0},
							0);}
				'#' => {m.add(Xy {x: x - x0, y: y - y0},
							2);}
				_   => {}
			}
		}
	}

	m.pos = Xy {x: 0, y: 0};
	m.h = h;
	m.w = w;
}

// TODO: xy iterator for Map
