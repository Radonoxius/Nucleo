#![no_std]
#![no_main]

const RCC_BASE: u32 = 0x4002_3800;

const GPIO_A_BASE: u32 = 0x4002_0000;
const GPIO_C_BASE: u32 = 0x4002_0800;

const SYSCFG_BASE: u32 = 0x4001_3800;
const NVIC_ISE_BASE: u32 = 0xE000_E100;
const EXTI_BASE: u32 = 0x4001_3C00;

use core::ptr::write_volatile;

use nucleo_f4::{entry, mem::{set_bit_high, set_bit_low, to_address_mut, toggle_bit}};

entry!(main);
fn main() -> ! {
    unsafe {
        let rcc_ahb1en = to_address_mut(RCC_BASE, 0x30);
        set_bit_high(rcc_ahb1en, 0);
        set_bit_high(rcc_ahb1en, 2);

        let rcc_apb2en = to_address_mut(RCC_BASE, 0x44);
        set_bit_high(rcc_apb2en, 14);
        let nvic_ise1 = to_address_mut(NVIC_ISE_BASE, 0x4);
        write_volatile(nvic_ise1, 1 << 8);
        let syscfg_extic4 = to_address_mut(SYSCFG_BASE, 0x14);
        set_bit_low(syscfg_extic4, 4);
        set_bit_high(syscfg_extic4, 5);
        set_bit_low(syscfg_extic4, 6);
        set_bit_low(syscfg_extic4, 7);

        let exti_im = to_address_mut(EXTI_BASE, 0x00);
        set_bit_high(exti_im, 13);
        let exti_rts = to_address_mut(EXTI_BASE, 0x08);
        set_bit_low(exti_rts, 13);
        let exti_fts = to_address_mut(EXTI_BASE, 0x0C);
        set_bit_high(exti_fts, 13);

        let gpio_a_mode = to_address_mut(GPIO_A_BASE, 0x00);
        set_bit_high(gpio_a_mode, 10);
        set_bit_low(gpio_a_mode, 11);
        let gpio_a_ospeed = to_address_mut(GPIO_A_BASE, 0x08);
        set_bit_high(gpio_a_ospeed, 10);
        set_bit_high(gpio_a_ospeed, 11);

        let gpio_c_mode = to_address_mut(GPIO_C_BASE, 0x00);
        set_bit_low(gpio_c_mode, 26);
        set_bit_low(gpio_c_mode, 27);
        let gpio_c_ospeed = to_address_mut(GPIO_C_BASE, 0x08);
        set_bit_high(gpio_c_ospeed, 26);
        set_bit_high(gpio_c_ospeed, 27);

        let gpio_a_od = to_address_mut(GPIO_A_BASE, 0x14);
        set_bit_low(gpio_a_od, 5);
    }

    loop {}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PressHandler() {
    unsafe {
        let gpio_a_od = to_address_mut(GPIO_A_BASE, 0x14);
        toggle_bit(gpio_a_od, 5);

        let exti_p = to_address_mut(EXTI_BASE, 0x14);
        write_volatile(exti_p, 1 << 13);
    }
}

#[unsafe(link_section = ".vector_table")]
#[unsafe(no_mangle)]
pub static EXTI13_INTERRUPT_HANDLER: unsafe extern "C" fn() = PressHandler;