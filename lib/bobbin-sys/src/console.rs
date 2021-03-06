//! Console support
//! 
//! The console supports writing byte slices and strings and can optionally
//! convert `\n` to `\r\n` for terminal output. It is implemented as a type
//! `Console` as well as a global singleton that can be set for use as
//! the default console used by `print!` and `println!`.
//! 
//! No serialization is performed by the console, so simultaneous calls may 
//! potentially be interleaved.
use bobbin_hal::serial::SerialTx;

use core::fmt::{self, Write};

const DIGITS: &[u8; 16] = b"0123456789abcdef";    

static mut CONSOLE: Option<Console<'static>> = None;

/// Describes the console operating mode.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConsoleMode {
    /// All output is passed directly to underlying device.
    Raw,
    /// `\n` in the output is converted to `\r\n`.
    Cooked,
}

/// Traits that must be implemented by the device used by the console for output.
pub trait ConsoleWrite {
    /// Write a byte slice to the console, translating according to ConsoleMode.
    fn write(&self, buf: &[u8]);
    /// Write a single byte to the console, translating according to ConsoleMode.
    fn putc(&self, b: u8);
}

impl<T: SerialTx<u8>> ConsoleWrite for T {
    fn write(&self, buf: &[u8]) {
        <Self as SerialTx<u8>>::write(self, buf);
    }
    fn putc(&self, b: u8) {
        <Self as SerialTx<u8>>::putc(self, b);
    }
}

/// A basic console driver.
#[derive(Clone)]
pub struct Console<'a>(&'a ConsoleWrite, ConsoleMode);

impl<'a> Console<'a> {
    /// Borrow a reference to the global console singleton.
    pub fn borrow() -> Option<&'static Console<'static>> {
        unsafe { CONSOLE.as_ref() }
    }    

    /// Set `console` as the global console singleton.
    pub fn set(console: Console<'static>) {
        unsafe { CONSOLE = Some(console) }
    }

    /// Create a new console using `other` as the underlying output
    /// device and `mode` as the console mode.
    pub fn new(other: &'a ConsoleWrite, mode: ConsoleMode) -> Self {
        Console(other, mode)
    }

    /// Write a byte string to the console using the current mode, possibly blocking.
    pub fn write(&self, buf: &[u8]) {
        match self.1 {
            ConsoleMode::Raw => self.0.write(buf),
            ConsoleMode::Cooked => self.write_cooked(buf),
        }
    }

    /// Write a byte string to the console using the current modewith a newline appended, 
    /// possibly blocking.
    pub fn writeln(&self, buf: &[u8]) {
        self.write(buf);
        self.write(b"\n");
    }

    /// Write a byte string to the console, translating `\n` to `\r\n`.
    pub fn write_cooked(&self, buf: &[u8]) {
        for byte in buf {
            if *byte == b'\n' {
                self.0.putc(b'\r');
            }
            self.0.putc(*byte);
        }        
    }

    /// Write `v` to the console as a string of base `base`.
    /// 
    /// NOTE: Only base 10 and base 16 are currently supported.
    pub fn write_u32(&self, mut v: u32, base: u32) {
        let mut buf = [0u8; 6];
        let mut i = buf.len();
        if v == 0 {
            self.write(b"0");            
        } else {
            if base == 16 {
                while v > 0 && i > 0 {
                    i -= 1;
                    buf[i] = DIGITS[(v % 16) as usize];
                    v = v / 16;
                }
            } else {
                while v > 0 && i > 0 {
                    i -= 1;
                    buf[i] = DIGITS[(v % 10) as usize];
                    v = v / 10;
                }        
            }
            self.write(&buf[i..])
        }
    }

    /// Write `v` to the console as a 2 byte lower case hex string.
    pub fn write_u8_hex(&self, v: u8) {
        self.write(&u8_to_hex(v));
    }

    /// Write `v` to the console as a 4 byte lower case hex string.
    pub fn write_u16_hex(&self, v: u16) {
        self.write(&u16_to_hex(v));
    }

    /// Write `v` to the console as an 8 byte lower case hex string.
    pub fn write_u32_hex(&self, v: u32) {
        self.write(&u32_to_hex(v));
    }

    pub unsafe fn dump_ptr(&self, ptr: *const u8, len: usize) {
        self.dump_slice(::core::slice::from_raw_parts(ptr, len))
    }

    /// Dump a byte slice to the console.
    pub fn dump_slice(&self, data: &[u8]) {
        let addr = data.as_ptr() as usize;
        for (i, d) in data.iter().enumerate() {
            if i % 32 == 0 {
                if i > 0 {
                    self.write(b"\n");
                }
                self.write_u32_hex((addr + i) as u32);
                self.write(b": ");
            }
            if i % 16 == 0 {
                self.write(b" ");
            }
            if i % 4 == 0 {
                self.write(b" ");
            }
            self.write_u8_hex(*d);
        }
        self.write(b"\n");
    }

}

#[inline]
fn u8_to_hex(c: u8) -> [u8; 2] {
    [DIGITS[((c >> 4) & 0xf) as usize], DIGITS[(c & 0xf) as usize]]
}

#[inline]
fn u16_to_hex(c: u16) -> [u8; 4] {
    let (a, b) = (u8_to_hex((c >> 8) as u8), u8_to_hex(c as u8));
    [a[0], a[1], b[0], b[1]]
}

#[inline]
fn u32_to_hex(c: u32) -> [u8; 8] {
    let (a, b) = (u16_to_hex((c >> 16) as u16), u16_to_hex(c as u16));
    [a[0], a[1], a[2], a[3], b[0], b[1], b[2], b[3]]
}
    

impl<'a> fmt::Write for Console<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {        
        self.0.write(s.as_bytes());
        Ok(())
    }
}

#[doc(hidden)]
pub fn with_console<F: FnOnce(&mut Console)>(f: F) {
    unsafe {
        if let Some(ref mut console) = CONSOLE {
            f(console)
        }
    }
}

#[doc(hidden)]
pub fn write_fmt(args: fmt::Arguments) {    
    with_console(|c| {
        c.write_fmt(args).ok();
    });
}

#[doc(hidden)]
pub fn write_str(s: &str) {
    with_console(|c| {
        c.write_str(s).ok();
    });
}

#[doc(hidden)]
pub fn write(buf: &[u8]) {
    with_console(|c| {
        c.write(buf);
    });
}

#[doc(hidden)]
pub fn write_u32(v: u32, base: u32) {
    with_console(|c| {
        c.write_u32(v, base);
    });
}

#[doc(hidden)]
pub fn write_u8_hex(v: u8) {
    with_console(|c| {
        c.write_u8_hex(v);
    });
}

#[doc(hidden)]
pub fn write_u16_hex(v: u16) {
    with_console(|c| {
        c.write_u16_hex(v);
    });
}

#[doc(hidden)]
pub fn write_u32_hex(v: u32) {
    with_console(|c| {
        c.write_u32_hex(v);
    });
}