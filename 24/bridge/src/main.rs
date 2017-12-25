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

fn part1(blocks: &Vec<Vec<u8>>) {

	let mut bridges: HashMap<usize, Vec<usize>> = HashMap::new();

	for (startpos, block) in blocks.iter().enumerate() {
		match block.binary_search(&0) {
			Ok(e)  => {
					walk(startpos,
						startpos,
						block[e^1] as u8,
						blocks,
						&mut bridges);
			}
			Err(_) => {}
		}
	}

	for b in bridges {
		println!("{:?}", b);
		for block in b.1 {
			print!("{:?} - ", blocks[block]);
		}
	}
}

fn walk(s: usize,
	n: usize,
	e: u8,
	blocks: &Vec<Vec<u8>>,
	mut bridges: &mut HashMap<usize, Vec<usize>>
	) {

	match bridges.entry(s) {
		Entry::Occupied(mut e) => {e.get_mut().push(n);}
		Entry::Vacant(e)       => {e.insert(vec![n]);}
	}

	let b_cur: Vec<usize> = bridges[&s].clone();

	for (i, block) in blocks
	.iter()
	.enumerate()
	.filter(|a| a.0 != s)
	.filter(|a| ! b_cur.contains(&a.0))
	.filter(|a| a.1.contains(&e))
	{
		match block.binary_search(&e) {
			Ok(e)  => {
					walk(s,
						i,
						block[e^1] as u8,
						blocks,
						&mut bridges);
			}
			Err(_) => {}
		}
	}
}

fn part2() {

}

fn read_stdin() -> Vec<Vec<u8>> {
	let std = io::stdin();
	let mut v = vec![];

	for l in std.lock().lines() {
		let l = l.unwrap();
		let s = l.split('/');

		let a: Vec<u8> = s.map(|a| a.parse::<u8>().unwrap()).collect();
		println!("{:?}", a);

		v.push(a.to_owned());
	}
	v
}
