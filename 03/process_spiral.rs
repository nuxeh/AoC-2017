use std::iter;

#[derive(Debug)]
enum Direction {
	Up,
	Down,
	Left,
	Right
}

// TODO debug macro
//macro_rules!

fn main() {

	let w_space = 11; // Must be odd
	let mut pos = ((w_space-1)/2, (w_space-1)/2); // Start from centre coordinate

	let mut vec = vec![vec![0; w_space]; w_space];

	let mut dir = Direction::Right;
	let mut next_dir: Direction;
	let mut next_pos: (usize, usize);

	for x in 1..121 {
		vec[pos.0][pos.1] = x;

		pos = get_next_pos(&pos, &dir);
		next_dir = get_next_dir(&dir);
		next_pos = get_next_pos(&pos, &next_dir);

		println!("{}", vec[next_pos.0][next_pos.1]);
		println!("{:?}", dir);

		if vec[next_pos.0][next_pos.1] == 0 {
			dir = next_dir;
		}

		println!("{:?}", dir);

		print_space(&vec);
		println!("");
	}

	print_space(&vec);
}

fn get_next_dir (cur: &Direction) -> Direction {
	match cur {
		&Direction::Up		=> {Direction::Left},
		&Direction::Down	=> {Direction::Right},
		&Direction::Left	=> {Direction::Down},
		&Direction::Right	=> {Direction::Up},
	}
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
