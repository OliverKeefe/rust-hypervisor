#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.boot")]
pub extern "C" fn boot() -> ! {
    unsafe {
        asm!(
            "la sp, __stack_top", // Loads __stack_top address into sp.
            "j {main}",          // Jump to main.
            main = sym main,    // Defines {main} in assembly.
            options(noreturn)  // No return from function.
        );
    }
}

unsafe extern "C" {
    static mut __bss: u8;
    static mut __bss_end: u8;
}

fn main() -> ! {
    // Fills BSS section with zeros.
    unsafe {
        let bss_start = &raw mut __bss;
        let bss_size = (&raw mut __bss_end as usize) - (&raw mut __bss as usize);
        core::ptr::write_bytes(bss_start, 0, bss_size);
    }

    loop{}
}

#[panic_handler]
pub fn panic_handler(info: &PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}