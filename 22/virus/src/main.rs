/* Advent of code */
/* day 22 */

use std::io;
use std::io::BufRead;
use std::collections::HashMap;

struct Xy {
	x: u64, // infinite, yeah!
	y: u64
}

fn main () {
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
			map.insert(Xy {x: x, y: y});
		}
	}
}
