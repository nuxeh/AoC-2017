/* Advent of code */
/* day 24 */

use std::io;
use std::io::BufRead;

fn main () {
	let v = read_stdin();

	part1(&v);
	part2();
}

fn part1(v: &Vec<Vec<u32>>) {

	let bridges = vec![vec![]];

	for (startpos, block) in v {
		if block.contains(0) {
			walk(startpos, n, v);
			n += 1;
		}
	}
}

fn walk(s: u32, n: u32, blocks: &Vec<Vec<u32>>) {
	for (startpos, _) in v {
		walk(startpos, v);
	}
}

fn part2() {

}

fn read_stdin() -> Vec<Vec<u32>> {
	let std = io::stdin();
	let mut v = vec![];

	for l in std.lock().lines() {
		let l = l.unwrap();
		let s = l.split('/');

		let a: Vec<u32> = s.map(|a| a.parse::<u32>().unwrap()).collect();
		println!("{:?}", a);

		v.push(a.to_owned());
	}
	v
}
