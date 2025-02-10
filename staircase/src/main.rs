
/// QUESTION:
///    Write a program that prints a staircase of size n.
///    This is a staircase of size n = 4.
///    Print a staircase of size n using # symbols and spaces.
///    Note: The last line must have 0 spaces in it.
///            #
///           ##
///          ###
///         ####
///      Its base and height are both equal to n. It is drawn using # symbols and spaces. The last line is not preceded by any spaces.

// stair case drawing function
fn draw_staircase(size: usize, step_structure: &char) {
	let mut current_step = size;

	while current_step > 0 {
		let step_size_diff = size - (current_step - 1);
		let stair_space = " ".repeat(size - step_size_diff);
		let stair_structure = format!("{}", step_structure).repeat(step_size_diff);

		println!("{}{}", stair_space, stair_structure);

		current_step -= 1;
	}
}

fn main() {
	// stair
	draw_staircase(10, &'#');
}