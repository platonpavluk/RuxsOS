#![no_std]
#![no_main]

use core::fmt::{self, Write};
use core::panic::PanicInfo;

mod vga_buffer;
mod keyboard;
mod commands;
mod filesystem;  // Модуль для файлової системи
use commands::process_commands;
use filesystem::FileSystem;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Очищаємо екран
    vga_buffer::clear_screen();

    // Виведення привітання та інформації про каталог
    vga_buffer::println("Welcome to RUXS!");
    vga_buffer::println("Type 'help' to see available commands.");

    // Обробка команд (можна залишити так, як є)
    process_commands();

    loop {}
}
