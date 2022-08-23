mod parse;
use std::fs;
use std::io::Write;

fn main() {

    // let mut file = fs::File::create("foo.txt").unwrap();
    // file.write_all(parse::main()).unwrap();
    // let contents = fs::read_to_string("foo.txt").unwrap();
    // assert_eq!(contents, "nju33");

    #[warn(unused_must_use)]
    let res = parse::main();
    println!("{:?}",res);


    // In order to Open from Ok() wrapping, have to unwrap()
    for r in res.unwrap().iter() {
        println!("{:?}", r);
    }
}
