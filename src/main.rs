// don't link the Rust standard library
#![no_std]
// disable all Rust-level entry points
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(philopp::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

use core::panic::PanicInfo;

// static HELLO: &[u8] = b"Hello World!";

// our existing panic handler
#[panic_handler]
// this funciton is called on panic
fn panic(info: &PanicInfo) -> ! {
    philopp::test_panic_handler(info)
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
fn one_eq_one() {
    assert_eq!(1, 1);
}
