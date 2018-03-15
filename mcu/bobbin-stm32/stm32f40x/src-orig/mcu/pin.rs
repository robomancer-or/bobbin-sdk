#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::bobbin_common::pin::*;
pub use ::bobbin_common::gate::*;
pub use super::gpio::*;

pin!(PA0, Pa0, GPIOA, Gpioa, PA0_PIN, GpioPin, GPIOA_PERIPH, 0);
   pin_source!(Pa0, super::adc::Adc1Ch0, super::sig::SigAdc, 0);
   pin_source!(Pa0, super::adc::Adc2Ch0, super::sig::SigAdc, 0);
   pin_source!(Pa0, super::adc::Adc3Ch0, super::sig::SigAdc, 0);
   pin_source!(Pa0, super::tim_gen::Tim2Ch1, super::sig::SigTim, 1);
   pin_source!(Pa0, super::tim_gen::Tim5Ch1, super::sig::SigTim, 2);
   pin_source!(Pa0, super::usart::Usart2, super::sig::SigCts, 7);
   pin_source!(Pa0, super::usart::Uart4, super::sig::SigTx, 8);
pin!(PA1, Pa1, GPIOA, Gpioa, PA1_PIN, GpioPin, GPIOA_PERIPH, 1);
   pin_source!(Pa1, super::adc::Adc1Ch1, super::sig::SigAdc, 0);
   pin_source!(Pa1, super::adc::Adc2Ch1, super::sig::SigAdc, 0);
   pin_source!(Pa1, super::adc::Adc3Ch1, super::sig::SigAdc, 0);
   pin_source!(Pa1, super::tim_gen::Tim2Ch2, super::sig::SigTim, 1);
   pin_source!(Pa1, super::tim_gen::Tim5Ch2, super::sig::SigTim, 2);
   pin_source!(Pa1, super::usart::Usart2, super::sig::SigRts, 7);
   pin_source!(Pa1, super::usart::Uart4, super::sig::SigRx, 8);
pin!(PA2, Pa2, GPIOA, Gpioa, PA2_PIN, GpioPin, GPIOA_PERIPH, 2);
   pin_source!(Pa2, super::adc::Adc1Ch2, super::sig::SigAdc, 0);
   pin_source!(Pa2, super::adc::Adc2Ch2, super::sig::SigAdc, 0);
   pin_source!(Pa2, super::adc::Adc3Ch2, super::sig::SigAdc, 0);
   pin_source!(Pa2, super::tim_gen::Tim2Ch3, super::sig::SigTim, 1);
   pin_source!(Pa2, super::tim_gen::Tim5Ch3, super::sig::SigTim, 2);
   pin_source!(Pa2, super::tim_gen::Tim9Ch1, super::sig::SigTim, 3);
   pin_source!(Pa2, super::usart::Usart2, super::sig::SigTx, 7);
pin!(PA3, Pa3, GPIOA, Gpioa, PA3_PIN, GpioPin, GPIOA_PERIPH, 3);
   pin_source!(Pa3, super::adc::Adc1Ch3, super::sig::SigAdc, 0);
   pin_source!(Pa3, super::adc::Adc2Ch3, super::sig::SigAdc, 0);
   pin_source!(Pa3, super::adc::Adc3Ch3, super::sig::SigAdc, 0);
   pin_source!(Pa3, super::tim_gen::Tim2Ch4, super::sig::SigTim, 1);
   pin_source!(Pa3, super::tim_gen::Tim5Ch4, super::sig::SigTim, 2);
   pin_source!(Pa3, super::tim_gen::Tim9Ch2, super::sig::SigTim, 3);
   pin_source!(Pa3, super::usart::Usart2, super::sig::SigRx, 7);
pin!(PA4, Pa4, GPIOA, Gpioa, PA4_PIN, GpioPin, GPIOA_PERIPH, 4);
   pin_source!(Pa4, super::adc::Adc1Ch4, super::sig::SigAdc, 0);
   pin_source!(Pa4, super::adc::Adc2Ch4, super::sig::SigAdc, 0);
   pin_source!(Pa4, super::usart::Usart2, super::sig::SigCk, 7);
pin!(PA5, Pa5, GPIOA, Gpioa, PA5_PIN, GpioPin, GPIOA_PERIPH, 5);
   pin_source!(Pa5, super::adc::Adc1Ch5, super::sig::SigAdc, 0);
   pin_source!(Pa5, super::adc::Adc2Ch5, super::sig::SigAdc, 0);
   pin_source!(Pa5, super::tim_gen::Tim2Ch1, super::sig::SigTim, 1);
pin!(PA6, Pa6, GPIOA, Gpioa, PA6_PIN, GpioPin, GPIOA_PERIPH, 6);
   pin_source!(Pa6, super::adc::Adc1Ch6, super::sig::SigAdc, 0);
   pin_source!(Pa6, super::adc::Adc2Ch6, super::sig::SigAdc, 0);
   pin_source!(Pa6, super::tim_gen::Tim3Ch1, super::sig::SigTim, 2);
   pin_source!(Pa6, super::tim_gen::Tim13Ch1, super::sig::SigTim, 9);
pin!(PA7, Pa7, GPIOA, Gpioa, PA7_PIN, GpioPin, GPIOA_PERIPH, 7);
   pin_source!(Pa7, super::adc::Adc1Ch7, super::sig::SigAdc, 0);
   pin_source!(Pa7, super::adc::Adc2Ch7, super::sig::SigAdc, 0);
   pin_source!(Pa7, super::tim_gen::Tim3Ch2, super::sig::SigTim, 2);
   pin_source!(Pa7, super::tim_gen::Tim14Ch1, super::sig::SigTim, 9);
pin!(PA8, Pa8, GPIOA, Gpioa, PA8_PIN, GpioPin, GPIOA_PERIPH, 8);
   pin_source!(Pa8, super::tim_adv::Tim1Ch1, super::sig::SigTim, 1);
   pin_source!(Pa8, super::usart::Usart1, super::sig::SigCk, 7);
pin!(PA9, Pa9, GPIOA, Gpioa, PA9_PIN, GpioPin, GPIOA_PERIPH, 9);
   pin_source!(Pa9, super::tim_adv::Tim1Ch2, super::sig::SigTim, 1);
   pin_source!(Pa9, super::usart::Usart1, super::sig::SigTx, 7);
pin!(PA10, Pa10, GPIOA, Gpioa, PA10_PIN, GpioPin, GPIOA_PERIPH, 10);
   pin_source!(Pa10, super::tim_adv::Tim1Ch3, super::sig::SigTim, 1);
   pin_source!(Pa10, super::usart::Usart1, super::sig::SigRx, 7);
pin!(PA11, Pa11, GPIOA, Gpioa, PA11_PIN, GpioPin, GPIOA_PERIPH, 11);
   pin_source!(Pa11, super::tim_adv::Tim1Ch4, super::sig::SigTim, 1);
   pin_source!(Pa11, super::usart::Usart1, super::sig::SigCts, 7);
pin!(PA12, Pa12, GPIOA, Gpioa, PA12_PIN, GpioPin, GPIOA_PERIPH, 12);
   pin_source!(Pa12, super::usart::Usart1, super::sig::SigRts, 7);
pin!(PA13, Pa13, GPIOA, Gpioa, PA13_PIN, GpioPin, GPIOA_PERIPH, 13);
pin!(PA14, Pa14, GPIOA, Gpioa, PA14_PIN, GpioPin, GPIOA_PERIPH, 14);
pin!(PA15, Pa15, GPIOA, Gpioa, PA15_PIN, GpioPin, GPIOA_PERIPH, 15);
   pin_source!(Pa15, super::tim_gen::Tim2Ch1, super::sig::SigTim, 1);
pin!(PB0, Pb0, GPIOB, Gpiob, PB0_PIN, GpioPin, GPIOB_PERIPH, 0);
   pin_source!(Pb0, super::adc::Adc1Ch8, super::sig::SigAdc, 0);
   pin_source!(Pb0, super::adc::Adc2Ch8, super::sig::SigAdc, 0);
   pin_source!(Pb0, super::tim_gen::Tim3Ch3, super::sig::SigTim, 2);
pin!(PB1, Pb1, GPIOB, Gpiob, PB1_PIN, GpioPin, GPIOB_PERIPH, 1);
   pin_source!(Pb1, super::adc::Adc1Ch9, super::sig::SigAdc, 0);
   pin_source!(Pb1, super::adc::Adc2Ch9, super::sig::SigAdc, 0);
   pin_source!(Pb1, super::tim_gen::Tim3Ch4, super::sig::SigTim, 2);
pin!(PB2, Pb2, GPIOB, Gpiob, PB2_PIN, GpioPin, GPIOB_PERIPH, 2);
pin!(PB3, Pb3, GPIOB, Gpiob, PB3_PIN, GpioPin, GPIOB_PERIPH, 3);
   pin_source!(Pb3, super::tim_gen::Tim2Ch2, super::sig::SigTim, 1);
pin!(PB4, Pb4, GPIOB, Gpiob, PB4_PIN, GpioPin, GPIOB_PERIPH, 4);
   pin_source!(Pb4, super::tim_gen::Tim3Ch1, super::sig::SigTim, 2);
pin!(PB5, Pb5, GPIOB, Gpiob, PB5_PIN, GpioPin, GPIOB_PERIPH, 5);
   pin_source!(Pb5, super::tim_gen::Tim3Ch2, super::sig::SigTim, 2);
pin!(PB6, Pb6, GPIOB, Gpiob, PB6_PIN, GpioPin, GPIOB_PERIPH, 6);
   pin_source!(Pb6, super::tim_gen::Tim4Ch1, super::sig::SigTim, 2);
   pin_source!(Pb6, super::usart::Usart1, super::sig::SigTx, 7);
pin!(PB7, Pb7, GPIOB, Gpiob, PB7_PIN, GpioPin, GPIOB_PERIPH, 7);
   pin_source!(Pb7, super::tim_gen::Tim4Ch2, super::sig::SigTim, 2);
   pin_source!(Pb7, super::usart::Usart1, super::sig::SigRx, 7);
pin!(PB8, Pb8, GPIOB, Gpiob, PB8_PIN, GpioPin, GPIOB_PERIPH, 8);
   pin_source!(Pb8, super::tim_gen::Tim4Ch3, super::sig::SigTim, 2);
   pin_source!(Pb8, super::tim_gen::Tim10Ch1, super::sig::SigTim, 3);
pin!(PB9, Pb9, GPIOB, Gpiob, PB9_PIN, GpioPin, GPIOB_PERIPH, 9);
   pin_source!(Pb9, super::tim_gen::Tim4Ch4, super::sig::SigTim, 2);
   pin_source!(Pb9, super::tim_gen::Tim11Ch1, super::sig::SigTim, 3);
pin!(PB10, Pb10, GPIOB, Gpiob, PB10_PIN, GpioPin, GPIOB_PERIPH, 10);
   pin_source!(Pb10, super::tim_gen::Tim2Ch3, super::sig::SigTim, 1);
   pin_source!(Pb10, super::usart::Usart3, super::sig::SigTx, 7);
pin!(PB11, Pb11, GPIOB, Gpiob, PB11_PIN, GpioPin, GPIOB_PERIPH, 11);
   pin_source!(Pb11, super::tim_gen::Tim2Ch4, super::sig::SigTim, 1);
   pin_source!(Pb11, super::usart::Usart3, super::sig::SigRx, 7);
pin!(PB12, Pb12, GPIOB, Gpiob, PB12_PIN, GpioPin, GPIOB_PERIPH, 12);
   pin_source!(Pb12, super::usart::Usart3, super::sig::SigCk, 7);
pin!(PB13, Pb13, GPIOB, Gpiob, PB13_PIN, GpioPin, GPIOB_PERIPH, 13);
   pin_source!(Pb13, super::usart::Usart3, super::sig::SigCts, 7);
pin!(PB14, Pb14, GPIOB, Gpiob, PB14_PIN, GpioPin, GPIOB_PERIPH, 14);
   pin_source!(Pb14, super::usart::Usart3, super::sig::SigRts, 7);
   pin_source!(Pb14, super::tim_gen::Tim12Ch1, super::sig::SigTim, 9);
pin!(PB15, Pb15, GPIOB, Gpiob, PB15_PIN, GpioPin, GPIOB_PERIPH, 15);
   pin_source!(Pb15, super::tim_gen::Tim12Ch2, super::sig::SigTim, 9);
pin!(PC0, Pc0, GPIOC, Gpioc, PC0_PIN, GpioPin, GPIOC_PERIPH, 0);
   pin_source!(Pc0, super::adc::Adc1Ch10, super::sig::SigAdc, 0);
   pin_source!(Pc0, super::adc::Adc2Ch10, super::sig::SigAdc, 0);
   pin_source!(Pc0, super::adc::Adc3Ch10, super::sig::SigAdc, 0);
pin!(PC1, Pc1, GPIOC, Gpioc, PC1_PIN, GpioPin, GPIOC_PERIPH, 1);
   pin_source!(Pc1, super::adc::Adc1Ch11, super::sig::SigAdc, 0);
   pin_source!(Pc1, super::adc::Adc2Ch11, super::sig::SigAdc, 0);
   pin_source!(Pc1, super::adc::Adc3Ch11, super::sig::SigAdc, 0);
pin!(PC2, Pc2, GPIOC, Gpioc, PC2_PIN, GpioPin, GPIOC_PERIPH, 2);
   pin_source!(Pc2, super::adc::Adc1Ch12, super::sig::SigAdc, 0);
   pin_source!(Pc2, super::adc::Adc2Ch12, super::sig::SigAdc, 0);
   pin_source!(Pc2, super::adc::Adc3Ch12, super::sig::SigAdc, 0);
pin!(PC3, Pc3, GPIOC, Gpioc, PC3_PIN, GpioPin, GPIOC_PERIPH, 3);
   pin_source!(Pc3, super::adc::Adc1Ch13, super::sig::SigAdc, 0);
   pin_source!(Pc3, super::adc::Adc2Ch13, super::sig::SigAdc, 0);
   pin_source!(Pc3, super::adc::Adc3Ch13, super::sig::SigAdc, 0);
pin!(PC4, Pc4, GPIOC, Gpioc, PC4_PIN, GpioPin, GPIOC_PERIPH, 4);
   pin_source!(Pc4, super::adc::Adc1Ch14, super::sig::SigAdc, 0);
   pin_source!(Pc4, super::adc::Adc2Ch14, super::sig::SigAdc, 0);
pin!(PC5, Pc5, GPIOC, Gpioc, PC5_PIN, GpioPin, GPIOC_PERIPH, 5);
   pin_source!(Pc5, super::adc::Adc1Ch15, super::sig::SigAdc, 0);
   pin_source!(Pc5, super::adc::Adc2Ch15, super::sig::SigAdc, 0);
pin!(PC6, Pc6, GPIOC, Gpioc, PC6_PIN, GpioPin, GPIOC_PERIPH, 6);
   pin_source!(Pc6, super::tim_gen::Tim3Ch1, super::sig::SigTim, 2);
   pin_source!(Pc6, super::tim_adv::Tim8Ch1, super::sig::SigTim, 3);
   pin_source!(Pc6, super::usart::Usart6, super::sig::SigTx, 8);
pin!(PC7, Pc7, GPIOC, Gpioc, PC7_PIN, GpioPin, GPIOC_PERIPH, 7);
   pin_source!(Pc7, super::tim_gen::Tim3Ch2, super::sig::SigTim, 2);
   pin_source!(Pc7, super::tim_adv::Tim8Ch2, super::sig::SigTim, 3);
   pin_source!(Pc7, super::usart::Usart6, super::sig::SigRx, 8);
pin!(PC8, Pc8, GPIOC, Gpioc, PC8_PIN, GpioPin, GPIOC_PERIPH, 8);
   pin_source!(Pc8, super::tim_gen::Tim3Ch3, super::sig::SigTim, 2);
   pin_source!(Pc8, super::tim_adv::Tim8Ch3, super::sig::SigTim, 3);
   pin_source!(Pc8, super::usart::Usart6, super::sig::SigCk, 8);
pin!(PC9, Pc9, GPIOC, Gpioc, PC9_PIN, GpioPin, GPIOC_PERIPH, 9);
   pin_source!(Pc9, super::tim_gen::Tim3Ch4, super::sig::SigTim, 2);
   pin_source!(Pc9, super::tim_adv::Tim8Ch4, super::sig::SigTim, 3);
pin!(PC10, Pc10, GPIOC, Gpioc, PC10_PIN, GpioPin, GPIOC_PERIPH, 10);
   pin_source!(Pc10, super::usart::Usart3, super::sig::SigTx, 7);
   pin_source!(Pc10, super::usart::Uart4, super::sig::SigTx, 8);
pin!(PC11, Pc11, GPIOC, Gpioc, PC11_PIN, GpioPin, GPIOC_PERIPH, 11);
   pin_source!(Pc11, super::usart::Usart3, super::sig::SigRx, 7);
   pin_source!(Pc11, super::usart::Uart4, super::sig::SigRx, 8);
pin!(PC12, Pc12, GPIOC, Gpioc, PC12_PIN, GpioPin, GPIOC_PERIPH, 12);
   pin_source!(Pc12, super::usart::Usart3, super::sig::SigCk, 7);
   pin_source!(Pc12, super::usart::Uart5, super::sig::SigTx, 8);
pin!(PC13, Pc13, GPIOC, Gpioc, PC13_PIN, GpioPin, GPIOC_PERIPH, 13);
pin!(PC14, Pc14, GPIOC, Gpioc, PC14_PIN, GpioPin, GPIOC_PERIPH, 14);
pin!(PC15, Pc15, GPIOC, Gpioc, PC15_PIN, GpioPin, GPIOC_PERIPH, 15);
pin!(PD0, Pd0, GPIOD, Gpiod, PD0_PIN, GpioPin, GPIOD_PERIPH, 0);
pin!(PD1, Pd1, GPIOD, Gpiod, PD1_PIN, GpioPin, GPIOD_PERIPH, 1);
pin!(PD2, Pd2, GPIOD, Gpiod, PD2_PIN, GpioPin, GPIOD_PERIPH, 2);
   pin_source!(Pd2, super::usart::Uart5, super::sig::SigRx, 8);
pin!(PD3, Pd3, GPIOD, Gpiod, PD3_PIN, GpioPin, GPIOD_PERIPH, 3);
   pin_source!(Pd3, super::usart::Usart2, super::sig::SigCts, 7);
pin!(PD4, Pd4, GPIOD, Gpiod, PD4_PIN, GpioPin, GPIOD_PERIPH, 4);
   pin_source!(Pd4, super::usart::Usart2, super::sig::SigRts, 7);
pin!(PD5, Pd5, GPIOD, Gpiod, PD5_PIN, GpioPin, GPIOD_PERIPH, 5);
   pin_source!(Pd5, super::usart::Usart2, super::sig::SigTx, 7);
pin!(PD6, Pd6, GPIOD, Gpiod, PD6_PIN, GpioPin, GPIOD_PERIPH, 6);
   pin_source!(Pd6, super::usart::Usart2, super::sig::SigRx, 7);
pin!(PD7, Pd7, GPIOD, Gpiod, PD7_PIN, GpioPin, GPIOD_PERIPH, 7);
   pin_source!(Pd7, super::usart::Usart2, super::sig::SigCk, 7);
pin!(PD8, Pd8, GPIOD, Gpiod, PD8_PIN, GpioPin, GPIOD_PERIPH, 8);
   pin_source!(Pd8, super::usart::Usart3, super::sig::SigTx, 7);
pin!(PD9, Pd9, GPIOD, Gpiod, PD9_PIN, GpioPin, GPIOD_PERIPH, 9);
   pin_source!(Pd9, super::usart::Usart3, super::sig::SigRx, 7);
pin!(PD10, Pd10, GPIOD, Gpiod, PD10_PIN, GpioPin, GPIOD_PERIPH, 10);
   pin_source!(Pd10, super::usart::Usart3, super::sig::SigCk, 7);
pin!(PD11, Pd11, GPIOD, Gpiod, PD11_PIN, GpioPin, GPIOD_PERIPH, 11);
   pin_source!(Pd11, super::usart::Usart3, super::sig::SigCts, 7);
pin!(PD12, Pd12, GPIOD, Gpiod, PD12_PIN, GpioPin, GPIOD_PERIPH, 12);
   pin_source!(Pd12, super::tim_gen::Tim4Ch1, super::sig::SigTim, 2);
   pin_source!(Pd12, super::usart::Usart3, super::sig::SigRts, 7);
pin!(PD13, Pd13, GPIOD, Gpiod, PD13_PIN, GpioPin, GPIOD_PERIPH, 13);
   pin_source!(Pd13, super::tim_gen::Tim4Ch2, super::sig::SigTim, 2);
pin!(PD14, Pd14, GPIOD, Gpiod, PD14_PIN, GpioPin, GPIOD_PERIPH, 14);
   pin_source!(Pd14, super::tim_gen::Tim4Ch3, super::sig::SigTim, 2);
pin!(PD15, Pd15, GPIOD, Gpiod, PD15_PIN, GpioPin, GPIOD_PERIPH, 15);
   pin_source!(Pd15, super::tim_gen::Tim4Ch4, super::sig::SigTim, 2);
pin!(PE0, Pe0, GPIOE, Gpioe, PE0_PIN, GpioPin, GPIOE_PERIPH, 0);
pin!(PE1, Pe1, GPIOE, Gpioe, PE1_PIN, GpioPin, GPIOE_PERIPH, 1);
pin!(PE2, Pe2, GPIOE, Gpioe, PE2_PIN, GpioPin, GPIOE_PERIPH, 2);
pin!(PE3, Pe3, GPIOE, Gpioe, PE3_PIN, GpioPin, GPIOE_PERIPH, 3);
pin!(PE4, Pe4, GPIOE, Gpioe, PE4_PIN, GpioPin, GPIOE_PERIPH, 4);
pin!(PE5, Pe5, GPIOE, Gpioe, PE5_PIN, GpioPin, GPIOE_PERIPH, 5);
   pin_source!(Pe5, super::tim_gen::Tim9Ch1, super::sig::SigTim, 3);
pin!(PE6, Pe6, GPIOE, Gpioe, PE6_PIN, GpioPin, GPIOE_PERIPH, 6);
   pin_source!(Pe6, super::tim_gen::Tim9Ch2, super::sig::SigTim, 3);
pin!(PE7, Pe7, GPIOE, Gpioe, PE7_PIN, GpioPin, GPIOE_PERIPH, 7);
pin!(PE8, Pe8, GPIOE, Gpioe, PE8_PIN, GpioPin, GPIOE_PERIPH, 8);
pin!(PE9, Pe9, GPIOE, Gpioe, PE9_PIN, GpioPin, GPIOE_PERIPH, 9);
   pin_source!(Pe9, super::tim_adv::Tim1Ch1, super::sig::SigTim, 1);
pin!(PE10, Pe10, GPIOE, Gpioe, PE10_PIN, GpioPin, GPIOE_PERIPH, 10);
pin!(PE11, Pe11, GPIOE, Gpioe, PE11_PIN, GpioPin, GPIOE_PERIPH, 11);
   pin_source!(Pe11, super::tim_adv::Tim1Ch2, super::sig::SigTim, 1);
pin!(PE12, Pe12, GPIOE, Gpioe, PE12_PIN, GpioPin, GPIOE_PERIPH, 12);
pin!(PE13, Pe13, GPIOE, Gpioe, PE13_PIN, GpioPin, GPIOE_PERIPH, 13);
   pin_source!(Pe13, super::tim_adv::Tim1Ch3, super::sig::SigTim, 1);
pin!(PE14, Pe14, GPIOE, Gpioe, PE14_PIN, GpioPin, GPIOE_PERIPH, 14);
   pin_source!(Pe14, super::tim_adv::Tim1Ch4, super::sig::SigTim, 1);
pin!(PE15, Pe15, GPIOE, Gpioe, PE15_PIN, GpioPin, GPIOE_PERIPH, 15);
pin!(PF0, Pf0, GPIOF, Gpiof, PF0_PIN, GpioPin, GPIOF_PERIPH, 0);
pin!(PF1, Pf1, GPIOF, Gpiof, PF1_PIN, GpioPin, GPIOF_PERIPH, 1);
pin!(PF2, Pf2, GPIOF, Gpiof, PF2_PIN, GpioPin, GPIOF_PERIPH, 2);
pin!(PF3, Pf3, GPIOF, Gpiof, PF3_PIN, GpioPin, GPIOF_PERIPH, 3);
   pin_source!(Pf3, super::adc::Adc3Ch9, super::sig::SigAdc, 0);
pin!(PF4, Pf4, GPIOF, Gpiof, PF4_PIN, GpioPin, GPIOF_PERIPH, 4);
   pin_source!(Pf4, super::adc::Adc3Ch14, super::sig::SigAdc, 0);
pin!(PF5, Pf5, GPIOF, Gpiof, PF5_PIN, GpioPin, GPIOF_PERIPH, 5);
   pin_source!(Pf5, super::adc::Adc3Ch15, super::sig::SigAdc, 0);
pin!(PF6, Pf6, GPIOF, Gpiof, PF6_PIN, GpioPin, GPIOF_PERIPH, 6);
   pin_source!(Pf6, super::adc::Adc3Ch4, super::sig::SigAdc, 0);
   pin_source!(Pf6, super::tim_gen::Tim10Ch1, super::sig::SigTim, 3);
pin!(PF7, Pf7, GPIOF, Gpiof, PF7_PIN, GpioPin, GPIOF_PERIPH, 7);
   pin_source!(Pf7, super::adc::Adc3Ch5, super::sig::SigAdc, 0);
   pin_source!(Pf7, super::tim_gen::Tim11Ch1, super::sig::SigTim, 3);
pin!(PF8, Pf8, GPIOF, Gpiof, PF8_PIN, GpioPin, GPIOF_PERIPH, 8);
   pin_source!(Pf8, super::adc::Adc3Ch6, super::sig::SigAdc, 0);
   pin_source!(Pf8, super::tim_gen::Tim13Ch1, super::sig::SigTim, 9);
pin!(PF9, Pf9, GPIOF, Gpiof, PF9_PIN, GpioPin, GPIOF_PERIPH, 9);
   pin_source!(Pf9, super::adc::Adc3Ch7, super::sig::SigAdc, 0);
   pin_source!(Pf9, super::tim_gen::Tim14Ch1, super::sig::SigTim, 9);
pin!(PF10, Pf10, GPIOF, Gpiof, PF10_PIN, GpioPin, GPIOF_PERIPH, 10);
   pin_source!(Pf10, super::adc::Adc3Ch8, super::sig::SigAdc, 0);
pin!(PF11, Pf11, GPIOF, Gpiof, PF11_PIN, GpioPin, GPIOF_PERIPH, 11);
pin!(PF12, Pf12, GPIOF, Gpiof, PF12_PIN, GpioPin, GPIOF_PERIPH, 12);
pin!(PF13, Pf13, GPIOF, Gpiof, PF13_PIN, GpioPin, GPIOF_PERIPH, 13);
pin!(PF14, Pf14, GPIOF, Gpiof, PF14_PIN, GpioPin, GPIOF_PERIPH, 14);
pin!(PF15, Pf15, GPIOF, Gpiof, PF15_PIN, GpioPin, GPIOF_PERIPH, 15);
pin!(PG0, Pg0, GPIOG, Gpiog, PG0_PIN, GpioPin, GPIOG_PERIPH, 0);
pin!(PG1, Pg1, GPIOG, Gpiog, PG1_PIN, GpioPin, GPIOG_PERIPH, 1);
pin!(PG2, Pg2, GPIOG, Gpiog, PG2_PIN, GpioPin, GPIOG_PERIPH, 2);
pin!(PG3, Pg3, GPIOG, Gpiog, PG3_PIN, GpioPin, GPIOG_PERIPH, 3);
pin!(PG4, Pg4, GPIOG, Gpiog, PG4_PIN, GpioPin, GPIOG_PERIPH, 4);
pin!(PG5, Pg5, GPIOG, Gpiog, PG5_PIN, GpioPin, GPIOG_PERIPH, 5);
pin!(PG6, Pg6, GPIOG, Gpiog, PG6_PIN, GpioPin, GPIOG_PERIPH, 6);
pin!(PG7, Pg7, GPIOG, Gpiog, PG7_PIN, GpioPin, GPIOG_PERIPH, 7);
   pin_source!(Pg7, super::usart::Usart6, super::sig::SigCk, 8);
pin!(PG8, Pg8, GPIOG, Gpiog, PG8_PIN, GpioPin, GPIOG_PERIPH, 8);
   pin_source!(Pg8, super::usart::Usart6, super::sig::SigRts, 8);
pin!(PG9, Pg9, GPIOG, Gpiog, PG9_PIN, GpioPin, GPIOG_PERIPH, 9);
   pin_source!(Pg9, super::usart::Usart6, super::sig::SigRx, 8);
pin!(PG10, Pg10, GPIOG, Gpiog, PG10_PIN, GpioPin, GPIOG_PERIPH, 10);
pin!(PG11, Pg11, GPIOG, Gpiog, PG11_PIN, GpioPin, GPIOG_PERIPH, 11);
pin!(PG12, Pg12, GPIOG, Gpiog, PG12_PIN, GpioPin, GPIOG_PERIPH, 12);
   pin_source!(Pg12, super::usart::Usart6, super::sig::SigRts, 8);
pin!(PG13, Pg13, GPIOG, Gpiog, PG13_PIN, GpioPin, GPIOG_PERIPH, 13);
   pin_source!(Pg13, super::usart::Usart6, super::sig::SigCts, 8);
pin!(PG14, Pg14, GPIOG, Gpiog, PG14_PIN, GpioPin, GPIOG_PERIPH, 14);
   pin_source!(Pg14, super::usart::Usart6, super::sig::SigTx, 8);
pin!(PG15, Pg15, GPIOG, Gpiog, PG15_PIN, GpioPin, GPIOG_PERIPH, 15);
   pin_source!(Pg15, super::usart::Usart6, super::sig::SigCts, 8);
pin!(PH0, Ph0, GPIOH, Gpioh, PH0_PIN, GpioPin, GPIOH_PERIPH, 0);
pin!(PH1, Ph1, GPIOH, Gpioh, PH1_PIN, GpioPin, GPIOH_PERIPH, 1);
pin!(PH2, Ph2, GPIOH, Gpioh, PH2_PIN, GpioPin, GPIOH_PERIPH, 2);
pin!(PH3, Ph3, GPIOH, Gpioh, PH3_PIN, GpioPin, GPIOH_PERIPH, 3);
pin!(PH4, Ph4, GPIOH, Gpioh, PH4_PIN, GpioPin, GPIOH_PERIPH, 4);
pin!(PH5, Ph5, GPIOH, Gpioh, PH5_PIN, GpioPin, GPIOH_PERIPH, 5);
pin!(PH6, Ph6, GPIOH, Gpioh, PH6_PIN, GpioPin, GPIOH_PERIPH, 6);
   pin_source!(Ph6, super::tim_gen::Tim12Ch1, super::sig::SigTim, 9);
pin!(PH7, Ph7, GPIOH, Gpioh, PH7_PIN, GpioPin, GPIOH_PERIPH, 7);
pin!(PH8, Ph8, GPIOH, Gpioh, PH8_PIN, GpioPin, GPIOH_PERIPH, 8);
pin!(PH9, Ph9, GPIOH, Gpioh, PH9_PIN, GpioPin, GPIOH_PERIPH, 9);
   pin_source!(Ph9, super::tim_gen::Tim12Ch2, super::sig::SigTim, 9);
pin!(PH10, Ph10, GPIOH, Gpioh, PH10_PIN, GpioPin, GPIOH_PERIPH, 10);
   pin_source!(Ph10, super::tim_gen::Tim5Ch1, super::sig::SigTim, 2);
pin!(PH11, Ph11, GPIOH, Gpioh, PH11_PIN, GpioPin, GPIOH_PERIPH, 11);
   pin_source!(Ph11, super::tim_gen::Tim5Ch2, super::sig::SigTim, 2);
pin!(PH12, Ph12, GPIOH, Gpioh, PH12_PIN, GpioPin, GPIOH_PERIPH, 12);
   pin_source!(Ph12, super::tim_gen::Tim5Ch3, super::sig::SigTim, 2);
pin!(PH13, Ph13, GPIOH, Gpioh, PH13_PIN, GpioPin, GPIOH_PERIPH, 13);
pin!(PH14, Ph14, GPIOH, Gpioh, PH14_PIN, GpioPin, GPIOH_PERIPH, 14);
pin!(PH15, Ph15, GPIOH, Gpioh, PH15_PIN, GpioPin, GPIOH_PERIPH, 15);
pin!(PI0, Pi0, GPIOI, Gpioi, PI0_PIN, GpioPin, GPIOI_PERIPH, 0);
   pin_source!(Pi0, super::tim_gen::Tim5Ch4, super::sig::SigTim, 2);
pin!(PI1, Pi1, GPIOI, Gpioi, PI1_PIN, GpioPin, GPIOI_PERIPH, 1);
pin!(PI2, Pi2, GPIOI, Gpioi, PI2_PIN, GpioPin, GPIOI_PERIPH, 2);
   pin_source!(Pi2, super::tim_adv::Tim8Ch4, super::sig::SigTim, 3);
pin!(PI3, Pi3, GPIOI, Gpioi, PI3_PIN, GpioPin, GPIOI_PERIPH, 3);
pin!(PI4, Pi4, GPIOI, Gpioi, PI4_PIN, GpioPin, GPIOI_PERIPH, 4);
pin!(PI5, Pi5, GPIOI, Gpioi, PI5_PIN, GpioPin, GPIOI_PERIPH, 5);
   pin_source!(Pi5, super::tim_adv::Tim8Ch1, super::sig::SigTim, 3);
pin!(PI6, Pi6, GPIOI, Gpioi, PI6_PIN, GpioPin, GPIOI_PERIPH, 6);
   pin_source!(Pi6, super::tim_adv::Tim8Ch2, super::sig::SigTim, 3);
pin!(PI7, Pi7, GPIOI, Gpioi, PI7_PIN, GpioPin, GPIOI_PERIPH, 7);
   pin_source!(Pi7, super::tim_adv::Tim8Ch3, super::sig::SigTim, 3);
pin!(PI8, Pi8, GPIOI, Gpioi, PI8_PIN, GpioPin, GPIOI_PERIPH, 8);
pin!(PI9, Pi9, GPIOI, Gpioi, PI9_PIN, GpioPin, GPIOI_PERIPH, 9);
pin!(PI10, Pi10, GPIOI, Gpioi, PI10_PIN, GpioPin, GPIOI_PERIPH, 10);
