#[allow(unused_imports)] use bobbin_common::*;

periph!( ADC0, Adc0, _ADC0, AdcPeriph, 0x40038000);
periph!( ADC1, Adc1, _ADC1, AdcPeriph, 0x40039000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="ADC Peripheral"]
pub struct AdcPeriph(pub usize); 

impl super::sig::Signal<super::sig::Ain0> for Adc0Ch0 {}
impl super::sig::SignalAin<super::sig::Ain0> for Adc0Ch0 {}
impl super::sig::Signal<super::sig::Ain1> for Adc0Ch1 {}
impl super::sig::SignalAin<super::sig::Ain1> for Adc0Ch1 {}
impl super::sig::Signal<super::sig::Ain2> for Adc0Ch2 {}
impl super::sig::SignalAin<super::sig::Ain2> for Adc0Ch2 {}
impl super::sig::Signal<super::sig::Ain3> for Adc0Ch3 {}
impl super::sig::SignalAin<super::sig::Ain3> for Adc0Ch3 {}
impl super::sig::Signal<super::sig::Ain4> for Adc0Ch4 {}
impl super::sig::SignalAin<super::sig::Ain4> for Adc0Ch4 {}
impl super::sig::Signal<super::sig::Ain5> for Adc0Ch5 {}
impl super::sig::SignalAin<super::sig::Ain5> for Adc0Ch5 {}
impl super::sig::Signal<super::sig::Ain6> for Adc0Ch6 {}
impl super::sig::SignalAin<super::sig::Ain6> for Adc0Ch6 {}
impl super::sig::Signal<super::sig::Ain7> for Adc0Ch7 {}
impl super::sig::SignalAin<super::sig::Ain7> for Adc0Ch7 {}
impl super::sig::Signal<super::sig::Ain8> for Adc0Ch8 {}
impl super::sig::SignalAin<super::sig::Ain8> for Adc0Ch8 {}
impl super::sig::Signal<super::sig::Ain9> for Adc0Ch9 {}
impl super::sig::SignalAin<super::sig::Ain9> for Adc0Ch9 {}
impl super::sig::Signal<super::sig::Ain10> for Adc0Ch10 {}
impl super::sig::SignalAin<super::sig::Ain10> for Adc0Ch10 {}
impl super::sig::Signal<super::sig::Ain11> for Adc0Ch11 {}
impl super::sig::SignalAin<super::sig::Ain11> for Adc0Ch11 {}
impl super::sig::Signal<super::sig::Ain12> for Adc0Ch12 {}
impl super::sig::SignalAin<super::sig::Ain12> for Adc0Ch12 {}
impl super::sig::Signal<super::sig::Ain13> for Adc0Ch13 {}
impl super::sig::SignalAin<super::sig::Ain13> for Adc0Ch13 {}
impl super::sig::Signal<super::sig::Ain14> for Adc0Ch14 {}
impl super::sig::SignalAin<super::sig::Ain14> for Adc0Ch14 {}
impl super::sig::Signal<super::sig::Ain15> for Adc0Ch15 {}
impl super::sig::SignalAin<super::sig::Ain15> for Adc0Ch15 {}
impl super::sig::Signal<super::sig::Ain16> for Adc0Ch16 {}
impl super::sig::SignalAin<super::sig::Ain16> for Adc0Ch16 {}
impl super::sig::Signal<super::sig::Ain17> for Adc0Ch17 {}
impl super::sig::SignalAin<super::sig::Ain17> for Adc0Ch17 {}
impl super::sig::Signal<super::sig::Ain18> for Adc0Ch18 {}
impl super::sig::SignalAin<super::sig::Ain18> for Adc0Ch18 {}
impl super::sig::Signal<super::sig::Ain19> for Adc0Ch19 {}
impl super::sig::SignalAin<super::sig::Ain19> for Adc0Ch19 {}

impl super::sig::Signal<super::sig::Ain0> for Adc1Ch0 {}
impl super::sig::SignalAin<super::sig::Ain0> for Adc1Ch0 {}
impl super::sig::Signal<super::sig::Ain1> for Adc1Ch1 {}
impl super::sig::SignalAin<super::sig::Ain1> for Adc1Ch1 {}
impl super::sig::Signal<super::sig::Ain2> for Adc1Ch2 {}
impl super::sig::SignalAin<super::sig::Ain2> for Adc1Ch2 {}
impl super::sig::Signal<super::sig::Ain3> for Adc1Ch3 {}
impl super::sig::SignalAin<super::sig::Ain3> for Adc1Ch3 {}
impl super::sig::Signal<super::sig::Ain4> for Adc1Ch4 {}
impl super::sig::SignalAin<super::sig::Ain4> for Adc1Ch4 {}
impl super::sig::Signal<super::sig::Ain5> for Adc1Ch5 {}
impl super::sig::SignalAin<super::sig::Ain5> for Adc1Ch5 {}
impl super::sig::Signal<super::sig::Ain6> for Adc1Ch6 {}
impl super::sig::SignalAin<super::sig::Ain6> for Adc1Ch6 {}
impl super::sig::Signal<super::sig::Ain7> for Adc1Ch7 {}
impl super::sig::SignalAin<super::sig::Ain7> for Adc1Ch7 {}
impl super::sig::Signal<super::sig::Ain8> for Adc1Ch8 {}
impl super::sig::SignalAin<super::sig::Ain8> for Adc1Ch8 {}
impl super::sig::Signal<super::sig::Ain9> for Adc1Ch9 {}
impl super::sig::SignalAin<super::sig::Ain9> for Adc1Ch9 {}
impl super::sig::Signal<super::sig::Ain10> for Adc1Ch10 {}
impl super::sig::SignalAin<super::sig::Ain10> for Adc1Ch10 {}
impl super::sig::Signal<super::sig::Ain11> for Adc1Ch11 {}
impl super::sig::SignalAin<super::sig::Ain11> for Adc1Ch11 {}
impl super::sig::Signal<super::sig::Ain12> for Adc1Ch12 {}
impl super::sig::SignalAin<super::sig::Ain12> for Adc1Ch12 {}
impl super::sig::Signal<super::sig::Ain13> for Adc1Ch13 {}
impl super::sig::SignalAin<super::sig::Ain13> for Adc1Ch13 {}
impl super::sig::Signal<super::sig::Ain14> for Adc1Ch14 {}
impl super::sig::SignalAin<super::sig::Ain14> for Adc1Ch14 {}
impl super::sig::Signal<super::sig::Ain15> for Adc1Ch15 {}
impl super::sig::SignalAin<super::sig::Ain15> for Adc1Ch15 {}
impl super::sig::Signal<super::sig::Ain16> for Adc1Ch16 {}
impl super::sig::SignalAin<super::sig::Ain16> for Adc1Ch16 {}
impl super::sig::Signal<super::sig::Ain17> for Adc1Ch17 {}
impl super::sig::SignalAin<super::sig::Ain17> for Adc1Ch17 {}
impl super::sig::Signal<super::sig::Ain18> for Adc1Ch18 {}
impl super::sig::SignalAin<super::sig::Ain18> for Adc1Ch18 {}
impl super::sig::Signal<super::sig::Ain19> for Adc1Ch19 {}
impl super::sig::SignalAin<super::sig::Ain19> for Adc1Ch19 {}


impl AdcPeriph {
    #[doc="Get the *mut pointer for the ACTSS register."]
    #[inline] pub fn actss_mut(&self) -> *mut Actss { 
        (self.0 + 0x0) as *mut Actss
    }

    #[doc="Get the *const pointer for the ACTSS register."]
    #[inline] pub fn actss_ptr(&self) -> *const Actss { 
           self.actss_mut()
    }

    #[doc="Read the ACTSS register."]
    #[inline] pub fn actss(&self) -> Actss { 
        unsafe {
            read_volatile(self.actss_ptr())
        }
    }

    #[doc="Write the ACTSS register."]
    #[inline] pub fn set_actss<F: FnOnce(Actss) -> Actss>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.actss_mut(), f(Actss(0)));
        }
        self
    }

    #[doc="Modify the ACTSS register."]
    #[inline] pub fn with_actss<F: FnOnce(Actss) -> Actss>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.actss_mut(), f(self.actss()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RIS register."]
    #[inline] pub fn ris_mut(&self) -> *mut Ris { 
        (self.0 + 0x4) as *mut Ris
    }

    #[doc="Get the *const pointer for the RIS register."]
    #[inline] pub fn ris_ptr(&self) -> *const Ris { 
           self.ris_mut()
    }

    #[doc="Read the RIS register."]
    #[inline] pub fn ris(&self) -> Ris { 
        unsafe {
            read_volatile(self.ris_ptr())
        }
    }

    #[doc="Write the RIS register."]
    #[inline] pub fn set_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ris_mut(), f(Ris(0)));
        }
        self
    }

    #[doc="Modify the RIS register."]
    #[inline] pub fn with_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ris_mut(), f(self.ris()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IM register."]
    #[inline] pub fn im_mut(&self) -> *mut Im { 
        (self.0 + 0x8) as *mut Im
    }

    #[doc="Get the *const pointer for the IM register."]
    #[inline] pub fn im_ptr(&self) -> *const Im { 
           self.im_mut()
    }

    #[doc="Read the IM register."]
    #[inline] pub fn im(&self) -> Im { 
        unsafe {
            read_volatile(self.im_ptr())
        }
    }

    #[doc="Write the IM register."]
    #[inline] pub fn set_im<F: FnOnce(Im) -> Im>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.im_mut(), f(Im(0)));
        }
        self
    }

    #[doc="Modify the IM register."]
    #[inline] pub fn with_im<F: FnOnce(Im) -> Im>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.im_mut(), f(self.im()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ISC register."]
    #[inline] pub fn isc_mut(&self) -> *mut Isc { 
        (self.0 + 0xc) as *mut Isc
    }

    #[doc="Get the *const pointer for the ISC register."]
    #[inline] pub fn isc_ptr(&self) -> *const Isc { 
           self.isc_mut()
    }

    #[doc="Read the ISC register."]
    #[inline] pub fn isc(&self) -> Isc { 
        unsafe {
            read_volatile(self.isc_ptr())
        }
    }

    #[doc="Write the ISC register."]
    #[inline] pub fn set_isc<F: FnOnce(Isc) -> Isc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.isc_mut(), f(Isc(0)));
        }
        self
    }

    #[doc="Modify the ISC register."]
    #[inline] pub fn with_isc<F: FnOnce(Isc) -> Isc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.isc_mut(), f(self.isc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the OSTAT register."]
    #[inline] pub fn ostat_mut(&self) -> *mut Ostat { 
        (self.0 + 0x10) as *mut Ostat
    }

    #[doc="Get the *const pointer for the OSTAT register."]
    #[inline] pub fn ostat_ptr(&self) -> *const Ostat { 
           self.ostat_mut()
    }

    #[doc="Read the OSTAT register."]
    #[inline] pub fn ostat(&self) -> Ostat { 
        unsafe {
            read_volatile(self.ostat_ptr())
        }
    }

    #[doc="Write the OSTAT register."]
    #[inline] pub fn set_ostat<F: FnOnce(Ostat) -> Ostat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ostat_mut(), f(Ostat(0)));
        }
        self
    }

    #[doc="Modify the OSTAT register."]
    #[inline] pub fn with_ostat<F: FnOnce(Ostat) -> Ostat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ostat_mut(), f(self.ostat()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EMUX register."]
    #[inline] pub fn emux_mut(&self) -> *mut Emux { 
        (self.0 + 0x14) as *mut Emux
    }

    #[doc="Get the *const pointer for the EMUX register."]
    #[inline] pub fn emux_ptr(&self) -> *const Emux { 
           self.emux_mut()
    }

    #[doc="Read the EMUX register."]
    #[inline] pub fn emux(&self) -> Emux { 
        unsafe {
            read_volatile(self.emux_ptr())
        }
    }

    #[doc="Write the EMUX register."]
    #[inline] pub fn set_emux<F: FnOnce(Emux) -> Emux>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.emux_mut(), f(Emux(0)));
        }
        self
    }

    #[doc="Modify the EMUX register."]
    #[inline] pub fn with_emux<F: FnOnce(Emux) -> Emux>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.emux_mut(), f(self.emux()));
        }
        self
    }

    #[doc="Get the *mut pointer for the USTAT register."]
    #[inline] pub fn ustat_mut(&self) -> *mut Ustat { 
        (self.0 + 0x18) as *mut Ustat
    }

    #[doc="Get the *const pointer for the USTAT register."]
    #[inline] pub fn ustat_ptr(&self) -> *const Ustat { 
           self.ustat_mut()
    }

    #[doc="Read the USTAT register."]
    #[inline] pub fn ustat(&self) -> Ustat { 
        unsafe {
            read_volatile(self.ustat_ptr())
        }
    }

    #[doc="Write the USTAT register."]
    #[inline] pub fn set_ustat<F: FnOnce(Ustat) -> Ustat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ustat_mut(), f(Ustat(0)));
        }
        self
    }

    #[doc="Modify the USTAT register."]
    #[inline] pub fn with_ustat<F: FnOnce(Ustat) -> Ustat>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ustat_mut(), f(self.ustat()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TSSEL register."]
    #[inline] pub fn tssel_mut(&self) -> *mut Tssel { 
        (self.0 + 0x1c) as *mut Tssel
    }

    #[doc="Get the *const pointer for the TSSEL register."]
    #[inline] pub fn tssel_ptr(&self) -> *const Tssel { 
           self.tssel_mut()
    }

    #[doc="Read the TSSEL register."]
    #[inline] pub fn tssel(&self) -> Tssel { 
        unsafe {
            read_volatile(self.tssel_ptr())
        }
    }

    #[doc="Write the TSSEL register."]
    #[inline] pub fn set_tssel<F: FnOnce(Tssel) -> Tssel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tssel_mut(), f(Tssel(0)));
        }
        self
    }

    #[doc="Modify the TSSEL register."]
    #[inline] pub fn with_tssel<F: FnOnce(Tssel) -> Tssel>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tssel_mut(), f(self.tssel()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SSPRI register."]
    #[inline] pub fn sspri_mut(&self) -> *mut Sspri { 
        (self.0 + 0x20) as *mut Sspri
    }

    #[doc="Get the *const pointer for the SSPRI register."]
    #[inline] pub fn sspri_ptr(&self) -> *const Sspri { 
           self.sspri_mut()
    }

    #[doc="Read the SSPRI register."]
    #[inline] pub fn sspri(&self) -> Sspri { 
        unsafe {
            read_volatile(self.sspri_ptr())
        }
    }

    #[doc="Write the SSPRI register."]
    #[inline] pub fn set_sspri<F: FnOnce(Sspri) -> Sspri>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sspri_mut(), f(Sspri(0)));
        }
        self
    }

    #[doc="Modify the SSPRI register."]
    #[inline] pub fn with_sspri<F: FnOnce(Sspri) -> Sspri>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sspri_mut(), f(self.sspri()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SPC register."]
    #[inline] pub fn spc_mut(&self) -> *mut Spc { 
        (self.0 + 0x24) as *mut Spc
    }

    #[doc="Get the *const pointer for the SPC register."]
    #[inline] pub fn spc_ptr(&self) -> *const Spc { 
           self.spc_mut()
    }

    #[doc="Read the SPC register."]
    #[inline] pub fn spc(&self) -> Spc { 
        unsafe {
            read_volatile(self.spc_ptr())
        }
    }

    #[doc="Write the SPC register."]
    #[inline] pub fn set_spc<F: FnOnce(Spc) -> Spc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.spc_mut(), f(Spc(0)));
        }
        self
    }

    #[doc="Modify the SPC register."]
    #[inline] pub fn with_spc<F: FnOnce(Spc) -> Spc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.spc_mut(), f(self.spc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PSSI register."]
    #[inline] pub fn pssi_mut(&self) -> *mut Pssi { 
        (self.0 + 0x28) as *mut Pssi
    }

    #[doc="Get the *const pointer for the PSSI register."]
    #[inline] pub fn pssi_ptr(&self) -> *const Pssi { 
           self.pssi_mut()
    }

    #[doc="Read the PSSI register."]
    #[inline] pub fn pssi(&self) -> Pssi { 
        unsafe {
            read_volatile(self.pssi_ptr())
        }
    }

    #[doc="Write the PSSI register."]
    #[inline] pub fn set_pssi<F: FnOnce(Pssi) -> Pssi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pssi_mut(), f(Pssi(0)));
        }
        self
    }

    #[doc="Modify the PSSI register."]
    #[inline] pub fn with_pssi<F: FnOnce(Pssi) -> Pssi>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pssi_mut(), f(self.pssi()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SAC register."]
    #[inline] pub fn sac_mut(&self) -> *mut Sac { 
        (self.0 + 0x30) as *mut Sac
    }

    #[doc="Get the *const pointer for the SAC register."]
    #[inline] pub fn sac_ptr(&self) -> *const Sac { 
           self.sac_mut()
    }

    #[doc="Read the SAC register."]
    #[inline] pub fn sac(&self) -> Sac { 
        unsafe {
            read_volatile(self.sac_ptr())
        }
    }

    #[doc="Write the SAC register."]
    #[inline] pub fn set_sac<F: FnOnce(Sac) -> Sac>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sac_mut(), f(Sac(0)));
        }
        self
    }

    #[doc="Modify the SAC register."]
    #[inline] pub fn with_sac<F: FnOnce(Sac) -> Sac>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sac_mut(), f(self.sac()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DCISC register."]
    #[inline] pub fn dcisc_mut(&self) -> *mut Dcisc { 
        (self.0 + 0x34) as *mut Dcisc
    }

    #[doc="Get the *const pointer for the DCISC register."]
    #[inline] pub fn dcisc_ptr(&self) -> *const Dcisc { 
           self.dcisc_mut()
    }

    #[doc="Read the DCISC register."]
    #[inline] pub fn dcisc(&self) -> Dcisc { 
        unsafe {
            read_volatile(self.dcisc_ptr())
        }
    }

    #[doc="Write the DCISC register."]
    #[inline] pub fn set_dcisc<F: FnOnce(Dcisc) -> Dcisc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dcisc_mut(), f(Dcisc(0)));
        }
        self
    }

    #[doc="Modify the DCISC register."]
    #[inline] pub fn with_dcisc<F: FnOnce(Dcisc) -> Dcisc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dcisc_mut(), f(self.dcisc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTL register."]
    #[inline] pub fn ctl_mut(&self) -> *mut Ctl { 
        (self.0 + 0x38) as *mut Ctl
    }

    #[doc="Get the *const pointer for the CTL register."]
    #[inline] pub fn ctl_ptr(&self) -> *const Ctl { 
           self.ctl_mut()
    }

    #[doc="Read the CTL register."]
    #[inline] pub fn ctl(&self) -> Ctl { 
        unsafe {
            read_volatile(self.ctl_ptr())
        }
    }

    #[doc="Write the CTL register."]
    #[inline] pub fn set_ctl<F: FnOnce(Ctl) -> Ctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctl_mut(), f(Ctl(0)));
        }
        self
    }

    #[doc="Modify the CTL register."]
    #[inline] pub fn with_ctl<F: FnOnce(Ctl) -> Ctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctl_mut(), f(self.ctl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SSMUX register."]
    #[inline] pub fn ssmux_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Ssmux { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x40 + (index * 32)) as *mut Ssmux
    }

    #[doc="Get the *const pointer for the SSMUX register."]
    #[inline] pub fn ssmux_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Ssmux { 
           self.ssmux_mut(index)
    }

    #[doc="Read the SSMUX register."]
    #[inline] pub fn ssmux<I: Into<bits::R4>>(&self, index: I) -> Ssmux { 
        unsafe {
            read_volatile(self.ssmux_ptr(index))
        }
    }

    #[doc="Write the SSMUX register."]
    #[inline] pub fn set_ssmux<I: Into<bits::R4>, F: FnOnce(Ssmux) -> Ssmux>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ssmux_mut(index), f(Ssmux(0)));
        }
        self
    }

    #[doc="Modify the SSMUX register."]
    #[inline] pub fn with_ssmux<I: Into<bits::R4> + Copy, F: FnOnce(Ssmux) -> Ssmux>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ssmux_mut(index), f(self.ssmux(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the SSCTL register."]
    #[inline] pub fn ssctl_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Ssctl { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x44 + (index * 32)) as *mut Ssctl
    }

    #[doc="Get the *const pointer for the SSCTL register."]
    #[inline] pub fn ssctl_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Ssctl { 
           self.ssctl_mut(index)
    }

    #[doc="Read the SSCTL register."]
    #[inline] pub fn ssctl<I: Into<bits::R4>>(&self, index: I) -> Ssctl { 
        unsafe {
            read_volatile(self.ssctl_ptr(index))
        }
    }

    #[doc="Write the SSCTL register."]
    #[inline] pub fn set_ssctl<I: Into<bits::R4>, F: FnOnce(Ssctl) -> Ssctl>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ssctl_mut(index), f(Ssctl(0)));
        }
        self
    }

    #[doc="Modify the SSCTL register."]
    #[inline] pub fn with_ssctl<I: Into<bits::R4> + Copy, F: FnOnce(Ssctl) -> Ssctl>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ssctl_mut(index), f(self.ssctl(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the SSFIFO register."]
    #[inline] pub fn ssfifo_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Ssfifo { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x48 + (index * 32)) as *mut Ssfifo
    }

    #[doc="Get the *const pointer for the SSFIFO register."]
    #[inline] pub fn ssfifo_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Ssfifo { 
           self.ssfifo_mut(index)
    }

    #[doc="Read the SSFIFO register."]
    #[inline] pub fn ssfifo<I: Into<bits::R4>>(&self, index: I) -> Ssfifo { 
        unsafe {
            read_volatile(self.ssfifo_ptr(index))
        }
    }

    #[doc="Write the SSFIFO register."]
    #[inline] pub fn set_ssfifo<I: Into<bits::R4>, F: FnOnce(Ssfifo) -> Ssfifo>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ssfifo_mut(index), f(Ssfifo(0)));
        }
        self
    }

    #[doc="Modify the SSFIFO register."]
    #[inline] pub fn with_ssfifo<I: Into<bits::R4> + Copy, F: FnOnce(Ssfifo) -> Ssfifo>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ssfifo_mut(index), f(self.ssfifo(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the SSFSTAT register."]
    #[inline] pub fn ssfstat_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Ssfstat { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x4c + (index * 32)) as *mut Ssfstat
    }

    #[doc="Get the *const pointer for the SSFSTAT register."]
    #[inline] pub fn ssfstat_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Ssfstat { 
           self.ssfstat_mut(index)
    }

    #[doc="Read the SSFSTAT register."]
    #[inline] pub fn ssfstat<I: Into<bits::R4>>(&self, index: I) -> Ssfstat { 
        unsafe {
            read_volatile(self.ssfstat_ptr(index))
        }
    }

    #[doc="Write the SSFSTAT register."]
    #[inline] pub fn set_ssfstat<I: Into<bits::R4>, F: FnOnce(Ssfstat) -> Ssfstat>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ssfstat_mut(index), f(Ssfstat(0)));
        }
        self
    }

    #[doc="Modify the SSFSTAT register."]
    #[inline] pub fn with_ssfstat<I: Into<bits::R4> + Copy, F: FnOnce(Ssfstat) -> Ssfstat>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ssfstat_mut(index), f(self.ssfstat(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the SSOP register."]
    #[inline] pub fn ssop_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Ssop { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x50 + (index * 32)) as *mut Ssop
    }

    #[doc="Get the *const pointer for the SSOP register."]
    #[inline] pub fn ssop_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Ssop { 
           self.ssop_mut(index)
    }

    #[doc="Read the SSOP register."]
    #[inline] pub fn ssop<I: Into<bits::R4>>(&self, index: I) -> Ssop { 
        unsafe {
            read_volatile(self.ssop_ptr(index))
        }
    }

    #[doc="Write the SSOP register."]
    #[inline] pub fn set_ssop<I: Into<bits::R4>, F: FnOnce(Ssop) -> Ssop>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ssop_mut(index), f(Ssop(0)));
        }
        self
    }

    #[doc="Modify the SSOP register."]
    #[inline] pub fn with_ssop<I: Into<bits::R4> + Copy, F: FnOnce(Ssop) -> Ssop>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ssop_mut(index), f(self.ssop(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the SSDC register."]
    #[inline] pub fn ssdc_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Ssdc { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x54 + (index * 32)) as *mut Ssdc
    }

    #[doc="Get the *const pointer for the SSDC register."]
    #[inline] pub fn ssdc_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Ssdc { 
           self.ssdc_mut(index)
    }

    #[doc="Read the SSDC register."]
    #[inline] pub fn ssdc<I: Into<bits::R4>>(&self, index: I) -> Ssdc { 
        unsafe {
            read_volatile(self.ssdc_ptr(index))
        }
    }

    #[doc="Write the SSDC register."]
    #[inline] pub fn set_ssdc<I: Into<bits::R4>, F: FnOnce(Ssdc) -> Ssdc>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ssdc_mut(index), f(Ssdc(0)));
        }
        self
    }

    #[doc="Modify the SSDC register."]
    #[inline] pub fn with_ssdc<I: Into<bits::R4> + Copy, F: FnOnce(Ssdc) -> Ssdc>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ssdc_mut(index), f(self.ssdc(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the SSEMUX register."]
    #[inline] pub fn ssemux_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Ssemux { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x58 + (index * 32)) as *mut Ssemux
    }

    #[doc="Get the *const pointer for the SSEMUX register."]
    #[inline] pub fn ssemux_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Ssemux { 
           self.ssemux_mut(index)
    }

    #[doc="Read the SSEMUX register."]
    #[inline] pub fn ssemux<I: Into<bits::R4>>(&self, index: I) -> Ssemux { 
        unsafe {
            read_volatile(self.ssemux_ptr(index))
        }
    }

    #[doc="Write the SSEMUX register."]
    #[inline] pub fn set_ssemux<I: Into<bits::R4>, F: FnOnce(Ssemux) -> Ssemux>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ssemux_mut(index), f(Ssemux(0)));
        }
        self
    }

    #[doc="Modify the SSEMUX register."]
    #[inline] pub fn with_ssemux<I: Into<bits::R4> + Copy, F: FnOnce(Ssemux) -> Ssemux>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.ssemux_mut(index), f(self.ssemux(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the SSTSH register."]
    #[inline] pub fn sstsh_mut<I: Into<bits::R4>>(&self, index: I) -> *mut Sstsh { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x5c + (index * 32)) as *mut Sstsh
    }

    #[doc="Get the *const pointer for the SSTSH register."]
    #[inline] pub fn sstsh_ptr<I: Into<bits::R4>>(&self, index: I) -> *const Sstsh { 
           self.sstsh_mut(index)
    }

    #[doc="Read the SSTSH register."]
    #[inline] pub fn sstsh<I: Into<bits::R4>>(&self, index: I) -> Sstsh { 
        unsafe {
            read_volatile(self.sstsh_ptr(index))
        }
    }

    #[doc="Write the SSTSH register."]
    #[inline] pub fn set_sstsh<I: Into<bits::R4>, F: FnOnce(Sstsh) -> Sstsh>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.sstsh_mut(index), f(Sstsh(0)));
        }
        self
    }

    #[doc="Modify the SSTSH register."]
    #[inline] pub fn with_sstsh<I: Into<bits::R4> + Copy, F: FnOnce(Sstsh) -> Sstsh>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.sstsh_mut(index), f(self.sstsh(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the DCRIC register."]
    #[inline] pub fn dcric_mut(&self) -> *mut Dcric { 
        (self.0 + 0xd00) as *mut Dcric
    }

    #[doc="Get the *const pointer for the DCRIC register."]
    #[inline] pub fn dcric_ptr(&self) -> *const Dcric { 
           self.dcric_mut()
    }

    #[doc="Write the DCRIC register."]
    #[inline] pub fn set_dcric<F: FnOnce(Dcric) -> Dcric>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dcric_mut(), f(Dcric(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the DCCTL register."]
    #[inline] pub fn dcctl_mut<I: Into<bits::R8>>(&self, index: I) -> *mut Dcctl { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0xe00 + (index << 2)) as *mut Dcctl
    }

    #[doc="Get the *const pointer for the DCCTL register."]
    #[inline] pub fn dcctl_ptr<I: Into<bits::R8>>(&self, index: I) -> *const Dcctl { 
           self.dcctl_mut(index)
    }

    #[doc="Read the DCCTL register."]
    #[inline] pub fn dcctl<I: Into<bits::R8>>(&self, index: I) -> Dcctl { 
        unsafe {
            read_volatile(self.dcctl_ptr(index))
        }
    }

    #[doc="Write the DCCTL register."]
    #[inline] pub fn set_dcctl<I: Into<bits::R8>, F: FnOnce(Dcctl) -> Dcctl>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.dcctl_mut(index), f(Dcctl(0)));
        }
        self
    }

    #[doc="Modify the DCCTL register."]
    #[inline] pub fn with_dcctl<I: Into<bits::R8> + Copy, F: FnOnce(Dcctl) -> Dcctl>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.dcctl_mut(index), f(self.dcctl(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the DCCMP register."]
    #[inline] pub fn dccmp_mut<I: Into<bits::R8>>(&self, index: I) -> *mut Dccmp { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0xe40 + (index << 2)) as *mut Dccmp
    }

    #[doc="Get the *const pointer for the DCCMP register."]
    #[inline] pub fn dccmp_ptr<I: Into<bits::R8>>(&self, index: I) -> *const Dccmp { 
           self.dccmp_mut(index)
    }

    #[doc="Read the DCCMP register."]
    #[inline] pub fn dccmp<I: Into<bits::R8>>(&self, index: I) -> Dccmp { 
        unsafe {
            read_volatile(self.dccmp_ptr(index))
        }
    }

    #[doc="Write the DCCMP register."]
    #[inline] pub fn set_dccmp<I: Into<bits::R8>, F: FnOnce(Dccmp) -> Dccmp>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.dccmp_mut(index), f(Dccmp(0)));
        }
        self
    }

    #[doc="Modify the DCCMP register."]
    #[inline] pub fn with_dccmp<I: Into<bits::R8> + Copy, F: FnOnce(Dccmp) -> Dccmp>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.dccmp_mut(index), f(self.dccmp(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the PP register."]
    #[inline] pub fn pp_mut(&self) -> *mut Pp { 
        (self.0 + 0xfc0) as *mut Pp
    }

    #[doc="Get the *const pointer for the PP register."]
    #[inline] pub fn pp_ptr(&self) -> *const Pp { 
           self.pp_mut()
    }

    #[doc="Read the PP register."]
    #[inline] pub fn pp(&self) -> Pp { 
        unsafe {
            read_volatile(self.pp_ptr())
        }
    }

    #[doc="Write the PP register."]
    #[inline] pub fn set_pp<F: FnOnce(Pp) -> Pp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pp_mut(), f(Pp(0)));
        }
        self
    }

    #[doc="Modify the PP register."]
    #[inline] pub fn with_pp<F: FnOnce(Pp) -> Pp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pp_mut(), f(self.pp()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PC register."]
    #[inline] pub fn pc_mut(&self) -> *mut Pc { 
        (self.0 + 0xfc4) as *mut Pc
    }

    #[doc="Get the *const pointer for the PC register."]
    #[inline] pub fn pc_ptr(&self) -> *const Pc { 
           self.pc_mut()
    }

    #[doc="Read the PC register."]
    #[inline] pub fn pc(&self) -> Pc { 
        unsafe {
            read_volatile(self.pc_ptr())
        }
    }

    #[doc="Write the PC register."]
    #[inline] pub fn set_pc<F: FnOnce(Pc) -> Pc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pc_mut(), f(Pc(0)));
        }
        self
    }

    #[doc="Modify the PC register."]
    #[inline] pub fn with_pc<F: FnOnce(Pc) -> Pc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pc_mut(), f(self.pc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CC register."]
    #[inline] pub fn cc_mut(&self) -> *mut Cc { 
        (self.0 + 0xfc8) as *mut Cc
    }

    #[doc="Get the *const pointer for the CC register."]
    #[inline] pub fn cc_ptr(&self) -> *const Cc { 
           self.cc_mut()
    }

    #[doc="Read the CC register."]
    #[inline] pub fn cc(&self) -> Cc { 
        unsafe {
            read_volatile(self.cc_ptr())
        }
    }

    #[doc="Write the CC register."]
    #[inline] pub fn set_cc<F: FnOnce(Cc) -> Cc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cc_mut(), f(Cc(0)));
        }
        self
    }

    #[doc="Modify the CC register."]
    #[inline] pub fn with_cc<F: FnOnce(Cc) -> Cc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cc_mut(), f(self.cc()));
        }
        self
    }

}

#[doc="ADC Active Sample Sequencer"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Actss(pub u32);
impl Actss {
    #[doc="ADC SS Enable"]
    #[inline] pub fn asen<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ASEN != 0"]
    #[inline] pub fn test_asen<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.asen(index) != 0
    }

    #[doc="Sets the ASEN field."]
    #[inline] pub fn set_asen<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="ADC SS DMA Enable"]
    #[inline] pub fn aden<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 8 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ADEN != 0"]
    #[inline] pub fn test_aden<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.aden(index) != 0
    }

    #[doc="Sets the ADEN field."]
    #[inline] pub fn set_aden<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 8 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="ADC Busy"]
    #[inline] pub fn busy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if BUSY != 0"]
    #[inline] pub fn test_busy(&self) -> bool {
        self.busy() != 0
    }

    #[doc="Sets the BUSY field."]
    #[inline] pub fn set_busy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Actss {
    #[inline]
    fn from(other: u32) -> Self {
         Actss(other)
    }
}

impl ::core::fmt::Display for Actss {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Actss {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.asen(0) != 0 { try!(write!(f, " asen[0]"))}
        if self.asen(1) != 0 { try!(write!(f, " asen[1]"))}
        if self.asen(2) != 0 { try!(write!(f, " asen[2]"))}
        if self.asen(3) != 0 { try!(write!(f, " asen[3]"))}
        if self.aden(0) != 0 { try!(write!(f, " aden[0]"))}
        if self.aden(1) != 0 { try!(write!(f, " aden[1]"))}
        if self.aden(2) != 0 { try!(write!(f, " aden[2]"))}
        if self.aden(3) != 0 { try!(write!(f, " aden[3]"))}
        if self.busy() != 0 { try!(write!(f, " busy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Raw Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ris(pub u32);
impl Ris {
    #[doc="SS Raw Interrupt Status"]
    #[inline] pub fn inr<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if INR != 0"]
    #[inline] pub fn test_inr<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.inr(index) != 0
    }

    #[doc="Sets the INR field."]
    #[inline] pub fn set_inr<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="SS DMA Raw Interrupt Status"]
    #[inline] pub fn dmainr<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 8 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DMAINR != 0"]
    #[inline] pub fn test_dmainr<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.dmainr(index) != 0
    }

    #[doc="Sets the DMAINR field."]
    #[inline] pub fn set_dmainr<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 8 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Digital Comparator Raw Interrupt Status"]
    #[inline] pub fn inrdc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if INRDC != 0"]
    #[inline] pub fn test_inrdc(&self) -> bool {
        self.inrdc() != 0
    }

    #[doc="Sets the INRDC field."]
    #[inline] pub fn set_inrdc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Ris {
    #[inline]
    fn from(other: u32) -> Self {
         Ris(other)
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
        if self.inr(0) != 0 { try!(write!(f, " inr[0]"))}
        if self.inr(1) != 0 { try!(write!(f, " inr[1]"))}
        if self.inr(2) != 0 { try!(write!(f, " inr[2]"))}
        if self.inr(3) != 0 { try!(write!(f, " inr[3]"))}
        if self.dmainr(0) != 0 { try!(write!(f, " dmainr[0]"))}
        if self.dmainr(1) != 0 { try!(write!(f, " dmainr[1]"))}
        if self.dmainr(2) != 0 { try!(write!(f, " dmainr[2]"))}
        if self.dmainr(3) != 0 { try!(write!(f, " dmainr[3]"))}
        if self.inrdc() != 0 { try!(write!(f, " inrdc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Interrupt Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Im(pub u32);
impl Im {
    #[doc="SS Interrupt Mask"]
    #[inline] pub fn mask<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MASK != 0"]
    #[inline] pub fn test_mask<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.mask(index) != 0
    }

    #[doc="Sets the MASK field."]
    #[inline] pub fn set_mask<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="SS DMA Interrupt Mask"]
    #[inline] pub fn dmamask<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 8 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DMAMASK != 0"]
    #[inline] pub fn test_dmamask<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.dmamask(index) != 0
    }

    #[doc="Sets the DMAMASK field."]
    #[inline] pub fn set_dmamask<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 8 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Digital Comparator Interrupt on SS"]
    #[inline] pub fn dconss<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 16 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DCONSS != 0"]
    #[inline] pub fn test_dconss<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.dconss(index) != 0
    }

    #[doc="Sets the DCONSS field."]
    #[inline] pub fn set_dconss<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 16 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Im {
    #[inline]
    fn from(other: u32) -> Self {
         Im(other)
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
        if self.mask(0) != 0 { try!(write!(f, " mask[0]"))}
        if self.mask(1) != 0 { try!(write!(f, " mask[1]"))}
        if self.mask(2) != 0 { try!(write!(f, " mask[2]"))}
        if self.mask(3) != 0 { try!(write!(f, " mask[3]"))}
        if self.dmamask(0) != 0 { try!(write!(f, " dmamask[0]"))}
        if self.dmamask(1) != 0 { try!(write!(f, " dmamask[1]"))}
        if self.dmamask(2) != 0 { try!(write!(f, " dmamask[2]"))}
        if self.dmamask(3) != 0 { try!(write!(f, " dmamask[3]"))}
        if self.dconss(0) != 0 { try!(write!(f, " dconss[0]"))}
        if self.dconss(1) != 0 { try!(write!(f, " dconss[1]"))}
        if self.dconss(2) != 0 { try!(write!(f, " dconss[2]"))}
        if self.dconss(3) != 0 { try!(write!(f, " dconss[3]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Interrupt Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Isc(pub u32);
impl Isc {
    #[doc="SS Interrupt Status and Clear"]
    #[inline] pub fn _in<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IN != 0"]
    #[inline] pub fn test_in<I: Into<bits::R4>>(&self, index: I) -> bool{
        self._in(index) != 0
    }

    #[doc="Sets the IN field."]
    #[inline] pub fn set_in<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="SS DMA Interrupt Status and Clear"]
    #[inline] pub fn dmain<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 8 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DMAIN != 0"]
    #[inline] pub fn test_dmain<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.dmain(index) != 0
    }

    #[doc="Sets the DMAIN field."]
    #[inline] pub fn set_dmain<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 8 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Digital Comparator Interrupt Status on SS"]
    #[inline] pub fn dcinss<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 16 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DCINSS != 0"]
    #[inline] pub fn test_dcinss<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.dcinss(index) != 0
    }

    #[doc="Sets the DCINSS field."]
    #[inline] pub fn set_dcinss<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 16 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Isc {
    #[inline]
    fn from(other: u32) -> Self {
         Isc(other)
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
        if self._in(0) != 0 { try!(write!(f, " _in[0]"))}
        if self._in(1) != 0 { try!(write!(f, " _in[1]"))}
        if self._in(2) != 0 { try!(write!(f, " _in[2]"))}
        if self._in(3) != 0 { try!(write!(f, " _in[3]"))}
        if self.dmain(0) != 0 { try!(write!(f, " dmain[0]"))}
        if self.dmain(1) != 0 { try!(write!(f, " dmain[1]"))}
        if self.dmain(2) != 0 { try!(write!(f, " dmain[2]"))}
        if self.dmain(3) != 0 { try!(write!(f, " dmain[3]"))}
        if self.dcinss(0) != 0 { try!(write!(f, " dcinss[0]"))}
        if self.dcinss(1) != 0 { try!(write!(f, " dcinss[1]"))}
        if self.dcinss(2) != 0 { try!(write!(f, " dcinss[2]"))}
        if self.dcinss(3) != 0 { try!(write!(f, " dcinss[3]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Overflow Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ostat(pub u32);
impl Ostat {
    #[doc="SS FIFO Overflow"]
    #[inline] pub fn ov<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OV != 0"]
    #[inline] pub fn test_ov<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.ov(index) != 0
    }

    #[doc="Sets the OV field."]
    #[inline] pub fn set_ov<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Ostat {
    #[inline]
    fn from(other: u32) -> Self {
         Ostat(other)
    }
}

impl ::core::fmt::Display for Ostat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ostat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ov(0) != 0 { try!(write!(f, " ov[0]"))}
        if self.ov(1) != 0 { try!(write!(f, " ov[1]"))}
        if self.ov(2) != 0 { try!(write!(f, " ov[2]"))}
        if self.ov(3) != 0 { try!(write!(f, " ov[3]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Event Multiplexer Select"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Emux(pub u32);
impl Emux {
    #[doc="SS Trigger Select"]
    #[inline] pub fn em<I: Into<bits::R4>>(&self, index: I) -> bits::U4 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EM != 0"]
    #[inline] pub fn test_em<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.em(index) != 0
    }

    #[doc="Sets the EM field."]
    #[inline] pub fn set_em<I: Into<bits::R4>, V: Into<bits::U4>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 2);
        self.0 &= !(0xf << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Emux {
    #[inline]
    fn from(other: u32) -> Self {
         Emux(other)
    }
}

impl ::core::fmt::Display for Emux {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Emux {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.em(0) != 0 { try!(write!(f, " em[0]=0x{:x}", self.em(0)))}
        if self.em(1) != 0 { try!(write!(f, " em[1]=0x{:x}", self.em(1)))}
        if self.em(2) != 0 { try!(write!(f, " em[2]=0x{:x}", self.em(2)))}
        if self.em(3) != 0 { try!(write!(f, " em[3]=0x{:x}", self.em(3)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Underflow Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ustat(pub u32);
impl Ustat {
    #[doc="SS FIFO Underflow"]
    #[inline] pub fn uv<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if UV != 0"]
    #[inline] pub fn test_uv<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.uv(index) != 0
    }

    #[doc="Sets the UV field."]
    #[inline] pub fn set_uv<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Ustat {
    #[inline]
    fn from(other: u32) -> Self {
         Ustat(other)
    }
}

impl ::core::fmt::Display for Ustat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ustat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.uv(0) != 0 { try!(write!(f, " uv[0]"))}
        if self.uv(1) != 0 { try!(write!(f, " uv[1]"))}
        if self.uv(2) != 0 { try!(write!(f, " uv[2]"))}
        if self.uv(3) != 0 { try!(write!(f, " uv[3]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Trigger Source Select"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tssel(pub u32);
impl Tssel {
    #[doc="Generator PWM Module Trigger Select"]
    #[inline] pub fn ps<I: Into<bits::R4>>(&self, index: I) -> bits::U2 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 4 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if PS != 0"]
    #[inline] pub fn test_ps<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.ps(index) != 0
    }

    #[doc="Sets the PS field."]
    #[inline] pub fn set_ps<I: Into<bits::R4>, V: Into<bits::U2>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        let shift: usize = 4 + (index << 3);
        self.0 &= !(0x3 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Tssel {
    #[inline]
    fn from(other: u32) -> Self {
         Tssel(other)
    }
}

impl ::core::fmt::Display for Tssel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tssel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ps(0) != 0 { try!(write!(f, " ps[0]=0x{:x}", self.ps(0)))}
        if self.ps(1) != 0 { try!(write!(f, " ps[1]=0x{:x}", self.ps(1)))}
        if self.ps(2) != 0 { try!(write!(f, " ps[2]=0x{:x}", self.ps(2)))}
        if self.ps(3) != 0 { try!(write!(f, " ps[3]=0x{:x}", self.ps(3)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Sample Sequencer Priority"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sspri(pub u32);
impl Sspri {
    #[doc="SS Priority"]
    #[inline] pub fn ss<I: Into<bits::R4>>(&self, index: I) -> bits::U2 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SS != 0"]
    #[inline] pub fn test_ss<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.ss(index) != 0
    }

    #[doc="Sets the SS field."]
    #[inline] pub fn set_ss<I: Into<bits::R4>, V: Into<bits::U2>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 2);
        self.0 &= !(0x3 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Sspri {
    #[inline]
    fn from(other: u32) -> Self {
         Sspri(other)
    }
}

impl ::core::fmt::Display for Sspri {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sspri {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ss(0) != 0 { try!(write!(f, " ss[0]=0x{:x}", self.ss(0)))}
        if self.ss(1) != 0 { try!(write!(f, " ss[1]=0x{:x}", self.ss(1)))}
        if self.ss(2) != 0 { try!(write!(f, " ss[2]=0x{:x}", self.ss(2)))}
        if self.ss(3) != 0 { try!(write!(f, " ss[3]=0x{:x}", self.ss(3)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Sample Phase Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Spc(pub u32);
impl Spc {
    #[doc="Phase Difference"]
    #[inline] pub fn phase(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if PHASE != 0"]
    #[inline] pub fn test_phase(&self) -> bool {
        self.phase() != 0
    }

    #[doc="Sets the PHASE field."]
    #[inline] pub fn set_phase<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Spc {
    #[inline]
    fn from(other: u32) -> Self {
         Spc(other)
    }
}

impl ::core::fmt::Display for Spc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Spc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.phase() != 0 { try!(write!(f, " phase=0x{:x}", self.phase()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Processor Sample Sequence Initiate"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pssi(pub u32);
impl Pssi {
    #[doc="SS Initiate"]
    #[inline] pub fn ss<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SS != 0"]
    #[inline] pub fn test_ss<I: Into<bits::R4>>(&self, index: I) -> bool{
        self.ss(index) != 0
    }

    #[doc="Sets the SS field."]
    #[inline] pub fn set_ss<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Synchronize Wait"]
    #[inline] pub fn syncwait(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if SYNCWAIT != 0"]
    #[inline] pub fn test_syncwait(&self) -> bool {
        self.syncwait() != 0
    }

    #[doc="Sets the SYNCWAIT field."]
    #[inline] pub fn set_syncwait<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Global Synchronize"]
    #[inline] pub fn gsync(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if GSYNC != 0"]
    #[inline] pub fn test_gsync(&self) -> bool {
        self.gsync() != 0
    }

    #[doc="Sets the GSYNC field."]
    #[inline] pub fn set_gsync<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Pssi {
    #[inline]
    fn from(other: u32) -> Self {
         Pssi(other)
    }
}

impl ::core::fmt::Display for Pssi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pssi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ss(0) != 0 { try!(write!(f, " ss[0]"))}
        if self.ss(1) != 0 { try!(write!(f, " ss[1]"))}
        if self.ss(2) != 0 { try!(write!(f, " ss[2]"))}
        if self.ss(3) != 0 { try!(write!(f, " ss[3]"))}
        if self.syncwait() != 0 { try!(write!(f, " syncwait"))}
        if self.gsync() != 0 { try!(write!(f, " gsync"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Sample Averaging Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sac(pub u32);
impl Sac {
    #[doc="Hardware Averaging Control"]
    #[inline] pub fn avg(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if AVG != 0"]
    #[inline] pub fn test_avg(&self) -> bool {
        self.avg() != 0
    }

    #[doc="Sets the AVG field."]
    #[inline] pub fn set_avg<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sac {
    #[inline]
    fn from(other: u32) -> Self {
         Sac(other)
    }
}

impl ::core::fmt::Display for Sac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sac {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.avg() != 0 { try!(write!(f, " avg=0x{:x}", self.avg()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Digital Comparator Interrupt Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcisc(pub u32);
impl Dcisc {
    #[doc="Digital Comparator Interrupt Status and Clear"]
    #[inline] pub fn dcint<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DCINT != 0"]
    #[inline] pub fn test_dcint<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.dcint(index) != 0
    }

    #[doc="Sets the DCINT field."]
    #[inline] pub fn set_dcint<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Dcisc {
    #[inline]
    fn from(other: u32) -> Self {
         Dcisc(other)
    }
}

impl ::core::fmt::Display for Dcisc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcisc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dcint(0) != 0 { try!(write!(f, " dcint[0]"))}
        if self.dcint(1) != 0 { try!(write!(f, " dcint[1]"))}
        if self.dcint(2) != 0 { try!(write!(f, " dcint[2]"))}
        if self.dcint(3) != 0 { try!(write!(f, " dcint[3]"))}
        if self.dcint(4) != 0 { try!(write!(f, " dcint[4]"))}
        if self.dcint(5) != 0 { try!(write!(f, " dcint[5]"))}
        if self.dcint(6) != 0 { try!(write!(f, " dcint[6]"))}
        if self.dcint(7) != 0 { try!(write!(f, " dcint[7]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctl(pub u32);
impl Ctl {
    #[doc="Voltage Reference Select"]
    #[inline] pub fn vref(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if VREF != 0"]
    #[inline] pub fn test_vref(&self) -> bool {
        self.vref() != 0
    }

    #[doc="Sets the VREF field."]
    #[inline] pub fn set_vref<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Dither Mode Enable"]
    #[inline] pub fn dither(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DITHER != 0"]
    #[inline] pub fn test_dither(&self) -> bool {
        self.dither() != 0
    }

    #[doc="Sets the DITHER field."]
    #[inline] pub fn set_dither<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for Ctl {
    #[inline]
    fn from(other: u32) -> Self {
         Ctl(other)
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
        if self.vref() != 0 { try!(write!(f, " vref"))}
        if self.dither() != 0 { try!(write!(f, " dither"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Sample Sequence Input Multiplexer Select"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ssmux(pub u32);
impl Ssmux {
    #[doc="1st Sample Input Select"]
    #[inline] pub fn mux<I: Into<bits::R8>>(&self, index: I) -> bits::U4 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if MUX != 0"]
    #[inline] pub fn test_mux<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.mux(index) != 0
    }

    #[doc="Sets the MUX field."]
    #[inline] pub fn set_mux<I: Into<bits::R8>, V: Into<bits::U4>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 2);
        self.0 &= !(0xf << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Ssmux {
    #[inline]
    fn from(other: u32) -> Self {
         Ssmux(other)
    }
}

impl ::core::fmt::Display for Ssmux {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ssmux {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mux(0) != 0 { try!(write!(f, " mux[0]=0x{:x}", self.mux(0)))}
        if self.mux(1) != 0 { try!(write!(f, " mux[1]=0x{:x}", self.mux(1)))}
        if self.mux(2) != 0 { try!(write!(f, " mux[2]=0x{:x}", self.mux(2)))}
        if self.mux(3) != 0 { try!(write!(f, " mux[3]=0x{:x}", self.mux(3)))}
        if self.mux(4) != 0 { try!(write!(f, " mux[4]=0x{:x}", self.mux(4)))}
        if self.mux(5) != 0 { try!(write!(f, " mux[5]=0x{:x}", self.mux(5)))}
        if self.mux(6) != 0 { try!(write!(f, " mux[6]=0x{:x}", self.mux(6)))}
        if self.mux(7) != 0 { try!(write!(f, " mux[7]=0x{:x}", self.mux(7)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Sample Sequence Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ssctl(pub u32);
impl Ssctl {
    #[doc="Sample Differential Input Select"]
    #[inline] pub fn d<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if D != 0"]
    #[inline] pub fn test_d<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.d(index) != 0
    }

    #[doc="Sets the D field."]
    #[inline] pub fn set_d<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 2);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Sample is End of Sequence"]
    #[inline] pub fn end<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 1 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if END != 0"]
    #[inline] pub fn test_end<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.end(index) != 0
    }

    #[doc="Sets the END field."]
    #[inline] pub fn set_end<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 1 + (index << 2);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Sample Interrupt Enable"]
    #[inline] pub fn ie<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 2 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IE != 0"]
    #[inline] pub fn test_ie<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.ie(index) != 0
    }

    #[doc="Sets the IE field."]
    #[inline] pub fn set_ie<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 2 + (index << 2);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Sample Temp Sensor Select"]
    #[inline] pub fn ts<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 3 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TS != 0"]
    #[inline] pub fn test_ts<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.ts(index) != 0
    }

    #[doc="Sets the TS field."]
    #[inline] pub fn set_ts<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 3 + (index << 2);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Ssctl {
    #[inline]
    fn from(other: u32) -> Self {
         Ssctl(other)
    }
}

impl ::core::fmt::Display for Ssctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ssctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.d(0) != 0 { try!(write!(f, " d[0]"))}
        if self.d(1) != 0 { try!(write!(f, " d[1]"))}
        if self.d(2) != 0 { try!(write!(f, " d[2]"))}
        if self.d(3) != 0 { try!(write!(f, " d[3]"))}
        if self.d(4) != 0 { try!(write!(f, " d[4]"))}
        if self.d(5) != 0 { try!(write!(f, " d[5]"))}
        if self.d(6) != 0 { try!(write!(f, " d[6]"))}
        if self.d(7) != 0 { try!(write!(f, " d[7]"))}
        if self.end(0) != 0 { try!(write!(f, " end[0]"))}
        if self.end(1) != 0 { try!(write!(f, " end[1]"))}
        if self.end(2) != 0 { try!(write!(f, " end[2]"))}
        if self.end(3) != 0 { try!(write!(f, " end[3]"))}
        if self.end(4) != 0 { try!(write!(f, " end[4]"))}
        if self.end(5) != 0 { try!(write!(f, " end[5]"))}
        if self.end(6) != 0 { try!(write!(f, " end[6]"))}
        if self.end(7) != 0 { try!(write!(f, " end[7]"))}
        if self.ie(0) != 0 { try!(write!(f, " ie[0]"))}
        if self.ie(1) != 0 { try!(write!(f, " ie[1]"))}
        if self.ie(2) != 0 { try!(write!(f, " ie[2]"))}
        if self.ie(3) != 0 { try!(write!(f, " ie[3]"))}
        if self.ie(4) != 0 { try!(write!(f, " ie[4]"))}
        if self.ie(5) != 0 { try!(write!(f, " ie[5]"))}
        if self.ie(6) != 0 { try!(write!(f, " ie[6]"))}
        if self.ie(7) != 0 { try!(write!(f, " ie[7]"))}
        if self.ts(0) != 0 { try!(write!(f, " ts[0]"))}
        if self.ts(1) != 0 { try!(write!(f, " ts[1]"))}
        if self.ts(2) != 0 { try!(write!(f, " ts[2]"))}
        if self.ts(3) != 0 { try!(write!(f, " ts[3]"))}
        if self.ts(4) != 0 { try!(write!(f, " ts[4]"))}
        if self.ts(5) != 0 { try!(write!(f, " ts[5]"))}
        if self.ts(6) != 0 { try!(write!(f, " ts[6]"))}
        if self.ts(7) != 0 { try!(write!(f, " ts[7]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Sample Sequence Result FIFO"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ssfifo(pub u32);
impl Ssfifo {
    #[doc="Conversion Result Data"]
    #[inline] pub fn data(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ssfifo {
    #[inline]
    fn from(other: u32) -> Self {
         Ssfifo(other)
    }
}

impl ::core::fmt::Display for Ssfifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ssfifo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Sample Sequence FIFO Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ssfstat(pub u32);
impl Ssfstat {
    #[doc="FIFO Tail Pointer"]
    #[inline] pub fn tptr(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if TPTR != 0"]
    #[inline] pub fn test_tptr(&self) -> bool {
        self.tptr() != 0
    }

    #[doc="Sets the TPTR field."]
    #[inline] pub fn set_tptr<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="FIFO Head Pointer"]
    #[inline] pub fn hptr(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if HPTR != 0"]
    #[inline] pub fn test_hptr(&self) -> bool {
        self.hptr() != 0
    }

    #[doc="Sets the HPTR field."]
    #[inline] pub fn set_hptr<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FIFO Empty"]
    #[inline] pub fn empty(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if EMPTY != 0"]
    #[inline] pub fn test_empty(&self) -> bool {
        self.empty() != 0
    }

    #[doc="Sets the EMPTY field."]
    #[inline] pub fn set_empty<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="FIFO Full"]
    #[inline] pub fn full(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if FULL != 0"]
    #[inline] pub fn test_full(&self) -> bool {
        self.full() != 0
    }

    #[doc="Sets the FULL field."]
    #[inline] pub fn set_full<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u32> for Ssfstat {
    #[inline]
    fn from(other: u32) -> Self {
         Ssfstat(other)
    }
}

impl ::core::fmt::Display for Ssfstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ssfstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tptr() != 0 { try!(write!(f, " tptr=0x{:x}", self.tptr()))}
        if self.hptr() != 0 { try!(write!(f, " hptr=0x{:x}", self.hptr()))}
        if self.empty() != 0 { try!(write!(f, " empty"))}
        if self.full() != 0 { try!(write!(f, " full"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Sample Sequence Operation"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ssop(pub u32);
impl Ssop {
    #[doc="Sample Digital Comparator Operation"]
    #[inline] pub fn sdcop<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SDCOP != 0"]
    #[inline] pub fn test_sdcop<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.sdcop(index) != 0
    }

    #[doc="Sets the SDCOP field."]
    #[inline] pub fn set_sdcop<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 2);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Ssop {
    #[inline]
    fn from(other: u32) -> Self {
         Ssop(other)
    }
}

impl ::core::fmt::Display for Ssop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ssop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sdcop(0) != 0 { try!(write!(f, " sdcop[0]"))}
        if self.sdcop(1) != 0 { try!(write!(f, " sdcop[1]"))}
        if self.sdcop(2) != 0 { try!(write!(f, " sdcop[2]"))}
        if self.sdcop(3) != 0 { try!(write!(f, " sdcop[3]"))}
        if self.sdcop(4) != 0 { try!(write!(f, " sdcop[4]"))}
        if self.sdcop(5) != 0 { try!(write!(f, " sdcop[5]"))}
        if self.sdcop(6) != 0 { try!(write!(f, " sdcop[6]"))}
        if self.sdcop(7) != 0 { try!(write!(f, " sdcop[7]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Sample Sequence Digital Comparator Select"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ssdc(pub u32);
impl Ssdc {
    #[doc="Sample Digital Comparator Select"]
    #[inline] pub fn sdcsel<I: Into<bits::R8>>(&self, index: I) -> bits::U4 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if SDCSEL != 0"]
    #[inline] pub fn test_sdcsel<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.sdcsel(index) != 0
    }

    #[doc="Sets the SDCSEL field."]
    #[inline] pub fn set_sdcsel<I: Into<bits::R8>, V: Into<bits::U4>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 2);
        self.0 &= !(0xf << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Ssdc {
    #[inline]
    fn from(other: u32) -> Self {
         Ssdc(other)
    }
}

impl ::core::fmt::Display for Ssdc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ssdc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sdcsel(0) != 0 { try!(write!(f, " sdcsel[0]=0x{:x}", self.sdcsel(0)))}
        if self.sdcsel(1) != 0 { try!(write!(f, " sdcsel[1]=0x{:x}", self.sdcsel(1)))}
        if self.sdcsel(2) != 0 { try!(write!(f, " sdcsel[2]=0x{:x}", self.sdcsel(2)))}
        if self.sdcsel(3) != 0 { try!(write!(f, " sdcsel[3]=0x{:x}", self.sdcsel(3)))}
        if self.sdcsel(4) != 0 { try!(write!(f, " sdcsel[4]=0x{:x}", self.sdcsel(4)))}
        if self.sdcsel(5) != 0 { try!(write!(f, " sdcsel[5]=0x{:x}", self.sdcsel(5)))}
        if self.sdcsel(6) != 0 { try!(write!(f, " sdcsel[6]=0x{:x}", self.sdcsel(6)))}
        if self.sdcsel(7) != 0 { try!(write!(f, " sdcsel[7]=0x{:x}", self.sdcsel(7)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Sample Sequence Extended Input Multiplexer Select"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ssemux(pub u32);
impl Ssemux {
    #[doc="Sample Input Select (Upper Bit)"]
    #[inline] pub fn emux<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EMUX != 0"]
    #[inline] pub fn test_emux<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.emux(index) != 0
    }

    #[doc="Sets the EMUX field."]
    #[inline] pub fn set_emux<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 2);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Ssemux {
    #[inline]
    fn from(other: u32) -> Self {
         Ssemux(other)
    }
}

impl ::core::fmt::Display for Ssemux {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ssemux {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.emux(0) != 0 { try!(write!(f, " emux[0]"))}
        if self.emux(1) != 0 { try!(write!(f, " emux[1]"))}
        if self.emux(2) != 0 { try!(write!(f, " emux[2]"))}
        if self.emux(3) != 0 { try!(write!(f, " emux[3]"))}
        if self.emux(4) != 0 { try!(write!(f, " emux[4]"))}
        if self.emux(5) != 0 { try!(write!(f, " emux[5]"))}
        if self.emux(6) != 0 { try!(write!(f, " emux[6]"))}
        if self.emux(7) != 0 { try!(write!(f, " emux[7]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Sample Sequence Sample and Hold Time"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sstsh(pub u32);
impl Sstsh {
    #[doc="Sample and Hold Period Select"]
    #[inline] pub fn tsh<I: Into<bits::R8>>(&self, index: I) -> bits::U4 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if TSH != 0"]
    #[inline] pub fn test_tsh<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.tsh(index) != 0
    }

    #[doc="Sets the TSH field."]
    #[inline] pub fn set_tsh<I: Into<bits::R8>, V: Into<bits::U4>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 2);
        self.0 &= !(0xf << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Sstsh {
    #[inline]
    fn from(other: u32) -> Self {
         Sstsh(other)
    }
}

impl ::core::fmt::Display for Sstsh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sstsh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tsh(0) != 0 { try!(write!(f, " tsh[0]=0x{:x}", self.tsh(0)))}
        if self.tsh(1) != 0 { try!(write!(f, " tsh[1]=0x{:x}", self.tsh(1)))}
        if self.tsh(2) != 0 { try!(write!(f, " tsh[2]=0x{:x}", self.tsh(2)))}
        if self.tsh(3) != 0 { try!(write!(f, " tsh[3]=0x{:x}", self.tsh(3)))}
        if self.tsh(4) != 0 { try!(write!(f, " tsh[4]=0x{:x}", self.tsh(4)))}
        if self.tsh(5) != 0 { try!(write!(f, " tsh[5]=0x{:x}", self.tsh(5)))}
        if self.tsh(6) != 0 { try!(write!(f, " tsh[6]=0x{:x}", self.tsh(6)))}
        if self.tsh(7) != 0 { try!(write!(f, " tsh[7]=0x{:x}", self.tsh(7)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Digital Comparator Reset Initial Conditions"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcric(pub u32);
impl Dcric {
    #[doc="Digital Comparator Interrupt"]
    #[inline] pub fn dcint<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DCINT != 0"]
    #[inline] pub fn test_dcint<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.dcint(index) != 0
    }

    #[doc="Sets the DCINT field."]
    #[inline] pub fn set_dcint<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Digital Comparator Trigger"]
    #[inline] pub fn dctrig<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 16 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DCTRIG != 0"]
    #[inline] pub fn test_dctrig<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.dctrig(index) != 0
    }

    #[doc="Sets the DCTRIG field."]
    #[inline] pub fn set_dctrig<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 16 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Dcric {
    #[inline]
    fn from(other: u32) -> Self {
         Dcric(other)
    }
}

impl ::core::fmt::Display for Dcric {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcric {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dcint(0) != 0 { try!(write!(f, " dcint[0]"))}
        if self.dcint(1) != 0 { try!(write!(f, " dcint[1]"))}
        if self.dcint(2) != 0 { try!(write!(f, " dcint[2]"))}
        if self.dcint(3) != 0 { try!(write!(f, " dcint[3]"))}
        if self.dcint(4) != 0 { try!(write!(f, " dcint[4]"))}
        if self.dcint(5) != 0 { try!(write!(f, " dcint[5]"))}
        if self.dcint(6) != 0 { try!(write!(f, " dcint[6]"))}
        if self.dcint(7) != 0 { try!(write!(f, " dcint[7]"))}
        if self.dctrig(0) != 0 { try!(write!(f, " dctrig[0]"))}
        if self.dctrig(1) != 0 { try!(write!(f, " dctrig[1]"))}
        if self.dctrig(2) != 0 { try!(write!(f, " dctrig[2]"))}
        if self.dctrig(3) != 0 { try!(write!(f, " dctrig[3]"))}
        if self.dctrig(4) != 0 { try!(write!(f, " dctrig[4]"))}
        if self.dctrig(5) != 0 { try!(write!(f, " dctrig[5]"))}
        if self.dctrig(6) != 0 { try!(write!(f, " dctrig[6]"))}
        if self.dctrig(7) != 0 { try!(write!(f, " dctrig[7]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Digital Comparator Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcctl(pub u32);
impl Dcctl {
    #[doc="Comparison Interrupt Mode"]
    #[inline] pub fn cim(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if CIM != 0"]
    #[inline] pub fn test_cim(&self) -> bool {
        self.cim() != 0
    }

    #[doc="Sets the CIM field."]
    #[inline] pub fn set_cim<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Comparison Interrupt Condition"]
    #[inline] pub fn cic(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if CIC != 0"]
    #[inline] pub fn test_cic(&self) -> bool {
        self.cic() != 0
    }

    #[doc="Sets the CIC field."]
    #[inline] pub fn set_cic<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Comparison Interrupt Enable"]
    #[inline] pub fn cie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CIE != 0"]
    #[inline] pub fn test_cie(&self) -> bool {
        self.cie() != 0
    }

    #[doc="Sets the CIE field."]
    #[inline] pub fn set_cie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Comparison Trigger Mode"]
    #[inline] pub fn ctm(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if CTM != 0"]
    #[inline] pub fn test_ctm(&self) -> bool {
        self.ctm() != 0
    }

    #[doc="Sets the CTM field."]
    #[inline] pub fn set_ctm<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Comparison Trigger Condition"]
    #[inline] pub fn ctc(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if CTC != 0"]
    #[inline] pub fn test_ctc(&self) -> bool {
        self.ctc() != 0
    }

    #[doc="Sets the CTC field."]
    #[inline] pub fn set_ctc<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Comparison Trigger Enable"]
    #[inline] pub fn cte(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CTE != 0"]
    #[inline] pub fn test_cte(&self) -> bool {
        self.cte() != 0
    }

    #[doc="Sets the CTE field."]
    #[inline] pub fn set_cte<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u32> for Dcctl {
    #[inline]
    fn from(other: u32) -> Self {
         Dcctl(other)
    }
}

impl ::core::fmt::Display for Dcctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cim() != 0 { try!(write!(f, " cim=0x{:x}", self.cim()))}
        if self.cic() != 0 { try!(write!(f, " cic=0x{:x}", self.cic()))}
        if self.cie() != 0 { try!(write!(f, " cie"))}
        if self.ctm() != 0 { try!(write!(f, " ctm=0x{:x}", self.ctm()))}
        if self.ctc() != 0 { try!(write!(f, " ctc=0x{:x}", self.ctc()))}
        if self.cte() != 0 { try!(write!(f, " cte"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Digital Comparator Range"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dccmp(pub u32);
impl Dccmp {
    #[doc="Compare"]
    #[inline] pub fn comp<I: Into<bits::R2>>(&self, index: I) -> bits::U12 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index * 16);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if COMP != 0"]
    #[inline] pub fn test_comp<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.comp(index) != 0
    }

    #[doc="Sets the COMP field."]
    #[inline] pub fn set_comp<I: Into<bits::R2>, V: Into<bits::U12>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index * 16);
        self.0 &= !(0xfff << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Dccmp {
    #[inline]
    fn from(other: u32) -> Self {
         Dccmp(other)
    }
}

impl ::core::fmt::Display for Dccmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dccmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.comp(0) != 0 { try!(write!(f, " comp[0]=0x{:x}", self.comp(0)))}
        if self.comp(1) != 0 { try!(write!(f, " comp[1]=0x{:x}", self.comp(1)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Peripheral Properties"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pp(pub u32);
impl Pp {
    #[doc="Maximum Conversion Rate"]
    #[inline] pub fn mcr(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if MCR != 0"]
    #[inline] pub fn test_mcr(&self) -> bool {
        self.mcr() != 0
    }

    #[doc="Sets the MCR field."]
    #[inline] pub fn set_mcr<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="ADC Channel Count"]
    #[inline] pub fn ch(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3f) as u8) } // [9:4]
    }

    #[doc="Returns true if CH != 0"]
    #[inline] pub fn test_ch(&self) -> bool {
        self.ch() != 0
    }

    #[doc="Sets the CH field."]
    #[inline] pub fn set_ch<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Digital Comparator Count"]
    #[inline] pub fn dc(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3f) as u8) } // [15:10]
    }

    #[doc="Returns true if DC != 0"]
    #[inline] pub fn test_dc(&self) -> bool {
        self.dc() != 0
    }

    #[doc="Sets the DC field."]
    #[inline] pub fn set_dc<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="ADC Architecture"]
    #[inline] pub fn _type(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if TYPE != 0"]
    #[inline] pub fn test_type(&self) -> bool {
        self._type() != 0
    }

    #[doc="Sets the TYPE field."]
    #[inline] pub fn set_type<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Resolution"]
    #[inline] pub fn rsl(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1f) as u8) } // [22:18]
    }

    #[doc="Returns true if RSL != 0"]
    #[inline] pub fn test_rsl(&self) -> bool {
        self.rsl() != 0
    }

    #[doc="Sets the RSL field."]
    #[inline] pub fn set_rsl<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Temperature Sensor"]
    #[inline] pub fn ts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if TS != 0"]
    #[inline] pub fn test_ts(&self) -> bool {
        self.ts() != 0
    }

    #[doc="Sets the TS field."]
    #[inline] pub fn set_ts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Application-Programmable Sample-and-Hold Time"]
    #[inline] pub fn apsht(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if APSHT != 0"]
    #[inline] pub fn test_apsht(&self) -> bool {
        self.apsht() != 0
    }

    #[doc="Sets the APSHT field."]
    #[inline] pub fn set_apsht<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Pp {
    #[inline]
    fn from(other: u32) -> Self {
         Pp(other)
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
        if self.mcr() != 0 { try!(write!(f, " mcr=0x{:x}", self.mcr()))}
        if self.ch() != 0 { try!(write!(f, " ch=0x{:x}", self.ch()))}
        if self.dc() != 0 { try!(write!(f, " dc=0x{:x}", self.dc()))}
        if self._type() != 0 { try!(write!(f, " type=0x{:x}", self._type()))}
        if self.rsl() != 0 { try!(write!(f, " rsl=0x{:x}", self.rsl()))}
        if self.ts() != 0 { try!(write!(f, " ts"))}
        if self.apsht() != 0 { try!(write!(f, " apsht"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Peripheral Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pc(pub u32);
impl Pc {
    #[doc="Conversion Rate"]
    #[inline] pub fn mcr(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if MCR != 0"]
    #[inline] pub fn test_mcr(&self) -> bool {
        self.mcr() != 0
    }

    #[doc="Sets the MCR field."]
    #[inline] pub fn set_mcr<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pc {
    #[inline]
    fn from(other: u32) -> Self {
         Pc(other)
    }
}

impl ::core::fmt::Display for Pc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mcr() != 0 { try!(write!(f, " mcr=0x{:x}", self.mcr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ADC Clock Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cc(pub u32);
impl Cc {
    #[doc="ADC Clock Source"]
    #[inline] pub fn cs(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CS != 0"]
    #[inline] pub fn test_cs(&self) -> bool {
        self.cs() != 0
    }

    #[doc="Sets the CS field."]
    #[inline] pub fn set_cs<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="PLL VCO Clock Divisor"]
    #[inline] pub fn clkdiv(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3f) as u8) } // [9:4]
    }

    #[doc="Returns true if CLKDIV != 0"]
    #[inline] pub fn test_clkdiv(&self) -> bool {
        self.clkdiv() != 0
    }

    #[doc="Sets the CLKDIV field."]
    #[inline] pub fn set_clkdiv<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Cc {
    #[inline]
    fn from(other: u32) -> Self {
         Cc(other)
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
        if self.clkdiv() != 0 { try!(write!(f, " clkdiv=0x{:x}", self.clkdiv()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

pub struct AdcCh { pub periph: AdcPeriph, pub index: usize }
channel!(ADC0_CH0, Adc0Ch0, ADC0, Adc0, _ADC0_CH0, AdcCh, _ADC0, 0);
channel!(ADC0_CH1, Adc0Ch1, ADC0, Adc0, _ADC0_CH1, AdcCh, _ADC0, 1);
channel!(ADC0_CH2, Adc0Ch2, ADC0, Adc0, _ADC0_CH2, AdcCh, _ADC0, 2);
channel!(ADC0_CH3, Adc0Ch3, ADC0, Adc0, _ADC0_CH3, AdcCh, _ADC0, 3);
channel!(ADC0_CH4, Adc0Ch4, ADC0, Adc0, _ADC0_CH4, AdcCh, _ADC0, 4);
channel!(ADC0_CH5, Adc0Ch5, ADC0, Adc0, _ADC0_CH5, AdcCh, _ADC0, 5);
channel!(ADC0_CH6, Adc0Ch6, ADC0, Adc0, _ADC0_CH6, AdcCh, _ADC0, 6);
channel!(ADC0_CH7, Adc0Ch7, ADC0, Adc0, _ADC0_CH7, AdcCh, _ADC0, 7);
channel!(ADC0_CH8, Adc0Ch8, ADC0, Adc0, _ADC0_CH8, AdcCh, _ADC0, 8);
channel!(ADC0_CH9, Adc0Ch9, ADC0, Adc0, _ADC0_CH9, AdcCh, _ADC0, 9);
channel!(ADC0_CH10, Adc0Ch10, ADC0, Adc0, _ADC0_CH10, AdcCh, _ADC0, 10);
channel!(ADC0_CH11, Adc0Ch11, ADC0, Adc0, _ADC0_CH11, AdcCh, _ADC0, 11);
channel!(ADC0_CH12, Adc0Ch12, ADC0, Adc0, _ADC0_CH12, AdcCh, _ADC0, 12);
channel!(ADC0_CH13, Adc0Ch13, ADC0, Adc0, _ADC0_CH13, AdcCh, _ADC0, 13);
channel!(ADC0_CH14, Adc0Ch14, ADC0, Adc0, _ADC0_CH14, AdcCh, _ADC0, 14);
channel!(ADC0_CH15, Adc0Ch15, ADC0, Adc0, _ADC0_CH15, AdcCh, _ADC0, 15);
channel!(ADC0_CH16, Adc0Ch16, ADC0, Adc0, _ADC0_CH16, AdcCh, _ADC0, 16);
channel!(ADC0_CH17, Adc0Ch17, ADC0, Adc0, _ADC0_CH17, AdcCh, _ADC0, 17);
channel!(ADC0_CH18, Adc0Ch18, ADC0, Adc0, _ADC0_CH18, AdcCh, _ADC0, 18);
channel!(ADC0_CH19, Adc0Ch19, ADC0, Adc0, _ADC0_CH19, AdcCh, _ADC0, 19);
channel!(ADC1_CH0, Adc1Ch0, ADC1, Adc1, _ADC1_CH0, AdcCh, _ADC1, 0);
channel!(ADC1_CH1, Adc1Ch1, ADC1, Adc1, _ADC1_CH1, AdcCh, _ADC1, 1);
channel!(ADC1_CH2, Adc1Ch2, ADC1, Adc1, _ADC1_CH2, AdcCh, _ADC1, 2);
channel!(ADC1_CH3, Adc1Ch3, ADC1, Adc1, _ADC1_CH3, AdcCh, _ADC1, 3);
channel!(ADC1_CH4, Adc1Ch4, ADC1, Adc1, _ADC1_CH4, AdcCh, _ADC1, 4);
channel!(ADC1_CH5, Adc1Ch5, ADC1, Adc1, _ADC1_CH5, AdcCh, _ADC1, 5);
channel!(ADC1_CH6, Adc1Ch6, ADC1, Adc1, _ADC1_CH6, AdcCh, _ADC1, 6);
channel!(ADC1_CH7, Adc1Ch7, ADC1, Adc1, _ADC1_CH7, AdcCh, _ADC1, 7);
channel!(ADC1_CH8, Adc1Ch8, ADC1, Adc1, _ADC1_CH8, AdcCh, _ADC1, 8);
channel!(ADC1_CH9, Adc1Ch9, ADC1, Adc1, _ADC1_CH9, AdcCh, _ADC1, 9);
channel!(ADC1_CH10, Adc1Ch10, ADC1, Adc1, _ADC1_CH10, AdcCh, _ADC1, 10);
channel!(ADC1_CH11, Adc1Ch11, ADC1, Adc1, _ADC1_CH11, AdcCh, _ADC1, 11);
channel!(ADC1_CH12, Adc1Ch12, ADC1, Adc1, _ADC1_CH12, AdcCh, _ADC1, 12);
channel!(ADC1_CH13, Adc1Ch13, ADC1, Adc1, _ADC1_CH13, AdcCh, _ADC1, 13);
channel!(ADC1_CH14, Adc1Ch14, ADC1, Adc1, _ADC1_CH14, AdcCh, _ADC1, 14);
channel!(ADC1_CH15, Adc1Ch15, ADC1, Adc1, _ADC1_CH15, AdcCh, _ADC1, 15);
channel!(ADC1_CH16, Adc1Ch16, ADC1, Adc1, _ADC1_CH16, AdcCh, _ADC1, 16);
channel!(ADC1_CH17, Adc1Ch17, ADC1, Adc1, _ADC1_CH17, AdcCh, _ADC1, 17);
channel!(ADC1_CH18, Adc1Ch18, ADC1, Adc1, _ADC1_CH18, AdcCh, _ADC1, 18);
channel!(ADC1_CH19, Adc1Ch19, ADC1, Adc1, _ADC1_CH19, AdcCh, _ADC1, 19);
