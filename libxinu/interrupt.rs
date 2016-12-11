
#[no_mangle]
pub fn disable1() {
    unsafe { asm!("cli") };
}

#[no_mangle]
pub fn disable_all() {
    disable1();
    non_maskable::disable();
}

#[no_mangle]
pub fn enable1() {
    unsafe { asm!("sti") };
}

#[no_mangle]
pub fn enable_all() {
    enable1();
    non_maskable::enable();
}

/// An IO port address.
pub struct Address(pub u16);

/// An IO port.
pub struct Port
{
    pub address: Address,
}

impl Port
{
    pub fn open(address: Address) -> Self {
        Port {
            address: address,
        }
    }

    pub fn read_u8(&mut self) -> u8 {
        let value: u8;
        unsafe {
            asm!("in $0, $1" : "={al}"(value) : "{dx}"(self.address.0) : "memory" : "intel", "volatile");
        }
        value
    }

    pub fn write_u8(&mut self, value: u8) {
        unsafe {
            asm!("out $1, $0" : : "{al}"(value), "{dx}"(self.address.0) : "memory" : "intel", "volatile");
        }
    }
}

pub mod non_maskable
{

    pub use super::Port;
    pub use super::Address;

    pub const PORT_ADDRESS: Address = Address(0x70);

    pub fn disable() {
        let val = port().read_u8() | 0x80;
        port().write_u8(val);
    }

    pub fn enable() {
        let val = port().read_u8() & 0x7f;
        port().write_u8(val);
    }

    fn port() -> Port { Port::open(PORT_ADDRESS) }
}

#[naked]
pub fn interrupt_handler() {
    unsafe {
        asm!("pushal");
        asm!("cld");

        // Do something.

        asm!("popal");
        asm!("iret");
    }
}
