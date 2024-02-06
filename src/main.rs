fn main() -> Result<(), std::io::Error> {
    let schema = std::env::args().nth(1).expect("No schema provided");

    println!("schema: {}", schema);
    Ok(())
}
