#![no_std]
#![no_main]
// #![feature(custom_test_frameworks)]
// #![test_runner(test_runner)]
// #![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use philopp::{exit_qemu, serial_prinln};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_prinln!("[ok]");
    exit_qemu(philopp::QemuExitCode::Success);
    loop {}
}

// fn test_runner(tests: &[&dyn Fn()]) {
//     serial_prinln!("Running {} tests", tests.len());
//     for test in tests {
//         test();
//         serial_prinln!("[test did not panic]");
//         exit_qemu(philopp::QemuExitCode::Failed);
//     }
//     exit_qemu(philopp::QemuExitCode::Success);
// }

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // test_main();

    should_fail();

    loop {}
}

// #[test_case]
fn should_fail() {
    serial_prinln!("should_panic::should_fail...\t");
    assert_eq!(0, 0);
}
