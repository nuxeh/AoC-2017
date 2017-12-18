/* Advent of code */
/* day 18 */

use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn main () {

	let stdin = io::stdin();

	let mut p: Vec<(char, char, i64, String)> = vec![];

	let mut rs = HashMap::<String, i64>::new();

	for line in stdin.lock().lines() {
		let line = line.unwrap();
		let split: Vec<_> = line.split(" ").collect();

		let mut n: i64 = 0;
		let mut r: char = '\0';
		if split.len() <= 2 {n = 0} else {
			let r2 = split[2].parse().unwrap_or('\0');
			match split[2].parse::<i64>() {
				Ok(v)  => n = v,
				Err(_) => r = r2
			}
			// map on Option and Result, also "if let"
		}

		p.push((split[1].chars().nth(0).unwrap(),
			r,
			n,
			split[0].to_string()));
	}

	part1(&mut p, &mut rs);
	part2();
}

fn inst(i: &(char, char, i64, String), rs: &mut HashMap<String, i64>) {

	let pc = rs["pc"];
	let r = i.0.to_string();

	let rv = *rs.entry(r.to_owned()).or_insert(0);

	let v;
	match i.1 {
		'\0' => v = i.2,
		_    => v = *rs.entry(i.1.to_string()).or_insert(0)
	}

	let mut jump = false;

	match i.3.as_ref() { // to slice

		"snd" => {rs.insert("snd".to_string(), rv);} // no brackets needed
		"set" => {rs.insert(r, v);}
		"add" => {rs.insert(r, rv + v);}
		"mul" => {rs.insert(r, rv * v);}
		"mod" => {rs.insert(r, rv % v);}
		"rcv" => {if v != 0 {println!("freq is {}", rs["snd"])};}
		"jgz" => {if v > 0  {
			jump = true;
			rs.insert("pc".to_string(), pc + i.2)} else {None};
		}

		_     => {println!("Unknown instruction {} !", i.3);}
	}

	if !jump {rs.insert("pc".to_string(), pc + 1);}
}

fn part1(p: &Vec<(char, char, i64, String)>, mut rs: &mut HashMap<String, i64>) {

	println!("{:?}", p);

	// rs["pc"] = 0; // doesn't work
	rs.insert("pc".to_string(), 0);

	loop {
		let pc = rs["pc"] as usize;

		println!("{:?}", p[pc]);

		inst(&p[pc], &mut rs);

		println!("{:?}", rs);

		if rs["pc"] >= p.len() as i64 {break;}
	}

	println!("{:?}", rs);
}

fn part2() {

}

// TODO: instruciton struct
