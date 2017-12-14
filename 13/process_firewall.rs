fn main()
{
	let mut a = vec![0; 10];
	println!("{:?}", a)
}

fn test_positions(_r: u32, _n: u32)
{
	let mut p = 0;
	let mut d = 1;
	let mut r = 1;

	for n in 0.._n {
		let mut s = "[";
		for x in 0.._r {
			if (x == p) {
				s = s + "*";
			} else {
				s = s + " ";
			}
		}
		println!("{}", s + "]" + p);

		if		(d > 0 && p == r - 1)	{d *= -1}
		else if		(d < 0 && p == 0)	{d *= -1}
		p += d;
	}

}
