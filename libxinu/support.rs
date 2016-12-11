#[lang = "eh_personality"]
pub extern fn eh_personality() {}

#[lang = "panic_fmt"]
pub extern fn rust_begin_unwind() -> ! { loop {} }

#[no_mangle]
pub extern fn __mulodi4() -> i64 {
    unimplemented!()
}
