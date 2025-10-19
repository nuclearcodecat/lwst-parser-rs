use std::fs;

use lwst::Lwst;

fn main() {
	let lwst = Lwst::new(&fs::read("sub.lwst").unwrap()).unwrap();
	println!("{:#?}", lwst);
}
