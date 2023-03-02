#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::{any::type_name, panic::PanicInfo};

use philopp::{exit_qemu, serial_print, serial_println};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(philopp::QemuExitCode::Success);

    // loop {}
    philopp::halt_loop();
}

trait PanicTestable {
    fn test(&self);
}

impl<T> PanicTestable for T
where
    T: Fn(),
{
    fn test(&self) {
        serial_print!("{}...\t", type_name::<T>());
        self();
        serial_println!("[test did not panic]");
    }
}

fn test_runner(tests: &[&dyn PanicTestable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.test();
        exit_qemu(philopp::QemuExitCode::Failed);
    }
    exit_qemu(philopp::QemuExitCode::Success);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // initilize exception
    philopp::init();

    test_main();

    // should_fail();
    // test_double_fault_exception();

    // loop {}
    philopp::halt_loop();
}

#[test_case]
fn should_fail() {
    assert_eq!(0, 1);
}

#[test_case]
fn should_double_fault_exception() {
    unsafe {
        *(0xabcdedfaa as *mut i32) = 42;
    }
}
