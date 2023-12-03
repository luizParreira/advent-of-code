mod first;
fn main() {
    // First
    let res = first::first::solve();
    match res {
        Ok(res) => println!("Response {res}"),
        Err(err) => println!("Error {err:?}"),
    }
}
