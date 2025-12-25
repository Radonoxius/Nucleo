use core::ptr::{read_volatile, write_volatile};

pub unsafe fn set_bit_high(addr: *mut u32, bit_index: u8) {
    let current_value = unsafe {
        read_volatile(addr)
    };

    unsafe {
        write_volatile(addr, current_value | (1 << bit_index));
    }
}

pub unsafe fn set_bit_low(addr: *mut u32, bit_index: u8) {
    let current_value = unsafe {
        read_volatile(addr)
    };

    unsafe {
        write_volatile(addr, current_value & !(1 << bit_index));
    }
}

pub unsafe fn toggle_bit(addr: *mut u32, bit_index: u8) {
    let current_value = unsafe {
        read_volatile(addr)
    };

    unsafe {
        write_volatile(addr, current_value ^ (1 << bit_index));
    }
}

pub unsafe fn to_address(base_address: u32, offset: u32) -> *const u32 {
    (base_address + offset) as *const u32
}

pub unsafe fn to_address_mut(base_address: u32, offset: u32) -> *mut u32 {
    (base_address + offset) as *mut u32
}