/* Advent of code */
/* day 24 */

use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn main () {
	let v = read_stdin();

	part1(&v);
	part2();
}

fn part1(blocks: &Vec<Vec<u32>>) {

	let mut bridges: HashMap<usize, Vec<usize>> = HashMap::new();
	let mut elem = 0;

	for (startpos, block) in blocks.iter().enumerate() {
		if block.contains(&0) {
			walk(startpos, startpos, elem, blocks, &mut bridges);
		}
	}
}

fn walk(s: usize,
	n: usize,
	e: usize,
	blocks: &Vec<Vec<u32>>,
	mut bridges: &mut HashMap<usize, Vec<usize>>
	) {

	match bridges.entry(s) {
		Entry::Occupied(mut e) => {e.get_mut().push(n);}
		Entry::Vacant(e)       => {e.insert(vec![n]);}
	}

	let mut e = 0;

	for (startpos, _) in blocks
	.iter()
	.enumerate()
	.filter(|a| a.0 != s)
	.filter(|a| a.1.contains(&0))
	{
		walk(startpos, n, e, blocks, &mut bridges);
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
