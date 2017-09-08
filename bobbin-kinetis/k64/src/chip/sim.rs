//! System Integration Module
#[allow(unused_imports)] use bobbin_common::*;

periph!(SIM, Sim, 0x40047000);

#[doc="System Integration Module"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sim(pub usize);
impl Sim {
    #[doc="Get the *mut pointer for the SOPT1 register."]
    #[inline] pub fn sopt1_mut(&self) -> *mut Sopt1 { 
        (self.0 + 0x0) as *mut Sopt1
    }

    #[doc="Get the *const pointer for the SOPT1 register."]
    #[inline] pub fn sopt1_ptr(&self) -> *const Sopt1 { 
           self.sopt1_mut()
    }

    #[doc="Read the SOPT1 register."]
    #[inline] pub fn sopt1(&self) -> Sopt1 { 
        unsafe {
            read_volatile(self.sopt1_ptr())
        }
    }

    #[doc="Write the SOPT1 register."]
    #[inline] pub fn set_sopt1<F: FnOnce(Sopt1) -> Sopt1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sopt1_mut(), f(Sopt1(0)));
        }
        self
    }

    #[doc="Modify the SOPT1 register."]
    #[inline] pub fn with_sopt1<F: FnOnce(Sopt1) -> Sopt1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sopt1_mut(), f(self.sopt1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SOPT1CFG register."]
    #[inline] pub fn sopt1cfg_mut(&self) -> *mut Sopt1cfg { 
        (self.0 + 0x4) as *mut Sopt1cfg
    }

    #[doc="Get the *const pointer for the SOPT1CFG register."]
    #[inline] pub fn sopt1cfg_ptr(&self) -> *const Sopt1cfg { 
           self.sopt1cfg_mut()
    }

    #[doc="Read the SOPT1CFG register."]
    #[inline] pub fn sopt1cfg(&self) -> Sopt1cfg { 
        unsafe {
            read_volatile(self.sopt1cfg_ptr())
        }
    }

    #[doc="Write the SOPT1CFG register."]
    #[inline] pub fn set_sopt1cfg<F: FnOnce(Sopt1cfg) -> Sopt1cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sopt1cfg_mut(), f(Sopt1cfg(0)));
        }
        self
    }

    #[doc="Modify the SOPT1CFG register."]
    #[inline] pub fn with_sopt1cfg<F: FnOnce(Sopt1cfg) -> Sopt1cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sopt1cfg_mut(), f(self.sopt1cfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SOPT2 register."]
    #[inline] pub fn sopt2_mut(&self) -> *mut Sopt2 { 
        (self.0 + 0x1004) as *mut Sopt2
    }

    #[doc="Get the *const pointer for the SOPT2 register."]
    #[inline] pub fn sopt2_ptr(&self) -> *const Sopt2 { 
           self.sopt2_mut()
    }

    #[doc="Read the SOPT2 register."]
    #[inline] pub fn sopt2(&self) -> Sopt2 { 
        unsafe {
            read_volatile(self.sopt2_ptr())
        }
    }

    #[doc="Write the SOPT2 register."]
    #[inline] pub fn set_sopt2<F: FnOnce(Sopt2) -> Sopt2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sopt2_mut(), f(Sopt2(0)));
        }
        self
    }

    #[doc="Modify the SOPT2 register."]
    #[inline] pub fn with_sopt2<F: FnOnce(Sopt2) -> Sopt2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sopt2_mut(), f(self.sopt2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SOPT4 register."]
    #[inline] pub fn sopt4_mut(&self) -> *mut Sopt4 { 
        (self.0 + 0x100c) as *mut Sopt4
    }

    #[doc="Get the *const pointer for the SOPT4 register."]
    #[inline] pub fn sopt4_ptr(&self) -> *const Sopt4 { 
           self.sopt4_mut()
    }

    #[doc="Read the SOPT4 register."]
    #[inline] pub fn sopt4(&self) -> Sopt4 { 
        unsafe {
            read_volatile(self.sopt4_ptr())
        }
    }

    #[doc="Write the SOPT4 register."]
    #[inline] pub fn set_sopt4<F: FnOnce(Sopt4) -> Sopt4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sopt4_mut(), f(Sopt4(0)));
        }
        self
    }

    #[doc="Modify the SOPT4 register."]
    #[inline] pub fn with_sopt4<F: FnOnce(Sopt4) -> Sopt4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sopt4_mut(), f(self.sopt4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SOPT5 register."]
    #[inline] pub fn sopt5_mut(&self) -> *mut Sopt5 { 
        (self.0 + 0x1010) as *mut Sopt5
    }

    #[doc="Get the *const pointer for the SOPT5 register."]
    #[inline] pub fn sopt5_ptr(&self) -> *const Sopt5 { 
           self.sopt5_mut()
    }

    #[doc="Read the SOPT5 register."]
    #[inline] pub fn sopt5(&self) -> Sopt5 { 
        unsafe {
            read_volatile(self.sopt5_ptr())
        }
    }

    #[doc="Write the SOPT5 register."]
    #[inline] pub fn set_sopt5<F: FnOnce(Sopt5) -> Sopt5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sopt5_mut(), f(Sopt5(0)));
        }
        self
    }

    #[doc="Modify the SOPT5 register."]
    #[inline] pub fn with_sopt5<F: FnOnce(Sopt5) -> Sopt5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sopt5_mut(), f(self.sopt5()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SOPT7 register."]
    #[inline] pub fn sopt7_mut(&self) -> *mut Sopt7 { 
        (self.0 + 0x1018) as *mut Sopt7
    }

    #[doc="Get the *const pointer for the SOPT7 register."]
    #[inline] pub fn sopt7_ptr(&self) -> *const Sopt7 { 
           self.sopt7_mut()
    }

    #[doc="Read the SOPT7 register."]
    #[inline] pub fn sopt7(&self) -> Sopt7 { 
        unsafe {
            read_volatile(self.sopt7_ptr())
        }
    }

    #[doc="Write the SOPT7 register."]
    #[inline] pub fn set_sopt7<F: FnOnce(Sopt7) -> Sopt7>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sopt7_mut(), f(Sopt7(0)));
        }
        self
    }

    #[doc="Modify the SOPT7 register."]
    #[inline] pub fn with_sopt7<F: FnOnce(Sopt7) -> Sopt7>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sopt7_mut(), f(self.sopt7()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SDID register."]
    #[inline] pub fn sdid_mut(&self) -> *mut Sdid { 
        (self.0 + 0x1024) as *mut Sdid
    }

    #[doc="Get the *const pointer for the SDID register."]
    #[inline] pub fn sdid_ptr(&self) -> *const Sdid { 
           self.sdid_mut()
    }

    #[doc="Read the SDID register."]
    #[inline] pub fn sdid(&self) -> Sdid { 
        unsafe {
            read_volatile(self.sdid_ptr())
        }
    }

    #[doc="Get the *mut pointer for the SCGC1 register."]
    #[inline] pub fn scgc1_mut(&self) -> *mut Scgc1 { 
        (self.0 + 0x1028) as *mut Scgc1
    }

    #[doc="Get the *const pointer for the SCGC1 register."]
    #[inline] pub fn scgc1_ptr(&self) -> *const Scgc1 { 
           self.scgc1_mut()
    }

    #[doc="Read the SCGC1 register."]
    #[inline] pub fn scgc1(&self) -> Scgc1 { 
        unsafe {
            read_volatile(self.scgc1_ptr())
        }
    }

    #[doc="Write the SCGC1 register."]
    #[inline] pub fn set_scgc1<F: FnOnce(Scgc1) -> Scgc1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scgc1_mut(), f(Scgc1(0)));
        }
        self
    }

    #[doc="Modify the SCGC1 register."]
    #[inline] pub fn with_scgc1<F: FnOnce(Scgc1) -> Scgc1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scgc1_mut(), f(self.scgc1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SCGC2 register."]
    #[inline] pub fn scgc2_mut(&self) -> *mut Scgc2 { 
        (self.0 + 0x102c) as *mut Scgc2
    }

    #[doc="Get the *const pointer for the SCGC2 register."]
    #[inline] pub fn scgc2_ptr(&self) -> *const Scgc2 { 
           self.scgc2_mut()
    }

    #[doc="Read the SCGC2 register."]
    #[inline] pub fn scgc2(&self) -> Scgc2 { 
        unsafe {
            read_volatile(self.scgc2_ptr())
        }
    }

    #[doc="Write the SCGC2 register."]
    #[inline] pub fn set_scgc2<F: FnOnce(Scgc2) -> Scgc2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scgc2_mut(), f(Scgc2(0)));
        }
        self
    }

    #[doc="Modify the SCGC2 register."]
    #[inline] pub fn with_scgc2<F: FnOnce(Scgc2) -> Scgc2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scgc2_mut(), f(self.scgc2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SCGC3 register."]
    #[inline] pub fn scgc3_mut(&self) -> *mut Scgc3 { 
        (self.0 + 0x1030) as *mut Scgc3
    }

    #[doc="Get the *const pointer for the SCGC3 register."]
    #[inline] pub fn scgc3_ptr(&self) -> *const Scgc3 { 
           self.scgc3_mut()
    }

    #[doc="Read the SCGC3 register."]
    #[inline] pub fn scgc3(&self) -> Scgc3 { 
        unsafe {
            read_volatile(self.scgc3_ptr())
        }
    }

    #[doc="Write the SCGC3 register."]
    #[inline] pub fn set_scgc3<F: FnOnce(Scgc3) -> Scgc3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scgc3_mut(), f(Scgc3(0)));
        }
        self
    }

    #[doc="Modify the SCGC3 register."]
    #[inline] pub fn with_scgc3<F: FnOnce(Scgc3) -> Scgc3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scgc3_mut(), f(self.scgc3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SCGC4 register."]
    #[inline] pub fn scgc4_mut(&self) -> *mut Scgc4 { 
        (self.0 + 0x1034) as *mut Scgc4
    }

    #[doc="Get the *const pointer for the SCGC4 register."]
    #[inline] pub fn scgc4_ptr(&self) -> *const Scgc4 { 
           self.scgc4_mut()
    }

    #[doc="Read the SCGC4 register."]
    #[inline] pub fn scgc4(&self) -> Scgc4 { 
        unsafe {
            read_volatile(self.scgc4_ptr())
        }
    }

    #[doc="Write the SCGC4 register."]
    #[inline] pub fn set_scgc4<F: FnOnce(Scgc4) -> Scgc4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scgc4_mut(), f(Scgc4(0)));
        }
        self
    }

    #[doc="Modify the SCGC4 register."]
    #[inline] pub fn with_scgc4<F: FnOnce(Scgc4) -> Scgc4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scgc4_mut(), f(self.scgc4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SCGC5 register."]
    #[inline] pub fn scgc5_mut(&self) -> *mut Scgc5 { 
        (self.0 + 0x1038) as *mut Scgc5
    }

    #[doc="Get the *const pointer for the SCGC5 register."]
    #[inline] pub fn scgc5_ptr(&self) -> *const Scgc5 { 
           self.scgc5_mut()
    }

    #[doc="Read the SCGC5 register."]
    #[inline] pub fn scgc5(&self) -> Scgc5 { 
        unsafe {
            read_volatile(self.scgc5_ptr())
        }
    }

    #[doc="Write the SCGC5 register."]
    #[inline] pub fn set_scgc5<F: FnOnce(Scgc5) -> Scgc5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scgc5_mut(), f(Scgc5(0)));
        }
        self
    }

    #[doc="Modify the SCGC5 register."]
    #[inline] pub fn with_scgc5<F: FnOnce(Scgc5) -> Scgc5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scgc5_mut(), f(self.scgc5()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SCGC6 register."]
    #[inline] pub fn scgc6_mut(&self) -> *mut Scgc6 { 
        (self.0 + 0x103c) as *mut Scgc6
    }

    #[doc="Get the *const pointer for the SCGC6 register."]
    #[inline] pub fn scgc6_ptr(&self) -> *const Scgc6 { 
           self.scgc6_mut()
    }

    #[doc="Read the SCGC6 register."]
    #[inline] pub fn scgc6(&self) -> Scgc6 { 
        unsafe {
            read_volatile(self.scgc6_ptr())
        }
    }

    #[doc="Write the SCGC6 register."]
    #[inline] pub fn set_scgc6<F: FnOnce(Scgc6) -> Scgc6>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scgc6_mut(), f(Scgc6(0)));
        }
        self
    }

    #[doc="Modify the SCGC6 register."]
    #[inline] pub fn with_scgc6<F: FnOnce(Scgc6) -> Scgc6>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scgc6_mut(), f(self.scgc6()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SCGC7 register."]
    #[inline] pub fn scgc7_mut(&self) -> *mut Scgc7 { 
        (self.0 + 0x1040) as *mut Scgc7
    }

    #[doc="Get the *const pointer for the SCGC7 register."]
    #[inline] pub fn scgc7_ptr(&self) -> *const Scgc7 { 
           self.scgc7_mut()
    }

    #[doc="Read the SCGC7 register."]
    #[inline] pub fn scgc7(&self) -> Scgc7 { 
        unsafe {
            read_volatile(self.scgc7_ptr())
        }
    }

    #[doc="Write the SCGC7 register."]
    #[inline] pub fn set_scgc7<F: FnOnce(Scgc7) -> Scgc7>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scgc7_mut(), f(Scgc7(0)));
        }
        self
    }

    #[doc="Modify the SCGC7 register."]
    #[inline] pub fn with_scgc7<F: FnOnce(Scgc7) -> Scgc7>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.scgc7_mut(), f(self.scgc7()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLKDIV1 register."]
    #[inline] pub fn clkdiv1_mut(&self) -> *mut Clkdiv1 { 
        (self.0 + 0x1044) as *mut Clkdiv1
    }

    #[doc="Get the *const pointer for the CLKDIV1 register."]
    #[inline] pub fn clkdiv1_ptr(&self) -> *const Clkdiv1 { 
           self.clkdiv1_mut()
    }

    #[doc="Read the CLKDIV1 register."]
    #[inline] pub fn clkdiv1(&self) -> Clkdiv1 { 
        unsafe {
            read_volatile(self.clkdiv1_ptr())
        }
    }

    #[doc="Write the CLKDIV1 register."]
    #[inline] pub fn set_clkdiv1<F: FnOnce(Clkdiv1) -> Clkdiv1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clkdiv1_mut(), f(Clkdiv1(0)));
        }
        self
    }

    #[doc="Modify the CLKDIV1 register."]
    #[inline] pub fn with_clkdiv1<F: FnOnce(Clkdiv1) -> Clkdiv1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clkdiv1_mut(), f(self.clkdiv1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CLKDIV2 register."]
    #[inline] pub fn clkdiv2_mut(&self) -> *mut Clkdiv2 { 
        (self.0 + 0x1048) as *mut Clkdiv2
    }

    #[doc="Get the *const pointer for the CLKDIV2 register."]
    #[inline] pub fn clkdiv2_ptr(&self) -> *const Clkdiv2 { 
           self.clkdiv2_mut()
    }

    #[doc="Read the CLKDIV2 register."]
    #[inline] pub fn clkdiv2(&self) -> Clkdiv2 { 
        unsafe {
            read_volatile(self.clkdiv2_ptr())
        }
    }

    #[doc="Write the CLKDIV2 register."]
    #[inline] pub fn set_clkdiv2<F: FnOnce(Clkdiv2) -> Clkdiv2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clkdiv2_mut(), f(Clkdiv2(0)));
        }
        self
    }

    #[doc="Modify the CLKDIV2 register."]
    #[inline] pub fn with_clkdiv2<F: FnOnce(Clkdiv2) -> Clkdiv2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.clkdiv2_mut(), f(self.clkdiv2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FCFG1 register."]
    #[inline] pub fn fcfg1_mut(&self) -> *mut Fcfg1 { 
        (self.0 + 0x104c) as *mut Fcfg1
    }

    #[doc="Get the *const pointer for the FCFG1 register."]
    #[inline] pub fn fcfg1_ptr(&self) -> *const Fcfg1 { 
           self.fcfg1_mut()
    }

    #[doc="Read the FCFG1 register."]
    #[inline] pub fn fcfg1(&self) -> Fcfg1 { 
        unsafe {
            read_volatile(self.fcfg1_ptr())
        }
    }

    #[doc="Write the FCFG1 register."]
    #[inline] pub fn set_fcfg1<F: FnOnce(Fcfg1) -> Fcfg1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fcfg1_mut(), f(Fcfg1(0)));
        }
        self
    }

    #[doc="Modify the FCFG1 register."]
    #[inline] pub fn with_fcfg1<F: FnOnce(Fcfg1) -> Fcfg1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fcfg1_mut(), f(self.fcfg1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FCFG2 register."]
    #[inline] pub fn fcfg2_mut(&self) -> *mut Fcfg2 { 
        (self.0 + 0x1050) as *mut Fcfg2
    }

    #[doc="Get the *const pointer for the FCFG2 register."]
    #[inline] pub fn fcfg2_ptr(&self) -> *const Fcfg2 { 
           self.fcfg2_mut()
    }

    #[doc="Read the FCFG2 register."]
    #[inline] pub fn fcfg2(&self) -> Fcfg2 { 
        unsafe {
            read_volatile(self.fcfg2_ptr())
        }
    }

    #[doc="Get the *mut pointer for the UIDH register."]
    #[inline] pub fn uidh_mut(&self) -> *mut Uidh { 
        (self.0 + 0x1054) as *mut Uidh
    }

    #[doc="Get the *const pointer for the UIDH register."]
    #[inline] pub fn uidh_ptr(&self) -> *const Uidh { 
           self.uidh_mut()
    }

    #[doc="Read the UIDH register."]
    #[inline] pub fn uidh(&self) -> Uidh { 
        unsafe {
            read_volatile(self.uidh_ptr())
        }
    }

    #[doc="Get the *mut pointer for the UIDMH register."]
    #[inline] pub fn uidmh_mut(&self) -> *mut Uidmh { 
        (self.0 + 0x1058) as *mut Uidmh
    }

    #[doc="Get the *const pointer for the UIDMH register."]
    #[inline] pub fn uidmh_ptr(&self) -> *const Uidmh { 
           self.uidmh_mut()
    }

    #[doc="Read the UIDMH register."]
    #[inline] pub fn uidmh(&self) -> Uidmh { 
        unsafe {
            read_volatile(self.uidmh_ptr())
        }
    }

    #[doc="Get the *mut pointer for the UIDML register."]
    #[inline] pub fn uidml_mut(&self) -> *mut Uidml { 
        (self.0 + 0x105c) as *mut Uidml
    }

    #[doc="Get the *const pointer for the UIDML register."]
    #[inline] pub fn uidml_ptr(&self) -> *const Uidml { 
           self.uidml_mut()
    }

    #[doc="Read the UIDML register."]
    #[inline] pub fn uidml(&self) -> Uidml { 
        unsafe {
            read_volatile(self.uidml_ptr())
        }
    }

    #[doc="Get the *mut pointer for the UIDL register."]
    #[inline] pub fn uidl_mut(&self) -> *mut Uidl { 
        (self.0 + 0x1060) as *mut Uidl
    }

    #[doc="Get the *const pointer for the UIDL register."]
    #[inline] pub fn uidl_ptr(&self) -> *const Uidl { 
           self.uidl_mut()
    }

    #[doc="Read the UIDL register."]
    #[inline] pub fn uidl(&self) -> Uidl { 
        unsafe {
            read_volatile(self.uidl_ptr())
        }
    }

}

#[doc="System Options Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sopt1(pub u32);
impl Sopt1 {
    #[doc="RAM size"]
    #[inline] pub fn ramsize(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if RAMSIZE != 0"]
    #[inline] pub fn test_ramsize(&self) -> bool {
        self.ramsize() != 0
    }

    #[doc="Sets the RAMSIZE field."]
    #[inline] pub fn set_ramsize<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="32K oscillator clock select"]
    #[inline] pub fn osc32ksel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if OSC32KSEL != 0"]
    #[inline] pub fn test_osc32ksel(&self) -> bool {
        self.osc32ksel() != 0
    }

    #[doc="Sets the OSC32KSEL field."]
    #[inline] pub fn set_osc32ksel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="USB voltage regulator in standby mode during VLPR and VLPW modes"]
    #[inline] pub fn usbvstby(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if USBVSTBY != 0"]
    #[inline] pub fn test_usbvstby(&self) -> bool {
        self.usbvstby() != 0
    }

    #[doc="Sets the USBVSTBY field."]
    #[inline] pub fn set_usbvstby<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="USB voltage regulator in standby mode during Stop, VLPS, LLS and VLLS modes."]
    #[inline] pub fn usbsstby(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if USBSSTBY != 0"]
    #[inline] pub fn test_usbsstby(&self) -> bool {
        self.usbsstby() != 0
    }

    #[doc="Sets the USBSSTBY field."]
    #[inline] pub fn set_usbsstby<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="USB voltage regulator enable"]
    #[inline] pub fn usbregen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if USBREGEN != 0"]
    #[inline] pub fn test_usbregen(&self) -> bool {
        self.usbregen() != 0
    }

    #[doc="Sets the USBREGEN field."]
    #[inline] pub fn set_usbregen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Sopt1 {
    #[inline]
    fn from(other: u32) -> Self {
         Sopt1(other)
    }
}

impl ::core::fmt::Display for Sopt1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sopt1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ramsize() != 0 { try!(write!(f, " ramsize=0x{:x}", self.ramsize()))}
        if self.osc32ksel() != 0 { try!(write!(f, " osc32ksel=0x{:x}", self.osc32ksel()))}
        if self.usbvstby() != 0 { try!(write!(f, " usbvstby"))}
        if self.usbsstby() != 0 { try!(write!(f, " usbsstby"))}
        if self.usbregen() != 0 { try!(write!(f, " usbregen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SOPT1 Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sopt1cfg(pub u32);
impl Sopt1cfg {
    #[doc="USB voltage regulator enable write enable"]
    #[inline] pub fn urwe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if URWE != 0"]
    #[inline] pub fn test_urwe(&self) -> bool {
        self.urwe() != 0
    }

    #[doc="Sets the URWE field."]
    #[inline] pub fn set_urwe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="USB voltage regulator VLP standby write enable"]
    #[inline] pub fn uvswe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if UVSWE != 0"]
    #[inline] pub fn test_uvswe(&self) -> bool {
        self.uvswe() != 0
    }

    #[doc="Sets the UVSWE field."]
    #[inline] pub fn set_uvswe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="USB voltage regulator stop standby write enable"]
    #[inline] pub fn usswe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if USSWE != 0"]
    #[inline] pub fn test_usswe(&self) -> bool {
        self.usswe() != 0
    }

    #[doc="Sets the USSWE field."]
    #[inline] pub fn set_usswe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

}

impl From<u32> for Sopt1cfg {
    #[inline]
    fn from(other: u32) -> Self {
         Sopt1cfg(other)
    }
}

impl ::core::fmt::Display for Sopt1cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sopt1cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.urwe() != 0 { try!(write!(f, " urwe"))}
        if self.uvswe() != 0 { try!(write!(f, " uvswe"))}
        if self.usswe() != 0 { try!(write!(f, " usswe"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Options Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sopt2(pub u32);
impl Sopt2 {
    #[doc="RTC clock out select"]
    #[inline] pub fn rtcclkoutsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RTCCLKOUTSEL != 0"]
    #[inline] pub fn test_rtcclkoutsel(&self) -> bool {
        self.rtcclkoutsel() != 0
    }

    #[doc="Sets the RTCCLKOUTSEL field."]
    #[inline] pub fn set_rtcclkoutsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="CLKOUT select"]
    #[inline] pub fn clkoutsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7) as u8) } // [7:5]
    }

    #[doc="Returns true if CLKOUTSEL != 0"]
    #[inline] pub fn test_clkoutsel(&self) -> bool {
        self.clkoutsel() != 0
    }

    #[doc="Sets the CLKOUTSEL field."]
    #[inline] pub fn set_clkoutsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="FlexBus security level"]
    #[inline] pub fn fbsl(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if FBSL != 0"]
    #[inline] pub fn test_fbsl(&self) -> bool {
        self.fbsl() != 0
    }

    #[doc="Sets the FBSL field."]
    #[inline] pub fn set_fbsl<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="PTD7 pad drive strength"]
    #[inline] pub fn ptd7pad(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PTD7PAD != 0"]
    #[inline] pub fn test_ptd7pad(&self) -> bool {
        self.ptd7pad() != 0
    }

    #[doc="Sets the PTD7PAD field."]
    #[inline] pub fn set_ptd7pad<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Debug trace clock select"]
    #[inline] pub fn traceclksel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TRACECLKSEL != 0"]
    #[inline] pub fn test_traceclksel(&self) -> bool {
        self.traceclksel() != 0
    }

    #[doc="Sets the TRACECLKSEL field."]
    #[inline] pub fn set_traceclksel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="PLL/FLL clock select"]
    #[inline] pub fn pllfllsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if PLLFLLSEL != 0"]
    #[inline] pub fn test_pllfllsel(&self) -> bool {
        self.pllfllsel() != 0
    }

    #[doc="Sets the PLLFLLSEL field."]
    #[inline] pub fn set_pllfllsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="USB clock source select"]
    #[inline] pub fn usbsrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if USBSRC != 0"]
    #[inline] pub fn test_usbsrc(&self) -> bool {
        self.usbsrc() != 0
    }

    #[doc="Sets the USBSRC field."]
    #[inline] pub fn set_usbsrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="RMII clock source select"]
    #[inline] pub fn rmiisrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if RMIISRC != 0"]
    #[inline] pub fn test_rmiisrc(&self) -> bool {
        self.rmiisrc() != 0
    }

    #[doc="Sets the RMIISRC field."]
    #[inline] pub fn set_rmiisrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="IEEE 1588 timestamp clock source select"]
    #[inline] pub fn timesrc(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if TIMESRC != 0"]
    #[inline] pub fn test_timesrc(&self) -> bool {
        self.timesrc() != 0
    }

    #[doc="Sets the TIMESRC field."]
    #[inline] pub fn set_timesrc<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="SDHC clock source select"]
    #[inline] pub fn sdhcsrc(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Returns true if SDHCSRC != 0"]
    #[inline] pub fn test_sdhcsrc(&self) -> bool {
        self.sdhcsrc() != 0
    }

    #[doc="Sets the SDHCSRC field."]
    #[inline] pub fn set_sdhcsrc<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Sopt2 {
    #[inline]
    fn from(other: u32) -> Self {
         Sopt2(other)
    }
}

impl ::core::fmt::Display for Sopt2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sopt2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rtcclkoutsel() != 0 { try!(write!(f, " rtcclkoutsel"))}
        if self.clkoutsel() != 0 { try!(write!(f, " clkoutsel=0x{:x}", self.clkoutsel()))}
        if self.fbsl() != 0 { try!(write!(f, " fbsl=0x{:x}", self.fbsl()))}
        if self.ptd7pad() != 0 { try!(write!(f, " ptd7pad"))}
        if self.traceclksel() != 0 { try!(write!(f, " traceclksel"))}
        if self.pllfllsel() != 0 { try!(write!(f, " pllfllsel=0x{:x}", self.pllfllsel()))}
        if self.usbsrc() != 0 { try!(write!(f, " usbsrc"))}
        if self.rmiisrc() != 0 { try!(write!(f, " rmiisrc"))}
        if self.timesrc() != 0 { try!(write!(f, " timesrc=0x{:x}", self.timesrc()))}
        if self.sdhcsrc() != 0 { try!(write!(f, " sdhcsrc=0x{:x}", self.sdhcsrc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Options Register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sopt4(pub u32);
impl Sopt4 {
    #[doc="FTM0 Fault 0 Select"]
    #[inline] pub fn ftm0flt0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FTM0FLT0 != 0"]
    #[inline] pub fn test_ftm0flt0(&self) -> bool {
        self.ftm0flt0() != 0
    }

    #[doc="Sets the FTM0FLT0 field."]
    #[inline] pub fn set_ftm0flt0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="FTM0 Fault 1 Select"]
    #[inline] pub fn ftm0flt1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FTM0FLT1 != 0"]
    #[inline] pub fn test_ftm0flt1(&self) -> bool {
        self.ftm0flt1() != 0
    }

    #[doc="Sets the FTM0FLT1 field."]
    #[inline] pub fn set_ftm0flt1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="FTM0 Fault 2 Select"]
    #[inline] pub fn ftm0flt2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FTM0FLT2 != 0"]
    #[inline] pub fn test_ftm0flt2(&self) -> bool {
        self.ftm0flt2() != 0
    }

    #[doc="Sets the FTM0FLT2 field."]
    #[inline] pub fn set_ftm0flt2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="FTM1 Fault 0 Select"]
    #[inline] pub fn ftm1flt0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if FTM1FLT0 != 0"]
    #[inline] pub fn test_ftm1flt0(&self) -> bool {
        self.ftm1flt0() != 0
    }

    #[doc="Sets the FTM1FLT0 field."]
    #[inline] pub fn set_ftm1flt0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FTM2 Fault 0 Select"]
    #[inline] pub fn ftm2flt0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if FTM2FLT0 != 0"]
    #[inline] pub fn test_ftm2flt0(&self) -> bool {
        self.ftm2flt0() != 0
    }

    #[doc="Sets the FTM2FLT0 field."]
    #[inline] pub fn set_ftm2flt0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="FTM3 Fault 0 Select"]
    #[inline] pub fn ftm3flt0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if FTM3FLT0 != 0"]
    #[inline] pub fn test_ftm3flt0(&self) -> bool {
        self.ftm3flt0() != 0
    }

    #[doc="Sets the FTM3FLT0 field."]
    #[inline] pub fn set_ftm3flt0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="FTM1 channel 0 input capture source select"]
    #[inline] pub fn ftm1ch0src(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if FTM1CH0SRC != 0"]
    #[inline] pub fn test_ftm1ch0src(&self) -> bool {
        self.ftm1ch0src() != 0
    }

    #[doc="Sets the FTM1CH0SRC field."]
    #[inline] pub fn set_ftm1ch0src<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="FTM2 channel 0 input capture source select"]
    #[inline] pub fn ftm2ch0src(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if FTM2CH0SRC != 0"]
    #[inline] pub fn test_ftm2ch0src(&self) -> bool {
        self.ftm2ch0src() != 0
    }

    #[doc="Sets the FTM2CH0SRC field."]
    #[inline] pub fn set_ftm2ch0src<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="FlexTimer 0 External Clock Pin Select"]
    #[inline] pub fn ftm0clksel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if FTM0CLKSEL != 0"]
    #[inline] pub fn test_ftm0clksel(&self) -> bool {
        self.ftm0clksel() != 0
    }

    #[doc="Sets the FTM0CLKSEL field."]
    #[inline] pub fn set_ftm0clksel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="FTM1 External Clock Pin Select"]
    #[inline] pub fn ftm1clksel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if FTM1CLKSEL != 0"]
    #[inline] pub fn test_ftm1clksel(&self) -> bool {
        self.ftm1clksel() != 0
    }

    #[doc="Sets the FTM1CLKSEL field."]
    #[inline] pub fn set_ftm1clksel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="FlexTimer 2 External Clock Pin Select"]
    #[inline] pub fn ftm2clksel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if FTM2CLKSEL != 0"]
    #[inline] pub fn test_ftm2clksel(&self) -> bool {
        self.ftm2clksel() != 0
    }

    #[doc="Sets the FTM2CLKSEL field."]
    #[inline] pub fn set_ftm2clksel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="FlexTimer 3 External Clock Pin Select"]
    #[inline] pub fn ftm3clksel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if FTM3CLKSEL != 0"]
    #[inline] pub fn test_ftm3clksel(&self) -> bool {
        self.ftm3clksel() != 0
    }

    #[doc="Sets the FTM3CLKSEL field."]
    #[inline] pub fn set_ftm3clksel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="FlexTimer 0 Hardware Trigger 0 Source Select"]
    #[inline] pub fn ftm0trg0src(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if FTM0TRG0SRC != 0"]
    #[inline] pub fn test_ftm0trg0src(&self) -> bool {
        self.ftm0trg0src() != 0
    }

    #[doc="Sets the FTM0TRG0SRC field."]
    #[inline] pub fn set_ftm0trg0src<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="FlexTimer 0 Hardware Trigger 1 Source Select"]
    #[inline] pub fn ftm0trg1src(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if FTM0TRG1SRC != 0"]
    #[inline] pub fn test_ftm0trg1src(&self) -> bool {
        self.ftm0trg1src() != 0
    }

    #[doc="Sets the FTM0TRG1SRC field."]
    #[inline] pub fn set_ftm0trg1src<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="FlexTimer 3 Hardware Trigger 0 Source Select"]
    #[inline] pub fn ftm3trg0src(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if FTM3TRG0SRC != 0"]
    #[inline] pub fn test_ftm3trg0src(&self) -> bool {
        self.ftm3trg0src() != 0
    }

    #[doc="Sets the FTM3TRG0SRC field."]
    #[inline] pub fn set_ftm3trg0src<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="FlexTimer 3 Hardware Trigger 1 Source Select"]
    #[inline] pub fn ftm3trg1src(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if FTM3TRG1SRC != 0"]
    #[inline] pub fn test_ftm3trg1src(&self) -> bool {
        self.ftm3trg1src() != 0
    }

    #[doc="Sets the FTM3TRG1SRC field."]
    #[inline] pub fn set_ftm3trg1src<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Sopt4 {
    #[inline]
    fn from(other: u32) -> Self {
         Sopt4(other)
    }
}

impl ::core::fmt::Display for Sopt4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sopt4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ftm0flt0() != 0 { try!(write!(f, " ftm0flt0"))}
        if self.ftm0flt1() != 0 { try!(write!(f, " ftm0flt1"))}
        if self.ftm0flt2() != 0 { try!(write!(f, " ftm0flt2"))}
        if self.ftm1flt0() != 0 { try!(write!(f, " ftm1flt0"))}
        if self.ftm2flt0() != 0 { try!(write!(f, " ftm2flt0"))}
        if self.ftm3flt0() != 0 { try!(write!(f, " ftm3flt0"))}
        if self.ftm1ch0src() != 0 { try!(write!(f, " ftm1ch0src=0x{:x}", self.ftm1ch0src()))}
        if self.ftm2ch0src() != 0 { try!(write!(f, " ftm2ch0src=0x{:x}", self.ftm2ch0src()))}
        if self.ftm0clksel() != 0 { try!(write!(f, " ftm0clksel"))}
        if self.ftm1clksel() != 0 { try!(write!(f, " ftm1clksel"))}
        if self.ftm2clksel() != 0 { try!(write!(f, " ftm2clksel"))}
        if self.ftm3clksel() != 0 { try!(write!(f, " ftm3clksel"))}
        if self.ftm0trg0src() != 0 { try!(write!(f, " ftm0trg0src"))}
        if self.ftm0trg1src() != 0 { try!(write!(f, " ftm0trg1src"))}
        if self.ftm3trg0src() != 0 { try!(write!(f, " ftm3trg0src"))}
        if self.ftm3trg1src() != 0 { try!(write!(f, " ftm3trg1src"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Options Register 5"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sopt5(pub u32);
impl Sopt5 {
    #[doc="UART 0 transmit data source select"]
    #[inline] pub fn uart0txsrc(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if UART0TXSRC != 0"]
    #[inline] pub fn test_uart0txsrc(&self) -> bool {
        self.uart0txsrc() != 0
    }

    #[doc="Sets the UART0TXSRC field."]
    #[inline] pub fn set_uart0txsrc<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="UART 0 receive data source select"]
    #[inline] pub fn uart0rxsrc(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if UART0RXSRC != 0"]
    #[inline] pub fn test_uart0rxsrc(&self) -> bool {
        self.uart0rxsrc() != 0
    }

    #[doc="Sets the UART0RXSRC field."]
    #[inline] pub fn set_uart0rxsrc<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="UART 1 transmit data source select"]
    #[inline] pub fn uart1txsrc(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if UART1TXSRC != 0"]
    #[inline] pub fn test_uart1txsrc(&self) -> bool {
        self.uart1txsrc() != 0
    }

    #[doc="Sets the UART1TXSRC field."]
    #[inline] pub fn set_uart1txsrc<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="UART 1 receive data source select"]
    #[inline] pub fn uart1rxsrc(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if UART1RXSRC != 0"]
    #[inline] pub fn test_uart1rxsrc(&self) -> bool {
        self.uart1rxsrc() != 0
    }

    #[doc="Sets the UART1RXSRC field."]
    #[inline] pub fn set_uart1rxsrc<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for Sopt5 {
    #[inline]
    fn from(other: u32) -> Self {
         Sopt5(other)
    }
}

impl ::core::fmt::Display for Sopt5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sopt5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.uart0txsrc() != 0 { try!(write!(f, " uart0txsrc=0x{:x}", self.uart0txsrc()))}
        if self.uart0rxsrc() != 0 { try!(write!(f, " uart0rxsrc=0x{:x}", self.uart0rxsrc()))}
        if self.uart1txsrc() != 0 { try!(write!(f, " uart1txsrc=0x{:x}", self.uart1txsrc()))}
        if self.uart1rxsrc() != 0 { try!(write!(f, " uart1rxsrc=0x{:x}", self.uart1rxsrc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Options Register 7"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sopt7(pub u32);
impl Sopt7 {
    #[doc="ADC0 trigger select"]
    #[inline] pub fn adc0trgsel(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if ADC0TRGSEL != 0"]
    #[inline] pub fn test_adc0trgsel(&self) -> bool {
        self.adc0trgsel() != 0
    }

    #[doc="Sets the ADC0TRGSEL field."]
    #[inline] pub fn set_adc0trgsel<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="ADC0 pretrigger select"]
    #[inline] pub fn adc0pretrgsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ADC0PRETRGSEL != 0"]
    #[inline] pub fn test_adc0pretrgsel(&self) -> bool {
        self.adc0pretrgsel() != 0
    }

    #[doc="Sets the ADC0PRETRGSEL field."]
    #[inline] pub fn set_adc0pretrgsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ADC0 alternate trigger enable"]
    #[inline] pub fn adc0alttrgen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ADC0ALTTRGEN != 0"]
    #[inline] pub fn test_adc0alttrgen(&self) -> bool {
        self.adc0alttrgen() != 0
    }

    #[doc="Sets the ADC0ALTTRGEN field."]
    #[inline] pub fn set_adc0alttrgen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="ADC1 trigger select"]
    #[inline] pub fn adc1trgsel(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if ADC1TRGSEL != 0"]
    #[inline] pub fn test_adc1trgsel(&self) -> bool {
        self.adc1trgsel() != 0
    }

    #[doc="Sets the ADC1TRGSEL field."]
    #[inline] pub fn set_adc1trgsel<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="ADC1 pre-trigger select"]
    #[inline] pub fn adc1pretrgsel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if ADC1PRETRGSEL != 0"]
    #[inline] pub fn test_adc1pretrgsel(&self) -> bool {
        self.adc1pretrgsel() != 0
    }

    #[doc="Sets the ADC1PRETRGSEL field."]
    #[inline] pub fn set_adc1pretrgsel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="ADC1 alternate trigger enable"]
    #[inline] pub fn adc1alttrgen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if ADC1ALTTRGEN != 0"]
    #[inline] pub fn test_adc1alttrgen(&self) -> bool {
        self.adc1alttrgen() != 0
    }

    #[doc="Sets the ADC1ALTTRGEN field."]
    #[inline] pub fn set_adc1alttrgen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Sopt7 {
    #[inline]
    fn from(other: u32) -> Self {
         Sopt7(other)
    }
}

impl ::core::fmt::Display for Sopt7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sopt7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adc0trgsel() != 0 { try!(write!(f, " adc0trgsel=0x{:x}", self.adc0trgsel()))}
        if self.adc0pretrgsel() != 0 { try!(write!(f, " adc0pretrgsel"))}
        if self.adc0alttrgen() != 0 { try!(write!(f, " adc0alttrgen"))}
        if self.adc1trgsel() != 0 { try!(write!(f, " adc1trgsel=0x{:x}", self.adc1trgsel()))}
        if self.adc1pretrgsel() != 0 { try!(write!(f, " adc1pretrgsel"))}
        if self.adc1alttrgen() != 0 { try!(write!(f, " adc1alttrgen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Device Identification Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sdid(pub u32);
impl Sdid {
    #[doc="Pincount identification"]
    #[inline] pub fn pinid(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if PINID != 0"]
    #[inline] pub fn test_pinid(&self) -> bool {
        self.pinid() != 0
    }

    #[doc="Sets the PINID field."]
    #[inline] pub fn set_pinid<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Kinetis family identification"]
    #[inline] pub fn famid(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if FAMID != 0"]
    #[inline] pub fn test_famid(&self) -> bool {
        self.famid() != 0
    }

    #[doc="Sets the FAMID field."]
    #[inline] pub fn set_famid<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Device Die ID"]
    #[inline] pub fn dieid(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1f) as u8) } // [11:7]
    }

    #[doc="Returns true if DIEID != 0"]
    #[inline] pub fn test_dieid(&self) -> bool {
        self.dieid() != 0
    }

    #[doc="Sets the DIEID field."]
    #[inline] pub fn set_dieid<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Device revision number"]
    #[inline] pub fn revid(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if REVID != 0"]
    #[inline] pub fn test_revid(&self) -> bool {
        self.revid() != 0
    }

    #[doc="Sets the REVID field."]
    #[inline] pub fn set_revid<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Kinetis Series ID"]
    #[inline] pub fn seriesid(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if SERIESID != 0"]
    #[inline] pub fn test_seriesid(&self) -> bool {
        self.seriesid() != 0
    }

    #[doc="Sets the SERIESID field."]
    #[inline] pub fn set_seriesid<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Kinetis Sub-Family ID"]
    #[inline] pub fn subfamid(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if SUBFAMID != 0"]
    #[inline] pub fn test_subfamid(&self) -> bool {
        self.subfamid() != 0
    }

    #[doc="Sets the SUBFAMID field."]
    #[inline] pub fn set_subfamid<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Kinetis Family ID"]
    #[inline] pub fn familyid(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
    }

    #[doc="Returns true if FAMILYID != 0"]
    #[inline] pub fn test_familyid(&self) -> bool {
        self.familyid() != 0
    }

    #[doc="Sets the FAMILYID field."]
    #[inline] pub fn set_familyid<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Sdid {
    #[inline]
    fn from(other: u32) -> Self {
         Sdid(other)
    }
}

impl ::core::fmt::Display for Sdid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sdid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pinid() != 0 { try!(write!(f, " pinid=0x{:x}", self.pinid()))}
        if self.famid() != 0 { try!(write!(f, " famid=0x{:x}", self.famid()))}
        if self.dieid() != 0 { try!(write!(f, " dieid=0x{:x}", self.dieid()))}
        if self.revid() != 0 { try!(write!(f, " revid=0x{:x}", self.revid()))}
        if self.seriesid() != 0 { try!(write!(f, " seriesid=0x{:x}", self.seriesid()))}
        if self.subfamid() != 0 { try!(write!(f, " subfamid=0x{:x}", self.subfamid()))}
        if self.familyid() != 0 { try!(write!(f, " familyid=0x{:x}", self.familyid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Clock Gating Control Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgc1(pub u32);
impl Scgc1 {
    #[doc="I2C2 Clock Gate Control"]
    #[inline] pub fn i2c2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if I2C2 != 0"]
    #[inline] pub fn test_i2c2(&self) -> bool {
        self.i2c2() != 0
    }

    #[doc="Sets the I2C2 field."]
    #[inline] pub fn set_i2c2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="UART4 Clock Gate Control"]
    #[inline] pub fn uart4(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if UART4 != 0"]
    #[inline] pub fn test_uart4(&self) -> bool {
        self.uart4() != 0
    }

    #[doc="Sets the UART4 field."]
    #[inline] pub fn set_uart4<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="UART5 Clock Gate Control"]
    #[inline] pub fn uart5(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if UART5 != 0"]
    #[inline] pub fn test_uart5(&self) -> bool {
        self.uart5() != 0
    }

    #[doc="Sets the UART5 field."]
    #[inline] pub fn set_uart5<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for Scgc1 {
    #[inline]
    fn from(other: u32) -> Self {
         Scgc1(other)
    }
}

impl ::core::fmt::Display for Scgc1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgc1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.i2c2() != 0 { try!(write!(f, " i2c2"))}
        if self.uart4() != 0 { try!(write!(f, " uart4"))}
        if self.uart5() != 0 { try!(write!(f, " uart5"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Clock Gating Control Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgc2(pub u32);
impl Scgc2 {
    #[doc="ENET Clock Gate Control"]
    #[inline] pub fn enet(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ENET != 0"]
    #[inline] pub fn test_enet(&self) -> bool {
        self.enet() != 0
    }

    #[doc="Sets the ENET field."]
    #[inline] pub fn set_enet<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DAC0 Clock Gate Control"]
    #[inline] pub fn dac0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DAC0 != 0"]
    #[inline] pub fn test_dac0(&self) -> bool {
        self.dac0() != 0
    }

    #[doc="Sets the DAC0 field."]
    #[inline] pub fn set_dac0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="DAC1 Clock Gate Control"]
    #[inline] pub fn dac1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if DAC1 != 0"]
    #[inline] pub fn test_dac1(&self) -> bool {
        self.dac1() != 0
    }

    #[doc="Sets the DAC1 field."]
    #[inline] pub fn set_dac1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u32> for Scgc2 {
    #[inline]
    fn from(other: u32) -> Self {
         Scgc2(other)
    }
}

impl ::core::fmt::Display for Scgc2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgc2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enet() != 0 { try!(write!(f, " enet"))}
        if self.dac0() != 0 { try!(write!(f, " dac0"))}
        if self.dac1() != 0 { try!(write!(f, " dac1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Clock Gating Control Register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgc3(pub u32);
impl Scgc3 {
    #[doc="RNGA Clock Gate Control"]
    #[inline] pub fn rnga(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RNGA != 0"]
    #[inline] pub fn test_rnga(&self) -> bool {
        self.rnga() != 0
    }

    #[doc="Sets the RNGA field."]
    #[inline] pub fn set_rnga<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SPI2 Clock Gate Control"]
    #[inline] pub fn spi2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SPI2 != 0"]
    #[inline] pub fn test_spi2(&self) -> bool {
        self.spi2() != 0
    }

    #[doc="Sets the SPI2 field."]
    #[inline] pub fn set_spi2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SDHC Clock Gate Control"]
    #[inline] pub fn sdhc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if SDHC != 0"]
    #[inline] pub fn test_sdhc(&self) -> bool {
        self.sdhc() != 0
    }

    #[doc="Sets the SDHC field."]
    #[inline] pub fn set_sdhc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="FTM2 Clock Gate Control"]
    #[inline] pub fn ftm2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if FTM2 != 0"]
    #[inline] pub fn test_ftm2(&self) -> bool {
        self.ftm2() != 0
    }

    #[doc="Sets the FTM2 field."]
    #[inline] pub fn set_ftm2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="FTM3 Clock Gate Control"]
    #[inline] pub fn ftm3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if FTM3 != 0"]
    #[inline] pub fn test_ftm3(&self) -> bool {
        self.ftm3() != 0
    }

    #[doc="Sets the FTM3 field."]
    #[inline] pub fn set_ftm3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="ADC1 Clock Gate Control"]
    #[inline] pub fn adc1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if ADC1 != 0"]
    #[inline] pub fn test_adc1(&self) -> bool {
        self.adc1() != 0
    }

    #[doc="Sets the ADC1 field."]
    #[inline] pub fn set_adc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

}

impl From<u32> for Scgc3 {
    #[inline]
    fn from(other: u32) -> Self {
         Scgc3(other)
    }
}

impl ::core::fmt::Display for Scgc3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgc3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rnga() != 0 { try!(write!(f, " rnga"))}
        if self.spi2() != 0 { try!(write!(f, " spi2"))}
        if self.sdhc() != 0 { try!(write!(f, " sdhc"))}
        if self.ftm2() != 0 { try!(write!(f, " ftm2"))}
        if self.ftm3() != 0 { try!(write!(f, " ftm3"))}
        if self.adc1() != 0 { try!(write!(f, " adc1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Clock Gating Control Register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgc4(pub u32);
impl Scgc4 {
    #[doc="EWM Clock Gate Control"]
    #[inline] pub fn ewm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EWM != 0"]
    #[inline] pub fn test_ewm(&self) -> bool {
        self.ewm() != 0
    }

    #[doc="Sets the EWM field."]
    #[inline] pub fn set_ewm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="CMT Clock Gate Control"]
    #[inline] pub fn cmt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CMT != 0"]
    #[inline] pub fn test_cmt(&self) -> bool {
        self.cmt() != 0
    }

    #[doc="Sets the CMT field."]
    #[inline] pub fn set_cmt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="I2C0 Clock Gate Control"]
    #[inline] pub fn i2c0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if I2C0 != 0"]
    #[inline] pub fn test_i2c0(&self) -> bool {
        self.i2c0() != 0
    }

    #[doc="Sets the I2C0 field."]
    #[inline] pub fn set_i2c0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="I2C1 Clock Gate Control"]
    #[inline] pub fn i2c1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if I2C1 != 0"]
    #[inline] pub fn test_i2c1(&self) -> bool {
        self.i2c1() != 0
    }

    #[doc="Sets the I2C1 field."]
    #[inline] pub fn set_i2c1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="UART0 Clock Gate Control"]
    #[inline] pub fn uart0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if UART0 != 0"]
    #[inline] pub fn test_uart0(&self) -> bool {
        self.uart0() != 0
    }

    #[doc="Sets the UART0 field."]
    #[inline] pub fn set_uart0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="UART1 Clock Gate Control"]
    #[inline] pub fn uart1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if UART1 != 0"]
    #[inline] pub fn test_uart1(&self) -> bool {
        self.uart1() != 0
    }

    #[doc="Sets the UART1 field."]
    #[inline] pub fn set_uart1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="UART2 Clock Gate Control"]
    #[inline] pub fn uart2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if UART2 != 0"]
    #[inline] pub fn test_uart2(&self) -> bool {
        self.uart2() != 0
    }

    #[doc="Sets the UART2 field."]
    #[inline] pub fn set_uart2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="UART3 Clock Gate Control"]
    #[inline] pub fn uart3(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if UART3 != 0"]
    #[inline] pub fn test_uart3(&self) -> bool {
        self.uart3() != 0
    }

    #[doc="Sets the UART3 field."]
    #[inline] pub fn set_uart3<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="USB Clock Gate Control"]
    #[inline] pub fn usbotg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if USBOTG != 0"]
    #[inline] pub fn test_usbotg(&self) -> bool {
        self.usbotg() != 0
    }

    #[doc="Sets the USBOTG field."]
    #[inline] pub fn set_usbotg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Comparator Clock Gate Control"]
    #[inline] pub fn cmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if CMP != 0"]
    #[inline] pub fn test_cmp(&self) -> bool {
        self.cmp() != 0
    }

    #[doc="Sets the CMP field."]
    #[inline] pub fn set_cmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="VREF Clock Gate Control"]
    #[inline] pub fn vref(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if VREF != 0"]
    #[inline] pub fn test_vref(&self) -> bool {
        self.vref() != 0
    }

    #[doc="Sets the VREF field."]
    #[inline] pub fn set_vref<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

}

impl From<u32> for Scgc4 {
    #[inline]
    fn from(other: u32) -> Self {
         Scgc4(other)
    }
}

impl ::core::fmt::Display for Scgc4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgc4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ewm() != 0 { try!(write!(f, " ewm"))}
        if self.cmt() != 0 { try!(write!(f, " cmt"))}
        if self.i2c0() != 0 { try!(write!(f, " i2c0"))}
        if self.i2c1() != 0 { try!(write!(f, " i2c1"))}
        if self.uart0() != 0 { try!(write!(f, " uart0"))}
        if self.uart1() != 0 { try!(write!(f, " uart1"))}
        if self.uart2() != 0 { try!(write!(f, " uart2"))}
        if self.uart3() != 0 { try!(write!(f, " uart3"))}
        if self.usbotg() != 0 { try!(write!(f, " usbotg"))}
        if self.cmp() != 0 { try!(write!(f, " cmp"))}
        if self.vref() != 0 { try!(write!(f, " vref"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Clock Gating Control Register 5"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgc5(pub u32);
impl Scgc5 {
    #[doc="Low Power Timer Access Control"]
    #[inline] pub fn lptmr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LPTMR != 0"]
    #[inline] pub fn test_lptmr(&self) -> bool {
        self.lptmr() != 0
    }

    #[doc="Sets the LPTMR field."]
    #[inline] pub fn set_lptmr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Port A Clock Gate Control"]
    #[inline] pub fn porta(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PORTA != 0"]
    #[inline] pub fn test_porta(&self) -> bool {
        self.porta() != 0
    }

    #[doc="Sets the PORTA field."]
    #[inline] pub fn set_porta<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port B Clock Gate Control"]
    #[inline] pub fn portb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PORTB != 0"]
    #[inline] pub fn test_portb(&self) -> bool {
        self.portb() != 0
    }

    #[doc="Sets the PORTB field."]
    #[inline] pub fn set_portb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port C Clock Gate Control"]
    #[inline] pub fn portc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PORTC != 0"]
    #[inline] pub fn test_portc(&self) -> bool {
        self.portc() != 0
    }

    #[doc="Sets the PORTC field."]
    #[inline] pub fn set_portc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port D Clock Gate Control"]
    #[inline] pub fn portd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PORTD != 0"]
    #[inline] pub fn test_portd(&self) -> bool {
        self.portd() != 0
    }

    #[doc="Sets the PORTD field."]
    #[inline] pub fn set_portd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port E Clock Gate Control"]
    #[inline] pub fn porte(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PORTE != 0"]
    #[inline] pub fn test_porte(&self) -> bool {
        self.porte() != 0
    }

    #[doc="Sets the PORTE field."]
    #[inline] pub fn set_porte<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u32> for Scgc5 {
    #[inline]
    fn from(other: u32) -> Self {
         Scgc5(other)
    }
}

impl ::core::fmt::Display for Scgc5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgc5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lptmr() != 0 { try!(write!(f, " lptmr"))}
        if self.porta() != 0 { try!(write!(f, " porta"))}
        if self.portb() != 0 { try!(write!(f, " portb"))}
        if self.portc() != 0 { try!(write!(f, " portc"))}
        if self.portd() != 0 { try!(write!(f, " portd"))}
        if self.porte() != 0 { try!(write!(f, " porte"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Clock Gating Control Register 6"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgc6(pub u32);
impl Scgc6 {
    #[doc="Flash Memory Clock Gate Control"]
    #[inline] pub fn ftf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FTF != 0"]
    #[inline] pub fn test_ftf(&self) -> bool {
        self.ftf() != 0
    }

    #[doc="Sets the FTF field."]
    #[inline] pub fn set_ftf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DMA Mux Clock Gate Control"]
    #[inline] pub fn dmamux(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DMAMUX != 0"]
    #[inline] pub fn test_dmamux(&self) -> bool {
        self.dmamux() != 0
    }

    #[doc="Sets the DMAMUX field."]
    #[inline] pub fn set_dmamux<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="FlexCAN0 Clock Gate Control"]
    #[inline] pub fn flexcan0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if FLEXCAN0 != 0"]
    #[inline] pub fn test_flexcan0(&self) -> bool {
        self.flexcan0() != 0
    }

    #[doc="Sets the FLEXCAN0 field."]
    #[inline] pub fn set_flexcan0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="RNGA Clock Gate Control"]
    #[inline] pub fn rnga(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RNGA != 0"]
    #[inline] pub fn test_rnga(&self) -> bool {
        self.rnga() != 0
    }

    #[doc="Sets the RNGA field."]
    #[inline] pub fn set_rnga<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="SPI0 Clock Gate Control"]
    #[inline] pub fn spi0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SPI0 != 0"]
    #[inline] pub fn test_spi0(&self) -> bool {
        self.spi0() != 0
    }

    #[doc="Sets the SPI0 field."]
    #[inline] pub fn set_spi0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SPI1 Clock Gate Control"]
    #[inline] pub fn spi1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if SPI1 != 0"]
    #[inline] pub fn test_spi1(&self) -> bool {
        self.spi1() != 0
    }

    #[doc="Sets the SPI1 field."]
    #[inline] pub fn set_spi1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="I2S Clock Gate Control"]
    #[inline] pub fn i2s(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if I2S != 0"]
    #[inline] pub fn test_i2s(&self) -> bool {
        self.i2s() != 0
    }

    #[doc="Sets the I2S field."]
    #[inline] pub fn set_i2s<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="CRC Clock Gate Control"]
    #[inline] pub fn crc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if CRC != 0"]
    #[inline] pub fn test_crc(&self) -> bool {
        self.crc() != 0
    }

    #[doc="Sets the CRC field."]
    #[inline] pub fn set_crc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="USB DCD Clock Gate Control"]
    #[inline] pub fn usbdcd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if USBDCD != 0"]
    #[inline] pub fn test_usbdcd(&self) -> bool {
        self.usbdcd() != 0
    }

    #[doc="Sets the USBDCD field."]
    #[inline] pub fn set_usbdcd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="PDB Clock Gate Control"]
    #[inline] pub fn pdb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if PDB != 0"]
    #[inline] pub fn test_pdb(&self) -> bool {
        self.pdb() != 0
    }

    #[doc="Sets the PDB field."]
    #[inline] pub fn set_pdb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="PIT Clock Gate Control"]
    #[inline] pub fn pit(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if PIT != 0"]
    #[inline] pub fn test_pit(&self) -> bool {
        self.pit() != 0
    }

    #[doc="Sets the PIT field."]
    #[inline] pub fn set_pit<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="FTM0 Clock Gate Control"]
    #[inline] pub fn ftm0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if FTM0 != 0"]
    #[inline] pub fn test_ftm0(&self) -> bool {
        self.ftm0() != 0
    }

    #[doc="Sets the FTM0 field."]
    #[inline] pub fn set_ftm0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="FTM1 Clock Gate Control"]
    #[inline] pub fn ftm1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if FTM1 != 0"]
    #[inline] pub fn test_ftm1(&self) -> bool {
        self.ftm1() != 0
    }

    #[doc="Sets the FTM1 field."]
    #[inline] pub fn set_ftm1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="FTM2 Clock Gate Control"]
    #[inline] pub fn ftm2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if FTM2 != 0"]
    #[inline] pub fn test_ftm2(&self) -> bool {
        self.ftm2() != 0
    }

    #[doc="Sets the FTM2 field."]
    #[inline] pub fn set_ftm2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="ADC0 Clock Gate Control"]
    #[inline] pub fn adc0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if ADC0 != 0"]
    #[inline] pub fn test_adc0(&self) -> bool {
        self.adc0() != 0
    }

    #[doc="Sets the ADC0 field."]
    #[inline] pub fn set_adc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="RTC Access Control"]
    #[inline] pub fn rtc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if RTC != 0"]
    #[inline] pub fn test_rtc(&self) -> bool {
        self.rtc() != 0
    }

    #[doc="Sets the RTC field."]
    #[inline] pub fn set_rtc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="DAC0 Clock Gate Control"]
    #[inline] pub fn dac0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if DAC0 != 0"]
    #[inline] pub fn test_dac0(&self) -> bool {
        self.dac0() != 0
    }

    #[doc="Sets the DAC0 field."]
    #[inline] pub fn set_dac0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Scgc6 {
    #[inline]
    fn from(other: u32) -> Self {
         Scgc6(other)
    }
}

impl ::core::fmt::Display for Scgc6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgc6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ftf() != 0 { try!(write!(f, " ftf"))}
        if self.dmamux() != 0 { try!(write!(f, " dmamux"))}
        if self.flexcan0() != 0 { try!(write!(f, " flexcan0"))}
        if self.rnga() != 0 { try!(write!(f, " rnga"))}
        if self.spi0() != 0 { try!(write!(f, " spi0"))}
        if self.spi1() != 0 { try!(write!(f, " spi1"))}
        if self.i2s() != 0 { try!(write!(f, " i2s"))}
        if self.crc() != 0 { try!(write!(f, " crc"))}
        if self.usbdcd() != 0 { try!(write!(f, " usbdcd"))}
        if self.pdb() != 0 { try!(write!(f, " pdb"))}
        if self.pit() != 0 { try!(write!(f, " pit"))}
        if self.ftm0() != 0 { try!(write!(f, " ftm0"))}
        if self.ftm1() != 0 { try!(write!(f, " ftm1"))}
        if self.ftm2() != 0 { try!(write!(f, " ftm2"))}
        if self.adc0() != 0 { try!(write!(f, " adc0"))}
        if self.rtc() != 0 { try!(write!(f, " rtc"))}
        if self.dac0() != 0 { try!(write!(f, " dac0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Clock Gating Control Register 7"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgc7(pub u32);
impl Scgc7 {
    #[doc="FlexBus Clock Gate Control"]
    #[inline] pub fn flexbus(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FLEXBUS != 0"]
    #[inline] pub fn test_flexbus(&self) -> bool {
        self.flexbus() != 0
    }

    #[doc="Sets the FLEXBUS field."]
    #[inline] pub fn set_flexbus<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DMA Clock Gate Control"]
    #[inline] pub fn dma(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DMA != 0"]
    #[inline] pub fn test_dma(&self) -> bool {
        self.dma() != 0
    }

    #[doc="Sets the DMA field."]
    #[inline] pub fn set_dma<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="MPU Clock Gate Control"]
    #[inline] pub fn mpu(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MPU != 0"]
    #[inline] pub fn test_mpu(&self) -> bool {
        self.mpu() != 0
    }

    #[doc="Sets the MPU field."]
    #[inline] pub fn set_mpu<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u32> for Scgc7 {
    #[inline]
    fn from(other: u32) -> Self {
         Scgc7(other)
    }
}

impl ::core::fmt::Display for Scgc7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgc7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.flexbus() != 0 { try!(write!(f, " flexbus"))}
        if self.dma() != 0 { try!(write!(f, " dma"))}
        if self.mpu() != 0 { try!(write!(f, " mpu"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Clock Divider Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clkdiv1(pub u32);
impl Clkdiv1 {
    #[doc="Clock 4 output divider value"]
    #[inline] pub fn outdiv4(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if OUTDIV4 != 0"]
    #[inline] pub fn test_outdiv4(&self) -> bool {
        self.outdiv4() != 0
    }

    #[doc="Sets the OUTDIV4 field."]
    #[inline] pub fn set_outdiv4<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Clock 3 output divider value"]
    #[inline] pub fn outdiv3(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if OUTDIV3 != 0"]
    #[inline] pub fn test_outdiv3(&self) -> bool {
        self.outdiv3() != 0
    }

    #[doc="Sets the OUTDIV3 field."]
    #[inline] pub fn set_outdiv3<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Clock 2 output divider value"]
    #[inline] pub fn outdiv2(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if OUTDIV2 != 0"]
    #[inline] pub fn test_outdiv2(&self) -> bool {
        self.outdiv2() != 0
    }

    #[doc="Sets the OUTDIV2 field."]
    #[inline] pub fn set_outdiv2<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Clock 1 output divider value"]
    #[inline] pub fn outdiv1(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
    }

    #[doc="Returns true if OUTDIV1 != 0"]
    #[inline] pub fn test_outdiv1(&self) -> bool {
        self.outdiv1() != 0
    }

    #[doc="Sets the OUTDIV1 field."]
    #[inline] pub fn set_outdiv1<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Clkdiv1 {
    #[inline]
    fn from(other: u32) -> Self {
         Clkdiv1(other)
    }
}

impl ::core::fmt::Display for Clkdiv1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clkdiv1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.outdiv4() != 0 { try!(write!(f, " outdiv4=0x{:x}", self.outdiv4()))}
        if self.outdiv3() != 0 { try!(write!(f, " outdiv3=0x{:x}", self.outdiv3()))}
        if self.outdiv2() != 0 { try!(write!(f, " outdiv2=0x{:x}", self.outdiv2()))}
        if self.outdiv1() != 0 { try!(write!(f, " outdiv1=0x{:x}", self.outdiv1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Clock Divider Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clkdiv2(pub u32);
impl Clkdiv2 {
    #[doc="USB clock divider fraction"]
    #[inline] pub fn usbfrac(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if USBFRAC != 0"]
    #[inline] pub fn test_usbfrac(&self) -> bool {
        self.usbfrac() != 0
    }

    #[doc="Sets the USBFRAC field."]
    #[inline] pub fn set_usbfrac<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="USB clock divider divisor"]
    #[inline] pub fn usbdiv(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7) as u8) } // [3:1]
    }

    #[doc="Returns true if USBDIV != 0"]
    #[inline] pub fn test_usbdiv(&self) -> bool {
        self.usbdiv() != 0
    }

    #[doc="Sets the USBDIV field."]
    #[inline] pub fn set_usbdiv<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Clkdiv2 {
    #[inline]
    fn from(other: u32) -> Self {
         Clkdiv2(other)
    }
}

impl ::core::fmt::Display for Clkdiv2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clkdiv2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.usbfrac() != 0 { try!(write!(f, " usbfrac"))}
        if self.usbdiv() != 0 { try!(write!(f, " usbdiv=0x{:x}", self.usbdiv()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Configuration Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fcfg1(pub u32);
impl Fcfg1 {
    #[doc="Flash Disable"]
    #[inline] pub fn flashdis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FLASHDIS != 0"]
    #[inline] pub fn test_flashdis(&self) -> bool {
        self.flashdis() != 0
    }

    #[doc="Sets the FLASHDIS field."]
    #[inline] pub fn set_flashdis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Flash Doze"]
    #[inline] pub fn flashdoze(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FLASHDOZE != 0"]
    #[inline] pub fn test_flashdoze(&self) -> bool {
        self.flashdoze() != 0
    }

    #[doc="Sets the FLASHDOZE field."]
    #[inline] pub fn set_flashdoze<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="FlexNVM partition"]
    #[inline] pub fn depart(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if DEPART != 0"]
    #[inline] pub fn test_depart(&self) -> bool {
        self.depart() != 0
    }

    #[doc="Sets the DEPART field."]
    #[inline] pub fn set_depart<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="EEPROM size"]
    #[inline] pub fn eesize(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if EESIZE != 0"]
    #[inline] pub fn test_eesize(&self) -> bool {
        self.eesize() != 0
    }

    #[doc="Sets the EESIZE field."]
    #[inline] pub fn set_eesize<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Program flash size"]
    #[inline] pub fn pfsize(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if PFSIZE != 0"]
    #[inline] pub fn test_pfsize(&self) -> bool {
        self.pfsize() != 0
    }

    #[doc="Sets the PFSIZE field."]
    #[inline] pub fn set_pfsize<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="FlexNVM size"]
    #[inline] pub fn nvmsize(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
    }

    #[doc="Returns true if NVMSIZE != 0"]
    #[inline] pub fn test_nvmsize(&self) -> bool {
        self.nvmsize() != 0
    }

    #[doc="Sets the NVMSIZE field."]
    #[inline] pub fn set_nvmsize<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Fcfg1 {
    #[inline]
    fn from(other: u32) -> Self {
         Fcfg1(other)
    }
}

impl ::core::fmt::Display for Fcfg1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fcfg1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.flashdis() != 0 { try!(write!(f, " flashdis"))}
        if self.flashdoze() != 0 { try!(write!(f, " flashdoze"))}
        if self.depart() != 0 { try!(write!(f, " depart=0x{:x}", self.depart()))}
        if self.eesize() != 0 { try!(write!(f, " eesize=0x{:x}", self.eesize()))}
        if self.pfsize() != 0 { try!(write!(f, " pfsize=0x{:x}", self.pfsize()))}
        if self.nvmsize() != 0 { try!(write!(f, " nvmsize=0x{:x}", self.nvmsize()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Configuration Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fcfg2(pub u32);
impl Fcfg2 {
    #[doc="Max address block 1"]
    #[inline] pub fn maxaddr1(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7f) as u8) } // [22:16]
    }

    #[doc="Returns true if MAXADDR1 != 0"]
    #[inline] pub fn test_maxaddr1(&self) -> bool {
        self.maxaddr1() != 0
    }

    #[doc="Sets the MAXADDR1 field."]
    #[inline] pub fn set_maxaddr1<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Program flash only"]
    #[inline] pub fn pflsh(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if PFLSH != 0"]
    #[inline] pub fn test_pflsh(&self) -> bool {
        self.pflsh() != 0
    }

    #[doc="Sets the PFLSH field."]
    #[inline] pub fn set_pflsh<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Max address block 0"]
    #[inline] pub fn maxaddr0(&self) -> bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7f) as u8) } // [30:24]
    }

    #[doc="Returns true if MAXADDR0 != 0"]
    #[inline] pub fn test_maxaddr0(&self) -> bool {
        self.maxaddr0() != 0
    }

    #[doc="Sets the MAXADDR0 field."]
    #[inline] pub fn set_maxaddr0<V: Into<bits::U7>>(mut self, value: V) -> Self {
        let value: bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Fcfg2 {
    #[inline]
    fn from(other: u32) -> Self {
         Fcfg2(other)
    }
}

impl ::core::fmt::Display for Fcfg2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fcfg2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.maxaddr1() != 0 { try!(write!(f, " maxaddr1=0x{:x}", self.maxaddr1()))}
        if self.pflsh() != 0 { try!(write!(f, " pflsh"))}
        if self.maxaddr0() != 0 { try!(write!(f, " maxaddr0=0x{:x}", self.maxaddr0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Unique Identification Register High"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Uidh(pub u32);
impl Uidh {
    #[doc="Unique Identification"]
    #[inline] pub fn uid(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if UID != 0"]
    #[inline] pub fn test_uid(&self) -> bool {
        self.uid() != 0
    }

    #[doc="Sets the UID field."]
    #[inline] pub fn set_uid<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Uidh {
    #[inline]
    fn from(other: u32) -> Self {
         Uidh(other)
    }
}

impl ::core::fmt::Display for Uidh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Uidh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Unique Identification Register Mid-High"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Uidmh(pub u32);
impl Uidmh {
    #[doc="Unique Identification"]
    #[inline] pub fn uid(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if UID != 0"]
    #[inline] pub fn test_uid(&self) -> bool {
        self.uid() != 0
    }

    #[doc="Sets the UID field."]
    #[inline] pub fn set_uid<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Uidmh {
    #[inline]
    fn from(other: u32) -> Self {
         Uidmh(other)
    }
}

impl ::core::fmt::Display for Uidmh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Uidmh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Unique Identification Register Mid Low"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Uidml(pub u32);
impl Uidml {
    #[doc="Unique Identification"]
    #[inline] pub fn uid(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if UID != 0"]
    #[inline] pub fn test_uid(&self) -> bool {
        self.uid() != 0
    }

    #[doc="Sets the UID field."]
    #[inline] pub fn set_uid<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Uidml {
    #[inline]
    fn from(other: u32) -> Self {
         Uidml(other)
    }
}

impl ::core::fmt::Display for Uidml {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Uidml {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Unique Identification Register Low"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Uidl(pub u32);
impl Uidl {
    #[doc="Unique Identification"]
    #[inline] pub fn uid(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if UID != 0"]
    #[inline] pub fn test_uid(&self) -> bool {
        self.uid() != 0
    }

    #[doc="Sets the UID field."]
    #[inline] pub fn set_uid<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Uidl {
    #[inline]
    fn from(other: u32) -> Self {
         Uidl(other)
    }
}

impl ::core::fmt::Display for Uidl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Uidl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

pub trait En {
    fn en(&self) -> u32;
    fn set_en(&self, value: u32);
}

impl Sim {
    #[inline] pub fn en<P: En>(&self, p: &P) -> u32 {
        p.en()
    }
    #[inline] pub fn set_en<P: En>(&self, p: &P, value: u32) {
        p.set_en(value)
    }
}

pub trait Rst {
    fn rst(&self) -> u32;
    fn set_rst(&self, value: u32);
}

impl Sim {
    #[inline] pub fn rst<P: Rst>(&self, p: &P) -> u32 {
        p.rst()
    }
    #[inline] pub fn set_rst<P: Rst>(&self, p: &P, value: u32) {
        p.set_rst(value)
    }
}

impl En for super::uart::Uart4 {
    #[inline] fn en(&self) -> u32 { SIM.scgc1().uart4().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc1(|r| r.set_uart4(value)); }
}

impl En for super::uart::Uart5 {
    #[inline] fn en(&self) -> u32 { SIM.scgc1().uart5().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc1(|r| r.set_uart5(value)); }
}

impl En for super::enet::Enet {
    #[inline] fn en(&self) -> u32 { SIM.scgc2().enet().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc2(|r| r.set_enet(value)); }
}

impl En for super::spi::Spi2 {
    #[inline] fn en(&self) -> u32 { SIM.scgc3().spi2().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc3(|r| r.set_spi2(value)); }
}

impl En for super::adc::Adc1 {
    #[inline] fn en(&self) -> u32 { SIM.scgc3().adc1().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc3(|r| r.set_adc1(value)); }
}

impl En for super::i2c::I2c0 {
    #[inline] fn en(&self) -> u32 { SIM.scgc4().i2c0().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc4(|r| r.set_i2c0(value)); }
}

impl En for super::i2c::I2c1 {
    #[inline] fn en(&self) -> u32 { SIM.scgc4().i2c1().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc4(|r| r.set_i2c1(value)); }
}

impl En for super::uart::Uart0 {
    #[inline] fn en(&self) -> u32 { SIM.scgc4().uart0().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc4(|r| r.set_uart0(value)); }
}

impl En for super::uart::Uart1 {
    #[inline] fn en(&self) -> u32 { SIM.scgc4().uart1().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc4(|r| r.set_uart1(value)); }
}

impl En for super::uart::Uart2 {
    #[inline] fn en(&self) -> u32 { SIM.scgc4().uart2().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc4(|r| r.set_uart2(value)); }
}

impl En for super::uart::Uart3 {
    #[inline] fn en(&self) -> u32 { SIM.scgc4().uart3().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc4(|r| r.set_uart3(value)); }
}

impl En for super::lptmr::Lptmr0 {
    #[inline] fn en(&self) -> u32 { SIM.scgc5().lptmr().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc5(|r| r.set_lptmr(value)); }
}

impl En for super::port::Porta {
    #[inline] fn en(&self) -> u32 { SIM.scgc5().porta().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc5(|r| r.set_porta(value)); }
}

impl En for super::port::Portb {
    #[inline] fn en(&self) -> u32 { SIM.scgc5().portb().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc5(|r| r.set_portb(value)); }
}

impl En for super::port::Portc {
    #[inline] fn en(&self) -> u32 { SIM.scgc5().portc().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc5(|r| r.set_portc(value)); }
}

impl En for super::port::Portd {
    #[inline] fn en(&self) -> u32 { SIM.scgc5().portd().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc5(|r| r.set_portd(value)); }
}

impl En for super::port::Porte {
    #[inline] fn en(&self) -> u32 { SIM.scgc5().porte().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc5(|r| r.set_porte(value)); }
}

impl En for super::dmamux::Dmamux {
    #[inline] fn en(&self) -> u32 { SIM.scgc6().dmamux().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc6(|r| r.set_dmamux(value)); }
}

impl En for super::spi::Spi0 {
    #[inline] fn en(&self) -> u32 { SIM.scgc6().spi0().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc6(|r| r.set_spi0(value)); }
}

impl En for super::spi::Spi1 {
    #[inline] fn en(&self) -> u32 { SIM.scgc6().spi1().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc6(|r| r.set_spi1(value)); }
}

impl En for super::crc::Crc {
    #[inline] fn en(&self) -> u32 { SIM.scgc6().crc().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc6(|r| r.set_crc(value)); }
}

impl En for super::pit::Pit {
    #[inline] fn en(&self) -> u32 { SIM.scgc6().pit().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc6(|r| r.set_pit(value)); }
}

impl En for super::ftm::Ftm0 {
    #[inline] fn en(&self) -> u32 { SIM.scgc6().ftm0().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc6(|r| r.set_ftm0(value)); }
}

impl En for super::ftm::Ftm1 {
    #[inline] fn en(&self) -> u32 { SIM.scgc6().ftm1().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc6(|r| r.set_ftm1(value)); }
}

impl En for super::ftm::Ftm2 {
    #[inline] fn en(&self) -> u32 { SIM.scgc6().ftm2().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc6(|r| r.set_ftm2(value)); }
}

impl En for super::adc::Adc0 {
    #[inline] fn en(&self) -> u32 { SIM.scgc6().adc0().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc6(|r| r.set_adc0(value)); }
}

impl En for super::edma::Dma {
    #[inline] fn en(&self) -> u32 { SIM.scgc7().dma().into() }
    #[inline] fn set_en(&self, value: u32) { SIM.with_scgc7(|r| r.set_dma(value)); }
}


