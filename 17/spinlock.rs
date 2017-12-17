fn main () {

	let test = true;
	let s; if test { s = 3 } else { s = 324 }

	let mut buf = vec![0];
	let mut p = 0;

	for i in 1..2018 {
		let l = buf.len();

		p = (p + s) % l;

//		println!("{} {}", p, l);

		buf.insert(p + 1, i);

		p = p + 1;

//		println!("{:?}", buf);
	}

	println!("{:?}", buf.get(p - 3 .. p + 4).unwrap());
}
