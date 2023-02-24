use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

use crate::{gdt::DOUBLE_FAULT_IST_INDEX, println};

// static IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        // idt.page_fault.set_handler_fn(page_fault_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler).
                set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_idt() {
    // unsafe {
    //     IDT.breakpoint.set_handler_fn(breakpoint_handler);
    //     IDT.load();
    // }

    IDT.load();
}

// breakpoint exception handler
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

#[allow(dead_code)]
// page fault exception handler
extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    println!(
        "EXCEPTION: PAGE_FAULT\nERROR_CODE:\n{:#?}\n{:#?}",
        error_code, stack_frame
    );
}

// double fault exception handler
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    panic!(
        "EXCEPTION: DOUBLE_FAULT\nERROR_CODE: {}\n{:#?}",
        error_code, stack_frame
    );
}

// #[test_case]
// fn test_breakpoint_exception() {
//     // invoke a breakpoint exception
//     x86_64::instructions::interrupts::int3();
// }
