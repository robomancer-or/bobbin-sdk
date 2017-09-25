#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::gpio::*;

periph!( GPIOA, Gpioa, _GPIOA, GpioPeriph, 0x48000000);
periph!( GPIOB, Gpiob, _GPIOB, GpioPeriph, 0x48000400);
periph!( GPIOC, Gpioc, _GPIOC, GpioPeriph, 0x48000800);
periph!( GPIOD, Gpiod, _GPIOD, GpioPeriph, 0x48000c00);
periph!( GPIOE, Gpioe, _GPIOE, GpioPeriph, 0x48001000);
periph!( GPIOF, Gpiof, _GPIOF, GpioPeriph, 0x48001400);
periph!( GPIOG, Gpiog, _GPIOG, GpioPeriph, 0x48001800);
periph!( GPIOH, Gpioh, _GPIOH, GpioPeriph, 0x48001c00);










pin!(PA0, Pa0, GPIOA, Gpioa, _PA0, GpioPin, _GPIOA, 0);
    alt_fn!(Pa0, super::sig::Adc1In1, 0);
    alt_fn!(Pa0, super::sig::Tim2Ch1, 1);
    alt_fn!(Pa0, super::sig::Tim2Etr, 1);
    alt_fn!(Pa0, super::sig::TscG1Io1, 3);
    alt_fn!(Pa0, super::sig::Usart2Cts, 7);
    alt_fn!(Pa0, super::sig::Comp1Out, 8);
    alt_fn!(Pa0, super::sig::Tim8Bkin, 9);
    alt_fn!(Pa0, super::sig::Tim8Etr, 10);
    alt_fn!(Pa0, super::sig::EventOut, 15);

pin!(PA1, Pa1, GPIOA, Gpioa, _PA1, GpioPin, _GPIOA, 1);
    alt_fn!(Pa1, super::sig::Adc1In2, 0);
    alt_fn!(Pa1, super::sig::RtcRefin, 0);
    alt_fn!(Pa1, super::sig::Tim2Ch2, 1);
    alt_fn!(Pa1, super::sig::TscG1Io2, 3);
    alt_fn!(Pa1, super::sig::Usart2Rts, 7);
    alt_fn!(Pa1, super::sig::Tim15Ch1n, 9);
    alt_fn!(Pa1, super::sig::EventOut, 15);

pin!(PA2, Pa2, GPIOA, Gpioa, _PA2, GpioPin, _GPIOA, 2);
    alt_fn!(Pa2, super::sig::Adc1In3, 0);
    alt_fn!(Pa2, super::sig::Tim2Ch3, 1);
    alt_fn!(Pa2, super::sig::TscG1Io3, 3);
    alt_fn!(Pa2, super::sig::Usart2Tx, 7);
    alt_fn!(Pa2, super::sig::Comp2Out, 8);
    alt_fn!(Pa2, super::sig::Tim15Ch1, 9);
    alt_fn!(Pa2, super::sig::EventOut, 15);

pin!(PA3, Pa3, GPIOA, Gpioa, _PA3, GpioPin, _GPIOA, 3);
    alt_fn!(Pa3, super::sig::Adc1In4, 0);
    alt_fn!(Pa3, super::sig::Tim2Ch3, 1);
    alt_fn!(Pa3, super::sig::TscG1Io4, 3);
    alt_fn!(Pa3, super::sig::Usart2Rx, 7);
    alt_fn!(Pa3, super::sig::Tim15Ch2, 9);
    alt_fn!(Pa3, super::sig::EventOut, 15);

pin!(PA4, Pa4, GPIOA, Gpioa, _PA4, GpioPin, _GPIOA, 4);
    alt_fn!(Pa4, super::sig::Adc2In1, 0);
    alt_fn!(Pa4, super::sig::Tim3Ch2, 2);
    alt_fn!(Pa4, super::sig::TscG2Io1, 3);
    alt_fn!(Pa4, super::sig::Spi1Nss, 5);
    alt_fn!(Pa4, super::sig::Spi3Nss, 6);
    alt_fn!(Pa4, super::sig::I2s3Ws, 6);
    alt_fn!(Pa4, super::sig::Usart2Ck, 7);
    alt_fn!(Pa4, super::sig::EventOut, 15);

pin!(PA5, Pa5, GPIOA, Gpioa, _PA5, GpioPin, _GPIOA, 5);
    alt_fn!(Pa5, super::sig::Adc2In2, 0);
    alt_fn!(Pa5, super::sig::Tim2Ch1, 1);
    alt_fn!(Pa5, super::sig::Tim2Etr, 1);
    alt_fn!(Pa5, super::sig::TscG2Io2, 3);
    alt_fn!(Pa5, super::sig::Spi1Sck, 5);
    alt_fn!(Pa5, super::sig::EventOut, 15);

pin!(PA6, Pa6, GPIOA, Gpioa, _PA6, GpioPin, _GPIOA, 6);
    alt_fn!(Pa6, super::sig::Adc2In3, 0);
    alt_fn!(Pa6, super::sig::Tim16Ch1, 1);
    alt_fn!(Pa6, super::sig::Tim3Ch1, 2);
    alt_fn!(Pa6, super::sig::TscG2Io3, 3);
    alt_fn!(Pa6, super::sig::Tim8Bkin, 4);
    alt_fn!(Pa6, super::sig::Spi1Miso, 5);
    alt_fn!(Pa6, super::sig::Tim1Bkin, 6);
    alt_fn!(Pa6, super::sig::Comp1Out, 8);
    alt_fn!(Pa6, super::sig::EventOut, 15);

pin!(PA7, Pa7, GPIOA, Gpioa, _PA7, GpioPin, _GPIOA, 7);
    alt_fn!(Pa7, super::sig::Adc2In4, 0);
    alt_fn!(Pa7, super::sig::Tim17Ch1, 1);
    alt_fn!(Pa7, super::sig::Tim3Ch2, 2);
    alt_fn!(Pa7, super::sig::TscG2Io4, 3);
    alt_fn!(Pa7, super::sig::Tim8Ch1n, 4);
    alt_fn!(Pa7, super::sig::Spi1Mosi, 5);
    alt_fn!(Pa7, super::sig::Tim1Ch1n, 6);
    alt_fn!(Pa7, super::sig::EventOut, 15);

pin!(PA8, Pa8, GPIOA, Gpioa, _PA8, GpioPin, _GPIOA, 8);
    alt_fn!(Pa8, super::sig::Mco, 0);
    alt_fn!(Pa8, super::sig::I2c3Scl, 3);
    alt_fn!(Pa8, super::sig::I2c2Smbal, 4);
    alt_fn!(Pa8, super::sig::I2s2Mck, 5);
    alt_fn!(Pa8, super::sig::Tim1Ch1, 6);
    alt_fn!(Pa8, super::sig::Usart1Ck, 7);
    alt_fn!(Pa8, super::sig::Comp3Out, 8);
    alt_fn!(Pa8, super::sig::Tim4Etr, 10);
    alt_fn!(Pa8, super::sig::EventOut, 15);

pin!(PA9, Pa9, GPIOA, Gpioa, _PA9, GpioPin, _GPIOA, 9);
    alt_fn!(Pa9, super::sig::I2c3Smbal, 2);
    alt_fn!(Pa9, super::sig::TscG4Io1, 3);
    alt_fn!(Pa9, super::sig::I2c2Scl, 4);
    alt_fn!(Pa9, super::sig::I2s3Mck, 5);
    alt_fn!(Pa9, super::sig::Tim1Ch2, 6);
    alt_fn!(Pa9, super::sig::Usart1Tx, 7);
    alt_fn!(Pa9, super::sig::Comp5Out, 8);
    alt_fn!(Pa9, super::sig::Tim15Bkin, 9);
    alt_fn!(Pa9, super::sig::Tim2Ch3, 10);
    alt_fn!(Pa9, super::sig::EventOut, 15);

pin!(PA10, Pa10, GPIOA, Gpioa, _PA10, GpioPin, _GPIOA, 10);
    alt_fn!(Pa10, super::sig::Tim17Bkin, 1);
    alt_fn!(Pa10, super::sig::TscG4Io2, 3);
    alt_fn!(Pa10, super::sig::I2c2Sda, 4);
    alt_fn!(Pa10, super::sig::Spi2Miso, 5);
    alt_fn!(Pa10, super::sig::I2s2extSd, 5);
    alt_fn!(Pa10, super::sig::Tim1Ch3, 6);
    alt_fn!(Pa10, super::sig::Usart1Rx, 7);
    alt_fn!(Pa10, super::sig::Comp6Out, 8);
    alt_fn!(Pa10, super::sig::Tim2Ch4, 10);
    alt_fn!(Pa10, super::sig::Tim8Bkin, 11);
    alt_fn!(Pa10, super::sig::EventOut, 15);

pin!(PA11, Pa11, GPIOA, Gpioa, _PA11, GpioPin, _GPIOA, 11);
    alt_fn!(Pa11, super::sig::Spi2Mosi, 5);
    alt_fn!(Pa11, super::sig::I2s2Sd, 5);
    alt_fn!(Pa11, super::sig::Tim1Ch1n, 6);
    alt_fn!(Pa11, super::sig::Usart1Cts, 7);
    alt_fn!(Pa11, super::sig::Comp1Out, 8);
    alt_fn!(Pa11, super::sig::CanRx, 9);
    alt_fn!(Pa11, super::sig::Tim4Ch1, 10);
    alt_fn!(Pa11, super::sig::Tim1Ch4, 11);
    alt_fn!(Pa11, super::sig::Tim1Bkin2, 12);
    alt_fn!(Pa11, super::sig::EventOut, 15);

pin!(PA12, Pa12, GPIOA, Gpioa, _PA12, GpioPin, _GPIOA, 12);
    alt_fn!(Pa12, super::sig::Tim16Ch1, 1);
    alt_fn!(Pa12, super::sig::I2sckin, 5);
    alt_fn!(Pa12, super::sig::Tim1Ch2n, 6);
    alt_fn!(Pa12, super::sig::Usart1Rts, 7);
    alt_fn!(Pa12, super::sig::Comp2Out, 8);
    alt_fn!(Pa12, super::sig::CanTx, 9);
    alt_fn!(Pa12, super::sig::Tim4Ch2, 10);
    alt_fn!(Pa12, super::sig::Tim1Etr, 11);
    alt_fn!(Pa12, super::sig::EventOut, 15);

pin!(PA13, Pa13, GPIOA, Gpioa, _PA13, GpioPin, _GPIOA, 13);
    alt_fn!(Pa13, super::sig::Swdio, 0);
    alt_fn!(Pa13, super::sig::Jtms, 0);
    alt_fn!(Pa13, super::sig::Tim16Ch1n, 1);
    alt_fn!(Pa13, super::sig::TscG4Io3, 3);
    alt_fn!(Pa13, super::sig::IrOut, 5);
    alt_fn!(Pa13, super::sig::Usart3Cts, 7);
    alt_fn!(Pa13, super::sig::Tim4Ch3, 10);
    alt_fn!(Pa13, super::sig::EventOut, 15);

pin!(PA14, Pa14, GPIOA, Gpioa, _PA14, GpioPin, _GPIOA, 14);
    alt_fn!(Pa14, super::sig::Swclk, 0);
    alt_fn!(Pa14, super::sig::Jtck, 0);
    alt_fn!(Pa14, super::sig::TscG4Io4, 3);
    alt_fn!(Pa14, super::sig::I2c1Sda, 4);
    alt_fn!(Pa14, super::sig::Tim8Ch2, 5);
    alt_fn!(Pa14, super::sig::Tim1Bkin, 6);
    alt_fn!(Pa14, super::sig::Usart2Tx, 7);
    alt_fn!(Pa14, super::sig::EventOut, 15);

pin!(PA15, Pa15, GPIOA, Gpioa, _PA15, GpioPin, _GPIOA, 15);
    alt_fn!(Pa15, super::sig::Jtdi, 0);
    alt_fn!(Pa15, super::sig::Tim2Ch1, 1);
    alt_fn!(Pa15, super::sig::Tim2Etr, 1);
    alt_fn!(Pa15, super::sig::Tim8Ch1, 2);
    alt_fn!(Pa15, super::sig::TscSync, 3);
    alt_fn!(Pa15, super::sig::I2c1Scl, 4);
    alt_fn!(Pa15, super::sig::Spi1Nss, 5);
    alt_fn!(Pa15, super::sig::Spi3Nss, 6);
    alt_fn!(Pa15, super::sig::I2s3Ws, 6);
    alt_fn!(Pa15, super::sig::Usart2Rx, 7);
    alt_fn!(Pa15, super::sig::Tim1Bkin, 9);
    alt_fn!(Pa15, super::sig::EventOut, 15);

pin!(PB0, Pb0, GPIOB, Gpiob, _PB0, GpioPin, _GPIOB, 0);
    alt_fn!(Pb0, super::sig::Adc2In12, 0);
    alt_fn!(Pb0, super::sig::Tim3Ch3, 2);
    alt_fn!(Pb0, super::sig::TscG3Io2, 3);
    alt_fn!(Pb0, super::sig::Tim8Ch2n, 4);
    alt_fn!(Pb0, super::sig::Tim1Ch2n, 6);
    alt_fn!(Pb0, super::sig::EventOut, 15);

pin!(PB1, Pb1, GPIOB, Gpiob, _PB1, GpioPin, _GPIOB, 1);
    alt_fn!(Pb1, super::sig::Adc3In1, 0);
    alt_fn!(Pb1, super::sig::Tim3Ch4, 2);
    alt_fn!(Pb1, super::sig::TscG3Io3, 3);
    alt_fn!(Pb1, super::sig::Tim8Ch3n, 4);
    alt_fn!(Pb1, super::sig::Tim1Ch3n, 6);
    alt_fn!(Pb1, super::sig::Comp4Out, 8);
    alt_fn!(Pb1, super::sig::EventOut, 15);

pin!(PB2, Pb2, GPIOB, Gpiob, _PB2, GpioPin, _GPIOB, 2);
    alt_fn!(Pb2, super::sig::Adc2In12, 0);
    alt_fn!(Pb2, super::sig::TscG3Io4, 3);
    alt_fn!(Pb2, super::sig::EventOut, 15);

pin!(PB3, Pb3, GPIOB, Gpiob, _PB3, GpioPin, _GPIOB, 3);
    alt_fn!(Pb3, super::sig::Jtdo, 0);
    alt_fn!(Pb3, super::sig::Traceswo, 0);
    alt_fn!(Pb3, super::sig::Tim2Ch2, 1);
    alt_fn!(Pb3, super::sig::Tim4Etr, 2);
    alt_fn!(Pb3, super::sig::TscG5Io1, 3);
    alt_fn!(Pb3, super::sig::Tim8Ch1n, 4);
    alt_fn!(Pb3, super::sig::Spi1Sck, 5);
    alt_fn!(Pb3, super::sig::Spi3Sck, 6);
    alt_fn!(Pb3, super::sig::I2s3Ck, 6);
    alt_fn!(Pb3, super::sig::Usart2Tx, 7);
    alt_fn!(Pb3, super::sig::Tim3Etr, 10);
    alt_fn!(Pb3, super::sig::EventOut, 15);

pin!(PB4, Pb4, GPIOB, Gpiob, _PB4, GpioPin, _GPIOB, 4);
    alt_fn!(Pb4, super::sig::Jtrst, 0);
    alt_fn!(Pb4, super::sig::Tim16Ch1, 1);
    alt_fn!(Pb4, super::sig::Tim3Ch1, 2);
    alt_fn!(Pb4, super::sig::TscG5Io1, 3);
    alt_fn!(Pb4, super::sig::Tim8Ch2n, 4);
    alt_fn!(Pb4, super::sig::Spi1Miso, 5);
    alt_fn!(Pb4, super::sig::Spi3Miso, 6);
    alt_fn!(Pb4, super::sig::I2s3extSd, 6);
    alt_fn!(Pb4, super::sig::Usart2Rx, 7);
    alt_fn!(Pb4, super::sig::Tim17Bkin, 10);
    alt_fn!(Pb4, super::sig::EventOut, 15);

pin!(PB5, Pb5, GPIOB, Gpiob, _PB5, GpioPin, _GPIOB, 5);
    alt_fn!(Pb5, super::sig::Tim16Bkin, 1);
    alt_fn!(Pb5, super::sig::Tim3Ch2, 2);
    alt_fn!(Pb5, super::sig::Tim8Ch3n, 3);
    alt_fn!(Pb5, super::sig::I2c1Smbal, 4);
    alt_fn!(Pb5, super::sig::Spi1Mosi, 5);
    alt_fn!(Pb5, super::sig::Spi3Mosi, 6);
    alt_fn!(Pb5, super::sig::I2s3Sd, 6);
    alt_fn!(Pb5, super::sig::Usart2Ck, 7);
    alt_fn!(Pb5, super::sig::I2c3Sda, 8);
    alt_fn!(Pb5, super::sig::Tim17Ch1, 10);
    alt_fn!(Pb5, super::sig::EventOut, 15);

pin!(PB6, Pb6, GPIOB, Gpiob, _PB6, GpioPin, _GPIOB, 6);
    alt_fn!(Pb6, super::sig::Tim16Ch1n, 1);
    alt_fn!(Pb6, super::sig::Tim4Ch1, 2);
    alt_fn!(Pb6, super::sig::TscG5Io3, 3);
    alt_fn!(Pb6, super::sig::I2c1Scl, 4);
    alt_fn!(Pb6, super::sig::Tim8Ch1, 5);
    alt_fn!(Pb6, super::sig::Tim8Etr, 6);
    alt_fn!(Pb6, super::sig::Usart1Tx, 7);
    alt_fn!(Pb6, super::sig::Tim8Bkin2, 10);
    alt_fn!(Pb6, super::sig::EventOut, 15);

pin!(PB7, Pb7, GPIOB, Gpiob, _PB7, GpioPin, _GPIOB, 7);
    alt_fn!(Pb7, super::sig::Tim17Ch1n, 1);
    alt_fn!(Pb7, super::sig::Tim4Ch2, 2);
    alt_fn!(Pb7, super::sig::TscG5Io4, 3);
    alt_fn!(Pb7, super::sig::I2c1Sda, 4);
    alt_fn!(Pb7, super::sig::Tim8Bkin, 5);
    alt_fn!(Pb7, super::sig::Usart1Rx, 7);
    alt_fn!(Pb7, super::sig::Tim3Ch4, 10);
    alt_fn!(Pb7, super::sig::FmcNadv, 12);
    alt_fn!(Pb7, super::sig::EventOut, 15);

pin!(PB8, Pb8, GPIOB, Gpiob, _PB8, GpioPin, _GPIOB, 8);
    alt_fn!(Pb8, super::sig::Tim16Ch1, 1);
    alt_fn!(Pb8, super::sig::Tim4Ch3, 2);
    alt_fn!(Pb8, super::sig::TscSync, 3);
    alt_fn!(Pb8, super::sig::I2c1Scl, 4);
    alt_fn!(Pb8, super::sig::Usart3Rx, 7);
    alt_fn!(Pb8, super::sig::Comp1Out, 8);
    alt_fn!(Pb8, super::sig::CanRx, 9);
    alt_fn!(Pb8, super::sig::Tim8Ch2, 10);
    alt_fn!(Pb8, super::sig::Tim1Bkin, 12);
    alt_fn!(Pb8, super::sig::EventOut, 15);

pin!(PB9, Pb9, GPIOB, Gpiob, _PB9, GpioPin, _GPIOB, 9);
    alt_fn!(Pb9, super::sig::Tim17Ch1, 1);
    alt_fn!(Pb9, super::sig::Tim4Ch4, 2);
    alt_fn!(Pb9, super::sig::I2c1Sda, 4);
    alt_fn!(Pb9, super::sig::IrOut, 6);
    alt_fn!(Pb9, super::sig::Usart3Tx, 7);
    alt_fn!(Pb9, super::sig::Comp2Out, 8);
    alt_fn!(Pb9, super::sig::CanTx, 9);
    alt_fn!(Pb9, super::sig::Tim8Ch3, 10);
    alt_fn!(Pb9, super::sig::EventOut, 15);

pin!(PB10, Pb10, GPIOB, Gpiob, _PB10, GpioPin, _GPIOB, 10);
    alt_fn!(Pb10, super::sig::Tim2Ch3, 1);
    alt_fn!(Pb10, super::sig::TscSync, 3);
    alt_fn!(Pb10, super::sig::Usart3Tx, 7);
    alt_fn!(Pb10, super::sig::EventOut, 15);

pin!(PB11, Pb11, GPIOB, Gpiob, _PB11, GpioPin, _GPIOB, 11);
    alt_fn!(Pb11, super::sig::Adc4In14, 0);
    alt_fn!(Pb11, super::sig::Tim2Ch4, 1);
    alt_fn!(Pb11, super::sig::TscG6Io1, 3);
    alt_fn!(Pb11, super::sig::Usart3Rx, 7);
    alt_fn!(Pb11, super::sig::EventOut, 15);

pin!(PB12, Pb12, GPIOB, Gpiob, _PB12, GpioPin, _GPIOB, 12);
    alt_fn!(Pb12, super::sig::Adc4In3, 0);
    alt_fn!(Pb12, super::sig::TscG6Io2, 3);
    alt_fn!(Pb12, super::sig::I2c1Smbal, 4);
    alt_fn!(Pb12, super::sig::Spi2Nss, 5);
    alt_fn!(Pb12, super::sig::I2s2Ws, 5);
    alt_fn!(Pb12, super::sig::Tim1Bkin, 6);
    alt_fn!(Pb12, super::sig::Usart3Ck, 7);
    alt_fn!(Pb12, super::sig::EventOut, 15);

pin!(PB13, Pb13, GPIOB, Gpiob, _PB13, GpioPin, _GPIOB, 13);
    alt_fn!(Pb13, super::sig::Adc4In5, 0);
    alt_fn!(Pb13, super::sig::TscG6Io3, 3);
    alt_fn!(Pb13, super::sig::Spi2Sck, 5);
    alt_fn!(Pb13, super::sig::I2s2Ck, 5);
    alt_fn!(Pb13, super::sig::Tim1Ch1n, 6);
    alt_fn!(Pb13, super::sig::Usart3Cts, 7);
    alt_fn!(Pb13, super::sig::EventOut, 15);

pin!(PB14, Pb14, GPIOB, Gpiob, _PB14, GpioPin, _GPIOB, 14);
    alt_fn!(Pb14, super::sig::Adc4In4, 0);
    alt_fn!(Pb14, super::sig::Tim15Ch1, 1);
    alt_fn!(Pb14, super::sig::TscG6Io4, 3);
    alt_fn!(Pb14, super::sig::Spi2Miso, 5);
    alt_fn!(Pb14, super::sig::I2s2extSd, 5);
    alt_fn!(Pb14, super::sig::Tim1Ch2n, 6);
    alt_fn!(Pb14, super::sig::Usart3Rts, 7);
    alt_fn!(Pb14, super::sig::EventOut, 15);

pin!(PB15, Pb15, GPIOB, Gpiob, _PB15, GpioPin, _GPIOB, 15);
    alt_fn!(Pb15, super::sig::Adc4In5, 0);
    alt_fn!(Pb15, super::sig::RtcRefin, 0);
    alt_fn!(Pb15, super::sig::Tim15Ch2, 1);
    alt_fn!(Pb15, super::sig::Tim15Ch1n, 2);
    alt_fn!(Pb15, super::sig::Tim1Ch3n, 3);
    alt_fn!(Pb15, super::sig::Spi2Mosi, 5);
    alt_fn!(Pb15, super::sig::I2s2Sd, 5);
    alt_fn!(Pb15, super::sig::EventOut, 15);

pin!(PC0, Pc0, GPIOC, Gpioc, _PC0, GpioPin, _GPIOC, 0);
    alt_fn!(Pc0, super::sig::Adc1In6, 0);
    alt_fn!(Pc0, super::sig::Adc2In6, 0);
    alt_fn!(Pc0, super::sig::EventOut, 1);
    alt_fn!(Pc0, super::sig::Tim1Ch1, 2);

pin!(PC1, Pc1, GPIOC, Gpioc, _PC1, GpioPin, _GPIOC, 1);
    alt_fn!(Pc1, super::sig::Adc1In7, 0);
    alt_fn!(Pc1, super::sig::Adc2In7, 0);
    alt_fn!(Pc1, super::sig::EventOut, 1);
    alt_fn!(Pc1, super::sig::Tim1Ch2, 2);

pin!(PC2, Pc2, GPIOC, Gpioc, _PC2, GpioPin, _GPIOC, 2);
    alt_fn!(Pc2, super::sig::Adc1In8, 0);
    alt_fn!(Pc2, super::sig::Adc2In8, 0);
    alt_fn!(Pc2, super::sig::EventOut, 1);
    alt_fn!(Pc2, super::sig::Tim1Ch3, 2);
    alt_fn!(Pc2, super::sig::Comp7Out, 3);

pin!(PC3, Pc3, GPIOC, Gpioc, _PC3, GpioPin, _GPIOC, 3);
    alt_fn!(Pc3, super::sig::Adc1In9, 0);
    alt_fn!(Pc3, super::sig::Adc2In9, 0);
    alt_fn!(Pc3, super::sig::EventOut, 1);
    alt_fn!(Pc3, super::sig::Tim1Ch4, 2);
    alt_fn!(Pc3, super::sig::Tim1Bkin2, 6);

pin!(PC4, Pc4, GPIOC, Gpioc, _PC4, GpioPin, _GPIOC, 4);
    alt_fn!(Pc4, super::sig::Adc2In5, 0);
    alt_fn!(Pc4, super::sig::EventOut, 1);
    alt_fn!(Pc4, super::sig::Tim1Chetr, 2);
    alt_fn!(Pc4, super::sig::Usart1Tx, 7);

pin!(PC5, Pc5, GPIOC, Gpioc, _PC5, GpioPin, _GPIOC, 5);
    alt_fn!(Pc5, super::sig::Adc2In11, 0);
    alt_fn!(Pc5, super::sig::EventOut, 1);
    alt_fn!(Pc5, super::sig::Tim15Bkin, 2);
    alt_fn!(Pc5, super::sig::TscG3Io1, 3);
    alt_fn!(Pc5, super::sig::Usart1Rx, 7);

pin!(PC6, Pc6, GPIOC, Gpioc, _PC6, GpioPin, _GPIOC, 6);
    alt_fn!(Pc6, super::sig::EventOut, 1);
    alt_fn!(Pc6, super::sig::Tim3Ch1, 2);
    alt_fn!(Pc6, super::sig::Tim8Ch1, 3);
    alt_fn!(Pc6, super::sig::I2s2Mck, 6);
    alt_fn!(Pc6, super::sig::Comp6Out, 7);

pin!(PC7, Pc7, GPIOC, Gpioc, _PC7, GpioPin, _GPIOC, 7);
    alt_fn!(Pc7, super::sig::EventOut, 1);
    alt_fn!(Pc7, super::sig::Tim3Ch2, 2);
    alt_fn!(Pc7, super::sig::Tim8Ch2, 3);
    alt_fn!(Pc7, super::sig::I2s3Mck, 6);
    alt_fn!(Pc7, super::sig::Comp5Out, 7);

pin!(PC8, Pc8, GPIOC, Gpioc, _PC8, GpioPin, _GPIOC, 8);
    alt_fn!(Pc8, super::sig::EventOut, 1);
    alt_fn!(Pc8, super::sig::Tim3Ch3, 2);
    alt_fn!(Pc8, super::sig::Tim8Ch3, 3);
    alt_fn!(Pc8, super::sig::Comp3Out, 7);

pin!(PC9, Pc9, GPIOC, Gpioc, _PC9, GpioPin, _GPIOC, 9);
    alt_fn!(Pc9, super::sig::EventOut, 1);
    alt_fn!(Pc9, super::sig::Tim3Ch4, 2);
    alt_fn!(Pc9, super::sig::I2c3Sda, 3);
    alt_fn!(Pc9, super::sig::Tim8Ch4, 3);
    alt_fn!(Pc9, super::sig::I2sckin, 5);
    alt_fn!(Pc9, super::sig::Tim8Bkin2, 6);

pin!(PC10, Pc10, GPIOC, Gpioc, _PC10, GpioPin, _GPIOC, 10);
    alt_fn!(Pc10, super::sig::EventOut, 1);
    alt_fn!(Pc10, super::sig::Tim8Ch1n, 3);
    alt_fn!(Pc10, super::sig::Uart4Tx, 5);
    alt_fn!(Pc10, super::sig::Spi3Sck, 6);
    alt_fn!(Pc10, super::sig::I2s3Ck, 6);
    alt_fn!(Pc10, super::sig::Usart3Tx, 7);

pin!(PC11, Pc11, GPIOC, Gpioc, _PC11, GpioPin, _GPIOC, 11);
    alt_fn!(Pc11, super::sig::EventOut, 1);
    alt_fn!(Pc11, super::sig::Tim8Ch2n, 3);
    alt_fn!(Pc11, super::sig::Uart4Rx, 5);
    alt_fn!(Pc11, super::sig::Spi3Miso, 6);
    alt_fn!(Pc11, super::sig::I2s3extSd, 6);
    alt_fn!(Pc11, super::sig::Usart3Rx, 7);

pin!(PC12, Pc12, GPIOC, Gpioc, _PC12, GpioPin, _GPIOC, 12);
    alt_fn!(Pc12, super::sig::EventOut, 1);
    alt_fn!(Pc12, super::sig::Tim8Ch3n, 3);
    alt_fn!(Pc12, super::sig::Uart5Tx, 5);
    alt_fn!(Pc12, super::sig::Spi3Mosi, 6);
    alt_fn!(Pc12, super::sig::I2s3Sd, 6);
    alt_fn!(Pc12, super::sig::Usart3Ck, 7);

pin!(PC13, Pc13, GPIOC, Gpioc, _PC13, GpioPin, _GPIOC, 13);
    alt_fn!(Pc13, super::sig::EventOut, 1);
    alt_fn!(Pc13, super::sig::Tim8Ch1n, 3);

pin!(PC14, Pc14, GPIOC, Gpioc, _PC14, GpioPin, _GPIOC, 14);
    alt_fn!(Pc14, super::sig::EventOut, 1);

pin!(PC15, Pc15, GPIOC, Gpioc, _PC15, GpioPin, _GPIOC, 15);
    alt_fn!(Pc15, super::sig::EventOut, 1);

pin!(PD0, Pd0, GPIOD, Gpiod, _PD0, GpioPin, _GPIOD, 0);
    alt_fn!(Pd0, super::sig::EventOut, 1);
    alt_fn!(Pd0, super::sig::CanRx, 7);
    alt_fn!(Pd0, super::sig::FmcD2, 12);

pin!(PD1, Pd1, GPIOD, Gpiod, _PD1, GpioPin, _GPIOD, 1);
    alt_fn!(Pd1, super::sig::EventOut, 1);
    alt_fn!(Pd1, super::sig::Tim8Ch4, 3);
    alt_fn!(Pd1, super::sig::Tim8Bkin2, 6);
    alt_fn!(Pd1, super::sig::CanTx, 7);
    alt_fn!(Pd1, super::sig::FmcD3, 12);

pin!(PD2, Pd2, GPIOD, Gpiod, _PD2, GpioPin, _GPIOD, 2);
    alt_fn!(Pd2, super::sig::EventOut, 1);
    alt_fn!(Pd2, super::sig::Tim3Etr, 2);
    alt_fn!(Pd2, super::sig::Tim8Bkin, 3);
    alt_fn!(Pd2, super::sig::Uart5Rx, 5);

pin!(PD3, Pd3, GPIOD, Gpiod, _PD3, GpioPin, _GPIOD, 3);
    alt_fn!(Pd3, super::sig::EventOut, 1);
    alt_fn!(Pd3, super::sig::Tim2Ch1, 2);
    alt_fn!(Pd3, super::sig::Tim2Etr, 2);
    alt_fn!(Pd3, super::sig::Usart2Cts, 7);
    alt_fn!(Pd3, super::sig::FmcClk, 12);

pin!(PD4, Pd4, GPIOD, Gpiod, _PD4, GpioPin, _GPIOD, 4);
    alt_fn!(Pd4, super::sig::EventOut, 1);
    alt_fn!(Pd4, super::sig::Tim2Ch2, 2);
    alt_fn!(Pd4, super::sig::Usart2Rts, 7);
    alt_fn!(Pd4, super::sig::FmcNoe, 12);

pin!(PD5, Pd5, GPIOD, Gpiod, _PD5, GpioPin, _GPIOD, 5);
    alt_fn!(Pd5, super::sig::EventOut, 1);
    alt_fn!(Pd5, super::sig::Usart2Tx, 7);
    alt_fn!(Pd5, super::sig::FmcNwe, 12);

pin!(PD6, Pd6, GPIOD, Gpiod, _PD6, GpioPin, _GPIOD, 6);
    alt_fn!(Pd6, super::sig::EventOut, 1);
    alt_fn!(Pd6, super::sig::Tim2Ch4, 2);
    alt_fn!(Pd6, super::sig::Usart2Rx, 7);
    alt_fn!(Pd6, super::sig::FmcNwait, 12);

pin!(PD7, Pd7, GPIOD, Gpiod, _PD7, GpioPin, _GPIOD, 7);
    alt_fn!(Pd7, super::sig::EventOut, 1);
    alt_fn!(Pd7, super::sig::Tim2Ch3, 2);
    alt_fn!(Pd7, super::sig::Usart2Ck, 7);
    alt_fn!(Pd7, super::sig::FmcNe1, 12);
    alt_fn!(Pd7, super::sig::FmcNce2, 12);

pin!(PD8, Pd8, GPIOD, Gpiod, _PD8, GpioPin, _GPIOD, 8);
    alt_fn!(Pd8, super::sig::Adc4In12, 0);
    alt_fn!(Pd8, super::sig::EventOut, 1);
    alt_fn!(Pd8, super::sig::Usart3Tx, 7);
    alt_fn!(Pd8, super::sig::FmcD13, 12);

pin!(PD9, Pd9, GPIOD, Gpiod, _PD9, GpioPin, _GPIOD, 9);
    alt_fn!(Pd9, super::sig::Adc4In13, 0);
    alt_fn!(Pd9, super::sig::EventOut, 1);
    alt_fn!(Pd9, super::sig::Usart3Rx, 7);
    alt_fn!(Pd9, super::sig::FmcD14, 12);

pin!(PD10, Pd10, GPIOD, Gpiod, _PD10, GpioPin, _GPIOD, 10);
    alt_fn!(Pd10, super::sig::Adc3In7, 0);
    alt_fn!(Pd10, super::sig::Adc4In7, 0);
    alt_fn!(Pd10, super::sig::EventOut, 1);
    alt_fn!(Pd10, super::sig::Usart3Ck, 7);
    alt_fn!(Pd10, super::sig::FmcD15, 12);

pin!(PD11, Pd11, GPIOD, Gpiod, _PD11, GpioPin, _GPIOD, 11);
    alt_fn!(Pd11, super::sig::Adc3In8, 0);
    alt_fn!(Pd11, super::sig::Adc4In8, 0);
    alt_fn!(Pd11, super::sig::EventOut, 1);
    alt_fn!(Pd11, super::sig::Usart3Cts, 7);
    alt_fn!(Pd11, super::sig::FmcA16, 12);

pin!(PD12, Pd12, GPIOD, Gpiod, _PD12, GpioPin, _GPIOD, 12);
    alt_fn!(Pd12, super::sig::Adc3In9, 0);
    alt_fn!(Pd12, super::sig::Adc4In9, 0);
    alt_fn!(Pd12, super::sig::EventOut, 1);
    alt_fn!(Pd12, super::sig::Tim4Ch1, 2);
    alt_fn!(Pd12, super::sig::TscG8Io1, 3);
    alt_fn!(Pd12, super::sig::Usart3Rts, 7);
    alt_fn!(Pd12, super::sig::FmcA17, 12);

pin!(PD13, Pd13, GPIOD, Gpiod, _PD13, GpioPin, _GPIOD, 13);
    alt_fn!(Pd13, super::sig::Adc3In10, 0);
    alt_fn!(Pd13, super::sig::Adc4In10, 0);
    alt_fn!(Pd13, super::sig::EventOut, 1);
    alt_fn!(Pd13, super::sig::Tim4Ch2, 2);
    alt_fn!(Pd13, super::sig::TscG8Io2, 3);
    alt_fn!(Pd13, super::sig::FmcA18, 12);

pin!(PD14, Pd14, GPIOD, Gpiod, _PD14, GpioPin, _GPIOD, 14);
    alt_fn!(Pd14, super::sig::Adc3In11, 0);
    alt_fn!(Pd14, super::sig::Adc4In11, 0);
    alt_fn!(Pd14, super::sig::EventOut, 1);
    alt_fn!(Pd14, super::sig::Tim4Ch3, 2);
    alt_fn!(Pd14, super::sig::TscG8Io3, 3);
    alt_fn!(Pd14, super::sig::FmcD0, 12);

pin!(PD15, Pd15, GPIOD, Gpiod, _PD15, GpioPin, _GPIOD, 15);
    alt_fn!(Pd15, super::sig::EventOut, 1);
    alt_fn!(Pd15, super::sig::Tim4Ch4, 2);
    alt_fn!(Pd15, super::sig::TscG8Io4, 3);
    alt_fn!(Pd15, super::sig::Spi2Nss, 6);
    alt_fn!(Pd15, super::sig::FmcD1, 12);

pin!(PE0, Pe0, GPIOE, Gpioe, _PE0, GpioPin, _GPIOE, 0);
    alt_fn!(Pe0, super::sig::EventOut, 1);
    alt_fn!(Pe0, super::sig::Tim4Etr, 2);
    alt_fn!(Pe0, super::sig::Tim16Ch1, 4);
    alt_fn!(Pe0, super::sig::Tim20Etr, 6);
    alt_fn!(Pe0, super::sig::Usart1Tx, 7);
    alt_fn!(Pe0, super::sig::FmcNbl0, 12);

pin!(PE1, Pe1, GPIOE, Gpioe, _PE1, GpioPin, _GPIOE, 1);
    alt_fn!(Pe1, super::sig::EventOut, 1);
    alt_fn!(Pe1, super::sig::Tim17Ch1, 4);
    alt_fn!(Pe1, super::sig::Tim20Ch4, 6);
    alt_fn!(Pe1, super::sig::Usart1Rx, 7);
    alt_fn!(Pe1, super::sig::FmcNbl1, 12);

pin!(PE2, Pe2, GPIOE, Gpioe, _PE2, GpioPin, _GPIOE, 2);
    alt_fn!(Pe2, super::sig::Traceck, 0);
    alt_fn!(Pe2, super::sig::EventOut, 1);
    alt_fn!(Pe2, super::sig::Tim3Ch1, 2);
    alt_fn!(Pe2, super::sig::TscG7Io1, 3);
    alt_fn!(Pe2, super::sig::Spi4Sck, 5);
    alt_fn!(Pe2, super::sig::Tim20Ch1, 6);
    alt_fn!(Pe2, super::sig::FmcA23, 12);

pin!(PE3, Pe3, GPIOE, Gpioe, _PE3, GpioPin, _GPIOE, 3);
    alt_fn!(Pe3, super::sig::Traced0, 0);
    alt_fn!(Pe3, super::sig::EventOut, 1);
    alt_fn!(Pe3, super::sig::Tim3Ch2, 2);
    alt_fn!(Pe3, super::sig::TscG7Io2, 3);
    alt_fn!(Pe3, super::sig::Spi4Nss, 5);
    alt_fn!(Pe3, super::sig::Tim20Ch2, 6);
    alt_fn!(Pe3, super::sig::FmcA19, 12);

pin!(PE4, Pe4, GPIOE, Gpioe, _PE4, GpioPin, _GPIOE, 4);
    alt_fn!(Pe4, super::sig::Traced1, 0);
    alt_fn!(Pe4, super::sig::EventOut, 1);
    alt_fn!(Pe4, super::sig::Tim3Ch3, 2);
    alt_fn!(Pe4, super::sig::TscG7Io3, 3);
    alt_fn!(Pe4, super::sig::Spi4Nss, 5);
    alt_fn!(Pe4, super::sig::Tim20Ch1n, 6);
    alt_fn!(Pe4, super::sig::FmcA20, 12);

pin!(PE5, Pe5, GPIOE, Gpioe, _PE5, GpioPin, _GPIOE, 5);
    alt_fn!(Pe5, super::sig::Traced2, 0);
    alt_fn!(Pe5, super::sig::EventOut, 1);
    alt_fn!(Pe5, super::sig::Tim3Ch4, 2);
    alt_fn!(Pe5, super::sig::TscG7Io4, 3);
    alt_fn!(Pe5, super::sig::Spi4Miso, 5);
    alt_fn!(Pe5, super::sig::Tim20Ch2n, 6);
    alt_fn!(Pe5, super::sig::FmcA21, 12);

pin!(PE6, Pe6, GPIOE, Gpioe, _PE6, GpioPin, _GPIOE, 6);
    alt_fn!(Pe6, super::sig::Traced3, 0);
    alt_fn!(Pe6, super::sig::EventOut, 1);
    alt_fn!(Pe6, super::sig::Spi4Mosi, 5);
    alt_fn!(Pe6, super::sig::Tim20Ch3n, 6);
    alt_fn!(Pe6, super::sig::FmcA22, 12);

pin!(PE7, Pe7, GPIOE, Gpioe, _PE7, GpioPin, _GPIOE, 7);
    alt_fn!(Pe7, super::sig::Adc3In13, 0);
    alt_fn!(Pe7, super::sig::EventOut, 1);
    alt_fn!(Pe7, super::sig::Tim1Etr, 2);
    alt_fn!(Pe7, super::sig::FmcD4, 12);

pin!(PE8, Pe8, GPIOE, Gpioe, _PE8, GpioPin, _GPIOE, 8);
    alt_fn!(Pe8, super::sig::Adc3In6, 0);
    alt_fn!(Pe8, super::sig::Adc4In6, 0);
    alt_fn!(Pe8, super::sig::EventOut, 1);
    alt_fn!(Pe8, super::sig::Tim1Ch1n, 2);
    alt_fn!(Pe8, super::sig::FmcD5, 12);

pin!(PE9, Pe9, GPIOE, Gpioe, _PE9, GpioPin, _GPIOE, 9);
    alt_fn!(Pe9, super::sig::Adc3In2, 0);
    alt_fn!(Pe9, super::sig::EventOut, 1);
    alt_fn!(Pe9, super::sig::Tim1Ch1, 2);
    alt_fn!(Pe9, super::sig::FmcD6, 12);

pin!(PE10, Pe10, GPIOE, Gpioe, _PE10, GpioPin, _GPIOE, 10);
    alt_fn!(Pe10, super::sig::Adc3In14, 0);
    alt_fn!(Pe10, super::sig::EventOut, 1);
    alt_fn!(Pe10, super::sig::Tim1Ch2n, 2);
    alt_fn!(Pe10, super::sig::FmcD7, 12);

pin!(PE11, Pe11, GPIOE, Gpioe, _PE11, GpioPin, _GPIOE, 11);
    alt_fn!(Pe11, super::sig::Adc3In15, 0);
    alt_fn!(Pe11, super::sig::EventOut, 1);
    alt_fn!(Pe11, super::sig::Tim1Ch2, 2);
    alt_fn!(Pe11, super::sig::Spi4Nss, 5);
    alt_fn!(Pe11, super::sig::FmcD8, 12);

pin!(PE12, Pe12, GPIOE, Gpioe, _PE12, GpioPin, _GPIOE, 12);
    alt_fn!(Pe12, super::sig::Adc3In16, 0);
    alt_fn!(Pe12, super::sig::EventOut, 1);
    alt_fn!(Pe12, super::sig::Tim1Ch3n, 2);
    alt_fn!(Pe12, super::sig::Spi4Sck, 5);
    alt_fn!(Pe12, super::sig::FmcD9, 12);

pin!(PE13, Pe13, GPIOE, Gpioe, _PE13, GpioPin, _GPIOE, 13);
    alt_fn!(Pe13, super::sig::Adc3In3, 0);
    alt_fn!(Pe13, super::sig::EventOut, 1);
    alt_fn!(Pe13, super::sig::Tim1Ch3, 2);
    alt_fn!(Pe13, super::sig::Spi4Miso, 5);
    alt_fn!(Pe13, super::sig::FmcD10, 12);

pin!(PE14, Pe14, GPIOE, Gpioe, _PE14, GpioPin, _GPIOE, 14);
    alt_fn!(Pe14, super::sig::Adc4In1, 0);
    alt_fn!(Pe14, super::sig::EventOut, 1);
    alt_fn!(Pe14, super::sig::Tim1Ch4, 2);
    alt_fn!(Pe14, super::sig::Spi4Mosi, 5);
    alt_fn!(Pe14, super::sig::Tim1Bkin2, 6);
    alt_fn!(Pe14, super::sig::FmcD11, 12);

pin!(PE15, Pe15, GPIOE, Gpioe, _PE15, GpioPin, _GPIOE, 15);
    alt_fn!(Pe15, super::sig::Adc4In2, 0);
    alt_fn!(Pe15, super::sig::EventOut, 1);
    alt_fn!(Pe15, super::sig::Tim1Bkin, 2);
    alt_fn!(Pe15, super::sig::Usart3Rx, 6);
    alt_fn!(Pe15, super::sig::FmcD12, 12);

pin!(PF0, Pf0, GPIOF, Gpiof, _PF0, GpioPin, _GPIOF, 0);
    alt_fn!(Pf0, super::sig::EventOut, 1);
    alt_fn!(Pf0, super::sig::I2c2Sda, 4);
    alt_fn!(Pf0, super::sig::Spi2Nss, 5);
    alt_fn!(Pf0, super::sig::I2s2Ws, 5);
    alt_fn!(Pf0, super::sig::Tim1Ch3n, 6);

pin!(PF1, Pf1, GPIOF, Gpiof, _PF1, GpioPin, _GPIOF, 1);
    alt_fn!(Pf1, super::sig::EventOut, 1);
    alt_fn!(Pf1, super::sig::I2c2Scl, 4);
    alt_fn!(Pf1, super::sig::Spi2Sck, 5);
    alt_fn!(Pf1, super::sig::I2s2Ck, 5);

pin!(PF2, Pf2, GPIOF, Gpiof, _PF2, GpioPin, _GPIOF, 2);
    alt_fn!(Pf2, super::sig::Adc1In10, 0);
    alt_fn!(Pf2, super::sig::Adc2In10, 0);
    alt_fn!(Pf2, super::sig::EventOut, 1);
    alt_fn!(Pf2, super::sig::Tim20Ch3, 2);
    alt_fn!(Pf2, super::sig::FmcA2, 12);

pin!(PF3, Pf3, GPIOF, Gpiof, _PF3, GpioPin, _GPIOF, 3);
    alt_fn!(Pf3, super::sig::EventOut, 1);
    alt_fn!(Pf3, super::sig::Tim20Ch4, 2);
    alt_fn!(Pf3, super::sig::FmcA3, 12);

pin!(PF4, Pf4, GPIOF, Gpiof, _PF4, GpioPin, _GPIOF, 4);
    alt_fn!(Pf4, super::sig::Adc1In5, 0);
    alt_fn!(Pf4, super::sig::EventOut, 1);
    alt_fn!(Pf4, super::sig::Comp1Out, 2);
    alt_fn!(Pf4, super::sig::Tim20Ch1n, 3);
    alt_fn!(Pf4, super::sig::FmcA4, 12);

pin!(PF5, Pf5, GPIOF, Gpiof, _PF5, GpioPin, _GPIOF, 5);
    alt_fn!(Pf5, super::sig::EventOut, 1);
    alt_fn!(Pf5, super::sig::Tim20Ch2n, 2);
    alt_fn!(Pf5, super::sig::FmcA5, 12);

pin!(PF6, Pf6, GPIOF, Gpiof, _PF6, GpioPin, _GPIOF, 6);
    alt_fn!(Pf6, super::sig::EventOut, 1);
    alt_fn!(Pf6, super::sig::Tim4Ch4, 2);
    alt_fn!(Pf6, super::sig::I2c2Scl, 4);
    alt_fn!(Pf6, super::sig::Usart3Rts, 6);
    alt_fn!(Pf6, super::sig::FmcNiord, 12);

pin!(PF7, Pf7, GPIOF, Gpiof, _PF7, GpioPin, _GPIOF, 7);
    alt_fn!(Pf7, super::sig::EventOut, 1);
    alt_fn!(Pf7, super::sig::Tim20Bkin, 2);
    alt_fn!(Pf7, super::sig::FmcNreg, 12);

pin!(PF8, Pf8, GPIOF, Gpiof, _PF8, GpioPin, _GPIOF, 8);
    alt_fn!(Pf8, super::sig::EventOut, 1);
    alt_fn!(Pf8, super::sig::Tim20Bkin2, 2);
    alt_fn!(Pf8, super::sig::FmcNiowr, 12);

pin!(PF9, Pf9, GPIOF, Gpiof, _PF9, GpioPin, _GPIOF, 9);
    alt_fn!(Pf9, super::sig::EventOut, 1);
    alt_fn!(Pf9, super::sig::Tim20Bkin, 2);
    alt_fn!(Pf9, super::sig::Tim15Ch1, 3);
    alt_fn!(Pf9, super::sig::Spi2Sck, 5);
    alt_fn!(Pf9, super::sig::FmcCd, 12);

pin!(PF10, Pf10, GPIOF, Gpiof, _PF10, GpioPin, _GPIOF, 10);
    alt_fn!(Pf10, super::sig::EventOut, 1);
    alt_fn!(Pf10, super::sig::Tim20Bkin2, 2);
    alt_fn!(Pf10, super::sig::Tim15Ch2, 3);
    alt_fn!(Pf10, super::sig::Spi2Sck, 5);
    alt_fn!(Pf10, super::sig::FmcIntr, 12);

pin!(PF11, Pf11, GPIOF, Gpiof, _PF11, GpioPin, _GPIOF, 11);
    alt_fn!(Pf11, super::sig::EventOut, 1);
    alt_fn!(Pf11, super::sig::Tim20Etr, 2);

pin!(PF12, Pf12, GPIOF, Gpiof, _PF12, GpioPin, _GPIOF, 12);
    alt_fn!(Pf12, super::sig::EventOut, 1);
    alt_fn!(Pf12, super::sig::Tim20Ch1, 2);
    alt_fn!(Pf12, super::sig::FmcA6, 12);

pin!(PF13, Pf13, GPIOF, Gpiof, _PF13, GpioPin, _GPIOF, 13);
    alt_fn!(Pf13, super::sig::EventOut, 1);
    alt_fn!(Pf13, super::sig::Tim20Ch2, 2);
    alt_fn!(Pf13, super::sig::FmcA7, 12);

pin!(PF14, Pf14, GPIOF, Gpiof, _PF14, GpioPin, _GPIOF, 14);
    alt_fn!(Pf14, super::sig::EventOut, 1);
    alt_fn!(Pf14, super::sig::Tim20Ch3, 2);
    alt_fn!(Pf14, super::sig::FmcA8, 12);

pin!(PF15, Pf15, GPIOF, Gpiof, _PF15, GpioPin, _GPIOF, 15);
    alt_fn!(Pf15, super::sig::EventOut, 1);
    alt_fn!(Pf15, super::sig::Tim20Ch4, 2);
    alt_fn!(Pf15, super::sig::FmcA9, 12);

pin!(PG0, Pg0, GPIOG, Gpiog, _PG0, GpioPin, _GPIOG, 0);
    alt_fn!(Pg0, super::sig::EventOut, 1);
    alt_fn!(Pg0, super::sig::Tim20Ch1n, 2);
    alt_fn!(Pg0, super::sig::FmcA10, 12);

pin!(PG1, Pg1, GPIOG, Gpiog, _PG1, GpioPin, _GPIOG, 1);
    alt_fn!(Pg1, super::sig::EventOut, 1);
    alt_fn!(Pg1, super::sig::Tim20Ch2n, 2);
    alt_fn!(Pg1, super::sig::FmcA11, 12);

pin!(PG2, Pg2, GPIOG, Gpiog, _PG2, GpioPin, _GPIOG, 2);
    alt_fn!(Pg2, super::sig::EventOut, 1);
    alt_fn!(Pg2, super::sig::Tim20Ch3n, 2);
    alt_fn!(Pg2, super::sig::FmcA12, 12);

pin!(PG3, Pg3, GPIOG, Gpiog, _PG3, GpioPin, _GPIOG, 3);
    alt_fn!(Pg3, super::sig::EventOut, 1);
    alt_fn!(Pg3, super::sig::Tim20Bkin, 2);
    alt_fn!(Pg3, super::sig::FmcA13, 12);

pin!(PG4, Pg4, GPIOG, Gpiog, _PG4, GpioPin, _GPIOG, 4);
    alt_fn!(Pg4, super::sig::EventOut, 1);
    alt_fn!(Pg4, super::sig::Tim20Bkin2, 2);
    alt_fn!(Pg4, super::sig::FmcA14, 12);

pin!(PG5, Pg5, GPIOG, Gpiog, _PG5, GpioPin, _GPIOG, 5);
    alt_fn!(Pg5, super::sig::EventOut, 1);
    alt_fn!(Pg5, super::sig::Tim20Etr, 2);
    alt_fn!(Pg5, super::sig::FmcA15, 12);

pin!(PG6, Pg6, GPIOG, Gpiog, _PG6, GpioPin, _GPIOG, 6);
    alt_fn!(Pg6, super::sig::EventOut, 1);
    alt_fn!(Pg6, super::sig::FmcInt2, 12);

pin!(PG7, Pg7, GPIOG, Gpiog, _PG7, GpioPin, _GPIOG, 7);
    alt_fn!(Pg7, super::sig::EventOut, 1);
    alt_fn!(Pg7, super::sig::FmcInt3, 12);

pin!(PG8, Pg8, GPIOG, Gpiog, _PG8, GpioPin, _GPIOG, 8);
    alt_fn!(Pg8, super::sig::EventOut, 1);

pin!(PG9, Pg9, GPIOG, Gpiog, _PG9, GpioPin, _GPIOG, 9);
    alt_fn!(Pg9, super::sig::EventOut, 1);
    alt_fn!(Pg9, super::sig::FmcNe2, 12);
    alt_fn!(Pg9, super::sig::FmcNce3, 12);

pin!(PG10, Pg10, GPIOG, Gpiog, _PG10, GpioPin, _GPIOG, 10);
    alt_fn!(Pg10, super::sig::EventOut, 1);
    alt_fn!(Pg10, super::sig::FmcNce41, 12);
    alt_fn!(Pg10, super::sig::FmcNe3, 12);

pin!(PG11, Pg11, GPIOG, Gpiog, _PG11, GpioPin, _GPIOG, 11);
    alt_fn!(Pg11, super::sig::EventOut, 1);
    alt_fn!(Pg11, super::sig::Fmc42, 12);

pin!(PG12, Pg12, GPIOG, Gpiog, _PG12, GpioPin, _GPIOG, 12);
    alt_fn!(Pg12, super::sig::EventOut, 1);
    alt_fn!(Pg12, super::sig::FmcNe4, 12);

pin!(PG13, Pg13, GPIOG, Gpiog, _PG13, GpioPin, _GPIOG, 13);
    alt_fn!(Pg13, super::sig::EventOut, 1);
    alt_fn!(Pg13, super::sig::FmcA24, 12);

pin!(PG14, Pg14, GPIOG, Gpiog, _PG14, GpioPin, _GPIOG, 14);
    alt_fn!(Pg14, super::sig::EventOut, 1);
    alt_fn!(Pg14, super::sig::FmcA25, 12);

pin!(PG15, Pg15, GPIOG, Gpiog, _PG15, GpioPin, _GPIOG, 15);
    alt_fn!(Pg15, super::sig::EventOut, 1);

pin!(PH0, Ph0, GPIOH, Gpioh, _PH0, GpioPin, _GPIOH, 0);
    alt_fn!(Ph0, super::sig::EventOut, 1);
    alt_fn!(Ph0, super::sig::Tim20Ch1, 2);
    alt_fn!(Ph0, super::sig::FmcA0, 12);

pin!(PH1, Ph1, GPIOH, Gpioh, _PH1, GpioPin, _GPIOH, 1);
    alt_fn!(Ph1, super::sig::EventOut, 1);
    alt_fn!(Ph1, super::sig::Tim20Ch2, 2);
    alt_fn!(Ph1, super::sig::FmcA1, 12);

pin!(PH2, Ph2, GPIOH, Gpioh, _PH2, GpioPin, _GPIOH, 2);
    alt_fn!(Ph2, super::sig::EventOut, 1);

