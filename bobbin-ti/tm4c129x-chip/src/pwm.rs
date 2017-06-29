pub const PWM0: Pwm0 = Periph(0x40028000, Pwm0Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="PWM Peripheral"]
pub struct Periph<T>(pub u32, pub T); 

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Pwm0Id {}
pub type Pwm0 = Periph<Pwm0Id>;

impl super::sig::Signal<super::sig::M0pwm0> for Pwm0Ch0 {}
impl super::sig::SignalPwm<super::sig::M0pwm0> for Pwm0Ch0 {}
impl super::sig::Signal<super::sig::M0pwm1> for Pwm0Ch1 {}
impl super::sig::SignalPwm<super::sig::M0pwm1> for Pwm0Ch1 {}
impl super::sig::Signal<super::sig::M0pwm2> for Pwm0Ch2 {}
impl super::sig::SignalPwm<super::sig::M0pwm2> for Pwm0Ch2 {}
impl super::sig::Signal<super::sig::M0pwm3> for Pwm0Ch3 {}
impl super::sig::SignalPwm<super::sig::M0pwm3> for Pwm0Ch3 {}


impl<T> Periph<T> {
#[doc="Get the *const pointer for the CTL register."]
  #[inline] pub fn ctl_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the CTL register."]
  #[inline] pub fn ctl_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the CTL register."]
  #[inline] pub fn ctl(&self) -> Ctl { 
     unsafe {
        Ctl(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the CTL register."]
  #[inline] pub fn set_ctl(&self, value: Ctl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CTL register."]
  #[inline] pub fn with_ctl<F: FnOnce(Ctl) -> Ctl>(&self, f: F) -> &Self {
     let tmp = self.ctl();
     self.set_ctl(f(tmp))
  }

#[doc="Get the *const pointer for the SYNC register."]
  #[inline] pub fn sync_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the SYNC register."]
  #[inline] pub fn sync_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the SYNC register."]
  #[inline] pub fn sync(&self) -> Sync { 
     unsafe {
        Sync(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the SYNC register."]
  #[inline] pub fn set_sync(&self, value: Sync) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the SYNC register."]
  #[inline] pub fn with_sync<F: FnOnce(Sync) -> Sync>(&self, f: F) -> &Self {
     let tmp = self.sync();
     self.set_sync(f(tmp))
  }

#[doc="Get the *const pointer for the ENABLE register."]
  #[inline] pub fn enable_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the ENABLE register."]
  #[inline] pub fn enable_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the ENABLE register."]
  #[inline] pub fn enable(&self) -> Enable { 
     unsafe {
        Enable(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the ENABLE register."]
  #[inline] pub fn set_enable(&self, value: Enable) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ENABLE register."]
  #[inline] pub fn with_enable<F: FnOnce(Enable) -> Enable>(&self, f: F) -> &Self {
     let tmp = self.enable();
     self.set_enable(f(tmp))
  }

#[doc="Get the *const pointer for the INVERT register."]
  #[inline] pub fn invert_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the INVERT register."]
  #[inline] pub fn invert_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the INVERT register."]
  #[inline] pub fn invert(&self) -> Invert { 
     unsafe {
        Invert(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the INVERT register."]
  #[inline] pub fn set_invert(&self, value: Invert) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the INVERT register."]
  #[inline] pub fn with_invert<F: FnOnce(Invert) -> Invert>(&self, f: F) -> &Self {
     let tmp = self.invert();
     self.set_invert(f(tmp))
  }

#[doc="Get the *const pointer for the FAULT register."]
  #[inline] pub fn fault_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the FAULT register."]
  #[inline] pub fn fault_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the FAULT register."]
  #[inline] pub fn fault(&self) -> Fault { 
     unsafe {
        Fault(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the FAULT register."]
  #[inline] pub fn set_fault(&self, value: Fault) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the FAULT register."]
  #[inline] pub fn with_fault<F: FnOnce(Fault) -> Fault>(&self, f: F) -> &Self {
     let tmp = self.fault();
     self.set_fault(f(tmp))
  }

#[doc="Get the *const pointer for the INTEN register."]
  #[inline] pub fn inten_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the INTEN register."]
  #[inline] pub fn inten_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Read the INTEN register."]
  #[inline] pub fn inten(&self) -> Inten { 
     unsafe {
        Inten(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
#[doc="Write the INTEN register."]
  #[inline] pub fn set_inten(&self, value: Inten) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the INTEN register."]
  #[inline] pub fn with_inten<F: FnOnce(Inten) -> Inten>(&self, f: F) -> &Self {
     let tmp = self.inten();
     self.set_inten(f(tmp))
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

#[doc="Get the *const pointer for the ISC register."]
  #[inline] pub fn isc_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the ISC register."]
  #[inline] pub fn isc_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the ISC register."]
  #[inline] pub fn isc(&self) -> Isc { 
     unsafe {
        Isc(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
#[doc="Write the ISC register."]
  #[inline] pub fn set_isc(&self, value: Isc) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ISC register."]
  #[inline] pub fn with_isc<F: FnOnce(Isc) -> Isc>(&self, f: F) -> &Self {
     let tmp = self.isc();
     self.set_isc(f(tmp))
  }

#[doc="Get the *const pointer for the STATUS register."]
  #[inline] pub fn status_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
#[doc="Get the *mut pointer for the STATUS register."]
  #[inline] pub fn status_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
#[doc="Read the STATUS register."]
  #[inline] pub fn status(&self) -> Status { 
     unsafe {
        Status(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
#[doc="Write the STATUS register."]
  #[inline] pub fn set_status(&self, value: Status) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the STATUS register."]
  #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
     let tmp = self.status();
     self.set_status(f(tmp))
  }

#[doc="Get the *const pointer for the FAULTVAL register."]
  #[inline] pub fn faultval_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
#[doc="Get the *mut pointer for the FAULTVAL register."]
  #[inline] pub fn faultval_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
#[doc="Read the FAULTVAL register."]
  #[inline] pub fn faultval(&self) -> Faultval { 
     unsafe {
        Faultval(::core::ptr::read_volatile(((self.0 as usize) + 0x24) as *const u32))
     }
  }
#[doc="Write the FAULTVAL register."]
  #[inline] pub fn set_faultval(&self, value: Faultval) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the FAULTVAL register."]
  #[inline] pub fn with_faultval<F: FnOnce(Faultval) -> Faultval>(&self, f: F) -> &Self {
     let tmp = self.faultval();
     self.set_faultval(f(tmp))
  }

#[doc="Get the *const pointer for the ENUPD register."]
  #[inline] pub fn enupd_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x28) as *const u32
  }
#[doc="Get the *mut pointer for the ENUPD register."]
  #[inline] pub fn enupd_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x28) as *mut u32
  }
#[doc="Read the ENUPD register."]
  #[inline] pub fn enupd(&self) -> Enupd { 
     unsafe {
        Enupd(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
     }
  }
#[doc="Write the ENUPD register."]
  #[inline] pub fn set_enupd(&self, value: Enupd) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ENUPD register."]
  #[inline] pub fn with_enupd<F: FnOnce(Enupd) -> Enupd>(&self, f: F) -> &Self {
     let tmp = self.enupd();
     self.set_enupd(f(tmp))
  }

#[doc="Get the *const pointer for the CH_CTL register."]
  #[inline] pub fn ch_ctl_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x40 + (index * 64)) as *const u32
  }
#[doc="Get the *mut pointer for the CH_CTL register."]
  #[inline] pub fn ch_ctl_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x40 + (index * 64)) as *mut u32
  }
#[doc="Read the CH_CTL register."]
  #[inline] pub fn ch_ctl(&self, index: usize) -> ChCtl { 
     assert!(index < 4);
     unsafe {
        ChCtl(::core::ptr::read_volatile(((self.0 as usize) + 0x40 + (index * 64)) as *const u32))
     }
  }
#[doc="Write the CH_CTL register."]
  #[inline] pub fn set_ch_ctl(&self, index: usize, value: ChCtl) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x40 + (index * 64)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CH_CTL register."]
  #[inline] pub fn with_ch_ctl<F: FnOnce(ChCtl) -> ChCtl>(&self, index: usize, f: F) -> &Self {
     let tmp = self.ch_ctl(index);
     self.set_ch_ctl(index, f(tmp))
  }

#[doc="Get the *const pointer for the CH_INTEN register."]
  #[inline] pub fn ch_inten_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x44 + (index * 64)) as *const u32
  }
#[doc="Get the *mut pointer for the CH_INTEN register."]
  #[inline] pub fn ch_inten_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x44 + (index * 64)) as *mut u32
  }
#[doc="Read the CH_INTEN register."]
  #[inline] pub fn ch_inten(&self, index: usize) -> ChInten { 
     assert!(index < 4);
     unsafe {
        ChInten(::core::ptr::read_volatile(((self.0 as usize) + 0x44 + (index * 64)) as *const u32))
     }
  }
#[doc="Write the CH_INTEN register."]
  #[inline] pub fn set_ch_inten(&self, index: usize, value: ChInten) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x44 + (index * 64)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CH_INTEN register."]
  #[inline] pub fn with_ch_inten<F: FnOnce(ChInten) -> ChInten>(&self, index: usize, f: F) -> &Self {
     let tmp = self.ch_inten(index);
     self.set_ch_inten(index, f(tmp))
  }

#[doc="Get the *const pointer for the CH_RIS register."]
  #[inline] pub fn ch_ris_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x48 + (index * 64)) as *const u32
  }
#[doc="Get the *mut pointer for the CH_RIS register."]
  #[inline] pub fn ch_ris_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x48 + (index * 64)) as *mut u32
  }
#[doc="Read the CH_RIS register."]
  #[inline] pub fn ch_ris(&self, index: usize) -> ChRis { 
     assert!(index < 4);
     unsafe {
        ChRis(::core::ptr::read_volatile(((self.0 as usize) + 0x48 + (index * 64)) as *const u32))
     }
  }
#[doc="Write the CH_RIS register."]
  #[inline] pub fn set_ch_ris(&self, index: usize, value: ChRis) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x48 + (index * 64)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CH_RIS register."]
  #[inline] pub fn with_ch_ris<F: FnOnce(ChRis) -> ChRis>(&self, index: usize, f: F) -> &Self {
     let tmp = self.ch_ris(index);
     self.set_ch_ris(index, f(tmp))
  }

#[doc="Get the *const pointer for the CH_ISC register."]
  #[inline] pub fn ch_isc_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x4c + (index * 64)) as *const u32
  }
#[doc="Get the *mut pointer for the CH_ISC register."]
  #[inline] pub fn ch_isc_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x4c + (index * 64)) as *mut u32
  }
#[doc="Read the CH_ISC register."]
  #[inline] pub fn ch_isc(&self, index: usize) -> ChIsc { 
     assert!(index < 4);
     unsafe {
        ChIsc(::core::ptr::read_volatile(((self.0 as usize) + 0x4c + (index * 64)) as *const u32))
     }
  }
#[doc="Write the CH_ISC register."]
  #[inline] pub fn set_ch_isc(&self, index: usize, value: ChIsc) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4c + (index * 64)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CH_ISC register."]
  #[inline] pub fn with_ch_isc<F: FnOnce(ChIsc) -> ChIsc>(&self, index: usize, f: F) -> &Self {
     let tmp = self.ch_isc(index);
     self.set_ch_isc(index, f(tmp))
  }

#[doc="Get the *const pointer for the CH_LOAD register."]
  #[inline] pub fn ch_load_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x50 + (index * 64)) as *const u32
  }
#[doc="Get the *mut pointer for the CH_LOAD register."]
  #[inline] pub fn ch_load_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x50 + (index * 64)) as *mut u32
  }
#[doc="Read the CH_LOAD register."]
  #[inline] pub fn ch_load(&self, index: usize) -> ChLoad { 
     assert!(index < 4);
     unsafe {
        ChLoad(::core::ptr::read_volatile(((self.0 as usize) + 0x50 + (index * 64)) as *const u32))
     }
  }
#[doc="Write the CH_LOAD register."]
  #[inline] pub fn set_ch_load(&self, index: usize, value: ChLoad) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x50 + (index * 64)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CH_LOAD register."]
  #[inline] pub fn with_ch_load<F: FnOnce(ChLoad) -> ChLoad>(&self, index: usize, f: F) -> &Self {
     let tmp = self.ch_load(index);
     self.set_ch_load(index, f(tmp))
  }

#[doc="Get the *const pointer for the CH_COUNT register."]
  #[inline] pub fn ch_count_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x54 + (index * 64)) as *const u32
  }
#[doc="Get the *mut pointer for the CH_COUNT register."]
  #[inline] pub fn ch_count_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x54 + (index * 64)) as *mut u32
  }
#[doc="Read the CH_COUNT register."]
  #[inline] pub fn ch_count(&self, index: usize) -> ChCount { 
     assert!(index < 4);
     unsafe {
        ChCount(::core::ptr::read_volatile(((self.0 as usize) + 0x54 + (index * 64)) as *const u32))
     }
  }
#[doc="Write the CH_COUNT register."]
  #[inline] pub fn set_ch_count(&self, index: usize, value: ChCount) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x54 + (index * 64)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CH_COUNT register."]
  #[inline] pub fn with_ch_count<F: FnOnce(ChCount) -> ChCount>(&self, index: usize, f: F) -> &Self {
     let tmp = self.ch_count(index);
     self.set_ch_count(index, f(tmp))
  }

#[doc="Get the *const pointer for the CH_CMPA register."]
  #[inline] pub fn ch_cmpa_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x58 + (index * 64)) as *const u32
  }
#[doc="Get the *mut pointer for the CH_CMPA register."]
  #[inline] pub fn ch_cmpa_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x58 + (index * 64)) as *mut u32
  }
#[doc="Read the CH_CMPA register."]
  #[inline] pub fn ch_cmpa(&self, index: usize) -> ChCmpa { 
     assert!(index < 4);
     unsafe {
        ChCmpa(::core::ptr::read_volatile(((self.0 as usize) + 0x58 + (index * 64)) as *const u32))
     }
  }
#[doc="Write the CH_CMPA register."]
  #[inline] pub fn set_ch_cmpa(&self, index: usize, value: ChCmpa) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x58 + (index * 64)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CH_CMPA register."]
  #[inline] pub fn with_ch_cmpa<F: FnOnce(ChCmpa) -> ChCmpa>(&self, index: usize, f: F) -> &Self {
     let tmp = self.ch_cmpa(index);
     self.set_ch_cmpa(index, f(tmp))
  }

#[doc="Get the *const pointer for the CH_CMPB register."]
  #[inline] pub fn ch_cmpb_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x5c + (index * 64)) as *const u32
  }
#[doc="Get the *mut pointer for the CH_CMPB register."]
  #[inline] pub fn ch_cmpb_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x5c + (index * 64)) as *mut u32
  }
#[doc="Read the CH_CMPB register."]
  #[inline] pub fn ch_cmpb(&self, index: usize) -> ChCmpb { 
     assert!(index < 4);
     unsafe {
        ChCmpb(::core::ptr::read_volatile(((self.0 as usize) + 0x5c + (index * 64)) as *const u32))
     }
  }
#[doc="Write the CH_CMPB register."]
  #[inline] pub fn set_ch_cmpb(&self, index: usize, value: ChCmpb) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x5c + (index * 64)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CH_CMPB register."]
  #[inline] pub fn with_ch_cmpb<F: FnOnce(ChCmpb) -> ChCmpb>(&self, index: usize, f: F) -> &Self {
     let tmp = self.ch_cmpb(index);
     self.set_ch_cmpb(index, f(tmp))
  }

#[doc="Get the *const pointer for the CH_GENA register."]
  #[inline] pub fn ch_gena_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x60 + (index * 64)) as *const u32
  }
#[doc="Get the *mut pointer for the CH_GENA register."]
  #[inline] pub fn ch_gena_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x60 + (index * 64)) as *mut u32
  }
#[doc="Read the CH_GENA register."]
  #[inline] pub fn ch_gena(&self, index: usize) -> ChGena { 
     assert!(index < 4);
     unsafe {
        ChGena(::core::ptr::read_volatile(((self.0 as usize) + 0x60 + (index * 64)) as *const u32))
     }
  }
#[doc="Write the CH_GENA register."]
  #[inline] pub fn set_ch_gena(&self, index: usize, value: ChGena) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x60 + (index * 64)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CH_GENA register."]
  #[inline] pub fn with_ch_gena<F: FnOnce(ChGena) -> ChGena>(&self, index: usize, f: F) -> &Self {
     let tmp = self.ch_gena(index);
     self.set_ch_gena(index, f(tmp))
  }

#[doc="Get the *const pointer for the CH_GENB register."]
  #[inline] pub fn ch_genb_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x64 + (index * 64)) as *const u32
  }
#[doc="Get the *mut pointer for the CH_GENB register."]
  #[inline] pub fn ch_genb_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x64 + (index * 64)) as *mut u32
  }
#[doc="Read the CH_GENB register."]
  #[inline] pub fn ch_genb(&self, index: usize) -> ChGenb { 
     assert!(index < 4);
     unsafe {
        ChGenb(::core::ptr::read_volatile(((self.0 as usize) + 0x64 + (index * 64)) as *const u32))
     }
  }
#[doc="Write the CH_GENB register."]
  #[inline] pub fn set_ch_genb(&self, index: usize, value: ChGenb) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x64 + (index * 64)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CH_GENB register."]
  #[inline] pub fn with_ch_genb<F: FnOnce(ChGenb) -> ChGenb>(&self, index: usize, f: F) -> &Self {
     let tmp = self.ch_genb(index);
     self.set_ch_genb(index, f(tmp))
  }

#[doc="Get the *const pointer for the CH_DBCTL register."]
  #[inline] pub fn ch_dbctl_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x68 + (index * 64)) as *const u32
  }
#[doc="Get the *mut pointer for the CH_DBCTL register."]
  #[inline] pub fn ch_dbctl_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x68 + (index * 64)) as *mut u32
  }
#[doc="Read the CH_DBCTL register."]
  #[inline] pub fn ch_dbctl(&self, index: usize) -> ChDbctl { 
     assert!(index < 4);
     unsafe {
        ChDbctl(::core::ptr::read_volatile(((self.0 as usize) + 0x68 + (index * 64)) as *const u32))
     }
  }
#[doc="Write the CH_DBCTL register."]
  #[inline] pub fn set_ch_dbctl(&self, index: usize, value: ChDbctl) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x68 + (index * 64)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CH_DBCTL register."]
  #[inline] pub fn with_ch_dbctl<F: FnOnce(ChDbctl) -> ChDbctl>(&self, index: usize, f: F) -> &Self {
     let tmp = self.ch_dbctl(index);
     self.set_ch_dbctl(index, f(tmp))
  }

#[doc="Get the *const pointer for the CH_DBRISE register."]
  #[inline] pub fn ch_dbrise_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x6c + (index * 64)) as *const u32
  }
#[doc="Get the *mut pointer for the CH_DBRISE register."]
  #[inline] pub fn ch_dbrise_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x6c + (index * 64)) as *mut u32
  }
#[doc="Read the CH_DBRISE register."]
  #[inline] pub fn ch_dbrise(&self, index: usize) -> ChDbrise { 
     assert!(index < 4);
     unsafe {
        ChDbrise(::core::ptr::read_volatile(((self.0 as usize) + 0x6c + (index * 64)) as *const u32))
     }
  }
#[doc="Write the CH_DBRISE register."]
  #[inline] pub fn set_ch_dbrise(&self, index: usize, value: ChDbrise) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x6c + (index * 64)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CH_DBRISE register."]
  #[inline] pub fn with_ch_dbrise<F: FnOnce(ChDbrise) -> ChDbrise>(&self, index: usize, f: F) -> &Self {
     let tmp = self.ch_dbrise(index);
     self.set_ch_dbrise(index, f(tmp))
  }

#[doc="Get the *const pointer for the CH_DBFALL register."]
  #[inline] pub fn ch_dbfall_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x70 + (index * 64)) as *const u32
  }
#[doc="Get the *mut pointer for the CH_DBFALL register."]
  #[inline] pub fn ch_dbfall_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x70 + (index * 64)) as *mut u32
  }
#[doc="Read the CH_DBFALL register."]
  #[inline] pub fn ch_dbfall(&self, index: usize) -> ChDbfall { 
     assert!(index < 4);
     unsafe {
        ChDbfall(::core::ptr::read_volatile(((self.0 as usize) + 0x70 + (index * 64)) as *const u32))
     }
  }
#[doc="Write the CH_DBFALL register."]
  #[inline] pub fn set_ch_dbfall(&self, index: usize, value: ChDbfall) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x70 + (index * 64)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CH_DBFALL register."]
  #[inline] pub fn with_ch_dbfall<F: FnOnce(ChDbfall) -> ChDbfall>(&self, index: usize, f: F) -> &Self {
     let tmp = self.ch_dbfall(index);
     self.set_ch_dbfall(index, f(tmp))
  }

#[doc="Get the *const pointer for the CH_FLTSRC0 register."]
  #[inline] pub fn ch_fltsrc0_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x74 + (index * 64)) as *const u32
  }
#[doc="Get the *mut pointer for the CH_FLTSRC0 register."]
  #[inline] pub fn ch_fltsrc0_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x74 + (index * 64)) as *mut u32
  }
#[doc="Read the CH_FLTSRC0 register."]
  #[inline] pub fn ch_fltsrc0(&self, index: usize) -> ChFltsrc0 { 
     assert!(index < 4);
     unsafe {
        ChFltsrc0(::core::ptr::read_volatile(((self.0 as usize) + 0x74 + (index * 64)) as *const u32))
     }
  }
#[doc="Write the CH_FLTSRC0 register."]
  #[inline] pub fn set_ch_fltsrc0(&self, index: usize, value: ChFltsrc0) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x74 + (index * 64)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CH_FLTSRC0 register."]
  #[inline] pub fn with_ch_fltsrc0<F: FnOnce(ChFltsrc0) -> ChFltsrc0>(&self, index: usize, f: F) -> &Self {
     let tmp = self.ch_fltsrc0(index);
     self.set_ch_fltsrc0(index, f(tmp))
  }

#[doc="Get the *const pointer for the CH_FLTSRC1 register."]
  #[inline] pub fn ch_fltsrc1_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x78 + (index * 64)) as *const u32
  }
#[doc="Get the *mut pointer for the CH_FLTSRC1 register."]
  #[inline] pub fn ch_fltsrc1_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x78 + (index * 64)) as *mut u32
  }
#[doc="Read the CH_FLTSRC1 register."]
  #[inline] pub fn ch_fltsrc1(&self, index: usize) -> ChFltsrc1 { 
     assert!(index < 4);
     unsafe {
        ChFltsrc1(::core::ptr::read_volatile(((self.0 as usize) + 0x78 + (index * 64)) as *const u32))
     }
  }
#[doc="Write the CH_FLTSRC1 register."]
  #[inline] pub fn set_ch_fltsrc1(&self, index: usize, value: ChFltsrc1) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x78 + (index * 64)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CH_FLTSRC1 register."]
  #[inline] pub fn with_ch_fltsrc1<F: FnOnce(ChFltsrc1) -> ChFltsrc1>(&self, index: usize, f: F) -> &Self {
     let tmp = self.ch_fltsrc1(index);
     self.set_ch_fltsrc1(index, f(tmp))
  }

#[doc="Get the *const pointer for the CH_MINFLTPER register."]
  #[inline] pub fn ch_minfltper_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x7c + (index * 64)) as *const u32
  }
#[doc="Get the *mut pointer for the CH_MINFLTPER register."]
  #[inline] pub fn ch_minfltper_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x7c + (index * 64)) as *mut u32
  }
#[doc="Read the CH_MINFLTPER register."]
  #[inline] pub fn ch_minfltper(&self, index: usize) -> ChMinfltper { 
     assert!(index < 4);
     unsafe {
        ChMinfltper(::core::ptr::read_volatile(((self.0 as usize) + 0x7c + (index * 64)) as *const u32))
     }
  }
#[doc="Write the CH_MINFLTPER register."]
  #[inline] pub fn set_ch_minfltper(&self, index: usize, value: ChMinfltper) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x7c + (index * 64)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CH_MINFLTPER register."]
  #[inline] pub fn with_ch_minfltper<F: FnOnce(ChMinfltper) -> ChMinfltper>(&self, index: usize, f: F) -> &Self {
     let tmp = self.ch_minfltper(index);
     self.set_ch_minfltper(index, f(tmp))
  }

#[doc="Get the *const pointer for the CH_FLTSEN register."]
  #[inline] pub fn ch_fltsen_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x800 + (index * 128)) as *const u32
  }
#[doc="Get the *mut pointer for the CH_FLTSEN register."]
  #[inline] pub fn ch_fltsen_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x800 + (index * 128)) as *mut u32
  }
#[doc="Read the CH_FLTSEN register."]
  #[inline] pub fn ch_fltsen(&self, index: usize) -> ChFltsen { 
     assert!(index < 4);
     unsafe {
        ChFltsen(::core::ptr::read_volatile(((self.0 as usize) + 0x800 + (index * 128)) as *const u32))
     }
  }
#[doc="Write the CH_FLTSEN register."]
  #[inline] pub fn set_ch_fltsen(&self, index: usize, value: ChFltsen) -> &Self {
     assert!(index < 4);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x800 + (index * 128)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CH_FLTSEN register."]
  #[inline] pub fn with_ch_fltsen<F: FnOnce(ChFltsen) -> ChFltsen>(&self, index: usize, f: F) -> &Self {
     let tmp = self.ch_fltsen(index);
     self.set_ch_fltsen(index, f(tmp))
  }

#[doc="Get the *const pointer for the CH_FLTSTAT0 register."]
  #[inline] pub fn ch_fltstat0_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x804 + (index * 128)) as *const u32
  }
#[doc="Get the *mut pointer for the CH_FLTSTAT0 register."]
  #[inline] pub fn ch_fltstat0_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 4);
     ((self.0 as usize) + 0x804 + (index * 128)) as *mut u32
  }
#[doc="Read the CH_FLTSTAT0 register."]
  #[inline] pub fn ch_fltstat0(&self, index: usize) -> ChFltstat0 { 
     assert!(index < 4);
     unsafe {
        ChFltstat0(::core::ptr::read_volatile(((self.0 as usize) + 0x804 + (index * 128)) as *const u32))
     }
  }

#[doc="Get the *const pointer for the CH_FLTSTAT1 register."]
  #[inline] pub fn ch_fltstat1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x808) as *const u32
  }
#[doc="Get the *mut pointer for the CH_FLTSTAT1 register."]
  #[inline] pub fn ch_fltstat1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x808) as *mut u32
  }
#[doc="Read the CH_FLTSTAT1 register."]
  #[inline] pub fn ch_fltstat1(&self) -> ChFltstat1 { 
     unsafe {
        ChFltstat1(::core::ptr::read_volatile(((self.0 as usize) + 0x808) as *const u32))
     }
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

#[doc="PWM Master Control"]
#[derive(PartialEq, Eq)]
pub struct Ctl(pub u32);
impl Ctl {
#[doc="Update PWM Generator n"]
  #[inline] pub fn globalsync(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
#[doc="Update PWM Generator n"]
  #[inline] pub fn set_globalsync(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Ctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.globalsync(0) != 0 { try!(write!(f, " globalsync[0]"))}
      if self.globalsync(1) != 0 { try!(write!(f, " globalsync[1]"))}
      if self.globalsync(2) != 0 { try!(write!(f, " globalsync[2]"))}
      if self.globalsync(3) != 0 { try!(write!(f, " globalsync[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Time Base Sync"]
#[derive(PartialEq, Eq)]
pub struct Sync(pub u32);
impl Sync {
#[doc="Reset Generator n Counter"]
  #[inline] pub fn sync(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
#[doc="Reset Generator n Counter"]
  #[inline] pub fn set_sync(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Sync {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sync {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sync(0) != 0 { try!(write!(f, " sync[0]"))}
      if self.sync(1) != 0 { try!(write!(f, " sync[1]"))}
      if self.sync(2) != 0 { try!(write!(f, " sync[2]"))}
      if self.sync(3) != 0 { try!(write!(f, " sync[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Output Enable"]
#[derive(PartialEq, Eq)]
pub struct Enable(pub u32);
impl Enable {
#[doc="MnPWMn Output Enable"]
  #[inline] pub fn pwmen(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
#[doc="MnPWMn Output Enable"]
  #[inline] pub fn set_pwmen(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Enable {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Enable {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwmen(0) != 0 { try!(write!(f, " pwmen[0]"))}
      if self.pwmen(1) != 0 { try!(write!(f, " pwmen[1]"))}
      if self.pwmen(2) != 0 { try!(write!(f, " pwmen[2]"))}
      if self.pwmen(3) != 0 { try!(write!(f, " pwmen[3]"))}
      if self.pwmen(4) != 0 { try!(write!(f, " pwmen[4]"))}
      if self.pwmen(5) != 0 { try!(write!(f, " pwmen[5]"))}
      if self.pwmen(6) != 0 { try!(write!(f, " pwmen[6]"))}
      if self.pwmen(7) != 0 { try!(write!(f, " pwmen[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Output Inversion"]
#[derive(PartialEq, Eq)]
pub struct Invert(pub u32);
impl Invert {
#[doc="Invert MnPWMn Signal"]
  #[inline] pub fn pwminv(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
#[doc="Invert MnPWMn Signal"]
  #[inline] pub fn set_pwminv(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Invert {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Invert {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwminv(0) != 0 { try!(write!(f, " pwminv[0]"))}
      if self.pwminv(1) != 0 { try!(write!(f, " pwminv[1]"))}
      if self.pwminv(2) != 0 { try!(write!(f, " pwminv[2]"))}
      if self.pwminv(3) != 0 { try!(write!(f, " pwminv[3]"))}
      if self.pwminv(4) != 0 { try!(write!(f, " pwminv[4]"))}
      if self.pwminv(5) != 0 { try!(write!(f, " pwminv[5]"))}
      if self.pwminv(6) != 0 { try!(write!(f, " pwminv[6]"))}
      if self.pwminv(7) != 0 { try!(write!(f, " pwminv[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Output Fault"]
#[derive(PartialEq, Eq)]
pub struct Fault(pub u32);
impl Fault {
#[doc="MnPWMn Fault"]
  #[inline] pub fn fault(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
#[doc="MnPWMn Fault"]
  #[inline] pub fn set_fault(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Fault {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fault {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fault(0) != 0 { try!(write!(f, " fault[0]"))}
      if self.fault(1) != 0 { try!(write!(f, " fault[1]"))}
      if self.fault(2) != 0 { try!(write!(f, " fault[2]"))}
      if self.fault(3) != 0 { try!(write!(f, " fault[3]"))}
      if self.fault(4) != 0 { try!(write!(f, " fault[4]"))}
      if self.fault(5) != 0 { try!(write!(f, " fault[5]"))}
      if self.fault(6) != 0 { try!(write!(f, " fault[6]"))}
      if self.fault(7) != 0 { try!(write!(f, " fault[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Interrupt Enable"]
#[derive(PartialEq, Eq)]
pub struct Inten(pub u32);
impl Inten {
#[doc="PWMn Interrupt Enable"]
  #[inline] pub fn intpwm(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
#[doc="PWMn Interrupt Enable"]
  #[inline] pub fn set_intpwm(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Interrupt Fault 0"]
  #[inline] pub fn intfault(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 16 + index;
     ((self.0 as u32) >> shift) & 0x1 // [16]
  }
#[doc="Interrupt Fault 0"]
  #[inline] pub fn set_intfault(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 16 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Interrupt Fault 1"]
  #[inline] pub fn pwm_inten_intfault1(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="Interrupt Fault 1"]
  #[inline] pub fn set_pwm_inten_intfault1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="Interrupt Fault 2"]
  #[inline] pub fn pwm_inten_intfault2(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="Interrupt Fault 2"]
  #[inline] pub fn set_pwm_inten_intfault2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="Interrupt Fault 3"]
  #[inline] pub fn pwm_inten_intfault3(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="Interrupt Fault 3"]
  #[inline] pub fn set_pwm_inten_intfault3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

}
impl ::core::fmt::Display for Inten {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Inten {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.intpwm(0) != 0 { try!(write!(f, " intpwm[0]"))}
      if self.intpwm(1) != 0 { try!(write!(f, " intpwm[1]"))}
      if self.intpwm(2) != 0 { try!(write!(f, " intpwm[2]"))}
      if self.intpwm(3) != 0 { try!(write!(f, " intpwm[3]"))}
      if self.intfault(0) != 0 { try!(write!(f, " intfault[0]"))}
      if self.intfault(1) != 0 { try!(write!(f, " intfault[1]"))}
      if self.intfault(2) != 0 { try!(write!(f, " intfault[2]"))}
      if self.intfault(3) != 0 { try!(write!(f, " intfault[3]"))}
      if self.pwm_inten_intfault1() != 0 { try!(write!(f, " pwm_inten_intfault1"))}
      if self.pwm_inten_intfault2() != 0 { try!(write!(f, " pwm_inten_intfault2"))}
      if self.pwm_inten_intfault3() != 0 { try!(write!(f, " pwm_inten_intfault3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Raw Interrupt Status"]
#[derive(PartialEq, Eq)]
pub struct Ris(pub u32);
impl Ris {
#[doc="PWM0 Interrupt Asserted"]
  #[inline] pub fn intpwm(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
#[doc="PWM0 Interrupt Asserted"]
  #[inline] pub fn set_intpwm(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="Interrupt Fault PWM 0"]
  #[inline] pub fn intfault(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 16 + index;
     ((self.0 as u32) >> shift) & 0x1 // [16]
  }
#[doc="Interrupt Fault PWM 0"]
  #[inline] pub fn set_intfault(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 16 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
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
      if self.intpwm(0) != 0 { try!(write!(f, " intpwm[0]"))}
      if self.intpwm(1) != 0 { try!(write!(f, " intpwm[1]"))}
      if self.intpwm(2) != 0 { try!(write!(f, " intpwm[2]"))}
      if self.intpwm(3) != 0 { try!(write!(f, " intpwm[3]"))}
      if self.intfault(0) != 0 { try!(write!(f, " intfault[0]"))}
      if self.intfault(1) != 0 { try!(write!(f, " intfault[1]"))}
      if self.intfault(2) != 0 { try!(write!(f, " intfault[2]"))}
      if self.intfault(3) != 0 { try!(write!(f, " intfault[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Interrupt Status and Clear"]
#[derive(PartialEq, Eq)]
pub struct Isc(pub u32);
impl Isc {
#[doc="PWMn Interrupt Status"]
  #[inline] pub fn intpwm(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
#[doc="PWMn Interrupt Status"]
  #[inline] pub fn set_intpwm(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

#[doc="FAULTn Interrupt Asserted"]
  #[inline] pub fn intfault(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 16 + index;
     ((self.0 as u32) >> shift) & 0x1 // [16]
  }
#[doc="FAULTn Interrupt Asserted"]
  #[inline] pub fn set_intfault(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 16 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Isc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Isc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.intpwm(0) != 0 { try!(write!(f, " intpwm[0]"))}
      if self.intpwm(1) != 0 { try!(write!(f, " intpwm[1]"))}
      if self.intpwm(2) != 0 { try!(write!(f, " intpwm[2]"))}
      if self.intpwm(3) != 0 { try!(write!(f, " intpwm[3]"))}
      if self.intfault(0) != 0 { try!(write!(f, " intfault[0]"))}
      if self.intfault(1) != 0 { try!(write!(f, " intfault[1]"))}
      if self.intfault(2) != 0 { try!(write!(f, " intfault[2]"))}
      if self.intfault(3) != 0 { try!(write!(f, " intfault[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Status"]
#[derive(PartialEq, Eq)]
pub struct Status(pub u32);
impl Status {
#[doc="Generator n Fault Status"]
  #[inline] pub fn fault(&self, index: usize) -> u32 {
     assert!(index < 4);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
#[doc="Generator n Fault Status"]
  #[inline] pub fn set_fault(mut self, index: usize, value: u32) -> Self {
     assert!(index < 4);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fault(0) != 0 { try!(write!(f, " fault[0]"))}
      if self.fault(1) != 0 { try!(write!(f, " fault[1]"))}
      if self.fault(2) != 0 { try!(write!(f, " fault[2]"))}
      if self.fault(3) != 0 { try!(write!(f, " fault[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Fault Condition Value"]
#[derive(PartialEq, Eq)]
pub struct Faultval(pub u32);
impl Faultval {
#[doc="MnPWM0 Fault Value"]
  #[inline] pub fn faultval(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
#[doc="MnPWM0 Fault Value"]
  #[inline] pub fn set_faultval(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Faultval {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Faultval {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.faultval(0) != 0 { try!(write!(f, " faultval[0]"))}
      if self.faultval(1) != 0 { try!(write!(f, " faultval[1]"))}
      if self.faultval(2) != 0 { try!(write!(f, " faultval[2]"))}
      if self.faultval(3) != 0 { try!(write!(f, " faultval[3]"))}
      if self.faultval(4) != 0 { try!(write!(f, " faultval[4]"))}
      if self.faultval(5) != 0 { try!(write!(f, " faultval[5]"))}
      if self.faultval(6) != 0 { try!(write!(f, " faultval[6]"))}
      if self.faultval(7) != 0 { try!(write!(f, " faultval[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Enable Update"]
#[derive(PartialEq, Eq)]
pub struct Enupd(pub u32);
impl Enupd {
#[doc="MnPWMn Enable Update Mode"]
  #[inline] pub fn enupd(&self, index: usize) -> u32 {
     assert!(index < 8);
     let shift: usize = 0 + index;
     ((self.0 as u32) >> shift) & 0x3 // [1:0]
  }
#[doc="MnPWMn Enable Update Mode"]
  #[inline] pub fn set_enupd(mut self, index: usize, value: u32) -> Self {
     assert!(index < 8);
     assert!((value & !0x3) == 0);
     let shift: usize = 0 + index;
     self.0 &= !(0x3 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Enupd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Enupd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.enupd(0) != 0 { try!(write!(f, " enupd[0]=0x{:x}", self.enupd(0)))}
      if self.enupd(1) != 0 { try!(write!(f, " enupd[1]=0x{:x}", self.enupd(1)))}
      if self.enupd(2) != 0 { try!(write!(f, " enupd[2]=0x{:x}", self.enupd(2)))}
      if self.enupd(3) != 0 { try!(write!(f, " enupd[3]=0x{:x}", self.enupd(3)))}
      if self.enupd(4) != 0 { try!(write!(f, " enupd[4]=0x{:x}", self.enupd(4)))}
      if self.enupd(5) != 0 { try!(write!(f, " enupd[5]=0x{:x}", self.enupd(5)))}
      if self.enupd(6) != 0 { try!(write!(f, " enupd[6]=0x{:x}", self.enupd(6)))}
      if self.enupd(7) != 0 { try!(write!(f, " enupd[7]=0x{:x}", self.enupd(7)))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWMn Control"]
#[derive(PartialEq, Eq)]
pub struct ChCtl(pub u32);
impl ChCtl {
#[doc="PWM Block Enable"]
  #[inline] pub fn enable(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="PWM Block Enable"]
  #[inline] pub fn set_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Counter Mode"]
  #[inline] pub fn ctl_mode(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Counter Mode"]
  #[inline] pub fn set_ctl_mode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Debug Mode"]
  #[inline] pub fn ctl_debug(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Debug Mode"]
  #[inline] pub fn set_ctl_debug(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Load Register Update Mode"]
  #[inline] pub fn ctl_loadupd(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Load Register Update Mode"]
  #[inline] pub fn set_ctl_loadupd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Comparator A Update Mode"]
  #[inline] pub fn ctl_cmpaupd(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Comparator A Update Mode"]
  #[inline] pub fn set_ctl_cmpaupd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Comparator B Update Mode"]
  #[inline] pub fn ctl_cmpbupd(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Comparator B Update Mode"]
  #[inline] pub fn set_ctl_cmpbupd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="PWMnGENA Update Mode"]
  #[inline] pub fn ctl_genaupd(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
#[doc="PWMnGENA Update Mode"]
  #[inline] pub fn set_ctl_genaupd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="PWMnGENB Update Mode"]
  #[inline] pub fn ctl_genbupd(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
#[doc="PWMnGENB Update Mode"]
  #[inline] pub fn set_ctl_genbupd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="PWMnDBCTL Update Mode"]
  #[inline] pub fn ctl_dbctlupd(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
#[doc="PWMnDBCTL Update Mode"]
  #[inline] pub fn set_ctl_dbctlupd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="PWMnDBRISE Update Mode"]
  #[inline] pub fn ctl_dbriseupd(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x3 // [13:12]
  }
#[doc="PWMnDBRISE Update Mode"]
  #[inline] pub fn set_ctl_dbriseupd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="PWMnDBFALL Update Mode"]
  #[inline] pub fn ctl_dbfallupd(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x3 // [15:14]
  }
#[doc="PWMnDBFALL Update Mode"]
  #[inline] pub fn set_ctl_dbfallupd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Fault Condition Source"]
  #[inline] pub fn ctl_fltsrc(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="Fault Condition Source"]
  #[inline] pub fn set_ctl_fltsrc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Minimum Fault Period"]
  #[inline] pub fn ctl_minfltper(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="Minimum Fault Period"]
  #[inline] pub fn set_ctl_minfltper(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="Latch Fault Input"]
  #[inline] pub fn ctl_latch(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="Latch Fault Input"]
  #[inline] pub fn set_ctl_latch(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

}
impl ::core::fmt::Display for ChCtl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChCtl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.ctl_mode() != 0 { try!(write!(f, " ctl_mode"))}
      if self.ctl_debug() != 0 { try!(write!(f, " ctl_debug"))}
      if self.ctl_loadupd() != 0 { try!(write!(f, " ctl_loadupd"))}
      if self.ctl_cmpaupd() != 0 { try!(write!(f, " ctl_cmpaupd"))}
      if self.ctl_cmpbupd() != 0 { try!(write!(f, " ctl_cmpbupd"))}
      if self.ctl_genaupd() != 0 { try!(write!(f, " ctl_genaupd=0x{:x}", self.ctl_genaupd()))}
      if self.ctl_genbupd() != 0 { try!(write!(f, " ctl_genbupd=0x{:x}", self.ctl_genbupd()))}
      if self.ctl_dbctlupd() != 0 { try!(write!(f, " ctl_dbctlupd=0x{:x}", self.ctl_dbctlupd()))}
      if self.ctl_dbriseupd() != 0 { try!(write!(f, " ctl_dbriseupd=0x{:x}", self.ctl_dbriseupd()))}
      if self.ctl_dbfallupd() != 0 { try!(write!(f, " ctl_dbfallupd=0x{:x}", self.ctl_dbfallupd()))}
      if self.ctl_fltsrc() != 0 { try!(write!(f, " ctl_fltsrc"))}
      if self.ctl_minfltper() != 0 { try!(write!(f, " ctl_minfltper"))}
      if self.ctl_latch() != 0 { try!(write!(f, " ctl_latch"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM0 Interrupt and Trigger Enable"]
#[derive(PartialEq, Eq)]
pub struct ChInten(pub u32);
impl ChInten {
#[doc="Interrupt for Counter=0"]
  #[inline] pub fn inten_intcntzero(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Interrupt for Counter=0"]
  #[inline] pub fn set_inten_intcntzero(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Interrupt for Counter=PWMnLOAD"]
  #[inline] pub fn inten_intcntload(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Interrupt for Counter=PWMnLOAD"]
  #[inline] pub fn set_inten_intcntload(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Interrupt for Counter=PWMnCMPA Up"]
  #[inline] pub fn inten_intcmpau(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Interrupt for Counter=PWMnCMPA Up"]
  #[inline] pub fn set_inten_intcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Interrupt for Counter=PWMnCMPA Down"]
  #[inline] pub fn inten_intcmpad(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Interrupt for Counter=PWMnCMPA Down"]
  #[inline] pub fn set_inten_intcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Interrupt for Counter=PWMnCMPB Up"]
  #[inline] pub fn inten_intcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Interrupt for Counter=PWMnCMPB Up"]
  #[inline] pub fn set_inten_intcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Interrupt for Counter=PWMnCMPB Down"]
  #[inline] pub fn inten_intcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Interrupt for Counter=PWMnCMPB Down"]
  #[inline] pub fn set_inten_intcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Trigger for Counter=0"]
  #[inline] pub fn inten_trcntzero(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Trigger for Counter=0"]
  #[inline] pub fn set_inten_trcntzero(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Trigger for Counter=PWMnLOAD"]
  #[inline] pub fn inten_trcntload(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Trigger for Counter=PWMnLOAD"]
  #[inline] pub fn set_inten_trcntload(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Trigger for Counter=PWMnCMPA Up"]
  #[inline] pub fn inten_trcmpau(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="Trigger for Counter=PWMnCMPA Up"]
  #[inline] pub fn set_inten_trcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Trigger for Counter=PWMnCMPA Down"]
  #[inline] pub fn inten_trcmpad(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="Trigger for Counter=PWMnCMPA Down"]
  #[inline] pub fn set_inten_trcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Trigger for Counter=PWMnCMPB Up"]
  #[inline] pub fn inten_trcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="Trigger for Counter=PWMnCMPB Up"]
  #[inline] pub fn set_inten_trcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Trigger for Counter=PWMnCMPB Down"]
  #[inline] pub fn inten_trcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="Trigger for Counter=PWMnCMPB Down"]
  #[inline] pub fn set_inten_trcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

}
impl ::core::fmt::Display for ChInten {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChInten {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.inten_intcntzero() != 0 { try!(write!(f, " inten_intcntzero"))}
      if self.inten_intcntload() != 0 { try!(write!(f, " inten_intcntload"))}
      if self.inten_intcmpau() != 0 { try!(write!(f, " inten_intcmpau"))}
      if self.inten_intcmpad() != 0 { try!(write!(f, " inten_intcmpad"))}
      if self.inten_intcmpbu() != 0 { try!(write!(f, " inten_intcmpbu"))}
      if self.inten_intcmpbd() != 0 { try!(write!(f, " inten_intcmpbd"))}
      if self.inten_trcntzero() != 0 { try!(write!(f, " inten_trcntzero"))}
      if self.inten_trcntload() != 0 { try!(write!(f, " inten_trcntload"))}
      if self.inten_trcmpau() != 0 { try!(write!(f, " inten_trcmpau"))}
      if self.inten_trcmpad() != 0 { try!(write!(f, " inten_trcmpad"))}
      if self.inten_trcmpbu() != 0 { try!(write!(f, " inten_trcmpbu"))}
      if self.inten_trcmpbd() != 0 { try!(write!(f, " inten_trcmpbd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM0 Raw Interrupt Status"]
#[derive(PartialEq, Eq)]
pub struct ChRis(pub u32);
impl ChRis {
#[doc="Counter=0 Interrupt Status"]
  #[inline] pub fn ris_intcntzero(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Counter=0 Interrupt Status"]
  #[inline] pub fn set_ris_intcntzero(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Counter=Load Interrupt Status"]
  #[inline] pub fn ris_intcntload(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Counter=Load Interrupt Status"]
  #[inline] pub fn set_ris_intcntload(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Comparator A Up Interrupt Status"]
  #[inline] pub fn ris_intcmpau(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Comparator A Up Interrupt Status"]
  #[inline] pub fn set_ris_intcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Comparator A Down Interrupt Status"]
  #[inline] pub fn ris_intcmpad(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Comparator A Down Interrupt Status"]
  #[inline] pub fn set_ris_intcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Comparator B Up Interrupt Status"]
  #[inline] pub fn ris_intcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Comparator B Up Interrupt Status"]
  #[inline] pub fn set_ris_intcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Comparator B Down Interrupt Status"]
  #[inline] pub fn ris_intcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Comparator B Down Interrupt Status"]
  #[inline] pub fn set_ris_intcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for ChRis {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChRis {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ris_intcntzero() != 0 { try!(write!(f, " ris_intcntzero"))}
      if self.ris_intcntload() != 0 { try!(write!(f, " ris_intcntload"))}
      if self.ris_intcmpau() != 0 { try!(write!(f, " ris_intcmpau"))}
      if self.ris_intcmpad() != 0 { try!(write!(f, " ris_intcmpad"))}
      if self.ris_intcmpbu() != 0 { try!(write!(f, " ris_intcmpbu"))}
      if self.ris_intcmpbd() != 0 { try!(write!(f, " ris_intcmpbd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM0 Interrupt Status and Clear"]
#[derive(PartialEq, Eq)]
pub struct ChIsc(pub u32);
impl ChIsc {
#[doc="Counter=0 Interrupt"]
  #[inline] pub fn isc_intcntzero(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Counter=0 Interrupt"]
  #[inline] pub fn set_isc_intcntzero(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Counter=Load Interrupt"]
  #[inline] pub fn isc_intcntload(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Counter=Load Interrupt"]
  #[inline] pub fn set_isc_intcntload(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Comparator A Up Interrupt"]
  #[inline] pub fn isc_intcmpau(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Comparator A Up Interrupt"]
  #[inline] pub fn set_isc_intcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Comparator A Down Interrupt"]
  #[inline] pub fn isc_intcmpad(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Comparator A Down Interrupt"]
  #[inline] pub fn set_isc_intcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Comparator B Up Interrupt"]
  #[inline] pub fn isc_intcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Comparator B Up Interrupt"]
  #[inline] pub fn set_isc_intcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Comparator B Down Interrupt"]
  #[inline] pub fn isc_intcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Comparator B Down Interrupt"]
  #[inline] pub fn set_isc_intcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

}
impl ::core::fmt::Display for ChIsc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChIsc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.isc_intcntzero() != 0 { try!(write!(f, " isc_intcntzero"))}
      if self.isc_intcntload() != 0 { try!(write!(f, " isc_intcntload"))}
      if self.isc_intcmpau() != 0 { try!(write!(f, " isc_intcmpau"))}
      if self.isc_intcmpad() != 0 { try!(write!(f, " isc_intcmpad"))}
      if self.isc_intcmpbu() != 0 { try!(write!(f, " isc_intcmpbu"))}
      if self.isc_intcmpbd() != 0 { try!(write!(f, " isc_intcmpbd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM0 Load"]
#[derive(PartialEq, Eq)]
pub struct ChLoad(pub u32);
impl ChLoad {
#[doc="Counter Load Value"]
  #[inline] pub fn load(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Counter Load Value"]
  #[inline] pub fn set_load(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for ChLoad {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChLoad {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.load() != 0 { try!(write!(f, " load=0x{:x}", self.load()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM0 Counter"]
#[derive(PartialEq, Eq)]
pub struct ChCount(pub u32);
impl ChCount {
#[doc="Counter Value"]
  #[inline] pub fn count(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Counter Value"]
  #[inline] pub fn set_count(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for ChCount {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChCount {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM0 Compare A"]
#[derive(PartialEq, Eq)]
pub struct ChCmpa(pub u32);
impl ChCmpa {
#[doc="Comparator A Value"]
  #[inline] pub fn cmpa(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Comparator A Value"]
  #[inline] pub fn set_cmpa(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for ChCmpa {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChCmpa {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cmpa() != 0 { try!(write!(f, " cmpa=0x{:x}", self.cmpa()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM0 Compare B"]
#[derive(PartialEq, Eq)]
pub struct ChCmpb(pub u32);
impl ChCmpb {
#[doc="Comparator B Value"]
  #[inline] pub fn cmpb(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Comparator B Value"]
  #[inline] pub fn set_cmpb(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for ChCmpb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChCmpb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cmpb() != 0 { try!(write!(f, " cmpb=0x{:x}", self.cmpb()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM0 Generator A Control"]
#[derive(PartialEq, Eq)]
pub struct ChGena(pub u32);
impl ChGena {
#[doc="Action for Counter=0"]
  #[inline] pub fn gena_actzero(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
#[doc="Action for Counter=0"]
  #[inline] pub fn set_gena_actzero(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Action for Counter=LOAD"]
  #[inline] pub fn gena_actload(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
#[doc="Action for Counter=LOAD"]
  #[inline] pub fn set_gena_actload(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Action for Comparator A Up"]
  #[inline] pub fn gena_actcmpau(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
#[doc="Action for Comparator A Up"]
  #[inline] pub fn set_gena_actcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Action for Comparator A Down"]
  #[inline] pub fn gena_actcmpad(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
#[doc="Action for Comparator A Down"]
  #[inline] pub fn set_gena_actcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Action for Comparator B Up"]
  #[inline] pub fn gena_actcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
#[doc="Action for Comparator B Up"]
  #[inline] pub fn set_gena_actcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Action for Comparator B Down"]
  #[inline] pub fn gena_actcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
#[doc="Action for Comparator B Down"]
  #[inline] pub fn set_gena_actcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

}
impl ::core::fmt::Display for ChGena {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChGena {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.gena_actzero() != 0 { try!(write!(f, " gena_actzero=0x{:x}", self.gena_actzero()))}
      if self.gena_actload() != 0 { try!(write!(f, " gena_actload=0x{:x}", self.gena_actload()))}
      if self.gena_actcmpau() != 0 { try!(write!(f, " gena_actcmpau=0x{:x}", self.gena_actcmpau()))}
      if self.gena_actcmpad() != 0 { try!(write!(f, " gena_actcmpad=0x{:x}", self.gena_actcmpad()))}
      if self.gena_actcmpbu() != 0 { try!(write!(f, " gena_actcmpbu=0x{:x}", self.gena_actcmpbu()))}
      if self.gena_actcmpbd() != 0 { try!(write!(f, " gena_actcmpbd=0x{:x}", self.gena_actcmpbd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM0 Generator B Control"]
#[derive(PartialEq, Eq)]
pub struct ChGenb(pub u32);
impl ChGenb {
#[doc="Action for Counter=0"]
  #[inline] pub fn genb_actzero(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x3 // [1:0]
  }
#[doc="Action for Counter=0"]
  #[inline] pub fn set_genb_actzero(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Action for Counter=LOAD"]
  #[inline] pub fn genb_actload(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x3 // [3:2]
  }
#[doc="Action for Counter=LOAD"]
  #[inline] pub fn set_genb_actload(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Action for Comparator A Up"]
  #[inline] pub fn genb_actcmpau(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
#[doc="Action for Comparator A Up"]
  #[inline] pub fn set_genb_actcmpau(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Action for Comparator A Down"]
  #[inline] pub fn genb_actcmpad(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x3 // [7:6]
  }
#[doc="Action for Comparator A Down"]
  #[inline] pub fn set_genb_actcmpad(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Action for Comparator B Up"]
  #[inline] pub fn genb_actcmpbu(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
#[doc="Action for Comparator B Up"]
  #[inline] pub fn set_genb_actcmpbu(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Action for Comparator B Down"]
  #[inline] pub fn genb_actcmpbd(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
#[doc="Action for Comparator B Down"]
  #[inline] pub fn set_genb_actcmpbd(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

}
impl ::core::fmt::Display for ChGenb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChGenb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.genb_actzero() != 0 { try!(write!(f, " genb_actzero=0x{:x}", self.genb_actzero()))}
      if self.genb_actload() != 0 { try!(write!(f, " genb_actload=0x{:x}", self.genb_actload()))}
      if self.genb_actcmpau() != 0 { try!(write!(f, " genb_actcmpau=0x{:x}", self.genb_actcmpau()))}
      if self.genb_actcmpad() != 0 { try!(write!(f, " genb_actcmpad=0x{:x}", self.genb_actcmpad()))}
      if self.genb_actcmpbu() != 0 { try!(write!(f, " genb_actcmpbu=0x{:x}", self.genb_actcmpbu()))}
      if self.genb_actcmpbd() != 0 { try!(write!(f, " genb_actcmpbd=0x{:x}", self.genb_actcmpbd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM0 Dead-Band Control"]
#[derive(PartialEq, Eq)]
pub struct ChDbctl(pub u32);
impl ChDbctl {
#[doc="Dead-Band Generator Enable"]
  #[inline] pub fn dbctl_enable(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Dead-Band Generator Enable"]
  #[inline] pub fn set_dbctl_enable(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for ChDbctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChDbctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbctl_enable() != 0 { try!(write!(f, " dbctl_enable"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM0 Dead-Band Rising-Edge Delay"]
#[derive(PartialEq, Eq)]
pub struct ChDbrise(pub u32);
impl ChDbrise {
#[doc="Dead-Band Rise Delay"]
  #[inline] pub fn dbrise_delay(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
#[doc="Dead-Band Rise Delay"]
  #[inline] pub fn set_dbrise_delay(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for ChDbrise {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChDbrise {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbrise_delay() != 0 { try!(write!(f, " dbrise_delay=0x{:x}", self.dbrise_delay()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM0 Dead-Band Falling-Edge-Delay"]
#[derive(PartialEq, Eq)]
pub struct ChDbfall(pub u32);
impl ChDbfall {
#[doc="Dead-Band Fall Delay"]
  #[inline] pub fn dbfall_delay(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xfff // [11:0]
  }
#[doc="Dead-Band Fall Delay"]
  #[inline] pub fn set_dbfall_delay(mut self, value: u32) -> Self {
     assert!((value & !0xfff) == 0);
     self.0 &= !(0xfff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for ChDbfall {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChDbfall {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dbfall_delay() != 0 { try!(write!(f, " dbfall_delay=0x{:x}", self.dbfall_delay()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM0 Fault Source 0"]
#[derive(PartialEq, Eq)]
pub struct ChFltsrc0(pub u32);
impl ChFltsrc0 {
#[doc="Fault0 Input"]
  #[inline] pub fn fault0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Fault0 Input"]
  #[inline] pub fn set_fault0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Fault1 Input"]
  #[inline] pub fn fault1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Fault1 Input"]
  #[inline] pub fn set_fault1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Fault2 Input"]
  #[inline] pub fn fault2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Fault2 Input"]
  #[inline] pub fn set_fault2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Fault3 Input"]
  #[inline] pub fn fault3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Fault3 Input"]
  #[inline] pub fn set_fault3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}
impl ::core::fmt::Display for ChFltsrc0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChFltsrc0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fault0() != 0 { try!(write!(f, " fault0"))}
      if self.fault1() != 0 { try!(write!(f, " fault1"))}
      if self.fault2() != 0 { try!(write!(f, " fault2"))}
      if self.fault3() != 0 { try!(write!(f, " fault3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM0 Fault Source 1"]
#[derive(PartialEq, Eq)]
pub struct ChFltsrc1(pub u32);
impl ChFltsrc1 {
#[doc="Digital Comparator 0"]
  #[inline] pub fn dcmp0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Digital Comparator 0"]
  #[inline] pub fn set_dcmp0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Digital Comparator 1"]
  #[inline] pub fn dcmp1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Digital Comparator 1"]
  #[inline] pub fn set_dcmp1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Digital Comparator 2"]
  #[inline] pub fn dcmp2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Digital Comparator 2"]
  #[inline] pub fn set_dcmp2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Digital Comparator 3"]
  #[inline] pub fn dcmp3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Digital Comparator 3"]
  #[inline] pub fn set_dcmp3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Digital Comparator 4"]
  #[inline] pub fn dcmp4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Digital Comparator 4"]
  #[inline] pub fn set_dcmp4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Digital Comparator 5"]
  #[inline] pub fn dcmp5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Digital Comparator 5"]
  #[inline] pub fn set_dcmp5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Digital Comparator 6"]
  #[inline] pub fn dcmp6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Digital Comparator 6"]
  #[inline] pub fn set_dcmp6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Digital Comparator 7"]
  #[inline] pub fn dcmp7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Digital Comparator 7"]
  #[inline] pub fn set_dcmp7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for ChFltsrc1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChFltsrc1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dcmp0() != 0 { try!(write!(f, " dcmp0"))}
      if self.dcmp1() != 0 { try!(write!(f, " dcmp1"))}
      if self.dcmp2() != 0 { try!(write!(f, " dcmp2"))}
      if self.dcmp3() != 0 { try!(write!(f, " dcmp3"))}
      if self.dcmp4() != 0 { try!(write!(f, " dcmp4"))}
      if self.dcmp5() != 0 { try!(write!(f, " dcmp5"))}
      if self.dcmp6() != 0 { try!(write!(f, " dcmp6"))}
      if self.dcmp7() != 0 { try!(write!(f, " dcmp7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM0 Minimum Fault Period"]
#[derive(PartialEq, Eq)]
pub struct ChMinfltper(pub u32);
impl ChMinfltper {
#[doc="Minimum Fault Period"]
  #[inline] pub fn minfltper(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Minimum Fault Period"]
  #[inline] pub fn set_minfltper(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for ChMinfltper {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChMinfltper {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.minfltper() != 0 { try!(write!(f, " minfltper=0x{:x}", self.minfltper()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM0 Fault Pin Logic Sense"]
#[derive(PartialEq, Eq)]
pub struct ChFltsen(pub u32);
impl ChFltsen {
#[doc="Fault0 Sense"]
  #[inline] pub fn fault0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Fault0 Sense"]
  #[inline] pub fn set_fault0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Fault1 Sense"]
  #[inline] pub fn fault1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Fault1 Sense"]
  #[inline] pub fn set_fault1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Fault2 Sense"]
  #[inline] pub fn fault2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Fault2 Sense"]
  #[inline] pub fn set_fault2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Fault3 Sense"]
  #[inline] pub fn fault3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Fault3 Sense"]
  #[inline] pub fn set_fault3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}
impl ::core::fmt::Display for ChFltsen {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChFltsen {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fault0() != 0 { try!(write!(f, " fault0"))}
      if self.fault1() != 0 { try!(write!(f, " fault1"))}
      if self.fault2() != 0 { try!(write!(f, " fault2"))}
      if self.fault3() != 0 { try!(write!(f, " fault3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM0 Fault Status 0"]
#[derive(PartialEq, Eq)]
pub struct ChFltstat0(pub u32);
impl ChFltstat0 {
#[doc="Fault Input 0"]
  #[inline] pub fn fault0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Fault Input 0"]
  #[inline] pub fn set_fault0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Fault Input 1"]
  #[inline] pub fn fault1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Fault Input 1"]
  #[inline] pub fn set_fault1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Fault Input 2"]
  #[inline] pub fn fault2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Fault Input 2"]
  #[inline] pub fn set_fault2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Fault Input 3"]
  #[inline] pub fn fault3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Fault Input 3"]
  #[inline] pub fn set_fault3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

}
impl ::core::fmt::Display for ChFltstat0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChFltstat0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fault0() != 0 { try!(write!(f, " fault0"))}
      if self.fault1() != 0 { try!(write!(f, " fault1"))}
      if self.fault2() != 0 { try!(write!(f, " fault2"))}
      if self.fault3() != 0 { try!(write!(f, " fault3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM0 Fault Status 1"]
#[derive(PartialEq, Eq)]
pub struct ChFltstat1(pub u32);
impl ChFltstat1 {
#[doc="Digital Comparator 0 Trigger"]
  #[inline] pub fn dcmp0(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Digital Comparator 0 Trigger"]
  #[inline] pub fn set_dcmp0(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Digital Comparator 1 Trigger"]
  #[inline] pub fn dcmp1(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Digital Comparator 1 Trigger"]
  #[inline] pub fn set_dcmp1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Digital Comparator 2 Trigger"]
  #[inline] pub fn dcmp2(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Digital Comparator 2 Trigger"]
  #[inline] pub fn set_dcmp2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Digital Comparator 3 Trigger"]
  #[inline] pub fn dcmp3(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Digital Comparator 3 Trigger"]
  #[inline] pub fn set_dcmp3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Digital Comparator 4 Trigger"]
  #[inline] pub fn dcmp4(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Digital Comparator 4 Trigger"]
  #[inline] pub fn set_dcmp4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Digital Comparator 5 Trigger"]
  #[inline] pub fn dcmp5(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Digital Comparator 5 Trigger"]
  #[inline] pub fn set_dcmp5(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Digital Comparator 6 Trigger"]
  #[inline] pub fn dcmp6(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Digital Comparator 6 Trigger"]
  #[inline] pub fn set_dcmp6(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Digital Comparator 7 Trigger"]
  #[inline] pub fn dcmp7(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Digital Comparator 7 Trigger"]
  #[inline] pub fn set_dcmp7(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for ChFltstat1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChFltstat1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dcmp0() != 0 { try!(write!(f, " dcmp0"))}
      if self.dcmp1() != 0 { try!(write!(f, " dcmp1"))}
      if self.dcmp2() != 0 { try!(write!(f, " dcmp2"))}
      if self.dcmp3() != 0 { try!(write!(f, " dcmp3"))}
      if self.dcmp4() != 0 { try!(write!(f, " dcmp4"))}
      if self.dcmp5() != 0 { try!(write!(f, " dcmp5"))}
      if self.dcmp6() != 0 { try!(write!(f, " dcmp6"))}
      if self.dcmp7() != 0 { try!(write!(f, " dcmp7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Peripheral Properties"]
#[derive(PartialEq, Eq)]
pub struct Pp(pub u32);
impl Pp {
#[doc="Generators"]
  #[inline] pub fn pwm_pp_gcnt(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
#[doc="Generators"]
  #[inline] pub fn set_pwm_pp_gcnt(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Fault Inputs (per PWM unit)"]
  #[inline] pub fn pwm_pp_fcnt(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0xf // [7:4]
  }
#[doc="Fault Inputs (per PWM unit)"]
  #[inline] pub fn set_pwm_pp_fcnt(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Extended Synchronization"]
  #[inline] pub fn pwm_pp_esync(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Extended Synchronization"]
  #[inline] pub fn set_pwm_pp_esync(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Extended Fault"]
  #[inline] pub fn pwm_pp_efault(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Extended Fault"]
  #[inline] pub fn set_pwm_pp_efault(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="One-Shot Mode"]
  #[inline] pub fn pwm_pp_one(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="One-Shot Mode"]
  #[inline] pub fn set_pwm_pp_one(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
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
      if self.pwm_pp_gcnt() != 0 { try!(write!(f, " pwm_pp_gcnt=0x{:x}", self.pwm_pp_gcnt()))}
      if self.pwm_pp_fcnt() != 0 { try!(write!(f, " pwm_pp_fcnt=0x{:x}", self.pwm_pp_fcnt()))}
      if self.pwm_pp_esync() != 0 { try!(write!(f, " pwm_pp_esync"))}
      if self.pwm_pp_efault() != 0 { try!(write!(f, " pwm_pp_efault"))}
      if self.pwm_pp_one() != 0 { try!(write!(f, " pwm_pp_one"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Clock Configuration"]
#[derive(PartialEq, Eq)]
pub struct Cc(pub u32);
impl Cc {
#[doc="PWM Clock Divider"]
  #[inline] pub fn pwm_cc_pwmdiv(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
#[doc="PWM Clock Divider"]
  #[inline] pub fn set_pwm_cc_pwmdiv(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Use PWM Clock Divisor"]
  #[inline] pub fn pwm_cc_usepwm(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Use PWM Clock Divisor"]
  #[inline] pub fn set_pwm_cc_usepwm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
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
      if self.pwm_cc_pwmdiv() != 0 { try!(write!(f, " pwm_cc_pwmdiv=0x{:x}", self.pwm_cc_pwmdiv()))}
      if self.pwm_cc_usepwm() != 0 { try!(write!(f, " pwm_cc_usepwm"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(Clone, Copy, PartialEq)]
#[doc="PWM Channel"]
pub struct Channel<P, T> { pub periph: Periph<T>, pub index: usize, pub id: P }

impl<P,T> Channel<P,T> {
   #[inline] pub fn periph(&self) -> &Periph<T> { &self.periph }
   #[inline] pub fn index(&self) -> usize { self.index }
}

pub const PWM0_CH0: Channel<Pwm0Ch0Id, Pwm0Id> = Channel { periph: PWM0, index: 0, id: Pwm0Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pwm0Ch0Id {}
pub type Pwm0Ch0 = Channel<Pwm0Ch0Id, Pwm0Id>;

pub const PWM0_CH1: Channel<Pwm0Ch1Id, Pwm0Id> = Channel { periph: PWM0, index: 1, id: Pwm0Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pwm0Ch1Id {}
pub type Pwm0Ch1 = Channel<Pwm0Ch1Id, Pwm0Id>;

pub const PWM0_CH2: Channel<Pwm0Ch2Id, Pwm0Id> = Channel { periph: PWM0, index: 2, id: Pwm0Ch2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pwm0Ch2Id {}
pub type Pwm0Ch2 = Channel<Pwm0Ch2Id, Pwm0Id>;

pub const PWM0_CH3: Channel<Pwm0Ch3Id, Pwm0Id> = Channel { periph: PWM0, index: 3, id: Pwm0Ch3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Pwm0Ch3Id {}
pub type Pwm0Ch3 = Channel<Pwm0Ch3Id, Pwm0Id>;

