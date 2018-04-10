extern crate libc;
extern crate rand;
use rand::{Rng, thread_rng};

fn main() {
	let mut rng = thread_rng();
	let mut t: i64 = 0;
	let increment_size = 128;
	let mut increment: Vec<i32> = vec![1;increment_size];
	let mut increment_index = 0;
	increment = gen_pattern(increment);
	loop {
		unsafe {
			libc::putchar(gen_s(t) as i32);
			libc::putchar(gen_b(t) as i32);
			libc::putchar(gen_c(t) as i32);
		}
		t+=(increment[increment_index] as i64);
		//println!("increment: {} {}", increment_index, increment[increment_index]);
		if t>=30000 {
			t=0;
			increment_index+=1;
		}
		if increment_index >= increment_size {
			//increment = gen_pattern(increment);
			increment_index = 0;
		}
	}
}

fn gen_s(t: i64) -> i64 {
	let mut s: i64 = 0;
	s = (t-(t>>4&t>>8)&t>>12)-2+(t>>4&t>>6);
	return s;
}

fn gen_b(t: i64) -> i64 {
	let mut s: i64 = 0;
	s = (t-(t>>4&t>>8)&t>>12)+(t>>4&t>>6);
	return s;
}

fn gen_c(t: i64) -> i64 {
	let mut s: i64 = 0;
	s = t%(14312^t*3>>3^t>>10);
	s
}

fn gen_pattern(mut increment: Vec<i32>) -> Vec<i32> {
	for index in 0..increment.len() {
		let mut rng = thread_rng();
		increment[index] = rng.gen_range(1, 2);
	}
	increment
}
