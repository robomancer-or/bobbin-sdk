#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::adc_f3::*;

periph!( ADC, Adc, _ADC, AdcPeriph, 0x50040000);

impl super::sig::Signal<super::sig::Adc1In1> for Adc1Ch1 {}
impl super::sig::SignalAdc<super::sig::Adc1In1> for Adc1Ch1 {}
impl super::sig::Signal<super::sig::Adc1In2> for Adc1Ch2 {}
impl super::sig::SignalAdc<super::sig::Adc1In2> for Adc1Ch2 {}
impl super::sig::Signal<super::sig::Adc1In3> for Adc1Ch3 {}
impl super::sig::SignalAdc<super::sig::Adc1In3> for Adc1Ch3 {}
impl super::sig::Signal<super::sig::Adc1In4> for Adc1Ch4 {}
impl super::sig::SignalAdc<super::sig::Adc1In4> for Adc1Ch4 {}
impl super::sig::Signal<super::sig::Adc1In5> for Adc1Ch5 {}
impl super::sig::SignalAdc<super::sig::Adc1In5> for Adc1Ch5 {}
impl super::sig::Signal<super::sig::Adc1In6> for Adc1Ch6 {}
impl super::sig::SignalAdc<super::sig::Adc1In6> for Adc1Ch6 {}
impl super::sig::Signal<super::sig::Adc1In7> for Adc1Ch7 {}
impl super::sig::SignalAdc<super::sig::Adc1In7> for Adc1Ch7 {}
impl super::sig::Signal<super::sig::Adc1In8> for Adc1Ch8 {}
impl super::sig::SignalAdc<super::sig::Adc1In8> for Adc1Ch8 {}
impl super::sig::Signal<super::sig::Adc1In9> for Adc1Ch9 {}
impl super::sig::SignalAdc<super::sig::Adc1In9> for Adc1Ch9 {}
impl super::sig::Signal<super::sig::Adc1In10> for Adc1Ch10 {}
impl super::sig::SignalAdc<super::sig::Adc1In10> for Adc1Ch10 {}
impl super::sig::Signal<super::sig::Adc1In11> for Adc1Ch11 {}
impl super::sig::SignalAdc<super::sig::Adc1In11> for Adc1Ch11 {}
impl super::sig::Signal<super::sig::Adc1In12> for Adc1Ch12 {}
impl super::sig::SignalAdc<super::sig::Adc1In12> for Adc1Ch12 {}
impl super::sig::Signal<super::sig::Adc1In13> for Adc1Ch13 {}
impl super::sig::SignalAdc<super::sig::Adc1In13> for Adc1Ch13 {}
impl super::sig::Signal<super::sig::Adc1In14> for Adc1Ch14 {}
impl super::sig::SignalAdc<super::sig::Adc1In14> for Adc1Ch14 {}
impl super::sig::Signal<super::sig::Adc1In15> for Adc1Ch15 {}
impl super::sig::SignalAdc<super::sig::Adc1In15> for Adc1Ch15 {}


channel!(ADC1_CH1, Adc1Ch1, ADC, Adc, _ADC1_CH1, AdcCh, _ADC, 1);
channel!(ADC1_CH2, Adc1Ch2, ADC, Adc, _ADC1_CH2, AdcCh, _ADC, 2);
channel!(ADC1_CH3, Adc1Ch3, ADC, Adc, _ADC1_CH3, AdcCh, _ADC, 3);
channel!(ADC1_CH4, Adc1Ch4, ADC, Adc, _ADC1_CH4, AdcCh, _ADC, 4);
channel!(ADC1_CH5, Adc1Ch5, ADC, Adc, _ADC1_CH5, AdcCh, _ADC, 5);
channel!(ADC1_CH6, Adc1Ch6, ADC, Adc, _ADC1_CH6, AdcCh, _ADC, 6);
channel!(ADC1_CH7, Adc1Ch7, ADC, Adc, _ADC1_CH7, AdcCh, _ADC, 7);
channel!(ADC1_CH8, Adc1Ch8, ADC, Adc, _ADC1_CH8, AdcCh, _ADC, 8);
channel!(ADC1_CH9, Adc1Ch9, ADC, Adc, _ADC1_CH9, AdcCh, _ADC, 9);
channel!(ADC1_CH10, Adc1Ch10, ADC, Adc, _ADC1_CH10, AdcCh, _ADC, 10);
channel!(ADC1_CH11, Adc1Ch11, ADC, Adc, _ADC1_CH11, AdcCh, _ADC, 11);
channel!(ADC1_CH12, Adc1Ch12, ADC, Adc, _ADC1_CH12, AdcCh, _ADC, 12);
channel!(ADC1_CH13, Adc1Ch13, ADC, Adc, _ADC1_CH13, AdcCh, _ADC, 13);
channel!(ADC1_CH14, Adc1Ch14, ADC, Adc, _ADC1_CH14, AdcCh, _ADC, 14);
channel!(ADC1_CH15, Adc1Ch15, ADC, Adc, _ADC1_CH15, AdcCh, _ADC, 15);
channel!(ADC1_TEMP, Adc1Temp, ADC, Adc, _ADC1_TEMP, AdcCh, _ADC, 17);
channel!(ADC1_REFINT, Adc1Refint, ADC, Adc, _ADC1_REFINT, AdcCh, _ADC, 0);
channel!(ADC1_BAT, Adc1Bat, ADC, Adc, _ADC1_BAT, AdcCh, _ADC, 18);

pub trait IrqAdc<T> {
    fn irq_adc(&self) -> T;
}

impl IrqAdc<super::irq::IrqAdc1> for Adc {
    fn irq_adc(&self) -> super::irq::IrqAdc1 { super::irq::IRQ_ADC1 }
}
