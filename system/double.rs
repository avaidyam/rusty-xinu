#![no_std]
#![feature(lang_items, libc)]
extern crate libc;

#[lang="panic_fmt"]
extern fn panic_fmt() -> !  { loop {} }

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[no_mangle]
pub extern fn double_input(input: i32) -> i32 {
	input * 2
}

