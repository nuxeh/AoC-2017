
    use std::io;
    use std::io::BufRead;
    
    fn main () {
    
    	let stdin = io::stdin();
    
    	let mut moves = vec![];
    
    	for line in stdin.lock().lines() {
    		let line = line.unwrap();
    		let split: Vec<_> = line.split(",").collect();
    
    		moves.push("cheese");
    		moves.push(split[0]);
    	}
    }


    error[E0597]: `line` does not live long enough
      --> re.rs:16:2
       |
    12 |   let split: Vec<_> = line.split(",").collect();
       |                       ---- borrow occurs here
    ...
    16 |  }
       |  ^ `line` dropped here while still borrowed
    17 | }
       | - borrowed value needs to live until here
    
    error: aborting due to previous error


So, i have no idea what any of this means...

It seems that line doesn't live until the next invocation of the loop.

Using split[0] causes line to go out of scope?

    for line in stdin.lock().lines() {
    	let line = line.unwrap().clone();
    	let split: Vec<_> = line.split(",").collect();

    	moves.push("cheese");
    	moves.push(split[0]);
    }

Doesn't seem to help (fails with the same error).

It feels like there's only one thing that works.
