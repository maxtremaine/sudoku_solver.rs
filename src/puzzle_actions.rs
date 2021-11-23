use crate::pure_functions;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Coords {
    row: u8,
    col: u8
}

fn cell(row: u8, col: u8) -> Coords {
    Coords { row, col }
}

#[derive(PartialEq, Debug)]
pub struct Cell {
    coords: Coords,
    value: u8,
    possible_values: Vec<u8>
}

impl Cell {
    pub fn new(coords: Coords, puzzle: [[u8; 9]; 9]) -> Self {
        Self {
            coords,
            value: puzzle[usize::from(coords.row)][usize::from(coords.col)],
            possible_values: pure_functions::get_missing_digits(get_cell_values(&get_related_cells(coords), puzzle))
        }
    }
}

fn get_group_coords() -> Vec<Vec<Coords>> {
    let groups: [[(u8, u8); 9]; 27] = [
        // Rows
        [ (0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8) ],
        [ (1, 0), (1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7), (1, 8) ],
        [ (2, 0), (2, 1), (2, 2), (2, 3), (2, 4), (2, 5), (2, 6), (2, 7), (2, 8) ],
        [ (3, 0), (3, 1), (3, 2), (3, 3), (3, 4), (3, 5), (3, 6), (3, 7), (3, 8) ],
        [ (4, 0), (4, 1), (4, 2), (4, 3), (4, 4), (4, 5), (4, 6), (4, 7), (4, 8) ],
        [ (5, 0), (5, 1), (5, 2), (5, 3), (5, 4), (5, 5), (5, 6), (5, 7), (5, 8) ],
        [ (6, 0), (6, 1), (6, 2), (6, 3), (6, 4), (6, 5), (6, 6), (6, 7), (6, 8) ],
        [ (7, 0), (7, 1), (7, 2), (7, 3), (7, 4), (7, 5), (7, 6), (7, 7), (7, 8) ],
        [ (8, 0), (8, 1), (8, 2), (8, 3), (8, 4), (8, 5), (8, 6), (8, 7), (8, 8) ],
        // Columns
        [ (0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0) ],
        [ (0, 1), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1), (6, 1), (7, 1), (8, 1) ],
        [ (0, 2), (1, 2), (2, 2), (3, 2), (4, 2), (5, 2), (6, 2), (7, 2), (8, 2) ],
        [ (0, 3), (1, 3), (2, 3), (3, 3), (4, 3), (5, 3), (6, 3), (7, 3), (8, 3) ],
        [ (0, 4), (1, 4), (2, 4), (3, 4), (4, 4), (5, 4), (6, 4), (7, 4), (8, 4) ],
        [ (0, 5), (1, 5), (2, 5), (3, 5), (4, 5), (5, 5), (6, 5), (7, 5), (8, 5) ],
        [ (0, 6), (1, 6), (2, 6), (3, 6), (4, 6), (5, 6), (6, 6), (7, 6), (8, 6) ],
        [ (0, 7), (1, 7), (2, 7), (3, 7), (4, 7), (5, 7), (6, 7), (7, 7), (8, 7) ],
        [ (0, 8), (1, 8), (2, 8), (3, 8), (4, 8), (5, 8), (6, 8), (7, 8), (8, 8) ],
        // Boxes
        [ (0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2), (2, 0), (2, 1), (2, 2) ],
        [ (0, 3), (0, 4), (0, 5), (1, 3), (1, 4), (1, 5), (2, 3), (2, 4), (2, 5) ],
        [ (0, 6), (0, 7), (0, 8), (1, 6), (1, 7), (1, 8), (2, 6), (2, 7), (2, 8) ],
        [ (3, 0), (3, 1), (3, 2), (4, 0), (4, 1), (4, 2), (5, 0), (5, 1), (5, 2) ],
        [ (3, 3), (3, 4), (3, 5), (4, 3), (4, 4), (4, 5), (5, 3), (5, 4), (5, 5) ],
        [ (3, 6), (3, 7), (3, 8), (4, 6), (4, 7), (4, 8), (5, 6), (5, 7), (5, 8) ],
        [ (6, 0), (6, 1), (6, 2), (7, 0), (7, 1), (7, 2), (8, 0), (8, 1), (8, 2) ],
        [ (6, 3), (6, 4), (6, 5), (7, 3), (7, 4), (7, 5), (8, 3), (8, 4), (8, 5) ],
        [ (6, 6), (6, 7), (6, 8), (7, 6), (7, 7), (7, 8), (8, 6), (8, 7), (8, 8) ]
    ];

    groups.iter()
        .map(|group| -> Vec<Coords> {
            group.iter()
                .map(|coord| cell(coord.0, coord.1))
                .collect()
        })
        .collect()
}

fn get_related_cells(index: Coords) -> Vec<Coords> {
    get_group_coords().iter()
        .filter(|coords| coords.contains(&index))
        .fold(Vec::new(), |mut acc, group| {
            group.iter()
                .for_each(|value| {
                    if !acc.contains(value) {
                        acc.push(*value)
                    }
                });
            acc
        })
}

fn get_cell_values(indexes: &Vec<Coords>, puzzle: [[u8; 9]; 9]) -> Vec<u8> {
    indexes.iter()
        .fold(Vec::new(), |mut acc, index| {
            let value = puzzle[usize::from(index.row)][usize::from(index.col)];
            if value != 0 {
                acc.push(value)
            }
            acc
        })
}

pub fn is_valid_puzzle(puzzle: [[u8; 9]; 9]) -> bool {
    for group in get_group_coords().iter() {
        let values = get_cell_values(group, puzzle);
        for value in values.iter() {
            if value > &9 {
                return false
            }
            let count = values.iter()
                .filter(|other_value| other_value == &value)
                .count();
            if count > 1 {
                return false
            }
        }
    }

    true
}

pub fn change_cell(index: Coords, value: u8, mut puzzle: [[u8; 9]; 9]) -> [[u8; 9]; 9] {
    puzzle[usize::from(index.row)][usize::from(index.col)] = value;
    puzzle
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_coordinates() {
        let constructor_instance = Coords { row: 4, col: 8 };
        let method_instance = cell(4, 8);
        assert_eq!(constructor_instance.row, method_instance.row);
        assert_eq!(constructor_instance.col, method_instance.col);
    }

    #[test]
    fn creates_cells() {
        let test_coords = Coords { row: 0, col: 0};
        let test_puzzle = [
            [ 0, 1, 0, 0, 0, 0, 2, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 8, 0, 0 ],
            [ 0, 5, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 7, 0, 0, 0, 0, 0, 4, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ]
        ];
        let output_cell = Cell {
            coords: test_coords,
            value: 0,
            possible_values: vec![ 3, 4, 5, 6, 7, 8, 9 ]
        };
        assert_eq!(output_cell, Cell::new(test_coords, test_puzzle));
    }

    #[test]
    fn gets_related_cells() {
        let test_group00 = vec![ cell(0, 0), cell(0, 1), cell(0, 2), cell(0, 3), cell(0, 4), cell(0, 5), cell(0, 6),
            cell(0, 7), cell(0, 8), cell(1, 0), cell(2, 0), cell(3, 0), cell(4, 0), cell(5, 0), cell(6, 0), cell(7, 0),
            cell(8, 0), cell(1, 1), cell(1, 2), cell(2, 1), cell(2, 2) ];
        let test_group72 = vec![ cell(7, 0), cell(7, 1), cell(7, 2), cell(7, 3), cell(7, 4), cell(7, 5), cell(7, 6),
            cell(7, 7), cell(7, 8), cell(0, 2), cell(1, 2), cell(2, 2), cell(3, 2), cell(4, 2), cell(5, 2), cell(6, 2),
            cell(8, 2), cell(6, 0), cell(6, 1), cell(8, 0), cell(8, 1) ];
        assert_eq!(test_group00, get_related_cells(cell(0, 0)));
        assert_eq!(test_group72, get_related_cells(cell(7, 2)));
    }

    #[test]
    fn gets_cell_values() {
        let test_puzzle = [
            [ 0, 1, 0, 0, 0, 0, 2, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 8, 0, 0 ],
            [ 0, 5, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 7, 0, 0, 0, 0, 0, 4, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ]
        ];
        let test_indexes0 = vec![ cell( 0, 0), cell(0, 1), cell(0, 6), cell(6, 1), cell(7, 1) ];
        let test_indexes1 = vec![ cell( 4, 2), cell(4, 3), cell(5, 6), cell(7, 7)];
        assert_eq!(vec![ 1, 2, 5, 7 ], get_cell_values(&test_indexes0, test_puzzle));
        assert_eq!(vec![ 8, 4 ], get_cell_values(&test_indexes1, test_puzzle));
    }

    #[test]
    fn validates_puzzle() {
        let valid_puzzle = [
            [ 0, 1, 0, 0, 0, 0, 2, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 8, 0, 0 ],
            [ 0, 5, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 7, 0, 0, 0, 0, 0, 4, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ]
        ];
        let invalid_puzzle0 = [
            [ 0, 1, 1, 0, 0, 0, 2, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 8, 0, 0 ],
            [ 0, 5, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 7, 0, 0, 0, 0, 0, 4, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ]
        ];
        let invalid_puzzle1 = [
            [ 0, 1, 0, 0, 0, 0, 2, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 10, 0, 0 ],
            [ 0, 5, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 7, 0, 0, 0, 0, 0, 4, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ]
        ];
        assert_eq!(true, is_valid_puzzle(valid_puzzle));
        assert_eq!(false, is_valid_puzzle(invalid_puzzle0));
        assert_eq!(false, is_valid_puzzle(invalid_puzzle1));
    }

    #[test]
    fn changes_values() {
        let input_puzzle = [
            [ 0, 1, 0, 0, 0, 0, 2, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 8, 0, 0 ],
            [ 0, 5, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 7, 0, 0, 0, 0, 0, 4, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ]
        ];
        let output_puzzle = [
            [ 4, 1, 0, 0, 0, 0, 2, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 0, 0, 0, 0, 0, 8, 0, 0 ],
            [ 0, 5, 0, 0, 0, 0, 0, 0, 0 ],
            [ 0, 7, 0, 0, 0, 0, 0, 4, 0 ],
            [ 0, 0, 0, 0, 0, 0, 0, 0, 0 ]
        ];
        assert_eq!(output_puzzle, change_cell(cell(0, 0), 4, input_puzzle));
    }
}