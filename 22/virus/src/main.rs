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
	map: HashMap<Xy, bool>
}

impl Map {
	fn new() -> Map {
		Map {
			w: 0, h: 0,
			pos: Xy {x: 0, y: 0},
			map: HashMap::<Xy, bool>::new()
		}
	}
}

fn main () {
	let mut map = Map::new();
	let mut pos = read_stdin(&mut map);

	println!("{:?}", map);
	println!("{:#?}", pos);

	print_map(&map);

	part1();
	part2();
}

fn part1() {

}

fn part2() {

}

fn print_map(m: &Map) {
//	for pos in map {
//		println!("{:?}", pos);
//	}

	for y in 0..m.h {
		let y = y as i64;
		for x in 0..m.w {
			let x = y as i64;
//			print!("{}", m.map[&Xy {x: x, y: y}]);
		}
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
				'.' => {m.map.insert(Xy {x: x - x0, y: y - y0},
							true);}
				'#' => {m.map.insert(Xy {x: x - x0, y: y - y0},
							false);}
				_   => {}
			}
		}
	}

	m.pos = Xy {x: x0, y: y0};
	m.h = h;
	m.w = w;
}
