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

	for l in std.lock().lines() {
		let l = l.unwrap();
		println!("{}", l);

	}
}
