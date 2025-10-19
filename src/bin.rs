use std::fs;

use lwst::Lwst;

fn main() {
	let lwst = Lwst::new(&fs::read("sub.lwst").unwrap()).unwrap();
	println!("{:#?}", lwst);
	for ix in 0..lwst.timing_array.len {
		let ix = lwst.timing_array.subtitle_ix_vec[ix];
		println!("{}\n", lwst.subtitle_table[ix as usize])
	}
}
