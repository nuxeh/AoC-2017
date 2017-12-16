use std::io;
use std::io::BufRead;

fn main () {

	let mut c: u8 = 64;
	let mut a: Vec<_> = vec![0; 16]
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

	let moves2: Vec<(String, u32, String, String, u32, u32)>;

	for m in moves {
		println!("{:?}", m);

		// Rust strings are UTF-8 and can't be indexed
		println!("{:?}", m.get(0..1).unwrap());

		let cmd = m.get(0..1).unwrap();

		match cmd {
			"s" => {
				let n: u32 = m.get(1..2).unwrap().parse().unwrap();
				moves2.push((cmd, n, "", "", 0, 0));
			}
			"x" => {

			}
			"p" => {

			}
			_ => {}
		}
	}
}
