pub mod encryptFile;

const AUTHOR: &str = "@Octalbyte";
const VERSION: &str = "0.1.0";

fn main() {
    println!("Locker v{} made by {}", VERSION, AUTHOR);
    encryptFile::encryptFile::hello();
}
