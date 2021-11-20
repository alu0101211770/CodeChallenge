pub mod seat_allocation;

fn main() {
    let positions = vec![(2, 2), (2, 7), (3, 2), (3, 8), (4, 1), (4, 6)];
    println!(
        "{}",
        seat_allocation::get_number_of_groups(4, 8, 4, &positions, 4)
    );
    println!("------------------------------------------");
    println!(
        "{}",
        seat_allocation::get_number_of_groups(4, 8, 4, &positions, 3)
    );
}
