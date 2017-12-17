fn main () {

	let test = true;
	let s; if test { s = 3 } else { s = 324 }

	let mut buf = vec![0];
	let mut p = 0;

	println!("{:?}", buf);

	for i in 1..2018 {
		let l = buf.len();

		p = (p + s) % l;

		println!("{} {}", p, l);

		if p == l-1 {
			buf.push(i);
		} else {
			buf[p+1] = i;
		}

		println!("{:?}", buf);
	}
}
