/* Advent of code */
/* day 24 */

use std::io;
use std::io::BufRead;

fn main () {

	let debug = false;
//	let debug = true;
	let blocks = read_stdin();

	println!("read {} blocks from standard input",
		 blocks.clone().iter().count());

	let mut bridges: Vec<Vec<usize>> = Vec::new();

	for (startpos, block) in blocks.iter().enumerate() {
		match block.binary_search(&0) {
			Ok(e)  => {
					walk(startpos,
					     block[e^1] as u8,
					     &blocks,
					     Vec::new(),
					     &mut bridges);
			}
			Err(_) => {}
		}
	}

	println!("found {} bridges", bridges.iter().count());

	part1(&blocks, &bridges, debug);
	part2(&blocks, &bridges, debug);
}

fn walk(n: usize,
	e: u8,
	blocks: &Vec<Vec<u8>>,
	mut crumbs: Vec<usize>,
	mut bridges: &mut Vec<Vec<usize>>,
	) {

	crumbs.push(n);
	bridges.push(crumbs.clone());

	for (i, block) in blocks
	.iter()
	.enumerate()
	.filter(|a| ! crumbs.contains(&a.0))
	.filter(|a| a.1.contains(&e))
	{
		match block.binary_search(&e) {
			Ok(ei)  => {
					walk(i,
					     block[ei^1] as u8,
					     blocks,
					     crumbs.clone(),
					     &mut bridges);
			}
			Err(s) => {println!("{:?} failed {} = {}", block, e, s);}
		}
	}
}

fn part1(blocks: &Vec<Vec<u8>>, bridges: &Vec<Vec<usize>>, debug: bool) {

	let mut strengths: Vec<u32> = Vec::new();

	for b in bridges {
		let mut sum: u32 = 0;
		for &block in b {
			if debug {print!("{:?} ", blocks[block])};
			sum += blocks[block]
				.iter()
				.fold(0, |acc,&a| acc + a as u32);
		}
		if debug {println!("\n{}", sum);}
		strengths.push(sum);
	}

	strengths.sort();
	println!("strongest has strength: {:?}", strengths.last().unwrap());
}

fn part2(blocks: &Vec<Vec<u8>>, bridges: &Vec<Vec<usize>>, debug: bool) {

	let mut lengths: Vec<(u32, &Vec<usize>)> = bridges
		.iter()
		.map(|a| (a.iter().count() as u32, a))
		.collect();

	lengths.sort();

	let max_length = lengths.last().unwrap().0;

	println!("longest bridge length: {}", max_length);

	let max_length: Vec<Vec<usize>> = lengths
		.iter()
		.filter(|a| a.0 == max_length)
		.map(|a| a.1.to_owned())
		.collect();

	part1(blocks, &max_length, debug);
}

fn read_stdin() -> Vec<Vec<u8>> {
	let std = io::stdin();
	let mut v = vec![];

	for l in std.lock().lines() {
		let l = l.unwrap();
		let s = l.split('/');

		let mut a: Vec<u8> = s.map(|a| a.parse::<u8>().unwrap()).collect();
//		println!("{:?}", a);

		a.sort();

		v.push(a.to_owned());
	}

	v
}
