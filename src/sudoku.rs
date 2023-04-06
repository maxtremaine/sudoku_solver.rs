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

// Puzzles are represented as 81 ints, top left across and down, with 0s for empty cells.
#[derive(PartialEq, Debug)]
struct Sudoku {
	numbers: [u8; 81]
}

impl Sudoku {
	// Returns the values of a puzzle for a specific group.
	fn get_group_values(&self, group: &[u8; 9]) -> [u8; 9] {
		group.iter().enumerate()
			.fold([0; 9], |mut acc, (i, group_index)| {
				acc[i] = self.numbers[usize::from(*group_index)];
				acc
			})
	}

	// Using a simple /81 index, what cells have to include 1-9 to satisfy the rules of Sudoku?
	fn get_related_cell_values(&self, index: u8) -> Vec<u8> {
		// Get the groups associated with "index"
		let related_groups: Vec<[u8; 9]> = GROUPS.iter()
			.fold(Vec::new(), |mut acc, group| {
				if group.contains(&index) {
					acc.push(*group)
				}
				acc
			});
		// Get the values associated with those groups, without duplication or zeros
		related_groups.iter()
			.fold(Vec::new(), |mut acc, group| {
				self.get_group_values(group).iter()
					.for_each(|value| {
						if *value != 0 && !acc.contains(value) {
							acc.push(*value)
						}
					});
				acc
			})
	}

	// Creates a shallow copy with one adjusted value.
	fn change_cell(&self, index: u8, value:u8) -> Sudoku {
		let mut new_values = self.numbers;
		new_values[usize::from(index)] = value;
		Sudoku{numbers: new_values}
	}

	// Does the puzzle in its current state satisfy the rules of Sudoku?
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

	const TEST_SUDOKU: Sudoku = Sudoku{numbers: TEST_PUZZLE};

	#[test]
	fn gets_group_values() {
		let group_to_check = GROUPS[0];
		let correct_answer: [u8; 9] = [ 0, 1, 0, 0, 0, 0, 2, 0, 0 ];
		assert_eq!(TEST_SUDOKU.get_group_values(&group_to_check), correct_answer)
	}

	#[test]
	fn gets_related_cell_values() {
		let related_to_0 = vec![1, 2];
		assert_eq!(related_to_0, TEST_SUDOKU.get_related_cell_values(0))
	}

	#[test]
	fn changes_cell_values() {
		let unchanged_puzzle: [u8; 81] = [
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
		let changed_puzzle: [u8; 81] = [
			0, 1, 0, 5, 0, 0, 2, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 8, 0, 0,
			0, 5, 0, 0, 0, 0, 0, 0, 0,
			0, 7, 0, 0, 0, 0, 0, 4, 0,
			0, 0, 0, 0, 0, 0, 0, 0, 0
		];
		let new_puzzle = TEST_SUDOKU.change_cell(3, 5);
		assert_eq!(Sudoku{numbers: changed_puzzle}, new_puzzle);
		assert_eq!(Sudoku{numbers: unchanged_puzzle}, TEST_SUDOKU);
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
		assert_eq!(true, Sudoku{numbers: TEST_PUZZLE}.is_valid());
        assert_eq!(false, Sudoku{numbers: invalid_puzzle_duplicates}.is_valid());
        assert_eq!(false, Sudoku{numbers: invalid_puzzle_greater_than_nine}.is_valid());
	}
}
