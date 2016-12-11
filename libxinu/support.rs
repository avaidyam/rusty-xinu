#[lang = "eh_personality"]
pub extern fn eh_personality() {}

#[lang = "panic_fmt"]
pub extern fn rust_begin_unwind() -> ! {
	loop {}
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
