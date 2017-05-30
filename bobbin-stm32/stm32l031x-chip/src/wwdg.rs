pub const WWDG: Wwdg = Wwdg(0x40002c00);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wwdg(pub u32);
impl Wwdg {
  pub fn cr(&self) -> Cr { 
     unsafe {
       Cr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  pub fn set_cr(&self, value: Cr) -> &Wwdg {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Wwdg {
     let tmp = self.cr();
     self.set_cr(f(tmp))
  }

  pub fn cfr(&self) -> Cfr { 
     unsafe {
       Cfr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
  pub fn set_cfr(&self, value: Cfr) -> &Wwdg {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
  pub fn with_cfr<F: FnOnce(Cfr) -> Cfr>(&self, f: F) -> &Wwdg {
     let tmp = self.cfr();
     self.set_cfr(f(tmp))
  }

  pub fn sr(&self) -> Sr { 
     unsafe {
       Sr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
  pub fn set_sr(&self, value: Sr) -> &Wwdg {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
  pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Wwdg {
     let tmp = self.sr();
     self.set_sr(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
  pub fn wdga(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_wdga(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn t(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7f // [6:0]
  }
  pub fn set_t(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 0);
     self.0 |= value << 0;
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
      if self.wdga() != 0 { try!(write!(f, " wdga"))}
      if self.t() != 0 { try!(write!(f, " t=0x{:x}", self.t()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cfr(pub u32);
impl Cfr {
  pub fn ewi(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
  pub fn set_ewi(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

  pub fn wdgtb(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  pub fn set_wdgtb(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn w(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7f // [6:0]
  }
  pub fn set_w(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cfr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cfr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ewi() != 0 { try!(write!(f, " ewi"))}
      if self.wdgtb() != 0 { try!(write!(f, " wdgtb=0x{:x}", self.wdgtb()))}
      if self.w() != 0 { try!(write!(f, " w=0x{:x}", self.w()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
  pub fn ewif(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_ewif(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
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
      if self.ewif() != 0 { try!(write!(f, " ewif"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
