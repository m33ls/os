#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader_api::{BootInfo, entry_point};
use bootloader_api::config::{BootloaderConfig, Mapping};
use kernel::println;

//mod vga_buffer;
mod framebuffer;
mod serial;

// Print to VGA text buffer on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("{}", info);
    kernel::hlt_loop();
}

// Print to serial on panic in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    
    //let mut framebuffer = unsafe { core::ptr::read(&info.framebuffer) };

    /*
    use os::memory;
    use os::memory::BootInfoFrameAllocator;
    use x86_64::{structures::paging::{Translate, Page}, VirtAddr};
    */

    let framebuffer = boot_info.framebuffer.take().unwrap();

    kernel::init(framebuffer);

    println!("Hello World!");

    /*
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physcial address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};
    */

    #[cfg(test)]
    test_main();

    println!("Success!");
    // panic!("Test message");

    //loop {}
    kernel::hlt_loop();
}
