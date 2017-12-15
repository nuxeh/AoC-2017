fn main ()
{
	let mut a: u64 = 65;
	let mut b: u64 = 8921;

	let f_a = 16807;
	let f_b = 48271;

	let d = 2147483647;

	for i in 0..5 {
		a = (a * f_a) % d;
		b = (b * f_b) % d;

		println!("{} {}", a, b)

	}
}

