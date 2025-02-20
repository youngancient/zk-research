// use field_tracker::{Ft, start_tscope, end_tscope, print_summary, summary};

// type Fq = Ft!(ark_bn254::Fq);

// fn main() {
// 	let mut a = Fq::from(3);
// 	let mut b = Fq::from(5);
// 	let mut c = a + b;

// 	start_tscope!("Layer 2");

// 	a += c;
// 	b += c;
// 	let mut d = a * b;

// 	end_tscope!();

// 	d -= c;

// 	print_summary!();
// }