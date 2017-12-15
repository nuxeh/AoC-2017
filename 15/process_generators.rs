fn main ()
{
	let mut part = 0;

	let mut a: u64 = 65;
	let mut b: u64 = 8921;

	if part > 0 {
		a = 289;
		b = 629;
	}

	let f_a = 16807;
	let f_b = 48271;

	let d = 2147483647;

	let mut count = 0;

	let mut target = 5;

	if part == 0 {
		target = 5;
	} else if part == 1 {
		target = 40000000;
	} else if part == 2 {
		target = 5000000;
	}

	let mut list_a = vec![];
	let mut list_b = vec![];

	for i in 0..target {
		a = (a * f_a) % d;
		b = (b * f_b) % d;

		if a % 4 == 0 { list_a.push(a); }
		if b % 8 == 0 { list_b.push(b); }

		if a as u16 == b as u16 {
			println!("match: {:b} {:b}", a as u16, b as u16);
			count += 1;
		}

		if part == 0 {
			println!("{} {}", a, b)
		}

	}

	println!("{} matches", count);

	if len(list_a) > len(list_b {
		let l = len(list_b);
		println!("list a is bigger");
	} else {
		let l = len(list_a);
		println!("list b is bigger");
	}


	for (i, a) in list_a.enumerate() {


	}
}

