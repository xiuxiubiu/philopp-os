// don't link the Rust standard library
#![no_std]
// disable all Rust-level entry points
#![no_main]
// custom test framework
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

// including it only for tests
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

mod vga_buffer;

use core::panic::PanicInfo;

// static HELLO: &[u8] = b"Hello World!";

#[cfg(not(test))]
#[panic_handler]
// this funciton is called on panic
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// don't mangle the name of this function
#[no_mangle]
// this function is the entry point
// since the linker looks for a function named `_start` by default
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;
    //
    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0b00010001;
    //     }
    // }

    vga_buffer::print_something();

    use core::fmt::Write;
    vga_buffer::WRITER
        .lock()
        .write_str("\nHello again!")
        .unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        ", some numbers: {} {}\n",
        42,
        1.337
    )
    .unwrap();

    println!("custom println");

    // panic!("Some panic message!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 2);
    println!("[ok]");
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
