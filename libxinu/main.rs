#![no_main]

extern {
    fn recvclr();
    fn kprintf(fmt : *mut u8, ...) -> i32;
    fn getpid() -> i32;
}

#[no_mangle]
pub extern fn main() -> i32 {
    unsafe {
        recvclr();
        loop {
            kprintf("main(), hello\n\0".as_ptr() as (*mut u8));
            kprintf("getpid: %d\n\0".as_ptr() as (*mut u8), getpid());
        }
    }
}
