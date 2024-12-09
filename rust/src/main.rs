/**
*
* md5
*
* This code was implemented using the original md5 message digest
* RFC, written by R. Rivest in April 1992.
*
* For more information about the md5 algorithm, or the original C
* source code implementation, please reference RFC 1321.
*
* (source: https://datatracker.ietf.org/doc/html/rfc1321)
*
*/
use std::time::Instant;

fn main() {
    let input = "helloworld";
    let start = Instant::now();
    println!("{}", md5::md5(input));
    println!("{:.4} ns", start.elapsed().subsec_nanos());
}
