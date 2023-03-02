#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(philopp::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use philopp::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    // loop {}
    philopp::halt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    philopp::test_panic_handler(info)
}

#[test_case]
fn integration_test() {
    println!("integration testing");
    assert_eq!(1, 1);
}
