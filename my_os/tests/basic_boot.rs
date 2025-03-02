#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(my_os::test_runner)]

use core::panic::PanicInfo;
use my_os::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    my_os::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("Test_println output");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

