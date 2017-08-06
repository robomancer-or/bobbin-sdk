#[allow(unused_imports)] use bobbin_common::bits;
pub const SSI0: Ssi0 = Periph(0x40008000, Ssi0Id {});
pub const SSI1: Ssi1 = Periph(0x40009000, Ssi1Id {});
pub const SSI2: Ssi2 = Periph(0x4000a000, Ssi2Id {});
pub const SSI3: Ssi3 = Periph(0x4000b000, Ssi3Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="SSI Peripheral"]
pub struct Periph<T>(pub u32, pub T); 

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Ssi0Id {}
pub type Ssi0 = Periph<Ssi0Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Ssi1Id {}
pub type Ssi1 = Periph<Ssi1Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Ssi2Id {}
pub type Ssi2 = Periph<Ssi2Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Ssi3Id {}
pub type Ssi3 = Periph<Ssi3Id>;






impl<T> Periph<T> {
#[doc="Get the *const pointer for the CR0 register."]
  #[inline] pub fn cr0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the CR0 register."]
  #[inline] pub fn cr0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the CR0 register."]
  #[inline] pub fn cr0(&self) -> Cr0 { 
     unsafe {
        Cr0(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the CR0 register."]
  #[inline] pub fn set_cr0(&self, value: Cr0) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CR0 register."]
  #[inline] pub fn with_cr0<F: FnOnce(Cr0) -> Cr0>(&self, f: F) -> &Self {
     let tmp = self.cr0();
     self.set_cr0(f(tmp))
  }

#[doc="Get the *const pointer for the CR1 register."]
  #[inline] pub fn cr1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the CR1 register."]
  #[inline] pub fn cr1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the CR1 register."]
  #[inline] pub fn cr1(&self) -> Cr1 { 
     unsafe {
        Cr1(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the CR1 register."]
  #[inline] pub fn set_cr1(&self, value: Cr1) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CR1 register."]
  #[inline] pub fn with_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
     let tmp = self.cr1();
     self.set_cr1(f(tmp))
  }

#[doc="Get the *const pointer for the DR register."]
  #[inline] pub fn dr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the DR register."]
  #[inline] pub fn dr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the DR register."]
  #[inline] pub fn dr(&self) -> Dr { 
     unsafe {
        Dr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the DR register."]
  #[inline] pub fn set_dr(&self, value: Dr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DR register."]
  #[inline] pub fn with_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
     let tmp = self.dr();
     self.set_dr(f(tmp))
  }

#[doc="Get the *const pointer for the SR register."]
  #[inline] pub fn sr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the SR register."]
  #[inline] pub fn sr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the SR register."]
  #[inline] pub fn sr(&self) -> Sr { 
     unsafe {
        Sr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the SR register."]
  #[inline] pub fn set_sr(&self, value: Sr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SR register."]
  #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
     let tmp = self.sr();
     self.set_sr(f(tmp))
  }

#[doc="Get the *const pointer for the CPSR register."]
  #[inline] pub fn cpsr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the CPSR register."]
  #[inline] pub fn cpsr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the CPSR register."]
  #[inline] pub fn cpsr(&self) -> Cpsr { 
     unsafe {
        Cpsr(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the CPSR register."]
  #[inline] pub fn set_cpsr(&self, value: Cpsr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CPSR register."]
  #[inline] pub fn with_cpsr<F: FnOnce(Cpsr) -> Cpsr>(&self, f: F) -> &Self {
     let tmp = self.cpsr();
     self.set_cpsr(f(tmp))
  }

#[doc="Get the *const pointer for the IM register."]
  #[inline] pub fn im_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the IM register."]
  #[inline] pub fn im_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Read the IM register."]
  #[inline] pub fn im(&self) -> Im { 
     unsafe {
        Im(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
#[doc="Write the IM register."]
  #[inline] pub fn set_im(&self, value: Im) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the IM register."]
  #[inline] pub fn with_im<F: FnOnce(Im) -> Im>(&self, f: F) -> &Self {
     let tmp = self.im();
     self.set_im(f(tmp))
  }

#[doc="Get the *const pointer for the RIS register."]
  #[inline] pub fn ris_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
#[doc="Get the *mut pointer for the RIS register."]
  #[inline] pub fn ris_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
#[doc="Read the RIS register."]
  #[inline] pub fn ris(&self) -> Ris { 
     unsafe {
        Ris(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
#[doc="Write the RIS register."]
  #[inline] pub fn set_ris(&self, value: Ris) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the RIS register."]
  #[inline] pub fn with_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
     let tmp = self.ris();
     self.set_ris(f(tmp))
  }

#[doc="Get the *const pointer for the MIS register."]
  #[inline] pub fn mis_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the MIS register."]
  #[inline] pub fn mis_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the MIS register."]
  #[inline] pub fn mis(&self) -> Mis { 
     unsafe {
        Mis(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
#[doc="Write the MIS register."]
  #[inline] pub fn set_mis(&self, value: Mis) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the MIS register."]
  #[inline] pub fn with_mis<F: FnOnce(Mis) -> Mis>(&self, f: F) -> &Self {
     let tmp = self.mis();
     self.set_mis(f(tmp))
  }

#[doc="Get the *const pointer for the ICR register."]
  #[inline] pub fn icr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
#[doc="Get the *mut pointer for the ICR register."]
  #[inline] pub fn icr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
#[doc="Write the ICR register."]
  #[inline] pub fn set_icr(&self, value: Icr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the DMACTL register."]
  #[inline] pub fn dmactl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
#[doc="Get the *mut pointer for the DMACTL register."]
  #[inline] pub fn dmactl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
#[doc="Read the DMACTL register."]
  #[inline] pub fn dmactl(&self) -> Dmactl { 
     unsafe {
        Dmactl(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }
#[doc="Write the DMACTL register."]
  #[inline] pub fn set_dmactl(&self, value: Dmactl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DMACTL register."]
  #[inline] pub fn with_dmactl<F: FnOnce(Dmactl) -> Dmactl>(&self, f: F) -> &Self {
     let tmp = self.dmactl();
     self.set_dmactl(f(tmp))
  }

#[doc="Get the *const pointer for the PP register."]
  #[inline] pub fn pp_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xfc0) as *const u32
  }
#[doc="Get the *mut pointer for the PP register."]
  #[inline] pub fn pp_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xfc0) as *mut u32
  }
#[doc="Read the PP register."]
  #[inline] pub fn pp(&self) -> Pp { 
     unsafe {
        Pp(::core::ptr::read_volatile(((self.0 as usize) + 0xfc0) as *const u32))
     }
  }
#[doc="Write the PP register."]
  #[inline] pub fn set_pp(&self, value: Pp) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xfc0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PP register."]
  #[inline] pub fn with_pp<F: FnOnce(Pp) -> Pp>(&self, f: F) -> &Self {
     let tmp = self.pp();
     self.set_pp(f(tmp))
  }

#[doc="Get the *const pointer for the CC register."]
  #[inline] pub fn cc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xfc8) as *const u32
  }
#[doc="Get the *mut pointer for the CC register."]
  #[inline] pub fn cc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xfc8) as *mut u32
  }
#[doc="Read the CC register."]
  #[inline] pub fn cc(&self) -> Cc { 
     unsafe {
        Cc(::core::ptr::read_volatile(((self.0 as usize) + 0xfc8) as *const u32))
     }
  }
#[doc="Write the CC register."]
  #[inline] pub fn set_cc(&self, value: Cc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xfc8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CC register."]
  #[inline] pub fn with_cc<F: FnOnce(Cc) -> Cc>(&self, f: F) -> &Self {
     let tmp = self.cc();
     self.set_cc(f(tmp))
  }

}

#[doc="SSI Control 0"]
#[derive(PartialEq, Eq)]
pub struct Cr0(pub u32);
impl Cr0 {
#[doc="SSI Data Size Select"]
  #[inline] pub fn dss(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="SSI Data Size Select"]
  #[inline] pub fn set_dss<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

#[doc="SSI Frame Format Select"]
  #[inline] pub fn frf(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
  }
#[doc="SSI Frame Format Select"]
  #[inline] pub fn set_frf<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="SSI Serial Clock Polarity"]
  #[inline] pub fn spo(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="SSI Serial Clock Polarity"]
  #[inline] pub fn set_spo<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="SSI Serial Clock Phase"]
  #[inline] pub fn sph(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
  }
#[doc="SSI Serial Clock Phase"]
  #[inline] pub fn set_sph<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="SSI Serial Clock Rate"]
  #[inline] pub fn scr(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
  }
#[doc="SSI Serial Clock Rate"]
  #[inline] pub fn set_scr<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 8);
     self.0 |= value << 8;
     self
  }

}
impl ::core::fmt::Display for Cr0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cr0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dss() != 0 { try!(write!(f, " dss=0x{:x}", self.dss()))}
      if self.frf() != 0 { try!(write!(f, " frf=0x{:x}", self.frf()))}
      if self.spo() != 0 { try!(write!(f, " spo"))}
      if self.sph() != 0 { try!(write!(f, " sph"))}
      if self.scr() != 0 { try!(write!(f, " scr=0x{:x}", self.scr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SSI Control 1"]
#[derive(PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
#[doc="SSI Loopback Mode"]
  #[inline] pub fn lbm(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="SSI Loopback Mode"]
  #[inline] pub fn set_lbm<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="SSI Synchronous Serial Port Enable"]
  #[inline] pub fn sse(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="SSI Synchronous Serial Port Enable"]
  #[inline] pub fn set_sse<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="SSI Master/Slave Select"]
  #[inline] pub fn ms(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="SSI Master/Slave Select"]
  #[inline] pub fn set_ms<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="End of Transmission"]
  #[inline] pub fn eot(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="End of Transmission"]
  #[inline] pub fn set_eot<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="SSI Mode"]
  #[inline] pub fn mode(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
  }
#[doc="SSI Mode"]
  #[inline] pub fn set_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="SSI Direction of Operation"]
  #[inline] pub fn dir(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
  }
#[doc="SSI Direction of Operation"]
  #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="High Speed Clock Enable"]
  #[inline] pub fn hsclken(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
  }
#[doc="High Speed Clock Enable"]
  #[inline] pub fn set_hsclken<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="FSS Hold Frame"]
  #[inline] pub fn fsshldfrm(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
  }
#[doc="FSS Hold Frame"]
  #[inline] pub fn set_fsshldfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Stop Frame (End of Message)"]
  #[inline] pub fn eom(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
  }
#[doc="Stop Frame (End of Message)"]
  #[inline] pub fn set_eom<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

}
impl ::core::fmt::Display for Cr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lbm() != 0 { try!(write!(f, " lbm"))}
      if self.sse() != 0 { try!(write!(f, " sse"))}
      if self.ms() != 0 { try!(write!(f, " ms"))}
      if self.eot() != 0 { try!(write!(f, " eot"))}
      if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
      if self.dir() != 0 { try!(write!(f, " dir"))}
      if self.hsclken() != 0 { try!(write!(f, " hsclken"))}
      if self.fsshldfrm() != 0 { try!(write!(f, " fsshldfrm"))}
      if self.eom() != 0 { try!(write!(f, " eom"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SSI Data"]
#[derive(PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
#[doc="SSI Receive/Transmit Data"]
  #[inline] pub fn data(&self) -> bits::U16 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
  }
#[doc="SSI Receive/Transmit Data"]
  #[inline] pub fn set_data<V: Into<bits::U16>>(mut self, value: V) -> Self {
     let value: bits::U16 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SSI Status"]
#[derive(PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
#[doc="SSI Transmit FIFO Empty"]
  #[inline] pub fn tfe(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="SSI Transmit FIFO Empty"]
  #[inline] pub fn set_tfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="SSI Transmit FIFO Not Full"]
  #[inline] pub fn tnf(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="SSI Transmit FIFO Not Full"]
  #[inline] pub fn set_tnf<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="SSI Receive FIFO Not Empty"]
  #[inline] pub fn rne(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="SSI Receive FIFO Not Empty"]
  #[inline] pub fn set_rne<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="SSI Receive FIFO Full"]
  #[inline] pub fn rff(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="SSI Receive FIFO Full"]
  #[inline] pub fn set_rff<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="SSI Busy Bit"]
  #[inline] pub fn bsy(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="SSI Busy Bit"]
  #[inline] pub fn set_bsy<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

}
impl ::core::fmt::Display for Sr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tfe() != 0 { try!(write!(f, " tfe"))}
      if self.tnf() != 0 { try!(write!(f, " tnf"))}
      if self.rne() != 0 { try!(write!(f, " rne"))}
      if self.rff() != 0 { try!(write!(f, " rff"))}
      if self.bsy() != 0 { try!(write!(f, " bsy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SSI Clock Prescale"]
#[derive(PartialEq, Eq)]
pub struct Cpsr(pub u32);
impl Cpsr {
#[doc="SSI Clock Prescale Divisor"]
  #[inline] pub fn cpsdvsr(&self) -> bits::U8 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
  }
#[doc="SSI Clock Prescale Divisor"]
  #[inline] pub fn set_cpsdvsr<V: Into<bits::U8>>(mut self, value: V) -> Self {
     let value: bits::U8 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cpsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cpsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cpsdvsr() != 0 { try!(write!(f, " cpsdvsr=0x{:x}", self.cpsdvsr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SSI Interrupt Mask"]
#[derive(PartialEq, Eq)]
pub struct Im(pub u32);
impl Im {
#[doc="SSI Receive Overrun Interrupt Mask"]
  #[inline] pub fn rorim(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="SSI Receive Overrun Interrupt Mask"]
  #[inline] pub fn set_rorim<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="SSI Receive Time-Out Interrupt Mask"]
  #[inline] pub fn rtim(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="SSI Receive Time-Out Interrupt Mask"]
  #[inline] pub fn set_rtim<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="SSI Receive FIFO Interrupt Mask"]
  #[inline] pub fn rxim(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="SSI Receive FIFO Interrupt Mask"]
  #[inline] pub fn set_rxim<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="SSI Transmit FIFO Interrupt Mask"]
  #[inline] pub fn txim(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="SSI Transmit FIFO Interrupt Mask"]
  #[inline] pub fn set_txim<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="SSI Receive DMA Interrupt Mask"]
  #[inline] pub fn dmarxim(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="SSI Receive DMA Interrupt Mask"]
  #[inline] pub fn set_dmarxim<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="SSI Transmit DMA Interrupt Mask"]
  #[inline] pub fn dmatxim(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="SSI Transmit DMA Interrupt Mask"]
  #[inline] pub fn set_dmatxim<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="End of Transmit Interrupt Mask"]
  #[inline] pub fn eotim(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="End of Transmit Interrupt Mask"]
  #[inline] pub fn set_eotim<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

}
impl ::core::fmt::Display for Im {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Im {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rorim() != 0 { try!(write!(f, " rorim"))}
      if self.rtim() != 0 { try!(write!(f, " rtim"))}
      if self.rxim() != 0 { try!(write!(f, " rxim"))}
      if self.txim() != 0 { try!(write!(f, " txim"))}
      if self.dmarxim() != 0 { try!(write!(f, " dmarxim"))}
      if self.dmatxim() != 0 { try!(write!(f, " dmatxim"))}
      if self.eotim() != 0 { try!(write!(f, " eotim"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SSI Raw Interrupt Status"]
#[derive(PartialEq, Eq)]
pub struct Ris(pub u32);
impl Ris {
#[doc="SSI Receive Overrun Raw Interrupt Status"]
  #[inline] pub fn rorris(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="SSI Receive Overrun Raw Interrupt Status"]
  #[inline] pub fn set_rorris<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="SSI Receive Time-Out Raw Interrupt Status"]
  #[inline] pub fn rtris(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="SSI Receive Time-Out Raw Interrupt Status"]
  #[inline] pub fn set_rtris<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="SSI Receive FIFO Raw Interrupt Status"]
  #[inline] pub fn rxris(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="SSI Receive FIFO Raw Interrupt Status"]
  #[inline] pub fn set_rxris<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="SSI Transmit FIFO Raw Interrupt Status"]
  #[inline] pub fn txris(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="SSI Transmit FIFO Raw Interrupt Status"]
  #[inline] pub fn set_txris<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="SSI Receive DMA Raw Interrupt Status"]
  #[inline] pub fn dmarxris(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="SSI Receive DMA Raw Interrupt Status"]
  #[inline] pub fn set_dmarxris<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="SSI Transmit DMA Raw Interrupt Status"]
  #[inline] pub fn dmatxris(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="SSI Transmit DMA Raw Interrupt Status"]
  #[inline] pub fn set_dmatxris<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="End of Transmit Raw Interrupt Status"]
  #[inline] pub fn eotris(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="End of Transmit Raw Interrupt Status"]
  #[inline] pub fn set_eotris<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

}
impl ::core::fmt::Display for Ris {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ris {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rorris() != 0 { try!(write!(f, " rorris"))}
      if self.rtris() != 0 { try!(write!(f, " rtris"))}
      if self.rxris() != 0 { try!(write!(f, " rxris"))}
      if self.txris() != 0 { try!(write!(f, " txris"))}
      if self.dmarxris() != 0 { try!(write!(f, " dmarxris"))}
      if self.dmatxris() != 0 { try!(write!(f, " dmatxris"))}
      if self.eotris() != 0 { try!(write!(f, " eotris"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SSI Masked Interrupt Status"]
#[derive(PartialEq, Eq)]
pub struct Mis(pub u32);
impl Mis {
#[doc="SSI Receive Overrun Masked Interrupt Status"]
  #[inline] pub fn rormis(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="SSI Receive Overrun Masked Interrupt Status"]
  #[inline] pub fn set_rormis<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="SSI Receive Time-Out Masked Interrupt Status"]
  #[inline] pub fn rtmis(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="SSI Receive Time-Out Masked Interrupt Status"]
  #[inline] pub fn set_rtmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="SSI Receive FIFO Masked Interrupt Status"]
  #[inline] pub fn rxmis(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
  }
#[doc="SSI Receive FIFO Masked Interrupt Status"]
  #[inline] pub fn set_rxmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="SSI Transmit FIFO Masked Interrupt Status"]
  #[inline] pub fn txmis(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="SSI Transmit FIFO Masked Interrupt Status"]
  #[inline] pub fn set_txmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="SSI Receive DMA Masked Interrupt Status"]
  #[inline] pub fn dmarxmis(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="SSI Receive DMA Masked Interrupt Status"]
  #[inline] pub fn set_dmarxmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="SSI Transmit DMA Masked Interrupt Status"]
  #[inline] pub fn dmatxmis(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="SSI Transmit DMA Masked Interrupt Status"]
  #[inline] pub fn set_dmatxmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="End of Transmit Masked Interrupt Status"]
  #[inline] pub fn eotmis(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="End of Transmit Masked Interrupt Status"]
  #[inline] pub fn set_eotmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

}
impl ::core::fmt::Display for Mis {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Mis {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rormis() != 0 { try!(write!(f, " rormis"))}
      if self.rtmis() != 0 { try!(write!(f, " rtmis"))}
      if self.rxmis() != 0 { try!(write!(f, " rxmis"))}
      if self.txmis() != 0 { try!(write!(f, " txmis"))}
      if self.dmarxmis() != 0 { try!(write!(f, " dmarxmis"))}
      if self.dmatxmis() != 0 { try!(write!(f, " dmatxmis"))}
      if self.eotmis() != 0 { try!(write!(f, " eotmis"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SSI Interrupt Clear"]
#[derive(PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
#[doc="SSI Receive Overrun Interrupt Clear"]
  #[inline] pub fn roric(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="SSI Receive Overrun Interrupt Clear"]
  #[inline] pub fn set_roric<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="SSI Receive Time-Out Interrupt Clear"]
  #[inline] pub fn rtic(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="SSI Receive Time-Out Interrupt Clear"]
  #[inline] pub fn set_rtic<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="SSI Receive DMA Interrupt Clear"]
  #[inline] pub fn dmarxic(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
  }
#[doc="SSI Receive DMA Interrupt Clear"]
  #[inline] pub fn set_dmarxic<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="SSI Transmit DMA Interrupt Clear"]
  #[inline] pub fn dmatxic(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
  }
#[doc="SSI Transmit DMA Interrupt Clear"]
  #[inline] pub fn set_dmatxic<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="End of Transmit Interrupt Clear"]
  #[inline] pub fn eotic(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
  }
#[doc="End of Transmit Interrupt Clear"]
  #[inline] pub fn set_eotic<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

}
impl ::core::fmt::Display for Icr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Icr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.roric() != 0 { try!(write!(f, " roric"))}
      if self.rtic() != 0 { try!(write!(f, " rtic"))}
      if self.dmarxic() != 0 { try!(write!(f, " dmarxic"))}
      if self.dmatxic() != 0 { try!(write!(f, " dmatxic"))}
      if self.eotic() != 0 { try!(write!(f, " eotic"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SSI DMA Control"]
#[derive(PartialEq, Eq)]
pub struct Dmactl(pub u32);
impl Dmactl {
#[doc="Receive DMA Enable"]
  #[inline] pub fn rxdmae(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Receive DMA Enable"]
  #[inline] pub fn set_rxdmae<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Transmit DMA Enable"]
  #[inline] pub fn txdmae(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
  }
#[doc="Transmit DMA Enable"]
  #[inline] pub fn set_txdmae<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

}
impl ::core::fmt::Display for Dmactl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dmactl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rxdmae() != 0 { try!(write!(f, " rxdmae"))}
      if self.txdmae() != 0 { try!(write!(f, " txdmae"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SSI Peripheral Properties"]
#[derive(PartialEq, Eq)]
pub struct Pp(pub u32);
impl Pp {
#[doc="High Speed Capability"]
  #[inline] pub fn hsclk(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="High Speed Capability"]
  #[inline] pub fn set_hsclk<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Mode of Operation"]
  #[inline] pub fn mode(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
  }
#[doc="Mode of Operation"]
  #[inline] pub fn set_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="FSS Hold Frame Capability"]
  #[inline] pub fn fsshldfrm(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="FSS Hold Frame Capability"]
  #[inline] pub fn set_fsshldfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
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
      if self.hsclk() != 0 { try!(write!(f, " hsclk"))}
      if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
      if self.fsshldfrm() != 0 { try!(write!(f, " fsshldfrm"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="SSI Clock Configuration"]
#[derive(PartialEq, Eq)]
pub struct Cc(pub u32);
impl Cc {
#[doc="SSI Baud Clock Source"]
  #[inline] pub fn cs(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
  }
#[doc="SSI Baud Clock Source"]
  #[inline] pub fn set_cs<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cs() != 0 { try!(write!(f, " cs=0x{:x}", self.cs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}