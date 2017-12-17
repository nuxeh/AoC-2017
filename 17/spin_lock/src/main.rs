#![feature(slice_rotate)]
// rustup install nightly
// cargo +nightly build

extern crate pbr;

use pbr::ProgressBar;

fn main () {

	let test = true;
	let part = 1;
	let s; if test { s = 3 } else { s = 324 }
	let t; if part == 1 { t = 2018 } else { t = 50000001 }

	let mut pb = ProgressBar::new(t);
	pb.format("|▓░░|");

	let mut buf = vec![0];
	let mut p = 0;

	for i in 1..t {
		let l = buf.len();

//		p = (p + s) % l;
asd
		buf.rotate(s % l);

//		println!("{} {}", p, l);

//		buf.insert(p + 1, i);
		buf.push(i);

//		p = p + 1;

		println!("{:?}", buf);

		if i % 1000 == 0 { pb.inc(); }
	}

//	println!("{:?}", buf.get(p - 3 .. p + 4).unwrap());
	println!("final value is {}", buf[0]);
	println!("value after 0 is {}", buf[buf.binary_search(&8).unwrap()+1]);

}
