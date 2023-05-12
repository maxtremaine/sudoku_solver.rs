use crate::blank_cell::BlankCell;
use crate::pure_functions::get_missing_digits;

// Groups are sets of cells that must incorporate ints from 1 to 9.
const GROUPS: [[usize; 9]; 27] = [
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

const FILE_TO_STRING_INDEXES: [usize; 81] = [16, 17, 18, 20, 21, 22, 24, 25, 26, 30, 31, 32, 34, 35,
	36, 38, 39, 40, 44, 45, 46, 48, 49, 50, 52, 53, 54, 72, 73, 74, 76, 77, 78, 80, 81, 82, 86,
	87, 88, 90, 91, 92, 94, 95, 96, 100, 101, 102, 104, 105, 106, 108, 109, 110, 128, 129, 130,
	132, 133, 134, 136, 137, 138, 142, 143, 144, 146, 147, 148, 150, 151, 152, 156, 157, 158, 160,
	161, 162, 164, 165, 166];

// Puzzles are represented as 81 ints, top left across and down, with 0s for empty cells.
#[derive(PartialEq, Debug)]
pub struct Sudoku {
	pub numbers: [u8; 81]
}

impl Sudoku {
	// Turns a .sudoku file into a Sudoku puzzle.
	pub fn from_file(sudoku_string: String) -> Result<Self, &'static str> {
		if sudoku_string.len() != 167 {
			return Err("A .sudoku file must be 167 characters long.")
		}
		// TODO: Switch sudoku string to chars.
		let sudoku_string = sudoku_string.as_bytes();
		let numbers: [u8; 81] = FILE_TO_STRING_INDEXES.iter()
			.enumerate()
			.fold([0; 81], |mut acc, (array_index, string_index)| {
				let character = char::from(sudoku_string[*string_index]);
				if character != '_' {
					let mut int_value: u8 = u32::from(character).try_into().unwrap();
					int_value -= 48; // Convert from byte to int.
					acc[array_index] = int_value;
				}
				acc
			});
		let output = Self::from(numbers)?;
		Ok(output)
	}

	pub fn from(puzzle_values: [u8; 81]) -> Result<Self, &'static str> {
		let output = Self{numbers: puzzle_values};
		if !output.is_valid() {
			return Err("The Sudoku file does not have a valid solution.")
		}
		Ok(output)
	}

	// Returns the values of a puzzle for a specific group.
	fn get_group_values(&self, group: &[usize; 9]) -> [u8; 9] {
		group.iter().enumerate()
			.fold([0; 9], |mut acc, (i, group_index)| {
				acc[i] = self.numbers[*group_index];
				acc
			})
	}

	// Using a simple /81 index, what cells have to include 1-9 to satisfy the rules of Sudoku?
	// TODO: Break out get_related_cell_indexes for TDD.
	fn get_related_cell_values(&self, index: usize) -> Vec<u8> {
		// Get the groups associated with "index"
		let related_groups: Vec<[usize; 9]> = GROUPS.iter()
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
	pub fn change_cell(&self, index: u8, value:u8) -> Self {
		let mut new_values = self.numbers;
		new_values[usize::from(index)] = value;
		Self{numbers: new_values}
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

	// Gets all of the blank cells in the puzzle, with degrees of freedom.
	pub fn get_blank_cells(&self) -> Vec<BlankCell> {
		self.numbers.iter().enumerate().fold(Vec::new(), |mut blank_cells, (i, value)| {
			if *value == 0 {
				let related_cell_values: Vec<u8> = self.get_related_cell_values(i);
				let possible_values: Vec<u8> = get_missing_digits(related_cell_values);
				let new_blank_cell = BlankCell{ index: i.try_into().unwrap(), possible_values};
				blank_cells.push(new_blank_cell);
			}
			blank_cells.sort_by(|a, b| a.possible_values.len().cmp(&b.possible_values.len()));
			blank_cells
		})
	}

	pub fn get_valid_solutions(self, verbose: bool) -> Vec<Self> {
		let max_run_index = self.get_blank_cells().len();
    
		// Add to working puzzles and collapse when no options are available.
		(1..=max_run_index)
			.fold(Vec::from([self]), |working_branches, run_count| {
				let new_working_branches = working_branches.iter()
					.fold(Vec::new(), |mut new_branches, current_branch| -> Vec<Sudoku> {
						let blank_cells = current_branch.get_blank_cells();
						let lowest_cell = &blank_cells[0];
						lowest_cell.possible_values.iter().for_each(|possible_value| {
							let new_branch = current_branch.change_cell(lowest_cell.index, *possible_value);
							new_branches.push(new_branch);
						});
						new_branches
					});
				if verbose {
					println!("Completed run {} with {} branches.", run_count, new_working_branches.len());
				}
				new_working_branches
			})
	}

	pub fn to_string(&self) -> String {
		let mut working_string: [char; 167] = [
			' ', ' ', 'a', 'b', 'c', ' ', 'd', 'e', 'f', ' ', 'g', 'h', 'i', '\n',
			'1', ' ', '_', '_', '_', '|', '_', '_', '_', '|', '_', '_', '_', '\n',
			'2', ' ', '_', '_', '_', '|', '_', '_', '_', '|', '_', '_', '_', '\n',
			'3', ' ', '_', '_', '_', '|', '_', '_', '_', '|', '_', '_', '_', '\n',
			' ', ' ', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '\n',
			'4', ' ', '_', '_', '_', '|', '_', '_', '_', '|', '_', '_', '_', '\n',
			'5', ' ', '_', '_', '_', '|', '_', '_', '_', '|', '_', '_', '_', '\n',
			'6', ' ', '_', '_', '_', '|', '_', '_', '_', '|', '_', '_', '_', '\n',
			' ', ' ', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', '\n',
			'7', ' ', '_', '_', '_', '|', '_', '_', '_', '|', '_', '_', '_', '\n',
			'8', ' ', '_', '_', '_', '|', '_', '_', '_', '|', '_', '_', '_', '\n',
			'9', ' ', '_', '_', '_', '|', '_', '_', '_', '|', '_', '_', '_'
		];
		
		working_string = FILE_TO_STRING_INDEXES.iter().enumerate()
			.fold(working_string, |mut working_string, (puzzle_index, string_index)| {
				let int_value: u8 = self.numbers[puzzle_index];
				let char_value: char = format!("{}", int_value).chars().nth(0).unwrap();
				working_string[*string_index] = char_value;
				working_string
			});
		working_string.iter().fold(String::from(""), |mut acc, character| {
			acc.push(*character);
			acc
		})
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
	fn creates_puzzle_from_file() {
		let valid_file: String = String::from("  abc def ghi\n1 7__|_4_|__1\n2 __1|___|2__\n3 _6_|2_9|_8_\n  -----------\n4 __3|5_4|9__\n5 1__|___|__4\n6 __2|1_8|5__\n  -----------\n7 _1_|9_6|_7_\n8 __8|___|4__\n9 6__|_2_|__8");
		let corresponding_puzzle: Sudoku = Sudoku{ numbers: [7, 0, 0, 0, 4, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 6, 0, 2, 0,
			9, 0, 8, 0, 0, 0, 3, 5, 0, 4, 9, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 2, 1, 0, 8, 5, 0,
			0, 0, 1, 0, 9, 0, 6, 0, 7, 0, 0, 0, 8, 0, 0, 0, 4, 0, 0, 6, 0, 0, 0, 2, 0, 0, 0, 8]};
		assert_eq!(Sudoku::from_file(valid_file).unwrap(), corresponding_puzzle)
	}

	#[test]
	fn creates_file_from_puzzle() {
		let valid_file: String = String::from("  abc def ghi\n1 712|954|836\n2 539|186|247\n3 684|237|519\n  -----------\n4 325|479|681\n5 198|365|724\n6 476|821|953\n  -----------\n7 247|593|168\n8 861|742|395\n9 953|618|472");
		let corresponding_puzzle: Sudoku = Sudoku{ numbers: [7, 1, 2, 9, 5, 4, 8, 3, 6, 5, 3, 9, 1,
				8, 6, 2, 4, 7, 6, 8, 4, 2, 3, 7, 5, 1, 9, 3, 2, 5, 4, 7, 9, 6, 8, 1, 1, 9, 8, 3, 6,
				5, 7, 2, 4, 4, 7, 6, 8, 2, 1, 9, 5, 3, 2, 4, 7, 5, 9, 3, 1, 6, 8, 8, 6, 1, 7, 4, 2,
				3, 9, 5, 9, 5, 3, 6, 1, 8, 4, 7, 2]};
		assert_eq!(corresponding_puzzle.to_string(), valid_file)
	}

	#[test]
	fn gets_group_values() {
		let group_to_check = GROUPS[0];
		let correct_answer: [u8; 9] = [ 0, 1, 0, 0, 0, 0, 2, 0, 0 ];
		assert_eq!(TEST_SUDOKU.get_group_values(&group_to_check), correct_answer);
	}

	#[test]
	fn gets_related_cell_values() {
		let related_to_0 = vec![1, 2];
		assert_eq!(related_to_0, TEST_SUDOKU.get_related_cell_values(0));
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

	#[test]
	fn gets_blank_cells() {
		let missing_one = [
            7, 0, 2, 9, 5, 4, 8, 3, 6,
			5, 3, 9, 1, 8, 6, 2, 4, 7,
			6, 8, 4, 2, 3, 7, 5, 1, 9,
			3, 2, 5, 4, 7, 9, 6, 8, 1,
			1, 9, 8, 3, 6, 5, 7, 2, 4,
			4, 7, 6, 8, 2, 1, 9, 5, 3,
			2, 4, 7, 5, 9, 3, 1, 6, 8,
			8, 6, 1, 7, 4, 2, 3, 9, 5,
			9, 5, 3, 6, 1, 8, 4, 7, 2
        ];
        let missing_one_puzzle = Sudoku{ numbers: missing_one };
        let blank_cells_generated = missing_one_puzzle.get_blank_cells();
        let blank_cells_created = vec![ BlankCell{ index: 1, possible_values: vec![1]}];
        assert_eq!(blank_cells_created, blank_cells_generated);
	}
}
