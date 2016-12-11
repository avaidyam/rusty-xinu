//use core::fmt::Arguments;

//#![feature(lang_items, libc)]
//extern crate libc;

//#[lang="panic_fmt"]
//#[allow(unused_variables)]
//pub extern fn rust_begin_unwind() {}

//#[lang="eh_personality"]
//extern fn eh_personality() {}

//#[lang="panic_fmt"]
//#pallow(unused_variables)]
//extern fn panic_fmt() {}

//pub extern fn rust_begin_unwind(args: fmt::Arguments, msg: &str, line: u32) {}


#![no_std]
#![feature(lang_items, libc)]
#![allow(unused_variables)]

#[lang = "eh_personality"]
pub extern fn eh_personality() {}

#[lang = "panic_fmt"]
pub extern fn rust_begin_unwind() -> ! {
	loop {}
}

/*
#[allow(dead_code)]
#[export_name = "rust_begin_unwind"]
extern "C" fn panic_fmt(_msg: Arguments, _file: &'static str, _line: u32) -> ! {
    loop {}
}*/

#[no_mangle]
pub extern fn double_input(input: i16) -> i16 {
	input * 2
}

#[no_mangle]
pub extern fn __mulodi4(a: i64, b: i64, overflow: &mut i32) -> i64 {
	*overflow = 0;
	let result = a.wrapping_mul(b);
	if a == <i64>::min_value() {
		if b != 0 && b != 1 {
			*overflow = 1;
		}
		return result;
	}
	if b == <i64>::min_value() {
		if a != 0 && a != 1 {
			*overflow = 1;
		}
		return result;
	}

	let sa = a >> 63;
	let abs_a = (a ^ sa) - sa;
	let sb = b >> 63;
	let abs_b = (b ^ sb) - sb;
	if abs_a < 2 || abs_b < 2 {
		return result;
	}
	if sa == sb {
		if abs_a > <i64>::max_value() / abs_b {
			*overflow = 1;
		}
	} else {
		if abs_a > <i64>::min_value() / -abs_b {
			*overflow = 1;
		}
	}
	result
}
