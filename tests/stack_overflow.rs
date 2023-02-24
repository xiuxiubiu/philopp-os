#![no_std]
#![no_main]
#![feature(custom_test_frameworks, abi_x86_interrupt)]
#![test_runner(philopp::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use lazy_static::lazy_static;
use philopp::{exit_qemu, gdt, serial_print, serial_println};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(philopp::QemuExitCode::Success);
    loop {}
}

#[no_mangle]
pub extern "x86-interrupt" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");

    philopp::gdt::init();
    TEST_IDT.load();

    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    philopp::test_panic_handler(info)
}

#[test_case]
#[allow(unconditional_recursion)]
fn stack_overflow() {
    fn stack_overflow() {
        stack_overflow();
    }
    stack_overflow();
}
