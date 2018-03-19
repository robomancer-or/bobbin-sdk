#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::gpio::*;

periph!( GPIOA, Gpioa, GPIOA_PERIPH, GpioPeriph, 0x400ff000, 0x1c);
periph!( GPIOB, Gpiob, GPIOB_PERIPH, GpioPeriph, 0x400ff040, 0x1d);
periph!( GPIOC, Gpioc, GPIOC_PERIPH, GpioPeriph, 0x400ff080, 0x1e);
periph!( GPIOD, Gpiod, GPIOD_PERIPH, GpioPeriph, 0x400ff0c0, 0x1f);
periph!( GPIOE, Gpioe, GPIOE_PERIPH, GpioPeriph, 0x400ff100, 0x20);

channel!(PA0, Pa0, GPIOA, Gpioa, PA0_CH, GpioCh, GPIOA_PERIPH, 0);
channel!(PA1, Pa1, GPIOA, Gpioa, PA1_CH, GpioCh, GPIOA_PERIPH, 1);
channel!(PA2, Pa2, GPIOA, Gpioa, PA2_CH, GpioCh, GPIOA_PERIPH, 2);
channel!(PA3, Pa3, GPIOA, Gpioa, PA3_CH, GpioCh, GPIOA_PERIPH, 3);
channel!(PA4, Pa4, GPIOA, Gpioa, PA4_CH, GpioCh, GPIOA_PERIPH, 4);
channel!(PA5, Pa5, GPIOA, Gpioa, PA5_CH, GpioCh, GPIOA_PERIPH, 5);
channel!(PA6, Pa6, GPIOA, Gpioa, PA6_CH, GpioCh, GPIOA_PERIPH, 6);
channel!(PA7, Pa7, GPIOA, Gpioa, PA7_CH, GpioCh, GPIOA_PERIPH, 7);
channel!(PA8, Pa8, GPIOA, Gpioa, PA8_CH, GpioCh, GPIOA_PERIPH, 8);
channel!(PA9, Pa9, GPIOA, Gpioa, PA9_CH, GpioCh, GPIOA_PERIPH, 9);
channel!(PA10, Pa10, GPIOA, Gpioa, PA10_CH, GpioCh, GPIOA_PERIPH, 10);
channel!(PA11, Pa11, GPIOA, Gpioa, PA11_CH, GpioCh, GPIOA_PERIPH, 11);
channel!(PA12, Pa12, GPIOA, Gpioa, PA12_CH, GpioCh, GPIOA_PERIPH, 12);
channel!(PA13, Pa13, GPIOA, Gpioa, PA13_CH, GpioCh, GPIOA_PERIPH, 13);
channel!(PA14, Pa14, GPIOA, Gpioa, PA14_CH, GpioCh, GPIOA_PERIPH, 14);
channel!(PA15, Pa15, GPIOA, Gpioa, PA15_CH, GpioCh, GPIOA_PERIPH, 15);
channel!(PA16, Pa16, GPIOA, Gpioa, PA16_CH, GpioCh, GPIOA_PERIPH, 16);
channel!(PA17, Pa17, GPIOA, Gpioa, PA17_CH, GpioCh, GPIOA_PERIPH, 17);
channel!(PA18, Pa18, GPIOA, Gpioa, PA18_CH, GpioCh, GPIOA_PERIPH, 18);
channel!(PA19, Pa19, GPIOA, Gpioa, PA19_CH, GpioCh, GPIOA_PERIPH, 19);
channel!(PA24, Pa24, GPIOA, Gpioa, PA24_CH, GpioCh, GPIOA_PERIPH, 24);
channel!(PA25, Pa25, GPIOA, Gpioa, PA25_CH, GpioCh, GPIOA_PERIPH, 25);
channel!(PA26, Pa26, GPIOA, Gpioa, PA26_CH, GpioCh, GPIOA_PERIPH, 26);
channel!(PA27, Pa27, GPIOA, Gpioa, PA27_CH, GpioCh, GPIOA_PERIPH, 27);
channel!(PA28, Pa28, GPIOA, Gpioa, PA28_CH, GpioCh, GPIOA_PERIPH, 28);
channel!(PA29, Pa29, GPIOA, Gpioa, PA29_CH, GpioCh, GPIOA_PERIPH, 29);
channel!(PB0, Pb0, GPIOB, Gpiob, PB0_CH, GpioCh, GPIOB_PERIPH, 0);
channel!(PB1, Pb1, GPIOB, Gpiob, PB1_CH, GpioCh, GPIOB_PERIPH, 1);
channel!(PB2, Pb2, GPIOB, Gpiob, PB2_CH, GpioCh, GPIOB_PERIPH, 2);
channel!(PB3, Pb3, GPIOB, Gpiob, PB3_CH, GpioCh, GPIOB_PERIPH, 3);
channel!(PB4, Pb4, GPIOB, Gpiob, PB4_CH, GpioCh, GPIOB_PERIPH, 4);
channel!(PB5, Pb5, GPIOB, Gpiob, PB5_CH, GpioCh, GPIOB_PERIPH, 5);
channel!(PB6, Pb6, GPIOB, Gpiob, PB6_CH, GpioCh, GPIOB_PERIPH, 6);
channel!(PB7, Pb7, GPIOB, Gpiob, PB7_CH, GpioCh, GPIOB_PERIPH, 7);
channel!(PB8, Pb8, GPIOB, Gpiob, PB8_CH, GpioCh, GPIOB_PERIPH, 8);
channel!(PB9, Pb9, GPIOB, Gpiob, PB9_CH, GpioCh, GPIOB_PERIPH, 9);
channel!(PB10, Pb10, GPIOB, Gpiob, PB10_CH, GpioCh, GPIOB_PERIPH, 10);
channel!(PB11, Pb11, GPIOB, Gpiob, PB11_CH, GpioCh, GPIOB_PERIPH, 11);
channel!(PB12, Pb12, GPIOB, Gpiob, PB12_CH, GpioCh, GPIOB_PERIPH, 12);
channel!(PB13, Pb13, GPIOB, Gpiob, PB13_CH, GpioCh, GPIOB_PERIPH, 13);
channel!(PB16, Pb16, GPIOB, Gpiob, PB16_CH, GpioCh, GPIOB_PERIPH, 16);
channel!(PB17, Pb17, GPIOB, Gpiob, PB17_CH, GpioCh, GPIOB_PERIPH, 17);
channel!(PB18, Pb18, GPIOB, Gpiob, PB18_CH, GpioCh, GPIOB_PERIPH, 18);
channel!(PB19, Pb19, GPIOB, Gpiob, PB19_CH, GpioCh, GPIOB_PERIPH, 19);
channel!(PB20, Pb20, GPIOB, Gpiob, PB20_CH, GpioCh, GPIOB_PERIPH, 20);
channel!(PB21, Pb21, GPIOB, Gpiob, PB21_CH, GpioCh, GPIOB_PERIPH, 21);
channel!(PB22, Pb22, GPIOB, Gpiob, PB22_CH, GpioCh, GPIOB_PERIPH, 22);
channel!(PB23, Pb23, GPIOB, Gpiob, PB23_CH, GpioCh, GPIOB_PERIPH, 23);
channel!(PC0, Pc0, GPIOC, Gpioc, PC0_CH, GpioCh, GPIOC_PERIPH, 0);
channel!(PC1, Pc1, GPIOC, Gpioc, PC1_CH, GpioCh, GPIOC_PERIPH, 1);
channel!(PC2, Pc2, GPIOC, Gpioc, PC2_CH, GpioCh, GPIOC_PERIPH, 2);
channel!(PC3, Pc3, GPIOC, Gpioc, PC3_CH, GpioCh, GPIOC_PERIPH, 3);
channel!(PC4, Pc4, GPIOC, Gpioc, PC4_CH, GpioCh, GPIOC_PERIPH, 4);
channel!(PC5, Pc5, GPIOC, Gpioc, PC5_CH, GpioCh, GPIOC_PERIPH, 5);
channel!(PC6, Pc6, GPIOC, Gpioc, PC6_CH, GpioCh, GPIOC_PERIPH, 6);
channel!(PC7, Pc7, GPIOC, Gpioc, PC7_CH, GpioCh, GPIOC_PERIPH, 7);
channel!(PC8, Pc8, GPIOC, Gpioc, PC8_CH, GpioCh, GPIOC_PERIPH, 8);
channel!(PC9, Pc9, GPIOC, Gpioc, PC9_CH, GpioCh, GPIOC_PERIPH, 9);
channel!(PC10, Pc10, GPIOC, Gpioc, PC10_CH, GpioCh, GPIOC_PERIPH, 10);
channel!(PC11, Pc11, GPIOC, Gpioc, PC11_CH, GpioCh, GPIOC_PERIPH, 11);
channel!(PC12, Pc12, GPIOC, Gpioc, PC12_CH, GpioCh, GPIOC_PERIPH, 12);
channel!(PC13, Pc13, GPIOC, Gpioc, PC13_CH, GpioCh, GPIOC_PERIPH, 13);
channel!(PC14, Pc14, GPIOC, Gpioc, PC14_CH, GpioCh, GPIOC_PERIPH, 14);
channel!(PC15, Pc15, GPIOC, Gpioc, PC15_CH, GpioCh, GPIOC_PERIPH, 15);
channel!(PC16, Pc16, GPIOC, Gpioc, PC16_CH, GpioCh, GPIOC_PERIPH, 16);
channel!(PC17, Pc17, GPIOC, Gpioc, PC17_CH, GpioCh, GPIOC_PERIPH, 17);
channel!(PC18, Pc18, GPIOC, Gpioc, PC18_CH, GpioCh, GPIOC_PERIPH, 18);
channel!(PC19, Pc19, GPIOC, Gpioc, PC19_CH, GpioCh, GPIOC_PERIPH, 19);
channel!(PD0, Pd0, GPIOD, Gpiod, PD0_CH, GpioCh, GPIOD_PERIPH, 0);
channel!(PD1, Pd1, GPIOD, Gpiod, PD1_CH, GpioCh, GPIOD_PERIPH, 1);
channel!(PD2, Pd2, GPIOD, Gpiod, PD2_CH, GpioCh, GPIOD_PERIPH, 2);
channel!(PD3, Pd3, GPIOD, Gpiod, PD3_CH, GpioCh, GPIOD_PERIPH, 3);
channel!(PD4, Pd4, GPIOD, Gpiod, PD4_CH, GpioCh, GPIOD_PERIPH, 4);
channel!(PD5, Pd5, GPIOD, Gpiod, PD5_CH, GpioCh, GPIOD_PERIPH, 5);
channel!(PD6, Pd6, GPIOD, Gpiod, PD6_CH, GpioCh, GPIOD_PERIPH, 6);
channel!(PD7, Pd7, GPIOD, Gpiod, PD7_CH, GpioCh, GPIOD_PERIPH, 7);
channel!(PD8, Pd8, GPIOD, Gpiod, PD8_CH, GpioCh, GPIOD_PERIPH, 8);
channel!(PD9, Pd9, GPIOD, Gpiod, PD9_CH, GpioCh, GPIOD_PERIPH, 9);
channel!(PD10, Pd10, GPIOD, Gpiod, PD10_CH, GpioCh, GPIOD_PERIPH, 10);
channel!(PD11, Pd11, GPIOD, Gpiod, PD11_CH, GpioCh, GPIOD_PERIPH, 11);
channel!(PD12, Pd12, GPIOD, Gpiod, PD12_CH, GpioCh, GPIOD_PERIPH, 12);
channel!(PD13, Pd13, GPIOD, Gpiod, PD13_CH, GpioCh, GPIOD_PERIPH, 13);
channel!(PD14, Pd14, GPIOD, Gpiod, PD14_CH, GpioCh, GPIOD_PERIPH, 14);
channel!(PD15, Pd15, GPIOD, Gpiod, PD15_CH, GpioCh, GPIOD_PERIPH, 15);
channel!(PE0, Pe0, GPIOE, Gpioe, PE0_CH, GpioCh, GPIOE_PERIPH, 0);
channel!(PE1, Pe1, GPIOE, Gpioe, PE1_CH, GpioCh, GPIOE_PERIPH, 1);
channel!(PE2, Pe2, GPIOE, Gpioe, PE2_CH, GpioCh, GPIOE_PERIPH, 2);
channel!(PE3, Pe3, GPIOE, Gpioe, PE3_CH, GpioCh, GPIOE_PERIPH, 3);
channel!(PE4, Pe4, GPIOE, Gpioe, PE4_CH, GpioCh, GPIOE_PERIPH, 4);
channel!(PE5, Pe5, GPIOE, Gpioe, PE5_CH, GpioCh, GPIOE_PERIPH, 5);
channel!(PE6, Pe6, GPIOE, Gpioe, PE6_CH, GpioCh, GPIOE_PERIPH, 6);
channel!(PE7, Pe7, GPIOE, Gpioe, PE7_CH, GpioCh, GPIOE_PERIPH, 7);
channel!(PE8, Pe8, GPIOE, Gpioe, PE8_CH, GpioCh, GPIOE_PERIPH, 8);
channel!(PE9, Pe9, GPIOE, Gpioe, PE9_CH, GpioCh, GPIOE_PERIPH, 9);
channel!(PE10, Pe10, GPIOE, Gpioe, PE10_CH, GpioCh, GPIOE_PERIPH, 10);
channel!(PE11, Pe11, GPIOE, Gpioe, PE11_CH, GpioCh, GPIOE_PERIPH, 11);
channel!(PE12, Pe12, GPIOE, Gpioe, PE12_CH, GpioCh, GPIOE_PERIPH, 12);
channel!(PE24, Pe24, GPIOE, Gpioe, PE24_CH, GpioCh, GPIOE_PERIPH, 24);
channel!(PE25, Pe25, GPIOE, Gpioe, PE25_CH, GpioCh, GPIOE_PERIPH, 25);
channel!(PE26, Pe26, GPIOE, Gpioe, PE26_CH, GpioCh, GPIOE_PERIPH, 26);
channel!(PE27, Pe27, GPIOE, Gpioe, PE27_CH, GpioCh, GPIOE_PERIPH, 27);
channel!(PE28, Pe28, GPIOE, Gpioe, PE28_CH, GpioCh, GPIOE_PERIPH, 28);
