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

	let target = 1024;
//	let target = 312051;

	let w_space = (target as f64).sqrt() as usize + 2;

	let port = ((w_space)/2, (w_space)/2); // Start from centre coordinate
	let mut pos = port;

	let mut vec = vec![vec![0; w_space]; w_space];
	let mut sums = vec![vec![0; w_space]; w_space];
	sums[port.0][port.1] = 1;

	let mut dir = Direction::Right;
	let mut next_dir: Direction;
	let mut next_pos: (usize, usize);

	let mut sol_b: u64 = 0;

	for x in 1..target {
		vec[pos.0][pos.1] = x;

		if x > 1 {
			if sol_b == 0 {
				let s = get_sum(&sums, &pos);

				sums[pos.0][pos.1] = s;

				if s > 312051 {
					sol_b = s;
				}
			}
		}

		pos = get_next_pos(&pos, &dir);
		next_dir = get_next_dir(&dir);
		next_pos = get_next_pos(&pos, &next_dir);

		if vec[next_pos.0][next_pos.1] == 0 {
			dir = next_dir;
		}

	}

	print_space(&vec);
	print_space(&sums);
	println!("distance: {}", get_distance(&pos, &port));
	println!("solution b: {}", sol_b);
}

fn get_sum (vec: &Vec<Vec<u64>>, pos: &(usize, usize)) -> u64 {
	let mut sum: u64 = 0;

	for x in -1..2 {
		for y in -1..2 {
			if ! (x == 0 && y == 0) {
				sum += vec[(pos.0 as i32 + y) as usize]
					[(pos.1 as i32 + x) as usize];
			}
		}
	}

	sum
}

fn get_distance (a: &(usize, usize), b: &(usize, usize)) -> u32 {
	let distance_y: i32 = a.0 as i32 - b.0 as i32;
	let distance_x: i32 = a.1 as i32 - b.1 as i32;

	(distance_y.abs() + distance_x.abs()) as u32
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

fn print_space (vec: &Vec<Vec<u64>>) {
	for x in 0..vec.len() {
		println!("{:?}", vec[x]);
	}
}
