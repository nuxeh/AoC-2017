use std::iter;

fn main() {

	let w_space = 11; // Must be even

	let mut vec: Vec<i32> = Vec::with_capacity(w_space*w_space);

	vec.extend(iter::repeat(0).take(w_space*w_space));

	let mut vec_2d = vec![vec![0; w_space]; w_space];

	let mut pos = ((w_space-1)/2, (w_space-1)/2); // Start from centre coordinate
	let mut pos_2d = ((w_space*w_space)-1) / 2;
	let mut index = 1;

	vec[pos_2d] = index;
	vec_2d[pos.0][pos.1] = index;

	println!("{:?}", vec);
	println!("{:?}", vec_2d);

}
