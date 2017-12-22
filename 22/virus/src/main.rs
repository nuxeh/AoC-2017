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
	x: usize, // infinite, yeah!
	y: usize
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
	let yo = (h - 1) / 2;
	let mut xo = 0;

	for (y, line) in all_lines.iter().enumerate() {
		let l = line.to_string();
		let w = l.chars().count();
		xo = (w - 1) / 2;
		println!("{} {}", xo, yo);

		for (x, c) in l.chars().enumerate() {
			match c {
				'.' => {map.insert(Xy {x: x - xo, y: y - yo},
							true);}
				'#' => {map.insert(Xy {x: x - xo, y: y - yo},
							false);}
				_   => {}
			}
		}
	}

	Xy {x: xo, y: yo}
}
