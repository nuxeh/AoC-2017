use std::iter;

enum Direction {
	Up,
	Down,
	Left,
	Right
}

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

	let mut dir = Direction::Right;
	let mut next_dir: Direction;
	let mut next_pos: (usize, usize);

	for x in 1..10 {

		vec_2d[pos_2d.0][pos_2d.1] = x;

		next_pos = get_next_pos(&pos_2d, &dir);

		match dir {
			Direction::Up		=> {
				next_dir = Direction::Left;
			},
			Direction::Down		=> {
				next_dir = Direction::Right;
			},
			Direction::Left		=> {
				next_dir = Direction::Down;
			},
			Direction::Right	=> {
				next_dir = Direction::Up;
			},
		}

		if vec_2d[next_pos.0][next_pos.1] == 0 {
			dir = next_dir;
			pos_2d = next_pos;
		}
	}

	print_space(&vec_2d);
}

fn get_next_pos (cur: &(usize, usize), dir: &Direction) -> (usize, usize) {
	match dir {
		&Direction::Up		=> {(cur.0-1, cur.1)},
		&Direction::Down	=> {(cur.0+1, cur.1)},
		&Direction::Left	=> {(cur.0, cur.1-1)},
		&Direction::Right	=> {(cur.0, cur.1+1)},
	}
}

fn print_space (vec: &Vec<Vec<i32>>) {
	for x in 0..vec.len() {
		println!("{:?}", vec[x]);
	}
}
