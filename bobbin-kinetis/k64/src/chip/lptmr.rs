#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::lptmr::*;

periph!(LptmrPeriph, LPTMR0, Lptmr0, 0x40040000);



