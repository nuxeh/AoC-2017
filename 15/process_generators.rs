fn main ()
{
	let mut a: u64 = 65;
	let mut b: u64 = 8921;

	a = 289;
	b = 629;

	let f_a = 16807;
	let f_b = 48271;

	let d = 2147483647;

	let mut count = 0;

	for i in 0..40000000 {
		a = (a * f_a) % d;
		b = (b * f_b) % d;

		if a as u16 == b as u16 {
			println!("match: {:b} {:b}", a as u16, b as u16);
			//println!("{:b}", a);
			//println!("{:b}", b);
			count += 1;
		}

//		println!("{} {}", a, b)

	}

	println!("{}", count);
}

