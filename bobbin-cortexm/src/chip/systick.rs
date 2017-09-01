//! System Timer, SysTick
#[allow(unused_imports)] use bobbin_common::*;

periph!(SYSTICK, Systick, 0xe000e000);

#[doc="System Timer, SysTick"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Systick(pub usize);
impl Systick {
    #[doc="Get the *const pointer for the CSR register."]
    #[inline] pub fn csr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x10) as *const u32
    }

    #[doc="Get the *mut pointer for the CSR register."]
    #[inline] pub fn csr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x10) as *mut u32
    }

    #[doc="Read the CSR register."]
    #[inline] pub fn csr(&self) -> Csr { 
        unsafe {
            Csr(read_volatile((self.0 + 0x10) as *const u32))
        }
    }

    #[doc="Write the CSR register."]
    #[inline] pub fn set_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
        let value = f(Csr(0));
        unsafe {
            write_volatile((self.0 + 0x10) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the CSR register."]
    #[inline] pub fn with_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
        let tmp = self.csr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x10) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the RVR register."]
    #[inline] pub fn rvr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x14) as *const u32
    }

    #[doc="Get the *mut pointer for the RVR register."]
    #[inline] pub fn rvr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x14) as *mut u32
    }

    #[doc="Read the RVR register."]
    #[inline] pub fn rvr(&self) -> Rvr { 
        unsafe {
            Rvr(read_volatile((self.0 + 0x14) as *const u32))
        }
    }

    #[doc="Write the RVR register."]
    #[inline] pub fn set_rvr<F: FnOnce(Rvr) -> Rvr>(&self, f: F) -> &Self {
        let value = f(Rvr(0));
        unsafe {
            write_volatile((self.0 + 0x14) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the RVR register."]
    #[inline] pub fn with_rvr<F: FnOnce(Rvr) -> Rvr>(&self, f: F) -> &Self {
        let tmp = self.rvr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x14) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the CVR register."]
    #[inline] pub fn cvr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x18) as *const u32
    }

    #[doc="Get the *mut pointer for the CVR register."]
    #[inline] pub fn cvr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x18) as *mut u32
    }

    #[doc="Read the CVR register."]
    #[inline] pub fn cvr(&self) -> Cvr { 
        unsafe {
            Cvr(read_volatile((self.0 + 0x18) as *const u32))
        }
    }

    #[doc="Write the CVR register."]
    #[inline] pub fn set_cvr<F: FnOnce(Cvr) -> Cvr>(&self, f: F) -> &Self {
        let value = f(Cvr(0));
        unsafe {
            write_volatile((self.0 + 0x18) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the CVR register."]
    #[inline] pub fn with_cvr<F: FnOnce(Cvr) -> Cvr>(&self, f: F) -> &Self {
        let tmp = self.cvr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x18) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the CALIB register."]
    #[inline] pub fn calib_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x1c) as *const u32
    }

    #[doc="Get the *mut pointer for the CALIB register."]
    #[inline] pub fn calib_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x1c) as *mut u32
    }

    #[doc="Read the CALIB register."]
    #[inline] pub fn calib(&self) -> Calib { 
        unsafe {
            Calib(read_volatile((self.0 + 0x1c) as *const u32))
        }
    }

    #[doc="Write the CALIB register."]
    #[inline] pub fn set_calib<F: FnOnce(Calib) -> Calib>(&self, f: F) -> &Self {
        let value = f(Calib(0));
        unsafe {
            write_volatile((self.0 + 0x1c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the CALIB register."]
    #[inline] pub fn with_calib<F: FnOnce(Calib) -> Calib>(&self, f: F) -> &Self {
        let tmp = self.calib();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x1c) as *mut u32, value.0);
        }
        self
    }

}

#[doc="Control and Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc="Returns 1 if timer counted to 0 since last time this was read."]
    #[inline] pub fn countflag(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns 1 if timer counted to 0 since last time this was read."]
    #[inline] pub fn test_countflag(&self) -> bool {
        self.countflag() != 0
    }

    #[doc="Returns 1 if timer counted to 0 since last time this was read."]
    #[inline] pub fn set_countflag<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Indicates the clock source: 0 = external clock, 1 = processor clock."]
    #[inline] pub fn clksource(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Indicates the clock source: 0 = external clock, 1 = processor clock."]
    #[inline] pub fn test_clksource(&self) -> bool {
        self.clksource() != 0
    }

    #[doc="Indicates the clock source: 0 = external clock, 1 = processor clock."]
    #[inline] pub fn set_clksource<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Enables SysTick exception request: 0 = counting down to zero does not assert the SysTick exception request, 1 = counting down to zero asserts the SysTick exception request."]
    #[inline] pub fn tickint(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Enables SysTick exception request: 0 = counting down to zero does not assert the SysTick exception request, 1 = counting down to zero asserts the SysTick exception request."]
    #[inline] pub fn test_tickint(&self) -> bool {
        self.tickint() != 0
    }

    #[doc="Enables SysTick exception request: 0 = counting down to zero does not assert the SysTick exception request, 1 = counting down to zero asserts the SysTick exception request."]
    #[inline] pub fn set_tickint<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Enables the counter: 0 = counter disabled, 1 = counter enabled."]
    #[inline] pub fn enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Enables the counter: 0 = counter disabled, 1 = counter enabled."]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Enables the counter: 0 = counter disabled, 1 = counter enabled."]
    #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Csr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.countflag() != 0 { try!(write!(f, " countflag"))}
        if self.clksource() != 0 { try!(write!(f, " clksource"))}
        if self.tickint() != 0 { try!(write!(f, " tickint"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reload Value Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rvr(pub u32);
impl Rvr {
    #[doc="Value to load into the SYST_CVR register when the counter is enabled and when it reaches 0"]
    #[inline] pub fn reload(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Value to load into the SYST_CVR register when the counter is enabled and when it reaches 0"]
    #[inline] pub fn test_reload(&self) -> bool {
        self.reload() != 0
    }

    #[doc="Value to load into the SYST_CVR register when the counter is enabled and when it reaches 0"]
    #[inline] pub fn set_reload<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Rvr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rvr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.reload() != 0 { try!(write!(f, " reload=0x{:x}", self.reload()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Current Value Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cvr(pub u32);
impl Cvr {
    #[doc="Reads return the current value of the SysTick counter. A write of any value clears the field to 0, and also clears the SYST_CSR COUNTFLAG bit to 0."]
    #[inline] pub fn current(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Reads return the current value of the SysTick counter. A write of any value clears the field to 0, and also clears the SYST_CSR COUNTFLAG bit to 0."]
    #[inline] pub fn test_current(&self) -> bool {
        self.current() != 0
    }

    #[doc="Reads return the current value of the SysTick counter. A write of any value clears the field to 0, and also clears the SYST_CSR COUNTFLAG bit to 0."]
    #[inline] pub fn set_current<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Cvr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cvr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.current() != 0 { try!(write!(f, " current=0x{:x}", self.current()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Calibration Value Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Calib(pub u32);
impl Calib {
    #[doc="Indicates whether the device provides a reference clock to the processor"]
    #[inline] pub fn noref(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Indicates whether the device provides a reference clock to the processor"]
    #[inline] pub fn test_noref(&self) -> bool {
        self.noref() != 0
    }

    #[doc="Indicates whether the device provides a reference clock to the processor"]
    #[inline] pub fn set_noref<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Indicates whether the TENMS value is exact: 0 = TENMS value is exact, 1 = TENMS value is inexact, or not given."]
    #[inline] pub fn skew(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Indicates whether the TENMS value is exact: 0 = TENMS value is exact, 1 = TENMS value is inexact, or not given."]
    #[inline] pub fn test_skew(&self) -> bool {
        self.skew() != 0
    }

    #[doc="Indicates whether the TENMS value is exact: 0 = TENMS value is exact, 1 = TENMS value is inexact, or not given."]
    #[inline] pub fn set_skew<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Reload value for 10ms (100Hz) timing, subject to system clock skew errors."]
    #[inline] pub fn tenms(&self) -> bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Reload value for 10ms (100Hz) timing, subject to system clock skew errors."]
    #[inline] pub fn test_tenms(&self) -> bool {
        self.tenms() != 0
    }

    #[doc="Reload value for 10ms (100Hz) timing, subject to system clock skew errors."]
    #[inline] pub fn set_tenms<V: Into<bits::U24>>(mut self, value: V) -> Self {
        let value: bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Calib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Calib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.noref() != 0 { try!(write!(f, " noref"))}
        if self.skew() != 0 { try!(write!(f, " skew"))}
        if self.tenms() != 0 { try!(write!(f, " tenms=0x{:x}", self.tenms()))}
        try!(write!(f, "]"));
        Ok(())
    }
}


