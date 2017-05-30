
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TimGenImpl(pub u32);
impl TimGenImpl {
  pub fn cr1(&self) -> Cr1 { 
     unsafe {
       Cr1(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
  pub fn set_cr1(&self, value: Cr1) -> &TimGenImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
  pub fn with_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &TimGenImpl {
     let tmp = self.cr1();
     self.set_cr1(f(tmp))
  }

  pub fn cr2(&self) -> Cr2 { 
     unsafe {
       Cr2(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
  pub fn set_cr2(&self, value: Cr2) -> &TimGenImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
  pub fn with_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &TimGenImpl {
     let tmp = self.cr2();
     self.set_cr2(f(tmp))
  }

  pub fn smcr(&self) -> Smcr { 
     unsafe {
       Smcr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
  pub fn set_smcr(&self, value: Smcr) -> &TimGenImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
  pub fn with_smcr<F: FnOnce(Smcr) -> Smcr>(&self, f: F) -> &TimGenImpl {
     let tmp = self.smcr();
     self.set_smcr(f(tmp))
  }

  pub fn dier(&self) -> Dier { 
     unsafe {
       Dier(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
  pub fn set_dier(&self, value: Dier) -> &TimGenImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
  pub fn with_dier<F: FnOnce(Dier) -> Dier>(&self, f: F) -> &TimGenImpl {
     let tmp = self.dier();
     self.set_dier(f(tmp))
  }

  pub fn sr(&self) -> Sr { 
     unsafe {
       Sr(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
  pub fn set_sr(&self, value: Sr) -> &TimGenImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
  pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &TimGenImpl {
     let tmp = self.sr();
     self.set_sr(f(tmp))
  }

  pub fn set_egr(&self, value: Egr) -> &TimGenImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }

  pub fn ccmr_output(&self, index: usize) -> CcmrOutput { 
     assert!(index < 2);
     unsafe {
        CcmrOutput(::core::ptr::read_volatile(((self.0 as usize) + 0x18 + (index << 2)) as *const u32))
     }
  }
  pub fn set_ccmr_output(&self, index: usize, value: CcmrOutput) -> &TimGenImpl {
     assert!(index < 2);
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x18 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
  pub fn with_ccmr_output<F: FnOnce(CcmrOutput) -> CcmrOutput>(&self, index: usize, f: F) -> &TimGenImpl {
     let tmp = self.ccmr_output(index);
     self.set_ccmr_output(index, f(tmp))
  }

  pub fn ccmr_input(&self, index: usize) -> CcmrInput { 
     assert!(index < 2);
     unsafe {
        CcmrInput(::core::ptr::read_volatile(((self.0 as usize) + 0x18 + (index << 2)) as *const u32))
     }
  }
  pub fn set_ccmr_input(&self, index: usize, value: CcmrInput) -> &TimGenImpl {
     assert!(index < 2);
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x18 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
  pub fn with_ccmr_input<F: FnOnce(CcmrInput) -> CcmrInput>(&self, index: usize, f: F) -> &TimGenImpl {
     let tmp = self.ccmr_input(index);
     self.set_ccmr_input(index, f(tmp))
  }

  pub fn ccer(&self) -> Ccer { 
     unsafe {
       Ccer(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
  pub fn set_ccer(&self, value: Ccer) -> &TimGenImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
  pub fn with_ccer<F: FnOnce(Ccer) -> Ccer>(&self, f: F) -> &TimGenImpl {
     let tmp = self.ccer();
     self.set_ccer(f(tmp))
  }

  pub fn cnt(&self) -> Cnt { 
     unsafe {
       Cnt(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }
  pub fn set_cnt(&self, value: Cnt) -> &TimGenImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }
  pub fn with_cnt<F: FnOnce(Cnt) -> Cnt>(&self, f: F) -> &TimGenImpl {
     let tmp = self.cnt();
     self.set_cnt(f(tmp))
  }

  pub fn psc(&self) -> Psc { 
     unsafe {
       Psc(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
     }
  }
  pub fn set_psc(&self, value: Psc) -> &TimGenImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
     }
     self
  }
  pub fn with_psc<F: FnOnce(Psc) -> Psc>(&self, f: F) -> &TimGenImpl {
     let tmp = self.psc();
     self.set_psc(f(tmp))
  }

  pub fn arr(&self) -> Arr { 
     unsafe {
       Arr(::core::ptr::read_volatile(((self.0 as usize) + 0x2c) as *const u32))
     }
  }
  pub fn set_arr(&self, value: Arr) -> &TimGenImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x2c) as *mut u32, value.0);
     }
     self
  }
  pub fn with_arr<F: FnOnce(Arr) -> Arr>(&self, f: F) -> &TimGenImpl {
     let tmp = self.arr();
     self.set_arr(f(tmp))
  }

  pub fn rcr(&self) -> Rcr { 
     unsafe {
       Rcr(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u32))
     }
  }
  pub fn set_rcr(&self, value: Rcr) -> &TimGenImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u32, value.0);
     }
     self
  }
  pub fn with_rcr<F: FnOnce(Rcr) -> Rcr>(&self, f: F) -> &TimGenImpl {
     let tmp = self.rcr();
     self.set_rcr(f(tmp))
  }

  pub fn bdtr(&self) -> Bdtr { 
     unsafe {
       Bdtr(::core::ptr::read_volatile(((self.0 as usize) + 0x44) as *const u32))
     }
  }
  pub fn set_bdtr(&self, value: Bdtr) -> &TimGenImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u32, value.0);
     }
     self
  }
  pub fn with_bdtr<F: FnOnce(Bdtr) -> Bdtr>(&self, f: F) -> &TimGenImpl {
     let tmp = self.bdtr();
     self.set_bdtr(f(tmp))
  }

  pub fn ccr(&self, index: usize) -> Ccr { 
     assert!(index < 4);
     unsafe {
        Ccr(::core::ptr::read_volatile(((self.0 as usize) + 0x34 + (index << 2)) as *const u32))
     }
  }
  pub fn set_ccr(&self, index: usize, value: Ccr) -> &TimGenImpl {
     assert!(index < 4);
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x34 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
  pub fn with_ccr<F: FnOnce(Ccr) -> Ccr>(&self, index: usize, f: F) -> &TimGenImpl {
     let tmp = self.ccr(index);
     self.set_ccr(index, f(tmp))
  }

  pub fn dcr(&self) -> Dcr { 
     unsafe {
       Dcr(::core::ptr::read_volatile(((self.0 as usize) + 0x48) as *const u32))
     }
  }
  pub fn set_dcr(&self, value: Dcr) -> &TimGenImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x48) as *mut u32, value.0);
     }
     self
  }
  pub fn with_dcr<F: FnOnce(Dcr) -> Dcr>(&self, f: F) -> &TimGenImpl {
     let tmp = self.dcr();
     self.set_dcr(f(tmp))
  }

  pub fn dmar(&self) -> Dmar { 
     unsafe {
       Dmar(::core::ptr::read_volatile(((self.0 as usize) + 0x4c) as *const u32))
     }
  }
  pub fn set_dmar(&self, value: Dmar) -> &TimGenImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x4c) as *mut u32, value.0);
     }
     self
  }
  pub fn with_dmar<F: FnOnce(Dmar) -> Dmar>(&self, f: F) -> &TimGenImpl {
     let tmp = self.dmar();
     self.set_dmar(f(tmp))
  }

  pub fn or(&self) -> Or { 
     unsafe {
       Or(::core::ptr::read_volatile(((self.0 as usize) + 0x50) as *const u32))
     }
  }
  pub fn set_or(&self, value: Or) -> &TimGenImpl {
     unsafe {
       ::core::ptr::write_volatile(((self.0 as usize) + 0x50) as *mut u32, value.0);
     }
     self
  }
  pub fn with_or<F: FnOnce(Or) -> Or>(&self, f: F) -> &TimGenImpl {
     let tmp = self.or();
     self.set_or(f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
  pub fn cen(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_cen(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn udis(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_udis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn urs(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_urs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn opm(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_opm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn dir(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_dir(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn cms(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x3 // [6:5]
  }
  pub fn set_cms(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn arpe(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_arpe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn ckd(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  pub fn set_ckd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn uifremap(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_uifremap(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
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
      if self.cen() != 0 { try!(write!(f, " cen"))}
      if self.udis() != 0 { try!(write!(f, " udis"))}
      if self.urs() != 0 { try!(write!(f, " urs"))}
      if self.opm() != 0 { try!(write!(f, " opm"))}
      if self.dir() != 0 { try!(write!(f, " dir"))}
      if self.cms() != 0 { try!(write!(f, " cms=0x{:x}", self.cms()))}
      if self.arpe() != 0 { try!(write!(f, " arpe"))}
      if self.ckd() != 0 { try!(write!(f, " ckd=0x{:x}", self.ckd()))}
      if self.uifremap() != 0 { try!(write!(f, " uifremap"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cr2(pub u32);
impl Cr2 {
  pub fn ti1s(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_ti1s(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn mms(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x7 // [6:4]
  }
  pub fn set_mms(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn ccds(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_ccds(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}
impl ::core::fmt::Display for Cr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ti1s() != 0 { try!(write!(f, " ti1s"))}
      if self.mms() != 0 { try!(write!(f, " mms=0x{:x}", self.mms()))}
      if self.ccds() != 0 { try!(write!(f, " ccds"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Smcr(pub u32);
impl Smcr {
  pub fn sms(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
  pub fn set_sms(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn occs(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_occs(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn ts(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x7 // [6:4]
  }
  pub fn set_ts(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn msm(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_msm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn etf(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
  pub fn set_etf(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

  pub fn etps(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x3 // [13:12]
  }
  pub fn set_etps(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn ece(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_ece(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn etp(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_etp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn sms_3(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
  pub fn set_sms_3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Smcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Smcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sms() != 0 { try!(write!(f, " sms=0x{:x}", self.sms()))}
      if self.occs() != 0 { try!(write!(f, " occs"))}
      if self.ts() != 0 { try!(write!(f, " ts=0x{:x}", self.ts()))}
      if self.msm() != 0 { try!(write!(f, " msm"))}
      if self.etf() != 0 { try!(write!(f, " etf=0x{:x}", self.etf()))}
      if self.etps() != 0 { try!(write!(f, " etps=0x{:x}", self.etps()))}
      if self.ece() != 0 { try!(write!(f, " ece"))}
      if self.etp() != 0 { try!(write!(f, " etp"))}
      if self.sms_3() != 0 { try!(write!(f, " sms_3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dier(pub u32);
impl Dier {
  pub fn tde(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_tde(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn ccde(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 9 + index;
     ((self.0 as u32) >> shift) & 0x1 // [9]
  }
  pub fn set_ccde(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 9 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  pub fn ude(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
  pub fn set_ude(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn tie(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_tie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ccie(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 1 + index;
     ((self.0 as u32) >> shift) & 0x1 // [1]
  }
  pub fn set_ccie(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 1 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  pub fn uie(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_uie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dier {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dier {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tde() != 0 { try!(write!(f, " tde"))}
      if self.ccde(0) != 0 { try!(write!(f, " ccde[0]"))}
      if self.ccde(1) != 0 { try!(write!(f, " ccde[1]"))}
      if self.ccde(2) != 0 { try!(write!(f, " ccde[2]"))}
      if self.ccde(3) != 0 { try!(write!(f, " ccde[3]"))}
      if self.ude() != 0 { try!(write!(f, " ude"))}
      if self.tie() != 0 { try!(write!(f, " tie"))}
      if self.ccie(0) != 0 { try!(write!(f, " ccie[0]"))}
      if self.ccie(1) != 0 { try!(write!(f, " ccie[1]"))}
      if self.ccie(2) != 0 { try!(write!(f, " ccie[2]"))}
      if self.ccie(3) != 0 { try!(write!(f, " ccie[3]"))}
      if self.uie() != 0 { try!(write!(f, " uie"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
  pub fn ccof(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 9 + index;
     ((self.0 as u32) >> shift) & 0x1 // [9]
  }
  pub fn set_ccof(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 9 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  pub fn tif(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_tif(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ccif(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 1 + index;
     ((self.0 as u32) >> shift) & 0x1 // [1]
  }
  pub fn set_ccif(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 1 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  pub fn uif(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_uif(mut self, value: u32) -> Self {
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
      if self.ccof(0) != 0 { try!(write!(f, " ccof[0]"))}
      if self.ccof(1) != 0 { try!(write!(f, " ccof[1]"))}
      if self.ccof(2) != 0 { try!(write!(f, " ccof[2]"))}
      if self.ccof(3) != 0 { try!(write!(f, " ccof[3]"))}
      if self.tif() != 0 { try!(write!(f, " tif"))}
      if self.ccif(0) != 0 { try!(write!(f, " ccif[0]"))}
      if self.ccif(1) != 0 { try!(write!(f, " ccif[1]"))}
      if self.ccif(2) != 0 { try!(write!(f, " ccif[2]"))}
      if self.ccif(3) != 0 { try!(write!(f, " ccif[3]"))}
      if self.uif() != 0 { try!(write!(f, " uif"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Egr(pub u32);
impl Egr {
  pub fn tg(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_tg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn ccg(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 1 + index;
     ((self.0 as u32) >> shift) & 0x1 // [1]
  }
  pub fn set_ccg(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 1 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  pub fn ug(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_ug(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Egr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Egr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tg() != 0 { try!(write!(f, " tg"))}
      if self.ccg(0) != 0 { try!(write!(f, " ccg[0]"))}
      if self.ccg(1) != 0 { try!(write!(f, " ccg[1]"))}
      if self.ccg(2) != 0 { try!(write!(f, " ccg[2]"))}
      if self.ccg(3) != 0 { try!(write!(f, " ccg[3]"))}
      if self.ug() != 0 { try!(write!(f, " ug"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct CcmrOutput(pub u32);
impl CcmrOutput {
  pub fn ccs(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 0 + (index << 3);
     ((self.0 as u32) >> shift) & 0x3 // [1:0]
  }
  pub fn set_ccs(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x3) == 0);
     let shift: usize = 0 + (index << 3);
     self.0 &= !(0x3 << shift);
     self.0 |= value << shift;
     self
  }

  pub fn ocfe(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 2 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [2]
  }
  pub fn set_ocfe(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 2 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  pub fn ocpe(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 3 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [3]
  }
  pub fn set_ocpe(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 3 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  pub fn ocm(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 4 + (index << 3);
     ((self.0 as u32) >> shift) & 0x7 // [6:4]
  }
  pub fn set_ocm(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x7) == 0);
     let shift: usize = 4 + (index << 3);
     self.0 &= !(0x7 << shift);
     self.0 |= value << shift;
     self
  }

  pub fn occe(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 7 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [7]
  }
  pub fn set_occe(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 7 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  pub fn ocm_3(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 16 + (index << 3);
     ((self.0 as u32) >> shift) & 0x1 // [16]
  }
  pub fn set_ocm_3(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x1) == 0);
     let shift: usize = 16 + (index << 3);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for CcmrOutput {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for CcmrOutput {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ccs(0) != 0 { try!(write!(f, " ccs[0]=0x{:x}", self.ccs(0)))}
      if self.ccs(1) != 0 { try!(write!(f, " ccs[1]=0x{:x}", self.ccs(1)))}
      if self.ocfe(0) != 0 { try!(write!(f, " ocfe[0]"))}
      if self.ocfe(1) != 0 { try!(write!(f, " ocfe[1]"))}
      if self.ocpe(0) != 0 { try!(write!(f, " ocpe[0]"))}
      if self.ocpe(1) != 0 { try!(write!(f, " ocpe[1]"))}
      if self.ocm(0) != 0 { try!(write!(f, " ocm[0]=0x{:x}", self.ocm(0)))}
      if self.ocm(1) != 0 { try!(write!(f, " ocm[1]=0x{:x}", self.ocm(1)))}
      if self.occe(0) != 0 { try!(write!(f, " occe[0]"))}
      if self.occe(1) != 0 { try!(write!(f, " occe[1]"))}
      if self.ocm_3(0) != 0 { try!(write!(f, " ocm_3[0]"))}
      if self.ocm_3(1) != 0 { try!(write!(f, " ocm_3[1]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct CcmrInput(pub u32);
impl CcmrInput {
  pub fn icf(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 4 + (index << 3);
     ((self.0 as u32) >> shift) & 0xf // [7:4]
  }
  pub fn set_icf(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0xf) == 0);
     let shift: usize = 4 + (index << 3);
     self.0 &= !(0xf << shift);
     self.0 |= value << shift;
     self
  }

  pub fn icpsc(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 2 + (index << 3);
     ((self.0 as u32) >> shift) & 0x3 // [3:2]
  }
  pub fn set_icpsc(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x3) == 0);
     let shift: usize = 2 + (index << 3);
     self.0 &= !(0x3 << shift);
     self.0 |= value << shift;
     self
  }

  pub fn ccs(&self, index: usize) -> u32 {
     assert!(index < 2);
     let shift: usize = 0 + (index << 3);
     ((self.0 as u32) >> shift) & 0x3 // [1:0]
  }
  pub fn set_ccs(mut self, index: usize, value: u32) -> Self {
     assert!(index < 2);
     assert!((value & !0x3) == 0);
     let shift: usize = 0 + (index << 3);
     self.0 &= !(0x3 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for CcmrInput {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for CcmrInput {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.icf(0) != 0 { try!(write!(f, " icf[0]=0x{:x}", self.icf(0)))}
      if self.icf(1) != 0 { try!(write!(f, " icf[1]=0x{:x}", self.icf(1)))}
      if self.icpsc(0) != 0 { try!(write!(f, " icpsc[0]=0x{:x}", self.icpsc(0)))}
      if self.icpsc(1) != 0 { try!(write!(f, " icpsc[1]=0x{:x}", self.icpsc(1)))}
      if self.ccs(0) != 0 { try!(write!(f, " ccs[0]=0x{:x}", self.ccs(0)))}
      if self.ccs(1) != 0 { try!(write!(f, " ccs[1]=0x{:x}", self.ccs(1)))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ccer(pub u32);
impl Ccer {
  pub fn cce(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 0 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_cce(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  pub fn ccp(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 1 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [1]
  }
  pub fn set_ccp(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 1 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  pub fn ccnp(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 3 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [3]
  }
  pub fn set_ccnp(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 3 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Ccer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ccer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cce(0) != 0 { try!(write!(f, " cce[0]"))}
      if self.cce(1) != 0 { try!(write!(f, " cce[1]"))}
      if self.cce(2) != 0 { try!(write!(f, " cce[2]"))}
      if self.cce(3) != 0 { try!(write!(f, " cce[3]"))}
      if self.ccp(0) != 0 { try!(write!(f, " ccp[0]"))}
      if self.ccp(1) != 0 { try!(write!(f, " ccp[1]"))}
      if self.ccp(2) != 0 { try!(write!(f, " ccp[2]"))}
      if self.ccp(3) != 0 { try!(write!(f, " ccp[3]"))}
      if self.ccnp(0) != 0 { try!(write!(f, " ccnp[0]"))}
      if self.ccnp(1) != 0 { try!(write!(f, " ccnp[1]"))}
      if self.ccnp(2) != 0 { try!(write!(f, " ccnp[2]"))}
      if self.ccnp(3) != 0 { try!(write!(f, " ccnp[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Cnt(pub u32);
impl Cnt {
  pub fn cntl(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_cntl(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn cnth(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x7fff // [30:16]
  }
  pub fn set_cnth(mut self, value: u32) -> Self {
     assert!((value & !0x7fff) == 0);
     self.0 &= !(0x7fff << 16);
     self.0 |= value << 16;
     self
  }

  pub fn cnt_or_uifcpy(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
  pub fn set_cnt_or_uifcpy(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

}
impl ::core::fmt::Display for Cnt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cnt {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cntl() != 0 { try!(write!(f, " cntl=0x{:x}", self.cntl()))}
      if self.cnth() != 0 { try!(write!(f, " cnth=0x{:x}", self.cnth()))}
      if self.cnt_or_uifcpy() != 0 { try!(write!(f, " cnt_or_uifcpy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Psc(pub u32);
impl Psc {
  pub fn psc(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_psc(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Psc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Psc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.psc() != 0 { try!(write!(f, " psc=0x{:x}", self.psc()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Arr(pub u32);
impl Arr {
  pub fn arrl(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_arrl(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn arrh(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xffff // [31:16]
  }
  pub fn set_arrh(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Arr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Arr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.arrl() != 0 { try!(write!(f, " arrl=0x{:x}", self.arrl()))}
      if self.arrh() != 0 { try!(write!(f, " arrh=0x{:x}", self.arrh()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Rcr(pub u32);
impl Rcr {
  pub fn rep(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_rep(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Rcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Rcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.rep() != 0 { try!(write!(f, " rep=0x{:x}", self.rep()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Bdtr(pub u32);
impl Bdtr {
  pub fn moe(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
  pub fn set_moe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

  pub fn aoe(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_aoe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

  pub fn bkp(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
  pub fn set_bkp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

  pub fn bke(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
  pub fn set_bke(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn ossr(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
  pub fn set_ossr(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

  pub fn ossi(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
  pub fn set_ossi(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn lock(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  pub fn set_lock(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn dtg(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
  pub fn set_dtg(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn bkf(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
  pub fn set_bkf(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Bdtr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Bdtr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.moe() != 0 { try!(write!(f, " moe"))}
      if self.aoe() != 0 { try!(write!(f, " aoe"))}
      if self.bkp() != 0 { try!(write!(f, " bkp"))}
      if self.bke() != 0 { try!(write!(f, " bke"))}
      if self.ossr() != 0 { try!(write!(f, " ossr"))}
      if self.ossi() != 0 { try!(write!(f, " ossi"))}
      if self.lock() != 0 { try!(write!(f, " lock=0x{:x}", self.lock()))}
      if self.dtg() != 0 { try!(write!(f, " dtg=0x{:x}", self.dtg()))}
      if self.bkf() != 0 { try!(write!(f, " bkf=0x{:x}", self.bkf()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ccr(pub u32);
impl Ccr {
  pub fn ccrl(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_ccrl(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

  pub fn ccrh(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xffff // [31:16]
  }
  pub fn set_ccrh(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Ccr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ccr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ccrl() != 0 { try!(write!(f, " ccrl=0x{:x}", self.ccrl()))}
      if self.ccrh() != 0 { try!(write!(f, " ccrh=0x{:x}", self.ccrh()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dcr(pub u32);
impl Dcr {
  pub fn dbl(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1f // [12:8]
  }
  pub fn set_dbl(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 8);
     self.0 |= value << 8;
     self
  }

  pub fn dba(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1f // [4:0]
  }
  pub fn set_dba(mut self, value: u32) -> Self {
     assert!((value & !0x1f) == 0);
     self.0 &= !(0x1f << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbl() != 0 { try!(write!(f, " dbl=0x{:x}", self.dbl()))}
      if self.dba() != 0 { try!(write!(f, " dba=0x{:x}", self.dba()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Dmar(pub u32);
impl Dmar {
  pub fn dmab(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_dmab(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dmar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dmar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dmab() != 0 { try!(write!(f, " dmab=0x{:x}", self.dmab()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Or(pub u32);
impl Or {
}
impl ::core::fmt::Display for Or {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Or {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
