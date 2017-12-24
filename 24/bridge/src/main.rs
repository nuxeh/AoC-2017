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

	let bridges = vec!{};

	for startpos in v {
		walk_combinations(startpos)
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
