//! Reset and clock control

#[allow(unused_imports)] use ::bobbin_common::*;

#[doc="Reset and clock control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RccPeriph(pub usize);
impl RccPeriph {
    #[doc="Get the *mut pointer for the CR register."]
    #[inline] pub fn cr_mut(&self) -> *mut Cr { 
        (self.0 + 0x0) as *mut Cr
    }

    #[doc="Get the *const pointer for the CR register."]
    #[inline] pub fn cr_ptr(&self) -> *const Cr { 
           self.cr_mut()
    }

    #[doc="Read the CR register."]
    #[inline] pub fn cr(&self) -> Cr { 
        unsafe {
            read_volatile(self.cr_ptr())
        }
    }

    #[doc="Write the CR register."]
    #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr_mut(), f(Cr(0)));
        }
        self
    }

    #[doc="Modify the CR register."]
    #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr_mut(), f(self.cr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CFGR register."]
    #[inline] pub fn cfgr_mut(&self) -> *mut Cfgr { 
        (self.0 + 0x4) as *mut Cfgr
    }

    #[doc="Get the *const pointer for the CFGR register."]
    #[inline] pub fn cfgr_ptr(&self) -> *const Cfgr { 
           self.cfgr_mut()
    }

    #[doc="Read the CFGR register."]
    #[inline] pub fn cfgr(&self) -> Cfgr { 
        unsafe {
            read_volatile(self.cfgr_ptr())
        }
    }

    #[doc="Write the CFGR register."]
    #[inline] pub fn set_cfgr<F: FnOnce(Cfgr) -> Cfgr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgr_mut(), f(Cfgr(0)));
        }
        self
    }

    #[doc="Modify the CFGR register."]
    #[inline] pub fn with_cfgr<F: FnOnce(Cfgr) -> Cfgr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgr_mut(), f(self.cfgr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CIR register."]
    #[inline] pub fn cir_mut(&self) -> *mut Cir { 
        (self.0 + 0x8) as *mut Cir
    }

    #[doc="Get the *const pointer for the CIR register."]
    #[inline] pub fn cir_ptr(&self) -> *const Cir { 
           self.cir_mut()
    }

    #[doc="Read the CIR register."]
    #[inline] pub fn cir(&self) -> Cir { 
        unsafe {
            read_volatile(self.cir_ptr())
        }
    }

    #[doc="Write the CIR register."]
    #[inline] pub fn set_cir<F: FnOnce(Cir) -> Cir>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cir_mut(), f(Cir(0)));
        }
        self
    }

    #[doc="Modify the CIR register."]
    #[inline] pub fn with_cir<F: FnOnce(Cir) -> Cir>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cir_mut(), f(self.cir()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APB2RSTR register."]
    #[inline] pub fn apb2rstr_mut(&self) -> *mut Apb2rstr { 
        (self.0 + 0xc) as *mut Apb2rstr
    }

    #[doc="Get the *const pointer for the APB2RSTR register."]
    #[inline] pub fn apb2rstr_ptr(&self) -> *const Apb2rstr { 
           self.apb2rstr_mut()
    }

    #[doc="Read the APB2RSTR register."]
    #[inline] pub fn apb2rstr(&self) -> Apb2rstr { 
        unsafe {
            read_volatile(self.apb2rstr_ptr())
        }
    }

    #[doc="Write the APB2RSTR register."]
    #[inline] pub fn set_apb2rstr<F: FnOnce(Apb2rstr) -> Apb2rstr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb2rstr_mut(), f(Apb2rstr(0)));
        }
        self
    }

    #[doc="Modify the APB2RSTR register."]
    #[inline] pub fn with_apb2rstr<F: FnOnce(Apb2rstr) -> Apb2rstr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb2rstr_mut(), f(self.apb2rstr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APB1RSTR register."]
    #[inline] pub fn apb1rstr_mut(&self) -> *mut Apb1rstr { 
        (self.0 + 0x10) as *mut Apb1rstr
    }

    #[doc="Get the *const pointer for the APB1RSTR register."]
    #[inline] pub fn apb1rstr_ptr(&self) -> *const Apb1rstr { 
           self.apb1rstr_mut()
    }

    #[doc="Read the APB1RSTR register."]
    #[inline] pub fn apb1rstr(&self) -> Apb1rstr { 
        unsafe {
            read_volatile(self.apb1rstr_ptr())
        }
    }

    #[doc="Write the APB1RSTR register."]
    #[inline] pub fn set_apb1rstr<F: FnOnce(Apb1rstr) -> Apb1rstr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1rstr_mut(), f(Apb1rstr(0)));
        }
        self
    }

    #[doc="Modify the APB1RSTR register."]
    #[inline] pub fn with_apb1rstr<F: FnOnce(Apb1rstr) -> Apb1rstr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1rstr_mut(), f(self.apb1rstr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AHBENR register."]
    #[inline] pub fn ahbenr_mut(&self) -> *mut Ahbenr { 
        (self.0 + 0x14) as *mut Ahbenr
    }

    #[doc="Get the *const pointer for the AHBENR register."]
    #[inline] pub fn ahbenr_ptr(&self) -> *const Ahbenr { 
           self.ahbenr_mut()
    }

    #[doc="Read the AHBENR register."]
    #[inline] pub fn ahbenr(&self) -> Ahbenr { 
        unsafe {
            read_volatile(self.ahbenr_ptr())
        }
    }

    #[doc="Write the AHBENR register."]
    #[inline] pub fn set_ahbenr<F: FnOnce(Ahbenr) -> Ahbenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahbenr_mut(), f(Ahbenr(0)));
        }
        self
    }

    #[doc="Modify the AHBENR register."]
    #[inline] pub fn with_ahbenr<F: FnOnce(Ahbenr) -> Ahbenr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahbenr_mut(), f(self.ahbenr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APB2ENR register."]
    #[inline] pub fn apb2enr_mut(&self) -> *mut Apb2enr { 
        (self.0 + 0x18) as *mut Apb2enr
    }

    #[doc="Get the *const pointer for the APB2ENR register."]
    #[inline] pub fn apb2enr_ptr(&self) -> *const Apb2enr { 
           self.apb2enr_mut()
    }

    #[doc="Read the APB2ENR register."]
    #[inline] pub fn apb2enr(&self) -> Apb2enr { 
        unsafe {
            read_volatile(self.apb2enr_ptr())
        }
    }

    #[doc="Write the APB2ENR register."]
    #[inline] pub fn set_apb2enr<F: FnOnce(Apb2enr) -> Apb2enr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb2enr_mut(), f(Apb2enr(0)));
        }
        self
    }

    #[doc="Modify the APB2ENR register."]
    #[inline] pub fn with_apb2enr<F: FnOnce(Apb2enr) -> Apb2enr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb2enr_mut(), f(self.apb2enr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the APB1ENR register."]
    #[inline] pub fn apb1enr_mut(&self) -> *mut Apb1enr { 
        (self.0 + 0x1c) as *mut Apb1enr
    }

    #[doc="Get the *const pointer for the APB1ENR register."]
    #[inline] pub fn apb1enr_ptr(&self) -> *const Apb1enr { 
           self.apb1enr_mut()
    }

    #[doc="Read the APB1ENR register."]
    #[inline] pub fn apb1enr(&self) -> Apb1enr { 
        unsafe {
            read_volatile(self.apb1enr_ptr())
        }
    }

    #[doc="Write the APB1ENR register."]
    #[inline] pub fn set_apb1enr<F: FnOnce(Apb1enr) -> Apb1enr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1enr_mut(), f(Apb1enr(0)));
        }
        self
    }

    #[doc="Modify the APB1ENR register."]
    #[inline] pub fn with_apb1enr<F: FnOnce(Apb1enr) -> Apb1enr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.apb1enr_mut(), f(self.apb1enr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BDCR register."]
    #[inline] pub fn bdcr_mut(&self) -> *mut Bdcr { 
        (self.0 + 0x20) as *mut Bdcr
    }

    #[doc="Get the *const pointer for the BDCR register."]
    #[inline] pub fn bdcr_ptr(&self) -> *const Bdcr { 
           self.bdcr_mut()
    }

    #[doc="Read the BDCR register."]
    #[inline] pub fn bdcr(&self) -> Bdcr { 
        unsafe {
            read_volatile(self.bdcr_ptr())
        }
    }

    #[doc="Write the BDCR register."]
    #[inline] pub fn set_bdcr<F: FnOnce(Bdcr) -> Bdcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bdcr_mut(), f(Bdcr(0)));
        }
        self
    }

    #[doc="Modify the BDCR register."]
    #[inline] pub fn with_bdcr<F: FnOnce(Bdcr) -> Bdcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.bdcr_mut(), f(self.bdcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CSR register."]
    #[inline] pub fn csr_mut(&self) -> *mut Csr { 
        (self.0 + 0x24) as *mut Csr
    }

    #[doc="Get the *const pointer for the CSR register."]
    #[inline] pub fn csr_ptr(&self) -> *const Csr { 
           self.csr_mut()
    }

    #[doc="Read the CSR register."]
    #[inline] pub fn csr(&self) -> Csr { 
        unsafe {
            read_volatile(self.csr_ptr())
        }
    }

    #[doc="Write the CSR register."]
    #[inline] pub fn set_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr_mut(), f(Csr(0)));
        }
        self
    }

    #[doc="Modify the CSR register."]
    #[inline] pub fn with_csr<F: FnOnce(Csr) -> Csr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.csr_mut(), f(self.csr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the AHBRSTR register."]
    #[inline] pub fn ahbrstr_mut(&self) -> *mut Ahbrstr { 
        (self.0 + 0x28) as *mut Ahbrstr
    }

    #[doc="Get the *const pointer for the AHBRSTR register."]
    #[inline] pub fn ahbrstr_ptr(&self) -> *const Ahbrstr { 
           self.ahbrstr_mut()
    }

    #[doc="Read the AHBRSTR register."]
    #[inline] pub fn ahbrstr(&self) -> Ahbrstr { 
        unsafe {
            read_volatile(self.ahbrstr_ptr())
        }
    }

    #[doc="Write the AHBRSTR register."]
    #[inline] pub fn set_ahbrstr<F: FnOnce(Ahbrstr) -> Ahbrstr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahbrstr_mut(), f(Ahbrstr(0)));
        }
        self
    }

    #[doc="Modify the AHBRSTR register."]
    #[inline] pub fn with_ahbrstr<F: FnOnce(Ahbrstr) -> Ahbrstr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ahbrstr_mut(), f(self.ahbrstr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CFGR2 register."]
    #[inline] pub fn cfgr2_mut(&self) -> *mut Cfgr2 { 
        (self.0 + 0x2c) as *mut Cfgr2
    }

    #[doc="Get the *const pointer for the CFGR2 register."]
    #[inline] pub fn cfgr2_ptr(&self) -> *const Cfgr2 { 
           self.cfgr2_mut()
    }

    #[doc="Read the CFGR2 register."]
    #[inline] pub fn cfgr2(&self) -> Cfgr2 { 
        unsafe {
            read_volatile(self.cfgr2_ptr())
        }
    }

    #[doc="Write the CFGR2 register."]
    #[inline] pub fn set_cfgr2<F: FnOnce(Cfgr2) -> Cfgr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgr2_mut(), f(Cfgr2(0)));
        }
        self
    }

    #[doc="Modify the CFGR2 register."]
    #[inline] pub fn with_cfgr2<F: FnOnce(Cfgr2) -> Cfgr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgr2_mut(), f(self.cfgr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CFGR3 register."]
    #[inline] pub fn cfgr3_mut(&self) -> *mut Cfgr3 { 
        (self.0 + 0x30) as *mut Cfgr3
    }

    #[doc="Get the *const pointer for the CFGR3 register."]
    #[inline] pub fn cfgr3_ptr(&self) -> *const Cfgr3 { 
           self.cfgr3_mut()
    }

    #[doc="Read the CFGR3 register."]
    #[inline] pub fn cfgr3(&self) -> Cfgr3 { 
        unsafe {
            read_volatile(self.cfgr3_ptr())
        }
    }

    #[doc="Write the CFGR3 register."]
    #[inline] pub fn set_cfgr3<F: FnOnce(Cfgr3) -> Cfgr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgr3_mut(), f(Cfgr3(0)));
        }
        self
    }

    #[doc="Modify the CFGR3 register."]
    #[inline] pub fn with_cfgr3<F: FnOnce(Cfgr3) -> Cfgr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgr3_mut(), f(self.cfgr3()));
        }
        self
    }

}

#[doc="Clock control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Internal High Speed clock enable"]
    #[inline] pub fn hsion(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HSION != 0"]
    #[inline] pub fn test_hsion(&self) -> bool {
        self.hsion() != 0
    }

    #[doc="Sets the HSION field."]
    #[inline] pub fn set_hsion<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Internal High Speed clock ready flag"]
    #[inline] pub fn hsirdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if HSIRDY != 0"]
    #[inline] pub fn test_hsirdy(&self) -> bool {
        self.hsirdy() != 0
    }

    #[doc="Sets the HSIRDY field."]
    #[inline] pub fn set_hsirdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Internal High Speed clock trimming"]
    #[inline] pub fn hsitrim(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1f) as u8) } // [7:3]
    }

    #[doc="Returns true if HSITRIM != 0"]
    #[inline] pub fn test_hsitrim(&self) -> bool {
        self.hsitrim() != 0
    }

    #[doc="Sets the HSITRIM field."]
    #[inline] pub fn set_hsitrim<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Internal High Speed clock Calibration"]
    #[inline] pub fn hsical(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if HSICAL != 0"]
    #[inline] pub fn test_hsical(&self) -> bool {
        self.hsical() != 0
    }

    #[doc="Sets the HSICAL field."]
    #[inline] pub fn set_hsical<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="External High Speed clock enable"]
    #[inline] pub fn hseon(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if HSEON != 0"]
    #[inline] pub fn test_hseon(&self) -> bool {
        self.hseon() != 0
    }

    #[doc="Sets the HSEON field."]
    #[inline] pub fn set_hseon<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="External High Speed clock ready flag"]
    #[inline] pub fn hserdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if HSERDY != 0"]
    #[inline] pub fn test_hserdy(&self) -> bool {
        self.hserdy() != 0
    }

    #[doc="Sets the HSERDY field."]
    #[inline] pub fn set_hserdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="External High Speed clock Bypass"]
    #[inline] pub fn hsebyp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if HSEBYP != 0"]
    #[inline] pub fn test_hsebyp(&self) -> bool {
        self.hsebyp() != 0
    }

    #[doc="Sets the HSEBYP field."]
    #[inline] pub fn set_hsebyp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Clock Security System enable"]
    #[inline] pub fn csson(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if CSSON != 0"]
    #[inline] pub fn test_csson(&self) -> bool {
        self.csson() != 0
    }

    #[doc="Sets the CSSON field."]
    #[inline] pub fn set_csson<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="PLL enable"]
    #[inline] pub fn pllon(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PLLON != 0"]
    #[inline] pub fn test_pllon(&self) -> bool {
        self.pllon() != 0
    }

    #[doc="Sets the PLLON field."]
    #[inline] pub fn set_pllon<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="PLL clock ready flag"]
    #[inline] pub fn pllrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PLLRDY != 0"]
    #[inline] pub fn test_pllrdy(&self) -> bool {
        self.pllrdy() != 0
    }

    #[doc="Sets the PLLRDY field."]
    #[inline] pub fn set_pllrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

}

impl From<u32> for Cr {
    #[inline]
    fn from(other: u32) -> Self {
         Cr(other)
    }
}

impl ::core::fmt::Display for Cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hsion() != 0 { try!(write!(f, " hsion"))}
        if self.hsirdy() != 0 { try!(write!(f, " hsirdy"))}
        if self.hsitrim() != 0 { try!(write!(f, " hsitrim=0x{:x}", self.hsitrim()))}
        if self.hsical() != 0 { try!(write!(f, " hsical=0x{:x}", self.hsical()))}
        if self.hseon() != 0 { try!(write!(f, " hseon"))}
        if self.hserdy() != 0 { try!(write!(f, " hserdy"))}
        if self.hsebyp() != 0 { try!(write!(f, " hsebyp"))}
        if self.csson() != 0 { try!(write!(f, " csson"))}
        if self.pllon() != 0 { try!(write!(f, " pllon"))}
        if self.pllrdy() != 0 { try!(write!(f, " pllrdy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock configuration register (RCC_CFGR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgr(pub u32);
impl Cfgr {
    #[doc="System clock Switch"]
    #[inline] pub fn sw(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SW != 0"]
    #[inline] pub fn test_sw(&self) -> bool {
        self.sw() != 0
    }

    #[doc="Sets the SW field."]
    #[inline] pub fn set_sw<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="System Clock Switch Status"]
    #[inline] pub fn sws(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if SWS != 0"]
    #[inline] pub fn test_sws(&self) -> bool {
        self.sws() != 0
    }

    #[doc="Sets the SWS field."]
    #[inline] pub fn set_sws<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="AHB prescaler"]
    #[inline] pub fn hpre(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if HPRE != 0"]
    #[inline] pub fn test_hpre(&self) -> bool {
        self.hpre() != 0
    }

    #[doc="Sets the HPRE field."]
    #[inline] pub fn set_hpre<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="APB Low speed prescaler (APB1)"]
    #[inline] pub fn ppre1(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if PPRE1 != 0"]
    #[inline] pub fn test_ppre1(&self) -> bool {
        self.ppre1() != 0
    }

    #[doc="Sets the PPRE1 field."]
    #[inline] pub fn set_ppre1<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="APB high speed prescaler (APB2)"]
    #[inline] pub fn ppre2(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x7) as u8) } // [13:11]
    }

    #[doc="Returns true if PPRE2 != 0"]
    #[inline] pub fn test_ppre2(&self) -> bool {
        self.ppre2() != 0
    }

    #[doc="Sets the PPRE2 field."]
    #[inline] pub fn set_ppre2<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="PLL entry clock source"]
    #[inline] pub fn pllsrc(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x3) as u8) } // [16:15]
    }

    #[doc="Returns true if PLLSRC != 0"]
    #[inline] pub fn test_pllsrc(&self) -> bool {
        self.pllsrc() != 0
    }

    #[doc="Sets the PLLSRC field."]
    #[inline] pub fn set_pllsrc<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="HSE divider for PLL entry"]
    #[inline] pub fn pllxtpre(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PLLXTPRE != 0"]
    #[inline] pub fn test_pllxtpre(&self) -> bool {
        self.pllxtpre() != 0
    }

    #[doc="Sets the PLLXTPRE field."]
    #[inline] pub fn set_pllxtpre<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="PLL Multiplication Factor"]
    #[inline] pub fn pllmul(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0xf) as u8) } // [21:18]
    }

    #[doc="Returns true if PLLMUL != 0"]
    #[inline] pub fn test_pllmul(&self) -> bool {
        self.pllmul() != 0
    }

    #[doc="Sets the PLLMUL field."]
    #[inline] pub fn set_pllmul<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="USB prescaler"]
    #[inline] pub fn usbpres(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if USBPRES != 0"]
    #[inline] pub fn test_usbpres(&self) -> bool {
        self.usbpres() != 0
    }

    #[doc="Sets the USBPRES field."]
    #[inline] pub fn set_usbpres<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Microcontroller clock output"]
    #[inline] pub fn mco(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if MCO != 0"]
    #[inline] pub fn test_mco(&self) -> bool {
        self.mco() != 0
    }

    #[doc="Sets the MCO field."]
    #[inline] pub fn set_mco<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Microcontroller Clock Output Flag"]
    #[inline] pub fn mcof(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if MCOF != 0"]
    #[inline] pub fn test_mcof(&self) -> bool {
        self.mcof() != 0
    }

    #[doc="Sets the MCOF field."]
    #[inline] pub fn set_mcof<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Microcontroller clock output prescaler"]
    #[inline] pub fn mcopre(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x7) as u8) } // [31:29]
    }

    #[doc="Returns true if MCOPRE != 0"]
    #[inline] pub fn test_mcopre(&self) -> bool {
        self.mcopre() != 0
    }

    #[doc="Sets the MCOPRE field."]
    #[inline] pub fn set_mcopre<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="I2S external clock source selection"]
    #[inline] pub fn i2ssrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if I2SSRC != 0"]
    #[inline] pub fn test_i2ssrc(&self) -> bool {
        self.i2ssrc() != 0
    }

    #[doc="Sets the I2SSRC field."]
    #[inline] pub fn set_i2ssrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

}

impl From<u32> for Cfgr {
    #[inline]
    fn from(other: u32) -> Self {
         Cfgr(other)
    }
}

impl ::core::fmt::Display for Cfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sw() != 0 { try!(write!(f, " sw=0x{:x}", self.sw()))}
        if self.sws() != 0 { try!(write!(f, " sws=0x{:x}", self.sws()))}
        if self.hpre() != 0 { try!(write!(f, " hpre=0x{:x}", self.hpre()))}
        if self.ppre1() != 0 { try!(write!(f, " ppre1=0x{:x}", self.ppre1()))}
        if self.ppre2() != 0 { try!(write!(f, " ppre2=0x{:x}", self.ppre2()))}
        if self.pllsrc() != 0 { try!(write!(f, " pllsrc=0x{:x}", self.pllsrc()))}
        if self.pllxtpre() != 0 { try!(write!(f, " pllxtpre"))}
        if self.pllmul() != 0 { try!(write!(f, " pllmul=0x{:x}", self.pllmul()))}
        if self.usbpres() != 0 { try!(write!(f, " usbpres"))}
        if self.mco() != 0 { try!(write!(f, " mco=0x{:x}", self.mco()))}
        if self.mcof() != 0 { try!(write!(f, " mcof"))}
        if self.mcopre() != 0 { try!(write!(f, " mcopre=0x{:x}", self.mcopre()))}
        if self.i2ssrc() != 0 { try!(write!(f, " i2ssrc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock interrupt register (RCC_CIR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cir(pub u32);
impl Cir {
    #[doc="LSI Ready Interrupt flag"]
    #[inline] pub fn lsirdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LSIRDYF != 0"]
    #[inline] pub fn test_lsirdyf(&self) -> bool {
        self.lsirdyf() != 0
    }

    #[doc="Sets the LSIRDYF field."]
    #[inline] pub fn set_lsirdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="LSE Ready Interrupt flag"]
    #[inline] pub fn lserdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LSERDYF != 0"]
    #[inline] pub fn test_lserdyf(&self) -> bool {
        self.lserdyf() != 0
    }

    #[doc="Sets the LSERDYF field."]
    #[inline] pub fn set_lserdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="HSI Ready Interrupt flag"]
    #[inline] pub fn hsirdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if HSIRDYF != 0"]
    #[inline] pub fn test_hsirdyf(&self) -> bool {
        self.hsirdyf() != 0
    }

    #[doc="Sets the HSIRDYF field."]
    #[inline] pub fn set_hsirdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="HSE Ready Interrupt flag"]
    #[inline] pub fn hserdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if HSERDYF != 0"]
    #[inline] pub fn test_hserdyf(&self) -> bool {
        self.hserdyf() != 0
    }

    #[doc="Sets the HSERDYF field."]
    #[inline] pub fn set_hserdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="PLL Ready Interrupt flag"]
    #[inline] pub fn pllrdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PLLRDYF != 0"]
    #[inline] pub fn test_pllrdyf(&self) -> bool {
        self.pllrdyf() != 0
    }

    #[doc="Sets the PLLRDYF field."]
    #[inline] pub fn set_pllrdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Clock Security System Interrupt flag"]
    #[inline] pub fn cssf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CSSF != 0"]
    #[inline] pub fn test_cssf(&self) -> bool {
        self.cssf() != 0
    }

    #[doc="Sets the CSSF field."]
    #[inline] pub fn set_cssf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="LSI Ready Interrupt Enable"]
    #[inline] pub fn lsirdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if LSIRDYIE != 0"]
    #[inline] pub fn test_lsirdyie(&self) -> bool {
        self.lsirdyie() != 0
    }

    #[doc="Sets the LSIRDYIE field."]
    #[inline] pub fn set_lsirdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="LSE Ready Interrupt Enable"]
    #[inline] pub fn lserdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LSERDYIE != 0"]
    #[inline] pub fn test_lserdyie(&self) -> bool {
        self.lserdyie() != 0
    }

    #[doc="Sets the LSERDYIE field."]
    #[inline] pub fn set_lserdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="HSI Ready Interrupt Enable"]
    #[inline] pub fn hsirdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if HSIRDYIE != 0"]
    #[inline] pub fn test_hsirdyie(&self) -> bool {
        self.hsirdyie() != 0
    }

    #[doc="Sets the HSIRDYIE field."]
    #[inline] pub fn set_hsirdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="HSE Ready Interrupt Enable"]
    #[inline] pub fn hserdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if HSERDYIE != 0"]
    #[inline] pub fn test_hserdyie(&self) -> bool {
        self.hserdyie() != 0
    }

    #[doc="Sets the HSERDYIE field."]
    #[inline] pub fn set_hserdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="PLL Ready Interrupt Enable"]
    #[inline] pub fn pllrdyie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PLLRDYIE != 0"]
    #[inline] pub fn test_pllrdyie(&self) -> bool {
        self.pllrdyie() != 0
    }

    #[doc="Sets the PLLRDYIE field."]
    #[inline] pub fn set_pllrdyie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="LSI Ready Interrupt Clear"]
    #[inline] pub fn lsirdyc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if LSIRDYC != 0"]
    #[inline] pub fn test_lsirdyc(&self) -> bool {
        self.lsirdyc() != 0
    }

    #[doc="Sets the LSIRDYC field."]
    #[inline] pub fn set_lsirdyc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="LSE Ready Interrupt Clear"]
    #[inline] pub fn lserdyc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if LSERDYC != 0"]
    #[inline] pub fn test_lserdyc(&self) -> bool {
        self.lserdyc() != 0
    }

    #[doc="Sets the LSERDYC field."]
    #[inline] pub fn set_lserdyc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="HSI Ready Interrupt Clear"]
    #[inline] pub fn hsirdyc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if HSIRDYC != 0"]
    #[inline] pub fn test_hsirdyc(&self) -> bool {
        self.hsirdyc() != 0
    }

    #[doc="Sets the HSIRDYC field."]
    #[inline] pub fn set_hsirdyc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="HSE Ready Interrupt Clear"]
    #[inline] pub fn hserdyc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if HSERDYC != 0"]
    #[inline] pub fn test_hserdyc(&self) -> bool {
        self.hserdyc() != 0
    }

    #[doc="Sets the HSERDYC field."]
    #[inline] pub fn set_hserdyc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="PLL Ready Interrupt Clear"]
    #[inline] pub fn pllrdyc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PLLRDYC != 0"]
    #[inline] pub fn test_pllrdyc(&self) -> bool {
        self.pllrdyc() != 0
    }

    #[doc="Sets the PLLRDYC field."]
    #[inline] pub fn set_pllrdyc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Clock security system interrupt clear"]
    #[inline] pub fn cssc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if CSSC != 0"]
    #[inline] pub fn test_cssc(&self) -> bool {
        self.cssc() != 0
    }

    #[doc="Sets the CSSC field."]
    #[inline] pub fn set_cssc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

}

impl From<u32> for Cir {
    #[inline]
    fn from(other: u32) -> Self {
         Cir(other)
    }
}

impl ::core::fmt::Display for Cir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lsirdyf() != 0 { try!(write!(f, " lsirdyf"))}
        if self.lserdyf() != 0 { try!(write!(f, " lserdyf"))}
        if self.hsirdyf() != 0 { try!(write!(f, " hsirdyf"))}
        if self.hserdyf() != 0 { try!(write!(f, " hserdyf"))}
        if self.pllrdyf() != 0 { try!(write!(f, " pllrdyf"))}
        if self.cssf() != 0 { try!(write!(f, " cssf"))}
        if self.lsirdyie() != 0 { try!(write!(f, " lsirdyie"))}
        if self.lserdyie() != 0 { try!(write!(f, " lserdyie"))}
        if self.hsirdyie() != 0 { try!(write!(f, " hsirdyie"))}
        if self.hserdyie() != 0 { try!(write!(f, " hserdyie"))}
        if self.pllrdyie() != 0 { try!(write!(f, " pllrdyie"))}
        if self.lsirdyc() != 0 { try!(write!(f, " lsirdyc"))}
        if self.lserdyc() != 0 { try!(write!(f, " lserdyc"))}
        if self.hsirdyc() != 0 { try!(write!(f, " hsirdyc"))}
        if self.hserdyc() != 0 { try!(write!(f, " hserdyc"))}
        if self.pllrdyc() != 0 { try!(write!(f, " pllrdyc"))}
        if self.cssc() != 0 { try!(write!(f, " cssc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB2 peripheral reset register (RCC_APB2RSTR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb2rstr(pub u32);
impl Apb2rstr {
    #[doc="SYSCFG and COMP reset"]
    #[inline] pub fn syscfgrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SYSCFGRST != 0"]
    #[inline] pub fn test_syscfgrst(&self) -> bool {
        self.syscfgrst() != 0
    }

    #[doc="Sets the SYSCFGRST field."]
    #[inline] pub fn set_syscfgrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="TIM1 timer reset"]
    #[inline] pub fn tim1rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TIM1RST != 0"]
    #[inline] pub fn test_tim1rst(&self) -> bool {
        self.tim1rst() != 0
    }

    #[doc="Sets the TIM1RST field."]
    #[inline] pub fn set_tim1rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="SPI 1 reset"]
    #[inline] pub fn spi1rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SPI1RST != 0"]
    #[inline] pub fn test_spi1rst(&self) -> bool {
        self.spi1rst() != 0
    }

    #[doc="Sets the SPI1RST field."]
    #[inline] pub fn set_spi1rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="TIM8 timer reset"]
    #[inline] pub fn tim8rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TIM8RST != 0"]
    #[inline] pub fn test_tim8rst(&self) -> bool {
        self.tim8rst() != 0
    }

    #[doc="Sets the TIM8RST field."]
    #[inline] pub fn set_tim8rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="USART1 reset"]
    #[inline] pub fn usart1rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if USART1RST != 0"]
    #[inline] pub fn test_usart1rst(&self) -> bool {
        self.usart1rst() != 0
    }

    #[doc="Sets the USART1RST field."]
    #[inline] pub fn set_usart1rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="TIM15 timer reset"]
    #[inline] pub fn tim15rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TIM15RST != 0"]
    #[inline] pub fn test_tim15rst(&self) -> bool {
        self.tim15rst() != 0
    }

    #[doc="Sets the TIM15RST field."]
    #[inline] pub fn set_tim15rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="TIM16 timer reset"]
    #[inline] pub fn tim16rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TIM16RST != 0"]
    #[inline] pub fn test_tim16rst(&self) -> bool {
        self.tim16rst() != 0
    }

    #[doc="Sets the TIM16RST field."]
    #[inline] pub fn set_tim16rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="TIM17 timer reset"]
    #[inline] pub fn tim17rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TIM17RST != 0"]
    #[inline] pub fn test_tim17rst(&self) -> bool {
        self.tim17rst() != 0
    }

    #[doc="Sets the TIM17RST field."]
    #[inline] pub fn set_tim17rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

}

impl From<u32> for Apb2rstr {
    #[inline]
    fn from(other: u32) -> Self {
         Apb2rstr(other)
    }
}

impl ::core::fmt::Display for Apb2rstr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apb2rstr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.syscfgrst() != 0 { try!(write!(f, " syscfgrst"))}
        if self.tim1rst() != 0 { try!(write!(f, " tim1rst"))}
        if self.spi1rst() != 0 { try!(write!(f, " spi1rst"))}
        if self.tim8rst() != 0 { try!(write!(f, " tim8rst"))}
        if self.usart1rst() != 0 { try!(write!(f, " usart1rst"))}
        if self.tim15rst() != 0 { try!(write!(f, " tim15rst"))}
        if self.tim16rst() != 0 { try!(write!(f, " tim16rst"))}
        if self.tim17rst() != 0 { try!(write!(f, " tim17rst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB1 peripheral reset register (RCC_APB1RSTR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1rstr(pub u32);
impl Apb1rstr {
    #[doc="Timer 2 reset"]
    #[inline] pub fn tim2rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TIM2RST != 0"]
    #[inline] pub fn test_tim2rst(&self) -> bool {
        self.tim2rst() != 0
    }

    #[doc="Sets the TIM2RST field."]
    #[inline] pub fn set_tim2rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Timer 3 reset"]
    #[inline] pub fn tim3rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TIM3RST != 0"]
    #[inline] pub fn test_tim3rst(&self) -> bool {
        self.tim3rst() != 0
    }

    #[doc="Sets the TIM3RST field."]
    #[inline] pub fn set_tim3rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Timer 14 reset"]
    #[inline] pub fn tim4rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TIM4RST != 0"]
    #[inline] pub fn test_tim4rst(&self) -> bool {
        self.tim4rst() != 0
    }

    #[doc="Sets the TIM4RST field."]
    #[inline] pub fn set_tim4rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Timer 6 reset"]
    #[inline] pub fn tim6rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TIM6RST != 0"]
    #[inline] pub fn test_tim6rst(&self) -> bool {
        self.tim6rst() != 0
    }

    #[doc="Sets the TIM6RST field."]
    #[inline] pub fn set_tim6rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Timer 7 reset"]
    #[inline] pub fn tim7rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TIM7RST != 0"]
    #[inline] pub fn test_tim7rst(&self) -> bool {
        self.tim7rst() != 0
    }

    #[doc="Sets the TIM7RST field."]
    #[inline] pub fn set_tim7rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Window watchdog reset"]
    #[inline] pub fn wwdgrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if WWDGRST != 0"]
    #[inline] pub fn test_wwdgrst(&self) -> bool {
        self.wwdgrst() != 0
    }

    #[doc="Sets the WWDGRST field."]
    #[inline] pub fn set_wwdgrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="SPI2 reset"]
    #[inline] pub fn spi2rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if SPI2RST != 0"]
    #[inline] pub fn test_spi2rst(&self) -> bool {
        self.spi2rst() != 0
    }

    #[doc="Sets the SPI2RST field."]
    #[inline] pub fn set_spi2rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="SPI3 reset"]
    #[inline] pub fn spi3rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if SPI3RST != 0"]
    #[inline] pub fn test_spi3rst(&self) -> bool {
        self.spi3rst() != 0
    }

    #[doc="Sets the SPI3RST field."]
    #[inline] pub fn set_spi3rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="USART 2 reset"]
    #[inline] pub fn usart2rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if USART2RST != 0"]
    #[inline] pub fn test_usart2rst(&self) -> bool {
        self.usart2rst() != 0
    }

    #[doc="Sets the USART2RST field."]
    #[inline] pub fn set_usart2rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="USART3 reset"]
    #[inline] pub fn usart3rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if USART3RST != 0"]
    #[inline] pub fn test_usart3rst(&self) -> bool {
        self.usart3rst() != 0
    }

    #[doc="Sets the USART3RST field."]
    #[inline] pub fn set_usart3rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="UART 4 reset"]
    #[inline] pub fn uart4rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if UART4RST != 0"]
    #[inline] pub fn test_uart4rst(&self) -> bool {
        self.uart4rst() != 0
    }

    #[doc="Sets the UART4RST field."]
    #[inline] pub fn set_uart4rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="UART 5 reset"]
    #[inline] pub fn uart5rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if UART5RST != 0"]
    #[inline] pub fn test_uart5rst(&self) -> bool {
        self.uart5rst() != 0
    }

    #[doc="Sets the UART5RST field."]
    #[inline] pub fn set_uart5rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="I2C1 reset"]
    #[inline] pub fn i2c1rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if I2C1RST != 0"]
    #[inline] pub fn test_i2c1rst(&self) -> bool {
        self.i2c1rst() != 0
    }

    #[doc="Sets the I2C1RST field."]
    #[inline] pub fn set_i2c1rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="I2C2 reset"]
    #[inline] pub fn i2c2rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if I2C2RST != 0"]
    #[inline] pub fn test_i2c2rst(&self) -> bool {
        self.i2c2rst() != 0
    }

    #[doc="Sets the I2C2RST field."]
    #[inline] pub fn set_i2c2rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="USB reset"]
    #[inline] pub fn usbrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if USBRST != 0"]
    #[inline] pub fn test_usbrst(&self) -> bool {
        self.usbrst() != 0
    }

    #[doc="Sets the USBRST field."]
    #[inline] pub fn set_usbrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="CAN reset"]
    #[inline] pub fn canrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if CANRST != 0"]
    #[inline] pub fn test_canrst(&self) -> bool {
        self.canrst() != 0
    }

    #[doc="Sets the CANRST field."]
    #[inline] pub fn set_canrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Power interface reset"]
    #[inline] pub fn pwrrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PWRRST != 0"]
    #[inline] pub fn test_pwrrst(&self) -> bool {
        self.pwrrst() != 0
    }

    #[doc="Sets the PWRRST field."]
    #[inline] pub fn set_pwrrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="DAC interface reset"]
    #[inline] pub fn dacrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if DACRST != 0"]
    #[inline] pub fn test_dacrst(&self) -> bool {
        self.dacrst() != 0
    }

    #[doc="Sets the DACRST field."]
    #[inline] pub fn set_dacrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

}

impl From<u32> for Apb1rstr {
    #[inline]
    fn from(other: u32) -> Self {
         Apb1rstr(other)
    }
}

impl ::core::fmt::Display for Apb1rstr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apb1rstr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tim2rst() != 0 { try!(write!(f, " tim2rst"))}
        if self.tim3rst() != 0 { try!(write!(f, " tim3rst"))}
        if self.tim4rst() != 0 { try!(write!(f, " tim4rst"))}
        if self.tim6rst() != 0 { try!(write!(f, " tim6rst"))}
        if self.tim7rst() != 0 { try!(write!(f, " tim7rst"))}
        if self.wwdgrst() != 0 { try!(write!(f, " wwdgrst"))}
        if self.spi2rst() != 0 { try!(write!(f, " spi2rst"))}
        if self.spi3rst() != 0 { try!(write!(f, " spi3rst"))}
        if self.usart2rst() != 0 { try!(write!(f, " usart2rst"))}
        if self.usart3rst() != 0 { try!(write!(f, " usart3rst"))}
        if self.uart4rst() != 0 { try!(write!(f, " uart4rst"))}
        if self.uart5rst() != 0 { try!(write!(f, " uart5rst"))}
        if self.i2c1rst() != 0 { try!(write!(f, " i2c1rst"))}
        if self.i2c2rst() != 0 { try!(write!(f, " i2c2rst"))}
        if self.usbrst() != 0 { try!(write!(f, " usbrst"))}
        if self.canrst() != 0 { try!(write!(f, " canrst"))}
        if self.pwrrst() != 0 { try!(write!(f, " pwrrst"))}
        if self.dacrst() != 0 { try!(write!(f, " dacrst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB Peripheral Clock enable register (RCC_AHBENR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahbenr(pub u32);
impl Ahbenr {
    #[doc="DMA1 clock enable"]
    #[inline] pub fn dmaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DMAEN != 0"]
    #[inline] pub fn test_dmaen(&self) -> bool {
        self.dmaen() != 0
    }

    #[doc="Sets the DMAEN field."]
    #[inline] pub fn set_dmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DMA2 clock enable"]
    #[inline] pub fn dma2en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DMA2EN != 0"]
    #[inline] pub fn test_dma2en(&self) -> bool {
        self.dma2en() != 0
    }

    #[doc="Sets the DMA2EN field."]
    #[inline] pub fn set_dma2en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="SRAM interface clock enable"]
    #[inline] pub fn sramen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SRAMEN != 0"]
    #[inline] pub fn test_sramen(&self) -> bool {
        self.sramen() != 0
    }

    #[doc="Sets the SRAMEN field."]
    #[inline] pub fn set_sramen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="FLITF clock enable"]
    #[inline] pub fn flitfen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if FLITFEN != 0"]
    #[inline] pub fn test_flitfen(&self) -> bool {
        self.flitfen() != 0
    }

    #[doc="Sets the FLITFEN field."]
    #[inline] pub fn set_flitfen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="CRC clock enable"]
    #[inline] pub fn crcen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CRCEN != 0"]
    #[inline] pub fn test_crcen(&self) -> bool {
        self.crcen() != 0
    }

    #[doc="Sets the CRCEN field."]
    #[inline] pub fn set_crcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="I/O port H clock enable"]
    #[inline] pub fn iophen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if IOPHEN != 0"]
    #[inline] pub fn test_iophen(&self) -> bool {
        self.iophen() != 0
    }

    #[doc="Sets the IOPHEN field."]
    #[inline] pub fn set_iophen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="I/O port A clock enable"]
    #[inline] pub fn iopaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if IOPAEN != 0"]
    #[inline] pub fn test_iopaen(&self) -> bool {
        self.iopaen() != 0
    }

    #[doc="Sets the IOPAEN field."]
    #[inline] pub fn set_iopaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="I/O port B clock enable"]
    #[inline] pub fn iopben(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if IOPBEN != 0"]
    #[inline] pub fn test_iopben(&self) -> bool {
        self.iopben() != 0
    }

    #[doc="Sets the IOPBEN field."]
    #[inline] pub fn set_iopben<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="I/O port C clock enable"]
    #[inline] pub fn iopcen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if IOPCEN != 0"]
    #[inline] pub fn test_iopcen(&self) -> bool {
        self.iopcen() != 0
    }

    #[doc="Sets the IOPCEN field."]
    #[inline] pub fn set_iopcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="I/O port D clock enable"]
    #[inline] pub fn iopden(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if IOPDEN != 0"]
    #[inline] pub fn test_iopden(&self) -> bool {
        self.iopden() != 0
    }

    #[doc="Sets the IOPDEN field."]
    #[inline] pub fn set_iopden<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="I/O port E clock enable"]
    #[inline] pub fn iopeen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if IOPEEN != 0"]
    #[inline] pub fn test_iopeen(&self) -> bool {
        self.iopeen() != 0
    }

    #[doc="Sets the IOPEEN field."]
    #[inline] pub fn set_iopeen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="I/O port F clock enable"]
    #[inline] pub fn iopfen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if IOPFEN != 0"]
    #[inline] pub fn test_iopfen(&self) -> bool {
        self.iopfen() != 0
    }

    #[doc="Sets the IOPFEN field."]
    #[inline] pub fn set_iopfen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="I/O port G clock enable"]
    #[inline] pub fn iopgen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if IOPGEN != 0"]
    #[inline] pub fn test_iopgen(&self) -> bool {
        self.iopgen() != 0
    }

    #[doc="Sets the IOPGEN field."]
    #[inline] pub fn set_iopgen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Touch sensing controller clock enable"]
    #[inline] pub fn tscen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if TSCEN != 0"]
    #[inline] pub fn test_tscen(&self) -> bool {
        self.tscen() != 0
    }

    #[doc="Sets the TSCEN field."]
    #[inline] pub fn set_tscen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="ADC1 and ADC2 clock enable"]
    #[inline] pub fn adc12en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if ADC12EN != 0"]
    #[inline] pub fn test_adc12en(&self) -> bool {
        self.adc12en() != 0
    }

    #[doc="Sets the ADC12EN field."]
    #[inline] pub fn set_adc12en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="ADC3 and ADC4 clock enable"]
    #[inline] pub fn adc34en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if ADC34EN != 0"]
    #[inline] pub fn test_adc34en(&self) -> bool {
        self.adc34en() != 0
    }

    #[doc="Sets the ADC34EN field."]
    #[inline] pub fn set_adc34en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

}

impl From<u32> for Ahbenr {
    #[inline]
    fn from(other: u32) -> Self {
         Ahbenr(other)
    }
}

impl ::core::fmt::Display for Ahbenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ahbenr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dmaen() != 0 { try!(write!(f, " dmaen"))}
        if self.dma2en() != 0 { try!(write!(f, " dma2en"))}
        if self.sramen() != 0 { try!(write!(f, " sramen"))}
        if self.flitfen() != 0 { try!(write!(f, " flitfen"))}
        if self.crcen() != 0 { try!(write!(f, " crcen"))}
        if self.iophen() != 0 { try!(write!(f, " iophen"))}
        if self.iopaen() != 0 { try!(write!(f, " iopaen"))}
        if self.iopben() != 0 { try!(write!(f, " iopben"))}
        if self.iopcen() != 0 { try!(write!(f, " iopcen"))}
        if self.iopden() != 0 { try!(write!(f, " iopden"))}
        if self.iopeen() != 0 { try!(write!(f, " iopeen"))}
        if self.iopfen() != 0 { try!(write!(f, " iopfen"))}
        if self.iopgen() != 0 { try!(write!(f, " iopgen"))}
        if self.tscen() != 0 { try!(write!(f, " tscen"))}
        if self.adc12en() != 0 { try!(write!(f, " adc12en"))}
        if self.adc34en() != 0 { try!(write!(f, " adc34en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB2 peripheral clock enable register (RCC_APB2ENR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb2enr(pub u32);
impl Apb2enr {
    #[doc="SYSCFG clock enable"]
    #[inline] pub fn syscfgen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SYSCFGEN != 0"]
    #[inline] pub fn test_syscfgen(&self) -> bool {
        self.syscfgen() != 0
    }

    #[doc="Sets the SYSCFGEN field."]
    #[inline] pub fn set_syscfgen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="TIM1 Timer clock enable"]
    #[inline] pub fn tim1en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TIM1EN != 0"]
    #[inline] pub fn test_tim1en(&self) -> bool {
        self.tim1en() != 0
    }

    #[doc="Sets the TIM1EN field."]
    #[inline] pub fn set_tim1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="SPI 1 clock enable"]
    #[inline] pub fn spi1en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SPI1EN != 0"]
    #[inline] pub fn test_spi1en(&self) -> bool {
        self.spi1en() != 0
    }

    #[doc="Sets the SPI1EN field."]
    #[inline] pub fn set_spi1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="TIM8 Timer clock enable"]
    #[inline] pub fn tim8en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if TIM8EN != 0"]
    #[inline] pub fn test_tim8en(&self) -> bool {
        self.tim8en() != 0
    }

    #[doc="Sets the TIM8EN field."]
    #[inline] pub fn set_tim8en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="USART1 clock enable"]
    #[inline] pub fn usart1en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if USART1EN != 0"]
    #[inline] pub fn test_usart1en(&self) -> bool {
        self.usart1en() != 0
    }

    #[doc="Sets the USART1EN field."]
    #[inline] pub fn set_usart1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="TIM15 timer clock enable"]
    #[inline] pub fn tim15en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if TIM15EN != 0"]
    #[inline] pub fn test_tim15en(&self) -> bool {
        self.tim15en() != 0
    }

    #[doc="Sets the TIM15EN field."]
    #[inline] pub fn set_tim15en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="TIM16 timer clock enable"]
    #[inline] pub fn tim16en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TIM16EN != 0"]
    #[inline] pub fn test_tim16en(&self) -> bool {
        self.tim16en() != 0
    }

    #[doc="Sets the TIM16EN field."]
    #[inline] pub fn set_tim16en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="TIM17 timer clock enable"]
    #[inline] pub fn tim17en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TIM17EN != 0"]
    #[inline] pub fn test_tim17en(&self) -> bool {
        self.tim17en() != 0
    }

    #[doc="Sets the TIM17EN field."]
    #[inline] pub fn set_tim17en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="TIM20 timer clock enable"]
    #[inline] pub fn tim20en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if TIM20EN != 0"]
    #[inline] pub fn test_tim20en(&self) -> bool {
        self.tim20en() != 0
    }

    #[doc="Sets the TIM20EN field."]
    #[inline] pub fn set_tim20en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

}

impl From<u32> for Apb2enr {
    #[inline]
    fn from(other: u32) -> Self {
         Apb2enr(other)
    }
}

impl ::core::fmt::Display for Apb2enr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apb2enr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.syscfgen() != 0 { try!(write!(f, " syscfgen"))}
        if self.tim1en() != 0 { try!(write!(f, " tim1en"))}
        if self.spi1en() != 0 { try!(write!(f, " spi1en"))}
        if self.tim8en() != 0 { try!(write!(f, " tim8en"))}
        if self.usart1en() != 0 { try!(write!(f, " usart1en"))}
        if self.tim15en() != 0 { try!(write!(f, " tim15en"))}
        if self.tim16en() != 0 { try!(write!(f, " tim16en"))}
        if self.tim17en() != 0 { try!(write!(f, " tim17en"))}
        if self.tim20en() != 0 { try!(write!(f, " tim20en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="APB1 peripheral clock enable register (RCC_APB1ENR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apb1enr(pub u32);
impl Apb1enr {
    #[doc="Timer 2 clock enable"]
    #[inline] pub fn tim2en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TIM2EN != 0"]
    #[inline] pub fn test_tim2en(&self) -> bool {
        self.tim2en() != 0
    }

    #[doc="Sets the TIM2EN field."]
    #[inline] pub fn set_tim2en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Timer 3 clock enable"]
    #[inline] pub fn tim3en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TIM3EN != 0"]
    #[inline] pub fn test_tim3en(&self) -> bool {
        self.tim3en() != 0
    }

    #[doc="Sets the TIM3EN field."]
    #[inline] pub fn set_tim3en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Timer 4 clock enable"]
    #[inline] pub fn tim4en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TIM4EN != 0"]
    #[inline] pub fn test_tim4en(&self) -> bool {
        self.tim4en() != 0
    }

    #[doc="Sets the TIM4EN field."]
    #[inline] pub fn set_tim4en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Timer 6 clock enable"]
    #[inline] pub fn tim6en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TIM6EN != 0"]
    #[inline] pub fn test_tim6en(&self) -> bool {
        self.tim6en() != 0
    }

    #[doc="Sets the TIM6EN field."]
    #[inline] pub fn set_tim6en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Timer 7 clock enable"]
    #[inline] pub fn tim7en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TIM7EN != 0"]
    #[inline] pub fn test_tim7en(&self) -> bool {
        self.tim7en() != 0
    }

    #[doc="Sets the TIM7EN field."]
    #[inline] pub fn set_tim7en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Window watchdog clock enable"]
    #[inline] pub fn wwdgen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if WWDGEN != 0"]
    #[inline] pub fn test_wwdgen(&self) -> bool {
        self.wwdgen() != 0
    }

    #[doc="Sets the WWDGEN field."]
    #[inline] pub fn set_wwdgen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="SPI 2 clock enable"]
    #[inline] pub fn spi2en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if SPI2EN != 0"]
    #[inline] pub fn test_spi2en(&self) -> bool {
        self.spi2en() != 0
    }

    #[doc="Sets the SPI2EN field."]
    #[inline] pub fn set_spi2en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="SPI 3 clock enable"]
    #[inline] pub fn spi3en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if SPI3EN != 0"]
    #[inline] pub fn test_spi3en(&self) -> bool {
        self.spi3en() != 0
    }

    #[doc="Sets the SPI3EN field."]
    #[inline] pub fn set_spi3en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="USART 2 clock enable"]
    #[inline] pub fn usart2en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if USART2EN != 0"]
    #[inline] pub fn test_usart2en(&self) -> bool {
        self.usart2en() != 0
    }

    #[doc="Sets the USART2EN field."]
    #[inline] pub fn set_usart2en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="USART 3 clock enable"]
    #[inline] pub fn usart3en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if USART3EN != 0"]
    #[inline] pub fn test_usart3en(&self) -> bool {
        self.usart3en() != 0
    }

    #[doc="Sets the USART3EN field."]
    #[inline] pub fn set_usart3en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="UART 4 clock enable"]
    #[inline] pub fn uart4en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if UART4EN != 0"]
    #[inline] pub fn test_uart4en(&self) -> bool {
        self.uart4en() != 0
    }

    #[doc="Sets the UART4EN field."]
    #[inline] pub fn set_uart4en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="UART 5 clock enable"]
    #[inline] pub fn uart5en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if UART5EN != 0"]
    #[inline] pub fn test_uart5en(&self) -> bool {
        self.uart5en() != 0
    }

    #[doc="Sets the UART5EN field."]
    #[inline] pub fn set_uart5en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="I2C 1 clock enable"]
    #[inline] pub fn i2c1en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if I2C1EN != 0"]
    #[inline] pub fn test_i2c1en(&self) -> bool {
        self.i2c1en() != 0
    }

    #[doc="Sets the I2C1EN field."]
    #[inline] pub fn set_i2c1en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="I2C 2 clock enable"]
    #[inline] pub fn i2c2en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if I2C2EN != 0"]
    #[inline] pub fn test_i2c2en(&self) -> bool {
        self.i2c2en() != 0
    }

    #[doc="Sets the I2C2EN field."]
    #[inline] pub fn set_i2c2en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="USB clock enable"]
    #[inline] pub fn usben(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if USBEN != 0"]
    #[inline] pub fn test_usben(&self) -> bool {
        self.usben() != 0
    }

    #[doc="Sets the USBEN field."]
    #[inline] pub fn set_usben<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="CAN clock enable"]
    #[inline] pub fn canen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if CANEN != 0"]
    #[inline] pub fn test_canen(&self) -> bool {
        self.canen() != 0
    }

    #[doc="Sets the CANEN field."]
    #[inline] pub fn set_canen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Power interface clock enable"]
    #[inline] pub fn pwren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PWREN != 0"]
    #[inline] pub fn test_pwren(&self) -> bool {
        self.pwren() != 0
    }

    #[doc="Sets the PWREN field."]
    #[inline] pub fn set_pwren<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="DAC interface clock enable"]
    #[inline] pub fn dacen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if DACEN != 0"]
    #[inline] pub fn test_dacen(&self) -> bool {
        self.dacen() != 0
    }

    #[doc="Sets the DACEN field."]
    #[inline] pub fn set_dacen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="I2C 3 clock enable"]
    #[inline] pub fn i2c3en(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if I2C3EN != 0"]
    #[inline] pub fn test_i2c3en(&self) -> bool {
        self.i2c3en() != 0
    }

    #[doc="Sets the I2C3EN field."]
    #[inline] pub fn set_i2c3en<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

}

impl From<u32> for Apb1enr {
    #[inline]
    fn from(other: u32) -> Self {
         Apb1enr(other)
    }
}

impl ::core::fmt::Display for Apb1enr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apb1enr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tim2en() != 0 { try!(write!(f, " tim2en"))}
        if self.tim3en() != 0 { try!(write!(f, " tim3en"))}
        if self.tim4en() != 0 { try!(write!(f, " tim4en"))}
        if self.tim6en() != 0 { try!(write!(f, " tim6en"))}
        if self.tim7en() != 0 { try!(write!(f, " tim7en"))}
        if self.wwdgen() != 0 { try!(write!(f, " wwdgen"))}
        if self.spi2en() != 0 { try!(write!(f, " spi2en"))}
        if self.spi3en() != 0 { try!(write!(f, " spi3en"))}
        if self.usart2en() != 0 { try!(write!(f, " usart2en"))}
        if self.usart3en() != 0 { try!(write!(f, " usart3en"))}
        if self.uart4en() != 0 { try!(write!(f, " uart4en"))}
        if self.uart5en() != 0 { try!(write!(f, " uart5en"))}
        if self.i2c1en() != 0 { try!(write!(f, " i2c1en"))}
        if self.i2c2en() != 0 { try!(write!(f, " i2c2en"))}
        if self.usben() != 0 { try!(write!(f, " usben"))}
        if self.canen() != 0 { try!(write!(f, " canen"))}
        if self.pwren() != 0 { try!(write!(f, " pwren"))}
        if self.dacen() != 0 { try!(write!(f, " dacen"))}
        if self.i2c3en() != 0 { try!(write!(f, " i2c3en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Backup domain control register (RCC_BDCR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bdcr(pub u32);
impl Bdcr {
    #[doc="External Low Speed oscillator enable"]
    #[inline] pub fn lseon(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LSEON != 0"]
    #[inline] pub fn test_lseon(&self) -> bool {
        self.lseon() != 0
    }

    #[doc="Sets the LSEON field."]
    #[inline] pub fn set_lseon<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="External Low Speed oscillator ready"]
    #[inline] pub fn lserdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LSERDY != 0"]
    #[inline] pub fn test_lserdy(&self) -> bool {
        self.lserdy() != 0
    }

    #[doc="Sets the LSERDY field."]
    #[inline] pub fn set_lserdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="External Low Speed oscillator bypass"]
    #[inline] pub fn lsebyp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if LSEBYP != 0"]
    #[inline] pub fn test_lsebyp(&self) -> bool {
        self.lsebyp() != 0
    }

    #[doc="Sets the LSEBYP field."]
    #[inline] pub fn set_lsebyp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="LSE oscillator drive capability"]
    #[inline] pub fn lsedrv(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if LSEDRV != 0"]
    #[inline] pub fn test_lsedrv(&self) -> bool {
        self.lsedrv() != 0
    }

    #[doc="Sets the LSEDRV field."]
    #[inline] pub fn set_lsedrv<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="RTC clock source selection"]
    #[inline] pub fn rtcsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if RTCSEL != 0"]
    #[inline] pub fn test_rtcsel(&self) -> bool {
        self.rtcsel() != 0
    }

    #[doc="Sets the RTCSEL field."]
    #[inline] pub fn set_rtcsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="RTC clock enable"]
    #[inline] pub fn rtcen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if RTCEN != 0"]
    #[inline] pub fn test_rtcen(&self) -> bool {
        self.rtcen() != 0
    }

    #[doc="Sets the RTCEN field."]
    #[inline] pub fn set_rtcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Backup domain software reset"]
    #[inline] pub fn bdrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if BDRST != 0"]
    #[inline] pub fn test_bdrst(&self) -> bool {
        self.bdrst() != 0
    }

    #[doc="Sets the BDRST field."]
    #[inline] pub fn set_bdrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Bdcr {
    #[inline]
    fn from(other: u32) -> Self {
         Bdcr(other)
    }
}

impl ::core::fmt::Display for Bdcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bdcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lseon() != 0 { try!(write!(f, " lseon"))}
        if self.lserdy() != 0 { try!(write!(f, " lserdy"))}
        if self.lsebyp() != 0 { try!(write!(f, " lsebyp"))}
        if self.lsedrv() != 0 { try!(write!(f, " lsedrv=0x{:x}", self.lsedrv()))}
        if self.rtcsel() != 0 { try!(write!(f, " rtcsel=0x{:x}", self.rtcsel()))}
        if self.rtcen() != 0 { try!(write!(f, " rtcen"))}
        if self.bdrst() != 0 { try!(write!(f, " bdrst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control/status register (RCC_CSR)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc="Internal low speed oscillator enable"]
    #[inline] pub fn lsion(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LSION != 0"]
    #[inline] pub fn test_lsion(&self) -> bool {
        self.lsion() != 0
    }

    #[doc="Sets the LSION field."]
    #[inline] pub fn set_lsion<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Internal low speed oscillator ready"]
    #[inline] pub fn lsirdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LSIRDY != 0"]
    #[inline] pub fn test_lsirdy(&self) -> bool {
        self.lsirdy() != 0
    }

    #[doc="Sets the LSIRDY field."]
    #[inline] pub fn set_lsirdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Remove reset flag"]
    #[inline] pub fn rmvf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if RMVF != 0"]
    #[inline] pub fn test_rmvf(&self) -> bool {
        self.rmvf() != 0
    }

    #[doc="Sets the RMVF field."]
    #[inline] pub fn set_rmvf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Option byte loader reset flag"]
    #[inline] pub fn oblrstf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if OBLRSTF != 0"]
    #[inline] pub fn test_oblrstf(&self) -> bool {
        self.oblrstf() != 0
    }

    #[doc="Sets the OBLRSTF field."]
    #[inline] pub fn set_oblrstf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="PIN reset flag"]
    #[inline] pub fn pinrstf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PINRSTF != 0"]
    #[inline] pub fn test_pinrstf(&self) -> bool {
        self.pinrstf() != 0
    }

    #[doc="Sets the PINRSTF field."]
    #[inline] pub fn set_pinrstf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="POR/PDR reset flag"]
    #[inline] pub fn porrstf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if PORRSTF != 0"]
    #[inline] pub fn test_porrstf(&self) -> bool {
        self.porrstf() != 0
    }

    #[doc="Sets the PORRSTF field."]
    #[inline] pub fn set_porrstf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Software reset flag"]
    #[inline] pub fn sftrstf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if SFTRSTF != 0"]
    #[inline] pub fn test_sftrstf(&self) -> bool {
        self.sftrstf() != 0
    }

    #[doc="Sets the SFTRSTF field."]
    #[inline] pub fn set_sftrstf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Independent watchdog reset flag"]
    #[inline] pub fn iwdgrstf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if IWDGRSTF != 0"]
    #[inline] pub fn test_iwdgrstf(&self) -> bool {
        self.iwdgrstf() != 0
    }

    #[doc="Sets the IWDGRSTF field."]
    #[inline] pub fn set_iwdgrstf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Window watchdog reset flag"]
    #[inline] pub fn wwdgrstf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if WWDGRSTF != 0"]
    #[inline] pub fn test_wwdgrstf(&self) -> bool {
        self.wwdgrstf() != 0
    }

    #[doc="Sets the WWDGRSTF field."]
    #[inline] pub fn set_wwdgrstf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Low-power reset flag"]
    #[inline] pub fn lpwrrstf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LPWRRSTF != 0"]
    #[inline] pub fn test_lpwrrstf(&self) -> bool {
        self.lpwrrstf() != 0
    }

    #[doc="Sets the LPWRRSTF field."]
    #[inline] pub fn set_lpwrrstf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Csr {
    #[inline]
    fn from(other: u32) -> Self {
         Csr(other)
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
        if self.lsion() != 0 { try!(write!(f, " lsion"))}
        if self.lsirdy() != 0 { try!(write!(f, " lsirdy"))}
        if self.rmvf() != 0 { try!(write!(f, " rmvf"))}
        if self.oblrstf() != 0 { try!(write!(f, " oblrstf"))}
        if self.pinrstf() != 0 { try!(write!(f, " pinrstf"))}
        if self.porrstf() != 0 { try!(write!(f, " porrstf"))}
        if self.sftrstf() != 0 { try!(write!(f, " sftrstf"))}
        if self.iwdgrstf() != 0 { try!(write!(f, " iwdgrstf"))}
        if self.wwdgrstf() != 0 { try!(write!(f, " wwdgrstf"))}
        if self.lpwrrstf() != 0 { try!(write!(f, " lpwrrstf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="AHB peripheral reset register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ahbrstr(pub u32);
impl Ahbrstr {
    #[doc="I/O port A reset"]
    #[inline] pub fn ioparst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if IOPARST != 0"]
    #[inline] pub fn test_ioparst(&self) -> bool {
        self.ioparst() != 0
    }

    #[doc="Sets the IOPARST field."]
    #[inline] pub fn set_ioparst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="I/O port B reset"]
    #[inline] pub fn iopbrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if IOPBRST != 0"]
    #[inline] pub fn test_iopbrst(&self) -> bool {
        self.iopbrst() != 0
    }

    #[doc="Sets the IOPBRST field."]
    #[inline] pub fn set_iopbrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="I/O port C reset"]
    #[inline] pub fn iopcrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if IOPCRST != 0"]
    #[inline] pub fn test_iopcrst(&self) -> bool {
        self.iopcrst() != 0
    }

    #[doc="Sets the IOPCRST field."]
    #[inline] pub fn set_iopcrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="I/O port D reset"]
    #[inline] pub fn iopdrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if IOPDRST != 0"]
    #[inline] pub fn test_iopdrst(&self) -> bool {
        self.iopdrst() != 0
    }

    #[doc="Sets the IOPDRST field."]
    #[inline] pub fn set_iopdrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="I/O port E reset"]
    #[inline] pub fn ioperst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if IOPERST != 0"]
    #[inline] pub fn test_ioperst(&self) -> bool {
        self.ioperst() != 0
    }

    #[doc="Sets the IOPERST field."]
    #[inline] pub fn set_ioperst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="I/O port F reset"]
    #[inline] pub fn iopfrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if IOPFRST != 0"]
    #[inline] pub fn test_iopfrst(&self) -> bool {
        self.iopfrst() != 0
    }

    #[doc="Sets the IOPFRST field."]
    #[inline] pub fn set_iopfrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Touch sensing controller reset"]
    #[inline] pub fn tscrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if TSCRST != 0"]
    #[inline] pub fn test_tscrst(&self) -> bool {
        self.tscrst() != 0
    }

    #[doc="Sets the TSCRST field."]
    #[inline] pub fn set_tscrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="ADC1 and ADC2 reset"]
    #[inline] pub fn adc12rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if ADC12RST != 0"]
    #[inline] pub fn test_adc12rst(&self) -> bool {
        self.adc12rst() != 0
    }

    #[doc="Sets the ADC12RST field."]
    #[inline] pub fn set_adc12rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="ADC3 and ADC4 reset"]
    #[inline] pub fn adc34rst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if ADC34RST != 0"]
    #[inline] pub fn test_adc34rst(&self) -> bool {
        self.adc34rst() != 0
    }

    #[doc="Sets the ADC34RST field."]
    #[inline] pub fn set_adc34rst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

}

impl From<u32> for Ahbrstr {
    #[inline]
    fn from(other: u32) -> Self {
         Ahbrstr(other)
    }
}

impl ::core::fmt::Display for Ahbrstr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ahbrstr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ioparst() != 0 { try!(write!(f, " ioparst"))}
        if self.iopbrst() != 0 { try!(write!(f, " iopbrst"))}
        if self.iopcrst() != 0 { try!(write!(f, " iopcrst"))}
        if self.iopdrst() != 0 { try!(write!(f, " iopdrst"))}
        if self.ioperst() != 0 { try!(write!(f, " ioperst"))}
        if self.iopfrst() != 0 { try!(write!(f, " iopfrst"))}
        if self.tscrst() != 0 { try!(write!(f, " tscrst"))}
        if self.adc12rst() != 0 { try!(write!(f, " adc12rst"))}
        if self.adc34rst() != 0 { try!(write!(f, " adc34rst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock configuration register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgr2(pub u32);
impl Cfgr2 {
    #[doc="PREDIV division factor"]
    #[inline] pub fn prediv(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if PREDIV != 0"]
    #[inline] pub fn test_prediv(&self) -> bool {
        self.prediv() != 0
    }

    #[doc="Sets the PREDIV field."]
    #[inline] pub fn set_prediv<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="ADC1 and ADC2 prescaler"]
    #[inline] pub fn adc12pres(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1f) as u8) } // [8:4]
    }

    #[doc="Returns true if ADC12PRES != 0"]
    #[inline] pub fn test_adc12pres(&self) -> bool {
        self.adc12pres() != 0
    }

    #[doc="Sets the ADC12PRES field."]
    #[inline] pub fn set_adc12pres<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ADC3 and ADC4 prescaler"]
    #[inline] pub fn adc34pres(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1f) as u8) } // [13:9]
    }

    #[doc="Returns true if ADC34PRES != 0"]
    #[inline] pub fn test_adc34pres(&self) -> bool {
        self.adc34pres() != 0
    }

    #[doc="Sets the ADC34PRES field."]
    #[inline] pub fn set_adc34pres<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 9);
        self.0 |= value << 9;
        self
    }

}

impl From<u32> for Cfgr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Cfgr2(other)
    }
}

impl ::core::fmt::Display for Cfgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.prediv() != 0 { try!(write!(f, " prediv=0x{:x}", self.prediv()))}
        if self.adc12pres() != 0 { try!(write!(f, " adc12pres=0x{:x}", self.adc12pres()))}
        if self.adc34pres() != 0 { try!(write!(f, " adc34pres=0x{:x}", self.adc34pres()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Clock configuration register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgr3(pub u32);
impl Cfgr3 {
    #[doc="USART1 clock source selection"]
    #[inline] pub fn usart1sw(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if USART1SW != 0"]
    #[inline] pub fn test_usart1sw(&self) -> bool {
        self.usart1sw() != 0
    }

    #[doc="Sets the USART1SW field."]
    #[inline] pub fn set_usart1sw<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="I2C1 clock source selection"]
    #[inline] pub fn i2c1sw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if I2C1SW != 0"]
    #[inline] pub fn test_i2c1sw(&self) -> bool {
        self.i2c1sw() != 0
    }

    #[doc="Sets the I2C1SW field."]
    #[inline] pub fn set_i2c1sw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="I2C2 clock source selection"]
    #[inline] pub fn i2c2sw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if I2C2SW != 0"]
    #[inline] pub fn test_i2c2sw(&self) -> bool {
        self.i2c2sw() != 0
    }

    #[doc="Sets the I2C2SW field."]
    #[inline] pub fn set_i2c2sw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="I2C3 clock source selection"]
    #[inline] pub fn i2c3sw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if I2C3SW != 0"]
    #[inline] pub fn test_i2c3sw(&self) -> bool {
        self.i2c3sw() != 0
    }

    #[doc="Sets the I2C3SW field."]
    #[inline] pub fn set_i2c3sw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="USART2 clock source selection"]
    #[inline] pub fn usart2sw(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if USART2SW != 0"]
    #[inline] pub fn test_usart2sw(&self) -> bool {
        self.usart2sw() != 0
    }

    #[doc="Sets the USART2SW field."]
    #[inline] pub fn set_usart2sw<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="USART3 clock source selection"]
    #[inline] pub fn usart3sw(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if USART3SW != 0"]
    #[inline] pub fn test_usart3sw(&self) -> bool {
        self.usart3sw() != 0
    }

    #[doc="Sets the USART3SW field."]
    #[inline] pub fn set_usart3sw<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Timer1 clock source selection"]
    #[inline] pub fn tim1sw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TIM1SW != 0"]
    #[inline] pub fn test_tim1sw(&self) -> bool {
        self.tim1sw() != 0
    }

    #[doc="Sets the TIM1SW field."]
    #[inline] pub fn set_tim1sw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Timer8 clock source selection"]
    #[inline] pub fn tim8sw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TIM8SW != 0"]
    #[inline] pub fn test_tim8sw(&self) -> bool {
        self.tim8sw() != 0
    }

    #[doc="Sets the TIM8SW field."]
    #[inline] pub fn set_tim8sw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="UART4 clock source selection"]
    #[inline] pub fn uart4sw(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if UART4SW != 0"]
    #[inline] pub fn test_uart4sw(&self) -> bool {
        self.uart4sw() != 0
    }

    #[doc="Sets the UART4SW field."]
    #[inline] pub fn set_uart4sw<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="UART5 clock source selection"]
    #[inline] pub fn uart5sw(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
    }

    #[doc="Returns true if UART5SW != 0"]
    #[inline] pub fn test_uart5sw(&self) -> bool {
        self.uart5sw() != 0
    }

    #[doc="Sets the UART5SW field."]
    #[inline] pub fn set_uart5sw<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 22);
        self.0 |= value << 22;
        self
    }

}

impl From<u32> for Cfgr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Cfgr3(other)
    }
}

impl ::core::fmt::Display for Cfgr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfgr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.usart1sw() != 0 { try!(write!(f, " usart1sw=0x{:x}", self.usart1sw()))}
        if self.i2c1sw() != 0 { try!(write!(f, " i2c1sw"))}
        if self.i2c2sw() != 0 { try!(write!(f, " i2c2sw"))}
        if self.i2c3sw() != 0 { try!(write!(f, " i2c3sw"))}
        if self.usart2sw() != 0 { try!(write!(f, " usart2sw=0x{:x}", self.usart2sw()))}
        if self.usart3sw() != 0 { try!(write!(f, " usart3sw=0x{:x}", self.usart3sw()))}
        if self.tim1sw() != 0 { try!(write!(f, " tim1sw"))}
        if self.tim8sw() != 0 { try!(write!(f, " tim8sw"))}
        if self.uart4sw() != 0 { try!(write!(f, " uart4sw=0x{:x}", self.uart4sw()))}
        if self.uart5sw() != 0 { try!(write!(f, " uart5sw=0x{:x}", self.uart5sw()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

