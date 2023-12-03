mod first;
mod second;
fn main() {
    First
    let res = first::first::solve_part_one();
    match res {
        Ok(res) => println!("First Part One -> {res}"),
        Err(err) => println!("First Part One Error -> {err:?}"),
    }

    println!("");

    let res = first::first::solve_part_two();
    match res {
        Ok(res) => println!("First Part Two -> {res}"),
        Err(err) => println!("First Part One Error -> {err:?}"),
    }

    println!("");

    let res = second::second::solve_part_one();
    match res {
        Ok(res) => println!("Second - Part One -> {res}"),
        Err(err) => println!("Second -Part One Error -> {err:?}"),
    }
}
