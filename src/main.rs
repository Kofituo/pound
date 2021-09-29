/* add the following */
use std::io;
use std::io::Read;
/* end */

fn main() {
    /* add the following */
    let mut buf = [0; 1];
    while io::stdin().read(&mut buf).expect("Failed to read line") == 1 && buf != [b'q'] {}
    /* end */
}
