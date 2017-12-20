/* Advent of code */
/* day 20 */

use std::io;
use std::io::BufRead;

#[derive(Debug)]
#[derive(Clone)]
//#[derive(Copy)]
struct P {
	p: Xyz,
	v: Xyz,
	a: Xyz
}

#[derive(Debug)]
#[derive(Clone)]
//#[derive(Copy)]
struct Xyz {
	x: i64,
	y: i64,
	z: i64
}

fn main () {

	let mut ps: Vec<P> = vec![];
	read_stdin(&mut ps);
//	println!("{:?}", ps);

	let mut ps1 = ps.clone();

	part1(&mut ps1);
	part2();
}

/* p=<474,3043,412>, v=<66,437,59>, a=<-6,-24,-4> */

fn read_stdin(v: &mut Vec<P>) {

	let stdin = io::stdin();

	for line in stdin.lock().lines() {
		let mut i: P = P {
			a: Xyz {x: 0, y: 0, z: 0},
			p: Xyz {x: 0, y: 0, z: 0},
			v: Xyz {x: 0, y: 0, z: 0}};

		let l = line.unwrap().to_string();
		let s: Vec<_> = l.split(", ").collect();

		for (n, j) in s.iter().enumerate() {
			let s2: Vec<_> = j.split(|c| c == '<' ||
						     c == '>' ||
						     c == ',').collect();

			let x: i64 = s2[1].parse().unwrap();
			let y: i64 = s2[2].parse().unwrap();
			let z: i64 = s2[3].parse().unwrap();

			match n {
				0 => i.p = Xyz {x: x, y: y, z: z},
				1 => i.v = Xyz {x: x, y: y, z: z},
				2 => i.a = Xyz {x: x, y: y, z: z},
				_ => {}
			}
		}

		println!("{:?}", i);
		v.push(i);
	}

}

fn part1(p: &mut Vec<P>) {

	loop {
		let mut closest_to_origin = 0;
		let mut closest_distance = -1;

		for (i, particle) in p.iter_mut().enumerate() {
			/* update velocities */
			particle.v.x += particle.a.x;
			particle.v.y += particle.a.y;
			particle.v.z += particle.a.z;
			/* update positions */
			particle.p.x += particle.v.x;
			particle.p.y += particle.v.y;
			particle.p.z += particle.v.z;
//			println!("{:?}", particle);

			let dist = particle.p.x.abs() +
				 particle.p.y.abs() +
				 particle.p.z.abs();

//			println!("{}", dist);

			if closest_distance == -1 || dist < closest_distance {
				closest_distance = dist;
				closest_to_origin = i;
			}
		}

		println!("closest to origin => {}", closest_to_origin);
	}
}

fn part2() {

}
