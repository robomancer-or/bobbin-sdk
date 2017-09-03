pub use bobbin_common::configure::*;
pub use bobbin_common::enabled::*;
pub use bobbin_common::serial::*;
pub use chip::sercom::*;
pub use super::pm::PmEnabled;

use bobbin_common::bits::*;

// NOTE: Before usage, power up and set clocks

// pm::set_sercom_enabled(sercom, true);

// // Set GCLK_GEN0 as source for SERCOM

// gclk::GCLK.set_clkctrl(gclk::Clkctrl(0)
//     .set_id(0x14 + sercom_index(sercom))
//     .set_gen(0x0)
//     .set_clken(1)
// );
// // Wait for synchronization
// while gclk::GCLK.status().syncbusy() != 0 {}


pub struct Config {
    ctrla: usart::Ctrla,
    ctrlb: usart::Ctrlb,
    baud: usart::Baud,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ctrla: usart::Ctrla(0).set_mode(0x1).set_dord(0x1),
            ctrlb: usart::Ctrlb(0).set_rxen(0x1).set_txen(0x1).set_chsize(0x0),
            baud: usart::Baud(0),
        }
    }
}

impl Config {
    pub fn set_baud(mut self, value: u16) -> Self {
        self.baud = usart::Baud(value);
        self
    }

    pub fn set_txpo<V: Into<U2>>(mut self, value: V) -> Self {
        self.ctrla = self.ctrla.set_txpo(value.into());
        self
    }

    pub fn set_rxpo<V: Into<U2>>(mut self, value: V) -> Self {
        self.ctrla = self.ctrla.set_rxpo(value.into());
        self
    }
}

impl Configure<Config> for SercomPeriph {
    fn config(&self) -> Config {
        let s = self.usart();
        Config {
            ctrla: s.ctrla(),
            ctrlb: s.ctrlb(),
            baud: s.baud(),
        }
    }

    fn configure(&self, cfg: Config) -> &Self {
        let s = self.usart();

        // Reset peripheral
        s.set_ctrla(|r| r.set_swrst(0x1));

        // Wait for reset
        while s.ctrla().swrst() != 0 {}

        // Update CTRLA
        s.set_ctrla(|_| cfg.ctrla);

        // Update CTRLB
        s.set_ctrlb(|_| cfg.ctrlb);

        // Wait for synchronization
        while s.syncbusy().ctrlb() != 0 {}

        // Update BAUD
        s.set_baud(|_| cfg.baud);

        self
    }
}

impl Enabled for SercomPeriph {
    fn enabled(&self) -> bool {
        self.usart().ctrla().test_enable()
    }

    fn set_enabled(&self, value: bool) -> &Self {
        self.usart().with_ctrla(|r| r.set_enable(value));
        self
    }        
}


impl SercomPeriph {
    pub fn configure(&self, baud: u16, tx_pad: u8, rx_pad: u8) -> &Self {
        let s = self.usart();

        // Before Use: Power up SERCOM
        // Before Use: Set SERCOM Clock

        // UART Initialization

        // Wait for synchronization
        while s.syncbusy().enable() != 0 {}

        // Disable the UART module
        s.with_ctrla(|r| r.set_enable(0));

        // Wait for synchronization
        while s.syncbusy().swrst() != 0 {}
        
        // Software Reset
        s.with_ctrla(|r| r.set_swrst(1));
    
        // Wait for synchronization
        while s.ctrla().swrst() != 0 {}

        // Wait for synchronization
        while s.syncbusy().swrst() != 0 || s.syncbusy().enable() != 0 {}

        // Update the UART pad settings, mode and data order settings

        s.set_ctrla(|r| r
            .set_txpo(tx_pad as u32)
            .set_rxpo(rx_pad as u32)
            .set_mode(0x1)
            .set_dord(1)
        );

        // Wait for synchronization
        while s.syncbusy().ctrlb() != 0 {}

        // Enable transmit and receive and set data size to 8 bits

        s.set_ctrlb(|r| r
            .set_rxen(1)
            .set_txen(1)
            .set_chsize(0)
        );

        // Load the baud value
        self.set_baud(baud);
        self
    }

    pub fn set_baud(&self, value: u16) -> &Self {
        let s = self.usart();
        s.set_baud(|_| usart::Baud(value));
        while s.syncbusy().enable() != 0 {}
        self
    }

    // pub fn write(&self, buf: &[u8]) -> &Self {
    //     for b in buf.iter() {
    //         self.putc(*b);
    //     }
    //     self
    // }
}

impl SerialTx<u8> for SercomPeriph {    
    fn can_tx(&self) -> bool {
        self.usart().intflag().dre() != 0
    }

    fn tx(&self, c: u8) -> &Self {
        self.usart().set_data(|r| r.set_data(c));
        self
    }
}

impl SerialRx<u8> for SercomPeriph {
    fn can_rx(&self) -> bool {
        self.usart().intflag().rxc() != 0
    }

    fn rx(&self) -> u8 {
        self.usart().data().data().value() as u8
    }
}