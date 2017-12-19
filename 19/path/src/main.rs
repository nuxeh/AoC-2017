/* Advent of code */
/* day 19 */

use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::io::BufRead;


fn main () {

	let stdin = io::stdin();

	let mut map: Vec<Vec<&str>> = vec![];

	for line in stdin.lock().lines() {
		let l = line.unwrap().to_string();
		let split: Vec<_> = l.split("").collect();

		/* std::vec::Vec<&str> */

		println!("{:?}", split);

//		let slice: &[&str] = &split;
//		println!("{:?}", slice);

		let mut slice2: Vec<&str> = vec![];// = split.to_vec();

//		split.clone_into(slice2);
		slice2.clone_from(&split).to_owned();

		map.push(slice2);
	}

//	let map2: Vec<_> = stdin.lock().lines().unwrap().split("").collect();

	part1();
	part2();
}

fn part1() {

}

fn part2() {

}

fn read_from_file(filename: &String) {

	let mut f = File::open(filename).expect("file not found");

	let mut input = String::new();
	f.read_to_string(&mut input)
		.expect("error reading file");

	let _lines = input.split(|c| c == '\n');

}
