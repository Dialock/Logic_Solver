
use std::io::{self, Write};
use std::collections::LinkedList;

pub fn run_tree() {

	let mut line_num = 1;
	let mut input = String::new();

	let _premise_fore 		= "Input Premise (";
	let _premise_end		= ") 'C' for Conclusion: ";
	let _conc_fore			= "Input Conclusion(";
	let _conc_end			= "): ";

	let mut fore 			= _premise_fore;
	let mut end 			= _premise_end;

	let mut is_condition 	= false;


	println!("Truth Tree Solver.");
	println!("'ZZZ' to quit modules.");
	loop {

    	println!("-------------------------\n");
    	print!("{}({}){}", fore, line_num, end);

    	io::stdout().flush()
    	 .expect("flush failed!");

    	io::stdin().read_line(&mut input)
    		.expect("Failed to read line");

    	if input.trim() == "ZZZ" { break; } 

    	if input.trim() == "C" {

    		if line_num > 1 && !is_condition {
    			is_condition = true;
    			fore 	= _conc_fore;
    			end 	= _conc_end;
    		}

    	} else {

    		let _v = check_string(&input);

			if _v {
				isolate_string(&input);
				line_num += 1;
			}

    		input.clear();

    	}
	}
}

/// check_string  validates a string
fn check_string(s: &str) -> bool {
	
	let mut list: LinkedList<char> = LinkedList::new();



	for c in s.chars() {
		if c == '(' {
			list.push_back(c);
		} else if c == ')' {
			if list.len() == 0 {
				println!("Parentheses not balanced.");
				return false;
			}
			list.pop_back();
		}

	}

	if list.len() > 0 {
		println!("Parentheses not balanced.");
		return false;
	} else {
		println!("Parentheses balanced.");
		return true;
	}

}

/// isolate_string helps break a string into its parts
fn isolate_string(s: &str) {
	println!("In isolate_string: {}", s);
}