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

		let mut n: i32 = 0;
		let mut r: char = '\0';
		if split.len() <= 2 {n = 0} else {
			let r2 = split[2].parse().unwrap_or('\0');
			match split[2].parse::<i32>() {
				Ok(v)  => n = v,
				Err(_) => r = r2
			}
			// map on Option and Result
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

	let r = i.0.to_string();
	let v = rs[&r];

	match i.3.as_ref() { // to slice

		"snd"	=>	{rs.insert("snd".to_string(), i.2);}
		"set"	=>	{rs.insert(r, i.2);}
		"add"	=>	{rs.insert(r, v + i.2);}
		"mul"	=>	{rs.insert(r, v * i.2);}
		"mod"	=>	{rs.insert(r, v % i.2);}

		_	=>	{println!("Unknown instruction!");}
	}
}

fn part1(p: &Vec<(char, char, i32, String)>, mut rs: &mut HashMap<String, i32>) {

	println!("{:?}", p);

	// rs["pc"] = 0; // doesn't work
	rs.insert("pc".to_string(), 0);

	loop {
		let pc = rs["pc"] as usize;

		inst(&p[pc], &mut rs);
	}

}

fn part2() {

}

// TODO: instruciton struct
