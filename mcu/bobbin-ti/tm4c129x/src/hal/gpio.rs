pub use bobbin_common::digital::*;
pub use bobbin_common::{Pin, AltFn};
pub use chip::gpio::*;
pub use super::sysctl::SysctlEnabled;
// use chip::sig::{SignalTx, SignalRx, SignalCcp, SignalPwm, SignalAin};

use core::ops::Deref;

pub enum Dir {
    In = 0,
    Out = 1,    
}


macro_rules! impl_mode {
    ($mode:ident, $meth:ident, $sig:ident) => (
        use chip::sig::$sig;

        pub trait $mode<SIG, PERIPH> {
            fn $meth(&self, _: &PERIPH) -> &Self;
        }

        impl<PERIPH, PIN, SIG> $mode<SIG, PERIPH> for PIN where PERIPH: $sig<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=GpioPin> {
            #[inline]
            fn $meth(&self, _: &PERIPH) -> &Self {
                self.mode_alt_fn(self.alt_fn());
                self
            }
        }            
    )
}

impl_mode!(ModeTx, mode_tx, SignalTx);
impl_mode!(ModeRx, mode_rx, SignalRx);
impl_mode!(ModeCcp, mode_ccp, SignalCcp);
impl_mode!(ModePwm, mode_pwm, SignalPwm);
impl_mode!(ModeAin, mode_ain, SignalAin);
impl_mode!(ModeI2cScl, mode_i2c_scl, SignalI2cScl);
impl_mode!(ModeI2cSda, mode_i2c_sda, SignalI2cSda);
impl_mode!(ModeSsiFss, mode_ssi_fss, SignalSsiFss);
impl_mode!(ModeSsiClk, mode_ssi_clk, SignalSsiClk);
impl_mode!(ModeSsiDat0, mode_ssi_dat0, SignalSsiDat0);
impl_mode!(ModeSsiDat1, mode_ssi_dat1, SignalSsiDat1);
impl_mode!(ModeSsiDat2, mode_ssi_dat2, SignalSsiDat2);
impl_mode!(ModeSsiDat3, mode_ssi_dat3, SignalSsiDat3);

// pub trait ModeTx<SIG, PERIPH> {
//     fn mode_tx(&self, _: &PERIPH) -> &Self;
// }

// pub trait ModeRx<SIG, PERIPH> {
//     fn mode_rx(&self, _: &PERIPH) -> &Self;
// }

// pub trait ModeCcp<SIG, PERIPH> {
//     fn mode_ccp(&self, _: &PERIPH) -> &Self;
// }

// pub trait ModePwm<SIG, PERIPH> {
//     fn mode_pwm(&self, _: &PERIPH) -> &Self;
// }

// pub trait ModeAin<SIG, PERIPH> {
//     fn mode_ain(&self, _: &PERIPH) -> &Self;
// }

// impl<PERIPH, PIN, SIG> ModeTx<SIG, PERIPH> for PIN where PERIPH: SignalTx<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=GpioPin> {
//     fn mode_tx(&self, _: &PERIPH) -> &Self {
//         self.mode_alt_fn(self.alt_fn());
//         self
//     }
// }

// impl<PERIPH, PIN, SIG> ModeRx<SIG, PERIPH> for PIN where PERIPH: SignalRx<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=GpioPin> {
//     fn mode_rx(&self, _: &PERIPH) -> &Self {
//         self.mode_alt_fn(self.alt_fn());
//         self
//     }
// }

// impl<PERIPH, PIN, SIG> ModeCcp<SIG, PERIPH> for PIN where PERIPH: SignalCcp<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=GpioPin> {
//     fn mode_ccp(&self, _: &PERIPH) -> &Self {
//         self.mode_alt_fn(self.alt_fn());
//         self
//     }
// }

// impl<PERIPH, PIN, SIG> ModePwm<SIG, PERIPH> for PIN where PERIPH: SignalPwm<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=GpioPin> {
//     fn mode_pwm(&self, _: &PERIPH) -> &Self {
//         self.mode_alt_fn(self.alt_fn());
//         self
//     }
// }

// impl<PERIPH, PIN, SIG> ModeAin<SIG, PERIPH> for PIN where PERIPH: SignalAin<SIG>, PIN: AltFn<SIG>, PIN: Deref<Target=GpioPin> {
//     fn mode_ain(&self, _: &PERIPH) -> &Self {
//         self.mode_alt_fn(self.alt_fn());        
//         self.deref().set_digital_enable(false).set_analog_select(true);
//         self
//     }
// }

// pub trait GpioExt {
//     fn set_dir(&self, value: Dir) -> &Self;
//     fn set_afsel(&self, value: bool) -> &Self;
//     fn set_pullup_select(&self, value: bool) -> &Self;
//     fn set_pulldown_select(&self, value: bool) -> &Self;
//     fn set_open_drain_select(&self, value: bool) -> &Self;
//     fn set_digital_enable(&self, value: bool) -> &Self;
//     fn set_analog_select(&self, value: bool) -> &Self;
//     fn set_port_control(&self, value: usize) -> &Self;

//     fn mode_input(&self) -> &Self {
//         self.set_dir(Dir::In);
//         self.set_afsel(false);
//         self.set_digital_enable(true);
//         self.set_port_control(0);
//         self
//     }

//     fn mode_output(&self) -> &Self {
//         self.set_dir(Dir::Out);
//         self.set_afsel(false);
//         self.set_digital_enable(true);
//         self.set_port_control(0);
//         self
//     }    

//     fn mode_alt_fn(&self, value: usize) -> &Self {
//         self.set_dir(Dir::In);
//         self.set_afsel(true);
//         self.set_digital_enable(true);
//         self.set_port_control(value);
//         self
//     }

//     fn pull_up(&self) -> &Self {
//         self.set_pullup_select(true)
//     }

//     fn pull_down(&self) -> &Self {
//         self.set_pulldown_select(true)
//     }    
// }

impl GpioPin {
    pub fn set_dir(&self, value: Dir) -> &Self {
        self.port.with_dir(|r| r.set_dir(self.index, value as u32));
        self
    }

    pub fn set_afsel(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.port.with_afsel(|r| r.set_afsel(self.index, value));
        self
    }

    pub fn set_pullup_select(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.port.with_pur(|r| r.set_pue(self.index, value));
        self
    }

    pub fn set_pulldown_select(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.port.with_pdr(|r| r.set_pde(self.index, value));
        self
    }

    pub fn set_open_drain_select(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.port.with_odr(|r| r.set_ode(self.index, value));
        self
    }

    pub fn set_digital_enable(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.port.with_den(|r| r.set_den(self.index, value));
        self
    }

    pub fn set_analog_select(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.port.with_amsel(|r| r.set_gpioamsel(self.index, value));
        self
    }

    pub fn set_port_control(&self, value: usize) -> &Self {
        self.port.with_pctl(|r| r.set_pmc(self.index, value as u32));
        self
    }

    pub fn mode_disabled(&self) -> &Self {
        self.set_dir(Dir::In);
        self.set_afsel(false);
        self.set_digital_enable(false);
        self.set_port_control(0);
        self
    }

    pub fn mode_input(&self) -> &Self {
        self.set_dir(Dir::In);
        self.set_afsel(false);
        self.set_digital_enable(true);
        self.set_port_control(0);
        self
    }

    pub fn mode_output(&self) -> &Self {
        self.set_dir(Dir::Out);
        self.set_afsel(false);
        self.set_digital_enable(true);
        self.set_port_control(0);
        self
    }    

    pub fn mode_alt_fn(&self, value: usize) -> &Self {
        self.set_dir(Dir::In);
        self.set_afsel(true);
        self.set_digital_enable(true);
        self.set_port_control(value);
        self
    }

    pub fn pull_up(&self) -> &Self {
        self.set_pullup_select(true)
    }

    pub fn pull_down(&self) -> &Self {
        self.set_pulldown_select(true)
    }    

    pub fn test_ris(&self) -> bool {
        self.port.ris().test_ris(self.index)
    }

    pub fn clr_ris(&self) -> &Self {
        self.port.set_icr(|r| r.set_ic(self.index, 1));
        self
    }

    pub fn set_is(&self, value: bool) -> &Self {
        self.port.set_is(|r| r.set_is(self.index, value));
        self
    }

    pub fn set_ibe(&self, value: bool) -> &Self {
        self.port.set_ibe(|r| r.set_ibe(self.index, value));
        self
    }

    pub fn set_iev(&self, value: bool) -> &Self {
        self.port.set_iev(|r| r.set_iev(self.index, value));
        self
    }

}

impl DigitalInput for GpioPin  {        
    fn input(&self) -> bool {            
        self.port.data().data(self.index) != 0
    }           
}

impl DigitalOutput for GpioPin {
    fn output(&self) -> bool {
        self.port.data().data(self.index) != 0
    }   

    fn set_output(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.port.with_data(|r| r.set_data(self.index, value));
        self
    }    

    fn toggle_output(&self) -> &Self {
        self.set_output(!self.output())
    }
}