mod seat_allocation;

fn main() {
    let positions: Vec<(u32, u32)> = vec![(2, 2), (2, 7), (3, 2), (3, 8), (4, 1), (4, 6)];
    println!(
        "Ejemplo 1: {}",
        seat_allocation::get_number_of_groups(4, 8, 4, &positions, 4)
    );
    println!(
        "Ejemplo 2: {}",
        seat_allocation::get_number_of_groups(4, 8, 4, &positions, 3)
    );
}
