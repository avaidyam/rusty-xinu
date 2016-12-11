extern {
    fn kill(pid: i32) -> i32;
    fn getpid() -> i32;
}

#[no_mangle]
pub extern fn userret() {
    unsafe {
        kill(getpid());
    }
}
