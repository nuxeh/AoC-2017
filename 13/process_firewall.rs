fn main()
{
	let mut a = vec![0; 10];
	println!("{:?}", a)
}

fn test_positions(r, n)
{
	let mut p = 0;
	let mut d = 1;
	let mut r = 1;

	for (n=0; n<max; n++) {
		let mut s = "[";
		for (x=0; x<l; x++) {
			if (x == p)
				s = concat!(s, "*");
			else
				s = concat!(s, " ");
		}
		println!(concat(s, "]", p));

		if		(d > 0 && p == r - 1)	{d *= -1}
		else if		(d < 0 && p == 0)	{d *= -1}
		p += d;
	}

}
