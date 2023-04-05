// Puzzles are represented as 81 ints, top left across and down, with 0s for empty cells.
struct Sudoku([u8; 81]);
// - [x] Switch this to a struct so 'trait' doesn't have to be used.

// Groups are sets of cells that must incorporate ints from 1 to 9.
const GROUPS: [[u8; 9]; 27] = [
	// Rows
	[0, 1, 2, 3, 4, 5, 6, 7, 8],
	[9, 10, 11, 12, 13, 14, 15, 16, 17],
	[18, 19, 20, 21, 22, 23, 24, 25, 26],
	[27, 28, 29, 30, 31, 32, 33, 34, 35],
	[36, 37, 38, 39, 40, 41, 42, 43, 44],
	[45, 46, 47, 48, 49, 50, 51, 52, 53],
	[54, 55, 56, 57, 58, 59, 60, 61, 62],
	[63, 64, 65, 66, 67, 68, 69, 70, 71],
	[72, 73, 74, 75, 76, 77, 78, 79, 80],
	// Columns
	[0, 9, 18, 27, 36, 45, 54, 63, 72],
	[1, 10, 19, 28, 37, 46, 55, 64, 73],
	[2, 11, 20, 29, 38, 47, 56, 65, 74],
	[3, 12, 21, 30, 39, 48, 57, 66, 75],
	[4, 13, 22, 31, 40, 49, 58, 67, 76],
	[5, 14, 23, 32, 41, 50, 59, 68, 77],
	[6, 15, 24, 33, 42, 51, 60, 69, 78],
	[7, 16, 25, 34, 43, 52, 61, 70, 79],
	[8, 17, 26, 35, 44, 53, 62, 71, 80],
	// Boxes
	[0, 1, 2, 9, 10, 11, 18, 19, 20],
	[3, 4, 5, 12, 13, 14, 21, 22, 23],
	[6, 7, 8, 15, 16, 17, 24, 25, 26],
	[27, 28, 29, 36, 37, 38, 45, 46, 47],
	[30, 31, 32, 39, 40, 41, 48, 49, 50],
	[33, 34, 35, 42, 43, 44, 51, 52, 53],
	[54, 55, 56, 63, 64, 65, 72, 73, 74],
	[57, 58, 59, 66, 67, 68, 75, 76, 77],
	[60, 61, 62, 69, 70, 71, 78, 79, 80],
];

impl Sudoku {
	fn get_group_values(&self, group: &[u8; 9]) -> [u8; 9] {
		group.iter().enumerate()
			.fold([0; 9], |mut acc, (i, group_index)| {
				acc[i] = self.0[usize::from(*group_index)];
				acc
			})
	}

	fn is_valid(&self) -> bool {
		for group in GROUPS.iter() {
			let values: [u8; 9] = self.get_group_values(group);
			for &value in values.iter() {
				// Value is outside of 0-9.
				if value > 9 {
					return false
				}
				// Value has duplicates.
				let value_count = values.iter()
					.filter(|comparable_value| **comparable_value != 0)
					.filter(|comparable_value| **comparable_value == value)
					.count();
				if value_count > 1 {
					return false
				}
			}
		}
		true
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const TEST_PUZZLE: [u8; 81] = [
		0, 1, 0, 0, 0, 0, 2, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 8, 0, 0,
		0, 5, 0, 0, 0, 0, 0, 0, 0,
		0, 7, 0, 0, 0, 0, 0, 4, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0
	];

	#[test]
	fn gets_group_values() {
		let group_to_check = GROUPS[0];
		let test_sudoku = Sudoku(TEST_PUZZLE);
		let correct_answer: [u8; 9] = [ 0, 1, 0, 0, 0, 0, 2, 0, 0 ];
		assert_eq!(test_sudoku.get_group_values(&group_to_check), correct_answer)
	}

	#[test]
	fn validates_puzzles() {
		let invalid_puzzle_duplicates = [
            0, 1, 1, 0, 0, 0, 2, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 8, 0, 0,
            0, 5, 0, 0, 0, 0, 0, 0, 0,
            0, 7, 0, 0, 0, 0, 0, 4, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0
        ];
        let invalid_puzzle_greater_than_nine = [
            0, 1, 0, 0, 0, 0, 2, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 10, 0, 0,
            0, 5, 0, 0, 0, 0, 0, 0, 0,
            0, 7, 0, 0, 0, 0, 0, 4, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0
        ];
		assert_eq!(true, Sudoku(TEST_PUZZLE).is_valid());
        assert_eq!(false, Sudoku(invalid_puzzle_duplicates).is_valid());
        assert_eq!(false, Sudoku(invalid_puzzle_greater_than_nine).is_valid());
	}
}
