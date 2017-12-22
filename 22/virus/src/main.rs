/* Advent of code */
/* day 22 */

use std::io;
use std::io::BufRead;
use std::collections::HashMap;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
struct Xy {
	x: usize, // infinite, yeah!
	y: usize
}

fn main () {
	let mut map = HashMap::<Xy, bool>::new();
	read_stdin(&mut map);
	part1();
	part2();
}

fn part1() {

}

fn part2() {

}

fn read_stdin(map: &mut HashMap<Xy, bool>) {

	let stdin = io::stdin();

	for (y, line) in stdin.lock().lines().enumerate() {
		let l = line.unwrap().to_string();

		for (x, c) in l.chars().enumerate() {
			match c {
				'.' => {map.insert(Xy {x: x, y: y}, true);}
				'#' => {map.insert(Xy {x: x, y: y}, false);}
				_   => {}
			}
		}
	}
}
