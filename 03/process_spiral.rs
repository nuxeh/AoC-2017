use std::iter;

fn main() {

	let w_space = 11; // Must be even

	let mut vec: Vec<i32> = Vec::with_capacity(w_space*w_space);
	vec.extend(iter::repeat(0).take(w_space*w_space));

	let mut vec_2d = vec![vec![0; w_space]; w_space];

	let mut pos = ((w_space*w_space)-1) / 2;
	let mut pos_2d = ((w_space-1)/2, (w_space-1)/2); // Start from centre coordinate

	let mut index = 1;

	vec[pos] = index;
	vec_2d[pos_2d.0][pos_2d.1] = index;

	println!("{:?}", vec);
	println!("{:?}", vec_2d);

	enum direction {
		up,
		down,
		left,
		right
	}


	let mut dir = direction::up;

	for x in 0..10 {
		println!("{}", x);

		vec_2d[pos_2d.0][pos_2d.1] = x;

//		pos_2d
	}
}
