fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    Err(std::io::Error::new(std::io::ErrorKind::Other, "An error occurred"))
}
