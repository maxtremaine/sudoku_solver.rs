//use crate::pure_functions;

#[derive(Debug, Copy, Clone)]
struct Cell {
    row: u8,
    col: u8
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

fn c(row: u8, col: u8) -> Cell {
    Cell { row, col }
}

fn get_group_coords() -> Vec<Vec<Cell>> {
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
        .map(|group| -> Vec<Cell> {
            group.iter()
                .map(|coord| c(coord.0, coord.1))
                .collect()
        })
        .collect()
}

fn get_related_cells(cell: Cell) -> Vec<Cell> {
    get_group_coords().iter()
        .filter(|coords| coords.contains(&cell))
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

fn get_cell_values(indexes: &Vec<Cell>, puzzle: [[u8; 9]; 9]) -> Vec<u8> {
    indexes.iter()
        .fold(Vec::new(), |mut acc, index| {
            let value = puzzle[usize::from(index.row)][usize::from(index.col)];
            if value != 0 {
                acc.push(value)
            }
            acc
        })
}

fn is_valid_puzzle(puzzle: [[u8; 9]; 9]) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_cells() {
        let constructor_instance = Cell { row: 4, col: 8 };
        let method_instance = c(4, 8);
        assert_eq!(constructor_instance.row, method_instance.row);
        assert_eq!(constructor_instance.col, method_instance.col);
    }

    #[test]
    fn gets_related_cells() {
        let test_group00 = vec![ c(0, 0), c(0, 1), c(0, 2), c(0, 3), c(0, 4), c(0, 5), c(0, 6),
            c(0, 7), c(0, 8), c(1, 0), c(2, 0), c(3, 0), c(4, 0), c(5, 0), c(6, 0), c(7, 0),
            c(8, 0), c(1, 1), c(1, 2), c(2, 1), c(2, 2) ];
        let test_group72 = vec![ c(7, 0), c(7, 1), c(7, 2), c(7, 3), c(7, 4), c(7, 5), c(7, 6),
            c(7, 7), c(7, 8), c(0, 2), c(1, 2), c(2, 2), c(3, 2), c(4, 2), c(5, 2), c(6, 2),
            c(8, 2), c(6, 0), c(6, 1), c(8, 0), c(8, 1) ];
        assert_eq!(test_group00, get_related_cells(c(0, 0)));
        assert_eq!(test_group72, get_related_cells(c(7, 2)));
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
        let test_indexes0 = vec![ c( 0, 0), c(0, 1), c(0, 6), c(6, 1), c(7, 1) ];
        let test_indexes1 = vec![ c( 4, 2), c(4, 3), c(5, 6), c(7, 7)];
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
}