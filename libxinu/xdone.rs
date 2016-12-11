extern {
    fn halt();
    fn kprintf(fmt : *mut u8, ...) -> i32;
}

#[no_mangle]
pub extern fn xdone() {
    kprintf("\n\nAll user processes have completed.\n\n".as_ptr() as (*mut u8()));
    halt();
}
