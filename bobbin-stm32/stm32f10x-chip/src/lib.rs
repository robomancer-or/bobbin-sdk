#![no_std]
extern crate bobbin_cortexm;
pub use bobbin_cortexm::chip::exc;
pub use bobbin_cortexm::chip::nvic;
pub use bobbin_cortexm::chip::scb;
pub use bobbin_cortexm::chip::systick;
pub use bobbin_cortexm::chip::mpu;
pub use bobbin_cortexm::chip::dcb;
pub use bobbin_cortexm::chip::itm;

extern crate stm32_common;

pub mod irq;
pub mod sig;
pub mod rcc;
pub mod flash;
pub mod pwr;
pub mod syscfg;
pub mod dbg;
pub mod bkp;
pub mod fsmc;
pub mod afio;
pub mod c_adc;
pub mod iwdg;
pub mod wwdg;
pub mod crc;
pub mod exti;
pub mod tim_gen;
pub mod tim_adv;
pub mod adc;
pub mod spi;
pub mod i2c;
pub mod gpio;
pub mod usart;
pub mod dma;
