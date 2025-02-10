// stair case drawing function
pub fn draw_staircase(size: usize, step_structure: &char) {
	let mut current_step = size;

	while current_step > 0 {
		let step_size_diff = size - (current_step - 1);
		let stair_space = " ".repeat(size - step_size_diff);
		let stair_structure = format!("{}", step_structure).repeat(step_size_diff);

		println!("{}{}", stair_space, stair_structure);

		current_step -= 1;
	}
}