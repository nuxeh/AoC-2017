use std::fs::File;
use std::io::prelude::*;

fn main()
{
	let mut a = vec![0; 10];
	println!("{:?}", a);

	test_positions(10, 24);




	let filename = "input.txt";
	let filename = "test.txt";

	let mut f = File::open(filename).expect("file not found!");
	println!("read file: {}", filename);

	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("error!");
	let lines: Vec<_> = contents.split(|c| c == '\n').collect();

	for line in lines {
		if line.len() == 0 {continue}

		println!("{}", line);

		let fields: Vec<_> = line.split(": ").collect();
		for field in fields {
			print!("{} ", field);
		}
		println!("");

//			let n = s.parse::<i32>().unwrap();
	}




	compute();
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

fn compute()
{

}
