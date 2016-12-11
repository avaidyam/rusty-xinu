extern {
    static currpid: i32;
}

#[no_mangle]
pub extern fn getpid() -> i32 {
    unsafe {
        return currpid
    }
}
