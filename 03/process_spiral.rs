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

	enum Direction {
		Up,
		Down,
		Left,
		Right
	}

	let mut dir = Direction::Right;
	let mut next_dir: Direction;
	let mut next_pos: (usize, usize);

	for x in 0..10 {
		println!("{}", x);

		vec_2d[pos_2d.0][pos_2d.1] = x;

		match dir {
			Direction::Up		=> {
				next_dir = Direction::Left;
				next_pos = (pos_2d.0+1, pos_2d.1);
			},
			Direction::Down		=> {
				next_dir = Direction::Right;
				next_pos = (pos_2d.0-1, pos_2d.1);
			},
			Direction::Left		=> {
				next_dir = Direction::Down;
				next_pos = (pos_2d.0, pos_2d.1-1);
			},
			Direction::Right	=> {
				next_dir = Direction::Up;
				next_pos = (pos_2d.0, pos_2d.1+1);
			},
		}

		if vec_2d[next_pos.0][next_pos.1] == 0 {
			dir = next_dir;
			pos_2d = next_pos;
		}
	}

	println!("{:?}", vec_2d);
}
