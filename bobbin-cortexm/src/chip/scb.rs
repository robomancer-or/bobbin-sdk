//! Nested Vectored Interrupt Controller
#[allow(unused_imports)] use bobbin_common::*;

periph!(ScbPeriph, SCB, Scb, 0xe000e000);

#[doc="Nested Vectored Interrupt Controller"]
pub trait ScbPeriph : Base {
#[doc="Get the *const pointer for the ACTLR register."]
   #[inline] fn actlr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Get the *mut pointer for the ACTLR register."]
   #[inline] fn actlr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Read the ACTLR register."]
   #[inline] fn actlr(&self) -> Actlr { 
      unsafe {
         Actlr(::core::ptr::read_volatile((self.base() + 0x8) as *const u32))
      }
   }
#[doc="Write the ACTLR register."]
   #[inline] fn set_actlr<F: FnOnce(Actlr) -> Actlr>(&self, f: F) -> &Self {
      let value = f(Actlr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the ACTLR register."]
   #[inline] fn with_actlr<F: FnOnce(Actlr) -> Actlr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Actlr(::core::ptr::read_volatile((self.base() + 0x8) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x8) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CPUID register."]
   #[inline] fn cpuid_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xd00)
   }
#[doc="Get the *mut pointer for the CPUID register."]
   #[inline] fn cpuid_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xd00)
   }
#[doc="Read the CPUID register."]
   #[inline] fn cpuid(&self) -> Cpuid { 
      unsafe {
         Cpuid(::core::ptr::read_volatile((self.base() + 0xd00) as *const u32))
      }
   }
#[doc="Write the CPUID register."]
   #[inline] fn set_cpuid<F: FnOnce(Cpuid) -> Cpuid>(&self, f: F) -> &Self {
      let value = f(Cpuid(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd00) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CPUID register."]
   #[inline] fn with_cpuid<F: FnOnce(Cpuid) -> Cpuid>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Cpuid(::core::ptr::read_volatile((self.base() + 0xd00) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd00) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the ICSR register."]
   #[inline] fn icsr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xd04)
   }
#[doc="Get the *mut pointer for the ICSR register."]
   #[inline] fn icsr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xd04)
   }
#[doc="Read the ICSR register."]
   #[inline] fn icsr(&self) -> Icsr { 
      unsafe {
         Icsr(::core::ptr::read_volatile((self.base() + 0xd04) as *const u32))
      }
   }
#[doc="Write the ICSR register."]
   #[inline] fn set_icsr<F: FnOnce(Icsr) -> Icsr>(&self, f: F) -> &Self {
      let value = f(Icsr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd04) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the ICSR register."]
   #[inline] fn with_icsr<F: FnOnce(Icsr) -> Icsr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Icsr(::core::ptr::read_volatile((self.base() + 0xd04) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd04) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the VTOR register."]
   #[inline] fn vtor_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xd08)
   }
#[doc="Get the *mut pointer for the VTOR register."]
   #[inline] fn vtor_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xd08)
   }
#[doc="Read the VTOR register."]
   #[inline] fn vtor(&self) -> Vtor { 
      unsafe {
         Vtor(::core::ptr::read_volatile((self.base() + 0xd08) as *const u32))
      }
   }
#[doc="Write the VTOR register."]
   #[inline] fn set_vtor<F: FnOnce(Vtor) -> Vtor>(&self, f: F) -> &Self {
      let value = f(Vtor(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd08) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the VTOR register."]
   #[inline] fn with_vtor<F: FnOnce(Vtor) -> Vtor>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Vtor(::core::ptr::read_volatile((self.base() + 0xd08) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd08) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the AIRCR register."]
   #[inline] fn aircr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xd0c)
   }
#[doc="Get the *mut pointer for the AIRCR register."]
   #[inline] fn aircr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xd0c)
   }
#[doc="Read the AIRCR register."]
   #[inline] fn aircr(&self) -> Aircr { 
      unsafe {
         Aircr(::core::ptr::read_volatile((self.base() + 0xd0c) as *const u32))
      }
   }
#[doc="Write the AIRCR register."]
   #[inline] fn set_aircr<F: FnOnce(Aircr) -> Aircr>(&self, f: F) -> &Self {
      let value = f(Aircr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd0c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the AIRCR register."]
   #[inline] fn with_aircr<F: FnOnce(Aircr) -> Aircr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Aircr(::core::ptr::read_volatile((self.base() + 0xd0c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd0c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SCR register."]
   #[inline] fn scr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xd10)
   }
#[doc="Get the *mut pointer for the SCR register."]
   #[inline] fn scr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xd10)
   }
#[doc="Read the SCR register."]
   #[inline] fn scr(&self) -> Scr { 
      unsafe {
         Scr(::core::ptr::read_volatile((self.base() + 0xd10) as *const u32))
      }
   }
#[doc="Write the SCR register."]
   #[inline] fn set_scr<F: FnOnce(Scr) -> Scr>(&self, f: F) -> &Self {
      let value = f(Scr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd10) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SCR register."]
   #[inline] fn with_scr<F: FnOnce(Scr) -> Scr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Scr(::core::ptr::read_volatile((self.base() + 0xd10) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd10) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CCR register."]
   #[inline] fn ccr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xd14)
   }
#[doc="Get the *mut pointer for the CCR register."]
   #[inline] fn ccr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xd14)
   }
#[doc="Read the CCR register."]
   #[inline] fn ccr(&self) -> Ccr { 
      unsafe {
         Ccr(::core::ptr::read_volatile((self.base() + 0xd14) as *const u32))
      }
   }
#[doc="Write the CCR register."]
   #[inline] fn set_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
      let value = f(Ccr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd14) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CCR register."]
   #[inline] fn with_ccr<F: FnOnce(Ccr) -> Ccr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ccr(::core::ptr::read_volatile((self.base() + 0xd14) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd14) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SHPR1 register."]
   #[inline] fn shpr1_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xd18)
   }
#[doc="Get the *mut pointer for the SHPR1 register."]
   #[inline] fn shpr1_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xd18)
   }
#[doc="Read the SHPR1 register."]
   #[inline] fn shpr1(&self) -> Shpr1 { 
      unsafe {
         Shpr1(::core::ptr::read_volatile((self.base() + 0xd18) as *const u32))
      }
   }
#[doc="Write the SHPR1 register."]
   #[inline] fn set_shpr1<F: FnOnce(Shpr1) -> Shpr1>(&self, f: F) -> &Self {
      let value = f(Shpr1(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd18) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SHPR1 register."]
   #[inline] fn with_shpr1<F: FnOnce(Shpr1) -> Shpr1>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Shpr1(::core::ptr::read_volatile((self.base() + 0xd18) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd18) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SHPR2 register."]
   #[inline] fn shpr2_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xd1c)
   }
#[doc="Get the *mut pointer for the SHPR2 register."]
   #[inline] fn shpr2_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xd1c)
   }
#[doc="Read the SHPR2 register."]
   #[inline] fn shpr2(&self) -> Shpr2 { 
      unsafe {
         Shpr2(::core::ptr::read_volatile((self.base() + 0xd1c) as *const u32))
      }
   }
#[doc="Write the SHPR2 register."]
   #[inline] fn set_shpr2<F: FnOnce(Shpr2) -> Shpr2>(&self, f: F) -> &Self {
      let value = f(Shpr2(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd1c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SHPR2 register."]
   #[inline] fn with_shpr2<F: FnOnce(Shpr2) -> Shpr2>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Shpr2(::core::ptr::read_volatile((self.base() + 0xd1c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd1c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SHPR3 register."]
   #[inline] fn shpr3_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xd20)
   }
#[doc="Get the *mut pointer for the SHPR3 register."]
   #[inline] fn shpr3_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xd20)
   }
#[doc="Read the SHPR3 register."]
   #[inline] fn shpr3(&self) -> Shpr3 { 
      unsafe {
         Shpr3(::core::ptr::read_volatile((self.base() + 0xd20) as *const u32))
      }
   }
#[doc="Write the SHPR3 register."]
   #[inline] fn set_shpr3<F: FnOnce(Shpr3) -> Shpr3>(&self, f: F) -> &Self {
      let value = f(Shpr3(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd20) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SHPR3 register."]
   #[inline] fn with_shpr3<F: FnOnce(Shpr3) -> Shpr3>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Shpr3(::core::ptr::read_volatile((self.base() + 0xd20) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd20) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SHCSR register."]
   #[inline] fn shcsr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xd24)
   }
#[doc="Get the *mut pointer for the SHCSR register."]
   #[inline] fn shcsr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xd24)
   }
#[doc="Read the SHCSR register."]
   #[inline] fn shcsr(&self) -> Shcsr { 
      unsafe {
         Shcsr(::core::ptr::read_volatile((self.base() + 0xd24) as *const u32))
      }
   }
#[doc="Write the SHCSR register."]
   #[inline] fn set_shcsr<F: FnOnce(Shcsr) -> Shcsr>(&self, f: F) -> &Self {
      let value = f(Shcsr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd24) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SHCSR register."]
   #[inline] fn with_shcsr<F: FnOnce(Shcsr) -> Shcsr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Shcsr(::core::ptr::read_volatile((self.base() + 0xd24) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd24) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CFSR register."]
   #[inline] fn cfsr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xd28)
   }
#[doc="Get the *mut pointer for the CFSR register."]
   #[inline] fn cfsr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xd28)
   }
#[doc="Read the CFSR register."]
   #[inline] fn cfsr(&self) -> Cfsr { 
      unsafe {
         Cfsr(::core::ptr::read_volatile((self.base() + 0xd28) as *const u32))
      }
   }
#[doc="Write the CFSR register."]
   #[inline] fn set_cfsr<F: FnOnce(Cfsr) -> Cfsr>(&self, f: F) -> &Self {
      let value = f(Cfsr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd28) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CFSR register."]
   #[inline] fn with_cfsr<F: FnOnce(Cfsr) -> Cfsr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Cfsr(::core::ptr::read_volatile((self.base() + 0xd28) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd28) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MMFSR register."]
   #[inline] fn mmfsr_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0xd28)
   }
#[doc="Get the *mut pointer for the MMFSR register."]
   #[inline] fn mmfsr_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0xd28)
   }
#[doc="Read the MMFSR register."]
   #[inline] fn mmfsr(&self) -> Mmfsr { 
      unsafe {
         Mmfsr(::core::ptr::read_volatile((self.base() + 0xd28) as *const u8))
      }
   }
#[doc="Write the MMFSR register."]
   #[inline] fn set_mmfsr<F: FnOnce(Mmfsr) -> Mmfsr>(&self, f: F) -> &Self {
      let value = f(Mmfsr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd28) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the MMFSR register."]
   #[inline] fn with_mmfsr<F: FnOnce(Mmfsr) -> Mmfsr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Mmfsr(::core::ptr::read_volatile((self.base() + 0xd28) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd28) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the BFSR register."]
   #[inline] fn bfsr_ptr(&self) -> *const u8 { 
       <Self as Base>::addr(&self, 0xd29)
   }
#[doc="Get the *mut pointer for the BFSR register."]
   #[inline] fn bfsr_mut(&self) -> *mut u8 { 
       <Self as Base>::addr(&self, 0xd29)
   }
#[doc="Read the BFSR register."]
   #[inline] fn bfsr(&self) -> Bfsr { 
      unsafe {
         Bfsr(::core::ptr::read_volatile((self.base() + 0xd29) as *const u8))
      }
   }
#[doc="Write the BFSR register."]
   #[inline] fn set_bfsr<F: FnOnce(Bfsr) -> Bfsr>(&self, f: F) -> &Self {
      let value = f(Bfsr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd29) as *mut u8, value.0);
      }
      self
   }
#[doc="Modify the BFSR register."]
   #[inline] fn with_bfsr<F: FnOnce(Bfsr) -> Bfsr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Bfsr(::core::ptr::read_volatile((self.base() + 0xd29) as *const u8))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd29) as *mut u8, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the UFSR register."]
   #[inline] fn ufsr_ptr(&self) -> *const u16 { 
       <Self as Base>::addr(&self, 0xd2a)
   }
#[doc="Get the *mut pointer for the UFSR register."]
   #[inline] fn ufsr_mut(&self) -> *mut u16 { 
       <Self as Base>::addr(&self, 0xd2a)
   }
#[doc="Read the UFSR register."]
   #[inline] fn ufsr(&self) -> Ufsr { 
      unsafe {
         Ufsr(::core::ptr::read_volatile((self.base() + 0xd2a) as *const u16))
      }
   }
#[doc="Write the UFSR register."]
   #[inline] fn set_ufsr<F: FnOnce(Ufsr) -> Ufsr>(&self, f: F) -> &Self {
      let value = f(Ufsr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd2a) as *mut u16, value.0);
      }
      self
   }
#[doc="Modify the UFSR register."]
   #[inline] fn with_ufsr<F: FnOnce(Ufsr) -> Ufsr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ufsr(::core::ptr::read_volatile((self.base() + 0xd2a) as *const u16))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd2a) as *mut u16, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the HFSR register."]
   #[inline] fn hfsr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xd2c)
   }
#[doc="Get the *mut pointer for the HFSR register."]
   #[inline] fn hfsr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xd2c)
   }
#[doc="Read the HFSR register."]
   #[inline] fn hfsr(&self) -> Hfsr { 
      unsafe {
         Hfsr(::core::ptr::read_volatile((self.base() + 0xd2c) as *const u32))
      }
   }
#[doc="Write the HFSR register."]
   #[inline] fn set_hfsr<F: FnOnce(Hfsr) -> Hfsr>(&self, f: F) -> &Self {
      let value = f(Hfsr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd2c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the HFSR register."]
   #[inline] fn with_hfsr<F: FnOnce(Hfsr) -> Hfsr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Hfsr(::core::ptr::read_volatile((self.base() + 0xd2c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd2c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the MMFAR register."]
   #[inline] fn mmfar_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xd34)
   }
#[doc="Get the *mut pointer for the MMFAR register."]
   #[inline] fn mmfar_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xd34)
   }
#[doc="Read the MMFAR register."]
   #[inline] fn mmfar(&self) -> Mmfar { 
      unsafe {
         Mmfar(::core::ptr::read_volatile((self.base() + 0xd34) as *const u32))
      }
   }
#[doc="Write the MMFAR register."]
   #[inline] fn set_mmfar<F: FnOnce(Mmfar) -> Mmfar>(&self, f: F) -> &Self {
      let value = f(Mmfar(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd34) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the MMFAR register."]
   #[inline] fn with_mmfar<F: FnOnce(Mmfar) -> Mmfar>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Mmfar(::core::ptr::read_volatile((self.base() + 0xd34) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd34) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the BFAR register."]
   #[inline] fn bfar_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xd38)
   }
#[doc="Get the *mut pointer for the BFAR register."]
   #[inline] fn bfar_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xd38)
   }
#[doc="Read the BFAR register."]
   #[inline] fn bfar(&self) -> Bfar { 
      unsafe {
         Bfar(::core::ptr::read_volatile((self.base() + 0xd38) as *const u32))
      }
   }
#[doc="Write the BFAR register."]
   #[inline] fn set_bfar<F: FnOnce(Bfar) -> Bfar>(&self, f: F) -> &Self {
      let value = f(Bfar(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd38) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the BFAR register."]
   #[inline] fn with_bfar<F: FnOnce(Bfar) -> Bfar>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Bfar(::core::ptr::read_volatile((self.base() + 0xd38) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd38) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the AFSR register."]
   #[inline] fn afsr_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xd3c)
   }
#[doc="Get the *mut pointer for the AFSR register."]
   #[inline] fn afsr_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xd3c)
   }
#[doc="Read the AFSR register."]
   #[inline] fn afsr(&self) -> Afsr { 
      unsafe {
         Afsr(::core::ptr::read_volatile((self.base() + 0xd3c) as *const u32))
      }
   }
#[doc="Write the AFSR register."]
   #[inline] fn set_afsr<F: FnOnce(Afsr) -> Afsr>(&self, f: F) -> &Self {
      let value = f(Afsr(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd3c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the AFSR register."]
   #[inline] fn with_afsr<F: FnOnce(Afsr) -> Afsr>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Afsr(::core::ptr::read_volatile((self.base() + 0xd3c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xd3c) as *mut u32, value.0);
      }
      self
   }

}

#[doc="Auxiliary Control Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Actlr(pub u32);
impl Actlr {
#[doc="When set to 1, disables IT folding."]
   #[inline] pub fn disfold(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="When set to 1, disables IT folding."]
   #[inline] pub fn set_disfold<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="When set to 1, disables write buffer use during default memory map accesses."]
   #[inline] pub fn disdefwbuf(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="When set to 1, disables write buffer use during default memory map accesses."]
   #[inline] pub fn set_disdefwbuf<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="When set to 1, disables interruption of load multiple and store multiple instructions."]
   #[inline] pub fn dismcycint(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="When set to 1, disables interruption of load multiple and store multiple instructions."]
   #[inline] pub fn set_dismcycint<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Actlr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Actlr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.disfold() != 0 { try!(write!(f, " disfold"))}
      if self.disdefwbuf() != 0 { try!(write!(f, " disdefwbuf"))}
      if self.dismcycint() != 0 { try!(write!(f, " dismcycint"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="CPUID Base Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cpuid(pub u32);
impl Cpuid {
#[doc="Implementer Code"]
   #[inline] pub fn implementer(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
   }
#[doc="Implementer Code"]
   #[inline] pub fn set_implementer<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Variant number, the r value in the rnpn product revision identifier"]
   #[inline] pub fn variant(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
   }
#[doc="Variant number, the r value in the rnpn product revision identifier"]
   #[inline] pub fn set_variant<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 20);
      self.0 |= value << 20;
      self
   }

#[doc="Reads as 0xF"]
   #[inline] pub fn constant(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
   }
#[doc="Reads as 0xF"]
   #[inline] pub fn set_constant<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Part number of the processor"]
   #[inline] pub fn partno(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xfff) as u16) } // [15:4]
   }
#[doc="Part number of the processor"]
   #[inline] pub fn set_partno<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Revision number, the p value in the rnpn product revision identifier"]
   #[inline] pub fn revision(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="Revision number, the p value in the rnpn product revision identifier"]
   #[inline] pub fn set_revision<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Cpuid {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cpuid {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.implementer() != 0 { try!(write!(f, " implementer=0x{:x}", self.implementer()))}
      if self.variant() != 0 { try!(write!(f, " variant=0x{:x}", self.variant()))}
      if self.constant() != 0 { try!(write!(f, " constant=0x{:x}", self.constant()))}
      if self.partno() != 0 { try!(write!(f, " partno=0x{:x}", self.partno()))}
      if self.revision() != 0 { try!(write!(f, " revision=0x{:x}", self.revision()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Interrupt Control and State Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Icsr(pub u32);
impl Icsr {
#[doc="NMI set-pending bit"]
   #[inline] pub fn nmipendset(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="NMI set-pending bit"]
   #[inline] pub fn set_nmipendset<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

#[doc="PendSV set-pending bit"]
   #[inline] pub fn pendsvset(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
   }
#[doc="PendSV set-pending bit"]
   #[inline] pub fn set_pendsvset<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 28);
      self.0 |= value << 28;
      self
   }

#[doc="PendSV clear-pending bit"]
   #[inline] pub fn pendsvclr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
   }
#[doc="PendSV clear-pending bit"]
   #[inline] pub fn set_pendsvclr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 27);
      self.0 |= value << 27;
      self
   }

#[doc="Systick exception set-pending bit"]
   #[inline] pub fn pendstset(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
   }
#[doc="Systick exception set-pending bit"]
   #[inline] pub fn set_pendstset<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 26);
      self.0 |= value << 26;
      self
   }

#[doc="Systick clear-pending bit"]
   #[inline] pub fn pendstclr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
   }
#[doc="Systick clear-pending bit"]
   #[inline] pub fn set_pendstclr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 25);
      self.0 |= value << 25;
      self
   }

#[doc="Interrupt pending flag, excluding NMI and Faults"]
   #[inline] pub fn isrpending(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
   }
#[doc="Interrupt pending flag, excluding NMI and Faults"]
   #[inline] pub fn set_isrpending<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 22);
      self.0 |= value << 22;
      self
   }

#[doc="Indicates the exception number of the highest priority pending enabled exception"]
   #[inline] pub fn vectpending(&self) -> bits::U6 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3f) as u8) } // [17:12]
   }
#[doc="Indicates the exception number of the highest priority pending enabled exception"]
   #[inline] pub fn set_vectpending<V: Into<bits::U6>>(mut self, value: V) -> Self {
      let value: bits::U6 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3f << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Indicates the exception number of the highest priority pending enabled exception"]
   #[inline] pub fn rettobase(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="Indicates the exception number of the highest priority pending enabled exception"]
   #[inline] pub fn set_rettobase<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="Contains the active exception number. Subtract 16 from this value to obtain the CMSIS IRQ number required to index into the Interrupt Clear-Enable, Set-Enable, Clear-Pending, Set-Pending, or Priority Registers"]
   #[inline] pub fn vectactive(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Contains the active exception number. Subtract 16 from this value to obtain the CMSIS IRQ number required to index into the Interrupt Clear-Enable, Set-Enable, Clear-Pending, Set-Pending, or Priority Registers"]
   #[inline] pub fn set_vectactive<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Icsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Icsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.nmipendset() != 0 { try!(write!(f, " nmipendset"))}
      if self.pendsvset() != 0 { try!(write!(f, " pendsvset"))}
      if self.pendsvclr() != 0 { try!(write!(f, " pendsvclr"))}
      if self.pendstset() != 0 { try!(write!(f, " pendstset"))}
      if self.pendstclr() != 0 { try!(write!(f, " pendstclr"))}
      if self.isrpending() != 0 { try!(write!(f, " isrpending"))}
      if self.vectpending() != 0 { try!(write!(f, " vectpending=0x{:x}", self.vectpending()))}
      if self.rettobase() != 0 { try!(write!(f, " rettobase"))}
      if self.vectactive() != 0 { try!(write!(f, " vectactive=0x{:x}", self.vectactive()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Vector Table Offset Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Vtor(pub u32);
impl Vtor {
#[doc="Vector table base offset field. It contains bits[29:7] of the offset of the table base from the bottom of the memory map."]
   #[inline] pub fn tbloff(&self) -> bits::U25 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1ffffff) as u32) } // [31:7]
   }
#[doc="Vector table base offset field. It contains bits[29:7] of the offset of the table base from the bottom of the memory map."]
   #[inline] pub fn set_tbloff<V: Into<bits::U25>>(mut self, value: V) -> Self {
      let value: bits::U25 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1ffffff << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for Vtor {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Vtor {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tbloff() != 0 { try!(write!(f, " tbloff=0x{:x}", self.tbloff()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Application Interrupt and Reset Control Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Aircr(pub u32);
impl Aircr {
#[doc="Register key: Reads as 0xFA05. On writes, write 0x5FA to VECTKEY, otherwise the write is ignored."]
   #[inline] pub fn vectkey(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }
#[doc="Register key: Reads as 0xFA05. On writes, write 0x5FA to VECTKEY, otherwise the write is ignored."]
   #[inline] pub fn set_vectkey<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Data endianness bit is implementation defined: 0 = Little-endian, 1 = Big-endian."]
   #[inline] pub fn endianness(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="Data endianness bit is implementation defined: 0 = Little-endian, 1 = Big-endian."]
   #[inline] pub fn set_endianness<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

#[doc="Interrupt priority grouping field is implementation defined. This field determines the split of group priority from subpriority, see Binary point."]
   #[inline] pub fn prigroup(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
   }
#[doc="Interrupt priority grouping field is implementation defined. This field determines the split of group priority from subpriority, see Binary point."]
   #[inline] pub fn set_prigroup<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="System reset request bit is implementation defined: 0 = no system reset request, 1 = asserts a signal to the outer system that requests a reset. This is intended to force a large system reset of all major components except for debug."]
   #[inline] pub fn sysresetreq(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="System reset request bit is implementation defined: 0 = no system reset request, 1 = asserts a signal to the outer system that requests a reset. This is intended to force a large system reset of all major components except for debug."]
   #[inline] pub fn set_sysresetreq<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Reserved for Debug use. This bit reads as 0. When writing to the register you must write 0 to this bit, otherwise behavior is Unpredictable."]
   #[inline] pub fn vectclractive(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Reserved for Debug use. This bit reads as 0. When writing to the register you must write 0 to this bit, otherwise behavior is Unpredictable."]
   #[inline] pub fn set_vectclractive<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Reserved for Debug use. This bit reads as 0. When writing to the register you must write 0 to this bit, otherwise behavior is Unpredictable."]
   #[inline] pub fn vectreset(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Reserved for Debug use. This bit reads as 0. When writing to the register you must write 0 to this bit, otherwise behavior is Unpredictable."]
   #[inline] pub fn set_vectreset<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Aircr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Aircr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.vectkey() != 0 { try!(write!(f, " vectkey=0x{:x}", self.vectkey()))}
      if self.endianness() != 0 { try!(write!(f, " endianness"))}
      if self.prigroup() != 0 { try!(write!(f, " prigroup=0x{:x}", self.prigroup()))}
      if self.sysresetreq() != 0 { try!(write!(f, " sysresetreq"))}
      if self.vectclractive() != 0 { try!(write!(f, " vectclractive"))}
      if self.vectreset() != 0 { try!(write!(f, " vectreset"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="System Control Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scr(pub u32);
impl Scr {
#[doc="Send Event on Pending bit"]
   #[inline] pub fn sevonpend(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Send Event on Pending bit"]
   #[inline] pub fn set_sevonpend<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Controls whether the processor uses sleep or deep sleep as its low power mode"]
   #[inline] pub fn sleepdeep(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Controls whether the processor uses sleep or deep sleep as its low power mode"]
   #[inline] pub fn set_sleepdeep<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Indicates sleep-on-exit when returning from Handler mode to Thread mode:"]
   #[inline] pub fn sleeponexit(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Indicates sleep-on-exit when returning from Handler mode to Thread mode:"]
   #[inline] pub fn set_sleeponexit<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Scr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Scr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sevonpend() != 0 { try!(write!(f, " sevonpend"))}
      if self.sleepdeep() != 0 { try!(write!(f, " sleepdeep"))}
      if self.sleeponexit() != 0 { try!(write!(f, " sleeponexit"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Configuration and Control Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ccr(pub u32);
impl Ccr {
#[doc="Indicates stack alignment on exception entry: 0 = 4-byte aligned1 = 8-byte aligned. On exception entry, the processor uses bit[9] of the stacked PSR to indicate the stack alignment. On return from the exception it uses this stacked bit to restore the correct stack alignment."]
   #[inline] pub fn stkalign(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="Indicates stack alignment on exception entry: 0 = 4-byte aligned1 = 8-byte aligned. On exception entry, the processor uses bit[9] of the stacked PSR to indicate the stack alignment. On return from the exception it uses this stacked bit to restore the correct stack alignment."]
   #[inline] pub fn set_stkalign<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions. This applies to the hard fault, NMI, and FAULTMASK escalated handlers: 0 = data bus faults caused by load and store instructions cause a lock-up, 1 = handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions. Set this bit to 1 only when the handler and its data are in absolutely safe memory. The normal use of this bit is to probe system devices and bridges to detect control path problems and fix them."]
   #[inline] pub fn bfhfnmign(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions. This applies to the hard fault, NMI, and FAULTMASK escalated handlers: 0 = data bus faults caused by load and store instructions cause a lock-up, 1 = handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions. Set this bit to 1 only when the handler and its data are in absolutely safe memory. The normal use of this bit is to probe system devices and bridges to detect control path problems and fix them."]
   #[inline] pub fn set_bfhfnmign<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0: 0 = do not trap divide by 0, 1 = trap divide by 0. When this bit is set to 0, a divide by zero returns a quotient of 0."]
   #[inline] pub fn div_0_trp(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0: 0 = do not trap divide by 0, 1 = trap divide by 0. When this bit is set to 0, a divide by zero returns a quotient of 0."]
   #[inline] pub fn set_div_0_trp<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Enables unaligned access traps: 0 = do not trap unaligned halfword and word accesses1 = trap unaligned halfword and word accesses. If this bit is set to 1, an unaligned access generates a UsageFault. Unaligned LDM, STM, LDRD, and STRD instructions always fault irrespective of whether UNALIGN_TRP is set to 1."]
   #[inline] pub fn unalign_trp(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Enables unaligned access traps: 0 = do not trap unaligned halfword and word accesses1 = trap unaligned halfword and word accesses. If this bit is set to 1, an unaligned access generates a UsageFault. Unaligned LDM, STM, LDRD, and STRD instructions always fault irrespective of whether UNALIGN_TRP is set to 1."]
   #[inline] pub fn set_unalign_trp<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Enables unprivileged software access to the STIR, see Software Trigger Interrupt Register: 0 = disable, 1 = enable."]
   #[inline] pub fn usersetmpend(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Enables unprivileged software access to the STIR, see Software Trigger Interrupt Register: 0 = disable, 1 = enable."]
   #[inline] pub fn set_usersetmpend<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Indicates how the processor enters Thread mode: 0 = processor can enter Thread mode only when no exception is active, 1 = processor can enter Thread mode from any level under the control of an EXC_RETURN value, see Exception return."]
   #[inline] pub fn nonbasethrdena(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Indicates how the processor enters Thread mode: 0 = processor can enter Thread mode only when no exception is active, 1 = processor can enter Thread mode from any level under the control of an EXC_RETURN value, see Exception return."]
   #[inline] pub fn set_nonbasethrdena<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
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
      if self.stkalign() != 0 { try!(write!(f, " stkalign"))}
      if self.bfhfnmign() != 0 { try!(write!(f, " bfhfnmign"))}
      if self.div_0_trp() != 0 { try!(write!(f, " div_0_trp"))}
      if self.unalign_trp() != 0 { try!(write!(f, " unalign_trp"))}
      if self.usersetmpend() != 0 { try!(write!(f, " usersetmpend"))}
      if self.nonbasethrdena() != 0 { try!(write!(f, " nonbasethrdena"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="System Handler Priority Register 1"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Shpr1(pub u32);
impl Shpr1 {
#[doc="Priority of system handler 6, UsageFault"]
   #[inline] pub fn pri_6(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }
#[doc="Priority of system handler 6, UsageFault"]
   #[inline] pub fn set_pri_6<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Priority of system handler 5, BusFault"]
   #[inline] pub fn pri_5(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
   }
#[doc="Priority of system handler 5, BusFault"]
   #[inline] pub fn set_pri_5<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Priority of system handler 4, MemManage"]
   #[inline] pub fn pri_4(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
   }
#[doc="Priority of system handler 4, MemManage"]
   #[inline] pub fn set_pri_4<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Shpr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Shpr1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pri_6() != 0 { try!(write!(f, " pri_6=0x{:x}", self.pri_6()))}
      if self.pri_5() != 0 { try!(write!(f, " pri_5=0x{:x}", self.pri_5()))}
      if self.pri_4() != 0 { try!(write!(f, " pri_4=0x{:x}", self.pri_4()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="System Handler Priority Register 2"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Shpr2(pub u32);
impl Shpr2 {
#[doc="Priority of system handler 11, SVCall"]
   #[inline] pub fn pri_11(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
   }
#[doc="Priority of system handler 11, SVCall"]
   #[inline] pub fn set_pri_11<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 24);
      self.0 |= value << 24;
      self
   }

}
impl ::core::fmt::Display for Shpr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Shpr2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pri_11() != 0 { try!(write!(f, " pri_11=0x{:x}", self.pri_11()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="System Handler Priority Register 3"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Shpr3(pub u32);
impl Shpr3 {
#[doc="Priority of system handler 15, SysTick exception"]
   #[inline] pub fn pri_15(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
   }
#[doc="Priority of system handler 15, SysTick exception"]
   #[inline] pub fn set_pri_15<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 24);
      self.0 |= value << 24;
      self
   }

#[doc="Priority of system handler 14, PendSV"]
   #[inline] pub fn pri_14(&self) -> bits::U8 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
   }
#[doc="Priority of system handler 14, PendSV"]
   #[inline] pub fn set_pri_14<V: Into<bits::U8>>(mut self, value: V) -> Self {
      let value: bits::U8 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xff << 16);
      self.0 |= value << 16;
      self
   }

}
impl ::core::fmt::Display for Shpr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Shpr3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pri_15() != 0 { try!(write!(f, " pri_15=0x{:x}", self.pri_15()))}
      if self.pri_14() != 0 { try!(write!(f, " pri_14=0x{:x}", self.pri_14()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="System Handler Control and State Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Shcsr(pub u32);
impl Shcsr {
#[doc="UsageFault enable bit, set to 1 to enable"]
   #[inline] pub fn usgfaultena(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
   }
#[doc="UsageFault enable bit, set to 1 to enable"]
   #[inline] pub fn set_usgfaultena<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 18);
      self.0 |= value << 18;
      self
   }

#[doc="BusFault enable bit, set to 1 to enable"]
   #[inline] pub fn busfaultena(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="BusFault enable bit, set to 1 to enable"]
   #[inline] pub fn set_busfaultena<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

#[doc="MemManage enable bit, set to 1 to enable"]
   #[inline] pub fn memfaultena(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="MemManage enable bit, set to 1 to enable"]
   #[inline] pub fn set_memfaultena<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="SVCall pending bit, reads as 1 if exception is pending"]
   #[inline] pub fn svcallpended(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
   }
#[doc="SVCall pending bit, reads as 1 if exception is pending"]
   #[inline] pub fn set_svcallpended<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 15);
      self.0 |= value << 15;
      self
   }

#[doc="BusFault exception pending bit, reads as 1 if exception is pending"]
   #[inline] pub fn busfaultpended(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
   }
#[doc="BusFault exception pending bit, reads as 1 if exception is pending"]
   #[inline] pub fn set_busfaultpended<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="MemManage exception pending bit, reads as 1 if exception is pending"]
   #[inline] pub fn memfaultpended(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="MemManage exception pending bit, reads as 1 if exception is pending"]
   #[inline] pub fn set_memfaultpended<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

#[doc="UsageFault exception pending bit, reads as 1 if exception is pending"]
   #[inline] pub fn usgfaultpended(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="UsageFault exception pending bit, reads as 1 if exception is pending"]
   #[inline] pub fn set_usgfaultpended<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="SysTick exception active bit, reads as 1 if exception is active"]
   #[inline] pub fn systickact(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="SysTick exception active bit, reads as 1 if exception is active"]
   #[inline] pub fn set_systickact<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="PendSV exception active bit, reads as 1 if exception is active"]
   #[inline] pub fn pendsvact(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="PendSV exception active bit, reads as 1 if exception is active"]
   #[inline] pub fn set_pendsvact<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="Debug monitor active bit, reads as 1 if Debug monitor is active"]
   #[inline] pub fn monitoract(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Debug monitor active bit, reads as 1 if Debug monitor is active"]
   #[inline] pub fn set_monitoract<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="SVCall active bit, reads as 1 if SVC call is active"]
   #[inline] pub fn svcallact(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="SVCall active bit, reads as 1 if SVC call is active"]
   #[inline] pub fn set_svcallact<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="UsageFault exception active bit, reads as 1 if exception is active"]
   #[inline] pub fn usgfaultact(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="UsageFault exception active bit, reads as 1 if exception is active"]
   #[inline] pub fn set_usgfaultact<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="BusFault exception active bit, reads as 1 if exception is active"]
   #[inline] pub fn busfaultact(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="BusFault exception active bit, reads as 1 if exception is active"]
   #[inline] pub fn set_busfaultact<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="MemManage exception active bit, reads as 1 if exception is active"]
   #[inline] pub fn memfaultact(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="MemManage exception active bit, reads as 1 if exception is active"]
   #[inline] pub fn set_memfaultact<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Shcsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Shcsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.usgfaultena() != 0 { try!(write!(f, " usgfaultena"))}
      if self.busfaultena() != 0 { try!(write!(f, " busfaultena"))}
      if self.memfaultena() != 0 { try!(write!(f, " memfaultena"))}
      if self.svcallpended() != 0 { try!(write!(f, " svcallpended"))}
      if self.busfaultpended() != 0 { try!(write!(f, " busfaultpended"))}
      if self.memfaultpended() != 0 { try!(write!(f, " memfaultpended"))}
      if self.usgfaultpended() != 0 { try!(write!(f, " usgfaultpended"))}
      if self.systickact() != 0 { try!(write!(f, " systickact"))}
      if self.pendsvact() != 0 { try!(write!(f, " pendsvact"))}
      if self.monitoract() != 0 { try!(write!(f, " monitoract"))}
      if self.svcallact() != 0 { try!(write!(f, " svcallact"))}
      if self.usgfaultact() != 0 { try!(write!(f, " usgfaultact"))}
      if self.busfaultact() != 0 { try!(write!(f, " busfaultact"))}
      if self.memfaultact() != 0 { try!(write!(f, " memfaultact"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Configurable Fault Status Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cfsr(pub u32);
impl Cfsr {
}
impl ::core::fmt::Display for Cfsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cfsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="MemManage Fault Status Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mmfsr(pub u8);
impl Mmfsr {
#[doc="MemManage Fault Address Register (MMFAR) valid flag: 0 = value in MMAR is not a valid fault address, 1 = MMAR holds a valid fault address. If a MemManage fault occurs and is escalated to a HardFault because of priority, the HardFault handler must set this bit to 0. This prevents problems on return to a stacked active MemManage fault handler whose MMAR value has been overwritten."]
   #[inline] pub fn mmarvalid(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="MemManage Fault Address Register (MMFAR) valid flag: 0 = value in MMAR is not a valid fault address, 1 = MMAR holds a valid fault address. If a MemManage fault occurs and is escalated to a HardFault because of priority, the HardFault handler must set this bit to 0. This prevents problems on return to a stacked active MemManage fault handler whose MMAR value has been overwritten."]
   #[inline] pub fn set_mmarvalid<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="MemManage fault on stacking for exception entry: 0 = no stacking fault, 1 = stacking for an exception entry has caused one or more access violations. When this bit is 1, the SP is still adjusted but the values in the context area on the stack might be incorrect. The processor has not written a fault address to the MMAR."]
   #[inline] pub fn mstkerr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="MemManage fault on stacking for exception entry: 0 = no stacking fault, 1 = stacking for an exception entry has caused one or more access violations. When this bit is 1, the SP is still adjusted but the values in the context area on the stack might be incorrect. The processor has not written a fault address to the MMAR."]
   #[inline] pub fn set_mstkerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="MemManage fault on unstacking for a return from exception: 0 = no unstacking fault, 1 = unstack for an exception return has caused one or more access violations. This fault is chained to the handler. This means that when this bit is 1, the original return stack is still present. The processor has not adjusted the SP from the failing return, and has not performed a new save. The processor has not written a fault address to the MMAR."]
   #[inline] pub fn munstkerr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="MemManage fault on unstacking for a return from exception: 0 = no unstacking fault, 1 = unstack for an exception return has caused one or more access violations. This fault is chained to the handler. This means that when this bit is 1, the original return stack is still present. The processor has not adjusted the SP from the failing return, and has not performed a new save. The processor has not written a fault address to the MMAR."]
   #[inline] pub fn set_munstkerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Data access violation flag: 0 = no data access violation fault, 1 = the processor attempted a load or store at a location that does not permit the operation. When this bit is 1, the PC value stacked for the exception return points to the faulting instruction. The processor has loaded the MMAR with the address of the attempted access."]
   #[inline] pub fn daccviol(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Data access violation flag: 0 = no data access violation fault, 1 = the processor attempted a load or store at a location that does not permit the operation. When this bit is 1, the PC value stacked for the exception return points to the faulting instruction. The processor has loaded the MMAR with the address of the attempted access."]
   #[inline] pub fn set_daccviol<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Instruction access violation flag: 0 = no instruction access violation fault, 1 = the processor attempted an instruction fetch from a location that does not permit execution. This fault occurs on any access to an XN region, even when the MPU is disabled or not present. When this bit is 1, the PC value stacked for the exception return points to the faulting instruction. The processor has not written a fault address to the MMAR."]
   #[inline] pub fn iaccviol(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Instruction access violation flag: 0 = no instruction access violation fault, 1 = the processor attempted an instruction fetch from a location that does not permit execution. This fault occurs on any access to an XN region, even when the MPU is disabled or not present. When this bit is 1, the PC value stacked for the exception return points to the faulting instruction. The processor has not written a fault address to the MMAR."]
   #[inline] pub fn set_iaccviol<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Mmfsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Mmfsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.mmarvalid() != 0 { try!(write!(f, " mmarvalid"))}
      if self.mstkerr() != 0 { try!(write!(f, " mstkerr"))}
      if self.munstkerr() != 0 { try!(write!(f, " munstkerr"))}
      if self.daccviol() != 0 { try!(write!(f, " daccviol"))}
      if self.iaccviol() != 0 { try!(write!(f, " iaccviol"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="BusFault Status Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bfsr(pub u8);
impl Bfsr {
#[doc="BusFault Address Register (BFAR) valid flag: 0 = value in BFAR is not a valid fault address, 1 = BFAR holds a valid fault address. The processor sets this bit to 1 after a BusFault where the address is known. Other faults can set this bit to 0, such as a MemManage fault occurring later. If a BusFault occurs and is escalated to a hard fault because of priority, the hard fault handler must set this bit to 0. This prevents problems if returning to a stacked active BusFault handler whose BFAR value has been overwritten."]
   #[inline] pub fn bfarvalid(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="BusFault Address Register (BFAR) valid flag: 0 = value in BFAR is not a valid fault address, 1 = BFAR holds a valid fault address. The processor sets this bit to 1 after a BusFault where the address is known. Other faults can set this bit to 0, such as a MemManage fault occurring later. If a BusFault occurs and is escalated to a hard fault because of priority, the hard fault handler must set this bit to 0. This prevents problems if returning to a stacked active BusFault handler whose BFAR value has been overwritten."]
   #[inline] pub fn set_bfarvalid<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

#[doc="BusFault on stacking for exception entry: 0 = no stacking fault, 1 = stacking for an exception entry has caused one or more BusFaults. When the processor sets this bit to 1, the SP is still adjusted but the values in the context area on the stack might be incorrect. The processor does not write a fault address to the BFAR."]
   #[inline] pub fn stkerr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="BusFault on stacking for exception entry: 0 = no stacking fault, 1 = stacking for an exception entry has caused one or more BusFaults. When the processor sets this bit to 1, the SP is still adjusted but the values in the context area on the stack might be incorrect. The processor does not write a fault address to the BFAR."]
   #[inline] pub fn set_stkerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="BusFault on unstacking for a return from exception: 0 = no unstacking fault, 1 = unstack for an exception return has caused one or more BusFaults. This fault is chained to the handler. This means that when the processor sets this bit to 1, the original return stack is still present. The processor does not adjust the SP from the failing return, does not performed a new save, and does not write a fault address to the BFAR."]
   #[inline] pub fn unstkerr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="BusFault on unstacking for a return from exception: 0 = no unstacking fault, 1 = unstack for an exception return has caused one or more BusFaults. This fault is chained to the handler. This means that when the processor sets this bit to 1, the original return stack is still present. The processor does not adjust the SP from the failing return, does not performed a new save, and does not write a fault address to the BFAR."]
   #[inline] pub fn set_unstkerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Imprecise data bus error: 0 = no imprecise data bus error, 1 = a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error. When the processor sets this bit to 1, it does not write a fault address to the BFAR. This is an asynchronous fault. Therefore, if it is detected when the priority of the current process is higher than the BusFault priority, the BusFault becomes pending and becomes active only when the processor returns from all higher priority processes. If a precise fault occurs before the processor enters the handler for the imprecise BusFault, the handler detects both IMPRECISERR set to 1 and one of the precise fault status bits set to 1."]
   #[inline] pub fn impreciserr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Imprecise data bus error: 0 = no imprecise data bus error, 1 = a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error. When the processor sets this bit to 1, it does not write a fault address to the BFAR. This is an asynchronous fault. Therefore, if it is detected when the priority of the current process is higher than the BusFault priority, the BusFault becomes pending and becomes active only when the processor returns from all higher priority processes. If a precise fault occurs before the processor enters the handler for the imprecise BusFault, the handler detects both IMPRECISERR set to 1 and one of the precise fault status bits set to 1."]
   #[inline] pub fn set_impreciserr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Precise data bus error: 0 = no precise data bus error, 1 = a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault. When the processor sets this bit is 1, it writes the faulting address to the BFAR."]
   #[inline] pub fn preciserr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Precise data bus error: 0 = no precise data bus error, 1 = a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault. When the processor sets this bit is 1, it writes the faulting address to the BFAR."]
   #[inline] pub fn set_preciserr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Instruction bus error: 0 = no instruction bus error, 1 = instruction bus error. The processor detects the instruction bus error on prefetching an instruction, but it sets the IBUSERR flag to 1 only if it attempts to issue the faulting instruction. When the processor sets this bit is 1, it does not write a fault address to the BFAR."]
   #[inline] pub fn ibuserr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Instruction bus error: 0 = no instruction bus error, 1 = instruction bus error. The processor detects the instruction bus error on prefetching an instruction, but it sets the IBUSERR flag to 1 only if it attempts to issue the faulting instruction. When the processor sets this bit is 1, it does not write a fault address to the BFAR."]
   #[inline] pub fn set_ibuserr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u8 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Bfsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Bfsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.bfarvalid() != 0 { try!(write!(f, " bfarvalid"))}
      if self.stkerr() != 0 { try!(write!(f, " stkerr"))}
      if self.unstkerr() != 0 { try!(write!(f, " unstkerr"))}
      if self.impreciserr() != 0 { try!(write!(f, " impreciserr"))}
      if self.preciserr() != 0 { try!(write!(f, " preciserr"))}
      if self.ibuserr() != 0 { try!(write!(f, " ibuserr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="UsageFault Status Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ufsr(pub u16);
impl Ufsr {
#[doc="Divide by zero UsageFault: 0 = no divide by zero fault, or divide by zero trapping not enabled, 1 = the processor has executed an SDIV or UDIV instruction with a divisor of 0. When the processor sets this bit to 1, the PC value stacked for the exception return points to the instruction that performed the divide by zero. Enable trapping of divide by zero by setting the DIV_0_TRP bit in the CCR to 1, see Configuration and Control Register."]
   #[inline] pub fn divbyzero(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="Divide by zero UsageFault: 0 = no divide by zero fault, or divide by zero trapping not enabled, 1 = the processor has executed an SDIV or UDIV instruction with a divisor of 0. When the processor sets this bit to 1, the PC value stacked for the exception return points to the instruction that performed the divide by zero. Enable trapping of divide by zero by setting the DIV_0_TRP bit in the CCR to 1, see Configuration and Control Register."]
   #[inline] pub fn set_divbyzero<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="Unaligned access UsageFault: 0 = no unaligned access fault, or unaligned access trapping not enabled, 1 = the processor has made an unaligned memory access. Enable trapping of unaligned accesses by setting the UNALIGN_TRP bit in the CCR to 1, see Configuration and Control Register. Unaligned LDM, STM, LDRD, and STRD instructions always fault irrespective of the setting of UNALIGN_TRP."]
   #[inline] pub fn unaligned(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Unaligned access UsageFault: 0 = no unaligned access fault, or unaligned access trapping not enabled, 1 = the processor has made an unaligned memory access. Enable trapping of unaligned accesses by setting the UNALIGN_TRP bit in the CCR to 1, see Configuration and Control Register. Unaligned LDM, STM, LDRD, and STRD instructions always fault irrespective of the setting of UNALIGN_TRP."]
   #[inline] pub fn set_unaligned<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="No coprocessor UsageFault. The processor does not support coprocessor instructions: 0 = no UsageFault caused by attempting to access a coprocessor, 1 = the processor has attempted to access a coprocessor."]
   #[inline] pub fn nocp(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="No coprocessor UsageFault. The processor does not support coprocessor instructions: 0 = no UsageFault caused by attempting to access a coprocessor, 1 = the processor has attempted to access a coprocessor."]
   #[inline] pub fn set_nocp<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Invalid PC load UsageFault, caused by an invalid PC load by EXC_RETURN: 0 = no invalid PC load UsageFault, 1 = the processor has attempted an illegal load of EXC_RETURN to the PC, as a result of an invalid context, or an invalid EXC_RETURN value. When this bit is set to 1, the PC value stacked for the exception return points to the instruction that tried to perform the illegal load of the PC."]
   #[inline] pub fn invpc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Invalid PC load UsageFault, caused by an invalid PC load by EXC_RETURN: 0 = no invalid PC load UsageFault, 1 = the processor has attempted an illegal load of EXC_RETURN to the PC, as a result of an invalid context, or an invalid EXC_RETURN value. When this bit is set to 1, the PC value stacked for the exception return points to the instruction that tried to perform the illegal load of the PC."]
   #[inline] pub fn set_invpc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Invalid state UsageFault: 0 = no invalid state UsageFault, 1 = the processor has attempted to execute an instruction that makes illegal use of the EPSR. When this bit is set to 1, the PC value stacked for the exception return points to the instruction that attempted the illegal use of the EPSR. This bit is not set to 1 if an undefined instruction uses the EPSR."]
   #[inline] pub fn invstate(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Invalid state UsageFault: 0 = no invalid state UsageFault, 1 = the processor has attempted to execute an instruction that makes illegal use of the EPSR. When this bit is set to 1, the PC value stacked for the exception return points to the instruction that attempted the illegal use of the EPSR. This bit is not set to 1 if an undefined instruction uses the EPSR."]
   #[inline] pub fn set_invstate<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Undefined instruction UsageFault: 0 = no undefined instruction UsageFault, 1 = the processor has attempted to execute an undefined instruction. When this bit is set to 1, the PC value stacked for the exception return points to the undefined instruction. An undefined instruction is an instruction that the processor cannot decode."]
   #[inline] pub fn undefinstr(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Undefined instruction UsageFault: 0 = no undefined instruction UsageFault, 1 = the processor has attempted to execute an undefined instruction. When this bit is set to 1, the PC value stacked for the exception return points to the undefined instruction. An undefined instruction is an instruction that the processor cannot decode."]
   #[inline] pub fn set_undefinstr<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u16 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Ufsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ufsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.divbyzero() != 0 { try!(write!(f, " divbyzero"))}
      if self.unaligned() != 0 { try!(write!(f, " unaligned"))}
      if self.nocp() != 0 { try!(write!(f, " nocp"))}
      if self.invpc() != 0 { try!(write!(f, " invpc"))}
      if self.invstate() != 0 { try!(write!(f, " invstate"))}
      if self.undefinstr() != 0 { try!(write!(f, " undefinstr"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="HardFault Status Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Hfsr(pub u32);
impl Hfsr {
#[doc="Reserved for Debug use. When writing to the register you must write 0 to this bit, otherwise behavior is Unpredictable."]
   #[inline] pub fn debugevt(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
   }
#[doc="Reserved for Debug use. When writing to the register you must write 0 to this bit, otherwise behavior is Unpredictable."]
   #[inline] pub fn set_debugevt<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 31);
      self.0 |= value << 31;
      self
   }

#[doc="Indicates a forced hard fault, generated by escalation of a fault with configurable priority that cannot be handles, either because of priority or because it is disabled: 0 = no forced HardFault, 1 = forced HardFault. When this bit is set to 1, the HardFault handler must read the other fault status registers to find the cause of the fault."]
   #[inline] pub fn forced(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
   }
#[doc="Indicates a forced hard fault, generated by escalation of a fault with configurable priority that cannot be handles, either because of priority or because it is disabled: 0 = no forced HardFault, 1 = forced HardFault. When this bit is set to 1, the HardFault handler must read the other fault status registers to find the cause of the fault."]
   #[inline] pub fn set_forced<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 30);
      self.0 |= value << 30;
      self
   }

#[doc="Indicates a BusFault on a vector table read during exception processing: 0 = no BusFault on vector table read, 1 = BusFault on vector table read. This error is always handled by the hard fault handler. When this bit is set to 1, the PC value stacked for the exception return points to the instruction that was preempted by the exception."]
   #[inline] pub fn vecttbl(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Indicates a BusFault on a vector table read during exception processing: 0 = no BusFault on vector table read, 1 = BusFault on vector table read. This error is always handled by the hard fault handler. When this bit is set to 1, the PC value stacked for the exception return points to the instruction that was preempted by the exception."]
   #[inline] pub fn set_vecttbl<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

}
impl ::core::fmt::Display for Hfsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Hfsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.debugevt() != 0 { try!(write!(f, " debugevt"))}
      if self.forced() != 0 { try!(write!(f, " forced"))}
      if self.vecttbl() != 0 { try!(write!(f, " vecttbl"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="MemManage Fault Address Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mmfar(pub u32);
impl Mmfar {
#[doc="When the MMARVALID bit of the MMFSR is set to 1, this field holds the address of the location that generated the MemManage fault"]
   #[inline] pub fn address(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="When the MMARVALID bit of the MMFSR is set to 1, this field holds the address of the location that generated the MemManage fault"]
   #[inline] pub fn set_address<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Mmfar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Mmfar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="BusFault Address Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bfar(pub u32);
impl Bfar {
#[doc="When the BFARVALID bit of the BFSR is set to 1, this field holds the address of the location that generated the BusFault"]
   #[inline] pub fn address(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="When the BFARVALID bit of the BFSR is set to 1, this field holds the address of the location that generated the BusFault"]
   #[inline] pub fn set_address<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Bfar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Bfar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="Auxiliary Fault Status Register"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Afsr(pub u32);
impl Afsr {
#[doc="Implementation defined. The bits map to the AUXFAULT input signals."]
   #[inline] pub fn impdef(&self) -> bits::U32 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
   }
#[doc="Implementation defined. The bits map to the AUXFAULT input signals."]
   #[inline] pub fn set_impdef<V: Into<bits::U32>>(mut self, value: V) -> Self {
      let value: bits::U32 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffffffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for Afsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Afsr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

