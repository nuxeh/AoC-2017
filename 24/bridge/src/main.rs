/* Advent of code */
/* day 24 */

use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn main () {
	let v = read_stdin();

	part1(&v);
	part2();
}

fn part1(blocks: &Vec<Vec<u32>>) {

	let mut bridges: HashMap<usize, Vec<u32>> = HashMap::new();
	let mut n = 0;

	for (startpos, block) in blocks.iter().enumerate() {
		if block.contains(&0) {
			bridges.insert(startpos, vec![]);
			walk(startpos, n, blocks, &mut bridges);
			n += 1;
		}
	}
}

fn walk(s: usize,
	n: usize,
	blocks: &Vec<Vec<u32>>,
	mut bridges: &mut HashMap<usize, Vec<u32>>
	) {

	for (startpos, _) in blocks
	.iter()
	.enumerate()
	.filter(|a| a.0 != s)
	.filter(|a| a.1.contains(&0))
	{
		walk(startpos, n, blocks, &mut bridges);
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
