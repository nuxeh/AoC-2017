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
}

fn main () {
	let mut map = Map::new();
	read_stdin(&mut map);

//	println!("{:?}", map);

	print_map(&map);

	part1();
	part2();
}

fn part1() {

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
			match v {
				false => {print!(".");}
				true  => {print!("#");}
			}
		}
		print!("\n");
	}
}

fn read_stdin(m: &mut Map) {

	let stdin = io::stdin();
	let all_lines: Vec<_> = stdin.lock().lines().map(|v| v.unwrap()).collect();

	let mut x0 = 0;
	let mut w = 0;
	let h = all_lines.len();
	let y0 = ((h - 1) / 2) as i64;

	for (y, line) in all_lines.iter().enumerate() {
		let l = line.to_string();
		w = l.chars().count();

		x0 = ((w - 1) / 2) as i64;
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

	m.pos = Xy {x: x0, y: y0};
	m.h = h;
	m.w = w;
}
