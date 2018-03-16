//! Alternate function I/O

#[allow(unused_imports)] use ::bobbin_common::*;

#[doc="Alternate function I/O"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct AfioPeriph(pub usize);
impl AfioPeriph {
    #[doc="Get the *mut pointer for the EVCR register."]
    #[inline] pub fn evcr_mut(&self) -> *mut Evcr { 
        (self.0 + 0x0) as *mut Evcr
    }

    #[doc="Get the *const pointer for the EVCR register."]
    #[inline] pub fn evcr_ptr(&self) -> *const Evcr { 
           self.evcr_mut()
    }

    #[doc="Read the EVCR register."]
    #[inline] pub fn evcr(&self) -> Evcr { 
        unsafe {
            read_volatile(self.evcr_ptr())
        }
    }

    #[doc="Write the EVCR register."]
    #[inline] pub fn set_evcr<F: FnOnce(Evcr) -> Evcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.evcr_mut(), f(Evcr(0)));
        }
        self
    }

    #[doc="Modify the EVCR register."]
    #[inline] pub fn with_evcr<F: FnOnce(Evcr) -> Evcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.evcr_mut(), f(self.evcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MAPR register."]
    #[inline] pub fn mapr_mut(&self) -> *mut Mapr { 
        (self.0 + 0x4) as *mut Mapr
    }

    #[doc="Get the *const pointer for the MAPR register."]
    #[inline] pub fn mapr_ptr(&self) -> *const Mapr { 
           self.mapr_mut()
    }

    #[doc="Read the MAPR register."]
    #[inline] pub fn mapr(&self) -> Mapr { 
        unsafe {
            read_volatile(self.mapr_ptr())
        }
    }

    #[doc="Write the MAPR register."]
    #[inline] pub fn set_mapr<F: FnOnce(Mapr) -> Mapr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mapr_mut(), f(Mapr(0)));
        }
        self
    }

    #[doc="Modify the MAPR register."]
    #[inline] pub fn with_mapr<F: FnOnce(Mapr) -> Mapr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mapr_mut(), f(self.mapr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EXTICR1 register."]
    #[inline] pub fn exticr1_mut(&self) -> *mut Exticr1 { 
        (self.0 + 0x8) as *mut Exticr1
    }

    #[doc="Get the *const pointer for the EXTICR1 register."]
    #[inline] pub fn exticr1_ptr(&self) -> *const Exticr1 { 
           self.exticr1_mut()
    }

    #[doc="Read the EXTICR1 register."]
    #[inline] pub fn exticr1(&self) -> Exticr1 { 
        unsafe {
            read_volatile(self.exticr1_ptr())
        }
    }

    #[doc="Write the EXTICR1 register."]
    #[inline] pub fn set_exticr1<F: FnOnce(Exticr1) -> Exticr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exticr1_mut(), f(Exticr1(0)));
        }
        self
    }

    #[doc="Modify the EXTICR1 register."]
    #[inline] pub fn with_exticr1<F: FnOnce(Exticr1) -> Exticr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exticr1_mut(), f(self.exticr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EXTICR2 register."]
    #[inline] pub fn exticr2_mut(&self) -> *mut Exticr2 { 
        (self.0 + 0xc) as *mut Exticr2
    }

    #[doc="Get the *const pointer for the EXTICR2 register."]
    #[inline] pub fn exticr2_ptr(&self) -> *const Exticr2 { 
           self.exticr2_mut()
    }

    #[doc="Read the EXTICR2 register."]
    #[inline] pub fn exticr2(&self) -> Exticr2 { 
        unsafe {
            read_volatile(self.exticr2_ptr())
        }
    }

    #[doc="Write the EXTICR2 register."]
    #[inline] pub fn set_exticr2<F: FnOnce(Exticr2) -> Exticr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exticr2_mut(), f(Exticr2(0)));
        }
        self
    }

    #[doc="Modify the EXTICR2 register."]
    #[inline] pub fn with_exticr2<F: FnOnce(Exticr2) -> Exticr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exticr2_mut(), f(self.exticr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EXTICR3 register."]
    #[inline] pub fn exticr3_mut(&self) -> *mut Exticr3 { 
        (self.0 + 0x10) as *mut Exticr3
    }

    #[doc="Get the *const pointer for the EXTICR3 register."]
    #[inline] pub fn exticr3_ptr(&self) -> *const Exticr3 { 
           self.exticr3_mut()
    }

    #[doc="Read the EXTICR3 register."]
    #[inline] pub fn exticr3(&self) -> Exticr3 { 
        unsafe {
            read_volatile(self.exticr3_ptr())
        }
    }

    #[doc="Write the EXTICR3 register."]
    #[inline] pub fn set_exticr3<F: FnOnce(Exticr3) -> Exticr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exticr3_mut(), f(Exticr3(0)));
        }
        self
    }

    #[doc="Modify the EXTICR3 register."]
    #[inline] pub fn with_exticr3<F: FnOnce(Exticr3) -> Exticr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exticr3_mut(), f(self.exticr3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EXTICR4 register."]
    #[inline] pub fn exticr4_mut(&self) -> *mut Exticr4 { 
        (self.0 + 0x14) as *mut Exticr4
    }

    #[doc="Get the *const pointer for the EXTICR4 register."]
    #[inline] pub fn exticr4_ptr(&self) -> *const Exticr4 { 
           self.exticr4_mut()
    }

    #[doc="Read the EXTICR4 register."]
    #[inline] pub fn exticr4(&self) -> Exticr4 { 
        unsafe {
            read_volatile(self.exticr4_ptr())
        }
    }

    #[doc="Write the EXTICR4 register."]
    #[inline] pub fn set_exticr4<F: FnOnce(Exticr4) -> Exticr4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exticr4_mut(), f(Exticr4(0)));
        }
        self
    }

    #[doc="Modify the EXTICR4 register."]
    #[inline] pub fn with_exticr4<F: FnOnce(Exticr4) -> Exticr4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exticr4_mut(), f(self.exticr4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MAPR2 register."]
    #[inline] pub fn mapr2_mut(&self) -> *mut Mapr2 { 
        (self.0 + 0x1c) as *mut Mapr2
    }

    #[doc="Get the *const pointer for the MAPR2 register."]
    #[inline] pub fn mapr2_ptr(&self) -> *const Mapr2 { 
           self.mapr2_mut()
    }

    #[doc="Read the MAPR2 register."]
    #[inline] pub fn mapr2(&self) -> Mapr2 { 
        unsafe {
            read_volatile(self.mapr2_ptr())
        }
    }

    #[doc="Write the MAPR2 register."]
    #[inline] pub fn set_mapr2<F: FnOnce(Mapr2) -> Mapr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mapr2_mut(), f(Mapr2(0)));
        }
        self
    }

    #[doc="Modify the MAPR2 register."]
    #[inline] pub fn with_mapr2<F: FnOnce(Mapr2) -> Mapr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mapr2_mut(), f(self.mapr2()));
        }
        self
    }

}

#[doc="Event Control Register (AFIO_EVCR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Evcr(pub u32);
impl Evcr {
    #[doc="Pin selection"]
    #[inline] pub fn pin(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if PIN != 0"]
    #[inline] pub fn test_pin(&self) -> bool {
        self.pin() != 0
    }

    #[doc="Sets the PIN field."]
    #[inline] pub fn set_pin<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Port selection"]
    #[inline] pub fn port(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if PORT != 0"]
    #[inline] pub fn test_port(&self) -> bool {
        self.port() != 0
    }

    #[doc="Sets the PORT field."]
    #[inline] pub fn set_port<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Event Output Enable"]
    #[inline] pub fn evoe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if EVOE != 0"]
    #[inline] pub fn test_evoe(&self) -> bool {
        self.evoe() != 0
    }

    #[doc="Sets the EVOE field."]
    #[inline] pub fn set_evoe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Evcr {
    #[inline]
    fn from(other: u32) -> Self {
         Evcr(other)
    }
}

impl ::core::fmt::Display for Evcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Evcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pin() != 0 { try!(write!(f, " pin=0x{:x}", self.pin()))}
        if self.port() != 0 { try!(write!(f, " port=0x{:x}", self.port()))}
        if self.evoe() != 0 { try!(write!(f, " evoe"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AF remap and debug I/O configuration register (AFIO_MAPR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mapr(pub u32);
impl Mapr {
    #[doc="SPI1 remapping"]
    #[inline] pub fn spi1_remap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SPI1_REMAP != 0"]
    #[inline] pub fn test_spi1_remap(&self) -> bool {
        self.spi1_remap() != 0
    }

    #[doc="Sets the SPI1_REMAP field."]
    #[inline] pub fn set_spi1_remap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="I2C1 remapping"]
    #[inline] pub fn i2c1_remap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if I2C1_REMAP != 0"]
    #[inline] pub fn test_i2c1_remap(&self) -> bool {
        self.i2c1_remap() != 0
    }

    #[doc="Sets the I2C1_REMAP field."]
    #[inline] pub fn set_i2c1_remap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="USART1 remapping"]
    #[inline] pub fn usart1_remap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if USART1_REMAP != 0"]
    #[inline] pub fn test_usart1_remap(&self) -> bool {
        self.usart1_remap() != 0
    }

    #[doc="Sets the USART1_REMAP field."]
    #[inline] pub fn set_usart1_remap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="USART2 remapping"]
    #[inline] pub fn usart2_remap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if USART2_REMAP != 0"]
    #[inline] pub fn test_usart2_remap(&self) -> bool {
        self.usart2_remap() != 0
    }

    #[doc="Sets the USART2_REMAP field."]
    #[inline] pub fn set_usart2_remap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="USART3 remapping"]
    #[inline] pub fn usart3_remap(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if USART3_REMAP != 0"]
    #[inline] pub fn test_usart3_remap(&self) -> bool {
        self.usart3_remap() != 0
    }

    #[doc="Sets the USART3_REMAP field."]
    #[inline] pub fn set_usart3_remap<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TIM1 remapping"]
    #[inline] pub fn tim1_remap(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if TIM1_REMAP != 0"]
    #[inline] pub fn test_tim1_remap(&self) -> bool {
        self.tim1_remap() != 0
    }

    #[doc="Sets the TIM1_REMAP field."]
    #[inline] pub fn set_tim1_remap<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="TIM2 remapping"]
    #[inline] pub fn tim2_remap(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if TIM2_REMAP != 0"]
    #[inline] pub fn test_tim2_remap(&self) -> bool {
        self.tim2_remap() != 0
    }

    #[doc="Sets the TIM2_REMAP field."]
    #[inline] pub fn set_tim2_remap<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="TIM3 remapping"]
    #[inline] pub fn tim3_remap(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if TIM3_REMAP != 0"]
    #[inline] pub fn test_tim3_remap(&self) -> bool {
        self.tim3_remap() != 0
    }

    #[doc="Sets the TIM3_REMAP field."]
    #[inline] pub fn set_tim3_remap<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="TIM4 remapping"]
    #[inline] pub fn tim4_remap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TIM4_REMAP != 0"]
    #[inline] pub fn test_tim4_remap(&self) -> bool {
        self.tim4_remap() != 0
    }

    #[doc="Sets the TIM4_REMAP field."]
    #[inline] pub fn set_tim4_remap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CAN1 remapping"]
    #[inline] pub fn can_remap(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
    }

    #[doc="Returns true if CAN_REMAP != 0"]
    #[inline] pub fn test_can_remap(&self) -> bool {
        self.can_remap() != 0
    }

    #[doc="Sets the CAN_REMAP field."]
    #[inline] pub fn set_can_remap<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port D0/Port D1 mapping on OSCIN/OSCOUT"]
    #[inline] pub fn pd01_remap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PD01_REMAP != 0"]
    #[inline] pub fn test_pd01_remap(&self) -> bool {
        self.pd01_remap() != 0
    }

    #[doc="Sets the PD01_REMAP field."]
    #[inline] pub fn set_pd01_remap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Set and cleared by software"]
    #[inline] pub fn tim5ch4_iremap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TIM5CH4_IREMAP != 0"]
    #[inline] pub fn test_tim5ch4_iremap(&self) -> bool {
        self.tim5ch4_iremap() != 0
    }

    #[doc="Sets the TIM5CH4_IREMAP field."]
    #[inline] pub fn set_tim5ch4_iremap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="ADC 1 External trigger injected conversion remapping"]
    #[inline] pub fn adc1_etrginj_remap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if ADC1_ETRGINJ_REMAP != 0"]
    #[inline] pub fn test_adc1_etrginj_remap(&self) -> bool {
        self.adc1_etrginj_remap() != 0
    }

    #[doc="Sets the ADC1_ETRGINJ_REMAP field."]
    #[inline] pub fn set_adc1_etrginj_remap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="ADC 1 external trigger regular conversion remapping"]
    #[inline] pub fn adc1_etrgreg_remap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if ADC1_ETRGREG_REMAP != 0"]
    #[inline] pub fn test_adc1_etrgreg_remap(&self) -> bool {
        self.adc1_etrgreg_remap() != 0
    }

    #[doc="Sets the ADC1_ETRGREG_REMAP field."]
    #[inline] pub fn set_adc1_etrgreg_remap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="ADC 2 external trigger injected conversion remapping"]
    #[inline] pub fn adc2_etrginj_remap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if ADC2_ETRGINJ_REMAP != 0"]
    #[inline] pub fn test_adc2_etrginj_remap(&self) -> bool {
        self.adc2_etrginj_remap() != 0
    }

    #[doc="Sets the ADC2_ETRGINJ_REMAP field."]
    #[inline] pub fn set_adc2_etrginj_remap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="ADC 2 external trigger regular conversion remapping"]
    #[inline] pub fn adc2_etrgreg_remap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if ADC2_ETRGREG_REMAP != 0"]
    #[inline] pub fn test_adc2_etrgreg_remap(&self) -> bool {
        self.adc2_etrgreg_remap() != 0
    }

    #[doc="Sets the ADC2_ETRGREG_REMAP field."]
    #[inline] pub fn set_adc2_etrgreg_remap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Serial wire JTAG configuration"]
    #[inline] pub fn swj_cfg(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if SWJ_CFG != 0"]
    #[inline] pub fn test_swj_cfg(&self) -> bool {
        self.swj_cfg() != 0
    }

    #[doc="Sets the SWJ_CFG field."]
    #[inline] pub fn set_swj_cfg<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Mapr {
    #[inline]
    fn from(other: u32) -> Self {
         Mapr(other)
    }
}

impl ::core::fmt::Display for Mapr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mapr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.spi1_remap() != 0 { try!(write!(f, " spi1_remap"))}
        if self.i2c1_remap() != 0 { try!(write!(f, " i2c1_remap"))}
        if self.usart1_remap() != 0 { try!(write!(f, " usart1_remap"))}
        if self.usart2_remap() != 0 { try!(write!(f, " usart2_remap"))}
        if self.usart3_remap() != 0 { try!(write!(f, " usart3_remap=0x{:x}", self.usart3_remap()))}
        if self.tim1_remap() != 0 { try!(write!(f, " tim1_remap=0x{:x}", self.tim1_remap()))}
        if self.tim2_remap() != 0 { try!(write!(f, " tim2_remap=0x{:x}", self.tim2_remap()))}
        if self.tim3_remap() != 0 { try!(write!(f, " tim3_remap=0x{:x}", self.tim3_remap()))}
        if self.tim4_remap() != 0 { try!(write!(f, " tim4_remap"))}
        if self.can_remap() != 0 { try!(write!(f, " can_remap=0x{:x}", self.can_remap()))}
        if self.pd01_remap() != 0 { try!(write!(f, " pd01_remap"))}
        if self.tim5ch4_iremap() != 0 { try!(write!(f, " tim5ch4_iremap"))}
        if self.adc1_etrginj_remap() != 0 { try!(write!(f, " adc1_etrginj_remap"))}
        if self.adc1_etrgreg_remap() != 0 { try!(write!(f, " adc1_etrgreg_remap"))}
        if self.adc2_etrginj_remap() != 0 { try!(write!(f, " adc2_etrginj_remap"))}
        if self.adc2_etrgreg_remap() != 0 { try!(write!(f, " adc2_etrgreg_remap"))}
        if self.swj_cfg() != 0 { try!(write!(f, " swj_cfg=0x{:x}", self.swj_cfg()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="External interrupt configuration register 1 (AFIO_EXTICR1)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exticr1(pub u32);
impl Exticr1 {
    #[doc="EXTI0 configuration"]
    #[inline] pub fn exti0(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EXTI0 != 0"]
    #[inline] pub fn test_exti0(&self) -> bool {
        self.exti0() != 0
    }

    #[doc="Sets the EXTI0 field."]
    #[inline] pub fn set_exti0<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="EXTI1 configuration"]
    #[inline] pub fn exti1(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if EXTI1 != 0"]
    #[inline] pub fn test_exti1(&self) -> bool {
        self.exti1() != 0
    }

    #[doc="Sets the EXTI1 field."]
    #[inline] pub fn set_exti1<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EXTI2 configuration"]
    #[inline] pub fn exti2(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if EXTI2 != 0"]
    #[inline] pub fn test_exti2(&self) -> bool {
        self.exti2() != 0
    }

    #[doc="Sets the EXTI2 field."]
    #[inline] pub fn set_exti2<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="EXTI3 configuration"]
    #[inline] pub fn exti3(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if EXTI3 != 0"]
    #[inline] pub fn test_exti3(&self) -> bool {
        self.exti3() != 0
    }

    #[doc="Sets the EXTI3 field."]
    #[inline] pub fn set_exti3<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u32> for Exticr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Exticr1(other)
    }
}

impl ::core::fmt::Display for Exticr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exticr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exti0() != 0 { try!(write!(f, " exti0=0x{:x}", self.exti0()))}
        if self.exti1() != 0 { try!(write!(f, " exti1=0x{:x}", self.exti1()))}
        if self.exti2() != 0 { try!(write!(f, " exti2=0x{:x}", self.exti2()))}
        if self.exti3() != 0 { try!(write!(f, " exti3=0x{:x}", self.exti3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="External interrupt configuration register 2 (AFIO_EXTICR2)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exticr2(pub u32);
impl Exticr2 {
    #[doc="EXTI4 configuration"]
    #[inline] pub fn exti4(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EXTI4 != 0"]
    #[inline] pub fn test_exti4(&self) -> bool {
        self.exti4() != 0
    }

    #[doc="Sets the EXTI4 field."]
    #[inline] pub fn set_exti4<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="EXTI5 configuration"]
    #[inline] pub fn exti5(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if EXTI5 != 0"]
    #[inline] pub fn test_exti5(&self) -> bool {
        self.exti5() != 0
    }

    #[doc="Sets the EXTI5 field."]
    #[inline] pub fn set_exti5<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EXTI6 configuration"]
    #[inline] pub fn exti6(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if EXTI6 != 0"]
    #[inline] pub fn test_exti6(&self) -> bool {
        self.exti6() != 0
    }

    #[doc="Sets the EXTI6 field."]
    #[inline] pub fn set_exti6<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="EXTI7 configuration"]
    #[inline] pub fn exti7(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if EXTI7 != 0"]
    #[inline] pub fn test_exti7(&self) -> bool {
        self.exti7() != 0
    }

    #[doc="Sets the EXTI7 field."]
    #[inline] pub fn set_exti7<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u32> for Exticr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Exticr2(other)
    }
}

impl ::core::fmt::Display for Exticr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exticr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exti4() != 0 { try!(write!(f, " exti4=0x{:x}", self.exti4()))}
        if self.exti5() != 0 { try!(write!(f, " exti5=0x{:x}", self.exti5()))}
        if self.exti6() != 0 { try!(write!(f, " exti6=0x{:x}", self.exti6()))}
        if self.exti7() != 0 { try!(write!(f, " exti7=0x{:x}", self.exti7()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="External interrupt configuration register 3 (AFIO_EXTICR3)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exticr3(pub u32);
impl Exticr3 {
    #[doc="EXTI8 configuration"]
    #[inline] pub fn exti8(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EXTI8 != 0"]
    #[inline] pub fn test_exti8(&self) -> bool {
        self.exti8() != 0
    }

    #[doc="Sets the EXTI8 field."]
    #[inline] pub fn set_exti8<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="EXTI9 configuration"]
    #[inline] pub fn exti9(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if EXTI9 != 0"]
    #[inline] pub fn test_exti9(&self) -> bool {
        self.exti9() != 0
    }

    #[doc="Sets the EXTI9 field."]
    #[inline] pub fn set_exti9<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EXTI10 configuration"]
    #[inline] pub fn exti10(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if EXTI10 != 0"]
    #[inline] pub fn test_exti10(&self) -> bool {
        self.exti10() != 0
    }

    #[doc="Sets the EXTI10 field."]
    #[inline] pub fn set_exti10<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="EXTI11 configuration"]
    #[inline] pub fn exti11(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if EXTI11 != 0"]
    #[inline] pub fn test_exti11(&self) -> bool {
        self.exti11() != 0
    }

    #[doc="Sets the EXTI11 field."]
    #[inline] pub fn set_exti11<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u32> for Exticr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Exticr3(other)
    }
}

impl ::core::fmt::Display for Exticr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exticr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exti8() != 0 { try!(write!(f, " exti8=0x{:x}", self.exti8()))}
        if self.exti9() != 0 { try!(write!(f, " exti9=0x{:x}", self.exti9()))}
        if self.exti10() != 0 { try!(write!(f, " exti10=0x{:x}", self.exti10()))}
        if self.exti11() != 0 { try!(write!(f, " exti11=0x{:x}", self.exti11()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="External interrupt configuration register 4 (AFIO_EXTICR4)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exticr4(pub u32);
impl Exticr4 {
    #[doc="EXTI12 configuration"]
    #[inline] pub fn exti12(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EXTI12 != 0"]
    #[inline] pub fn test_exti12(&self) -> bool {
        self.exti12() != 0
    }

    #[doc="Sets the EXTI12 field."]
    #[inline] pub fn set_exti12<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="EXTI13 configuration"]
    #[inline] pub fn exti13(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if EXTI13 != 0"]
    #[inline] pub fn test_exti13(&self) -> bool {
        self.exti13() != 0
    }

    #[doc="Sets the EXTI13 field."]
    #[inline] pub fn set_exti13<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EXTI14 configuration"]
    #[inline] pub fn exti14(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if EXTI14 != 0"]
    #[inline] pub fn test_exti14(&self) -> bool {
        self.exti14() != 0
    }

    #[doc="Sets the EXTI14 field."]
    #[inline] pub fn set_exti14<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="EXTI15 configuration"]
    #[inline] pub fn exti15(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if EXTI15 != 0"]
    #[inline] pub fn test_exti15(&self) -> bool {
        self.exti15() != 0
    }

    #[doc="Sets the EXTI15 field."]
    #[inline] pub fn set_exti15<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u32> for Exticr4 {
    #[inline]
    fn from(other: u32) -> Self {
         Exticr4(other)
    }
}

impl ::core::fmt::Display for Exticr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exticr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exti12() != 0 { try!(write!(f, " exti12=0x{:x}", self.exti12()))}
        if self.exti13() != 0 { try!(write!(f, " exti13=0x{:x}", self.exti13()))}
        if self.exti14() != 0 { try!(write!(f, " exti14=0x{:x}", self.exti14()))}
        if self.exti15() != 0 { try!(write!(f, " exti15=0x{:x}", self.exti15()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AF remap and debug I/O configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mapr2(pub u32);
impl Mapr2 {
    #[doc="TIM9 remapping"]
    #[inline] pub fn tim9_remap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TIM9_REMAP != 0"]
    #[inline] pub fn test_tim9_remap(&self) -> bool {
        self.tim9_remap() != 0
    }

    #[doc="Sets the TIM9_REMAP field."]
    #[inline] pub fn set_tim9_remap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="TIM10 remapping"]
    #[inline] pub fn tim10_remap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TIM10_REMAP != 0"]
    #[inline] pub fn test_tim10_remap(&self) -> bool {
        self.tim10_remap() != 0
    }

    #[doc="Sets the TIM10_REMAP field."]
    #[inline] pub fn set_tim10_remap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="TIM11 remapping"]
    #[inline] pub fn tim11_remap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TIM11_REMAP != 0"]
    #[inline] pub fn test_tim11_remap(&self) -> bool {
        self.tim11_remap() != 0
    }

    #[doc="Sets the TIM11_REMAP field."]
    #[inline] pub fn set_tim11_remap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="TIM13 remapping"]
    #[inline] pub fn tim13_remap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TIM13_REMAP != 0"]
    #[inline] pub fn test_tim13_remap(&self) -> bool {
        self.tim13_remap() != 0
    }

    #[doc="Sets the TIM13_REMAP field."]
    #[inline] pub fn set_tim13_remap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="TIM14 remapping"]
    #[inline] pub fn tim14_remap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TIM14_REMAP != 0"]
    #[inline] pub fn test_tim14_remap(&self) -> bool {
        self.tim14_remap() != 0
    }

    #[doc="Sets the TIM14_REMAP field."]
    #[inline] pub fn set_tim14_remap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="NADV connect/disconnect"]
    #[inline] pub fn fsmc_nadv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if FSMC_NADV != 0"]
    #[inline] pub fn test_fsmc_nadv(&self) -> bool {
        self.fsmc_nadv() != 0
    }

    #[doc="Sets the FSMC_NADV field."]
    #[inline] pub fn set_fsmc_nadv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Mapr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Mapr2(other)
    }
}

impl ::core::fmt::Display for Mapr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mapr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tim9_remap() != 0 { try!(write!(f, " tim9_remap"))}
        if self.tim10_remap() != 0 { try!(write!(f, " tim10_remap"))}
        if self.tim11_remap() != 0 { try!(write!(f, " tim11_remap"))}
        if self.tim13_remap() != 0 { try!(write!(f, " tim13_remap"))}
        if self.tim14_remap() != 0 { try!(write!(f, " tim14_remap"))}
        if self.fsmc_nadv() != 0 { try!(write!(f, " fsmc_nadv"))}
        try!(write!(f, "]"));
        Ok(())
    }
}
