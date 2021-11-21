const GROUPS: [[(u8, u8); 9]; 27] = [
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

fn get_related_cells(cell: (u8, u8)) -> Vec<(u8, u8)> {
    GROUPS.iter()
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

fn get_cell_values(indexes: Vec<(u8, u8)>, puzzle: [[u8; 9]; 9]) -> Vec<u8> {
    indexes.iter()
        .fold(Vec::new(), |mut acc, index| {
            let value = puzzle[usize::from(index.0)][usize::from(index.1)];
            if value != 0 {
                acc.push(value)
            }
            acc
        })
}

fn is_valid_puzzle(puzzle: [[u8; 9]; 9]) -> bool {
    for group in GROUPS.iter() {
        let values = get_cell_values(Vec::from(*group), puzzle);
        for value in values.iter() {
            let count = values.iter()
                .filter(|other_value| *other_value == value)
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
    fn gets_related_cells() {
        let test_group00 = vec![ (0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7),
            (0, 8), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (1, 1), (1, 2),
            (2, 1), (2, 2) ];
        let test_group72 = vec![ (7, 0), (7, 1), (7, 2), (7, 3), (7, 4), (7, 5), (7, 6), (7, 7),
            (7, 8), (0, 2), (1, 2), (2, 2), (3, 2), (4, 2), (5, 2), (6, 2), (8, 2), (6, 0), (6, 1),
            (8, 0), (8, 1) ];
        assert_eq!(test_group00, get_related_cells((0, 0)));
        assert_eq!(test_group72, get_related_cells((7, 2)));
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
        let test_indexes0 = vec![ ( 0, 0), (0, 1), (0, 6), (6, 1), (7, 1) ];
        let test_indexes1 = vec![ ( 4, 2), (4, 3), (5, 6), (7, 7)];
        assert_eq!(vec![ 1, 2, 5, 7 ], get_cell_values(test_indexes0, test_puzzle));
        assert_eq!(vec![ 8, 4 ], get_cell_values(test_indexes1, test_puzzle));
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
        let invalid_puzzle = [
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
        assert_eq!(true, is_valid_puzzle(valid_puzzle));
        assert_eq!(false, is_valid_puzzle(invalid_puzzle));
    }
}