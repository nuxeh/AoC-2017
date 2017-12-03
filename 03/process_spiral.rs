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
	let target = 100;
	let w_space = (target as f64).sqrt() as usize + 3;

	let port = ((w_space-1)/2, (w_space-1)/2); // Start from centre coordinate
	let mut pos = port;

	let mut vec = vec![vec![0; w_space]; w_space];
	let mut sums = vec![vec![0; w_space]; w_space];

	let mut dir = Direction::Right;
	let mut next_dir: Direction;
	let mut next_pos: (usize, usize);

	for x in 1..target {
		vec[pos.0][pos.1] = x;

		sums[pos.0][pos.1] = get_sum(&vec, &pos);

		pos = get_next_pos(&pos, &dir);
		next_dir = get_next_dir(&dir);
		next_pos = get_next_pos(&pos, &next_dir);

//		println!("{}", vec[next_pos.0][next_pos.1]);
//		println!("{:?}", dir);

		if vec[next_pos.0][next_pos.1] == 0 {
			dir = next_dir;
		}

//		println!("{:?}", dir);
//		print_space(&vec);
//		println!("");
	}

	print_space(&vec);
	println!("disance: {}", get_distance(&pos, &port));
	print_space(&sums);

}

fn get_sum (vec: &Vec<Vec<u32>>, pos: &(usize, usize)) -> u32 {
	let mut sum: u32 = 0;
	let dirs = vec![Direction::Up, Direction::Down,
			Direction::Left, Direction::Right];
	let mut p: (usize, usize);

	for d in dirs {
		p = get_next_pos(&pos, &d);
		sum += vec[p.0][p.1];
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

fn print_space (vec: &Vec<Vec<u32>>) {
	for x in 0..vec.len() {
		println!("{:?}", vec[x]);
	}
}
