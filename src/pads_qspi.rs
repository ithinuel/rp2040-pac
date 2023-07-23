#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Voltage select. Per bank control"]
    pub voltage_select: VOLTAGE_SELECT,
    _reserved_1_gpio_qspi: [u8; 0x8c],
}
impl RegisterBlock {
    #[doc = "0x04..0x7c - Pad control register"]
    #[inline(always)]
    pub const fn gpio_qspi_sclk(&self) -> &[GPIO_QSPI_SCLK; 30] {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x08..0x80 - Pad control register"]
    #[inline(always)]
    pub const fn gpio_qspi_sd0(&self) -> &[GPIO_QSPI_SD0; 30] {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    #[doc = "0x0c..0x84 - Pad control register"]
    #[inline(always)]
    pub const fn gpio_qspi_sd1(&self) -> &[GPIO_QSPI_SD1; 30] {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x10..0x88 - Pad control register"]
    #[inline(always)]
    pub const fn gpio_qspi_sd2(&self) -> &[GPIO_QSPI_SD2; 30] {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x14..0x8c - Pad control register"]
    #[inline(always)]
    pub const fn gpio_qspi_sd3(&self) -> &[GPIO_QSPI_SD3; 30] {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
    #[doc = "0x18..0x90 - Pad control register"]
    #[inline(always)]
    pub const fn gpio_qspi_ss(&self) -> &[GPIO_QSPI_SS; 30] {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
}
pub use crate::pads_bank0::gpio as gpio_qspi_sclk;
pub use crate::pads_bank0::gpio as gpio_qspi_sd0;
pub use crate::pads_bank0::gpio as gpio_qspi_sd1;
pub use crate::pads_bank0::gpio as gpio_qspi_sd2;
pub use crate::pads_bank0::gpio as gpio_qspi_sd3;
pub use crate::pads_bank0::gpio as gpio_qspi_ss;
pub use crate::pads_bank0::voltage_select;
pub use crate::pads_bank0::GPIO as GPIO_QSPI_SCLK;
pub use crate::pads_bank0::GPIO as GPIO_QSPI_SD0;
pub use crate::pads_bank0::GPIO as GPIO_QSPI_SD1;
pub use crate::pads_bank0::GPIO as GPIO_QSPI_SD2;
pub use crate::pads_bank0::GPIO as GPIO_QSPI_SD3;
pub use crate::pads_bank0::GPIO as GPIO_QSPI_SS;
pub use crate::pads_bank0::VOLTAGE_SELECT;
