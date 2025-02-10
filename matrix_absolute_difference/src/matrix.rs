use std::ops::Sub;

/**
 * @description calculate absolute difference
 */
pub fn calculate_abs_diff(matrix: &Vec<Vec<f64>>) -> f64 {
	let elements: Vec<_> = matrix
		.iter()
		.flatten()
		.collect();

	if elements.len() != matrix.len() * matrix.len() {
		panic!("matrix not a square matrix");
	}

	let mut left_diagonal_sum: f64 = 0.0;
	let mut right_diagonal_sum: f64 = 0.0;

	for (i, row) in matrix.iter().enumerate() {
		let left_diagonal_element = row.get(i);
		let right_diagonal_element = row.get(matrix.len() - 1 - i);

		left_diagonal_sum += left_diagonal_element.unwrap();
		right_diagonal_sum += right_diagonal_element.unwrap();
	}

	left_diagonal_sum.sub(right_diagonal_sum).abs()
}