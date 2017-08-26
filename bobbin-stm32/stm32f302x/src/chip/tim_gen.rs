#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::tim_gen::*;

periph!(TimGenPeriph, TIM2, Tim2, 0x40000000);
periph!(TimGenPeriph, TIM3, Tim3, 0x40000400);
periph!(TimGenPeriph, TIM4, Tim4, 0x40000800);
periph!(TimGenPeriph, TIM15, Tim15, 0x40014000);
periph!(TimGenPeriph, TIM16, Tim16, 0x40014400);
periph!(TimGenPeriph, TIM17, Tim17, 0x40014800);

impl super::sig::Signal<super::sig::Tim2Ch1> for Tim2Ch1 {}
impl super::sig::SignalTim<super::sig::Tim2Ch1> for Tim2Ch1 {}
impl super::sig::Signal<super::sig::Tim2Ch2> for Tim2Ch2 {}
impl super::sig::SignalTim<super::sig::Tim2Ch2> for Tim2Ch2 {}
impl super::sig::Signal<super::sig::Tim2Ch3> for Tim2Ch3 {}
impl super::sig::SignalTim<super::sig::Tim2Ch3> for Tim2Ch3 {}
impl super::sig::Signal<super::sig::Tim2Ch4> for Tim2Ch4 {}
impl super::sig::SignalTim<super::sig::Tim2Ch4> for Tim2Ch4 {}

impl super::sig::Signal<super::sig::Tim3Ch1> for Tim3Ch1 {}
impl super::sig::SignalTim<super::sig::Tim3Ch1> for Tim3Ch1 {}
impl super::sig::Signal<super::sig::Tim3Ch2> for Tim3Ch2 {}
impl super::sig::SignalTim<super::sig::Tim3Ch2> for Tim3Ch2 {}
impl super::sig::Signal<super::sig::Tim3Ch3> for Tim3Ch3 {}
impl super::sig::SignalTim<super::sig::Tim3Ch3> for Tim3Ch3 {}
impl super::sig::Signal<super::sig::Tim3Ch4> for Tim3Ch4 {}
impl super::sig::SignalTim<super::sig::Tim3Ch4> for Tim3Ch4 {}

impl super::sig::Signal<super::sig::Tim4Ch1> for Tim4Ch1 {}
impl super::sig::SignalTim<super::sig::Tim4Ch1> for Tim4Ch1 {}
impl super::sig::Signal<super::sig::Tim4Ch2> for Tim4Ch2 {}
impl super::sig::SignalTim<super::sig::Tim4Ch2> for Tim4Ch2 {}
impl super::sig::Signal<super::sig::Tim4Ch3> for Tim4Ch3 {}
impl super::sig::SignalTim<super::sig::Tim4Ch3> for Tim4Ch3 {}
impl super::sig::Signal<super::sig::Tim4Ch4> for Tim4Ch4 {}
impl super::sig::SignalTim<super::sig::Tim4Ch4> for Tim4Ch4 {}

impl super::sig::Signal<super::sig::Tim15Ch1> for Tim15Ch1 {}
impl super::sig::SignalTim<super::sig::Tim15Ch1> for Tim15Ch1 {}
impl super::sig::Signal<super::sig::Tim15Ch2> for Tim15Ch2 {}
impl super::sig::SignalTim<super::sig::Tim15Ch2> for Tim15Ch2 {}

impl super::sig::Signal<super::sig::Tim16Ch1> for Tim16Ch1 {}
impl super::sig::SignalTim<super::sig::Tim16Ch1> for Tim16Ch1 {}
impl super::sig::Signal<super::sig::Tim16Ch1n> for Tim16Ch1 {}
impl super::sig::SignalTimn<super::sig::Tim16Ch1n> for Tim16Ch1 {}

impl super::sig::Signal<super::sig::Tim17Ch1> for Tim17Ch1 {}
impl super::sig::SignalTim<super::sig::Tim17Ch1> for Tim17Ch1 {}
impl super::sig::Signal<super::sig::Tim17Ch1n> for Tim17Ch1 {}
impl super::sig::SignalTimn<super::sig::Tim17Ch1n> for Tim17Ch1 {}


channel!(TIM2_CH1, Tim2Ch1, TIM2, Tim2, 0);
channel!(TIM2_CH2, Tim2Ch2, TIM2, Tim2, 1);
channel!(TIM2_CH3, Tim2Ch3, TIM2, Tim2, 2);
channel!(TIM2_CH4, Tim2Ch4, TIM2, Tim2, 3);
channel!(TIM3_CH1, Tim3Ch1, TIM3, Tim3, 0);
channel!(TIM3_CH2, Tim3Ch2, TIM3, Tim3, 1);
channel!(TIM3_CH3, Tim3Ch3, TIM3, Tim3, 2);
channel!(TIM3_CH4, Tim3Ch4, TIM3, Tim3, 3);
channel!(TIM4_CH1, Tim4Ch1, TIM4, Tim4, 0);
channel!(TIM4_CH2, Tim4Ch2, TIM4, Tim4, 1);
channel!(TIM4_CH3, Tim4Ch3, TIM4, Tim4, 2);
channel!(TIM4_CH4, Tim4Ch4, TIM4, Tim4, 3);
channel!(TIM15_CH1, Tim15Ch1, TIM15, Tim15, 0);
channel!(TIM15_CH2, Tim15Ch2, TIM15, Tim15, 1);
channel!(TIM16_CH1, Tim16Ch1, TIM16, Tim16, 0);
channel!(TIM17_CH1, Tim17Ch1, TIM17, Tim17, 0);
