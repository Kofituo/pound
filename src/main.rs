use std::io;
use std::io::Read;

fn main() {
    let mut buf = [0; 1];
    /*modify*/
    while io::stdin().read(&mut buf).expect("Failed to read line") == 1 && buf != [b'q'] {}
    /*end*/
}
