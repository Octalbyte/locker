pub mod encrypt_file;

const AUTHOR: &str = "@Octalbyte";
const VERSION: &str = "0.1.0";

fn main() {
    println!("Locker v{} made by {}", VERSION, AUTHOR);
    encrypt_file::encrypt_file::hello();
}
