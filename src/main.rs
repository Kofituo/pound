/* add the following */
use std::io;
use std::io::Read;
/* end */

fn main() {
    /* add the following */
    let mut buf = [0; 1];
    loop {
        io::stdin()
            .read_exact(&mut buf)
            .expect("Failed to read line");
    }
    /* end */
}
