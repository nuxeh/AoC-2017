use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "input.txt";

    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

//    println!("With text:\n{}", contents);


//	to_arr(&contents);
//}


//fn to_arr (str: &String) {
	let mut split = contents.split("\n");

//	println!("{}", str);

	let mut vec = Vec::new();
	for s in split {
		let n = s.parse::<i32>().unwrap();

		println!("{}", n);
		vec.push(s.parse::<i32>().unwrap())
	}

	println!("{:?}", vec);
}
