use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn main () {

	let test = 0;

	let l;
	if test == 1 { l = 5; } else { l = 16; }

	let mut c: u8 = 96;
	let mut a: Vec<_> = vec![0; l]
		.iter().map(|_| {c += 1; c as char}).collect();
	println!("{:?}", a);

	let stdin = io::stdin();

	let mut moves = vec![];

	for line in stdin.lock().lines() {
		let line = line.unwrap();
		let split: Vec<_> = line.split(",").collect();

		for m in split {
//			println!("{:?}", m);
			moves.push(m.to_owned());
		}
	}

	let mut moves2: Vec<(char, u32, char, char, u32, u32)> = vec![];

	for m in moves.iter() { // without iter gives error due to below loop
//		println!("{:?}", m);

		// Rust strings are UTF-8 and can't be indexed
//		println!("{:?}", m.get(0..1).unwrap());

		let cmd = m.get(0..1).unwrap().chars().nth(0).unwrap();

		let split: Vec<_> = m.get(1..).unwrap()
			.split("/").collect();

		match cmd {
			's' => {
				let n: u32 = m.get(1..).unwrap().parse()
					.unwrap_or(0);
				moves2.push((cmd, n, '\0', '\0', 0, 0));
			}
			'x' => {
				let f: u32 = split[0].parse()
					.unwrap_or(0);
				let t: u32 = split[1].parse()
					.unwrap_or(0);
				moves2.push((cmd, 0, '\0', '\0', f, t));
			}
			'p' => {
				let f = split[0].chars().nth(0).unwrap_or('\0');
				let t = split[1].chars().nth(0).unwrap_or('\0');
				moves2.push((cmd, 0, f, t, 0, 0));
			}
			_ => {}
		}
	}

	println!("{:?}", a);

	let mut seen = HashMap::<String, u32>::new();
	let mut count = 0;

	loop {
		for (_i, m) in moves2.iter().enumerate() {
	//		println!("{:?}", moves[_i]);
	//		println!("{:?}", m);
			match m.0 {
				's' => {spin(&mut a, m.1)}
				'x' => {swap(&mut a, m.4 as usize, m.5 as usize)}
				'p' => {swap_a(&mut a, m.2, m.3)}
				_   => {}
			}
	//		println!("-> {:?}", a);
		}

		let st: String = a.iter().cloned().collect();
		if seen.contains_key(&st) { break; }
		seen.insert(st.to_owned(), count);

		if count == 1000 { break; }

		count += 1;
	}

//	swap(&mut a, 2, 4);
//	println!("{:?}", a);
//	swap_a(&mut a, 'A', 'B');
//	println!("{:?}", a);
//	spin(&mut a, 2);

	println!("{:?}", a);

	println!("{}", count);

	for s in &seen {
		println!("{:?}", s);
//		let c = seen.get(s.0).unwrap();

		if *s.1 == 0 { println!("first dance: {}", s.0); }

		if *s.1 == 1000 % (count-2) {
			println!("1000th dance: {}", s.0);
		}

		if *s.1 == 1000000000 % (count-2) {
			println!("last dance: {}", s.0);
		}

	}
}

fn spin(a: &mut Vec<char>, _n: u32) {

	let n = a.len() - _n as usize;

	let x = a.get(0..n).unwrap().to_owned();
	let y = a.get(n..).unwrap().to_owned();

//	println!("{:?}", x);
//	println!("{:?}", y);

	for (i, c) in y.iter().enumerate() {
//		println!("{} {} -> {}", i, c, i);
		a[i] = *c;
	}

	for (i, c) in x.iter().enumerate() {
		let l = i+(a.len() - n);
//		println!("{} {} -> {}", i, c, l);
		a[l] = *c;
	}

	// TODO: use rotate()
}

fn swap(a: &mut Vec<char>, f: usize, t: usize) {

	let temp = a[f];
	a[f] = a[t];
	a[t] = temp;
}

fn swap_a(a: &mut Vec<char>, cf: char, ct: char) {

	let f = a.iter().position(|&a| a == cf).unwrap_or(0);
	let t = a.iter().position(|&a| a == ct).unwrap_or(0);

	let temp = a[f];
	a[f] = a[t];
	a[t] = temp;
}
