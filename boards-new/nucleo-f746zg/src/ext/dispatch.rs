
// pub trait RegisterExc {
//     type Handle;
//     fn register_exc<H: 'static + HandleIrq>(&self, irq: u8, handler: H) -> Result<Self::Handle, RegisterError>;
// }

// pub trait RegisterIrq {
//     type Handle;
//     fn register_irq<H: 'static + HandleIrq>(&self, irq: u8, handler: H) -> Result<Self::Handle, RegisterError>;
// }

// pub trait EnableIrq {
//     fn enable_irq(&self, irq: u8);
// }
// pub trait DisableIrq {
//     fn disable_irq(&self, irq: u8);
// }