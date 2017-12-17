#![feature(slice_rotate)]
// rustup install nightly
// cargo +nightly build

extern crate pbr;

use pbr::ProgressBar;

fn main () {
	let test = false;
	let s; if test { s = 3 } else { s = 324 }

	part1(s);
	part2(s);
}

fn part1(s: usize){

	let part = 1;
	let t; if part == 1 { t = 2018 } else { t = 50000001 }

	let mut pb = ProgressBar::new(t-1);
	pb.format("|▓░░|");

	let mut buf = vec![0];
	let mut buf2 = vec![0];
	let mut p = 0;

	for i in 1..t {
		let l = buf.len();

		/* method 1 */
		p = (p + s) % l;
		buf.insert(p + 1, i);
		p = p + 1;

		/* method 2 */
		buf2.rotate(s % l);
		buf2.push(i);

		pb.inc();

//		println!("{:?}", buf);
	}

	println!("");
	println!("{:?}", buf.get(p - 3 .. p + 4).unwrap());
	println!("method 1 final value is {}", buf[p+1]);

	println!("method 2 final value is {}", buf2[0]);
}

fn part2(s: usize) {

	/* Part 2 */
	/* buf[1] is only changed when write position is 1
	   therefore buffer contents don't matter, just the
	   index at which buf[1] is written */

	let mut p = 0;
	let mut last_p0 = 0;
	let mut l = 1;

	for i in 1..50000000 {
		p = (p + s) % l;

		if p == 0 {
			println!("p = 0 at {}\t difference {}", i, i - last_p0);
			last_p0 = i;
		}

		l += 1;
		p = p + 1;
	}
}
