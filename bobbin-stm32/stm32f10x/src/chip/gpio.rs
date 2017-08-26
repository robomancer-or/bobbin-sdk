#[allow(unused_imports)] use bobbin_common::*;

periph!(GpioPeriph, GPIOA, Gpioa, 0x40010800);
periph!(GpioPeriph, GPIOB, Gpiob, 0x40010c00);
periph!(GpioPeriph, GPIOC, Gpioc, 0x40011000);
periph!(GpioPeriph, GPIOD, Gpiod, 0x40011400);
periph!(GpioPeriph, GPIOE, Gpioe, 0x40011800);
periph!(GpioPeriph, GPIOF, Gpiof, 0x40011c00);
periph!(GpioPeriph, GPIOG, Gpiog, 0x40012000);









pub trait GpioPeriph : Base {
#[doc="Get the *const pointer for the CRL register."]
   #[inline] fn crl_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Get the *mut pointer for the CRL register."]
   #[inline] fn crl_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Read the CRL register."]
   #[inline] fn crl(&self) -> Crl { 
      unsafe {
         Crl(::core::ptr::read_volatile((self.base() + 0x0) as *const u32))
      }
   }
#[doc="Write the CRL register."]
   #[inline] fn set_crl<F: FnOnce(Crl) -> Crl>(&self, f: F) -> &Self {
      let value = f(Crl(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CRL register."]
   #[inline] fn with_crl<F: FnOnce(Crl) -> Crl>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Crl(::core::ptr::read_volatile((self.base() + 0x0) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CRH register."]
   #[inline] fn crh_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Get the *mut pointer for the CRH register."]
   #[inline] fn crh_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Read the CRH register."]
   #[inline] fn crh(&self) -> Crh { 
      unsafe {
         Crh(::core::ptr::read_volatile((self.base() + 0x4) as *const u32))
      }
   }
#[doc="Write the CRH register."]
   #[inline] fn set_crh<F: FnOnce(Crh) -> Crh>(&self, f: F) -> &Self {
      let value = f(Crh(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CRH register."]
   #[inline] fn with_crh<F: FnOnce(Crh) -> Crh>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Crh(::core::ptr::read_volatile((self.base() + 0x4) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the IDR register."]
   #[inline] fn idr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Get the *mut pointer for the IDR register."]
   #[inline] fn idr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Read the IDR register."]
   #[inline] fn idr(&self) -> Idr { 
      unsafe {
         Idr(::core::ptr::read_volatile((self.base() + 0x8) as *const u32))
      }
   }

#[doc="Get the *const pointer for the ODR register."]
   #[inline] fn odr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xc)
   }
#[doc="Get the *mut pointer for the ODR register."]
   #[inline] fn odr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xc)
   }
#[doc="Read the ODR register."]
   #[inline] fn odr(&self) -> Odr { 
      unsafe {
         Odr(::core::ptr::read_volatile((self.base() + 0xc) as *const u32))
      }
   }
#[doc="Write the ODR register."]
   #[inline] fn set_odr<F: FnOnce(Odr) -> Odr>(&self, f: F) -> &Self {
      let value = f(Odr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the ODR register."]
   #[inline] fn with_odr<F: FnOnce(Odr) -> Odr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Odr(::core::ptr::read_volatile((self.base() + 0xc) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the BSRR register."]
   #[inline] fn bsrr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x10)
   }
#[doc="Get the *mut pointer for the BSRR register."]
   #[inline] fn bsrr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x10)
   }
#[doc="Write the BSRR register."]
   #[inline] fn set_bsrr<F: FnOnce(Bsrr) -> Bsrr>(&self, f: F) -> &Self {
      let value = f(Bsrr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x10) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the BRR register."]
   #[inline] fn brr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x14)
   }
#[doc="Get the *mut pointer for the BRR register."]
   #[inline] fn brr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x14)
   }
#[doc="Write the BRR register."]
   #[inline] fn set_brr<F: FnOnce(Brr) -> Brr>(&self, f: F) -> &Self {
      let value = f(Brr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x14) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the LCKR register."]
   #[inline] fn lckr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x18)
   }
#[doc="Get the *mut pointer for the LCKR register."]
   #[inline] fn lckr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x18)
   }
#[doc="Read the LCKR register."]
   #[inline] fn lckr(&self) -> Lckr { 
      unsafe {
         Lckr(::core::ptr::read_volatile((self.base() + 0x18) as *const u32))
      }
   }
#[doc="Write the LCKR register."]
   #[inline] fn set_lckr<F: FnOnce(Lckr) -> Lckr>(&self, f: F) -> &Self {
      let value = f(Lckr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x18) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the LCKR register."]
   #[inline] fn with_lckr<F: FnOnce(Lckr) -> Lckr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Lckr(::core::ptr::read_volatile((self.base() + 0x18) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x18) as *mut u32, value.0);
      }
      self
   }

}

#[doc="Port configuration register low (GPIOn_CRL)"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Crl(pub u32);
impl Crl {
#[doc="Port n.m mode bits"]
   #[inline] pub fn mode<I: Into<bits::R8>>(&self, index: I) -> bits::U2 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + (index << 2);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x3) as u8) } // [1:0]
   }
#[doc="Port n.m mode bits"]
   #[inline] pub fn set_mode<I: Into<bits::R8>, V: Into<bits::U2>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + (index << 2);
      self.0 &= !(0x3 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="Port n.m configuration bits"]
   #[inline] pub fn cnf<I: Into<bits::R8>>(&self, index: I) -> bits::U2 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 2 + (index << 2);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x3) as u8) } // [3:2]
   }
#[doc="Port n.m configuration bits"]
   #[inline] pub fn set_cnf<I: Into<bits::R8>, V: Into<bits::U2>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      let shift: usize = 2 + (index << 2);
      self.0 &= !(0x3 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Crl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Crl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mode(0) != 0 { try!(write!(f, " mode[0]=0x{:x}", self.mode(0)))}
      if self.mode(1) != 0 { try!(write!(f, " mode[1]=0x{:x}", self.mode(1)))}
      if self.mode(2) != 0 { try!(write!(f, " mode[2]=0x{:x}", self.mode(2)))}
      if self.mode(3) != 0 { try!(write!(f, " mode[3]=0x{:x}", self.mode(3)))}
      if self.mode(4) != 0 { try!(write!(f, " mode[4]=0x{:x}", self.mode(4)))}
      if self.mode(5) != 0 { try!(write!(f, " mode[5]=0x{:x}", self.mode(5)))}
      if self.mode(6) != 0 { try!(write!(f, " mode[6]=0x{:x}", self.mode(6)))}
      if self.mode(7) != 0 { try!(write!(f, " mode[7]=0x{:x}", self.mode(7)))}
      if self.cnf(0) != 0 { try!(write!(f, " cnf[0]=0x{:x}", self.cnf(0)))}
      if self.cnf(1) != 0 { try!(write!(f, " cnf[1]=0x{:x}", self.cnf(1)))}
      if self.cnf(2) != 0 { try!(write!(f, " cnf[2]=0x{:x}", self.cnf(2)))}
      if self.cnf(3) != 0 { try!(write!(f, " cnf[3]=0x{:x}", self.cnf(3)))}
      if self.cnf(4) != 0 { try!(write!(f, " cnf[4]=0x{:x}", self.cnf(4)))}
      if self.cnf(5) != 0 { try!(write!(f, " cnf[5]=0x{:x}", self.cnf(5)))}
      if self.cnf(6) != 0 { try!(write!(f, " cnf[6]=0x{:x}", self.cnf(6)))}
      if self.cnf(7) != 0 { try!(write!(f, " cnf[7]=0x{:x}", self.cnf(7)))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Port configuration register high (GPIOn_CRL)"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Crh(pub u32);
impl Crh {
#[doc="Port n.m mode bits"]
   #[inline] pub fn mode<I: Into<bits::R8>>(&self, index: I) -> bits::U2 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + (index << 2);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x3) as u8) } // [1:0]
   }
#[doc="Port n.m mode bits"]
   #[inline] pub fn set_mode<I: Into<bits::R8>, V: Into<bits::U2>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + (index << 2);
      self.0 &= !(0x3 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="Port n.m configuration bits"]
   #[inline] pub fn cnf<I: Into<bits::R8>>(&self, index: I) -> bits::U2 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 2 + (index << 2);
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x3) as u8) } // [3:2]
   }
#[doc="Port n.m configuration bits"]
   #[inline] pub fn set_cnf<I: Into<bits::R8>, V: Into<bits::U2>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      let shift: usize = 2 + (index << 2);
      self.0 &= !(0x3 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Crh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Crh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mode(0) != 0 { try!(write!(f, " mode[0]=0x{:x}", self.mode(0)))}
      if self.mode(1) != 0 { try!(write!(f, " mode[1]=0x{:x}", self.mode(1)))}
      if self.mode(2) != 0 { try!(write!(f, " mode[2]=0x{:x}", self.mode(2)))}
      if self.mode(3) != 0 { try!(write!(f, " mode[3]=0x{:x}", self.mode(3)))}
      if self.mode(4) != 0 { try!(write!(f, " mode[4]=0x{:x}", self.mode(4)))}
      if self.mode(5) != 0 { try!(write!(f, " mode[5]=0x{:x}", self.mode(5)))}
      if self.mode(6) != 0 { try!(write!(f, " mode[6]=0x{:x}", self.mode(6)))}
      if self.mode(7) != 0 { try!(write!(f, " mode[7]=0x{:x}", self.mode(7)))}
      if self.cnf(0) != 0 { try!(write!(f, " cnf[0]=0x{:x}", self.cnf(0)))}
      if self.cnf(1) != 0 { try!(write!(f, " cnf[1]=0x{:x}", self.cnf(1)))}
      if self.cnf(2) != 0 { try!(write!(f, " cnf[2]=0x{:x}", self.cnf(2)))}
      if self.cnf(3) != 0 { try!(write!(f, " cnf[3]=0x{:x}", self.cnf(3)))}
      if self.cnf(4) != 0 { try!(write!(f, " cnf[4]=0x{:x}", self.cnf(4)))}
      if self.cnf(5) != 0 { try!(write!(f, " cnf[5]=0x{:x}", self.cnf(5)))}
      if self.cnf(6) != 0 { try!(write!(f, " cnf[6]=0x{:x}", self.cnf(6)))}
      if self.cnf(7) != 0 { try!(write!(f, " cnf[7]=0x{:x}", self.cnf(7)))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Port input data register (GPIOn_IDR)"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Idr(pub u32);
impl Idr {
#[doc="Port input data"]
   #[inline] pub fn idr<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
      let index: bits::R16 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Port input data"]
   #[inline] pub fn set_idr<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Idr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Idr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.idr(0) != 0 { try!(write!(f, " idr[0]"))}
      if self.idr(1) != 0 { try!(write!(f, " idr[1]"))}
      if self.idr(2) != 0 { try!(write!(f, " idr[2]"))}
      if self.idr(3) != 0 { try!(write!(f, " idr[3]"))}
      if self.idr(4) != 0 { try!(write!(f, " idr[4]"))}
      if self.idr(5) != 0 { try!(write!(f, " idr[5]"))}
      if self.idr(6) != 0 { try!(write!(f, " idr[6]"))}
      if self.idr(7) != 0 { try!(write!(f, " idr[7]"))}
      if self.idr(8) != 0 { try!(write!(f, " idr[8]"))}
      if self.idr(9) != 0 { try!(write!(f, " idr[9]"))}
      if self.idr(10) != 0 { try!(write!(f, " idr[10]"))}
      if self.idr(11) != 0 { try!(write!(f, " idr[11]"))}
      if self.idr(12) != 0 { try!(write!(f, " idr[12]"))}
      if self.idr(13) != 0 { try!(write!(f, " idr[13]"))}
      if self.idr(14) != 0 { try!(write!(f, " idr[14]"))}
      if self.idr(15) != 0 { try!(write!(f, " idr[15]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Port output data register (GPIOn_ODR)"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Odr(pub u32);
impl Odr {
#[doc="Port output data"]
   #[inline] pub fn odr<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
      let index: bits::R16 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Port output data"]
   #[inline] pub fn set_odr<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Odr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Odr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.odr(0) != 0 { try!(write!(f, " odr[0]"))}
      if self.odr(1) != 0 { try!(write!(f, " odr[1]"))}
      if self.odr(2) != 0 { try!(write!(f, " odr[2]"))}
      if self.odr(3) != 0 { try!(write!(f, " odr[3]"))}
      if self.odr(4) != 0 { try!(write!(f, " odr[4]"))}
      if self.odr(5) != 0 { try!(write!(f, " odr[5]"))}
      if self.odr(6) != 0 { try!(write!(f, " odr[6]"))}
      if self.odr(7) != 0 { try!(write!(f, " odr[7]"))}
      if self.odr(8) != 0 { try!(write!(f, " odr[8]"))}
      if self.odr(9) != 0 { try!(write!(f, " odr[9]"))}
      if self.odr(10) != 0 { try!(write!(f, " odr[10]"))}
      if self.odr(11) != 0 { try!(write!(f, " odr[11]"))}
      if self.odr(12) != 0 { try!(write!(f, " odr[12]"))}
      if self.odr(13) != 0 { try!(write!(f, " odr[13]"))}
      if self.odr(14) != 0 { try!(write!(f, " odr[14]"))}
      if self.odr(15) != 0 { try!(write!(f, " odr[15]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Port bit set/reset register (GPIOn_BSRR)"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bsrr(pub u32);
impl Bsrr {
#[doc="Set bit n"]
   #[inline] pub fn bs<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
      let index: bits::R16 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Set bit n"]
   #[inline] pub fn set_bs<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="Reset bit n"]
   #[inline] pub fn br<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
      let index: bits::R16 = index.into();
      let index: usize = index.value();
      let shift: usize = 16 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [16]
   }
#[doc="Reset bit n"]
   #[inline] pub fn set_br<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 16 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Bsrr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Bsrr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.bs(0) != 0 { try!(write!(f, " bs[0]"))}
      if self.bs(1) != 0 { try!(write!(f, " bs[1]"))}
      if self.bs(2) != 0 { try!(write!(f, " bs[2]"))}
      if self.bs(3) != 0 { try!(write!(f, " bs[3]"))}
      if self.bs(4) != 0 { try!(write!(f, " bs[4]"))}
      if self.bs(5) != 0 { try!(write!(f, " bs[5]"))}
      if self.bs(6) != 0 { try!(write!(f, " bs[6]"))}
      if self.bs(7) != 0 { try!(write!(f, " bs[7]"))}
      if self.bs(8) != 0 { try!(write!(f, " bs[8]"))}
      if self.bs(9) != 0 { try!(write!(f, " bs[9]"))}
      if self.bs(10) != 0 { try!(write!(f, " bs[10]"))}
      if self.bs(11) != 0 { try!(write!(f, " bs[11]"))}
      if self.bs(12) != 0 { try!(write!(f, " bs[12]"))}
      if self.bs(13) != 0 { try!(write!(f, " bs[13]"))}
      if self.bs(14) != 0 { try!(write!(f, " bs[14]"))}
      if self.bs(15) != 0 { try!(write!(f, " bs[15]"))}
      if self.br(0) != 0 { try!(write!(f, " br[0]"))}
      if self.br(1) != 0 { try!(write!(f, " br[1]"))}
      if self.br(2) != 0 { try!(write!(f, " br[2]"))}
      if self.br(3) != 0 { try!(write!(f, " br[3]"))}
      if self.br(4) != 0 { try!(write!(f, " br[4]"))}
      if self.br(5) != 0 { try!(write!(f, " br[5]"))}
      if self.br(6) != 0 { try!(write!(f, " br[6]"))}
      if self.br(7) != 0 { try!(write!(f, " br[7]"))}
      if self.br(8) != 0 { try!(write!(f, " br[8]"))}
      if self.br(9) != 0 { try!(write!(f, " br[9]"))}
      if self.br(10) != 0 { try!(write!(f, " br[10]"))}
      if self.br(11) != 0 { try!(write!(f, " br[11]"))}
      if self.br(12) != 0 { try!(write!(f, " br[12]"))}
      if self.br(13) != 0 { try!(write!(f, " br[13]"))}
      if self.br(14) != 0 { try!(write!(f, " br[14]"))}
      if self.br(15) != 0 { try!(write!(f, " br[15]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Port bit reset register (GPIOn_BRR)"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Brr(pub u32);
impl Brr {
#[doc="Reset bit n"]
   #[inline] pub fn br<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
      let index: bits::R16 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Reset bit n"]
   #[inline] pub fn set_br<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Brr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Brr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.br(0) != 0 { try!(write!(f, " br[0]"))}
      if self.br(1) != 0 { try!(write!(f, " br[1]"))}
      if self.br(2) != 0 { try!(write!(f, " br[2]"))}
      if self.br(3) != 0 { try!(write!(f, " br[3]"))}
      if self.br(4) != 0 { try!(write!(f, " br[4]"))}
      if self.br(5) != 0 { try!(write!(f, " br[5]"))}
      if self.br(6) != 0 { try!(write!(f, " br[6]"))}
      if self.br(7) != 0 { try!(write!(f, " br[7]"))}
      if self.br(8) != 0 { try!(write!(f, " br[8]"))}
      if self.br(9) != 0 { try!(write!(f, " br[9]"))}
      if self.br(10) != 0 { try!(write!(f, " br[10]"))}
      if self.br(11) != 0 { try!(write!(f, " br[11]"))}
      if self.br(12) != 0 { try!(write!(f, " br[12]"))}
      if self.br(13) != 0 { try!(write!(f, " br[13]"))}
      if self.br(14) != 0 { try!(write!(f, " br[14]"))}
      if self.br(15) != 0 { try!(write!(f, " br[15]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Port configuration lock register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Lckr(pub u32);
impl Lckr {
#[doc="Port Lock bit n"]
   #[inline] pub fn lck0<I: Into<bits::R16>>(&self, index: I) -> bits::U1 {
      let index: bits::R16 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Port Lock bit n"]
   #[inline] pub fn set_lck0<I: Into<bits::R16>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R16 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="Lock key"]
   #[inline] pub fn lckk(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="Lock key"]
   #[inline] pub fn set_lckk<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

}
impl ::core::fmt::Display for Lckr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Lckr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.lck0(0) != 0 { try!(write!(f, " lck0[0]"))}
      if self.lck0(1) != 0 { try!(write!(f, " lck0[1]"))}
      if self.lck0(2) != 0 { try!(write!(f, " lck0[2]"))}
      if self.lck0(3) != 0 { try!(write!(f, " lck0[3]"))}
      if self.lck0(4) != 0 { try!(write!(f, " lck0[4]"))}
      if self.lck0(5) != 0 { try!(write!(f, " lck0[5]"))}
      if self.lck0(6) != 0 { try!(write!(f, " lck0[6]"))}
      if self.lck0(7) != 0 { try!(write!(f, " lck0[7]"))}
      if self.lck0(8) != 0 { try!(write!(f, " lck0[8]"))}
      if self.lck0(9) != 0 { try!(write!(f, " lck0[9]"))}
      if self.lck0(10) != 0 { try!(write!(f, " lck0[10]"))}
      if self.lck0(11) != 0 { try!(write!(f, " lck0[11]"))}
      if self.lck0(12) != 0 { try!(write!(f, " lck0[12]"))}
      if self.lck0(13) != 0 { try!(write!(f, " lck0[13]"))}
      if self.lck0(14) != 0 { try!(write!(f, " lck0[14]"))}
      if self.lck0(15) != 0 { try!(write!(f, " lck0[15]"))}
      if self.lckk() != 0 { try!(write!(f, " lckk"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
pin!(PA0, Pa0, GPIOA, Gpioa, 0);

pin!(PA1, Pa1, GPIOA, Gpioa, 1);

pin!(PA2, Pa2, GPIOA, Gpioa, 2);

pin!(PA3, Pa3, GPIOA, Gpioa, 3);

pin!(PA4, Pa4, GPIOA, Gpioa, 4);

pin!(PA5, Pa5, GPIOA, Gpioa, 5);

pin!(PA6, Pa6, GPIOA, Gpioa, 6);

pin!(PA7, Pa7, GPIOA, Gpioa, 7);

pin!(PA8, Pa8, GPIOA, Gpioa, 8);

pin!(PA9, Pa9, GPIOA, Gpioa, 9);

pin!(PA10, Pa10, GPIOA, Gpioa, 10);

pin!(PA11, Pa11, GPIOA, Gpioa, 11);

pin!(PA12, Pa12, GPIOA, Gpioa, 12);

pin!(PA13, Pa13, GPIOA, Gpioa, 13);

pin!(PA14, Pa14, GPIOA, Gpioa, 14);

pin!(PA15, Pa15, GPIOA, Gpioa, 15);

pin!(PB0, Pb0, GPIOB, Gpiob, 0);

pin!(PB1, Pb1, GPIOB, Gpiob, 1);

pin!(PB2, Pb2, GPIOB, Gpiob, 2);

pin!(PB3, Pb3, GPIOB, Gpiob, 3);

pin!(PB4, Pb4, GPIOB, Gpiob, 4);

pin!(PB5, Pb5, GPIOB, Gpiob, 5);

pin!(PB6, Pb6, GPIOB, Gpiob, 6);

pin!(PB7, Pb7, GPIOB, Gpiob, 7);

pin!(PB8, Pb8, GPIOB, Gpiob, 8);

pin!(PB9, Pb9, GPIOB, Gpiob, 9);

pin!(PB10, Pb10, GPIOB, Gpiob, 10);

pin!(PB11, Pb11, GPIOB, Gpiob, 11);

pin!(PB12, Pb12, GPIOB, Gpiob, 12);

pin!(PB13, Pb13, GPIOB, Gpiob, 13);

pin!(PB14, Pb14, GPIOB, Gpiob, 14);

pin!(PB15, Pb15, GPIOB, Gpiob, 15);

pin!(PC0, Pc0, GPIOC, Gpioc, 0);

pin!(PC1, Pc1, GPIOC, Gpioc, 1);

pin!(PC2, Pc2, GPIOC, Gpioc, 2);

pin!(PC3, Pc3, GPIOC, Gpioc, 3);

pin!(PC4, Pc4, GPIOC, Gpioc, 4);

pin!(PC5, Pc5, GPIOC, Gpioc, 5);

pin!(PC6, Pc6, GPIOC, Gpioc, 6);

pin!(PC7, Pc7, GPIOC, Gpioc, 7);

pin!(PC8, Pc8, GPIOC, Gpioc, 8);

pin!(PC9, Pc9, GPIOC, Gpioc, 9);

pin!(PC10, Pc10, GPIOC, Gpioc, 10);

pin!(PC11, Pc11, GPIOC, Gpioc, 11);

pin!(PC12, Pc12, GPIOC, Gpioc, 12);

pin!(PC13, Pc13, GPIOC, Gpioc, 13);

pin!(PC14, Pc14, GPIOC, Gpioc, 14);

pin!(PC15, Pc15, GPIOC, Gpioc, 15);

pin!(PD0, Pd0, GPIOD, Gpiod, 0);

pin!(PD1, Pd1, GPIOD, Gpiod, 1);

pin!(PD2, Pd2, GPIOD, Gpiod, 2);

pin!(PD3, Pd3, GPIOD, Gpiod, 3);

pin!(PD4, Pd4, GPIOD, Gpiod, 4);

pin!(PD5, Pd5, GPIOD, Gpiod, 5);

pin!(PD6, Pd6, GPIOD, Gpiod, 6);

pin!(PD7, Pd7, GPIOD, Gpiod, 7);

pin!(PD8, Pd8, GPIOD, Gpiod, 8);

pin!(PD9, Pd9, GPIOD, Gpiod, 9);

pin!(PD10, Pd10, GPIOD, Gpiod, 10);

pin!(PD11, Pd11, GPIOD, Gpiod, 11);

pin!(PD12, Pd12, GPIOD, Gpiod, 12);

pin!(PD13, Pd13, GPIOD, Gpiod, 13);

pin!(PD14, Pd14, GPIOD, Gpiod, 14);

pin!(PD15, Pd15, GPIOD, Gpiod, 15);

pin!(PE0, Pe0, GPIOE, Gpioe, 0);

pin!(PE1, Pe1, GPIOE, Gpioe, 1);

pin!(PE2, Pe2, GPIOE, Gpioe, 2);

pin!(PE3, Pe3, GPIOE, Gpioe, 3);

pin!(PE4, Pe4, GPIOE, Gpioe, 4);

pin!(PE5, Pe5, GPIOE, Gpioe, 5);

pin!(PE6, Pe6, GPIOE, Gpioe, 6);

pin!(PE7, Pe7, GPIOE, Gpioe, 7);

pin!(PE8, Pe8, GPIOE, Gpioe, 8);

pin!(PE9, Pe9, GPIOE, Gpioe, 9);

pin!(PE10, Pe10, GPIOE, Gpioe, 10);

pin!(PE11, Pe11, GPIOE, Gpioe, 11);

pin!(PE12, Pe12, GPIOE, Gpioe, 12);

pin!(PE13, Pe13, GPIOE, Gpioe, 13);

pin!(PE14, Pe14, GPIOE, Gpioe, 14);

pin!(PE15, Pe15, GPIOE, Gpioe, 15);

pin!(PF0, Pf0, GPIOF, Gpiof, 0);

pin!(PF1, Pf1, GPIOF, Gpiof, 1);

pin!(PF2, Pf2, GPIOF, Gpiof, 2);

pin!(PF3, Pf3, GPIOF, Gpiof, 3);

pin!(PF4, Pf4, GPIOF, Gpiof, 4);

pin!(PF5, Pf5, GPIOF, Gpiof, 5);

pin!(PF6, Pf6, GPIOF, Gpiof, 6);

pin!(PF7, Pf7, GPIOF, Gpiof, 7);

pin!(PF8, Pf8, GPIOF, Gpiof, 8);

pin!(PF9, Pf9, GPIOF, Gpiof, 9);

pin!(PF10, Pf10, GPIOF, Gpiof, 10);

pin!(PF11, Pf11, GPIOF, Gpiof, 11);

pin!(PF12, Pf12, GPIOF, Gpiof, 12);

pin!(PF13, Pf13, GPIOF, Gpiof, 13);

pin!(PF14, Pf14, GPIOF, Gpiof, 14);

pin!(PF15, Pf15, GPIOF, Gpiof, 15);

pin!(PG0, Pg0, GPIOG, Gpiog, 0);

pin!(PG1, Pg1, GPIOG, Gpiog, 1);

pin!(PG2, Pg2, GPIOG, Gpiog, 2);

pin!(PG3, Pg3, GPIOG, Gpiog, 3);

pin!(PG4, Pg4, GPIOG, Gpiog, 4);

pin!(PG5, Pg5, GPIOG, Gpiog, 5);

pin!(PG6, Pg6, GPIOG, Gpiog, 6);

pin!(PG7, Pg7, GPIOG, Gpiog, 7);

pin!(PG8, Pg8, GPIOG, Gpiog, 8);

pin!(PG9, Pg9, GPIOG, Gpiog, 9);

pin!(PG10, Pg10, GPIOG, Gpiog, 10);

pin!(PG11, Pg11, GPIOG, Gpiog, 11);

pin!(PG12, Pg12, GPIOG, Gpiog, 12);

pin!(PG13, Pg13, GPIOG, Gpiog, 13);

pin!(PG14, Pg14, GPIOG, Gpiog, 14);

pin!(PG15, Pg15, GPIOG, Gpiog, 15);

