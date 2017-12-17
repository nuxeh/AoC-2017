#![feature(slice_rotate)]
// rustup install nightly
// cargo +nightly build

extern crate pbr;

use pbr::ProgressBar;

fn main () {

	let test = false;
	let part = 2;
	let s; if test { s = 3 } else { s = 324 }
	let t; if part == 1 { t = 2018 } else { t = 50000001 }

	let mut pb = ProgressBar::new(t);
	pb.format("|▓░░|");

	let mut buf = vec![0];
	let mut p = 0;

//	let mut after_zero = vec![];

	let mut last_p0 = 0;
	let mut l = 1;

	for i in 1..50000000 {
//		let l = buf.len();

		p = (p + s) % l;

//		buf.rotate(s % l);

//		println!("{} {}", p, l);

		if p == 0 {
			println!("p = 0 at {}\t difference {}", i, i - last_p0);
			last_p0 = i;
		}

//		buf.insert(p + 1, i);
//		buf.push(i);

		l += 1;
		p = p + 1;

//		println!("{:?}", buf);

//		after_zero.push(buf[1]);

//		if i % 1000 == 0 { pb.inc(); }
	}

//	println!("{:?}", buf.get(p - 3 .. p + 4).unwrap());
//	println!("{:?}", buf);
//	println!("final value is {}", buf[0]);
//	println!("value after 0 is {}", buf[1]);
//	println!("{:?}", after_zero);

}
