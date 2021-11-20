use std::collections::HashMap;

pub fn get_number_of_groups(
    n_rows: u32,
    n_columns: u32,
    aisle_seat: u32,
    positions: &Vec<(u32, u32)>,
    group_size: u32,
) -> u32 {
    let aisle_seat = n_columns - aisle_seat; // Invert it since we are moving right to left.
    let mut rows_hash: HashMap<u32, u32> = HashMap::new();
    for row in 0..n_rows {
        rows_hash.insert(row, 0);
    }
    for position in positions {
        let row_position = position.0 - 1;
        let column_position = position.1;
        let row = rows_hash.entry(row_position).or_insert(0);
        add_ocuppied_seat(column_position, row, n_columns);
    }

    let binary_group_size = 2_u32.pow(group_size) - 1;
    let mut possible_groups: u32 = 0;
    for (_, current_binary) in rows_hash {
        let mut last_marked_seat: u32 = 0;
        for column_shift in 0..n_columns - group_size + 1 {
            if column_shift == 0 || column_shift >= last_marked_seat {
                let group_end = column_shift + group_size;
                let distance_to_aisle = i32::abs(aisle_seat as i32 - column_shift as i32);
                if column_shift >= aisle_seat
                    || (group_end <= aisle_seat // ~a || b     a -> b
                        || (distance_to_aisle > 1
                            && i32::abs(distance_to_aisle - group_size as i32) > 1))
                {
                    let mask = 0 | binary_group_size << column_shift;
                    if !current_binary & mask == mask {
                        possible_groups += 1;
                        last_marked_seat = group_end;
                    }
                }
            }
        }
    }
    possible_groups
}

fn add_ocuppied_seat(column_shift: u32, row: &mut u32, n_columns: u32) {
    assert!(column_shift <= n_columns); // TODO test it out
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
