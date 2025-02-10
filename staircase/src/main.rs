mod staircase;

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

fn main() {
	// stair
	staircase::draw_staircase(10, &'#');
}