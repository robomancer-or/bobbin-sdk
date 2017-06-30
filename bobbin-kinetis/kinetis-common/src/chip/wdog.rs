
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="WDOG Peripheral"]
pub struct Periph<T>(pub u32, pub T); 



impl<T> Periph<T> {
#[doc="Get the *const pointer for the STCTRLH register."]
  #[inline] pub fn stctrlh_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x0) as *const u16
  }
#[doc="Get the *mut pointer for the STCTRLH register."]
  #[inline] pub fn stctrlh_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x0) as *mut u16
  }
#[doc="Read the STCTRLH register."]
  #[inline] pub fn stctrlh(&self) -> Stctrlh { 
     unsafe {
        Stctrlh(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u16))
     }
  }
#[doc="Write the STCTRLH register."]
  #[inline] pub fn set_stctrlh(&self, value: Stctrlh) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the STCTRLH register."]
  #[inline] pub fn with_stctrlh<F: FnOnce(Stctrlh) -> Stctrlh>(&self, f: F) -> &Self {
     let tmp = self.stctrlh();
     self.set_stctrlh(f(tmp))
  }

#[doc="Get the *const pointer for the STCTRLL register."]
  #[inline] pub fn stctrll_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x2) as *const u16
  }
#[doc="Get the *mut pointer for the STCTRLL register."]
  #[inline] pub fn stctrll_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x2) as *mut u16
  }
#[doc="Read the STCTRLL register."]
  #[inline] pub fn stctrll(&self) -> Stctrll { 
     unsafe {
        Stctrll(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u16))
     }
  }
#[doc="Write the STCTRLL register."]
  #[inline] pub fn set_stctrll(&self, value: Stctrll) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the STCTRLL register."]
  #[inline] pub fn with_stctrll<F: FnOnce(Stctrll) -> Stctrll>(&self, f: F) -> &Self {
     let tmp = self.stctrll();
     self.set_stctrll(f(tmp))
  }

#[doc="Get the *const pointer for the TOVALH register."]
  #[inline] pub fn tovalh_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x4) as *const u16
  }
#[doc="Get the *mut pointer for the TOVALH register."]
  #[inline] pub fn tovalh_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x4) as *mut u16
  }
#[doc="Read the TOVALH register."]
  #[inline] pub fn tovalh(&self) -> Tovalh { 
     unsafe {
        Tovalh(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u16))
     }
  }
#[doc="Write the TOVALH register."]
  #[inline] pub fn set_tovalh(&self, value: Tovalh) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the TOVALH register."]
  #[inline] pub fn with_tovalh<F: FnOnce(Tovalh) -> Tovalh>(&self, f: F) -> &Self {
     let tmp = self.tovalh();
     self.set_tovalh(f(tmp))
  }

#[doc="Get the *const pointer for the TOVALL register."]
  #[inline] pub fn tovall_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x6) as *const u16
  }
#[doc="Get the *mut pointer for the TOVALL register."]
  #[inline] pub fn tovall_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x6) as *mut u16
  }
#[doc="Read the TOVALL register."]
  #[inline] pub fn tovall(&self) -> Tovall { 
     unsafe {
        Tovall(::core::ptr::read_volatile(((self.0 as usize) + 0x6) as *const u16))
     }
  }
#[doc="Write the TOVALL register."]
  #[inline] pub fn set_tovall(&self, value: Tovall) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x6) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the TOVALL register."]
  #[inline] pub fn with_tovall<F: FnOnce(Tovall) -> Tovall>(&self, f: F) -> &Self {
     let tmp = self.tovall();
     self.set_tovall(f(tmp))
  }

#[doc="Get the *const pointer for the WINH register."]
  #[inline] pub fn winh_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x8) as *const u16
  }
#[doc="Get the *mut pointer for the WINH register."]
  #[inline] pub fn winh_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x8) as *mut u16
  }
#[doc="Read the WINH register."]
  #[inline] pub fn winh(&self) -> Winh { 
     unsafe {
        Winh(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u16))
     }
  }
#[doc="Write the WINH register."]
  #[inline] pub fn set_winh(&self, value: Winh) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the WINH register."]
  #[inline] pub fn with_winh<F: FnOnce(Winh) -> Winh>(&self, f: F) -> &Self {
     let tmp = self.winh();
     self.set_winh(f(tmp))
  }

#[doc="Get the *const pointer for the WINL register."]
  #[inline] pub fn winl_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0xa) as *const u16
  }
#[doc="Get the *mut pointer for the WINL register."]
  #[inline] pub fn winl_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0xa) as *mut u16
  }
#[doc="Read the WINL register."]
  #[inline] pub fn winl(&self) -> Winl { 
     unsafe {
        Winl(::core::ptr::read_volatile(((self.0 as usize) + 0xa) as *const u16))
     }
  }
#[doc="Write the WINL register."]
  #[inline] pub fn set_winl(&self, value: Winl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xa) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the WINL register."]
  #[inline] pub fn with_winl<F: FnOnce(Winl) -> Winl>(&self, f: F) -> &Self {
     let tmp = self.winl();
     self.set_winl(f(tmp))
  }

#[doc="Get the *const pointer for the REFRESH register."]
  #[inline] pub fn refresh_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0xc) as *const u16
  }
#[doc="Get the *mut pointer for the REFRESH register."]
  #[inline] pub fn refresh_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0xc) as *mut u16
  }
#[doc="Read the REFRESH register."]
  #[inline] pub fn refresh(&self) -> Refresh { 
     unsafe {
        Refresh(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u16))
     }
  }
#[doc="Write the REFRESH register."]
  #[inline] pub fn set_refresh(&self, value: Refresh) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the REFRESH register."]
  #[inline] pub fn with_refresh<F: FnOnce(Refresh) -> Refresh>(&self, f: F) -> &Self {
     let tmp = self.refresh();
     self.set_refresh(f(tmp))
  }

#[doc="Get the *const pointer for the UNLOCK register."]
  #[inline] pub fn unlock_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0xe) as *const u16
  }
#[doc="Get the *mut pointer for the UNLOCK register."]
  #[inline] pub fn unlock_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0xe) as *mut u16
  }
#[doc="Read the UNLOCK register."]
  #[inline] pub fn unlock(&self) -> Unlock { 
     unsafe {
        Unlock(::core::ptr::read_volatile(((self.0 as usize) + 0xe) as *const u16))
     }
  }
#[doc="Write the UNLOCK register."]
  #[inline] pub fn set_unlock(&self, value: Unlock) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xe) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the UNLOCK register."]
  #[inline] pub fn with_unlock<F: FnOnce(Unlock) -> Unlock>(&self, f: F) -> &Self {
     let tmp = self.unlock();
     self.set_unlock(f(tmp))
  }

#[doc="Get the *const pointer for the TMROUTH register."]
  #[inline] pub fn tmrouth_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x10) as *const u16
  }
#[doc="Get the *mut pointer for the TMROUTH register."]
  #[inline] pub fn tmrouth_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x10) as *mut u16
  }
#[doc="Read the TMROUTH register."]
  #[inline] pub fn tmrouth(&self) -> Tmrouth { 
     unsafe {
        Tmrouth(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u16))
     }
  }
#[doc="Write the TMROUTH register."]
  #[inline] pub fn set_tmrouth(&self, value: Tmrouth) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the TMROUTH register."]
  #[inline] pub fn with_tmrouth<F: FnOnce(Tmrouth) -> Tmrouth>(&self, f: F) -> &Self {
     let tmp = self.tmrouth();
     self.set_tmrouth(f(tmp))
  }

#[doc="Get the *const pointer for the TMROUTL register."]
  #[inline] pub fn tmroutl_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x12) as *const u16
  }
#[doc="Get the *mut pointer for the TMROUTL register."]
  #[inline] pub fn tmroutl_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x12) as *mut u16
  }
#[doc="Read the TMROUTL register."]
  #[inline] pub fn tmroutl(&self) -> Tmroutl { 
     unsafe {
        Tmroutl(::core::ptr::read_volatile(((self.0 as usize) + 0x12) as *const u16))
     }
  }
#[doc="Write the TMROUTL register."]
  #[inline] pub fn set_tmroutl(&self, value: Tmroutl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x12) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the TMROUTL register."]
  #[inline] pub fn with_tmroutl<F: FnOnce(Tmroutl) -> Tmroutl>(&self, f: F) -> &Self {
     let tmp = self.tmroutl();
     self.set_tmroutl(f(tmp))
  }

#[doc="Get the *const pointer for the RSTCNT register."]
  #[inline] pub fn rstcnt_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x14) as *const u16
  }
#[doc="Get the *mut pointer for the RSTCNT register."]
  #[inline] pub fn rstcnt_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x14) as *mut u16
  }
#[doc="Read the RSTCNT register."]
  #[inline] pub fn rstcnt(&self) -> Rstcnt { 
     unsafe {
        Rstcnt(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u16))
     }
  }
#[doc="Write the RSTCNT register."]
  #[inline] pub fn set_rstcnt(&self, value: Rstcnt) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the RSTCNT register."]
  #[inline] pub fn with_rstcnt<F: FnOnce(Rstcnt) -> Rstcnt>(&self, f: F) -> &Self {
     let tmp = self.rstcnt();
     self.set_rstcnt(f(tmp))
  }

#[doc="Get the *const pointer for the PRESC register."]
  #[inline] pub fn presc_ptr(&self) -> *const u16 { 
     ((self.0 as usize) + 0x16) as *const u16
  }
#[doc="Get the *mut pointer for the PRESC register."]
  #[inline] pub fn presc_mut(&self) -> *mut u16 { 
     ((self.0 as usize) + 0x16) as *mut u16
  }
#[doc="Read the PRESC register."]
  #[inline] pub fn presc(&self) -> Presc { 
     unsafe {
        Presc(::core::ptr::read_volatile(((self.0 as usize) + 0x16) as *const u16))
     }
  }
#[doc="Write the PRESC register."]
  #[inline] pub fn set_presc(&self, value: Presc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x16) as *mut u16, value.0);
     }
     self
  }
#[doc="Modify the PRESC register."]
  #[inline] pub fn with_presc<F: FnOnce(Presc) -> Presc>(&self, f: F) -> &Self {
     let tmp = self.presc();
     self.set_presc(f(tmp))
  }

}

#[doc="Watchdog Status and Control Register High"]
#[derive(PartialEq, Eq)]
pub struct Stctrlh(pub u16);
impl Stctrlh {
#[doc="Enables or disables the WDOG\'s operation"]
  #[inline] pub fn wdogen(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0x1 // [0]
  }
#[doc="Enables or disables the WDOG\'s operation"]
  #[inline] pub fn set_wdogen(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Selects clock source for the WDOG timer and other internal timing operations."]
  #[inline] pub fn clksrc(&self) -> u16 {
     ((self.0 as u16) >> 1) & 0x1 // [1]
  }
#[doc="Selects clock source for the WDOG timer and other internal timing operations."]
  #[inline] pub fn set_clksrc(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Used to enable the debug breadcrumbs feature"]
  #[inline] pub fn irqrsten(&self) -> u16 {
     ((self.0 as u16) >> 2) & 0x1 // [2]
  }
#[doc="Used to enable the debug breadcrumbs feature"]
  #[inline] pub fn set_irqrsten(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Enables Windowing mode."]
  #[inline] pub fn winen(&self) -> u16 {
     ((self.0 as u16) >> 3) & 0x1 // [3]
  }
#[doc="Enables Windowing mode."]
  #[inline] pub fn set_winen(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Enables updates to watchdog write-once registers, after the reset-triggered initial configuration window (WCT) closes, through unlock sequence"]
  #[inline] pub fn allowupdate(&self) -> u16 {
     ((self.0 as u16) >> 4) & 0x1 // [4]
  }
#[doc="Enables updates to watchdog write-once registers, after the reset-triggered initial configuration window (WCT) closes, through unlock sequence"]
  #[inline] pub fn set_allowupdate(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Enables or disables WDOG in Debug mode."]
  #[inline] pub fn dbgen(&self) -> u16 {
     ((self.0 as u16) >> 5) & 0x1 // [5]
  }
#[doc="Enables or disables WDOG in Debug mode."]
  #[inline] pub fn set_dbgen(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Enables or disables WDOG in Stop mode."]
  #[inline] pub fn stopen(&self) -> u16 {
     ((self.0 as u16) >> 6) & 0x1 // [6]
  }
#[doc="Enables or disables WDOG in Stop mode."]
  #[inline] pub fn set_stopen(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Enables or disables WDOG in Wait mode."]
  #[inline] pub fn waiten(&self) -> u16 {
     ((self.0 as u16) >> 7) & 0x1 // [7]
  }
#[doc="Enables or disables WDOG in Wait mode."]
  #[inline] pub fn set_waiten(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Puts the watchdog in the functional test mode"]
  #[inline] pub fn testwdog(&self) -> u16 {
     ((self.0 as u16) >> 10) & 0x1 // [10]
  }
#[doc="Puts the watchdog in the functional test mode"]
  #[inline] pub fn set_testwdog(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Effective only if TESTWDOG is set. Selects the test to be run on the watchdog timer."]
  #[inline] pub fn testsel(&self) -> u16 {
     ((self.0 as u16) >> 11) & 0x1 // [11]
  }
#[doc="Effective only if TESTWDOG is set. Selects the test to be run on the watchdog timer."]
  #[inline] pub fn set_testsel(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="This 2-bit field selects the byte to be tested when the watchdog is in the byte test mode."]
  #[inline] pub fn bytesel(&self) -> u16 {
     ((self.0 as u16) >> 12) & 0x3 // [13:12]
  }
#[doc="This 2-bit field selects the byte to be tested when the watchdog is in the byte test mode."]
  #[inline] pub fn set_bytesel(mut self, value: u16) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Allows the WDOG\'s functional test mode to be disabled permanently"]
  #[inline] pub fn distestwdog(&self) -> u16 {
     ((self.0 as u16) >> 14) & 0x1 // [14]
  }
#[doc="Allows the WDOG\'s functional test mode to be disabled permanently"]
  #[inline] pub fn set_distestwdog(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

}
impl ::core::fmt::Display for Stctrlh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Stctrlh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wdogen() != 0 { try!(write!(f, " wdogen"))}
      if self.clksrc() != 0 { try!(write!(f, " clksrc"))}
      if self.irqrsten() != 0 { try!(write!(f, " irqrsten"))}
      if self.winen() != 0 { try!(write!(f, " winen"))}
      if self.allowupdate() != 0 { try!(write!(f, " allowupdate"))}
      if self.dbgen() != 0 { try!(write!(f, " dbgen"))}
      if self.stopen() != 0 { try!(write!(f, " stopen"))}
      if self.waiten() != 0 { try!(write!(f, " waiten"))}
      if self.testwdog() != 0 { try!(write!(f, " testwdog"))}
      if self.testsel() != 0 { try!(write!(f, " testsel"))}
      if self.bytesel() != 0 { try!(write!(f, " bytesel=0x{:x}", self.bytesel()))}
      if self.distestwdog() != 0 { try!(write!(f, " distestwdog"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Status and Control Register Low"]
#[derive(PartialEq, Eq)]
pub struct Stctrll(pub u16);
impl Stctrll {
#[doc="Interrupt flag"]
  #[inline] pub fn intflg(&self) -> u16 {
     ((self.0 as u16) >> 15) & 0x1 // [15]
  }
#[doc="Interrupt flag"]
  #[inline] pub fn set_intflg(mut self, value: u16) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

}
impl ::core::fmt::Display for Stctrll {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Stctrll {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.intflg() != 0 { try!(write!(f, " intflg"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Time-out Value Register High"]
#[derive(PartialEq, Eq)]
pub struct Tovalh(pub u16);
impl Tovalh {
#[doc="Defines the upper 16 bits of the 32-bit time-out value for the watchdog timer"]
  #[inline] pub fn tovalhigh(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
#[doc="Defines the upper 16 bits of the 32-bit time-out value for the watchdog timer"]
  #[inline] pub fn set_tovalhigh(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tovalh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tovalh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tovalhigh() != 0 { try!(write!(f, " tovalhigh=0x{:x}", self.tovalhigh()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Time-out Value Register Low"]
#[derive(PartialEq, Eq)]
pub struct Tovall(pub u16);
impl Tovall {
#[doc="Defines the lower 16 bits of the 32-bit time-out value for the watchdog timer"]
  #[inline] pub fn tovallow(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
#[doc="Defines the lower 16 bits of the 32-bit time-out value for the watchdog timer"]
  #[inline] pub fn set_tovallow(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tovall {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tovall {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tovallow() != 0 { try!(write!(f, " tovallow=0x{:x}", self.tovallow()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Window Register High"]
#[derive(PartialEq, Eq)]
pub struct Winh(pub u16);
impl Winh {
#[doc="Defines the upper 16 bits of the 32-bit window for the windowed mode of operation of the watchdog"]
  #[inline] pub fn winhigh(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
#[doc="Defines the upper 16 bits of the 32-bit window for the windowed mode of operation of the watchdog"]
  #[inline] pub fn set_winhigh(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Winh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Winh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.winhigh() != 0 { try!(write!(f, " winhigh=0x{:x}", self.winhigh()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Window Register Low"]
#[derive(PartialEq, Eq)]
pub struct Winl(pub u16);
impl Winl {
#[doc="Defines the lower 16 bits of the 32-bit window for the windowed mode of operation of the watchdog"]
  #[inline] pub fn winlow(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
#[doc="Defines the lower 16 bits of the 32-bit window for the windowed mode of operation of the watchdog"]
  #[inline] pub fn set_winlow(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Winl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Winl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.winlow() != 0 { try!(write!(f, " winlow=0x{:x}", self.winlow()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Refresh register"]
#[derive(PartialEq, Eq)]
pub struct Refresh(pub u16);
impl Refresh {
#[doc="Watchdog refresh register"]
  #[inline] pub fn wdogrefresh(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
#[doc="Watchdog refresh register"]
  #[inline] pub fn set_wdogrefresh(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Refresh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Refresh {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wdogrefresh() != 0 { try!(write!(f, " wdogrefresh=0x{:x}", self.wdogrefresh()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Unlock register"]
#[derive(PartialEq, Eq)]
pub struct Unlock(pub u16);
impl Unlock {
#[doc="Writing the unlock sequence values to this register to makes the watchdog write-once registers writable again"]
  #[inline] pub fn wdogunlock(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
#[doc="Writing the unlock sequence values to this register to makes the watchdog write-once registers writable again"]
  #[inline] pub fn set_wdogunlock(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Unlock {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Unlock {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wdogunlock() != 0 { try!(write!(f, " wdogunlock=0x{:x}", self.wdogunlock()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Timer Output Register High"]
#[derive(PartialEq, Eq)]
pub struct Tmrouth(pub u16);
impl Tmrouth {
#[doc="Shows the value of the upper 16 bits of the watchdog timer."]
  #[inline] pub fn timerouthigh(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
#[doc="Shows the value of the upper 16 bits of the watchdog timer."]
  #[inline] pub fn set_timerouthigh(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tmrouth {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tmrouth {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.timerouthigh() != 0 { try!(write!(f, " timerouthigh=0x{:x}", self.timerouthigh()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Timer Output Register Low"]
#[derive(PartialEq, Eq)]
pub struct Tmroutl(pub u16);
impl Tmroutl {
#[doc="Shows the value of the lower 16 bits of the watchdog timer."]
  #[inline] pub fn timeroutlow(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
#[doc="Shows the value of the lower 16 bits of the watchdog timer."]
  #[inline] pub fn set_timeroutlow(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tmroutl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tmroutl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.timeroutlow() != 0 { try!(write!(f, " timeroutlow=0x{:x}", self.timeroutlow()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Reset Count register"]
#[derive(PartialEq, Eq)]
pub struct Rstcnt(pub u16);
impl Rstcnt {
#[doc="Counts the number of times the watchdog resets the system"]
  #[inline] pub fn rstcnt(&self) -> u16 {
     ((self.0 as u16) >> 0) & 0xffff // [15:0]
  }
#[doc="Counts the number of times the watchdog resets the system"]
  #[inline] pub fn set_rstcnt(mut self, value: u16) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Rstcnt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rstcnt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rstcnt() != 0 { try!(write!(f, " rstcnt=0x{:x}", self.rstcnt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Watchdog Prescaler register"]
#[derive(PartialEq, Eq)]
pub struct Presc(pub u16);
impl Presc {
#[doc="3-bit prescaler for the watchdog clock source"]
  #[inline] pub fn prescval(&self) -> u16 {
     ((self.0 as u16) >> 8) & 0x7 // [10:8]
  }
#[doc="3-bit prescaler for the watchdog clock source"]
  #[inline] pub fn set_prescval(mut self, value: u16) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

}
impl ::core::fmt::Display for Presc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Presc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.prescval() != 0 { try!(write!(f, " prescval=0x{:x}", self.prescval()))}
      try!(write!(f, "]"));
      Ok(())
   }
}