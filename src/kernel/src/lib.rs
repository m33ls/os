#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use bootloader_api::BootInfo;
use bootloader_api::info::FrameBufferInfo;
use bootloader_boot_config::{BootConfig, LevelFilter};

pub mod serial;
//pub mod vga_buffer;
pub mod framebuffer;
pub mod interrupts;
pub mod gdt;
pub mod memory;
pub mod logger;

/// Initialize a text-based logger using the given pixel-based framebuffer as output.
pub fn init_logger(
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    log_level: LevelFilter,
    frame_buffer_logger_status: bool,
    serial_logger_status: bool,
) {
    let logger = logger::LOGGER.get_or_init(move || {
        logger::LockedLogger::new(
            framebuffer,
            info,
            frame_buffer_logger_status,
            serial_logger_status,
        )
    });
    log::set_logger(logger).expect("logger already set");
    log::set_max_level(convert_level(log_level));
    log::info!("Framebuffer info: {:?}\n", info);
}

fn convert_level(level: LevelFilter) -> log::LevelFilter {
    match level {
        LevelFilter::Off => log::LevelFilter::Off,
        LevelFilter::Error => log::LevelFilter::Error,
        LevelFilter::Warn => log::LevelFilter::Warn,
        LevelFilter::Info => log::LevelFilter::Info,
        LevelFilter::Debug => log::LevelFilter::Debug,
        LevelFilter::Trace => log::LevelFilter::Trace,
    }
}

pub fn init(framebuffer: bootloader_api::info::FrameBuffer) {
    // initialize logger
    //let framebuffer = boot_info.framebuffer.take().unwrap();
    let info = framebuffer.info();
    let buffer = framebuffer.into_buffer(); 
    init_logger(buffer, info, LevelFilter::Trace, true, true);
    
    // Initialize tables and enable interrupts
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        log::info!("{}...\t", core::any::type_name::<T>());
        self();
        log::info!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    log::info!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    log::info!("[failed]\n");
    log::info!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[cfg(test)]
use bootloader_api::{entry_point};

#[cfg(test)]
entry_point!(test_kernel_main);

/// Entry point for `cargo test`
#[cfg(test)]
fn test_kernel_main(boot_info: &'static mut BootInfo) -> ! {
    init(boot_info);
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}


