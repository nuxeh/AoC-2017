/* Advent of code */
/* day 22 */

use std::io;
use std::io::BufRead;
use std::collections::HashMap;

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
	map: HashMap<Xy, bool>
}

impl Map {
	fn new() -> Map {
		Map {
			w: 0, h: 0,
			tl: Xy {x: 0, y: 0},
			br: Xy {x: 0, y: 0},
			pos: Xy {x: 0, y: 0},
			map: HashMap::<Xy, bool>::new()
		}
	}

	fn add(&mut self, p: Xy, v: bool) {

		/* extend map extents */
		if p.x < self.tl.x {self.tl.x = p.x;}
		if p.y < self.tl.y {self.tl.y = p.y;}
		if p.x > self.br.x {self.br.x = p.x;}
		if p.y > self.br.y {self.br.y = p.y;}

		self.map.insert(p, v);
	}

	fn move_one(&mut self, d: u8) {

		match d {
			0 => {self.pos.y -= 1;},
			1 => {self.pos.x += 1;},
			2 => {self.pos.y += 1;},
			3 => {self.pos.x -= 1;},
			_ => {}
		}
	}

	fn get_xy(&self, x: i64, y: i64) -> bool {
		self.map[&Xy {x: x, y: y}]
	}

	fn get_p(&self, p: &Xy) -> bool {
		self.map[p]
	}

	fn get(&self) -> bool {
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

fn part1(m: &mut Map) {
	let mut dir = 0;

	for _ in 0..5 {
		match m.get() {
			true  => {turn_right(&mut dir)}
			false => {turn_left(&mut dir)}
		}
		m.move_one(dir);
		print_map(m);
	}
}

fn part2() {

}

fn print_map(m: &Map) {

	println!("tl = {:#?}", m.tl);
	println!("br = {:#?}", m.br);

	for y in m.tl.y .. m.br.y + 1 {
		let y = y as i64;
		for x in m.tl.x .. m.br.y + 1 {
			let x = x as i64;
			let v = m.map[&Xy {x: x, y: y}];

			let mut e1 = ' ';
			let mut e2 = e1;
			if x == m.pos.x && y == m.pos.y {
				e1 = '['; e2 = ']';
			}

			match v {
				false => {print!("{}.{}", e1, e2);}
				true  => {print!("{}#{}", e1, e2);}
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
							false);}
				'#' => {m.add(Xy {x: x - x0, y: y - y0},
							true);}
				_   => {}
			}
		}
	}

	m.pos = Xy {x: 0, y: 0};
	m.h = h;
	m.w = w;
}
