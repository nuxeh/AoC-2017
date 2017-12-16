fn main () {

	let mut c: u8 = 64;
	let mut a: Vec<_> = vec![0; 16]
		.iter().map(|_| {c += 1; c as char}).collect();
	println!("{:?}", a);
}
