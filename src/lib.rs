#![no_std]
#![cfg_attr(test, no_main)]
// custom test framework
#![feature(custom_test_frameworks, abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod interrupts;
pub mod serial;
pub mod vga_buffer;

use core::panic::PanicInfo;

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) -> () {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_prinln!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_prinln!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_prinln!("[failed]\n");
    serial_prinln!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

#[test_case]
fn trivial_assertion() {
    // print!("trivial assertion... ");
    assert_eq!(1, 1);
    // serial_prinln!("trivial assertion...");
    // loop {}
}

pub fn init() {
    interrupts::init_idt();
}
