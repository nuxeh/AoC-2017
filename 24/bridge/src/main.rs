/* Advent of code */
/* day 24 */

use std::io;
use std::io::BufRead;

fn main () {
	read_stdin();

	part1();
	part2();
}

fn part1() {

}

fn part2() {

}

fn read_stdin() {
	let std = io::stdin();
//	let mut v = vec![];

	for l in std.lock().lines() {
		let l = l.unwrap();
		println!("{}", l);

		let s = l.split('/');

		let a: Vec<u32> = s.map(|a| a.parse::<u32>().unwrap()).collect();

		println!("{:?}", a);

//		v.push(s.map(|a| a.parse::<u32>()).to_owned());

	}
}
