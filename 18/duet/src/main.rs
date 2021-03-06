/* Advent of code */
/* day 18 */

use std::io;
use std::io::BufRead;
use std::collections::HashMap;
//use std::thread;
use std::sync::mpsc::channel;

fn main () {

	let stdin = io::stdin();

	let mut p: Vec<(char, char, i64, i32, String)> = vec![];

	let mut rs = HashMap::<String, i64>::new();
	let mut rs2_0 = HashMap::<String, i64>::new();
	let mut rs2_1 = HashMap::<String, i64>::new();
	rs2_0.insert("p".to_string(), 0);
	rs2_1.insert("p".to_string(), 1);

	for line in stdin.lock().lines() {
		let line = line.unwrap();
		let split: Vec<_> = line.split(" ").collect();

		let rv;
		let or;
		match split[1].parse::<i32>() {
			Ok(v)  => {rv = v; or = '\0';}
			Err(_) => {rv = 0; or = split[1].chars().nth(0).unwrap();}
		}

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

		p.push((or,
			r,
			n,
			rv,
			split[0].to_string()));
	}

	part1(&mut p, &mut rs);
	println!("last written frequency is {}", rs["answer"]);
	println!("part 2 ............");
	part2(&mut p, &mut rs2_0, &mut rs2_1);
}

fn inst(i: &(char, char, i64, i32, String), rs: &mut HashMap<String, i64>) -> i8 {

	let mut ret = 0;

	let pc = rs["pc"];
	let r = i.0.to_string();

	let rv;
	if i.0 == '\0' {
		rv = i.3 as i64;
	} else {
		rv = *rs.entry(r.to_owned()).or_insert(0);
	}

	let v;
	match i.1 {
		'\0' => v = i.2,
		_    => v = *rs.entry(i.1.to_string()).or_insert(0)
	}

	let mut jump = false;

	match i.4.as_ref() { // to slice

		"snd" => {
				rs.insert("snd".to_string(), rv);
				ret = -1;
			}

		"rcv" => {
				if rv != 0 {
					if rs.contains_key("snd") {
//						println!("rcv is {}", rs["snd"]);
						let r = rs["snd"];
						rs.insert("answer".to_string(), r);
					}
				}

				if rs["wait"] == 0 {
					let rcv = *rs.entry("rcv".to_string()).or_insert(0);
					rs.insert(r, rcv);
					rs.insert("wait".to_string(), 1);
				} else {
					return -2;
				}
			}

		"set" => {rs.insert(r, v);}
		"add" => {rs.insert(r, rv + v);}
		"mul" => {rs.insert(r, rv * v);}
		"mod" => {rs.insert(r, rv % v);}

		"jgz" => {if rv > 0  {
			jump = true;
			rs.insert("pc".to_string(), pc + v)} else {None};
		}

		_     => {println!("Unknown instruction {} !", i.4);}
	}

	if !jump {rs.insert("pc".to_string(), pc + 1);}

	ret
}

fn part1(p: &Vec<(char, char, i64, i32, String)>, mut rs: &mut HashMap<String, i64>) {

//	println!("{:?}", p);

	// rs["pc"] = 0; // doesn't work
	rs.insert("pc".to_string(), 0);
	rs.insert("wait".to_string(), 0);

	loop {
		let pc = rs["pc"] as usize;

//		println!("{:?}", p[pc]);

		if inst(&p[pc], &mut rs) == -2 {break;}

//		println!("{:?}", rs);

		if rs["pc"] >= p.len() as i64 {break;}
	}

//	println!("{:?}", rs);
}

fn part2(p: &Vec<(char, char, i64, i32, String)>,
	 mut rs: &mut HashMap<String, i64>,
	 mut rs1: &mut HashMap<String, i64>) {

	let (tx, rx) = channel();
	let (tx2, rx2) = channel();

	rs.insert("pc".to_string(), 0);
	rs1.insert("pc".to_string(), 0);
	rs.insert("wait".to_string(), 1);
	rs1.insert("wait".to_string(), 1);

//	run(&mut p, &mut rs);

	let mut send_count_1 = 0;

	loop {
		let pc = rs["pc"] as usize;
		let pc1 = rs1["pc"] as usize;

		match inst(&p[pc], &mut rs) {
			-1 => {
				let s = rs["snd"];
				let _ = tx.send(s);
//				println!("0 sending {}", s);
			}
			-2 => {
				let m = rx2.try_recv();
				if ! m.is_err() {
					let r = m.unwrap();
//					println!("0 received {}", r);
					rs.insert("rcv".to_string(), r);
					rs.insert("wait".to_string(), 0);
				}
			}
			_  => {}
		}

//		println!("0: {:?}", rs);

		match inst(&p[pc1], &mut rs1) {
			-1 => {
				let s = rs1["snd"];
				let _ = tx2.send(s);
				send_count_1 += 1;
				println!("1 sending {} [{}]", s, send_count_1);
			}
			-2 => {
				let m = rx.try_recv();
				if ! m.is_err() {
					let r = m.unwrap();
//					println!("1 received {}", r);
					rs1.insert("rcv".to_string(), r);
					rs1.insert("wait".to_string(), 0);
				}
			}
			_  => {}
		}

//		println!("1: {:?}", rs1);

		if rs["pc"] >= p.len() as i64 {break;}
		if rs1["pc"] >= p.len() as i64 {break;}
	}

}

fn run(p: &Vec<(char, char, i64, i32, String)>, mut rs: &mut HashMap<String, i64>) {

	rs.insert("pc".to_string(), 0);

	loop {
		let pc = rs["pc"] as usize;
		inst(&p[pc], &mut rs);
		if rs["pc"] >= p.len() as i64 {break;}
	}

}

// TODO: instruciton struct
