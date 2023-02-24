#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(philopp::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use philopp::{exit_qemu, serial_println};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(philopp::QemuExitCode::Success);
    loop {}
}

// fn test_runner(tests: &[&dyn Fn()]) {
//     serial_println!("Running {} tests", tests.len());
//     for test in tests {
//         test();
//         serial_println!("[test did not panic]");
//         exit_qemu(philopp::QemuExitCode::Failed);
//     }
//     exit_qemu(philopp::QemuExitCode::Success);
// }

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // initilize exception
    philopp::init();

    test_main();

    // should_fail();
    // test_double_fault_exception();

    loop {}
}

#[test_case]
fn should_fail() {
    serial_println!("should_panic::should_fail...\t");
    assert_eq!(0, 0);
}

#[test_case]
fn test_double_fault_exception() {
    unsafe {
        *(0xabcdedfaa as *mut i32) = 42;
    }
}
