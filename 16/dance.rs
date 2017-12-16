use std::io;
use std::io::BufRead;

fn main () {

	let test = 1;

	let l;
	if test == 1 { l = 5; } else { l = 16; }

	let mut c: u8 = 64;
	let mut a: Vec<_> = vec![0; l]
		.iter().map(|_| {c += 1; c as char}).collect();
	println!("{:?}", a);

	let stdin = io::stdin();

	let mut moves = vec![];

	for line in stdin.lock().lines() {
		let line = line.unwrap();
		let split: Vec<_> = line.split(",").collect();

		for m in split {
			println!("{:?}", m);
			moves.push(m.to_owned());
		}
	}

	let mut moves2: Vec<(char, u32, char, char, u32, u32)> = vec![];

	for m in moves {
		println!("{:?}", m);

		// Rust strings are UTF-8 and can't be indexed
		println!("{:?}", m.get(0..1).unwrap());

		let cmd = m.get(0..1).unwrap().chars().nth(0).unwrap();

		let split: Vec<_> = m.get(1..).unwrap()
			.split("/").collect();

		match cmd {
			's' => {
				let n: u32 = m.get(1..2).unwrap().parse()
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

	for m in moves2 {
		println!("{:?}", m);
	}

	swap(&mut a, 2, 4);
	println!("{:?}", a);
	swap_a(&mut a, 'A', 'B');
	println!("{:?}", a);
	spin(&mut a, 2);
	println!("{:?}", a);
}

fn spin(a: &mut Vec<char>, _n: u32) {

	let n = _n as usize;

	let x = a.get(0..n).unwrap().to_owned();
	let y = a.get(n..).unwrap().to_owned();

	println!("{:?}", x);
	println!("{:?}", y);

	for (i, c) in y.iter().enumerate() {
		println!("{} {} -> {}", i, c, i);
		a[i] = *c;
	}

	for (i, c) in x.iter().enumerate() {
		println!("{} {}", i, n);
		let l = i+(a.len() - n);
		println!("{} {} -> {}", i, c, l);
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

	let f = a.iter().position(|&a| a == cf).unwrap();
	let t = a.iter().position(|&a| a == ct).unwrap();

	let temp = a[f];
	a[f] = a[t];
	a[t] = temp;
}
