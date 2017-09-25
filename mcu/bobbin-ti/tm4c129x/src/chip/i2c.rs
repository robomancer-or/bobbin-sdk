#[allow(unused_imports)] use bobbin_common::*;

periph!( I2C0, I2c0, _I2C0, I2cPeriph, 0x40020000);
periph!( I2C1, I2c1, _I2C1, I2cPeriph, 0x40021000);
periph!( I2C2, I2c2, _I2C2, I2cPeriph, 0x40022000);
periph!( I2C3, I2c3, _I2C3, I2cPeriph, 0x40023000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="I2C Peripheral"]
pub struct I2cPeriph(pub usize); 

impl super::sig::Signal<super::sig::I2c0scl> for I2c0 {}
impl super::sig::SignalI2cScl<super::sig::I2c0scl> for I2c0 {}
impl super::sig::Signal<super::sig::I2c0sda> for I2c0 {}
impl super::sig::SignalI2cSda<super::sig::I2c0sda> for I2c0 {}

impl super::sig::Signal<super::sig::I2c1scl> for I2c1 {}
impl super::sig::SignalI2cScl<super::sig::I2c1scl> for I2c1 {}
impl super::sig::Signal<super::sig::I2c1sda> for I2c1 {}
impl super::sig::SignalI2cSda<super::sig::I2c1sda> for I2c1 {}

impl super::sig::Signal<super::sig::I2c2scl> for I2c2 {}
impl super::sig::SignalI2cScl<super::sig::I2c2scl> for I2c2 {}
impl super::sig::Signal<super::sig::I2c2sda> for I2c2 {}
impl super::sig::SignalI2cSda<super::sig::I2c2sda> for I2c2 {}

impl super::sig::Signal<super::sig::I2c3scl> for I2c3 {}
impl super::sig::SignalI2cScl<super::sig::I2c3scl> for I2c3 {}
impl super::sig::Signal<super::sig::I2c3sda> for I2c3 {}
impl super::sig::SignalI2cSda<super::sig::I2c3sda> for I2c3 {}


impl I2cPeriph {
    #[doc="Get the *mut pointer for the FIFODATA register."]
    #[inline] pub fn fifodata_mut(&self) -> *mut Fifodata { 
        (self.0 + 0xf00) as *mut Fifodata
    }

    #[doc="Get the *const pointer for the FIFODATA register."]
    #[inline] pub fn fifodata_ptr(&self) -> *const Fifodata { 
           self.fifodata_mut()
    }

    #[doc="Read the FIFODATA register."]
    #[inline] pub fn fifodata(&self) -> Fifodata { 
        unsafe {
            read_volatile(self.fifodata_ptr())
        }
    }

    #[doc="Write the FIFODATA register."]
    #[inline] pub fn set_fifodata<F: FnOnce(Fifodata) -> Fifodata>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifodata_mut(), f(Fifodata(0)));
        }
        self
    }

    #[doc="Modify the FIFODATA register."]
    #[inline] pub fn with_fifodata<F: FnOnce(Fifodata) -> Fifodata>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifodata_mut(), f(self.fifodata()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FIFOCTL register."]
    #[inline] pub fn fifoctl_mut(&self) -> *mut Fifoctl { 
        (self.0 + 0xf04) as *mut Fifoctl
    }

    #[doc="Get the *const pointer for the FIFOCTL register."]
    #[inline] pub fn fifoctl_ptr(&self) -> *const Fifoctl { 
           self.fifoctl_mut()
    }

    #[doc="Read the FIFOCTL register."]
    #[inline] pub fn fifoctl(&self) -> Fifoctl { 
        unsafe {
            read_volatile(self.fifoctl_ptr())
        }
    }

    #[doc="Write the FIFOCTL register."]
    #[inline] pub fn set_fifoctl<F: FnOnce(Fifoctl) -> Fifoctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifoctl_mut(), f(Fifoctl(0)));
        }
        self
    }

    #[doc="Modify the FIFOCTL register."]
    #[inline] pub fn with_fifoctl<F: FnOnce(Fifoctl) -> Fifoctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifoctl_mut(), f(self.fifoctl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FIFOSTATUS register."]
    #[inline] pub fn fifostatus_mut(&self) -> *mut Fifostatus { 
        (self.0 + 0xf08) as *mut Fifostatus
    }

    #[doc="Get the *const pointer for the FIFOSTATUS register."]
    #[inline] pub fn fifostatus_ptr(&self) -> *const Fifostatus { 
           self.fifostatus_mut()
    }

    #[doc="Read the FIFOSTATUS register."]
    #[inline] pub fn fifostatus(&self) -> Fifostatus { 
        unsafe {
            read_volatile(self.fifostatus_ptr())
        }
    }

    #[doc="Write the FIFOSTATUS register."]
    #[inline] pub fn set_fifostatus<F: FnOnce(Fifostatus) -> Fifostatus>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifostatus_mut(), f(Fifostatus(0)));
        }
        self
    }

    #[doc="Modify the FIFOSTATUS register."]
    #[inline] pub fn with_fifostatus<F: FnOnce(Fifostatus) -> Fifostatus>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fifostatus_mut(), f(self.fifostatus()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PP register."]
    #[inline] pub fn pp_mut(&self) -> *mut Pp { 
        (self.0 + 0xfc0) as *mut Pp
    }

    #[doc="Get the *const pointer for the PP register."]
    #[inline] pub fn pp_ptr(&self) -> *const Pp { 
           self.pp_mut()
    }

    #[doc="Read the PP register."]
    #[inline] pub fn pp(&self) -> Pp { 
        unsafe {
            read_volatile(self.pp_ptr())
        }
    }

    #[doc="Write the PP register."]
    #[inline] pub fn set_pp<F: FnOnce(Pp) -> Pp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pp_mut(), f(Pp(0)));
        }
        self
    }

    #[doc="Modify the PP register."]
    #[inline] pub fn with_pp<F: FnOnce(Pp) -> Pp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pp_mut(), f(self.pp()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PC register."]
    #[inline] pub fn pc_mut(&self) -> *mut Pc { 
        (self.0 + 0xfc4) as *mut Pc
    }

    #[doc="Get the *const pointer for the PC register."]
    #[inline] pub fn pc_ptr(&self) -> *const Pc { 
           self.pc_mut()
    }

    #[doc="Read the PC register."]
    #[inline] pub fn pc(&self) -> Pc { 
        unsafe {
            read_volatile(self.pc_ptr())
        }
    }

    #[doc="Write the PC register."]
    #[inline] pub fn set_pc<F: FnOnce(Pc) -> Pc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pc_mut(), f(Pc(0)));
        }
        self
    }

    #[doc="Modify the PC register."]
    #[inline] pub fn with_pc<F: FnOnce(Pc) -> Pc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pc_mut(), f(self.pc()));
        }
        self
    }

}

#[doc="I2C FIFO Data"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fifodata(pub u32);
impl Fifodata {
    #[doc="I2C TX FIFO Write Data Byte"]
    #[inline] pub fn data(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fifodata {
    #[inline]
    fn from(other: u32) -> Self {
         Fifodata(other)
    }
}

impl ::core::fmt::Display for Fifodata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fifodata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C FIFO Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fifoctl(pub u32);
impl Fifoctl {
    #[doc="TX FIFO Trigger"]
    #[inline] pub fn txtrig(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if TXTRIG != 0"]
    #[inline] pub fn test_txtrig(&self) -> bool {
        self.txtrig() != 0
    }

    #[doc="Sets the TXTRIG field."]
    #[inline] pub fn set_txtrig<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DMA TX Channel Enable"]
    #[inline] pub fn dmatxena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if DMATXENA != 0"]
    #[inline] pub fn test_dmatxena(&self) -> bool {
        self.dmatxena() != 0
    }

    #[doc="Sets the DMATXENA field."]
    #[inline] pub fn set_dmatxena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="TX FIFO Flush"]
    #[inline] pub fn txflush(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if TXFLUSH != 0"]
    #[inline] pub fn test_txflush(&self) -> bool {
        self.txflush() != 0
    }

    #[doc="Sets the TXFLUSH field."]
    #[inline] pub fn set_txflush<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="TX Control Assignment"]
    #[inline] pub fn txasgnmt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if TXASGNMT != 0"]
    #[inline] pub fn test_txasgnmt(&self) -> bool {
        self.txasgnmt() != 0
    }

    #[doc="Sets the TXASGNMT field."]
    #[inline] pub fn set_txasgnmt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="RX FIFO Trigger"]
    #[inline] pub fn rxtrig(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if RXTRIG != 0"]
    #[inline] pub fn test_rxtrig(&self) -> bool {
        self.rxtrig() != 0
    }

    #[doc="Sets the RXTRIG field."]
    #[inline] pub fn set_rxtrig<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DMA RX Channel Enable"]
    #[inline] pub fn dmarxena(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if DMARXENA != 0"]
    #[inline] pub fn test_dmarxena(&self) -> bool {
        self.dmarxena() != 0
    }

    #[doc="Sets the DMARXENA field."]
    #[inline] pub fn set_dmarxena<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="RX FIFO Flush"]
    #[inline] pub fn rxflush(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if RXFLUSH != 0"]
    #[inline] pub fn test_rxflush(&self) -> bool {
        self.rxflush() != 0
    }

    #[doc="Sets the RXFLUSH field."]
    #[inline] pub fn set_rxflush<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="RX Control Assignment"]
    #[inline] pub fn rxasgnmt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if RXASGNMT != 0"]
    #[inline] pub fn test_rxasgnmt(&self) -> bool {
        self.rxasgnmt() != 0
    }

    #[doc="Sets the RXASGNMT field."]
    #[inline] pub fn set_rxasgnmt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Fifoctl {
    #[inline]
    fn from(other: u32) -> Self {
         Fifoctl(other)
    }
}

impl ::core::fmt::Display for Fifoctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fifoctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txtrig() != 0 { try!(write!(f, " txtrig=0x{:x}", self.txtrig()))}
        if self.dmatxena() != 0 { try!(write!(f, " dmatxena"))}
        if self.txflush() != 0 { try!(write!(f, " txflush"))}
        if self.txasgnmt() != 0 { try!(write!(f, " txasgnmt"))}
        if self.rxtrig() != 0 { try!(write!(f, " rxtrig=0x{:x}", self.rxtrig()))}
        if self.dmarxena() != 0 { try!(write!(f, " dmarxena"))}
        if self.rxflush() != 0 { try!(write!(f, " rxflush"))}
        if self.rxasgnmt() != 0 { try!(write!(f, " rxasgnmt"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C FIFO Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fifostatus(pub u32);
impl Fifostatus {
    #[doc="TX FIFO Empty"]
    #[inline] pub fn txfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TXFE != 0"]
    #[inline] pub fn test_txfe(&self) -> bool {
        self.txfe() != 0
    }

    #[doc="Sets the TXFE field."]
    #[inline] pub fn set_txfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="TX FIFO Full"]
    #[inline] pub fn txff(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXFF != 0"]
    #[inline] pub fn test_txff(&self) -> bool {
        self.txff() != 0
    }

    #[doc="Sets the TXFF field."]
    #[inline] pub fn set_txff<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TX FIFO Below Trigger Level"]
    #[inline] pub fn txblwtrig(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TXBLWTRIG != 0"]
    #[inline] pub fn test_txblwtrig(&self) -> bool {
        self.txblwtrig() != 0
    }

    #[doc="Sets the TXBLWTRIG field."]
    #[inline] pub fn set_txblwtrig<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="RX FIFO Empty"]
    #[inline] pub fn rxfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if RXFE != 0"]
    #[inline] pub fn test_rxfe(&self) -> bool {
        self.rxfe() != 0
    }

    #[doc="Sets the RXFE field."]
    #[inline] pub fn set_rxfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="RX FIFO Full"]
    #[inline] pub fn rxff(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if RXFF != 0"]
    #[inline] pub fn test_rxff(&self) -> bool {
        self.rxff() != 0
    }

    #[doc="Sets the RXFF field."]
    #[inline] pub fn set_rxff<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="RX FIFO Above Trigger Level"]
    #[inline] pub fn rxabvtrig(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if RXABVTRIG != 0"]
    #[inline] pub fn test_rxabvtrig(&self) -> bool {
        self.rxabvtrig() != 0
    }

    #[doc="Sets the RXABVTRIG field."]
    #[inline] pub fn set_rxabvtrig<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

}

impl From<u32> for Fifostatus {
    #[inline]
    fn from(other: u32) -> Self {
         Fifostatus(other)
    }
}

impl ::core::fmt::Display for Fifostatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fifostatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txfe() != 0 { try!(write!(f, " txfe"))}
        if self.txff() != 0 { try!(write!(f, " txff"))}
        if self.txblwtrig() != 0 { try!(write!(f, " txblwtrig"))}
        if self.rxfe() != 0 { try!(write!(f, " rxfe"))}
        if self.rxff() != 0 { try!(write!(f, " rxff"))}
        if self.rxabvtrig() != 0 { try!(write!(f, " rxabvtrig"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Peripheral Properties"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pp(pub u32);
impl Pp {
    #[doc="High-Speed Capable"]
    #[inline] pub fn hs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HS != 0"]
    #[inline] pub fn test_hs(&self) -> bool {
        self.hs() != 0
    }

    #[doc="Sets the HS field."]
    #[inline] pub fn set_hs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pp {
    #[inline]
    fn from(other: u32) -> Self {
         Pp(other)
    }
}

impl ::core::fmt::Display for Pp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hs() != 0 { try!(write!(f, " hs"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Peripheral Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pc(pub u32);
impl Pc {
    #[doc="High-Speed Capable"]
    #[inline] pub fn hs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HS != 0"]
    #[inline] pub fn test_hs(&self) -> bool {
        self.hs() != 0
    }

    #[doc="Sets the HS field."]
    #[inline] pub fn set_hs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pc {
    #[inline]
    fn from(other: u32) -> Self {
         Pc(other)
    }
}

impl ::core::fmt::Display for Pc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hs() != 0 { try!(write!(f, " hs"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

impl I2cPeriph {
    #[inline] pub fn master(&self) -> master::Master {
        master::Master(self.0 + 0x0)
    }
    #[inline] pub fn slave(&self) -> slave::Slave {
        slave::Slave(self.0 + 0x0)
    }
}

pub mod master {
    #[allow(unused_imports)] use bobbin_common::*;
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Master(pub usize);
impl Master {
    #[doc="Get the *mut pointer for the MSA register."]
    #[inline] pub fn msa_mut(&self) -> *mut Msa { 
        (self.0 + 0x0) as *mut Msa
    }

    #[doc="Get the *const pointer for the MSA register."]
    #[inline] pub fn msa_ptr(&self) -> *const Msa { 
           self.msa_mut()
    }

    #[doc="Read the MSA register."]
    #[inline] pub fn msa(&self) -> Msa { 
        unsafe {
            read_volatile(self.msa_ptr())
        }
    }

    #[doc="Write the MSA register."]
    #[inline] pub fn set_msa<F: FnOnce(Msa) -> Msa>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.msa_mut(), f(Msa(0)));
        }
        self
    }

    #[doc="Modify the MSA register."]
    #[inline] pub fn with_msa<F: FnOnce(Msa) -> Msa>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.msa_mut(), f(self.msa()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MCS_WRITE register."]
    #[inline] pub fn mcs_write_mut(&self) -> *mut McsWrite { 
        (self.0 + 0x4) as *mut McsWrite
    }

    #[doc="Get the *const pointer for the MCS_WRITE register."]
    #[inline] pub fn mcs_write_ptr(&self) -> *const McsWrite { 
           self.mcs_write_mut()
    }

    #[doc="Write the MCS_WRITE register."]
    #[inline] pub fn set_mcs_write<F: FnOnce(McsWrite) -> McsWrite>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mcs_write_mut(), f(McsWrite(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the MCS_READ register."]
    #[inline] pub fn mcs_read_mut(&self) -> *mut McsRead { 
        (self.0 + 0x4) as *mut McsRead
    }

    #[doc="Get the *const pointer for the MCS_READ register."]
    #[inline] pub fn mcs_read_ptr(&self) -> *const McsRead { 
           self.mcs_read_mut()
    }

    #[doc="Read the MCS_READ register."]
    #[inline] pub fn mcs_read(&self) -> McsRead { 
        unsafe {
            read_volatile(self.mcs_read_ptr())
        }
    }

    #[doc="Get the *mut pointer for the MDR register."]
    #[inline] pub fn mdr_mut(&self) -> *mut Mdr { 
        (self.0 + 0x8) as *mut Mdr
    }

    #[doc="Get the *const pointer for the MDR register."]
    #[inline] pub fn mdr_ptr(&self) -> *const Mdr { 
           self.mdr_mut()
    }

    #[doc="Read the MDR register."]
    #[inline] pub fn mdr(&self) -> Mdr { 
        unsafe {
            read_volatile(self.mdr_ptr())
        }
    }

    #[doc="Write the MDR register."]
    #[inline] pub fn set_mdr<F: FnOnce(Mdr) -> Mdr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mdr_mut(), f(Mdr(0)));
        }
        self
    }

    #[doc="Modify the MDR register."]
    #[inline] pub fn with_mdr<F: FnOnce(Mdr) -> Mdr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mdr_mut(), f(self.mdr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MTPR register."]
    #[inline] pub fn mtpr_mut(&self) -> *mut Mtpr { 
        (self.0 + 0xc) as *mut Mtpr
    }

    #[doc="Get the *const pointer for the MTPR register."]
    #[inline] pub fn mtpr_ptr(&self) -> *const Mtpr { 
           self.mtpr_mut()
    }

    #[doc="Read the MTPR register."]
    #[inline] pub fn mtpr(&self) -> Mtpr { 
        unsafe {
            read_volatile(self.mtpr_ptr())
        }
    }

    #[doc="Write the MTPR register."]
    #[inline] pub fn set_mtpr<F: FnOnce(Mtpr) -> Mtpr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mtpr_mut(), f(Mtpr(0)));
        }
        self
    }

    #[doc="Modify the MTPR register."]
    #[inline] pub fn with_mtpr<F: FnOnce(Mtpr) -> Mtpr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mtpr_mut(), f(self.mtpr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MIMR register."]
    #[inline] pub fn mimr_mut(&self) -> *mut Mimr { 
        (self.0 + 0x10) as *mut Mimr
    }

    #[doc="Get the *const pointer for the MIMR register."]
    #[inline] pub fn mimr_ptr(&self) -> *const Mimr { 
           self.mimr_mut()
    }

    #[doc="Read the MIMR register."]
    #[inline] pub fn mimr(&self) -> Mimr { 
        unsafe {
            read_volatile(self.mimr_ptr())
        }
    }

    #[doc="Write the MIMR register."]
    #[inline] pub fn set_mimr<F: FnOnce(Mimr) -> Mimr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mimr_mut(), f(Mimr(0)));
        }
        self
    }

    #[doc="Modify the MIMR register."]
    #[inline] pub fn with_mimr<F: FnOnce(Mimr) -> Mimr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mimr_mut(), f(self.mimr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MRIS register."]
    #[inline] pub fn mris_mut(&self) -> *mut Mris { 
        (self.0 + 0x14) as *mut Mris
    }

    #[doc="Get the *const pointer for the MRIS register."]
    #[inline] pub fn mris_ptr(&self) -> *const Mris { 
           self.mris_mut()
    }

    #[doc="Read the MRIS register."]
    #[inline] pub fn mris(&self) -> Mris { 
        unsafe {
            read_volatile(self.mris_ptr())
        }
    }

    #[doc="Write the MRIS register."]
    #[inline] pub fn set_mris<F: FnOnce(Mris) -> Mris>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mris_mut(), f(Mris(0)));
        }
        self
    }

    #[doc="Modify the MRIS register."]
    #[inline] pub fn with_mris<F: FnOnce(Mris) -> Mris>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mris_mut(), f(self.mris()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MMIS register."]
    #[inline] pub fn mmis_mut(&self) -> *mut Mmis { 
        (self.0 + 0x18) as *mut Mmis
    }

    #[doc="Get the *const pointer for the MMIS register."]
    #[inline] pub fn mmis_ptr(&self) -> *const Mmis { 
           self.mmis_mut()
    }

    #[doc="Read the MMIS register."]
    #[inline] pub fn mmis(&self) -> Mmis { 
        unsafe {
            read_volatile(self.mmis_ptr())
        }
    }

    #[doc="Write the MMIS register."]
    #[inline] pub fn set_mmis<F: FnOnce(Mmis) -> Mmis>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mmis_mut(), f(Mmis(0)));
        }
        self
    }

    #[doc="Modify the MMIS register."]
    #[inline] pub fn with_mmis<F: FnOnce(Mmis) -> Mmis>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mmis_mut(), f(self.mmis()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MICR register."]
    #[inline] pub fn micr_mut(&self) -> *mut Micr { 
        (self.0 + 0x1c) as *mut Micr
    }

    #[doc="Get the *const pointer for the MICR register."]
    #[inline] pub fn micr_ptr(&self) -> *const Micr { 
           self.micr_mut()
    }

    #[doc="Write the MICR register."]
    #[inline] pub fn set_micr<F: FnOnce(Micr) -> Micr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.micr_mut(), f(Micr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the MCR register."]
    #[inline] pub fn mcr_mut(&self) -> *mut Mcr { 
        (self.0 + 0x20) as *mut Mcr
    }

    #[doc="Get the *const pointer for the MCR register."]
    #[inline] pub fn mcr_ptr(&self) -> *const Mcr { 
           self.mcr_mut()
    }

    #[doc="Read the MCR register."]
    #[inline] pub fn mcr(&self) -> Mcr { 
        unsafe {
            read_volatile(self.mcr_ptr())
        }
    }

    #[doc="Write the MCR register."]
    #[inline] pub fn set_mcr<F: FnOnce(Mcr) -> Mcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mcr_mut(), f(Mcr(0)));
        }
        self
    }

    #[doc="Modify the MCR register."]
    #[inline] pub fn with_mcr<F: FnOnce(Mcr) -> Mcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mcr_mut(), f(self.mcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MCLKOCNT register."]
    #[inline] pub fn mclkocnt_mut(&self) -> *mut Mclkocnt { 
        (self.0 + 0x24) as *mut Mclkocnt
    }

    #[doc="Get the *const pointer for the MCLKOCNT register."]
    #[inline] pub fn mclkocnt_ptr(&self) -> *const Mclkocnt { 
           self.mclkocnt_mut()
    }

    #[doc="Read the MCLKOCNT register."]
    #[inline] pub fn mclkocnt(&self) -> Mclkocnt { 
        unsafe {
            read_volatile(self.mclkocnt_ptr())
        }
    }

    #[doc="Write the MCLKOCNT register."]
    #[inline] pub fn set_mclkocnt<F: FnOnce(Mclkocnt) -> Mclkocnt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mclkocnt_mut(), f(Mclkocnt(0)));
        }
        self
    }

    #[doc="Modify the MCLKOCNT register."]
    #[inline] pub fn with_mclkocnt<F: FnOnce(Mclkocnt) -> Mclkocnt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mclkocnt_mut(), f(self.mclkocnt()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MBMON register."]
    #[inline] pub fn mbmon_mut(&self) -> *mut Mbmon { 
        (self.0 + 0x2c) as *mut Mbmon
    }

    #[doc="Get the *const pointer for the MBMON register."]
    #[inline] pub fn mbmon_ptr(&self) -> *const Mbmon { 
           self.mbmon_mut()
    }

    #[doc="Read the MBMON register."]
    #[inline] pub fn mbmon(&self) -> Mbmon { 
        unsafe {
            read_volatile(self.mbmon_ptr())
        }
    }

    #[doc="Write the MBMON register."]
    #[inline] pub fn set_mbmon<F: FnOnce(Mbmon) -> Mbmon>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mbmon_mut(), f(Mbmon(0)));
        }
        self
    }

    #[doc="Modify the MBMON register."]
    #[inline] pub fn with_mbmon<F: FnOnce(Mbmon) -> Mbmon>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mbmon_mut(), f(self.mbmon()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MBLEN register."]
    #[inline] pub fn mblen_mut(&self) -> *mut Mblen { 
        (self.0 + 0x30) as *mut Mblen
    }

    #[doc="Get the *const pointer for the MBLEN register."]
    #[inline] pub fn mblen_ptr(&self) -> *const Mblen { 
           self.mblen_mut()
    }

    #[doc="Read the MBLEN register."]
    #[inline] pub fn mblen(&self) -> Mblen { 
        unsafe {
            read_volatile(self.mblen_ptr())
        }
    }

    #[doc="Write the MBLEN register."]
    #[inline] pub fn set_mblen<F: FnOnce(Mblen) -> Mblen>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mblen_mut(), f(Mblen(0)));
        }
        self
    }

    #[doc="Modify the MBLEN register."]
    #[inline] pub fn with_mblen<F: FnOnce(Mblen) -> Mblen>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mblen_mut(), f(self.mblen()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MBCNT register."]
    #[inline] pub fn mbcnt_mut(&self) -> *mut Mbcnt { 
        (self.0 + 0x34) as *mut Mbcnt
    }

    #[doc="Get the *const pointer for the MBCNT register."]
    #[inline] pub fn mbcnt_ptr(&self) -> *const Mbcnt { 
           self.mbcnt_mut()
    }

    #[doc="Read the MBCNT register."]
    #[inline] pub fn mbcnt(&self) -> Mbcnt { 
        unsafe {
            read_volatile(self.mbcnt_ptr())
        }
    }

    #[doc="Write the MBCNT register."]
    #[inline] pub fn set_mbcnt<F: FnOnce(Mbcnt) -> Mbcnt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mbcnt_mut(), f(Mbcnt(0)));
        }
        self
    }

    #[doc="Modify the MBCNT register."]
    #[inline] pub fn with_mbcnt<F: FnOnce(Mbcnt) -> Mbcnt>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mbcnt_mut(), f(self.mbcnt()));
        }
        self
    }

}

#[doc="I2C Master Slave Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Msa(pub u32);
impl Msa {
    #[doc="Receive not send"]
    #[inline] pub fn rs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RS != 0"]
    #[inline] pub fn test_rs(&self) -> bool {
        self.rs() != 0
    }

    #[doc="Sets the RS field."]
    #[inline] pub fn set_rs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="I2C Slave Address"]
    #[inline] pub fn sa(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7f) as u8) } // [7:1]
    }

    #[doc="Returns true if SA != 0"]
    #[inline] pub fn test_sa(&self) -> bool {
        self.sa() != 0
    }

    #[doc="Sets the SA field."]
    #[inline] pub fn set_sa<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Msa {
    #[inline]
    fn from(other: u32) -> Self {
         Msa(other)
    }
}

impl ::core::fmt::Display for Msa {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Msa {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rs() != 0 { try!(write!(f, " rs"))}
        if self.sa() != 0 { try!(write!(f, " sa=0x{:x}", self.sa()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Control/Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct McsWrite(pub u32);
impl McsWrite {
    #[doc="I2C Master Enable"]
    #[inline] pub fn run(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RUN != 0"]
    #[inline] pub fn test_run(&self) -> bool {
        self.run() != 0
    }

    #[doc="Sets the RUN field."]
    #[inline] pub fn set_run<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Generate START"]
    #[inline] pub fn start(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if START != 0"]
    #[inline] pub fn test_start(&self) -> bool {
        self.start() != 0
    }

    #[doc="Sets the START field."]
    #[inline] pub fn set_start<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Generate STOP"]
    #[inline] pub fn stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if STOP != 0"]
    #[inline] pub fn test_stop(&self) -> bool {
        self.stop() != 0
    }

    #[doc="Sets the STOP field."]
    #[inline] pub fn set_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Data Acknowledge Enable"]
    #[inline] pub fn ack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ACK != 0"]
    #[inline] pub fn test_ack(&self) -> bool {
        self.ack() != 0
    }

    #[doc="Sets the ACK field."]
    #[inline] pub fn set_ack<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="High Speed"]
    #[inline] pub fn hs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if HS != 0"]
    #[inline] pub fn test_hs(&self) -> bool {
        self.hs() != 0
    }

    #[doc="Sets the HS field."]
    #[inline] pub fn set_hs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Quick Command"]
    #[inline] pub fn qcmd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if QCMD != 0"]
    #[inline] pub fn test_qcmd(&self) -> bool {
        self.qcmd() != 0
    }

    #[doc="Sets the QCMD field."]
    #[inline] pub fn set_qcmd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Burst Enable"]
    #[inline] pub fn burst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if BURST != 0"]
    #[inline] pub fn test_burst(&self) -> bool {
        self.burst() != 0
    }

    #[doc="Sets the BURST field."]
    #[inline] pub fn set_burst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for McsWrite {
    #[inline]
    fn from(other: u32) -> Self {
         McsWrite(other)
    }
}

impl ::core::fmt::Display for McsWrite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for McsWrite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.run() != 0 { try!(write!(f, " run"))}
        if self.start() != 0 { try!(write!(f, " start"))}
        if self.stop() != 0 { try!(write!(f, " stop"))}
        if self.ack() != 0 { try!(write!(f, " ack"))}
        if self.hs() != 0 { try!(write!(f, " hs"))}
        if self.qcmd() != 0 { try!(write!(f, " qcmd"))}
        if self.burst() != 0 { try!(write!(f, " burst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Control/Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct McsRead(pub u32);
impl McsRead {
    #[doc="I2C Busy"]
    #[inline] pub fn busy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BUSY != 0"]
    #[inline] pub fn test_busy(&self) -> bool {
        self.busy() != 0
    }

    #[doc="Sets the BUSY field."]
    #[inline] pub fn set_busy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Error"]
    #[inline] pub fn error(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ERROR != 0"]
    #[inline] pub fn test_error(&self) -> bool {
        self.error() != 0
    }

    #[doc="Sets the ERROR field."]
    #[inline] pub fn set_error<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Acknowledge Address"]
    #[inline] pub fn adrack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ADRACK != 0"]
    #[inline] pub fn test_adrack(&self) -> bool {
        self.adrack() != 0
    }

    #[doc="Sets the ADRACK field."]
    #[inline] pub fn set_adrack<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Acknowledge Data"]
    #[inline] pub fn datack(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DATACK != 0"]
    #[inline] pub fn test_datack(&self) -> bool {
        self.datack() != 0
    }

    #[doc="Sets the DATACK field."]
    #[inline] pub fn set_datack<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Arbitration Lost"]
    #[inline] pub fn arblst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ARBLST != 0"]
    #[inline] pub fn test_arblst(&self) -> bool {
        self.arblst() != 0
    }

    #[doc="Sets the ARBLST field."]
    #[inline] pub fn set_arblst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="I2C Idle"]
    #[inline] pub fn idle(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if IDLE != 0"]
    #[inline] pub fn test_idle(&self) -> bool {
        self.idle() != 0
    }

    #[doc="Sets the IDLE field."]
    #[inline] pub fn set_idle<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Bus Busy"]
    #[inline] pub fn busbsy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if BUSBSY != 0"]
    #[inline] pub fn test_busbsy(&self) -> bool {
        self.busbsy() != 0
    }

    #[doc="Sets the BUSBSY field."]
    #[inline] pub fn set_busbsy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Clock Timeout Error"]
    #[inline] pub fn clkto(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CLKTO != 0"]
    #[inline] pub fn test_clkto(&self) -> bool {
        self.clkto() != 0
    }

    #[doc="Sets the CLKTO field."]
    #[inline] pub fn set_clkto<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="DMA TX Active Status"]
    #[inline] pub fn actdmatx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if ACTDMATX != 0"]
    #[inline] pub fn test_actdmatx(&self) -> bool {
        self.actdmatx() != 0
    }

    #[doc="Sets the ACTDMATX field."]
    #[inline] pub fn set_actdmatx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="DMA RX Active Status"]
    #[inline] pub fn actdmarx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if ACTDMARX != 0"]
    #[inline] pub fn test_actdmarx(&self) -> bool {
        self.actdmarx() != 0
    }

    #[doc="Sets the ACTDMARX field."]
    #[inline] pub fn set_actdmarx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for McsRead {
    #[inline]
    fn from(other: u32) -> Self {
         McsRead(other)
    }
}

impl ::core::fmt::Display for McsRead {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for McsRead {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.busy() != 0 { try!(write!(f, " busy"))}
        if self.error() != 0 { try!(write!(f, " error"))}
        if self.adrack() != 0 { try!(write!(f, " adrack"))}
        if self.datack() != 0 { try!(write!(f, " datack"))}
        if self.arblst() != 0 { try!(write!(f, " arblst"))}
        if self.idle() != 0 { try!(write!(f, " idle"))}
        if self.busbsy() != 0 { try!(write!(f, " busbsy"))}
        if self.clkto() != 0 { try!(write!(f, " clkto"))}
        if self.actdmatx() != 0 { try!(write!(f, " actdmatx"))}
        if self.actdmarx() != 0 { try!(write!(f, " actdmarx"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Data"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mdr(pub u32);
impl Mdr {
    #[doc="This byte contains the data transferred during a transaction"]
    #[inline] pub fn data(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mdr {
    #[inline]
    fn from(other: u32) -> Self {
         Mdr(other)
    }
}

impl ::core::fmt::Display for Mdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Timer Period"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mtpr(pub u32);
impl Mtpr {
    #[doc="Timer Period"]
    #[inline] pub fn tpr(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if TPR != 0"]
    #[inline] pub fn test_tpr(&self) -> bool {
        self.tpr() != 0
    }

    #[doc="Sets the TPR field."]
    #[inline] pub fn set_tpr<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="High-Speed Enable"]
    #[inline] pub fn hs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if HS != 0"]
    #[inline] pub fn test_hs(&self) -> bool {
        self.hs() != 0
    }

    #[doc="Sets the HS field."]
    #[inline] pub fn set_hs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Glitch Suppression Pulse Width"]
    #[inline] pub fn pulsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if PULSEL != 0"]
    #[inline] pub fn test_pulsel(&self) -> bool {
        self.pulsel() != 0
    }

    #[doc="Sets the PULSEL field."]
    #[inline] pub fn set_pulsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Mtpr {
    #[inline]
    fn from(other: u32) -> Self {
         Mtpr(other)
    }
}

impl ::core::fmt::Display for Mtpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mtpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tpr() != 0 { try!(write!(f, " tpr=0x{:x}", self.tpr()))}
        if self.hs() != 0 { try!(write!(f, " hs"))}
        if self.pulsel() != 0 { try!(write!(f, " pulsel=0x{:x}", self.pulsel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Interrupt Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mimr(pub u32);
impl Mimr {
    #[doc="Master Interrupt Mask"]
    #[inline] pub fn im(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IM != 0"]
    #[inline] pub fn test_im(&self) -> bool {
        self.im() != 0
    }

    #[doc="Sets the IM field."]
    #[inline] pub fn set_im<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clock Timeout Interrupt Mask"]
    #[inline] pub fn clkim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CLKIM != 0"]
    #[inline] pub fn test_clkim(&self) -> bool {
        self.clkim() != 0
    }

    #[doc="Sets the CLKIM field."]
    #[inline] pub fn set_clkim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Receive DMA Interrupt Mask"]
    #[inline] pub fn dmarxim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DMARXIM != 0"]
    #[inline] pub fn test_dmarxim(&self) -> bool {
        self.dmarxim() != 0
    }

    #[doc="Sets the DMARXIM field."]
    #[inline] pub fn set_dmarxim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Transmit DMA Interrupt Mask"]
    #[inline] pub fn dmatxim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DMATXIM != 0"]
    #[inline] pub fn test_dmatxim(&self) -> bool {
        self.dmatxim() != 0
    }

    #[doc="Sets the DMATXIM field."]
    #[inline] pub fn set_dmatxim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Address/Data NACK Interrupt Mask"]
    #[inline] pub fn nackim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NACKIM != 0"]
    #[inline] pub fn test_nackim(&self) -> bool {
        self.nackim() != 0
    }

    #[doc="Sets the NACKIM field."]
    #[inline] pub fn set_nackim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="START Detection Interrupt Mask"]
    #[inline] pub fn startim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if STARTIM != 0"]
    #[inline] pub fn test_startim(&self) -> bool {
        self.startim() != 0
    }

    #[doc="Sets the STARTIM field."]
    #[inline] pub fn set_startim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="STOP Detection Interrupt Mask"]
    #[inline] pub fn stopim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if STOPIM != 0"]
    #[inline] pub fn test_stopim(&self) -> bool {
        self.stopim() != 0
    }

    #[doc="Sets the STOPIM field."]
    #[inline] pub fn set_stopim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Arbitration Lost Interrupt Mask"]
    #[inline] pub fn arblostim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ARBLOSTIM != 0"]
    #[inline] pub fn test_arblostim(&self) -> bool {
        self.arblostim() != 0
    }

    #[doc="Sets the ARBLOSTIM field."]
    #[inline] pub fn set_arblostim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Transmit FIFO Request Interrupt Mask"]
    #[inline] pub fn txim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TXIM != 0"]
    #[inline] pub fn test_txim(&self) -> bool {
        self.txim() != 0
    }

    #[doc="Sets the TXIM field."]
    #[inline] pub fn set_txim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Receive FIFO Request Interrupt Mask"]
    #[inline] pub fn rxim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RXIM != 0"]
    #[inline] pub fn test_rxim(&self) -> bool {
        self.rxim() != 0
    }

    #[doc="Sets the RXIM field."]
    #[inline] pub fn set_rxim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Transmit FIFO Empty Interrupt Mask"]
    #[inline] pub fn txfeim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if TXFEIM != 0"]
    #[inline] pub fn test_txfeim(&self) -> bool {
        self.txfeim() != 0
    }

    #[doc="Sets the TXFEIM field."]
    #[inline] pub fn set_txfeim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Receive FIFO Full Interrupt Mask"]
    #[inline] pub fn rxffim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if RXFFIM != 0"]
    #[inline] pub fn test_rxffim(&self) -> bool {
        self.rxffim() != 0
    }

    #[doc="Sets the RXFFIM field."]
    #[inline] pub fn set_rxffim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for Mimr {
    #[inline]
    fn from(other: u32) -> Self {
         Mimr(other)
    }
}

impl ::core::fmt::Display for Mimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mimr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.im() != 0 { try!(write!(f, " im"))}
        if self.clkim() != 0 { try!(write!(f, " clkim"))}
        if self.dmarxim() != 0 { try!(write!(f, " dmarxim"))}
        if self.dmatxim() != 0 { try!(write!(f, " dmatxim"))}
        if self.nackim() != 0 { try!(write!(f, " nackim"))}
        if self.startim() != 0 { try!(write!(f, " startim"))}
        if self.stopim() != 0 { try!(write!(f, " stopim"))}
        if self.arblostim() != 0 { try!(write!(f, " arblostim"))}
        if self.txim() != 0 { try!(write!(f, " txim"))}
        if self.rxim() != 0 { try!(write!(f, " rxim"))}
        if self.txfeim() != 0 { try!(write!(f, " txfeim"))}
        if self.rxffim() != 0 { try!(write!(f, " rxffim"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Raw Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mris(pub u32);
impl Mris {
    #[doc="Master Raw Interrupt Status"]
    #[inline] pub fn ris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RIS != 0"]
    #[inline] pub fn test_ris(&self) -> bool {
        self.ris() != 0
    }

    #[doc="Sets the RIS field."]
    #[inline] pub fn set_ris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clock Timeout Raw Interrupt Status"]
    #[inline] pub fn clkris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CLKRIS != 0"]
    #[inline] pub fn test_clkris(&self) -> bool {
        self.clkris() != 0
    }

    #[doc="Sets the CLKRIS field."]
    #[inline] pub fn set_clkris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Receive DMA Raw Interrupt Status"]
    #[inline] pub fn dmarxris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DMARXRIS != 0"]
    #[inline] pub fn test_dmarxris(&self) -> bool {
        self.dmarxris() != 0
    }

    #[doc="Sets the DMARXRIS field."]
    #[inline] pub fn set_dmarxris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Transmit DMA Raw Interrupt Status"]
    #[inline] pub fn dmatxris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DMATXRIS != 0"]
    #[inline] pub fn test_dmatxris(&self) -> bool {
        self.dmatxris() != 0
    }

    #[doc="Sets the DMATXRIS field."]
    #[inline] pub fn set_dmatxris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Address/Data NACK Raw Interrupt Status"]
    #[inline] pub fn nackris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NACKRIS != 0"]
    #[inline] pub fn test_nackris(&self) -> bool {
        self.nackris() != 0
    }

    #[doc="Sets the NACKRIS field."]
    #[inline] pub fn set_nackris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="START Detection Raw Interrupt Status"]
    #[inline] pub fn startris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if STARTRIS != 0"]
    #[inline] pub fn test_startris(&self) -> bool {
        self.startris() != 0
    }

    #[doc="Sets the STARTRIS field."]
    #[inline] pub fn set_startris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="STOP Detection Raw Interrupt Status"]
    #[inline] pub fn stopris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if STOPRIS != 0"]
    #[inline] pub fn test_stopris(&self) -> bool {
        self.stopris() != 0
    }

    #[doc="Sets the STOPRIS field."]
    #[inline] pub fn set_stopris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Arbitration Lost Raw Interrupt Status"]
    #[inline] pub fn arblostris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ARBLOSTRIS != 0"]
    #[inline] pub fn test_arblostris(&self) -> bool {
        self.arblostris() != 0
    }

    #[doc="Sets the ARBLOSTRIS field."]
    #[inline] pub fn set_arblostris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Transmit Request Raw Interrupt Status"]
    #[inline] pub fn txris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TXRIS != 0"]
    #[inline] pub fn test_txris(&self) -> bool {
        self.txris() != 0
    }

    #[doc="Sets the TXRIS field."]
    #[inline] pub fn set_txris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Receive FIFO Request Raw Interrupt Status"]
    #[inline] pub fn rxris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RXRIS != 0"]
    #[inline] pub fn test_rxris(&self) -> bool {
        self.rxris() != 0
    }

    #[doc="Sets the RXRIS field."]
    #[inline] pub fn set_rxris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Transmit FIFO Empty Raw Interrupt Status"]
    #[inline] pub fn txferis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if TXFERIS != 0"]
    #[inline] pub fn test_txferis(&self) -> bool {
        self.txferis() != 0
    }

    #[doc="Sets the TXFERIS field."]
    #[inline] pub fn set_txferis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Receive FIFO Full Raw Interrupt Status"]
    #[inline] pub fn rxffris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if RXFFRIS != 0"]
    #[inline] pub fn test_rxffris(&self) -> bool {
        self.rxffris() != 0
    }

    #[doc="Sets the RXFFRIS field."]
    #[inline] pub fn set_rxffris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for Mris {
    #[inline]
    fn from(other: u32) -> Self {
         Mris(other)
    }
}

impl ::core::fmt::Display for Mris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ris() != 0 { try!(write!(f, " ris"))}
        if self.clkris() != 0 { try!(write!(f, " clkris"))}
        if self.dmarxris() != 0 { try!(write!(f, " dmarxris"))}
        if self.dmatxris() != 0 { try!(write!(f, " dmatxris"))}
        if self.nackris() != 0 { try!(write!(f, " nackris"))}
        if self.startris() != 0 { try!(write!(f, " startris"))}
        if self.stopris() != 0 { try!(write!(f, " stopris"))}
        if self.arblostris() != 0 { try!(write!(f, " arblostris"))}
        if self.txris() != 0 { try!(write!(f, " txris"))}
        if self.rxris() != 0 { try!(write!(f, " rxris"))}
        if self.txferis() != 0 { try!(write!(f, " txferis"))}
        if self.rxffris() != 0 { try!(write!(f, " rxffris"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Masked Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mmis(pub u32);
impl Mmis {
    #[doc="Masked Interrupt Status"]
    #[inline] pub fn mis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MIS != 0"]
    #[inline] pub fn test_mis(&self) -> bool {
        self.mis() != 0
    }

    #[doc="Sets the MIS field."]
    #[inline] pub fn set_mis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clock Timeout Masked Interrupt Status"]
    #[inline] pub fn clkmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CLKMIS != 0"]
    #[inline] pub fn test_clkmis(&self) -> bool {
        self.clkmis() != 0
    }

    #[doc="Sets the CLKMIS field."]
    #[inline] pub fn set_clkmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Receive DMA Interrupt Status"]
    #[inline] pub fn dmarxmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DMARXMIS != 0"]
    #[inline] pub fn test_dmarxmis(&self) -> bool {
        self.dmarxmis() != 0
    }

    #[doc="Sets the DMARXMIS field."]
    #[inline] pub fn set_dmarxmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Transmit DMA Interrupt Status"]
    #[inline] pub fn dmatxmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DMATXMIS != 0"]
    #[inline] pub fn test_dmatxmis(&self) -> bool {
        self.dmatxmis() != 0
    }

    #[doc="Sets the DMATXMIS field."]
    #[inline] pub fn set_dmatxmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Address/Data NACK Interrupt Mask"]
    #[inline] pub fn nackmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NACKMIS != 0"]
    #[inline] pub fn test_nackmis(&self) -> bool {
        self.nackmis() != 0
    }

    #[doc="Sets the NACKMIS field."]
    #[inline] pub fn set_nackmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="START Detection Interrupt Mask"]
    #[inline] pub fn startmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if STARTMIS != 0"]
    #[inline] pub fn test_startmis(&self) -> bool {
        self.startmis() != 0
    }

    #[doc="Sets the STARTMIS field."]
    #[inline] pub fn set_startmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="STOP Detection Interrupt Mask"]
    #[inline] pub fn stopmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if STOPMIS != 0"]
    #[inline] pub fn test_stopmis(&self) -> bool {
        self.stopmis() != 0
    }

    #[doc="Sets the STOPMIS field."]
    #[inline] pub fn set_stopmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Arbitration Lost Interrupt Mask"]
    #[inline] pub fn arblostmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ARBLOSTMIS != 0"]
    #[inline] pub fn test_arblostmis(&self) -> bool {
        self.arblostmis() != 0
    }

    #[doc="Sets the ARBLOSTMIS field."]
    #[inline] pub fn set_arblostmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Transmit Request Interrupt Mask"]
    #[inline] pub fn txmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TXMIS != 0"]
    #[inline] pub fn test_txmis(&self) -> bool {
        self.txmis() != 0
    }

    #[doc="Sets the TXMIS field."]
    #[inline] pub fn set_txmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Receive FIFO Request Interrupt Mask"]
    #[inline] pub fn rxmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RXMIS != 0"]
    #[inline] pub fn test_rxmis(&self) -> bool {
        self.rxmis() != 0
    }

    #[doc="Sets the RXMIS field."]
    #[inline] pub fn set_rxmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Transmit FIFO Empty Interrupt Mask"]
    #[inline] pub fn txfemis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if TXFEMIS != 0"]
    #[inline] pub fn test_txfemis(&self) -> bool {
        self.txfemis() != 0
    }

    #[doc="Sets the TXFEMIS field."]
    #[inline] pub fn set_txfemis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Receive FIFO Full Interrupt Mask"]
    #[inline] pub fn rxffmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if RXFFMIS != 0"]
    #[inline] pub fn test_rxffmis(&self) -> bool {
        self.rxffmis() != 0
    }

    #[doc="Sets the RXFFMIS field."]
    #[inline] pub fn set_rxffmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for Mmis {
    #[inline]
    fn from(other: u32) -> Self {
         Mmis(other)
    }
}

impl ::core::fmt::Display for Mmis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mmis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mis() != 0 { try!(write!(f, " mis"))}
        if self.clkmis() != 0 { try!(write!(f, " clkmis"))}
        if self.dmarxmis() != 0 { try!(write!(f, " dmarxmis"))}
        if self.dmatxmis() != 0 { try!(write!(f, " dmatxmis"))}
        if self.nackmis() != 0 { try!(write!(f, " nackmis"))}
        if self.startmis() != 0 { try!(write!(f, " startmis"))}
        if self.stopmis() != 0 { try!(write!(f, " stopmis"))}
        if self.arblostmis() != 0 { try!(write!(f, " arblostmis"))}
        if self.txmis() != 0 { try!(write!(f, " txmis"))}
        if self.rxmis() != 0 { try!(write!(f, " rxmis"))}
        if self.txfemis() != 0 { try!(write!(f, " txfemis"))}
        if self.rxffmis() != 0 { try!(write!(f, " rxffmis"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Interrupt Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Micr(pub u32);
impl Micr {
    #[doc="Master Interrupt Clear"]
    #[inline] pub fn ic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IC != 0"]
    #[inline] pub fn test_ic(&self) -> bool {
        self.ic() != 0
    }

    #[doc="Sets the IC field."]
    #[inline] pub fn set_ic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clock Timeout Interrupt Clear"]
    #[inline] pub fn clkic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CLKIC != 0"]
    #[inline] pub fn test_clkic(&self) -> bool {
        self.clkic() != 0
    }

    #[doc="Sets the CLKIC field."]
    #[inline] pub fn set_clkic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Receive DMA Interrupt Clear"]
    #[inline] pub fn dmarxic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DMARXIC != 0"]
    #[inline] pub fn test_dmarxic(&self) -> bool {
        self.dmarxic() != 0
    }

    #[doc="Sets the DMARXIC field."]
    #[inline] pub fn set_dmarxic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Transmit DMA Interrupt Clear"]
    #[inline] pub fn dmatxic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DMATXIC != 0"]
    #[inline] pub fn test_dmatxic(&self) -> bool {
        self.dmatxic() != 0
    }

    #[doc="Sets the DMATXIC field."]
    #[inline] pub fn set_dmatxic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Address/Data NACK Interrupt Clear"]
    #[inline] pub fn nackic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NACKIC != 0"]
    #[inline] pub fn test_nackic(&self) -> bool {
        self.nackic() != 0
    }

    #[doc="Sets the NACKIC field."]
    #[inline] pub fn set_nackic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="START Detection Interrupt Clear"]
    #[inline] pub fn startic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if STARTIC != 0"]
    #[inline] pub fn test_startic(&self) -> bool {
        self.startic() != 0
    }

    #[doc="Sets the STARTIC field."]
    #[inline] pub fn set_startic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="STOP Detection Interrupt Clear"]
    #[inline] pub fn stopic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if STOPIC != 0"]
    #[inline] pub fn test_stopic(&self) -> bool {
        self.stopic() != 0
    }

    #[doc="Sets the STOPIC field."]
    #[inline] pub fn set_stopic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Arbitration Lost Interrupt Clear"]
    #[inline] pub fn arblostic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ARBLOSTIC != 0"]
    #[inline] pub fn test_arblostic(&self) -> bool {
        self.arblostic() != 0
    }

    #[doc="Sets the ARBLOSTIC field."]
    #[inline] pub fn set_arblostic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Transmit FIFO Request Interrupt Clear"]
    #[inline] pub fn txic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TXIC != 0"]
    #[inline] pub fn test_txic(&self) -> bool {
        self.txic() != 0
    }

    #[doc="Sets the TXIC field."]
    #[inline] pub fn set_txic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Receive FIFO Request Interrupt Clear"]
    #[inline] pub fn rxic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RXIC != 0"]
    #[inline] pub fn test_rxic(&self) -> bool {
        self.rxic() != 0
    }

    #[doc="Sets the RXIC field."]
    #[inline] pub fn set_rxic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Transmit FIFO Empty Interrupt Clear"]
    #[inline] pub fn txfeic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if TXFEIC != 0"]
    #[inline] pub fn test_txfeic(&self) -> bool {
        self.txfeic() != 0
    }

    #[doc="Sets the TXFEIC field."]
    #[inline] pub fn set_txfeic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Receive FIFO Full Interrupt Clear"]
    #[inline] pub fn rxffic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if RXFFIC != 0"]
    #[inline] pub fn test_rxffic(&self) -> bool {
        self.rxffic() != 0
    }

    #[doc="Sets the RXFFIC field."]
    #[inline] pub fn set_rxffic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for Micr {
    #[inline]
    fn from(other: u32) -> Self {
         Micr(other)
    }
}

impl ::core::fmt::Display for Micr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Micr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ic() != 0 { try!(write!(f, " ic"))}
        if self.clkic() != 0 { try!(write!(f, " clkic"))}
        if self.dmarxic() != 0 { try!(write!(f, " dmarxic"))}
        if self.dmatxic() != 0 { try!(write!(f, " dmatxic"))}
        if self.nackic() != 0 { try!(write!(f, " nackic"))}
        if self.startic() != 0 { try!(write!(f, " startic"))}
        if self.stopic() != 0 { try!(write!(f, " stopic"))}
        if self.arblostic() != 0 { try!(write!(f, " arblostic"))}
        if self.txic() != 0 { try!(write!(f, " txic"))}
        if self.rxic() != 0 { try!(write!(f, " rxic"))}
        if self.txfeic() != 0 { try!(write!(f, " txfeic"))}
        if self.rxffic() != 0 { try!(write!(f, " rxffic"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc="I2C Loopback"]
    #[inline] pub fn lpbk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LPBK != 0"]
    #[inline] pub fn test_lpbk(&self) -> bool {
        self.lpbk() != 0
    }

    #[doc="Sets the LPBK field."]
    #[inline] pub fn set_lpbk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="I2C Master Function Enable"]
    #[inline] pub fn mfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MFE != 0"]
    #[inline] pub fn test_mfe(&self) -> bool {
        self.mfe() != 0
    }

    #[doc="Sets the MFE field."]
    #[inline] pub fn set_mfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="I2C Slave Function Enable"]
    #[inline] pub fn sfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SFE != 0"]
    #[inline] pub fn test_sfe(&self) -> bool {
        self.sfe() != 0
    }

    #[doc="Sets the SFE field."]
    #[inline] pub fn set_sfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for Mcr {
    #[inline]
    fn from(other: u32) -> Self {
         Mcr(other)
    }
}

impl ::core::fmt::Display for Mcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lpbk() != 0 { try!(write!(f, " lpbk"))}
        if self.mfe() != 0 { try!(write!(f, " mfe"))}
        if self.sfe() != 0 { try!(write!(f, " sfe"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Clock Low Timeout Count"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mclkocnt(pub u32);
impl Mclkocnt {
    #[doc="I2C Master Count"]
    #[inline] pub fn cntl(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CNTL != 0"]
    #[inline] pub fn test_cntl(&self) -> bool {
        self.cntl() != 0
    }

    #[doc="Sets the CNTL field."]
    #[inline] pub fn set_cntl<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mclkocnt {
    #[inline]
    fn from(other: u32) -> Self {
         Mclkocnt(other)
    }
}

impl ::core::fmt::Display for Mclkocnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mclkocnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cntl() != 0 { try!(write!(f, " cntl=0x{:x}", self.cntl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Bus Monitor"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mbmon(pub u32);
impl Mbmon {
    #[doc="I2C SCL Status"]
    #[inline] pub fn scl(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SCL != 0"]
    #[inline] pub fn test_scl(&self) -> bool {
        self.scl() != 0
    }

    #[doc="Sets the SCL field."]
    #[inline] pub fn set_scl<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="I2C SDA Status"]
    #[inline] pub fn sda(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SDA != 0"]
    #[inline] pub fn test_sda(&self) -> bool {
        self.sda() != 0
    }

    #[doc="Sets the SDA field."]
    #[inline] pub fn set_sda<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Mbmon {
    #[inline]
    fn from(other: u32) -> Self {
         Mbmon(other)
    }
}

impl ::core::fmt::Display for Mbmon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mbmon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.scl() != 0 { try!(write!(f, " scl"))}
        if self.sda() != 0 { try!(write!(f, " sda"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Burst Length"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mblen(pub u32);
impl Mblen {
    #[doc="I2C Burst Length"]
    #[inline] pub fn cntl(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CNTL != 0"]
    #[inline] pub fn test_cntl(&self) -> bool {
        self.cntl() != 0
    }

    #[doc="Sets the CNTL field."]
    #[inline] pub fn set_cntl<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mblen {
    #[inline]
    fn from(other: u32) -> Self {
         Mblen(other)
    }
}

impl ::core::fmt::Display for Mblen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mblen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cntl() != 0 { try!(write!(f, " cntl=0x{:x}", self.cntl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Master Burst Count"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mbcnt(pub u32);
impl Mbcnt {
    #[doc="I2C Master Burst Count"]
    #[inline] pub fn cntl(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CNTL != 0"]
    #[inline] pub fn test_cntl(&self) -> bool {
        self.cntl() != 0
    }

    #[doc="Sets the CNTL field."]
    #[inline] pub fn set_cntl<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mbcnt {
    #[inline]
    fn from(other: u32) -> Self {
         Mbcnt(other)
    }
}

impl ::core::fmt::Display for Mbcnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mbcnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cntl() != 0 { try!(write!(f, " cntl=0x{:x}", self.cntl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

}
// End of master

pub mod slave {
    #[allow(unused_imports)] use bobbin_common::*;
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Slave(pub usize);
impl Slave {
    #[doc="Get the *mut pointer for the SOAR register."]
    #[inline] pub fn soar_mut(&self) -> *mut Soar { 
        (self.0 + 0x800) as *mut Soar
    }

    #[doc="Get the *const pointer for the SOAR register."]
    #[inline] pub fn soar_ptr(&self) -> *const Soar { 
           self.soar_mut()
    }

    #[doc="Read the SOAR register."]
    #[inline] pub fn soar(&self) -> Soar { 
        unsafe {
            read_volatile(self.soar_ptr())
        }
    }

    #[doc="Write the SOAR register."]
    #[inline] pub fn set_soar<F: FnOnce(Soar) -> Soar>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.soar_mut(), f(Soar(0)));
        }
        self
    }

    #[doc="Modify the SOAR register."]
    #[inline] pub fn with_soar<F: FnOnce(Soar) -> Soar>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.soar_mut(), f(self.soar()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SCSR_READ register."]
    #[inline] pub fn scsr_read_mut(&self) -> *mut ScsrRead { 
        (self.0 + 0x804) as *mut ScsrRead
    }

    #[doc="Get the *const pointer for the SCSR_READ register."]
    #[inline] pub fn scsr_read_ptr(&self) -> *const ScsrRead { 
           self.scsr_read_mut()
    }

    #[doc="Read the SCSR_READ register."]
    #[inline] pub fn scsr_read(&self) -> ScsrRead { 
        unsafe {
            read_volatile(self.scsr_read_ptr())
        }
    }

    #[doc="Write the SCSR_READ register."]
    #[inline] pub fn set_scsr_read<F: FnOnce(ScsrRead) -> ScsrRead>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scsr_read_mut(), f(ScsrRead(0)));
        }
        self
    }

    #[doc="Modify the SCSR_READ register."]
    #[inline] pub fn with_scsr_read<F: FnOnce(ScsrRead) -> ScsrRead>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scsr_read_mut(), f(self.scsr_read()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SCSR_WRITE register."]
    #[inline] pub fn scsr_write_mut(&self) -> *mut ScsrWrite { 
        (self.0 + 0x804) as *mut ScsrWrite
    }

    #[doc="Get the *const pointer for the SCSR_WRITE register."]
    #[inline] pub fn scsr_write_ptr(&self) -> *const ScsrWrite { 
           self.scsr_write_mut()
    }

    #[doc="Read the SCSR_WRITE register."]
    #[inline] pub fn scsr_write(&self) -> ScsrWrite { 
        unsafe {
            read_volatile(self.scsr_write_ptr())
        }
    }

    #[doc="Write the SCSR_WRITE register."]
    #[inline] pub fn set_scsr_write<F: FnOnce(ScsrWrite) -> ScsrWrite>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scsr_write_mut(), f(ScsrWrite(0)));
        }
        self
    }

    #[doc="Modify the SCSR_WRITE register."]
    #[inline] pub fn with_scsr_write<F: FnOnce(ScsrWrite) -> ScsrWrite>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scsr_write_mut(), f(self.scsr_write()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SDR register."]
    #[inline] pub fn sdr_mut(&self) -> *mut Sdr { 
        (self.0 + 0x808) as *mut Sdr
    }

    #[doc="Get the *const pointer for the SDR register."]
    #[inline] pub fn sdr_ptr(&self) -> *const Sdr { 
           self.sdr_mut()
    }

    #[doc="Read the SDR register."]
    #[inline] pub fn sdr(&self) -> Sdr { 
        unsafe {
            read_volatile(self.sdr_ptr())
        }
    }

    #[doc="Write the SDR register."]
    #[inline] pub fn set_sdr<F: FnOnce(Sdr) -> Sdr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sdr_mut(), f(Sdr(0)));
        }
        self
    }

    #[doc="Modify the SDR register."]
    #[inline] pub fn with_sdr<F: FnOnce(Sdr) -> Sdr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sdr_mut(), f(self.sdr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SIMR register."]
    #[inline] pub fn simr_mut(&self) -> *mut Simr { 
        (self.0 + 0x80c) as *mut Simr
    }

    #[doc="Get the *const pointer for the SIMR register."]
    #[inline] pub fn simr_ptr(&self) -> *const Simr { 
           self.simr_mut()
    }

    #[doc="Read the SIMR register."]
    #[inline] pub fn simr(&self) -> Simr { 
        unsafe {
            read_volatile(self.simr_ptr())
        }
    }

    #[doc="Write the SIMR register."]
    #[inline] pub fn set_simr<F: FnOnce(Simr) -> Simr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.simr_mut(), f(Simr(0)));
        }
        self
    }

    #[doc="Modify the SIMR register."]
    #[inline] pub fn with_simr<F: FnOnce(Simr) -> Simr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.simr_mut(), f(self.simr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SRIS register."]
    #[inline] pub fn sris_mut(&self) -> *mut Sris { 
        (self.0 + 0x810) as *mut Sris
    }

    #[doc="Get the *const pointer for the SRIS register."]
    #[inline] pub fn sris_ptr(&self) -> *const Sris { 
           self.sris_mut()
    }

    #[doc="Read the SRIS register."]
    #[inline] pub fn sris(&self) -> Sris { 
        unsafe {
            read_volatile(self.sris_ptr())
        }
    }

    #[doc="Write the SRIS register."]
    #[inline] pub fn set_sris<F: FnOnce(Sris) -> Sris>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sris_mut(), f(Sris(0)));
        }
        self
    }

    #[doc="Modify the SRIS register."]
    #[inline] pub fn with_sris<F: FnOnce(Sris) -> Sris>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sris_mut(), f(self.sris()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SMIS register."]
    #[inline] pub fn smis_mut(&self) -> *mut Smis { 
        (self.0 + 0x814) as *mut Smis
    }

    #[doc="Get the *const pointer for the SMIS register."]
    #[inline] pub fn smis_ptr(&self) -> *const Smis { 
           self.smis_mut()
    }

    #[doc="Read the SMIS register."]
    #[inline] pub fn smis(&self) -> Smis { 
        unsafe {
            read_volatile(self.smis_ptr())
        }
    }

    #[doc="Write the SMIS register."]
    #[inline] pub fn set_smis<F: FnOnce(Smis) -> Smis>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.smis_mut(), f(Smis(0)));
        }
        self
    }

    #[doc="Modify the SMIS register."]
    #[inline] pub fn with_smis<F: FnOnce(Smis) -> Smis>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.smis_mut(), f(self.smis()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SICR register."]
    #[inline] pub fn sicr_mut(&self) -> *mut Sicr { 
        (self.0 + 0x818) as *mut Sicr
    }

    #[doc="Get the *const pointer for the SICR register."]
    #[inline] pub fn sicr_ptr(&self) -> *const Sicr { 
           self.sicr_mut()
    }

    #[doc="Write the SICR register."]
    #[inline] pub fn set_sicr<F: FnOnce(Sicr) -> Sicr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sicr_mut(), f(Sicr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the SOAR2 register."]
    #[inline] pub fn soar2_mut(&self) -> *mut Soar2 { 
        (self.0 + 0x81c) as *mut Soar2
    }

    #[doc="Get the *const pointer for the SOAR2 register."]
    #[inline] pub fn soar2_ptr(&self) -> *const Soar2 { 
           self.soar2_mut()
    }

    #[doc="Read the SOAR2 register."]
    #[inline] pub fn soar2(&self) -> Soar2 { 
        unsafe {
            read_volatile(self.soar2_ptr())
        }
    }

    #[doc="Write the SOAR2 register."]
    #[inline] pub fn set_soar2<F: FnOnce(Soar2) -> Soar2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.soar2_mut(), f(Soar2(0)));
        }
        self
    }

    #[doc="Modify the SOAR2 register."]
    #[inline] pub fn with_soar2<F: FnOnce(Soar2) -> Soar2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.soar2_mut(), f(self.soar2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SACKCTL register."]
    #[inline] pub fn sackctl_mut(&self) -> *mut Sackctl { 
        (self.0 + 0x820) as *mut Sackctl
    }

    #[doc="Get the *const pointer for the SACKCTL register."]
    #[inline] pub fn sackctl_ptr(&self) -> *const Sackctl { 
           self.sackctl_mut()
    }

    #[doc="Read the SACKCTL register."]
    #[inline] pub fn sackctl(&self) -> Sackctl { 
        unsafe {
            read_volatile(self.sackctl_ptr())
        }
    }

    #[doc="Write the SACKCTL register."]
    #[inline] pub fn set_sackctl<F: FnOnce(Sackctl) -> Sackctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sackctl_mut(), f(Sackctl(0)));
        }
        self
    }

    #[doc="Modify the SACKCTL register."]
    #[inline] pub fn with_sackctl<F: FnOnce(Sackctl) -> Sackctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sackctl_mut(), f(self.sackctl()));
        }
        self
    }

}

#[doc="I2C Slave Own Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Soar(pub u32);
impl Soar {
    #[doc="I2C Slave Own Address"]
    #[inline] pub fn oar(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if OAR != 0"]
    #[inline] pub fn test_oar(&self) -> bool {
        self.oar() != 0
    }

    #[doc="Sets the OAR field."]
    #[inline] pub fn set_oar<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Soar {
    #[inline]
    fn from(other: u32) -> Self {
         Soar(other)
    }
}

impl ::core::fmt::Display for Soar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Soar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.oar() != 0 { try!(write!(f, " oar=0x{:x}", self.oar()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Slave Control/Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct ScsrRead(pub u32);
impl ScsrRead {
    #[doc="Receive Request"]
    #[inline] pub fn rreq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RREQ != 0"]
    #[inline] pub fn test_rreq(&self) -> bool {
        self.rreq() != 0
    }

    #[doc="Sets the RREQ field."]
    #[inline] pub fn set_rreq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="TX FIFO Enable"]
    #[inline] pub fn txfifo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXFIFO != 0"]
    #[inline] pub fn test_txfifo(&self) -> bool {
        self.txfifo() != 0
    }

    #[doc="Sets the TXFIFO field."]
    #[inline] pub fn set_txfifo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="First Byte Received"]
    #[inline] pub fn fbr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FBR != 0"]
    #[inline] pub fn test_fbr(&self) -> bool {
        self.fbr() != 0
    }

    #[doc="Sets the FBR field."]
    #[inline] pub fn set_fbr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="OAR2 Address Matched"]
    #[inline] pub fn oar2sel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if OAR2SEL != 0"]
    #[inline] pub fn test_oar2sel(&self) -> bool {
        self.oar2sel() != 0
    }

    #[doc="Sets the OAR2SEL field."]
    #[inline] pub fn set_oar2sel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Quick Command Status"]
    #[inline] pub fn qcmdst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if QCMDST != 0"]
    #[inline] pub fn test_qcmdst(&self) -> bool {
        self.qcmdst() != 0
    }

    #[doc="Sets the QCMDST field."]
    #[inline] pub fn set_qcmdst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Quick Command Read / Write"]
    #[inline] pub fn qcmdrw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if QCMDRW != 0"]
    #[inline] pub fn test_qcmdrw(&self) -> bool {
        self.qcmdrw() != 0
    }

    #[doc="Sets the QCMDRW field."]
    #[inline] pub fn set_qcmdrw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="DMA TX Active Status"]
    #[inline] pub fn actdmatx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if ACTDMATX != 0"]
    #[inline] pub fn test_actdmatx(&self) -> bool {
        self.actdmatx() != 0
    }

    #[doc="Sets the ACTDMATX field."]
    #[inline] pub fn set_actdmatx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="DMA RX Active Status"]
    #[inline] pub fn actdmarx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if ACTDMARX != 0"]
    #[inline] pub fn test_actdmarx(&self) -> bool {
        self.actdmarx() != 0
    }

    #[doc="Sets the ACTDMARX field."]
    #[inline] pub fn set_actdmarx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for ScsrRead {
    #[inline]
    fn from(other: u32) -> Self {
         ScsrRead(other)
    }
}

impl ::core::fmt::Display for ScsrRead {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for ScsrRead {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rreq() != 0 { try!(write!(f, " rreq"))}
        if self.txfifo() != 0 { try!(write!(f, " txfifo"))}
        if self.fbr() != 0 { try!(write!(f, " fbr"))}
        if self.oar2sel() != 0 { try!(write!(f, " oar2sel"))}
        if self.qcmdst() != 0 { try!(write!(f, " qcmdst"))}
        if self.qcmdrw() != 0 { try!(write!(f, " qcmdrw"))}
        if self.actdmatx() != 0 { try!(write!(f, " actdmatx"))}
        if self.actdmarx() != 0 { try!(write!(f, " actdmarx"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Slave Control/Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct ScsrWrite(pub u32);
impl ScsrWrite {
    #[doc="Device Active"]
    #[inline] pub fn da(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DA != 0"]
    #[inline] pub fn test_da(&self) -> bool {
        self.da() != 0
    }

    #[doc="Sets the DA field."]
    #[inline] pub fn set_da<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit Request"]
    #[inline] pub fn treq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TREQ != 0"]
    #[inline] pub fn test_treq(&self) -> bool {
        self.treq() != 0
    }

    #[doc="Sets the TREQ field."]
    #[inline] pub fn set_treq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="RX FIFO Enable"]
    #[inline] pub fn rxfifo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RXFIFO != 0"]
    #[inline] pub fn test_rxfifo(&self) -> bool {
        self.rxfifo() != 0
    }

    #[doc="Sets the RXFIFO field."]
    #[inline] pub fn set_rxfifo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u32> for ScsrWrite {
    #[inline]
    fn from(other: u32) -> Self {
         ScsrWrite(other)
    }
}

impl ::core::fmt::Display for ScsrWrite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for ScsrWrite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.da() != 0 { try!(write!(f, " da"))}
        if self.treq() != 0 { try!(write!(f, " treq"))}
        if self.rxfifo() != 0 { try!(write!(f, " rxfifo"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Slave Data"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sdr(pub u32);
impl Sdr {
    #[doc="Data for Transfer"]
    #[inline] pub fn data(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sdr {
    #[inline]
    fn from(other: u32) -> Self {
         Sdr(other)
    }
}

impl ::core::fmt::Display for Sdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Slave Interrupt Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Simr(pub u32);
impl Simr {
    #[doc="Data Interrupt Mask"]
    #[inline] pub fn dataim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DATAIM != 0"]
    #[inline] pub fn test_dataim(&self) -> bool {
        self.dataim() != 0
    }

    #[doc="Sets the DATAIM field."]
    #[inline] pub fn set_dataim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Start Condition Interrupt Mask"]
    #[inline] pub fn startim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if STARTIM != 0"]
    #[inline] pub fn test_startim(&self) -> bool {
        self.startim() != 0
    }

    #[doc="Sets the STARTIM field."]
    #[inline] pub fn set_startim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Stop Condition Interrupt Mask"]
    #[inline] pub fn stopim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if STOPIM != 0"]
    #[inline] pub fn test_stopim(&self) -> bool {
        self.stopim() != 0
    }

    #[doc="Sets the STOPIM field."]
    #[inline] pub fn set_stopim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receive DMA Interrupt Mask"]
    #[inline] pub fn dmarxim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DMARXIM != 0"]
    #[inline] pub fn test_dmarxim(&self) -> bool {
        self.dmarxim() != 0
    }

    #[doc="Sets the DMARXIM field."]
    #[inline] pub fn set_dmarxim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Transmit DMA Interrupt Mask"]
    #[inline] pub fn dmatxim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DMATXIM != 0"]
    #[inline] pub fn test_dmatxim(&self) -> bool {
        self.dmatxim() != 0
    }

    #[doc="Sets the DMATXIM field."]
    #[inline] pub fn set_dmatxim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Transmit FIFO Request Interrupt Mask"]
    #[inline] pub fn txim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXIM != 0"]
    #[inline] pub fn test_txim(&self) -> bool {
        self.txim() != 0
    }

    #[doc="Sets the TXIM field."]
    #[inline] pub fn set_txim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Receive FIFO Request Interrupt Mask"]
    #[inline] pub fn rxim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RXIM != 0"]
    #[inline] pub fn test_rxim(&self) -> bool {
        self.rxim() != 0
    }

    #[doc="Sets the RXIM field."]
    #[inline] pub fn set_rxim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transmit FIFO Empty Interrupt Mask"]
    #[inline] pub fn txfeim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXFEIM != 0"]
    #[inline] pub fn test_txfeim(&self) -> bool {
        self.txfeim() != 0
    }

    #[doc="Sets the TXFEIM field."]
    #[inline] pub fn set_txfeim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Receive FIFO Full Interrupt Mask"]
    #[inline] pub fn rxffim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if RXFFIM != 0"]
    #[inline] pub fn test_rxffim(&self) -> bool {
        self.rxffim() != 0
    }

    #[doc="Sets the RXFFIM field."]
    #[inline] pub fn set_rxffim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Simr {
    #[inline]
    fn from(other: u32) -> Self {
         Simr(other)
    }
}

impl ::core::fmt::Display for Simr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Simr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dataim() != 0 { try!(write!(f, " dataim"))}
        if self.startim() != 0 { try!(write!(f, " startim"))}
        if self.stopim() != 0 { try!(write!(f, " stopim"))}
        if self.dmarxim() != 0 { try!(write!(f, " dmarxim"))}
        if self.dmatxim() != 0 { try!(write!(f, " dmatxim"))}
        if self.txim() != 0 { try!(write!(f, " txim"))}
        if self.rxim() != 0 { try!(write!(f, " rxim"))}
        if self.txfeim() != 0 { try!(write!(f, " txfeim"))}
        if self.rxffim() != 0 { try!(write!(f, " rxffim"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Slave Raw Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sris(pub u32);
impl Sris {
    #[doc="Data Raw Interrupt Status"]
    #[inline] pub fn dataris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DATARIS != 0"]
    #[inline] pub fn test_dataris(&self) -> bool {
        self.dataris() != 0
    }

    #[doc="Sets the DATARIS field."]
    #[inline] pub fn set_dataris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Start Condition Raw Interrupt Status"]
    #[inline] pub fn startris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if STARTRIS != 0"]
    #[inline] pub fn test_startris(&self) -> bool {
        self.startris() != 0
    }

    #[doc="Sets the STARTRIS field."]
    #[inline] pub fn set_startris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Stop Condition Raw Interrupt Status"]
    #[inline] pub fn stopris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if STOPRIS != 0"]
    #[inline] pub fn test_stopris(&self) -> bool {
        self.stopris() != 0
    }

    #[doc="Sets the STOPRIS field."]
    #[inline] pub fn set_stopris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receive DMA Raw Interrupt Status"]
    #[inline] pub fn dmarxris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DMARXRIS != 0"]
    #[inline] pub fn test_dmarxris(&self) -> bool {
        self.dmarxris() != 0
    }

    #[doc="Sets the DMARXRIS field."]
    #[inline] pub fn set_dmarxris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Transmit DMA Raw Interrupt Status"]
    #[inline] pub fn dmatxris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DMATXRIS != 0"]
    #[inline] pub fn test_dmatxris(&self) -> bool {
        self.dmatxris() != 0
    }

    #[doc="Sets the DMATXRIS field."]
    #[inline] pub fn set_dmatxris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Transmit Request Raw Interrupt Status"]
    #[inline] pub fn txris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXRIS != 0"]
    #[inline] pub fn test_txris(&self) -> bool {
        self.txris() != 0
    }

    #[doc="Sets the TXRIS field."]
    #[inline] pub fn set_txris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Receive FIFO Request Raw Interrupt Status"]
    #[inline] pub fn rxris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RXRIS != 0"]
    #[inline] pub fn test_rxris(&self) -> bool {
        self.rxris() != 0
    }

    #[doc="Sets the RXRIS field."]
    #[inline] pub fn set_rxris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transmit FIFO Empty Raw Interrupt Status"]
    #[inline] pub fn txferis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXFERIS != 0"]
    #[inline] pub fn test_txferis(&self) -> bool {
        self.txferis() != 0
    }

    #[doc="Sets the TXFERIS field."]
    #[inline] pub fn set_txferis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Receive FIFO Full Raw Interrupt Status"]
    #[inline] pub fn rxffris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if RXFFRIS != 0"]
    #[inline] pub fn test_rxffris(&self) -> bool {
        self.rxffris() != 0
    }

    #[doc="Sets the RXFFRIS field."]
    #[inline] pub fn set_rxffris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Sris {
    #[inline]
    fn from(other: u32) -> Self {
         Sris(other)
    }
}

impl ::core::fmt::Display for Sris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dataris() != 0 { try!(write!(f, " dataris"))}
        if self.startris() != 0 { try!(write!(f, " startris"))}
        if self.stopris() != 0 { try!(write!(f, " stopris"))}
        if self.dmarxris() != 0 { try!(write!(f, " dmarxris"))}
        if self.dmatxris() != 0 { try!(write!(f, " dmatxris"))}
        if self.txris() != 0 { try!(write!(f, " txris"))}
        if self.rxris() != 0 { try!(write!(f, " rxris"))}
        if self.txferis() != 0 { try!(write!(f, " txferis"))}
        if self.rxffris() != 0 { try!(write!(f, " rxffris"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Slave Masked Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Smis(pub u32);
impl Smis {
    #[doc="Data Masked Interrupt Status"]
    #[inline] pub fn datamis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DATAMIS != 0"]
    #[inline] pub fn test_datamis(&self) -> bool {
        self.datamis() != 0
    }

    #[doc="Sets the DATAMIS field."]
    #[inline] pub fn set_datamis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Start Condition Masked Interrupt Status"]
    #[inline] pub fn startmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if STARTMIS != 0"]
    #[inline] pub fn test_startmis(&self) -> bool {
        self.startmis() != 0
    }

    #[doc="Sets the STARTMIS field."]
    #[inline] pub fn set_startmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Stop Condition Masked Interrupt Status"]
    #[inline] pub fn stopmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if STOPMIS != 0"]
    #[inline] pub fn test_stopmis(&self) -> bool {
        self.stopmis() != 0
    }

    #[doc="Sets the STOPMIS field."]
    #[inline] pub fn set_stopmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receive DMA Masked Interrupt Status"]
    #[inline] pub fn dmarxmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DMARXMIS != 0"]
    #[inline] pub fn test_dmarxmis(&self) -> bool {
        self.dmarxmis() != 0
    }

    #[doc="Sets the DMARXMIS field."]
    #[inline] pub fn set_dmarxmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Transmit DMA Masked Interrupt Status"]
    #[inline] pub fn dmatxmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DMATXMIS != 0"]
    #[inline] pub fn test_dmatxmis(&self) -> bool {
        self.dmatxmis() != 0
    }

    #[doc="Sets the DMATXMIS field."]
    #[inline] pub fn set_dmatxmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Transmit FIFO Request Interrupt Mask"]
    #[inline] pub fn txmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXMIS != 0"]
    #[inline] pub fn test_txmis(&self) -> bool {
        self.txmis() != 0
    }

    #[doc="Sets the TXMIS field."]
    #[inline] pub fn set_txmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Receive FIFO Request Interrupt Mask"]
    #[inline] pub fn rxmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RXMIS != 0"]
    #[inline] pub fn test_rxmis(&self) -> bool {
        self.rxmis() != 0
    }

    #[doc="Sets the RXMIS field."]
    #[inline] pub fn set_rxmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transmit FIFO Empty Interrupt Mask"]
    #[inline] pub fn txfemis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXFEMIS != 0"]
    #[inline] pub fn test_txfemis(&self) -> bool {
        self.txfemis() != 0
    }

    #[doc="Sets the TXFEMIS field."]
    #[inline] pub fn set_txfemis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Receive FIFO Full Interrupt Mask"]
    #[inline] pub fn rxffmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if RXFFMIS != 0"]
    #[inline] pub fn test_rxffmis(&self) -> bool {
        self.rxffmis() != 0
    }

    #[doc="Sets the RXFFMIS field."]
    #[inline] pub fn set_rxffmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Smis {
    #[inline]
    fn from(other: u32) -> Self {
         Smis(other)
    }
}

impl ::core::fmt::Display for Smis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Smis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.datamis() != 0 { try!(write!(f, " datamis"))}
        if self.startmis() != 0 { try!(write!(f, " startmis"))}
        if self.stopmis() != 0 { try!(write!(f, " stopmis"))}
        if self.dmarxmis() != 0 { try!(write!(f, " dmarxmis"))}
        if self.dmatxmis() != 0 { try!(write!(f, " dmatxmis"))}
        if self.txmis() != 0 { try!(write!(f, " txmis"))}
        if self.rxmis() != 0 { try!(write!(f, " rxmis"))}
        if self.txfemis() != 0 { try!(write!(f, " txfemis"))}
        if self.rxffmis() != 0 { try!(write!(f, " rxffmis"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Slave Interrupt Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sicr(pub u32);
impl Sicr {
    #[doc="Data Interrupt Clear"]
    #[inline] pub fn dataic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DATAIC != 0"]
    #[inline] pub fn test_dataic(&self) -> bool {
        self.dataic() != 0
    }

    #[doc="Sets the DATAIC field."]
    #[inline] pub fn set_dataic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Start Condition Interrupt Clear"]
    #[inline] pub fn startic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if STARTIC != 0"]
    #[inline] pub fn test_startic(&self) -> bool {
        self.startic() != 0
    }

    #[doc="Sets the STARTIC field."]
    #[inline] pub fn set_startic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Stop Condition Interrupt Clear"]
    #[inline] pub fn stopic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if STOPIC != 0"]
    #[inline] pub fn test_stopic(&self) -> bool {
        self.stopic() != 0
    }

    #[doc="Sets the STOPIC field."]
    #[inline] pub fn set_stopic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receive DMA Interrupt Clear"]
    #[inline] pub fn dmarxic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DMARXIC != 0"]
    #[inline] pub fn test_dmarxic(&self) -> bool {
        self.dmarxic() != 0
    }

    #[doc="Sets the DMARXIC field."]
    #[inline] pub fn set_dmarxic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Transmit DMA Interrupt Clear"]
    #[inline] pub fn dmatxic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DMATXIC != 0"]
    #[inline] pub fn test_dmatxic(&self) -> bool {
        self.dmatxic() != 0
    }

    #[doc="Sets the DMATXIC field."]
    #[inline] pub fn set_dmatxic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Transmit Request Interrupt Mask"]
    #[inline] pub fn txic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXIC != 0"]
    #[inline] pub fn test_txic(&self) -> bool {
        self.txic() != 0
    }

    #[doc="Sets the TXIC field."]
    #[inline] pub fn set_txic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Receive Request Interrupt Mask"]
    #[inline] pub fn rxic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RXIC != 0"]
    #[inline] pub fn test_rxic(&self) -> bool {
        self.rxic() != 0
    }

    #[doc="Sets the RXIC field."]
    #[inline] pub fn set_rxic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transmit FIFO Empty Interrupt Mask"]
    #[inline] pub fn txfeic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXFEIC != 0"]
    #[inline] pub fn test_txfeic(&self) -> bool {
        self.txfeic() != 0
    }

    #[doc="Sets the TXFEIC field."]
    #[inline] pub fn set_txfeic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Receive FIFO Full Interrupt Mask"]
    #[inline] pub fn rxffic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if RXFFIC != 0"]
    #[inline] pub fn test_rxffic(&self) -> bool {
        self.rxffic() != 0
    }

    #[doc="Sets the RXFFIC field."]
    #[inline] pub fn set_rxffic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Sicr {
    #[inline]
    fn from(other: u32) -> Self {
         Sicr(other)
    }
}

impl ::core::fmt::Display for Sicr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sicr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dataic() != 0 { try!(write!(f, " dataic"))}
        if self.startic() != 0 { try!(write!(f, " startic"))}
        if self.stopic() != 0 { try!(write!(f, " stopic"))}
        if self.dmarxic() != 0 { try!(write!(f, " dmarxic"))}
        if self.dmatxic() != 0 { try!(write!(f, " dmatxic"))}
        if self.txic() != 0 { try!(write!(f, " txic"))}
        if self.rxic() != 0 { try!(write!(f, " rxic"))}
        if self.txfeic() != 0 { try!(write!(f, " txfeic"))}
        if self.rxffic() != 0 { try!(write!(f, " rxffic"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Slave Own Address 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Soar2(pub u32);
impl Soar2 {
    #[doc="I2C Slave Own Address 2"]
    #[inline] pub fn oar2(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7f) as u8) } // [6:0]
    }

    #[doc="Returns true if OAR2 != 0"]
    #[inline] pub fn test_oar2(&self) -> bool {
        self.oar2() != 0
    }

    #[doc="Sets the OAR2 field."]
    #[inline] pub fn set_oar2<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="I2C Slave Own Address 2 Enable"]
    #[inline] pub fn oar2en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OAR2EN != 0"]
    #[inline] pub fn test_oar2en(&self) -> bool {
        self.oar2en() != 0
    }

    #[doc="Sets the OAR2EN field."]
    #[inline] pub fn set_oar2en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Soar2 {
    #[inline]
    fn from(other: u32) -> Self {
         Soar2(other)
    }
}

impl ::core::fmt::Display for Soar2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Soar2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.oar2() != 0 { try!(write!(f, " oar2=0x{:x}", self.oar2()))}
        if self.oar2en() != 0 { try!(write!(f, " oar2en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2C Slave ACK Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sackctl(pub u32);
impl Sackctl {
    #[doc="I2C Slave ACK Override Enable"]
    #[inline] pub fn ackoen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ACKOEN != 0"]
    #[inline] pub fn test_ackoen(&self) -> bool {
        self.ackoen() != 0
    }

    #[doc="Sets the ACKOEN field."]
    #[inline] pub fn set_ackoen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="I2C Slave ACK Override Value"]
    #[inline] pub fn ackoval(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ACKOVAL != 0"]
    #[inline] pub fn test_ackoval(&self) -> bool {
        self.ackoval() != 0
    }

    #[doc="Sets the ACKOVAL field."]
    #[inline] pub fn set_ackoval<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Sackctl {
    #[inline]
    fn from(other: u32) -> Self {
         Sackctl(other)
    }
}

impl ::core::fmt::Display for Sackctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sackctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ackoen() != 0 { try!(write!(f, " ackoen"))}
        if self.ackoval() != 0 { try!(write!(f, " ackoval"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

}
// End of slave

