/* Advent of code */
/* day 19 */

use std::fs::File;
use std::io::prelude::*;

fn main () {

	let test = true;

	let filename;
	if test {filename = "test.txt";} else {filename = "iput.txt";}

	let mut f = File::open(filename).expect("file not found");

	let mut input = String::new();
	f.read_to_string(&mut input)
		.expect("error reading file");

	let split = input.split(|c| c == '\n');

	let map = vec![vec![0; 200]; 200];

	part1();
	part2();
}

fn part1() {

}

fn part2() {

}
