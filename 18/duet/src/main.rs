/* Advent of code */
/* day 18 */

use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn main () {

	let stdin = io::stdin();

	let mut p: Vec<(char, char, i32, String)> = vec![];

	let mut rs = HashMap::<String, i32>::new();

	for line in stdin.lock().lines() {
		let line = line.unwrap();
		let split: Vec<_> = line.split(" ").collect();

		let n: i32;
		let mut r: char = '\0';
		if split.len() <= 2 {n = 0} else {
			n = split[2].parse().unwrap_or(0);
			r = split[2].parse().unwrap_or('\0');
		}

		p.push((split[1].chars().nth(0).unwrap(),
			r,
			n,
			split[0].to_string()));
	}

	part1(&mut p, &mut rs);
	part2();
}

fn inst(i: &(char, char, i32, String), rs: &mut HashMap<String, i32>) {

}

fn part1(p: &Vec<(char, char, i32, String)>, mut rs: &mut HashMap<String, i32>) {

	println!("{:?}", p);

	for i in p {
		println!("{:?}", i);
		inst(i, &mut rs);
	}

}

fn part2() {

}
