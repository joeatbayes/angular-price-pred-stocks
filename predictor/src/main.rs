pub mod bar_parser;
//pub use crate::bar_parser as parser;
use crate::bar_parser::bar_parser as parser;

fn main() {
    println!("Hello, world!");

    let hm = parser::read_file(&"../data/bars/SPY.csv");
    println!("hm={0:#?}", hm);

    let (dts, opens) = hm.slice(1,15);
    println!("dts={0:#?} opens={1:#?} ", dts, opens);
}
