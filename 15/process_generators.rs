fn main ()
{
	let part = 2;
	let test = false;

	let mut a: u64 = 65;
	let mut b: u64 = 8921;

	if !test {
		a = 289;
		b = 629;
		println!("using real data");
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
	println!("target: {}", target);

	let mut list_a = vec![];
	let mut list_b = vec![];

	if part == 1 {
		for _i in 0..target {
			a = (a * f_a) % d;
			b = (b * f_b) % d;

			if a as u16 == b as u16 {
				println!("match: {:b} {:b}", a as u16, b as u16);
				count += 1;
			}

			if part == 0 {
				println!("{} {}", a, b)
			}

		}
	} else {
		loop {
			a = (a * f_a) % d;
			b = (b * f_b) % d;

			if a % 4 == 0 { list_a.push(a); }
			if b % 8 == 0 { list_b.push(b); }

			if list_a.len() >= target && list_b.len() >= target { break; }
		}

		if list_a.len() > list_b.len() {
			println!("list a is bigger");
		} else {
			println!("list b is bigger");
		}

		count = 0;
		for i in 0..target {
			if list_a[i] as u16 == list_b[i] as u16 {
				println!("match: {:b} {:b}", list_a[i] as u16, list_b[i] as u16);
				count += 1;
			}
		}
	}

	println!("{} matches", count);
}

