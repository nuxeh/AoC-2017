use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;

fn main()
{
	test_positions(10, 24);



	// TODO: env reading, stdin
	let filename = "input.txt";
//	let filename = "test.txt";

	let mut f = File::open(filename).expect("file not found!");
	println!("read file: {}", filename);

	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("error!");
	let lines: Vec<_> = contents.split(|c| c == '\n').collect();

	let mut ranges = HashMap::<u32, u32>::new();
	let mut max_depth = 0;

	for line in lines {
		if line.len() == 0 {continue}

		println!("{}", line);

		let fields: Vec<_> = line.split(": ").collect();
		for field in &fields {
			print!("{} ", field);
		}
		ranges.insert(fields[0].parse().unwrap(), fields[1].parse().unwrap());
		println!("{:?}", ranges);

		let d:u32  = fields[0].parse().unwrap();
		if d > max_depth {max_depth = d}
	}

	println!("max depth: {}", max_depth);

	let mut delay = 0;
	loop {
		if compute(&ranges, max_depth, delay) {break}
		delay += 1;
	}
	println!("delay {} succeeded", delay);
}

fn test_positions(_r: u32, _n: u32)
{
	let mut p: i32 = 0;
	let mut d: i32 = 1;

	for n in 0.._n {
		let mut s = "[".to_string();
		for x in 0.._r {
			if x == p as u32 {
				s = s + "*";
			} else {
				s = s + " ";
			}
		}

		let test = n % ((_r - 1) * 2);

		println!("{} {} {}", s + "]", p, test);

		if		d > 0 && p == _r as i32 - 1	{d *= -1}
		else if		d < 0 && p == 0			{d *= -1}
		p += d;
	}
}

fn pos_n(r: u32, n: u32) -> u32 {n % ((r - 1) * 2)}

fn compute(ranges: &HashMap<u32, u32>, max_depth: u32, delay: u32) -> bool
{
	for i in 0..max_depth+1 {
//		println!("{}", i);
		if !ranges.contains_key(&i) {continue}
		if pos_n(ranges[&i], i+delay) == 0 {
//			println!("collision at {}", i);
			return false
		}
	}
	true
}

fn map_severity()
{

}
