//! Signals

pub trait Signal<T> {}

pub trait Tx {}
pub trait SignalTx<T> {}
pub trait Rx {}
pub trait SignalRx<T> {}
pub trait Cts {}
pub trait SignalCts<T> {}
pub trait Rts {}
pub trait SignalRts<T> {}
pub trait Ck {}
pub trait SignalCk<T> {}
pub trait Adc {}
pub trait SignalAdc<T> {}
pub trait Tim {}
pub trait SignalTim<T> {}
pub trait Timn {}
pub trait SignalTimn<T> {}

pub struct Usart1Tx {}
pub struct Usart1Rx {}
pub struct Usart1Cts {}
pub struct Usart1Rts {}
pub struct Usart1Ck {}
pub struct Usart2Tx {}
pub struct Usart2Rx {}
pub struct Usart2Cts {}
pub struct Usart2Rts {}
pub struct Usart2Ck {}
pub struct Usart3Tx {}
pub struct Usart3Rx {}
pub struct Usart3Cts {}
pub struct Usart3Rts {}
pub struct Usart3Ck {}
pub struct Uart4Tx {}
pub struct Uart4Rx {}
pub struct Uart4Cts {}
pub struct Uart4Rts {}
pub struct Uart4Ck {}
pub struct Uart5Tx {}
pub struct Uart5Rx {}
pub struct Uart5Cts {}
pub struct Uart5Rts {}
pub struct Uart5Ck {}
pub struct Adc1In1 {}
pub struct Adc1In2 {}
pub struct Adc1In3 {}
pub struct Adc1In4 {}
pub struct Adc1In5 {}
pub struct Adc1In6 {}
pub struct Adc1In7 {}
pub struct Adc1In8 {}
pub struct Adc1In9 {}
pub struct Adc1In10 {}
pub struct Adc1In11 {}
pub struct Adc1In12 {}
pub struct Adc1In13 {}
pub struct Adc1In14 {}
pub struct Adc1In15 {}
pub struct Adc2In1 {}
pub struct Adc2In2 {}
pub struct Adc2In3 {}
pub struct Adc2In4 {}
pub struct Adc2In5 {}
pub struct Adc2In6 {}
pub struct Adc2In7 {}
pub struct Adc2In8 {}
pub struct Adc2In9 {}
pub struct Adc2In10 {}
pub struct Adc2In11 {}
pub struct Adc2In12 {}
pub struct Adc2In13 {}
pub struct Adc2In14 {}
pub struct Adc2In15 {}
pub struct Adc3In1 {}
pub struct Adc3In2 {}
pub struct Adc3In3 {}
pub struct Adc3In4 {}
pub struct Adc3In5 {}
pub struct Adc3In6 {}
pub struct Adc3In7 {}
pub struct Adc3In8 {}
pub struct Adc3In9 {}
pub struct Adc3In10 {}
pub struct Adc3In11 {}
pub struct Adc3In12 {}
pub struct Adc3In13 {}
pub struct Adc3In14 {}
pub struct Adc3In15 {}
pub struct Adc4In1 {}
pub struct Adc4In2 {}
pub struct Adc4In3 {}
pub struct Adc4In4 {}
pub struct Adc4In5 {}
pub struct Adc4In6 {}
pub struct Adc4In7 {}
pub struct Adc4In8 {}
pub struct Adc4In9 {}
pub struct Adc4In10 {}
pub struct Adc4In11 {}
pub struct Adc4In12 {}
pub struct Adc4In13 {}
pub struct Adc4In14 {}
pub struct Adc4In15 {}
pub struct Tim2Ch1 {}
pub struct Tim2Ch2 {}
pub struct Tim2Ch3 {}
pub struct Tim2Ch4 {}
pub struct Tim3Ch1 {}
pub struct Tim3Ch2 {}
pub struct Tim3Ch3 {}
pub struct Tim3Ch4 {}
pub struct Tim4Ch1 {}
pub struct Tim4Ch2 {}
pub struct Tim4Ch3 {}
pub struct Tim4Ch4 {}
pub struct Tim15Ch1 {}
pub struct Tim15Ch2 {}
pub struct Tim16Ch1 {}
pub struct Tim16Ch1n {}
pub struct Tim17Ch1 {}
pub struct Tim17Ch1n {}
pub struct Tim1Ch1 {}
pub struct Tim1Ch2 {}
pub struct Tim1Ch3 {}
pub struct Tim1Ch4 {}
pub struct Tim8Ch1 {}
pub struct Tim8Ch2 {}
pub struct Tim8Ch3 {}
pub struct Tim8Ch4 {}
pub struct Tim20Ch1 {}
pub struct Tim20Ch2 {}
pub struct Tim20Ch3 {}
pub struct Tim20Ch4 {}
pub struct Tim2Etr {}
pub struct TscG1Io1 {}
pub struct Comp1Out {}
pub struct Tim8Bkin {}
pub struct Tim8Etr {}
pub struct EventOut {}
pub struct RtcRefin {}
pub struct TscG1Io2 {}
pub struct Tim15Ch1n {}
pub struct TscG1Io3 {}
pub struct Comp2Out {}
pub struct TscG1Io4 {}
pub struct TscG2Io1 {}
pub struct Spi1Nss {}
pub struct Spi3Nss {}
pub struct I2s3Ws {}
pub struct TscG2Io2 {}
pub struct Spi1Sck {}
pub struct TscG2Io3 {}
pub struct Spi1Miso {}
pub struct Tim1Bkin {}
pub struct TscG2Io4 {}
pub struct Tim8Ch1n {}
pub struct Spi1Mosi {}
pub struct Tim1Ch1n {}
pub struct Mco {}
pub struct I2c3Scl {}
pub struct I2c2Smbal {}
pub struct I2s2Mck {}
pub struct Comp3Out {}
pub struct Tim4Etr {}
pub struct I2c3Smbal {}
pub struct TscG4Io1 {}
pub struct I2c2Scl {}
pub struct I2s3Mck {}
pub struct Comp5Out {}
pub struct Tim15Bkin {}
pub struct Tim17Bkin {}
pub struct TscG4Io2 {}
pub struct I2c2Sda {}
pub struct Spi2Miso {}
pub struct I2s2extSd {}
pub struct Comp6Out {}
pub struct Spi2Mosi {}
pub struct I2s2Sd {}
pub struct CanRx {}
pub struct Tim1Bkin2 {}
pub struct I2sckin {}
pub struct Tim1Ch2n {}
pub struct CanTx {}
pub struct Tim1Etr {}
pub struct Swdio {}
pub struct Jtms {}
pub struct TscG4Io3 {}
pub struct IrOut {}
pub struct Swclk {}
pub struct Jtck {}
pub struct TscG4Io4 {}
pub struct I2c1Sda {}
pub struct Jtdi {}
pub struct TscSync {}
pub struct I2c1Scl {}
pub struct TscG3Io2 {}
pub struct Tim8Ch2n {}
pub struct TscG3Io3 {}
pub struct Tim8Ch3n {}
pub struct Tim1Ch3n {}
pub struct Comp4Out {}
pub struct TscG3Io4 {}
pub struct Jtdo {}
pub struct Traceswo {}
pub struct TscG5Io1 {}
pub struct Spi3Sck {}
pub struct I2s3Ck {}
pub struct Tim3Etr {}
pub struct Jtrst {}
pub struct Spi3Miso {}
pub struct I2s3extSd {}
pub struct Tim16Bkin {}
pub struct I2c1Smbal {}
pub struct Spi3Mosi {}
pub struct I2s3Sd {}
pub struct I2c3Sda {}
pub struct TscG5Io3 {}
pub struct Tim8Bkin2 {}
pub struct TscG5Io4 {}
pub struct FmcNadv {}
pub struct TscG6Io1 {}
pub struct TscG6Io2 {}
pub struct Spi2Nss {}
pub struct I2s2Ws {}
pub struct TscG6Io3 {}
pub struct Spi2Sck {}
pub struct I2s2Ck {}
pub struct TscG6Io4 {}
pub struct Comp7Out {}
pub struct Tim1Chetr {}
pub struct TscG3Io1 {}
pub struct FmcD2 {}
pub struct FmcD3 {}
pub struct FmcClk {}
pub struct FmcNoe {}
pub struct FmcNwe {}
pub struct FmcNwait {}
pub struct FmcNe1 {}
pub struct FmcNce2 {}
pub struct FmcD13 {}
pub struct FmcD14 {}
pub struct FmcD15 {}
pub struct FmcA16 {}
pub struct TscG8Io1 {}
pub struct FmcA17 {}
pub struct TscG8Io2 {}
pub struct FmcA18 {}
pub struct TscG8Io3 {}
pub struct FmcD0 {}
pub struct TscG8Io4 {}
pub struct FmcD1 {}
pub struct Tim20Etr {}
pub struct FmcNbl0 {}
pub struct FmcNbl1 {}
pub struct Traceck {}
pub struct TscG7Io1 {}
pub struct Spi4Sck {}
pub struct FmcA23 {}
pub struct Traced0 {}
pub struct TscG7Io2 {}
pub struct Spi4Nss {}
pub struct FmcA19 {}
pub struct Traced1 {}
pub struct TscG7Io3 {}
pub struct Tim20Ch1n {}
pub struct FmcA20 {}
pub struct Traced2 {}
pub struct TscG7Io4 {}
pub struct Spi4Miso {}
pub struct Tim20Ch2n {}
pub struct FmcA21 {}
pub struct Traced3 {}
pub struct Spi4Mosi {}
pub struct Tim20Ch3n {}
pub struct FmcA22 {}
pub struct FmcD4 {}
pub struct FmcD5 {}
pub struct FmcD6 {}
pub struct FmcD7 {}
pub struct FmcD8 {}
pub struct Adc3In16 {}
pub struct FmcD9 {}
pub struct FmcD10 {}
pub struct FmcD11 {}
pub struct FmcD12 {}
pub struct FmcA2 {}
pub struct FmcA3 {}
pub struct FmcA4 {}
pub struct FmcA5 {}
pub struct FmcNiord {}
pub struct Tim20Bkin {}
pub struct FmcNreg {}
pub struct Tim20Bkin2 {}
pub struct FmcNiowr {}
pub struct FmcCd {}
pub struct FmcIntr {}
pub struct FmcA6 {}
pub struct FmcA7 {}
pub struct FmcA8 {}
pub struct FmcA9 {}
pub struct FmcA10 {}
pub struct FmcA11 {}
pub struct FmcA12 {}
pub struct FmcA13 {}
pub struct FmcA14 {}
pub struct FmcA15 {}
pub struct FmcInt2 {}
pub struct FmcInt3 {}
pub struct FmcNe2 {}
pub struct FmcNce3 {}
pub struct FmcNce41 {}
pub struct FmcNe3 {}
pub struct Fmc42 {}
pub struct FmcNe4 {}
pub struct FmcA24 {}
pub struct FmcA25 {}
pub struct FmcA0 {}
pub struct FmcA1 {}
