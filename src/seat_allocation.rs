use std::collections::HashMap;

pub fn get_number_of_groups(
    n_rows: u32,
    n_columns: u32,
    aisle_seat: u32,
    positions: &Vec<(u32, u32)>,
    group_size: u32,
) -> u32 {
    // Invert the aisle seat, since we are moving right to left.
    let aisle_seat = n_columns - aisle_seat;
    let mut class_rows: HashMap<u32, u32> = HashMap::new();
    for row in 0..n_rows {
        class_rows.insert(row, 0);
    }
    // Filling the Hash with the occupied seats. Each row is a binary word where
    // the '1' are persons and the '0' are empty seats.
    for (row_position, column_position) in positions {
        let row = class_rows.entry(row_position - 1).or_insert(0);
        add_ocuppied_seat(row, *column_position, n_columns);
    }

    let binary_group_size = 2_u32.pow(group_size) - 1;
    let mut possible_groups = 0;
    for (_, current_row) in class_rows {
        let mut last_marked_seat = 0;
        for column_shift in 0..=n_columns - group_size {
            if column_shift == 0 || column_shift >= last_marked_seat {
                let group_end = column_shift + group_size;
                let distance_to_aisle = (aisle_seat as i32 - column_shift as i32).abs();
                let distance_to_group_end = (distance_to_aisle - group_size as i32).abs();
                // a -> (b -> (c && d)) === ~a || (~b || (c && d))
                if column_shift >= aisle_seat
                    || (group_end <= aisle_seat
                        || (distance_to_aisle > 1 && distance_to_group_end > 1))
                {
                    // Move the mask to the location where the group would be
                    let mask = 0 | binary_group_size << column_shift;
                    if !current_row & mask == mask {
                        possible_groups += 1;
                        last_marked_seat = group_end;
                    }
                }
            }
        }
    }
    possible_groups
}

fn add_ocuppied_seat(row: &mut u32, column_shift: u32, n_columns: u32) {
    assert!(column_shift <= n_columns);
    *row |= 1 << (n_columns - column_shift)
}

/*
    1.1 Von Neumann
    ===============

    0 X || 0 0 0 0
    0 0 || 0 0 0 0
    0 0 || 0 0 X 0
    0 0 || 0 0 X 0
*/
#[test]
fn von_neuman() {
    let positions = vec![(1, 2), (3, 5), (4, 5)];
    assert_eq!(get_number_of_groups(4, 6, 2, &positions, 3), 2)
}

/*
    1.2 Turing
    ==========

    0 X || 0 0 0 0
    0 0 || 0 0 0 0
    0 0 || 0 0 X 0
    0 0 || 0 X 0 0
*/
#[test]
fn turing() {
    let positions = vec![(1, 2), (3, 5), (4, 4)];
    assert_eq!(get_number_of_groups(4, 6, 2, &positions, 4), 3)
}

/*
    1.3 Boole
    =========

    0 X 0 0 || 0 0
    0 0 X 0 || 0 X
    0 0 0 0 || 0 X
    0 0 X 0 || 0 0
*/
#[test]
fn boole() {
    let positions = vec![(1, 2), (2, 3), (2, 6), (3, 6), (4, 3)];
    assert_eq!(get_number_of_groups(4, 6, 4, &positions, 2), 7)
}

/*
    2.1 Ada Byron
    =============

    0 0 0 X || 0 0 0 0
    X X 0 0 || 0 0 X 0
    0 0 0 0 || 0 0 0 0
    0 0 0 0 || 0 0 0 X
*/
#[test]
fn ada_byron() {
    let positions = vec![(1, 4), (2, 1), (2, 2), (2, 7), (4, 8)];
    assert_eq!(get_number_of_groups(4, 8, 4, &positions, 4), 5)
}

/*
    2.4 Donald Knuth
    ================

    X 0 0 0 || 0 0 X 0
    0 0 0 X || 0 0 0 0
    0 X 0 0 || 0 0 0 0
    0 0 0 0 || 0 0 0 X
*/
#[test]
fn donald_knuth() {
    let positions = vec![(1, 1), (1, 7), (2, 4), (3, 2), (4, 8)];
    assert_eq!(get_number_of_groups(4, 8, 4, &positions, 5), 3)
}

/*
    Carrel 1
    ========

    0 0 0 ||
*/
#[test]
fn carrel_1() {
    let positions = vec![];
    assert_eq!(get_number_of_groups(1, 3, 3, &positions, 3), 1)
}

/*
    Carrel 2
    ========

    0 0 || 0
*/
#[test]
fn carrel_2() {
    let positions = vec![];
    assert_eq!(get_number_of_groups(1, 3, 2, &positions, 3), 0)
}

/*
    Carrel 3
    ========

    || 0 0 0
*/
#[test]
fn carrel_3() {
    let positions = vec![];
    assert_eq!(get_number_of_groups(1, 3, 0, &positions, 3), 1)
}
