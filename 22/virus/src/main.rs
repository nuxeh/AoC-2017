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

fn main () {
	let mut map = HashMap::<Xy, bool>::new();
	let mut pos = read_stdin(&mut map);

	println!("{:#?}", map);
	println!("{:#?}", pos);

	part1();
	part2();
}

fn part1() {

}

fn part2() {

}

fn read_stdin(map: &mut HashMap<Xy, bool>) -> Xy {

	let stdin = io::stdin();
	let all_lines: Vec<_> = stdin.lock().lines().map(|v| v.unwrap()).collect();
	let h = all_lines.len();
	let y0 = ((h - 1) / 2) as i64;
	let mut x0 = 0;

	for (y, line) in all_lines.iter().enumerate() {
		let y = y as i64;
		let l = line.to_string();
		let w = l.chars().count();
		x0 = ((w - 1) / 2) as i64;
		println!("{} {}", x0, y0);

		for (x, c) in l.chars().enumerate() {
			let x = x as i64;
			match c {
				'.' => {map.insert(Xy {x: x - x0, y: y - y0},
							true);}
				'#' => {map.insert(Xy {x: x - x0, y: y - y0},
							false);}
				_   => {}
			}
		}
	}

	Xy {x: x0, y: y0}
}
