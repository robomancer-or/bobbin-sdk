//! Reset Control Module

#[allow(unused_imports)] use ::bobbin_common::*;

#[doc="Reset Control Module"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RcmPeriph(pub usize);
impl RcmPeriph {
    #[doc="Get the *mut pointer for the VERID register."]
    #[inline] pub fn verid_mut(&self) -> *mut Verid { 
        (self.0 + 0x0) as *mut Verid
    }

    #[doc="Get the *const pointer for the VERID register."]
    #[inline] pub fn verid_ptr(&self) -> *const Verid { 
           self.verid_mut()
    }

    #[doc="Read the VERID register."]
    #[inline] pub fn verid(&self) -> Verid { 
        unsafe {
            read_volatile(self.verid_ptr())
        }
    }

    #[doc="Get the *mut pointer for the PARAM register."]
    #[inline] pub fn param_mut(&self) -> *mut Param { 
        (self.0 + 0x4) as *mut Param
    }

    #[doc="Get the *const pointer for the PARAM register."]
    #[inline] pub fn param_ptr(&self) -> *const Param { 
           self.param_mut()
    }

    #[doc="Read the PARAM register."]
    #[inline] pub fn param(&self) -> Param { 
        unsafe {
            read_volatile(self.param_ptr())
        }
    }

    #[doc="Get the *mut pointer for the SRS register."]
    #[inline] pub fn srs_mut(&self) -> *mut Srs { 
        (self.0 + 0x8) as *mut Srs
    }

    #[doc="Get the *const pointer for the SRS register."]
    #[inline] pub fn srs_ptr(&self) -> *const Srs { 
           self.srs_mut()
    }

    #[doc="Read the SRS register."]
    #[inline] pub fn srs(&self) -> Srs { 
        unsafe {
            read_volatile(self.srs_ptr())
        }
    }

    #[doc="Get the *mut pointer for the RPC register."]
    #[inline] pub fn rpc_mut(&self) -> *mut Rpc { 
        (self.0 + 0xc) as *mut Rpc
    }

    #[doc="Get the *const pointer for the RPC register."]
    #[inline] pub fn rpc_ptr(&self) -> *const Rpc { 
           self.rpc_mut()
    }

    #[doc="Read the RPC register."]
    #[inline] pub fn rpc(&self) -> Rpc { 
        unsafe {
            read_volatile(self.rpc_ptr())
        }
    }

    #[doc="Write the RPC register."]
    #[inline] pub fn set_rpc<F: FnOnce(Rpc) -> Rpc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rpc_mut(), f(Rpc(0)));
        }
        self
    }

    #[doc="Modify the RPC register."]
    #[inline] pub fn with_rpc<F: FnOnce(Rpc) -> Rpc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rpc_mut(), f(self.rpc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SSRS register."]
    #[inline] pub fn ssrs_mut(&self) -> *mut Ssrs { 
        (self.0 + 0x18) as *mut Ssrs
    }

    #[doc="Get the *const pointer for the SSRS register."]
    #[inline] pub fn ssrs_ptr(&self) -> *const Ssrs { 
           self.ssrs_mut()
    }

    #[doc="Read the SSRS register."]
    #[inline] pub fn ssrs(&self) -> Ssrs { 
        unsafe {
            read_volatile(self.ssrs_ptr())
        }
    }

    #[doc="Write the SSRS register."]
    #[inline] pub fn set_ssrs<F: FnOnce(Ssrs) -> Ssrs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ssrs_mut(), f(Ssrs(0)));
        }
        self
    }

    #[doc="Modify the SSRS register."]
    #[inline] pub fn with_ssrs<F: FnOnce(Ssrs) -> Ssrs>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ssrs_mut(), f(self.ssrs()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SRIE register."]
    #[inline] pub fn srie_mut(&self) -> *mut Srie { 
        (self.0 + 0x1c) as *mut Srie
    }

    #[doc="Get the *const pointer for the SRIE register."]
    #[inline] pub fn srie_ptr(&self) -> *const Srie { 
           self.srie_mut()
    }

    #[doc="Read the SRIE register."]
    #[inline] pub fn srie(&self) -> Srie { 
        unsafe {
            read_volatile(self.srie_ptr())
        }
    }

    #[doc="Write the SRIE register."]
    #[inline] pub fn set_srie<F: FnOnce(Srie) -> Srie>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.srie_mut(), f(Srie(0)));
        }
        self
    }

    #[doc="Modify the SRIE register."]
    #[inline] pub fn with_srie<F: FnOnce(Srie) -> Srie>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.srie_mut(), f(self.srie()));
        }
        self
    }

}

#[doc="Version ID Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc="Feature Specification Number"]
    #[inline] pub fn feature(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if FEATURE != 0"]
    #[inline] pub fn test_feature(&self) -> bool {
        self.feature() != 0
    }

    #[doc="Sets the FEATURE field."]
    #[inline] pub fn set_feature<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Minor Version Number"]
    #[inline] pub fn minor(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if MINOR != 0"]
    #[inline] pub fn test_minor(&self) -> bool {
        self.minor() != 0
    }

    #[doc="Sets the MINOR field."]
    #[inline] pub fn set_minor<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Major Version Number"]
    #[inline] pub fn major(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if MAJOR != 0"]
    #[inline] pub fn test_major(&self) -> bool {
        self.major() != 0
    }

    #[doc="Sets the MAJOR field."]
    #[inline] pub fn set_major<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Verid {
    #[inline]
    fn from(other: u32) -> Self {
         Verid(other)
    }
}

impl ::core::fmt::Display for Verid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Verid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.feature() != 0 { try!(write!(f, " feature=0x{:x}", self.feature()))}
        if self.minor() != 0 { try!(write!(f, " minor=0x{:x}", self.minor()))}
        if self.major() != 0 { try!(write!(f, " major=0x{:x}", self.major()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Parameter Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Param(pub u32);
impl Param {
    #[doc="Existence of SRS[WAKEUP] status indication feature"]
    #[inline] pub fn ewakeup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EWAKEUP != 0"]
    #[inline] pub fn test_ewakeup(&self) -> bool {
        self.ewakeup() != 0
    }

    #[doc="Sets the EWAKEUP field."]
    #[inline] pub fn set_ewakeup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Existence of SRS[LVD] status indication feature"]
    #[inline] pub fn elvd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ELVD != 0"]
    #[inline] pub fn test_elvd(&self) -> bool {
        self.elvd() != 0
    }

    #[doc="Sets the ELVD field."]
    #[inline] pub fn set_elvd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Existence of SRS[LOC] status indication feature"]
    #[inline] pub fn eloc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ELOC != 0"]
    #[inline] pub fn test_eloc(&self) -> bool {
        self.eloc() != 0
    }

    #[doc="Sets the ELOC field."]
    #[inline] pub fn set_eloc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Existence of SRS[LOL] status indication feature"]
    #[inline] pub fn elol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ELOL != 0"]
    #[inline] pub fn test_elol(&self) -> bool {
        self.elol() != 0
    }

    #[doc="Sets the ELOL field."]
    #[inline] pub fn set_elol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Existence of SRS[WDOG] status indication feature"]
    #[inline] pub fn ewdog(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if EWDOG != 0"]
    #[inline] pub fn test_ewdog(&self) -> bool {
        self.ewdog() != 0
    }

    #[doc="Sets the EWDOG field."]
    #[inline] pub fn set_ewdog<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Existence of SRS[PIN] status indication feature"]
    #[inline] pub fn epin(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if EPIN != 0"]
    #[inline] pub fn test_epin(&self) -> bool {
        self.epin() != 0
    }

    #[doc="Sets the EPIN field."]
    #[inline] pub fn set_epin<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Existence of SRS[POR] status indication feature"]
    #[inline] pub fn epor(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if EPOR != 0"]
    #[inline] pub fn test_epor(&self) -> bool {
        self.epor() != 0
    }

    #[doc="Sets the EPOR field."]
    #[inline] pub fn set_epor<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Existence of SRS[JTAG] status indication feature"]
    #[inline] pub fn ejtag(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if EJTAG != 0"]
    #[inline] pub fn test_ejtag(&self) -> bool {
        self.ejtag() != 0
    }

    #[doc="Sets the EJTAG field."]
    #[inline] pub fn set_ejtag<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Existence of SRS[LOCKUP] status indication feature"]
    #[inline] pub fn elockup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ELOCKUP != 0"]
    #[inline] pub fn test_elockup(&self) -> bool {
        self.elockup() != 0
    }

    #[doc="Sets the ELOCKUP field."]
    #[inline] pub fn set_elockup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Existence of SRS[SW] status indication feature"]
    #[inline] pub fn esw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if ESW != 0"]
    #[inline] pub fn test_esw(&self) -> bool {
        self.esw() != 0
    }

    #[doc="Sets the ESW field."]
    #[inline] pub fn set_esw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Existence of SRS[MDM_AP] status indication feature"]
    #[inline] pub fn emdm_ap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if EMDM_AP != 0"]
    #[inline] pub fn test_emdm_ap(&self) -> bool {
        self.emdm_ap() != 0
    }

    #[doc="Sets the EMDM_AP field."]
    #[inline] pub fn set_emdm_ap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Existence of SRS[SACKERR] status indication feature"]
    #[inline] pub fn esackerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if ESACKERR != 0"]
    #[inline] pub fn test_esackerr(&self) -> bool {
        self.esackerr() != 0
    }

    #[doc="Sets the ESACKERR field."]
    #[inline] pub fn set_esackerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Existence of SRS[TAMPER] status indication feature"]
    #[inline] pub fn etamper(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if ETAMPER != 0"]
    #[inline] pub fn test_etamper(&self) -> bool {
        self.etamper() != 0
    }

    #[doc="Sets the ETAMPER field."]
    #[inline] pub fn set_etamper<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Existence of SRS[CORE1] status indication feature"]
    #[inline] pub fn ecore1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if ECORE1 != 0"]
    #[inline] pub fn test_ecore1(&self) -> bool {
        self.ecore1() != 0
    }

    #[doc="Sets the ECORE1 field."]
    #[inline] pub fn set_ecore1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Param {
    #[inline]
    fn from(other: u32) -> Self {
         Param(other)
    }
}

impl ::core::fmt::Display for Param {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Param {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ewakeup() != 0 { try!(write!(f, " ewakeup"))}
        if self.elvd() != 0 { try!(write!(f, " elvd"))}
        if self.eloc() != 0 { try!(write!(f, " eloc"))}
        if self.elol() != 0 { try!(write!(f, " elol"))}
        if self.ewdog() != 0 { try!(write!(f, " ewdog"))}
        if self.epin() != 0 { try!(write!(f, " epin"))}
        if self.epor() != 0 { try!(write!(f, " epor"))}
        if self.ejtag() != 0 { try!(write!(f, " ejtag"))}
        if self.elockup() != 0 { try!(write!(f, " elockup"))}
        if self.esw() != 0 { try!(write!(f, " esw"))}
        if self.emdm_ap() != 0 { try!(write!(f, " emdm_ap"))}
        if self.esackerr() != 0 { try!(write!(f, " esackerr"))}
        if self.etamper() != 0 { try!(write!(f, " etamper"))}
        if self.ecore1() != 0 { try!(write!(f, " ecore1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Reset Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Srs(pub u32);
impl Srs {
    #[doc="Low-Voltage Detect Reset or High-Voltage Detect Reset"]
    #[inline] pub fn lvd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LVD != 0"]
    #[inline] pub fn test_lvd(&self) -> bool {
        self.lvd() != 0
    }

    #[doc="Sets the LVD field."]
    #[inline] pub fn set_lvd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Loss-of-Clock Reset"]
    #[inline] pub fn loc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if LOC != 0"]
    #[inline] pub fn test_loc(&self) -> bool {
        self.loc() != 0
    }

    #[doc="Sets the LOC field."]
    #[inline] pub fn set_loc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Loss-of-Lock Reset"]
    #[inline] pub fn lol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if LOL != 0"]
    #[inline] pub fn test_lol(&self) -> bool {
        self.lol() != 0
    }

    #[doc="Sets the LOL field."]
    #[inline] pub fn set_lol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Watchdog"]
    #[inline] pub fn wdog(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if WDOG != 0"]
    #[inline] pub fn test_wdog(&self) -> bool {
        self.wdog() != 0
    }

    #[doc="Sets the WDOG field."]
    #[inline] pub fn set_wdog<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="External Reset Pin"]
    #[inline] pub fn pin(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PIN != 0"]
    #[inline] pub fn test_pin(&self) -> bool {
        self.pin() != 0
    }

    #[doc="Sets the PIN field."]
    #[inline] pub fn set_pin<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Power-On Reset"]
    #[inline] pub fn por(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if POR != 0"]
    #[inline] pub fn test_por(&self) -> bool {
        self.por() != 0
    }

    #[doc="Sets the POR field."]
    #[inline] pub fn set_por<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="JTAG generated reset"]
    #[inline] pub fn jtag(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if JTAG != 0"]
    #[inline] pub fn test_jtag(&self) -> bool {
        self.jtag() != 0
    }

    #[doc="Sets the JTAG field."]
    #[inline] pub fn set_jtag<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Core Lockup"]
    #[inline] pub fn lockup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LOCKUP != 0"]
    #[inline] pub fn test_lockup(&self) -> bool {
        self.lockup() != 0
    }

    #[doc="Sets the LOCKUP field."]
    #[inline] pub fn set_lockup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Software"]
    #[inline] pub fn sw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SW != 0"]
    #[inline] pub fn test_sw(&self) -> bool {
        self.sw() != 0
    }

    #[doc="Sets the SW field."]
    #[inline] pub fn set_sw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="MDM-AP System Reset Request"]
    #[inline] pub fn mdm_ap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if MDM_AP != 0"]
    #[inline] pub fn test_mdm_ap(&self) -> bool {
        self.mdm_ap() != 0
    }

    #[doc="Sets the MDM_AP field."]
    #[inline] pub fn set_mdm_ap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Stop Acknowledge Error"]
    #[inline] pub fn sackerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if SACKERR != 0"]
    #[inline] pub fn test_sackerr(&self) -> bool {
        self.sackerr() != 0
    }

    #[doc="Sets the SACKERR field."]
    #[inline] pub fn set_sackerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u32> for Srs {
    #[inline]
    fn from(other: u32) -> Self {
         Srs(other)
    }
}

impl ::core::fmt::Display for Srs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Srs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lvd() != 0 { try!(write!(f, " lvd"))}
        if self.loc() != 0 { try!(write!(f, " loc"))}
        if self.lol() != 0 { try!(write!(f, " lol"))}
        if self.wdog() != 0 { try!(write!(f, " wdog"))}
        if self.pin() != 0 { try!(write!(f, " pin"))}
        if self.por() != 0 { try!(write!(f, " por"))}
        if self.jtag() != 0 { try!(write!(f, " jtag"))}
        if self.lockup() != 0 { try!(write!(f, " lockup"))}
        if self.sw() != 0 { try!(write!(f, " sw"))}
        if self.mdm_ap() != 0 { try!(write!(f, " mdm_ap"))}
        if self.sackerr() != 0 { try!(write!(f, " sackerr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reset Pin Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rpc(pub u32);
impl Rpc {
    #[doc="Reset Pin Filter Select in Run and Wait Modes"]
    #[inline] pub fn rstfltsrw(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if RSTFLTSRW != 0"]
    #[inline] pub fn test_rstfltsrw(&self) -> bool {
        self.rstfltsrw() != 0
    }

    #[doc="Sets the RSTFLTSRW field."]
    #[inline] pub fn set_rstfltsrw<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Reset Pin Filter Select in Stop Mode"]
    #[inline] pub fn rstfltss(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RSTFLTSS != 0"]
    #[inline] pub fn test_rstfltss(&self) -> bool {
        self.rstfltss() != 0
    }

    #[doc="Sets the RSTFLTSS field."]
    #[inline] pub fn set_rstfltss<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Reset Pin Filter Bus Clock Select"]
    #[inline] pub fn rstfltsel(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
    }

    #[doc="Returns true if RSTFLTSEL != 0"]
    #[inline] pub fn test_rstfltsel(&self) -> bool {
        self.rstfltsel() != 0
    }

    #[doc="Sets the RSTFLTSEL field."]
    #[inline] pub fn set_rstfltsel<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Rpc {
    #[inline]
    fn from(other: u32) -> Self {
         Rpc(other)
    }
}

impl ::core::fmt::Display for Rpc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rpc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rstfltsrw() != 0 { try!(write!(f, " rstfltsrw=0x{:x}", self.rstfltsrw()))}
        if self.rstfltss() != 0 { try!(write!(f, " rstfltss"))}
        if self.rstfltsel() != 0 { try!(write!(f, " rstfltsel=0x{:x}", self.rstfltsel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Sticky System Reset Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ssrs(pub u32);
impl Ssrs {
    #[doc="Sticky Low-Voltage Detect Reset"]
    #[inline] pub fn slvd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SLVD != 0"]
    #[inline] pub fn test_slvd(&self) -> bool {
        self.slvd() != 0
    }

    #[doc="Sets the SLVD field."]
    #[inline] pub fn set_slvd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Sticky Loss-of-Clock Reset"]
    #[inline] pub fn sloc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SLOC != 0"]
    #[inline] pub fn test_sloc(&self) -> bool {
        self.sloc() != 0
    }

    #[doc="Sets the SLOC field."]
    #[inline] pub fn set_sloc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Sticky Loss-of-Lock Reset"]
    #[inline] pub fn slol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SLOL != 0"]
    #[inline] pub fn test_slol(&self) -> bool {
        self.slol() != 0
    }

    #[doc="Sets the SLOL field."]
    #[inline] pub fn set_slol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Sticky Watchdog"]
    #[inline] pub fn swdog(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SWDOG != 0"]
    #[inline] pub fn test_swdog(&self) -> bool {
        self.swdog() != 0
    }

    #[doc="Sets the SWDOG field."]
    #[inline] pub fn set_swdog<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Sticky External Reset Pin"]
    #[inline] pub fn spin(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SPIN != 0"]
    #[inline] pub fn test_spin(&self) -> bool {
        self.spin() != 0
    }

    #[doc="Sets the SPIN field."]
    #[inline] pub fn set_spin<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Sticky Power-On Reset"]
    #[inline] pub fn spor(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SPOR != 0"]
    #[inline] pub fn test_spor(&self) -> bool {
        self.spor() != 0
    }

    #[doc="Sets the SPOR field."]
    #[inline] pub fn set_spor<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Sticky JTAG generated reset"]
    #[inline] pub fn sjtag(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SJTAG != 0"]
    #[inline] pub fn test_sjtag(&self) -> bool {
        self.sjtag() != 0
    }

    #[doc="Sets the SJTAG field."]
    #[inline] pub fn set_sjtag<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Sticky Core Lockup"]
    #[inline] pub fn slockup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SLOCKUP != 0"]
    #[inline] pub fn test_slockup(&self) -> bool {
        self.slockup() != 0
    }

    #[doc="Sets the SLOCKUP field."]
    #[inline] pub fn set_slockup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Sticky Software"]
    #[inline] pub fn ssw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SSW != 0"]
    #[inline] pub fn test_ssw(&self) -> bool {
        self.ssw() != 0
    }

    #[doc="Sets the SSW field."]
    #[inline] pub fn set_ssw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Sticky MDM-AP System Reset Request"]
    #[inline] pub fn smdm_ap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SMDM_AP != 0"]
    #[inline] pub fn test_smdm_ap(&self) -> bool {
        self.smdm_ap() != 0
    }

    #[doc="Sets the SMDM_AP field."]
    #[inline] pub fn set_smdm_ap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Sticky Stop Acknowledge Error"]
    #[inline] pub fn ssackerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if SSACKERR != 0"]
    #[inline] pub fn test_ssackerr(&self) -> bool {
        self.ssackerr() != 0
    }

    #[doc="Sets the SSACKERR field."]
    #[inline] pub fn set_ssackerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u32> for Ssrs {
    #[inline]
    fn from(other: u32) -> Self {
         Ssrs(other)
    }
}

impl ::core::fmt::Display for Ssrs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ssrs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.slvd() != 0 { try!(write!(f, " slvd"))}
        if self.sloc() != 0 { try!(write!(f, " sloc"))}
        if self.slol() != 0 { try!(write!(f, " slol"))}
        if self.swdog() != 0 { try!(write!(f, " swdog"))}
        if self.spin() != 0 { try!(write!(f, " spin"))}
        if self.spor() != 0 { try!(write!(f, " spor"))}
        if self.sjtag() != 0 { try!(write!(f, " sjtag"))}
        if self.slockup() != 0 { try!(write!(f, " slockup"))}
        if self.ssw() != 0 { try!(write!(f, " ssw"))}
        if self.smdm_ap() != 0 { try!(write!(f, " smdm_ap"))}
        if self.ssackerr() != 0 { try!(write!(f, " ssackerr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Reset Interrupt Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Srie(pub u32);
impl Srie {
    #[doc="Reset Delay Time"]
    #[inline] pub fn delay(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if DELAY != 0"]
    #[inline] pub fn test_delay(&self) -> bool {
        self.delay() != 0
    }

    #[doc="Sets the DELAY field."]
    #[inline] pub fn set_delay<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Loss-of-Clock Interrupt"]
    #[inline] pub fn loc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if LOC != 0"]
    #[inline] pub fn test_loc(&self) -> bool {
        self.loc() != 0
    }

    #[doc="Sets the LOC field."]
    #[inline] pub fn set_loc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Loss-of-Lock Interrupt"]
    #[inline] pub fn lol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if LOL != 0"]
    #[inline] pub fn test_lol(&self) -> bool {
        self.lol() != 0
    }

    #[doc="Sets the LOL field."]
    #[inline] pub fn set_lol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Watchdog Interrupt"]
    #[inline] pub fn wdog(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if WDOG != 0"]
    #[inline] pub fn test_wdog(&self) -> bool {
        self.wdog() != 0
    }

    #[doc="Sets the WDOG field."]
    #[inline] pub fn set_wdog<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="External Reset Pin Interrupt"]
    #[inline] pub fn pin(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PIN != 0"]
    #[inline] pub fn test_pin(&self) -> bool {
        self.pin() != 0
    }

    #[doc="Sets the PIN field."]
    #[inline] pub fn set_pin<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Global Interrupt Enable"]
    #[inline] pub fn gie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GIE != 0"]
    #[inline] pub fn test_gie(&self) -> bool {
        self.gie() != 0
    }

    #[doc="Sets the GIE field."]
    #[inline] pub fn set_gie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="JTAG generated reset"]
    #[inline] pub fn jtag(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if JTAG != 0"]
    #[inline] pub fn test_jtag(&self) -> bool {
        self.jtag() != 0
    }

    #[doc="Sets the JTAG field."]
    #[inline] pub fn set_jtag<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Core Lockup Interrupt"]
    #[inline] pub fn lockup(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LOCKUP != 0"]
    #[inline] pub fn test_lockup(&self) -> bool {
        self.lockup() != 0
    }

    #[doc="Sets the LOCKUP field."]
    #[inline] pub fn set_lockup<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Software Interrupt"]
    #[inline] pub fn sw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SW != 0"]
    #[inline] pub fn test_sw(&self) -> bool {
        self.sw() != 0
    }

    #[doc="Sets the SW field."]
    #[inline] pub fn set_sw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="MDM-AP System Reset Request"]
    #[inline] pub fn mdm_ap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if MDM_AP != 0"]
    #[inline] pub fn test_mdm_ap(&self) -> bool {
        self.mdm_ap() != 0
    }

    #[doc="Sets the MDM_AP field."]
    #[inline] pub fn set_mdm_ap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Stop Acknowledge Error Interrupt"]
    #[inline] pub fn sackerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if SACKERR != 0"]
    #[inline] pub fn test_sackerr(&self) -> bool {
        self.sackerr() != 0
    }

    #[doc="Sets the SACKERR field."]
    #[inline] pub fn set_sackerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u32> for Srie {
    #[inline]
    fn from(other: u32) -> Self {
         Srie(other)
    }
}

impl ::core::fmt::Display for Srie {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Srie {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.delay() != 0 { try!(write!(f, " delay=0x{:x}", self.delay()))}
        if self.loc() != 0 { try!(write!(f, " loc"))}
        if self.lol() != 0 { try!(write!(f, " lol"))}
        if self.wdog() != 0 { try!(write!(f, " wdog"))}
        if self.pin() != 0 { try!(write!(f, " pin"))}
        if self.gie() != 0 { try!(write!(f, " gie"))}
        if self.jtag() != 0 { try!(write!(f, " jtag"))}
        if self.lockup() != 0 { try!(write!(f, " lockup"))}
        if self.sw() != 0 { try!(write!(f, " sw"))}
        if self.mdm_ap() != 0 { try!(write!(f, " mdm_ap"))}
        if self.sackerr() != 0 { try!(write!(f, " sackerr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

