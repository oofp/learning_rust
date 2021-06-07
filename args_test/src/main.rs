use std::env::{args,Args};
use std::iter::Skip;

fn main() {
    println!("Hello, arguments!");
    let arg_it = args();

    let arg_len = arg_it.len();
    println! ("arguments:{}",arg_len);

    //let _skip_app = arg_it.next();
    let args2:Skip<Args> = arg_it.skip(1); 

    for a in args2 {
        println! ("{}",a)
    }

    let args3 = args();
    for a in args3 {
        println! ("{:?}",a.parse::<u32>())
    }
}
