mod matrix;

/// Given a square matrix, calculate the absolute difference between the sums of its diagonals. For example, the square matrix arr is shown below:
/// Example - using 3 by 3 matrix
/// 1 2 3
/// 4 5 6
/// 9 8 9 
/// The left-to-right diagonal = 1 + 5 + 9 = 15. The right to left diagonal = 3 + 5 + 9 = 17. Their absolute difference is |15 - 17| = 2.

fn main() {
  let matrix = vec![vec![1.0,2.0,3.0], vec![4.0,5.0,6.0], vec![9.0,8.0,9.0]];

	let abs_diff = matrix::calculate_abs_diff(&matrix);

	println!("abs diff: {}", abs_diff);
}
