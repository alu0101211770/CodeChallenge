use std::collections::HashMap;

pub fn get_number_of_groups(
    n_rows: u32,
    n_columns: u32,
    aisle_seat: u32,
    positions: &Vec<(u32, u32)>,
    group_size: u32,
) -> u32 {
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

    for (_row, current_binary) in rows_hash {
        let mut last_marked_seat: u32 = 0;
        for column_shift in 0..n_columns - group_size + 1 {
            if (column_shift == 0 || column_shift >= last_marked_seat)
                && (column_shift >= aisle_seat
                    || ((column_shift + group_size <= aisle_seat) || // ~a || b     a -> b
                    (i32::abs(aisle_seat as i32 - column_shift as i32) > 1
                        && i32::abs(aisle_seat as i32 - column_shift as i32 - group_size as i32) > 1 )))
            {
                let mask = 0 | binary_group_size << column_shift;
                if !current_binary & mask == mask {
                    possible_groups += 1;
                    last_marked_seat = column_shift + group_size;
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
