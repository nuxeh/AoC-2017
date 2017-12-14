fn main()
{
	let mut a = vec![0; 10];
	println!("{:?}", a);

	test_positions(10, 24);
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
