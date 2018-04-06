use mcu::systick::SYSTICK;
use mcu::nvic::*;

use core::ops::Deref;

static mut EXC_HANDLERS_PTR: *mut Option<ExceptionHandler> = ::core::ptr::null_mut();
static mut EXC_HANDLERS_LEN: usize = 0;

#[derive(Debug)]
pub enum Error {
    Unavailable,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Exception {
    Reset,
    NMI,
    HardFault,
    MemManage,
    BusFault,
    UsageFault,
    SVCall,
    PendSV,
    SysTick,
    Interrupt(u8),
    Reserved(u8),
}

impl From<u8> for Exception {
    fn from(other: u8) -> Self {
        match other {
            1 => Exception::Reset,
            2 => Exception::NMI,
            3 => Exception::HardFault,
            4 => Exception::MemManage,
            5 => Exception::BusFault,
            6 => Exception::UsageFault,
            7 | 8 | 9 | 10 => Exception::Reserved(other),
            11 => Exception::SVCall,
            12 | 13 => Exception::Reserved(other),
            14 => Exception::PendSV,
            15 => Exception::SysTick,
            _ => Exception::Interrupt(other - 16),
        }
    }
}

impl From<Exception> for u8 {
    fn from(other: Exception) -> u8 {
        match other {
            Exception::Reset => 1,
            Exception::NMI => 2,
            Exception::HardFault => 3,
            Exception::MemManage => 4,
            Exception::BusFault => 5,
            Exception::UsageFault => 6,
            Exception::SVCall => 11,
            Exception::PendSV => 14,
            Exception::SysTick => 15,
            Exception::Interrupt(n) => n + 16,
            Exception::Reserved(n) => n,
        }
    }
}

pub trait HandleException {
    unsafe fn handle_exception(&self, exc: Exception);
}

#[derive(Clone, Copy)]
pub struct ExceptionHandler {
    exc_num: u8,
    handler: *const HandleException,
}

impl ExceptionHandler {
    fn new(exc_num: u8, handler: *const HandleException) -> Self {
        Self { exc_num, handler }
    }
}

#[must_use]
#[derive(Debug)]
pub struct Guard<'a, H: 'a> {
    exc_num: u8,
    index: usize,
    handler: &'a H,
}

impl<'a, H: 'a> Guard<'a, H> {
    pub fn exc_num(&self) -> u8 {
        self.exc_num
    }    
}

impl<'a, H: 'a> Drop for Guard<'a, H> {
    fn drop(&mut self) {
        if Dispatcher::slots_used_for_exc(self.exc_num) <= 1 {
            match self.exc_num {
                15 => { SYSTICK.set_tick_interrupt(false); },
                e @ _ if e >= 16 => { NVIC.set_enabled(e, false); },
                _ => {},
            }
        }
        Dispatcher::handlers()[self.index] = None
    }
}

impl<'a, H: 'a> Deref for Guard<'a, H> {
    type Target = H;
    fn deref(&self) -> &H {
        self.handler
    }
}

pub struct Dispatcher {}

impl Dispatcher {
    pub fn init(slots: &'static mut [Option<ExceptionHandler>]) {
        unsafe {
            EXC_HANDLERS_PTR = slots.as_mut_ptr();
            EXC_HANDLERS_LEN = slots.len();
        }
    }

    pub fn slots() -> usize {
        Self::handlers().len()
    }

    pub fn slots_used() -> usize {
        Self::handlers().iter().filter(|h| h.is_some()).count()
    }

    pub fn slots_used_for_exc(exc_num: u8) -> usize {
        let mut count = 0;
        for h in Self::handlers().iter() {
            if let &Some(h) = h {
                if h.exc_num == exc_num {
                    count += 1;
                }
            }
        }
        count
    }

    #[inline]
    pub fn handlers() -> &'static mut [Option<ExceptionHandler>] {
        unsafe {
            ::core::slice::from_raw_parts_mut(EXC_HANDLERS_PTR, EXC_HANDLERS_LEN)
        }

    }
    
    pub fn register_handler<H: 'static + HandleException>(exc_num: u8, handler: &H) -> Result<Guard<H>, Error> {
        let exc_handler = ExceptionHandler::new(exc_num, handler);
        let exc_handlers = Self::handlers();
        for i in 0..exc_handlers.len() {
            if exc_handlers[i].is_none() {
                exc_handlers[i] = Some(exc_handler);
                match exc_handler.exc_num {
                    15 => { SYSTICK.set_tick_interrupt(true); },
                    e @ _ if e >= 16 => { NVIC.set_enabled(e- 16, true); },
                    _ => {},
                }
                return Ok(Guard { exc_num: exc_num, index: i, handler})
            }
        }
        Err(Error::Unavailable)
    }

    pub fn register_svcall_handler<H: 'static + HandleException>(handler: &H) -> Result<Guard<H>, Error> {        
        Self::register_handler(11, handler)
    }

    pub fn register_pendsv_handler<H: 'static + HandleException>(handler: &H) -> Result<Guard<H>, Error> {        
        Self::register_handler(14, handler)
    }

    pub fn register_systick_handler<H: 'static + HandleException>(handler: &H) -> Result<Guard<H>, Error> {
        Self::register_handler(15, handler)
    }

    pub fn register_irq_handler<H: 'static + HandleException>(irq: u8, handler: &H) -> Result<Guard<H>, Error> {        
        Self::register_handler(irq + 16, handler)
    }

    #[inline]
    pub unsafe fn dispatch(exc_num: u8) -> bool {
        let mut handled: bool = false;
        let exc_handlers = Self::handlers();
        for i in 0..exc_handlers.len() {
            if let Some(handler) = exc_handlers[i] {                    
                if handler.exc_num == exc_num {
                    (*handler.handler).handle_exception(Exception::from(exc_num));
                    handled = true;
                }
            }
        }
        handled
    }
}
