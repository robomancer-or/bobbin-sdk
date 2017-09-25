#[allow(unused_imports)] use bobbin_common::*;

periph!( TIMER0, Timer0, _TIMER0, TimerPeriph, 0x40030000);
periph!( TIMER1, Timer1, _TIMER1, TimerPeriph, 0x40031000);
periph!( TIMER2, Timer2, _TIMER2, TimerPeriph, 0x40032000);
periph!( TIMER3, Timer3, _TIMER3, TimerPeriph, 0x40033000);
periph!( TIMER4, Timer4, _TIMER4, TimerPeriph, 0x40034000);
periph!( TIMER5, Timer5, _TIMER5, TimerPeriph, 0x40035000);
periph!( TIMER6, Timer6, _TIMER6, TimerPeriph, 0x400e0000);
periph!( TIMER7, Timer7, _TIMER7, TimerPeriph, 0x400e1000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="TIMER Peripheral"]
pub struct TimerPeriph(pub usize); 

impl super::sig::Signal<super::sig::T0ccp0> for Timer0a {}
impl super::sig::SignalCcp<super::sig::T0ccp0> for Timer0a {}
impl super::sig::Signal<super::sig::T0ccp1> for Timer0b {}
impl super::sig::SignalCcp<super::sig::T0ccp1> for Timer0b {}

impl super::sig::Signal<super::sig::T1ccp0> for Timer1a {}
impl super::sig::SignalCcp<super::sig::T1ccp0> for Timer1a {}
impl super::sig::Signal<super::sig::T1ccp1> for Timer1b {}
impl super::sig::SignalCcp<super::sig::T1ccp1> for Timer1b {}

impl super::sig::Signal<super::sig::T2ccp0> for Timer2a {}
impl super::sig::SignalCcp<super::sig::T2ccp0> for Timer2a {}
impl super::sig::Signal<super::sig::T2ccp1> for Timer2b {}
impl super::sig::SignalCcp<super::sig::T2ccp1> for Timer2b {}

impl super::sig::Signal<super::sig::T3ccp0> for Timer3a {}
impl super::sig::SignalCcp<super::sig::T3ccp0> for Timer3a {}
impl super::sig::Signal<super::sig::T3ccp1> for Timer3b {}
impl super::sig::SignalCcp<super::sig::T3ccp1> for Timer3b {}

impl super::sig::Signal<super::sig::T4ccp0> for Timer4a {}
impl super::sig::SignalCcp<super::sig::T4ccp0> for Timer4a {}
impl super::sig::Signal<super::sig::T4ccp1> for Timer4b {}
impl super::sig::SignalCcp<super::sig::T4ccp1> for Timer4b {}

impl super::sig::Signal<super::sig::T5ccp0> for Timer5a {}
impl super::sig::SignalCcp<super::sig::T5ccp0> for Timer5a {}
impl super::sig::Signal<super::sig::T5ccp1> for Timer5b {}
impl super::sig::SignalCcp<super::sig::T5ccp1> for Timer5b {}

impl super::sig::Signal<super::sig::T6ccp0> for Timer6a {}
impl super::sig::SignalCcp<super::sig::T6ccp0> for Timer6a {}
impl super::sig::Signal<super::sig::T6ccp1> for Timer6b {}
impl super::sig::SignalCcp<super::sig::T6ccp1> for Timer6b {}

impl super::sig::Signal<super::sig::T7ccp0> for Timer7a {}
impl super::sig::SignalCcp<super::sig::T7ccp0> for Timer7a {}
impl super::sig::Signal<super::sig::T7ccp1> for Timer7b {}
impl super::sig::SignalCcp<super::sig::T7ccp1> for Timer7b {}


impl TimerPeriph {
    #[doc="Get the *mut pointer for the CFG register."]
    #[inline] pub fn cfg_mut(&self) -> *mut Cfg { 
        (self.0 + 0x0) as *mut Cfg
    }

    #[doc="Get the *const pointer for the CFG register."]
    #[inline] pub fn cfg_ptr(&self) -> *const Cfg { 
           self.cfg_mut()
    }

    #[doc="Read the CFG register."]
    #[inline] pub fn cfg(&self) -> Cfg { 
        unsafe {
            read_volatile(self.cfg_ptr())
        }
    }

    #[doc="Write the CFG register."]
    #[inline] pub fn set_cfg<F: FnOnce(Cfg) -> Cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfg_mut(), f(Cfg(0)));
        }
        self
    }

    #[doc="Modify the CFG register."]
    #[inline] pub fn with_cfg<F: FnOnce(Cfg) -> Cfg>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfg_mut(), f(self.cfg()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TMR register."]
    #[inline] pub fn tmr_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Tmr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x4 + (index << 2)) as *mut Tmr
    }

    #[doc="Get the *const pointer for the TMR register."]
    #[inline] pub fn tmr_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Tmr { 
           self.tmr_mut(index)
    }

    #[doc="Read the TMR register."]
    #[inline] pub fn tmr<I: Into<bits::R2>>(&self, index: I) -> Tmr { 
        unsafe {
            read_volatile(self.tmr_ptr(index))
        }
    }

    #[doc="Write the TMR register."]
    #[inline] pub fn set_tmr<I: Into<bits::R2>, F: FnOnce(Tmr) -> Tmr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tmr_mut(index), f(Tmr(0)));
        }
        self
    }

    #[doc="Modify the TMR register."]
    #[inline] pub fn with_tmr<I: Into<bits::R2> + Copy, F: FnOnce(Tmr) -> Tmr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tmr_mut(index), f(self.tmr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTL register."]
    #[inline] pub fn ctl_mut(&self) -> *mut Ctl { 
        (self.0 + 0xc) as *mut Ctl
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

    #[doc="Get the *mut pointer for the SYNC register."]
    #[inline] pub fn sync_mut(&self) -> *mut Sync { 
        (self.0 + 0x10) as *mut Sync
    }

    #[doc="Get the *const pointer for the SYNC register."]
    #[inline] pub fn sync_ptr(&self) -> *const Sync { 
           self.sync_mut()
    }

    #[doc="Read the SYNC register."]
    #[inline] pub fn sync(&self) -> Sync { 
        unsafe {
            read_volatile(self.sync_ptr())
        }
    }

    #[doc="Write the SYNC register."]
    #[inline] pub fn set_sync<F: FnOnce(Sync) -> Sync>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sync_mut(), f(Sync(0)));
        }
        self
    }

    #[doc="Modify the SYNC register."]
    #[inline] pub fn with_sync<F: FnOnce(Sync) -> Sync>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sync_mut(), f(self.sync()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IMR register."]
    #[inline] pub fn imr_mut(&self) -> *mut Imr { 
        (self.0 + 0x18) as *mut Imr
    }

    #[doc="Get the *const pointer for the IMR register."]
    #[inline] pub fn imr_ptr(&self) -> *const Imr { 
           self.imr_mut()
    }

    #[doc="Read the IMR register."]
    #[inline] pub fn imr(&self) -> Imr { 
        unsafe {
            read_volatile(self.imr_ptr())
        }
    }

    #[doc="Write the IMR register."]
    #[inline] pub fn set_imr<F: FnOnce(Imr) -> Imr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.imr_mut(), f(Imr(0)));
        }
        self
    }

    #[doc="Modify the IMR register."]
    #[inline] pub fn with_imr<F: FnOnce(Imr) -> Imr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.imr_mut(), f(self.imr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RIS register."]
    #[inline] pub fn ris_mut(&self) -> *mut Ris { 
        (self.0 + 0x1c) as *mut Ris
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

    #[doc="Get the *mut pointer for the MIS register."]
    #[inline] pub fn mis_mut(&self) -> *mut Mis { 
        (self.0 + 0x20) as *mut Mis
    }

    #[doc="Get the *const pointer for the MIS register."]
    #[inline] pub fn mis_ptr(&self) -> *const Mis { 
           self.mis_mut()
    }

    #[doc="Read the MIS register."]
    #[inline] pub fn mis(&self) -> Mis { 
        unsafe {
            read_volatile(self.mis_ptr())
        }
    }

    #[doc="Write the MIS register."]
    #[inline] pub fn set_mis<F: FnOnce(Mis) -> Mis>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mis_mut(), f(Mis(0)));
        }
        self
    }

    #[doc="Modify the MIS register."]
    #[inline] pub fn with_mis<F: FnOnce(Mis) -> Mis>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mis_mut(), f(self.mis()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ICR register."]
    #[inline] pub fn icr_mut(&self) -> *mut Icr { 
        (self.0 + 0x24) as *mut Icr
    }

    #[doc="Get the *const pointer for the ICR register."]
    #[inline] pub fn icr_ptr(&self) -> *const Icr { 
           self.icr_mut()
    }

    #[doc="Write the ICR register."]
    #[inline] pub fn set_icr<F: FnOnce(Icr) -> Icr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icr_mut(), f(Icr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the TILR register."]
    #[inline] pub fn tilr_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Tilr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x28 + (index << 2)) as *mut Tilr
    }

    #[doc="Get the *const pointer for the TILR register."]
    #[inline] pub fn tilr_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Tilr { 
           self.tilr_mut(index)
    }

    #[doc="Read the TILR register."]
    #[inline] pub fn tilr<I: Into<bits::R2>>(&self, index: I) -> Tilr { 
        unsafe {
            read_volatile(self.tilr_ptr(index))
        }
    }

    #[doc="Write the TILR register."]
    #[inline] pub fn set_tilr<I: Into<bits::R2>, F: FnOnce(Tilr) -> Tilr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tilr_mut(index), f(Tilr(0)));
        }
        self
    }

    #[doc="Modify the TILR register."]
    #[inline] pub fn with_tilr<I: Into<bits::R2> + Copy, F: FnOnce(Tilr) -> Tilr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tilr_mut(index), f(self.tilr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the TMTCHR register."]
    #[inline] pub fn tmtchr_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Tmtchr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x30 + (index << 2)) as *mut Tmtchr
    }

    #[doc="Get the *const pointer for the TMTCHR register."]
    #[inline] pub fn tmtchr_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Tmtchr { 
           self.tmtchr_mut(index)
    }

    #[doc="Read the TMTCHR register."]
    #[inline] pub fn tmtchr<I: Into<bits::R2>>(&self, index: I) -> Tmtchr { 
        unsafe {
            read_volatile(self.tmtchr_ptr(index))
        }
    }

    #[doc="Write the TMTCHR register."]
    #[inline] pub fn set_tmtchr<I: Into<bits::R2>, F: FnOnce(Tmtchr) -> Tmtchr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tmtchr_mut(index), f(Tmtchr(0)));
        }
        self
    }

    #[doc="Modify the TMTCHR register."]
    #[inline] pub fn with_tmtchr<I: Into<bits::R2> + Copy, F: FnOnce(Tmtchr) -> Tmtchr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tmtchr_mut(index), f(self.tmtchr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the TPR register."]
    #[inline] pub fn tpr_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Tpr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x38 + (index << 2)) as *mut Tpr
    }

    #[doc="Get the *const pointer for the TPR register."]
    #[inline] pub fn tpr_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Tpr { 
           self.tpr_mut(index)
    }

    #[doc="Read the TPR register."]
    #[inline] pub fn tpr<I: Into<bits::R2>>(&self, index: I) -> Tpr { 
        unsafe {
            read_volatile(self.tpr_ptr(index))
        }
    }

    #[doc="Write the TPR register."]
    #[inline] pub fn set_tpr<I: Into<bits::R2>, F: FnOnce(Tpr) -> Tpr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tpr_mut(index), f(Tpr(0)));
        }
        self
    }

    #[doc="Modify the TPR register."]
    #[inline] pub fn with_tpr<I: Into<bits::R2> + Copy, F: FnOnce(Tpr) -> Tpr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tpr_mut(index), f(self.tpr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the TPMR register."]
    #[inline] pub fn tpmr_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Tpmr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x40 + (index << 2)) as *mut Tpmr
    }

    #[doc="Get the *const pointer for the TPMR register."]
    #[inline] pub fn tpmr_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Tpmr { 
           self.tpmr_mut(index)
    }

    #[doc="Read the TPMR register."]
    #[inline] pub fn tpmr<I: Into<bits::R2>>(&self, index: I) -> Tpmr { 
        unsafe {
            read_volatile(self.tpmr_ptr(index))
        }
    }

    #[doc="Write the TPMR register."]
    #[inline] pub fn set_tpmr<I: Into<bits::R2>, F: FnOnce(Tpmr) -> Tpmr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tpmr_mut(index), f(Tpmr(0)));
        }
        self
    }

    #[doc="Modify the TPMR register."]
    #[inline] pub fn with_tpmr<I: Into<bits::R2> + Copy, F: FnOnce(Tpmr) -> Tpmr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tpmr_mut(index), f(self.tpmr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the TR register."]
    #[inline] pub fn tr_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Tr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x48 + (index << 2)) as *mut Tr
    }

    #[doc="Get the *const pointer for the TR register."]
    #[inline] pub fn tr_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Tr { 
           self.tr_mut(index)
    }

    #[doc="Read the TR register."]
    #[inline] pub fn tr<I: Into<bits::R2>>(&self, index: I) -> Tr { 
        unsafe {
            read_volatile(self.tr_ptr(index))
        }
    }

    #[doc="Write the TR register."]
    #[inline] pub fn set_tr<I: Into<bits::R2>, F: FnOnce(Tr) -> Tr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tr_mut(index), f(Tr(0)));
        }
        self
    }

    #[doc="Modify the TR register."]
    #[inline] pub fn with_tr<I: Into<bits::R2> + Copy, F: FnOnce(Tr) -> Tr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tr_mut(index), f(self.tr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the TV register."]
    #[inline] pub fn tv_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Tv { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x50 + (index << 2)) as *mut Tv
    }

    #[doc="Get the *const pointer for the TV register."]
    #[inline] pub fn tv_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Tv { 
           self.tv_mut(index)
    }

    #[doc="Read the TV register."]
    #[inline] pub fn tv<I: Into<bits::R2>>(&self, index: I) -> Tv { 
        unsafe {
            read_volatile(self.tv_ptr(index))
        }
    }

    #[doc="Write the TV register."]
    #[inline] pub fn set_tv<I: Into<bits::R2>, F: FnOnce(Tv) -> Tv>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tv_mut(index), f(Tv(0)));
        }
        self
    }

    #[doc="Modify the TV register."]
    #[inline] pub fn with_tv<I: Into<bits::R2> + Copy, F: FnOnce(Tv) -> Tv>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tv_mut(index), f(self.tv(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the RTCPD register."]
    #[inline] pub fn rtcpd_mut(&self) -> *mut Rtcpd { 
        (self.0 + 0x58) as *mut Rtcpd
    }

    #[doc="Get the *const pointer for the RTCPD register."]
    #[inline] pub fn rtcpd_ptr(&self) -> *const Rtcpd { 
           self.rtcpd_mut()
    }

    #[doc="Read the RTCPD register."]
    #[inline] pub fn rtcpd(&self) -> Rtcpd { 
        unsafe {
            read_volatile(self.rtcpd_ptr())
        }
    }

    #[doc="Write the RTCPD register."]
    #[inline] pub fn set_rtcpd<F: FnOnce(Rtcpd) -> Rtcpd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rtcpd_mut(), f(Rtcpd(0)));
        }
        self
    }

    #[doc="Modify the RTCPD register."]
    #[inline] pub fn with_rtcpd<F: FnOnce(Rtcpd) -> Rtcpd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rtcpd_mut(), f(self.rtcpd()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TPS register."]
    #[inline] pub fn tps_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Tps { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x5c + (index << 2)) as *mut Tps
    }

    #[doc="Get the *const pointer for the TPS register."]
    #[inline] pub fn tps_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Tps { 
           self.tps_mut(index)
    }

    #[doc="Read the TPS register."]
    #[inline] pub fn tps<I: Into<bits::R2>>(&self, index: I) -> Tps { 
        unsafe {
            read_volatile(self.tps_ptr(index))
        }
    }

    #[doc="Write the TPS register."]
    #[inline] pub fn set_tps<I: Into<bits::R2>, F: FnOnce(Tps) -> Tps>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tps_mut(index), f(Tps(0)));
        }
        self
    }

    #[doc="Modify the TPS register."]
    #[inline] pub fn with_tps<I: Into<bits::R2> + Copy, F: FnOnce(Tps) -> Tps>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tps_mut(index), f(self.tps(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the DMAEV register."]
    #[inline] pub fn dmaev_mut(&self) -> *mut Dmaev { 
        (self.0 + 0x6c) as *mut Dmaev
    }

    #[doc="Get the *const pointer for the DMAEV register."]
    #[inline] pub fn dmaev_ptr(&self) -> *const Dmaev { 
           self.dmaev_mut()
    }

    #[doc="Read the DMAEV register."]
    #[inline] pub fn dmaev(&self) -> Dmaev { 
        unsafe {
            read_volatile(self.dmaev_ptr())
        }
    }

    #[doc="Write the DMAEV register."]
    #[inline] pub fn set_dmaev<F: FnOnce(Dmaev) -> Dmaev>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dmaev_mut(), f(Dmaev(0)));
        }
        self
    }

    #[doc="Modify the DMAEV register."]
    #[inline] pub fn with_dmaev<F: FnOnce(Dmaev) -> Dmaev>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dmaev_mut(), f(self.dmaev()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ADCEV register."]
    #[inline] pub fn adcev_mut(&self) -> *mut Adcev { 
        (self.0 + 0x70) as *mut Adcev
    }

    #[doc="Get the *const pointer for the ADCEV register."]
    #[inline] pub fn adcev_ptr(&self) -> *const Adcev { 
           self.adcev_mut()
    }

    #[doc="Read the ADCEV register."]
    #[inline] pub fn adcev(&self) -> Adcev { 
        unsafe {
            read_volatile(self.adcev_ptr())
        }
    }

    #[doc="Write the ADCEV register."]
    #[inline] pub fn set_adcev<F: FnOnce(Adcev) -> Adcev>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.adcev_mut(), f(Adcev(0)));
        }
        self
    }

    #[doc="Modify the ADCEV register."]
    #[inline] pub fn with_adcev<F: FnOnce(Adcev) -> Adcev>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.adcev_mut(), f(self.adcev()));
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

#[doc="GPTM Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc="GPTM Configuration"]
    #[inline] pub fn cfg(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if CFG != 0"]
    #[inline] pub fn test_cfg(&self) -> bool {
        self.cfg() != 0
    }

    #[doc="Sets the CFG field."]
    #[inline] pub fn set_cfg<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cfg {
    #[inline]
    fn from(other: u32) -> Self {
         Cfg(other)
    }
}

impl ::core::fmt::Display for Cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cfg() != 0 { try!(write!(f, " cfg=0x{:x}", self.cfg()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPTM Timer n Mode"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tmr(pub u32);
impl Tmr {
    #[doc="GPTM Timer n Mode"]
    #[inline] pub fn tmr(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if TMR != 0"]
    #[inline] pub fn test_tmr(&self) -> bool {
        self.tmr() != 0
    }

    #[doc="Sets the TMR field."]
    #[inline] pub fn set_tmr<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="GPTM Timer n Capture Mode"]
    #[inline] pub fn tcmr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TCMR != 0"]
    #[inline] pub fn test_tcmr(&self) -> bool {
        self.tcmr() != 0
    }

    #[doc="Sets the TCMR field."]
    #[inline] pub fn set_tcmr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="GPTM Timer n Alternate Mode Select"]
    #[inline] pub fn tams(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TAMS != 0"]
    #[inline] pub fn test_tams(&self) -> bool {
        self.tams() != 0
    }

    #[doc="Sets the TAMS field."]
    #[inline] pub fn set_tams<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPTM Timer n Count Direction"]
    #[inline] pub fn tcdir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TCDIR != 0"]
    #[inline] pub fn test_tcdir(&self) -> bool {
        self.tcdir() != 0
    }

    #[doc="Sets the TCDIR field."]
    #[inline] pub fn set_tcdir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPTM Timer n Match Interrupt Enable"]
    #[inline] pub fn tmie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TMIE != 0"]
    #[inline] pub fn test_tmie(&self) -> bool {
        self.tmie() != 0
    }

    #[doc="Sets the TMIE field."]
    #[inline] pub fn set_tmie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="GPTM Timer n Wait-on-Trigger"]
    #[inline] pub fn twot(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TWOT != 0"]
    #[inline] pub fn test_twot(&self) -> bool {
        self.twot() != 0
    }

    #[doc="Sets the TWOT field."]
    #[inline] pub fn set_twot<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="GPTM Timer n Snap-Shot Mode"]
    #[inline] pub fn tsnaps(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TSNAPS != 0"]
    #[inline] pub fn test_tsnaps(&self) -> bool {
        self.tsnaps() != 0
    }

    #[doc="Sets the TSNAPS field."]
    #[inline] pub fn set_tsnaps<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="GPTM Timer n Interval Load Write"]
    #[inline] pub fn tild(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TILD != 0"]
    #[inline] pub fn test_tild(&self) -> bool {
        self.tild() != 0
    }

    #[doc="Sets the TILD field."]
    #[inline] pub fn set_tild<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="GPTM Timer n PWM Interrupt Enable"]
    #[inline] pub fn tpwmie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TPWMIE != 0"]
    #[inline] pub fn test_tpwmie(&self) -> bool {
        self.tpwmie() != 0
    }

    #[doc="Sets the TPWMIE field."]
    #[inline] pub fn set_tpwmie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="GPTM Timer n Match Register Update"]
    #[inline] pub fn tmrsu(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if TMRSU != 0"]
    #[inline] pub fn test_tmrsu(&self) -> bool {
        self.tmrsu() != 0
    }

    #[doc="Sets the TMRSU field."]
    #[inline] pub fn set_tmrsu<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="GPTM Timer n PWM Legacy Operation"]
    #[inline] pub fn tplo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TPLO != 0"]
    #[inline] pub fn test_tplo(&self) -> bool {
        self.tplo() != 0
    }

    #[doc="Sets the TPLO field."]
    #[inline] pub fn set_tplo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="One-shot/Periodic Interrupt Disable"]
    #[inline] pub fn tcintd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TCINTD != 0"]
    #[inline] pub fn test_tcintd(&self) -> bool {
        self.tcintd() != 0
    }

    #[doc="Sets the TCINTD field."]
    #[inline] pub fn set_tcintd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Timer Compare Action Select"]
    #[inline] pub fn tcact(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x7) as u8) } // [15:13]
    }

    #[doc="Returns true if TCACT != 0"]
    #[inline] pub fn test_tcact(&self) -> bool {
        self.tcact() != 0
    }

    #[doc="Sets the TCACT field."]
    #[inline] pub fn set_tcact<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u32> for Tmr {
    #[inline]
    fn from(other: u32) -> Self {
         Tmr(other)
    }
}

impl ::core::fmt::Display for Tmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tmr() != 0 { try!(write!(f, " tmr=0x{:x}", self.tmr()))}
        if self.tcmr() != 0 { try!(write!(f, " tcmr"))}
        if self.tams() != 0 { try!(write!(f, " tams"))}
        if self.tcdir() != 0 { try!(write!(f, " tcdir"))}
        if self.tmie() != 0 { try!(write!(f, " tmie"))}
        if self.twot() != 0 { try!(write!(f, " twot"))}
        if self.tsnaps() != 0 { try!(write!(f, " tsnaps"))}
        if self.tild() != 0 { try!(write!(f, " tild"))}
        if self.tpwmie() != 0 { try!(write!(f, " tpwmie"))}
        if self.tmrsu() != 0 { try!(write!(f, " tmrsu"))}
        if self.tplo() != 0 { try!(write!(f, " tplo"))}
        if self.tcintd() != 0 { try!(write!(f, " tcintd"))}
        if self.tcact() != 0 { try!(write!(f, " tcact=0x{:x}", self.tcact()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPTM Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctl(pub u32);
impl Ctl {
    #[doc="GPTM Timer n Enable"]
    #[inline] pub fn ten<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TEN != 0"]
    #[inline] pub fn test_ten<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.ten(index) != 0
    }

    #[doc="Sets the TEN field."]
    #[inline] pub fn set_ten<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM Timer n Stall Enable"]
    #[inline] pub fn tstall<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 1 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TSTALL != 0"]
    #[inline] pub fn test_tstall<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.tstall(index) != 0
    }

    #[doc="Sets the TSTALL field."]
    #[inline] pub fn set_tstall<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 1 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM Timer n Event Mode"]
    #[inline] pub fn tevent<I: Into<bits::R2>>(&self, index: I) -> bits::U2 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 2 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if TEVENT != 0"]
    #[inline] pub fn test_tevent<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.tevent(index) != 0
    }

    #[doc="Sets the TEVENT field."]
    #[inline] pub fn set_tevent<I: Into<bits::R2>, V: Into<bits::U2>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        let shift: usize = 2 + (index << 3);
        self.0 &= !(0x3 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM RTC Stall Enable"]
    #[inline] pub fn rtcen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RTCEN != 0"]
    #[inline] pub fn test_rtcen(&self) -> bool {
        self.rtcen() != 0
    }

    #[doc="Sets the RTCEN field."]
    #[inline] pub fn set_rtcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPTM Timer n Output Trigger Enable"]
    #[inline] pub fn tote<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 5 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TOTE != 0"]
    #[inline] pub fn test_tote<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.tote(index) != 0
    }

    #[doc="Sets the TOTE field."]
    #[inline] pub fn set_tote<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 5 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM Timer n PWM Output Level"]
    #[inline] pub fn tpwml<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 6 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TPWML != 0"]
    #[inline] pub fn test_tpwml<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.tpwml(index) != 0
    }

    #[doc="Sets the TPWML field."]
    #[inline] pub fn set_tpwml<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 6 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
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
        if self.ten(0) != 0 { try!(write!(f, " ten[0]"))}
        if self.ten(1) != 0 { try!(write!(f, " ten[1]"))}
        if self.tstall(0) != 0 { try!(write!(f, " tstall[0]"))}
        if self.tstall(1) != 0 { try!(write!(f, " tstall[1]"))}
        if self.tevent(0) != 0 { try!(write!(f, " tevent[0]=0x{:x}", self.tevent(0)))}
        if self.tevent(1) != 0 { try!(write!(f, " tevent[1]=0x{:x}", self.tevent(1)))}
        if self.rtcen() != 0 { try!(write!(f, " rtcen"))}
        if self.tote(0) != 0 { try!(write!(f, " tote[0]"))}
        if self.tote(1) != 0 { try!(write!(f, " tote[1]"))}
        if self.tpwml(0) != 0 { try!(write!(f, " tpwml[0]"))}
        if self.tpwml(1) != 0 { try!(write!(f, " tpwml[1]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPTM Synchronize"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sync(pub u32);
impl Sync {
    #[doc="Synchronize GPTM Timer n"]
    #[inline] pub fn synct<I: Into<bits::R8>>(&self, index: I) -> bits::U2 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 1);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SYNCT != 0"]
    #[inline] pub fn test_synct<I: Into<bits::R8>>(&self, index: I) -> bool{
        self.synct(index) != 0
    }

    #[doc="Sets the SYNCT field."]
    #[inline] pub fn set_synct<I: Into<bits::R8>, V: Into<bits::U2>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 1);
        self.0 &= !(0x3 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Sync {
    #[inline]
    fn from(other: u32) -> Self {
         Sync(other)
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
        if self.synct(0) != 0 { try!(write!(f, " synct[0]=0x{:x}", self.synct(0)))}
        if self.synct(1) != 0 { try!(write!(f, " synct[1]=0x{:x}", self.synct(1)))}
        if self.synct(2) != 0 { try!(write!(f, " synct[2]=0x{:x}", self.synct(2)))}
        if self.synct(3) != 0 { try!(write!(f, " synct[3]=0x{:x}", self.synct(3)))}
        if self.synct(4) != 0 { try!(write!(f, " synct[4]=0x{:x}", self.synct(4)))}
        if self.synct(5) != 0 { try!(write!(f, " synct[5]=0x{:x}", self.synct(5)))}
        if self.synct(6) != 0 { try!(write!(f, " synct[6]=0x{:x}", self.synct(6)))}
        if self.synct(7) != 0 { try!(write!(f, " synct[7]=0x{:x}", self.synct(7)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPTM Interrupt Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Imr(pub u32);
impl Imr {
    #[doc="GPTM Timer n Time-Out Interrupt Mask"]
    #[inline] pub fn ttoim<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TTOIM != 0"]
    #[inline] pub fn test_ttoim<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.ttoim(index) != 0
    }

    #[doc="Sets the TTOIM field."]
    #[inline] pub fn set_ttoim<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM Timer n Capture Mode Match Interrupt Mask"]
    #[inline] pub fn cmim<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 1 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CMIM != 0"]
    #[inline] pub fn test_cmim<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.cmim(index) != 0
    }

    #[doc="Sets the CMIM field."]
    #[inline] pub fn set_cmim<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 1 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM Timer n Capture Mode Event Interrupt Mask"]
    #[inline] pub fn ceim<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 2 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CEIM != 0"]
    #[inline] pub fn test_ceim<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.ceim(index) != 0
    }

    #[doc="Sets the CEIM field."]
    #[inline] pub fn set_ceim<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 2 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM RTC Interrupt Mask"]
    #[inline] pub fn rtcim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RTCIM != 0"]
    #[inline] pub fn test_rtcim(&self) -> bool {
        self.rtcim() != 0
    }

    #[doc="Sets the RTCIM field."]
    #[inline] pub fn set_rtcim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPTM Timer n Match Interrupt Mask"]
    #[inline] pub fn tmim<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 4 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TMIM != 0"]
    #[inline] pub fn test_tmim<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.tmim(index) != 0
    }

    #[doc="Sets the TMIM field."]
    #[inline] pub fn set_tmim<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 4 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM Timer n DMA Done Interrupt Mask"]
    #[inline] pub fn dmaim<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 5 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DMAIM != 0"]
    #[inline] pub fn test_dmaim<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.dmaim(index) != 0
    }

    #[doc="Sets the DMAIM field."]
    #[inline] pub fn set_dmaim<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 5 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Imr {
    #[inline]
    fn from(other: u32) -> Self {
         Imr(other)
    }
}

impl ::core::fmt::Display for Imr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Imr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ttoim(0) != 0 { try!(write!(f, " ttoim[0]"))}
        if self.ttoim(1) != 0 { try!(write!(f, " ttoim[1]"))}
        if self.cmim(0) != 0 { try!(write!(f, " cmim[0]"))}
        if self.cmim(1) != 0 { try!(write!(f, " cmim[1]"))}
        if self.ceim(0) != 0 { try!(write!(f, " ceim[0]"))}
        if self.ceim(1) != 0 { try!(write!(f, " ceim[1]"))}
        if self.rtcim() != 0 { try!(write!(f, " rtcim"))}
        if self.tmim(0) != 0 { try!(write!(f, " tmim[0]"))}
        if self.tmim(1) != 0 { try!(write!(f, " tmim[1]"))}
        if self.dmaim(0) != 0 { try!(write!(f, " dmaim[0]"))}
        if self.dmaim(1) != 0 { try!(write!(f, " dmaim[1]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPTM Raw Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ris(pub u32);
impl Ris {
    #[doc="GPTM Timer n Time-Out Raw Interrupt"]
    #[inline] pub fn ttoris<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TTORIS != 0"]
    #[inline] pub fn test_ttoris<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.ttoris(index) != 0
    }

    #[doc="Sets the TTORIS field."]
    #[inline] pub fn set_ttoris<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM Timer n Capture Mode Match Raw Interrupt"]
    #[inline] pub fn cmris<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 1 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CMRIS != 0"]
    #[inline] pub fn test_cmris<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.cmris(index) != 0
    }

    #[doc="Sets the CMRIS field."]
    #[inline] pub fn set_cmris<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 1 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM Timer n Capture Mode Event Raw Interrupt"]
    #[inline] pub fn ceris<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 2 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CERIS != 0"]
    #[inline] pub fn test_ceris<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.ceris(index) != 0
    }

    #[doc="Sets the CERIS field."]
    #[inline] pub fn set_ceris<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 2 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM RTC Raw Interrupt"]
    #[inline] pub fn rtcris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RTCRIS != 0"]
    #[inline] pub fn test_rtcris(&self) -> bool {
        self.rtcris() != 0
    }

    #[doc="Sets the RTCRIS field."]
    #[inline] pub fn set_rtcris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPTM Timer A Match Raw Interrupt"]
    #[inline] pub fn tamris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TAMRIS != 0"]
    #[inline] pub fn test_tamris(&self) -> bool {
        self.tamris() != 0
    }

    #[doc="Sets the TAMRIS field."]
    #[inline] pub fn set_tamris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPTM Timer n DMA Done Raw Interrupt Status"]
    #[inline] pub fn dmaris<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 5 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DMARIS != 0"]
    #[inline] pub fn test_dmaris<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.dmaris(index) != 0
    }

    #[doc="Sets the DMARIS field."]
    #[inline] pub fn set_dmaris<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 5 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM Timer B Match Raw Interrupt"]
    #[inline] pub fn tbmris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TBMRIS != 0"]
    #[inline] pub fn test_tbmris(&self) -> bool {
        self.tbmris() != 0
    }

    #[doc="Sets the TBMRIS field."]
    #[inline] pub fn set_tbmris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
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
        if self.ttoris(0) != 0 { try!(write!(f, " ttoris[0]"))}
        if self.ttoris(1) != 0 { try!(write!(f, " ttoris[1]"))}
        if self.cmris(0) != 0 { try!(write!(f, " cmris[0]"))}
        if self.cmris(1) != 0 { try!(write!(f, " cmris[1]"))}
        if self.ceris(0) != 0 { try!(write!(f, " ceris[0]"))}
        if self.ceris(1) != 0 { try!(write!(f, " ceris[1]"))}
        if self.rtcris() != 0 { try!(write!(f, " rtcris"))}
        if self.tamris() != 0 { try!(write!(f, " tamris"))}
        if self.dmaris(0) != 0 { try!(write!(f, " dmaris[0]"))}
        if self.dmaris(1) != 0 { try!(write!(f, " dmaris[1]"))}
        if self.tbmris() != 0 { try!(write!(f, " tbmris"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPTM Masked Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mis(pub u32);
impl Mis {
    #[doc="GPTM Timer n Time-Out Masked Interrupt"]
    #[inline] pub fn ttomis<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TTOMIS != 0"]
    #[inline] pub fn test_ttomis<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.ttomis(index) != 0
    }

    #[doc="Sets the TTOMIS field."]
    #[inline] pub fn set_ttomis<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM Timer n Capture Mode Match Masked Interrupt"]
    #[inline] pub fn cmmis<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 1 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CMMIS != 0"]
    #[inline] pub fn test_cmmis<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.cmmis(index) != 0
    }

    #[doc="Sets the CMMIS field."]
    #[inline] pub fn set_cmmis<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 1 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM Timer n Capture Mode Event Masked Interrupt"]
    #[inline] pub fn cemis<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 2 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CEMIS != 0"]
    #[inline] pub fn test_cemis<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.cemis(index) != 0
    }

    #[doc="Sets the CEMIS field."]
    #[inline] pub fn set_cemis<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 2 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM RTC Masked Interrupt"]
    #[inline] pub fn rtcmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RTCMIS != 0"]
    #[inline] pub fn test_rtcmis(&self) -> bool {
        self.rtcmis() != 0
    }

    #[doc="Sets the RTCMIS field."]
    #[inline] pub fn set_rtcmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPTM Timer A Match Masked Interrupt"]
    #[inline] pub fn tammis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TAMMIS != 0"]
    #[inline] pub fn test_tammis(&self) -> bool {
        self.tammis() != 0
    }

    #[doc="Sets the TAMMIS field."]
    #[inline] pub fn set_tammis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPTM Timer n DMA Done Masked Interrupt"]
    #[inline] pub fn dmamis<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 5 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DMAMIS != 0"]
    #[inline] pub fn test_dmamis<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.dmamis(index) != 0
    }

    #[doc="Sets the DMAMIS field."]
    #[inline] pub fn set_dmamis<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 5 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM Timer B Match Masked Interrupt"]
    #[inline] pub fn tbmmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TBMMIS != 0"]
    #[inline] pub fn test_tbmmis(&self) -> bool {
        self.tbmmis() != 0
    }

    #[doc="Sets the TBMMIS field."]
    #[inline] pub fn set_tbmmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Mis {
    #[inline]
    fn from(other: u32) -> Self {
         Mis(other)
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
        if self.ttomis(0) != 0 { try!(write!(f, " ttomis[0]"))}
        if self.ttomis(1) != 0 { try!(write!(f, " ttomis[1]"))}
        if self.cmmis(0) != 0 { try!(write!(f, " cmmis[0]"))}
        if self.cmmis(1) != 0 { try!(write!(f, " cmmis[1]"))}
        if self.cemis(0) != 0 { try!(write!(f, " cemis[0]"))}
        if self.cemis(1) != 0 { try!(write!(f, " cemis[1]"))}
        if self.rtcmis() != 0 { try!(write!(f, " rtcmis"))}
        if self.tammis() != 0 { try!(write!(f, " tammis"))}
        if self.dmamis(0) != 0 { try!(write!(f, " dmamis[0]"))}
        if self.dmamis(1) != 0 { try!(write!(f, " dmamis[1]"))}
        if self.tbmmis() != 0 { try!(write!(f, " tbmmis"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPTM Interrupt Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc="GPTM Timer n Time-Out Raw Interrupt"]
    #[inline] pub fn ttocint<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TTOCINT != 0"]
    #[inline] pub fn test_ttocint<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.ttocint(index) != 0
    }

    #[doc="Sets the TTOCINT field."]
    #[inline] pub fn set_ttocint<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM Timer n Capture Mode Match Interrupt Clear"]
    #[inline] pub fn cmcint<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 1 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CMCINT != 0"]
    #[inline] pub fn test_cmcint<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.cmcint(index) != 0
    }

    #[doc="Sets the CMCINT field."]
    #[inline] pub fn set_cmcint<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 1 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM Timer n Capture Mode Event Interrupt Clear"]
    #[inline] pub fn cecint<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 2 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CECINT != 0"]
    #[inline] pub fn test_cecint<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.cecint(index) != 0
    }

    #[doc="Sets the CECINT field."]
    #[inline] pub fn set_cecint<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 2 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM RTC Interrupt Clear"]
    #[inline] pub fn rtccint(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RTCCINT != 0"]
    #[inline] pub fn test_rtccint(&self) -> bool {
        self.rtccint() != 0
    }

    #[doc="Sets the RTCCINT field."]
    #[inline] pub fn set_rtccint<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPTM Timer A Match Interrupt Clear"]
    #[inline] pub fn tamcint(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TAMCINT != 0"]
    #[inline] pub fn test_tamcint(&self) -> bool {
        self.tamcint() != 0
    }

    #[doc="Sets the TAMCINT field."]
    #[inline] pub fn set_tamcint<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="GPTM Timer n DMA Done Interrupt Clear"]
    #[inline] pub fn dmaint<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 5 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DMAINT != 0"]
    #[inline] pub fn test_dmaint<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.dmaint(index) != 0
    }

    #[doc="Sets the DMAINT field."]
    #[inline] pub fn set_dmaint<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 5 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM Timer B Match Interrupt Clear"]
    #[inline] pub fn tbmcint(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TBMCINT != 0"]
    #[inline] pub fn test_tbmcint(&self) -> bool {
        self.tbmcint() != 0
    }

    #[doc="Sets the TBMCINT field."]
    #[inline] pub fn set_tbmcint<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for Icr {
    #[inline]
    fn from(other: u32) -> Self {
         Icr(other)
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
        if self.ttocint(0) != 0 { try!(write!(f, " ttocint[0]"))}
        if self.ttocint(1) != 0 { try!(write!(f, " ttocint[1]"))}
        if self.cmcint(0) != 0 { try!(write!(f, " cmcint[0]"))}
        if self.cmcint(1) != 0 { try!(write!(f, " cmcint[1]"))}
        if self.cecint(0) != 0 { try!(write!(f, " cecint[0]"))}
        if self.cecint(1) != 0 { try!(write!(f, " cecint[1]"))}
        if self.rtccint() != 0 { try!(write!(f, " rtccint"))}
        if self.tamcint() != 0 { try!(write!(f, " tamcint"))}
        if self.dmaint(0) != 0 { try!(write!(f, " dmaint[0]"))}
        if self.dmaint(1) != 0 { try!(write!(f, " dmaint[1]"))}
        if self.tbmcint() != 0 { try!(write!(f, " tbmcint"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPTM Timer n Interval Load"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tilr(pub u32);
impl Tilr {
    #[doc="Gets the TILR field."]
    #[inline] pub fn tilr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if TILR != 0"]
    #[inline] pub fn test_tilr(&self) -> bool {
        self.tilr() != 0
    }

    #[doc="Sets the TILR field."]
    #[inline] pub fn set_tilr<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tilr {
    #[inline]
    fn from(other: u32) -> Self {
         Tilr(other)
    }
}

impl ::core::fmt::Display for Tilr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tilr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPTM Timer n Match"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tmtchr(pub u32);
impl Tmtchr {
    #[doc="Gets the TMTCHR field."]
    #[inline] pub fn tmtchr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if TMTCHR != 0"]
    #[inline] pub fn test_tmtchr(&self) -> bool {
        self.tmtchr() != 0
    }

    #[doc="Sets the TMTCHR field."]
    #[inline] pub fn set_tmtchr<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tmtchr {
    #[inline]
    fn from(other: u32) -> Self {
         Tmtchr(other)
    }
}

impl ::core::fmt::Display for Tmtchr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tmtchr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPTM Timer n Prescale"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tpr(pub u32);
impl Tpr {
    #[doc="GPTM Timer n Prescale"]
    #[inline] pub fn tpsr(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if TPSR != 0"]
    #[inline] pub fn test_tpsr(&self) -> bool {
        self.tpsr() != 0
    }

    #[doc="Sets the TPSR field."]
    #[inline] pub fn set_tpsr<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tpr {
    #[inline]
    fn from(other: u32) -> Self {
         Tpr(other)
    }
}

impl ::core::fmt::Display for Tpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tpsr() != 0 { try!(write!(f, " tpsr=0x{:x}", self.tpsr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPTM Timer n Prescale Match"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tpmr(pub u32);
impl Tpmr {
    #[doc="GPTM Timer n Prescale Match"]
    #[inline] pub fn tpsmr(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if TPSMR != 0"]
    #[inline] pub fn test_tpsmr(&self) -> bool {
        self.tpsmr() != 0
    }

    #[doc="Sets the TPSMR field."]
    #[inline] pub fn set_tpsmr<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tpmr {
    #[inline]
    fn from(other: u32) -> Self {
         Tpmr(other)
    }
}

impl ::core::fmt::Display for Tpmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tpmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tpsmr() != 0 { try!(write!(f, " tpsmr=0x{:x}", self.tpsmr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPTM Timer n"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tr(pub u32);
impl Tr {
    #[doc="Gets the TR field."]
    #[inline] pub fn tr(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if TR != 0"]
    #[inline] pub fn test_tr(&self) -> bool {
        self.tr() != 0
    }

    #[doc="Sets the TR field."]
    #[inline] pub fn set_tr<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tr {
    #[inline]
    fn from(other: u32) -> Self {
         Tr(other)
    }
}

impl ::core::fmt::Display for Tr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPTM Timer n Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tv(pub u32);
impl Tv {
    #[doc="Gets the TV field."]
    #[inline] pub fn tv(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if TV != 0"]
    #[inline] pub fn test_tv(&self) -> bool {
        self.tv() != 0
    }

    #[doc="Sets the TV field."]
    #[inline] pub fn set_tv<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tv {
    #[inline]
    fn from(other: u32) -> Self {
         Tv(other)
    }
}

impl ::core::fmt::Display for Tv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPTM RTC Predivide"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rtcpd(pub u32);
impl Rtcpd {
    #[doc="RTC Predivide Counter Value"]
    #[inline] pub fn rtcpd(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if RTCPD != 0"]
    #[inline] pub fn test_rtcpd(&self) -> bool {
        self.rtcpd() != 0
    }

    #[doc="Sets the RTCPD field."]
    #[inline] pub fn set_rtcpd<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rtcpd {
    #[inline]
    fn from(other: u32) -> Self {
         Rtcpd(other)
    }
}

impl ::core::fmt::Display for Rtcpd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rtcpd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rtcpd() != 0 { try!(write!(f, " rtcpd=0x{:x}", self.rtcpd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPTM Timer n Prescale Snapshot"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tps(pub u32);
impl Tps {
    #[doc="GPTM Timer n Prescaler Snapshot"]
    #[inline] pub fn pss(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if PSS != 0"]
    #[inline] pub fn test_pss(&self) -> bool {
        self.pss() != 0
    }

    #[doc="Sets the PSS field."]
    #[inline] pub fn set_pss<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tps {
    #[inline]
    fn from(other: u32) -> Self {
         Tps(other)
    }
}

impl ::core::fmt::Display for Tps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pss() != 0 { try!(write!(f, " pss=0x{:x}", self.pss()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPTM DMA Event"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmaev(pub u32);
impl Dmaev {
    #[doc="GPTM n Time-Out Event DMA Trigger Enable"]
    #[inline] pub fn ttodmaen<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TTODMAEN != 0"]
    #[inline] pub fn test_ttodmaen<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.ttodmaen(index) != 0
    }

    #[doc="Sets the TTODMAEN field."]
    #[inline] pub fn set_ttodmaen<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM n Capture Match Event DMA Trigger Enable"]
    #[inline] pub fn cmdmaen<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 1 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CMDMAEN != 0"]
    #[inline] pub fn test_cmdmaen<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.cmdmaen(index) != 0
    }

    #[doc="Sets the CMDMAEN field."]
    #[inline] pub fn set_cmdmaen<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 1 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM n Capture Event DMA Trigger Enable"]
    #[inline] pub fn cedmaen<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 2 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CEDMAEN != 0"]
    #[inline] pub fn test_cedmaen<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.cedmaen(index) != 0
    }

    #[doc="Sets the CEDMAEN field."]
    #[inline] pub fn set_cedmaen<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 2 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM RTC Match Event DMA Trigger Enable"]
    #[inline] pub fn rtcdmaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RTCDMAEN != 0"]
    #[inline] pub fn test_rtcdmaen(&self) -> bool {
        self.rtcdmaen() != 0
    }

    #[doc="Sets the RTCDMAEN field."]
    #[inline] pub fn set_rtcdmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPTM n Mode Match Event DMA Trigger Enable"]
    #[inline] pub fn tmdmaen<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 4 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TMDMAEN != 0"]
    #[inline] pub fn test_tmdmaen<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.tmdmaen(index) != 0
    }

    #[doc="Sets the TMDMAEN field."]
    #[inline] pub fn set_tmdmaen<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 4 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Dmaev {
    #[inline]
    fn from(other: u32) -> Self {
         Dmaev(other)
    }
}

impl ::core::fmt::Display for Dmaev {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmaev {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ttodmaen(0) != 0 { try!(write!(f, " ttodmaen[0]"))}
        if self.ttodmaen(1) != 0 { try!(write!(f, " ttodmaen[1]"))}
        if self.cmdmaen(0) != 0 { try!(write!(f, " cmdmaen[0]"))}
        if self.cmdmaen(1) != 0 { try!(write!(f, " cmdmaen[1]"))}
        if self.cedmaen(0) != 0 { try!(write!(f, " cedmaen[0]"))}
        if self.cedmaen(1) != 0 { try!(write!(f, " cedmaen[1]"))}
        if self.rtcdmaen() != 0 { try!(write!(f, " rtcdmaen"))}
        if self.tmdmaen(0) != 0 { try!(write!(f, " tmdmaen[0]"))}
        if self.tmdmaen(1) != 0 { try!(write!(f, " tmdmaen[1]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPTM ADC Event"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Adcev(pub u32);
impl Adcev {
    #[doc="GPTM n Time-Out Event ADC Trigger Enable"]
    #[inline] pub fn ttoadcen<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TTOADCEN != 0"]
    #[inline] pub fn test_ttoadcen<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.ttoadcen(index) != 0
    }

    #[doc="Sets the TTOADCEN field."]
    #[inline] pub fn set_ttoadcen<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM n Capture Match Event ADC Trigger Enable"]
    #[inline] pub fn cmadcen<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 1 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CMADCEN != 0"]
    #[inline] pub fn test_cmadcen<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.cmadcen(index) != 0
    }

    #[doc="Sets the CMADCEN field."]
    #[inline] pub fn set_cmadcen<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 1 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM Capture Event ADC Trigger Enable"]
    #[inline] pub fn ceadcen<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 2 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CEADCEN != 0"]
    #[inline] pub fn test_ceadcen<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.ceadcen(index) != 0
    }

    #[doc="Sets the CEADCEN field."]
    #[inline] pub fn set_ceadcen<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 2 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="GPTM RTC Match Event ADC Trigger Enable"]
    #[inline] pub fn rtcadcen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RTCADCEN != 0"]
    #[inline] pub fn test_rtcadcen(&self) -> bool {
        self.rtcadcen() != 0
    }

    #[doc="Sets the RTCADCEN field."]
    #[inline] pub fn set_rtcadcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="GPTM n Mode Match Event ADC Trigger Enable"]
    #[inline] pub fn tmadcen<I: Into<bits::R2>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 4 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TMADCEN != 0"]
    #[inline] pub fn test_tmadcen<I: Into<bits::R2>>(&self, index: I) -> bool{
        self.tmadcen(index) != 0
    }

    #[doc="Sets the TMADCEN field."]
    #[inline] pub fn set_tmadcen<I: Into<bits::R2>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 4 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Adcev {
    #[inline]
    fn from(other: u32) -> Self {
         Adcev(other)
    }
}

impl ::core::fmt::Display for Adcev {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Adcev {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ttoadcen(0) != 0 { try!(write!(f, " ttoadcen[0]"))}
        if self.ttoadcen(1) != 0 { try!(write!(f, " ttoadcen[1]"))}
        if self.cmadcen(0) != 0 { try!(write!(f, " cmadcen[0]"))}
        if self.cmadcen(1) != 0 { try!(write!(f, " cmadcen[1]"))}
        if self.ceadcen(0) != 0 { try!(write!(f, " ceadcen[0]"))}
        if self.ceadcen(1) != 0 { try!(write!(f, " ceadcen[1]"))}
        if self.rtcadcen() != 0 { try!(write!(f, " rtcadcen"))}
        if self.tmadcen(0) != 0 { try!(write!(f, " tmadcen[0]"))}
        if self.tmadcen(1) != 0 { try!(write!(f, " tmadcen[1]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPTM Peripheral Properties"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pp(pub u32);
impl Pp {
    #[doc="Count Size"]
    #[inline] pub fn size(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if SIZE != 0"]
    #[inline] pub fn test_size(&self) -> bool {
        self.size() != 0
    }

    #[doc="Sets the SIZE field."]
    #[inline] pub fn set_size<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Chain with Other Timers"]
    #[inline] pub fn chain(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CHAIN != 0"]
    #[inline] pub fn test_chain(&self) -> bool {
        self.chain() != 0
    }

    #[doc="Sets the CHAIN field."]
    #[inline] pub fn set_chain<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Synchronize Start"]
    #[inline] pub fn synccnt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SYNCCNT != 0"]
    #[inline] pub fn test_synccnt(&self) -> bool {
        self.synccnt() != 0
    }

    #[doc="Sets the SYNCCNT field."]
    #[inline] pub fn set_synccnt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Alternate Clock Source"]
    #[inline] pub fn altclk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if ALTCLK != 0"]
    #[inline] pub fn test_altclk(&self) -> bool {
        self.altclk() != 0
    }

    #[doc="Sets the ALTCLK field."]
    #[inline] pub fn set_altclk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
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
        if self.size() != 0 { try!(write!(f, " size=0x{:x}", self.size()))}
        if self.chain() != 0 { try!(write!(f, " chain"))}
        if self.synccnt() != 0 { try!(write!(f, " synccnt"))}
        if self.altclk() != 0 { try!(write!(f, " altclk"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPTM Clock Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cc(pub u32);
impl Cc {
    #[doc="Alternate Clock Source"]
    #[inline] pub fn altclk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ALTCLK != 0"]
    #[inline] pub fn test_altclk(&self) -> bool {
        self.altclk() != 0
    }

    #[doc="Sets the ALTCLK field."]
    #[inline] pub fn set_altclk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
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
        if self.altclk() != 0 { try!(write!(f, " altclk"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

pub struct TimerCh { pub periph: TimerPeriph, pub index: usize }
channel!(TIMER0A, Timer0a, TIMER0, Timer0, _TIMER0A, TimerCh, _TIMER0, 0);
channel!(TIMER0B, Timer0b, TIMER0, Timer0, _TIMER0B, TimerCh, _TIMER0, 1);
channel!(TIMER1A, Timer1a, TIMER1, Timer1, _TIMER1A, TimerCh, _TIMER1, 0);
channel!(TIMER1B, Timer1b, TIMER1, Timer1, _TIMER1B, TimerCh, _TIMER1, 1);
channel!(TIMER2A, Timer2a, TIMER2, Timer2, _TIMER2A, TimerCh, _TIMER2, 0);
channel!(TIMER2B, Timer2b, TIMER2, Timer2, _TIMER2B, TimerCh, _TIMER2, 1);
channel!(TIMER3A, Timer3a, TIMER3, Timer3, _TIMER3A, TimerCh, _TIMER3, 0);
channel!(TIMER3B, Timer3b, TIMER3, Timer3, _TIMER3B, TimerCh, _TIMER3, 1);
channel!(TIMER4A, Timer4a, TIMER4, Timer4, _TIMER4A, TimerCh, _TIMER4, 0);
channel!(TIMER4B, Timer4b, TIMER4, Timer4, _TIMER4B, TimerCh, _TIMER4, 1);
channel!(TIMER5A, Timer5a, TIMER5, Timer5, _TIMER5A, TimerCh, _TIMER5, 0);
channel!(TIMER5B, Timer5b, TIMER5, Timer5, _TIMER5B, TimerCh, _TIMER5, 1);
channel!(TIMER6A, Timer6a, TIMER6, Timer6, _TIMER6A, TimerCh, _TIMER6, 0);
channel!(TIMER6B, Timer6b, TIMER6, Timer6, _TIMER6B, TimerCh, _TIMER6, 1);
channel!(TIMER7A, Timer7a, TIMER7, Timer7, _TIMER7A, TimerCh, _TIMER7, 0);
channel!(TIMER7B, Timer7b, TIMER7, Timer7, _TIMER7B, TimerCh, _TIMER7, 1);

pub trait IrqTimer<T> {
    fn irq_timer(&self) -> T;
}

impl IrqTimer<super::irq::IrqTimer0a> for Timer0a {
    fn irq_timer(&self) -> super::irq::IrqTimer0a { super::irq::IRQ_TIMER0A }
}

impl IrqTimer<super::irq::IrqTimer0b> for Timer0b {
    fn irq_timer(&self) -> super::irq::IrqTimer0b { super::irq::IRQ_TIMER0B }
}

impl IrqTimer<super::irq::IrqTimer1a> for Timer1a {
    fn irq_timer(&self) -> super::irq::IrqTimer1a { super::irq::IRQ_TIMER1A }
}

impl IrqTimer<super::irq::IrqTimer1b> for Timer1b {
    fn irq_timer(&self) -> super::irq::IrqTimer1b { super::irq::IRQ_TIMER1B }
}

impl IrqTimer<super::irq::IrqTimer2a> for Timer2a {
    fn irq_timer(&self) -> super::irq::IrqTimer2a { super::irq::IRQ_TIMER2A }
}

impl IrqTimer<super::irq::IrqTimer2b> for Timer2b {
    fn irq_timer(&self) -> super::irq::IrqTimer2b { super::irq::IRQ_TIMER2B }
}

impl IrqTimer<super::irq::IrqTimer3a> for Timer3a {
    fn irq_timer(&self) -> super::irq::IrqTimer3a { super::irq::IRQ_TIMER3A }
}

impl IrqTimer<super::irq::IrqTimer3b> for Timer3b {
    fn irq_timer(&self) -> super::irq::IrqTimer3b { super::irq::IRQ_TIMER3B }
}

impl IrqTimer<super::irq::IrqTimer4a> for Timer4a {
    fn irq_timer(&self) -> super::irq::IrqTimer4a { super::irq::IRQ_TIMER4A }
}

impl IrqTimer<super::irq::IrqTimer4b> for Timer4b {
    fn irq_timer(&self) -> super::irq::IrqTimer4b { super::irq::IRQ_TIMER4B }
}

impl IrqTimer<super::irq::IrqTimer5a> for Timer5a {
    fn irq_timer(&self) -> super::irq::IrqTimer5a { super::irq::IRQ_TIMER5A }
}

impl IrqTimer<super::irq::IrqTimer5b> for Timer5b {
    fn irq_timer(&self) -> super::irq::IrqTimer5b { super::irq::IRQ_TIMER5B }
}

impl IrqTimer<super::irq::IrqTimer6a> for Timer6a {
    fn irq_timer(&self) -> super::irq::IrqTimer6a { super::irq::IRQ_TIMER6A }
}

impl IrqTimer<super::irq::IrqTimer6b> for Timer6b {
    fn irq_timer(&self) -> super::irq::IrqTimer6b { super::irq::IRQ_TIMER6B }
}

impl IrqTimer<super::irq::IrqTimer7a> for Timer7a {
    fn irq_timer(&self) -> super::irq::IrqTimer7a { super::irq::IRQ_TIMER7A }
}

impl IrqTimer<super::irq::IrqTimer7b> for Timer7b {
    fn irq_timer(&self) -> super::irq::IrqTimer7b { super::irq::IRQ_TIMER7B }
}
