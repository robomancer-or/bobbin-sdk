//! FLASH
#[allow(unused_imports)] use bobbin_common::*;

periph!(FlashPeriph, FLASH, Flash, 0x40022000);

#[doc="FLASH"]
pub trait FlashPeriph : Base {
#[doc="Get the *const pointer for the ACR register."]
   #[inline] fn acr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Get the *mut pointer for the ACR register."]
   #[inline] fn acr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Read the ACR register."]
   #[inline] fn acr(&self) -> Acr { 
      unsafe {
         Acr(::core::ptr::read_volatile((self.base() + 0x0) as *const u32))
      }
   }
#[doc="Write the ACR register."]
   #[inline] fn set_acr<F: FnOnce(Acr) -> Acr>(&self, f: F) -> &Self {
      let value = f(Acr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the ACR register."]
   #[inline] fn with_acr<F: FnOnce(Acr) -> Acr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Acr(::core::ptr::read_volatile((self.base() + 0x0) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the KEYR register."]
   #[inline] fn keyr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Get the *mut pointer for the KEYR register."]
   #[inline] fn keyr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Write the KEYR register."]
   #[inline] fn set_keyr<F: FnOnce(Keyr) -> Keyr>(&self, f: F) -> &Self {
      let value = f(Keyr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the OPTKEYR register."]
   #[inline] fn optkeyr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Get the *mut pointer for the OPTKEYR register."]
   #[inline] fn optkeyr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Write the OPTKEYR register."]
   #[inline] fn set_optkeyr<F: FnOnce(Optkeyr) -> Optkeyr>(&self, f: F) -> &Self {
      let value = f(Optkeyr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x8) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SR register."]
   #[inline] fn sr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xc)
   }
#[doc="Get the *mut pointer for the SR register."]
   #[inline] fn sr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xc)
   }
#[doc="Read the SR register."]
   #[inline] fn sr(&self) -> Sr { 
      unsafe {
         Sr(::core::ptr::read_volatile((self.base() + 0xc) as *const u32))
      }
   }
#[doc="Write the SR register."]
   #[inline] fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
      let value = f(Sr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SR register."]
   #[inline] fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Sr(::core::ptr::read_volatile((self.base() + 0xc) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CR register."]
   #[inline] fn cr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x10)
   }
#[doc="Get the *mut pointer for the CR register."]
   #[inline] fn cr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x10)
   }
#[doc="Read the CR register."]
   #[inline] fn cr(&self) -> Cr { 
      unsafe {
         Cr(::core::ptr::read_volatile((self.base() + 0x10) as *const u32))
      }
   }
#[doc="Write the CR register."]
   #[inline] fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
      let value = f(Cr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x10) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CR register."]
   #[inline] fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Cr(::core::ptr::read_volatile((self.base() + 0x10) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x10) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the AR register."]
   #[inline] fn ar_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x14)
   }
#[doc="Get the *mut pointer for the AR register."]
   #[inline] fn ar_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x14)
   }
#[doc="Write the AR register."]
   #[inline] fn set_ar<F: FnOnce(Ar) -> Ar>(&self, f: F) -> &Self {
      let value = f(Ar(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x14) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the OBR register."]
   #[inline] fn obr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x1c)
   }
#[doc="Get the *mut pointer for the OBR register."]
   #[inline] fn obr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x1c)
   }
#[doc="Read the OBR register."]
   #[inline] fn obr(&self) -> Obr { 
      unsafe {
         Obr(::core::ptr::read_volatile((self.base() + 0x1c) as *const u32))
      }
   }

#[doc="Get the *const pointer for the WRPR register."]
   #[inline] fn wrpr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x20)
   }
#[doc="Get the *mut pointer for the WRPR register."]
   #[inline] fn wrpr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x20)
   }
#[doc="Read the WRPR register."]
   #[inline] fn wrpr(&self) -> Wrpr { 
      unsafe {
         Wrpr(::core::ptr::read_volatile((self.base() + 0x20) as *const u32))
      }
   }

}

#[doc="Flash access control register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Acr(pub u32);
impl Acr {
#[doc="Latency"]
   #[inline] pub fn latency(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
   }
#[doc="Latency"]
   #[inline] pub fn set_latency<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Flash half cycle access enable"]
   #[inline] pub fn hlfcya(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Flash half cycle access enable"]
   #[inline] pub fn set_hlfcya<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Prefetch buffer enable"]
   #[inline] pub fn prftbe(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Prefetch buffer enable"]
   #[inline] pub fn set_prftbe<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Prefetch buffer status"]
   #[inline] pub fn prftbs(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Prefetch buffer status"]
   #[inline] pub fn set_prftbs<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

}
impl ::core::fmt::Display for Acr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Acr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.latency() != 0 { try!(write!(f, " latency=0x{:x}", self.latency()))}
      if self.hlfcya() != 0 { try!(write!(f, " hlfcya"))}
      if self.prftbe() != 0 { try!(write!(f, " prftbe"))}
      if self.prftbs() != 0 { try!(write!(f, " prftbs"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Flash key register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Keyr(pub u32);
impl Keyr {
#[doc="FPEC key"]
   #[inline] pub fn key(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="FPEC key"]
   #[inline] pub fn set_key<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Keyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Keyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Flash option key register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Optkeyr(pub u32);
impl Optkeyr {
#[doc="Option byte key"]
   #[inline] pub fn optkey(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Option byte key"]
   #[inline] pub fn set_optkey<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Optkeyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Optkeyr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Status register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
#[doc="End of operation"]
   #[inline] pub fn eop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="End of operation"]
   #[inline] pub fn set_eop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Write protection error"]
   #[inline] pub fn wrprterr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Write protection error"]
   #[inline] pub fn set_wrprterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Programming error"]
   #[inline] pub fn pgerr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Programming error"]
   #[inline] pub fn set_pgerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Busy"]
   #[inline] pub fn bsy(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Busy"]
   #[inline] pub fn set_bsy<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
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
      if self.eop() != 0 { try!(write!(f, " eop"))}
      if self.wrprterr() != 0 { try!(write!(f, " wrprterr"))}
      if self.pgerr() != 0 { try!(write!(f, " pgerr"))}
      if self.bsy() != 0 { try!(write!(f, " bsy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Control register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
#[doc="Programming"]
   #[inline] pub fn pg(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Programming"]
   #[inline] pub fn set_pg<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Page Erase"]
   #[inline] pub fn per(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Page Erase"]
   #[inline] pub fn set_per<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Mass Erase"]
   #[inline] pub fn mer(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Mass Erase"]
   #[inline] pub fn set_mer<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Option byte programming"]
   #[inline] pub fn optpg(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Option byte programming"]
   #[inline] pub fn set_optpg<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Option byte erase"]
   #[inline] pub fn opter(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Option byte erase"]
   #[inline] pub fn set_opter<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Start"]
   #[inline] pub fn strt(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Start"]
   #[inline] pub fn set_strt<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Lock"]
   #[inline] pub fn lock(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Lock"]
   #[inline] pub fn set_lock<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="Option bytes write enable"]
   #[inline] pub fn optwre(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="Option bytes write enable"]
   #[inline] pub fn set_optwre<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="Error interrupt enable"]
   #[inline] pub fn errie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="Error interrupt enable"]
   #[inline] pub fn set_errie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="End of operation interrupt enable"]
   #[inline] pub fn eopie(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="End of operation interrupt enable"]
   #[inline] pub fn set_eopie<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
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
      if self.pg() != 0 { try!(write!(f, " pg"))}
      if self.per() != 0 { try!(write!(f, " per"))}
      if self.mer() != 0 { try!(write!(f, " mer"))}
      if self.optpg() != 0 { try!(write!(f, " optpg"))}
      if self.opter() != 0 { try!(write!(f, " opter"))}
      if self.strt() != 0 { try!(write!(f, " strt"))}
      if self.lock() != 0 { try!(write!(f, " lock"))}
      if self.optwre() != 0 { try!(write!(f, " optwre"))}
      if self.errie() != 0 { try!(write!(f, " errie"))}
      if self.eopie() != 0 { try!(write!(f, " eopie"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Flash address register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ar(pub u32);
impl Ar {
#[doc="Flash Address"]
   #[inline] pub fn far(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Flash Address"]
   #[inline] pub fn set_far<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Ar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Option byte register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Obr(pub u32);
impl Obr {
#[doc="Option byte error"]
   #[inline] pub fn opterr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Option byte error"]
   #[inline] pub fn set_opterr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Read protection"]
   #[inline] pub fn rdprt(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Read protection"]
   #[inline] pub fn set_rdprt<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="WDG_SW"]
   #[inline] pub fn wdg_sw(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="WDG_SW"]
   #[inline] pub fn set_wdg_sw<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="nRST_STOP"]
   #[inline] pub fn nrst_stop(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="nRST_STOP"]
   #[inline] pub fn set_nrst_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="nRST_STDBY"]
   #[inline] pub fn nrst_stdby(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="nRST_STDBY"]
   #[inline] pub fn set_nrst_stdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Data0"]
   #[inline] pub fn data0(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0xff) as u8) } // [17:10]
   }
#[doc="Data0"]
   #[inline] pub fn set_data0<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 10);
      self.0 |= value << 10;
      self
   }

#[doc="Data1"]
   #[inline] pub fn data1(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0xff) as u8) } // [25:18]
   }
#[doc="Data1"]
   #[inline] pub fn set_data1<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 18);
      self.0 |= value << 18;
      self
   }

}
impl ::core::fmt::Display for Obr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Obr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.opterr() != 0 { try!(write!(f, " opterr"))}
      if self.rdprt() != 0 { try!(write!(f, " rdprt"))}
      if self.wdg_sw() != 0 { try!(write!(f, " wdg_sw"))}
      if self.nrst_stop() != 0 { try!(write!(f, " nrst_stop"))}
      if self.nrst_stdby() != 0 { try!(write!(f, " nrst_stdby"))}
      if self.data0() != 0 { try!(write!(f, " data0=0x{:x}", self.data0()))}
      if self.data1() != 0 { try!(write!(f, " data1=0x{:x}", self.data1()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Write protection register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wrpr(pub u32);
impl Wrpr {
#[doc="Write protect"]
   #[inline] pub fn wrp(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Write protect"]
   #[inline] pub fn set_wrp<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Wrpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wrpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

