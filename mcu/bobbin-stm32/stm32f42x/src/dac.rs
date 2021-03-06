
::bobbin_mcu::periph!( DAC, Dac, DAC_PERIPH, DacPeriph, DAC_OWNED, DAC_REF_COUNT, 0x40007400, 0x00, 0x08);


#[doc="Digital-to-analog converter"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DacPeriph(pub usize);
impl DacPeriph {
    #[doc="Get the CR Register."]
    #[inline] pub fn cr_reg(&self) -> ::bobbin_mcu::register::Register<Cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr, 0x0)
    }

    #[doc="Get the *mut pointer for the CR register."]
    #[inline] pub fn cr_mut(&self) -> *mut Cr { 
        self.cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR register."]
    #[inline] pub fn cr_ptr(&self) -> *const Cr { 
        self.cr_reg().ptr()
    }

    #[doc="Read the CR register."]
    #[inline] pub fn cr(&self) -> Cr { 
        self.cr_reg().read()
    }

    #[doc="Write the CR register."]
    #[inline] pub fn write_cr(&self, value: Cr) -> &Self { 
        self.cr_reg().write(value);
        self
    }

    #[doc="Set the CR register."]
    #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        self.cr_reg().set(f);
        self
    }

    #[doc="Modify the CR register."]
    #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        self.cr_reg().with(f);
        self
    }

    #[doc="Get the SWTRIGR Register."]
    #[inline] pub fn swtrigr_reg(&self) -> ::bobbin_mcu::register::Register<Swtrigr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Swtrigr, 0x4)
    }

    #[doc="Get the *mut pointer for the SWTRIGR register."]
    #[inline] pub fn swtrigr_mut(&self) -> *mut Swtrigr { 
        self.swtrigr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SWTRIGR register."]
    #[inline] pub fn swtrigr_ptr(&self) -> *const Swtrigr { 
        self.swtrigr_reg().ptr()
    }

    #[doc="Write the SWTRIGR register."]
    #[inline] pub fn write_swtrigr(&self, value: Swtrigr) -> &Self { 
        self.swtrigr_reg().write(value);
        self
    }

    #[doc="Set the SWTRIGR register."]
    #[inline] pub fn set_swtrigr<F: FnOnce(Swtrigr) -> Swtrigr>(&self, f: F) -> &Self {
        self.swtrigr_reg().set(f);
        self
    }

    #[doc="Get the DHR12R1 Register."]
    #[inline] pub fn dhr12r1_reg(&self) -> ::bobbin_mcu::register::Register<Dhr12r1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dhr12r1, 0x8)
    }

    #[doc="Get the *mut pointer for the DHR12R1 register."]
    #[inline] pub fn dhr12r1_mut(&self) -> *mut Dhr12r1 { 
        self.dhr12r1_reg().ptr()
    }

    #[doc="Get the *const pointer for the DHR12R1 register."]
    #[inline] pub fn dhr12r1_ptr(&self) -> *const Dhr12r1 { 
        self.dhr12r1_reg().ptr()
    }

    #[doc="Read the DHR12R1 register."]
    #[inline] pub fn dhr12r1(&self) -> Dhr12r1 { 
        self.dhr12r1_reg().read()
    }

    #[doc="Write the DHR12R1 register."]
    #[inline] pub fn write_dhr12r1(&self, value: Dhr12r1) -> &Self { 
        self.dhr12r1_reg().write(value);
        self
    }

    #[doc="Set the DHR12R1 register."]
    #[inline] pub fn set_dhr12r1<F: FnOnce(Dhr12r1) -> Dhr12r1>(&self, f: F) -> &Self {
        self.dhr12r1_reg().set(f);
        self
    }

    #[doc="Modify the DHR12R1 register."]
    #[inline] pub fn with_dhr12r1<F: FnOnce(Dhr12r1) -> Dhr12r1>(&self, f: F) -> &Self {
        self.dhr12r1_reg().with(f);
        self
    }

    #[doc="Get the DHR12L1 Register."]
    #[inline] pub fn dhr12l1_reg(&self) -> ::bobbin_mcu::register::Register<Dhr12l1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dhr12l1, 0xc)
    }

    #[doc="Get the *mut pointer for the DHR12L1 register."]
    #[inline] pub fn dhr12l1_mut(&self) -> *mut Dhr12l1 { 
        self.dhr12l1_reg().ptr()
    }

    #[doc="Get the *const pointer for the DHR12L1 register."]
    #[inline] pub fn dhr12l1_ptr(&self) -> *const Dhr12l1 { 
        self.dhr12l1_reg().ptr()
    }

    #[doc="Read the DHR12L1 register."]
    #[inline] pub fn dhr12l1(&self) -> Dhr12l1 { 
        self.dhr12l1_reg().read()
    }

    #[doc="Write the DHR12L1 register."]
    #[inline] pub fn write_dhr12l1(&self, value: Dhr12l1) -> &Self { 
        self.dhr12l1_reg().write(value);
        self
    }

    #[doc="Set the DHR12L1 register."]
    #[inline] pub fn set_dhr12l1<F: FnOnce(Dhr12l1) -> Dhr12l1>(&self, f: F) -> &Self {
        self.dhr12l1_reg().set(f);
        self
    }

    #[doc="Modify the DHR12L1 register."]
    #[inline] pub fn with_dhr12l1<F: FnOnce(Dhr12l1) -> Dhr12l1>(&self, f: F) -> &Self {
        self.dhr12l1_reg().with(f);
        self
    }

    #[doc="Get the DHR8R1 Register."]
    #[inline] pub fn dhr8r1_reg(&self) -> ::bobbin_mcu::register::Register<Dhr8r1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dhr8r1, 0x10)
    }

    #[doc="Get the *mut pointer for the DHR8R1 register."]
    #[inline] pub fn dhr8r1_mut(&self) -> *mut Dhr8r1 { 
        self.dhr8r1_reg().ptr()
    }

    #[doc="Get the *const pointer for the DHR8R1 register."]
    #[inline] pub fn dhr8r1_ptr(&self) -> *const Dhr8r1 { 
        self.dhr8r1_reg().ptr()
    }

    #[doc="Read the DHR8R1 register."]
    #[inline] pub fn dhr8r1(&self) -> Dhr8r1 { 
        self.dhr8r1_reg().read()
    }

    #[doc="Write the DHR8R1 register."]
    #[inline] pub fn write_dhr8r1(&self, value: Dhr8r1) -> &Self { 
        self.dhr8r1_reg().write(value);
        self
    }

    #[doc="Set the DHR8R1 register."]
    #[inline] pub fn set_dhr8r1<F: FnOnce(Dhr8r1) -> Dhr8r1>(&self, f: F) -> &Self {
        self.dhr8r1_reg().set(f);
        self
    }

    #[doc="Modify the DHR8R1 register."]
    #[inline] pub fn with_dhr8r1<F: FnOnce(Dhr8r1) -> Dhr8r1>(&self, f: F) -> &Self {
        self.dhr8r1_reg().with(f);
        self
    }

    #[doc="Get the DHR12R2 Register."]
    #[inline] pub fn dhr12r2_reg(&self) -> ::bobbin_mcu::register::Register<Dhr12r2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dhr12r2, 0x14)
    }

    #[doc="Get the *mut pointer for the DHR12R2 register."]
    #[inline] pub fn dhr12r2_mut(&self) -> *mut Dhr12r2 { 
        self.dhr12r2_reg().ptr()
    }

    #[doc="Get the *const pointer for the DHR12R2 register."]
    #[inline] pub fn dhr12r2_ptr(&self) -> *const Dhr12r2 { 
        self.dhr12r2_reg().ptr()
    }

    #[doc="Read the DHR12R2 register."]
    #[inline] pub fn dhr12r2(&self) -> Dhr12r2 { 
        self.dhr12r2_reg().read()
    }

    #[doc="Write the DHR12R2 register."]
    #[inline] pub fn write_dhr12r2(&self, value: Dhr12r2) -> &Self { 
        self.dhr12r2_reg().write(value);
        self
    }

    #[doc="Set the DHR12R2 register."]
    #[inline] pub fn set_dhr12r2<F: FnOnce(Dhr12r2) -> Dhr12r2>(&self, f: F) -> &Self {
        self.dhr12r2_reg().set(f);
        self
    }

    #[doc="Modify the DHR12R2 register."]
    #[inline] pub fn with_dhr12r2<F: FnOnce(Dhr12r2) -> Dhr12r2>(&self, f: F) -> &Self {
        self.dhr12r2_reg().with(f);
        self
    }

    #[doc="Get the DHR12L2 Register."]
    #[inline] pub fn dhr12l2_reg(&self) -> ::bobbin_mcu::register::Register<Dhr12l2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dhr12l2, 0x18)
    }

    #[doc="Get the *mut pointer for the DHR12L2 register."]
    #[inline] pub fn dhr12l2_mut(&self) -> *mut Dhr12l2 { 
        self.dhr12l2_reg().ptr()
    }

    #[doc="Get the *const pointer for the DHR12L2 register."]
    #[inline] pub fn dhr12l2_ptr(&self) -> *const Dhr12l2 { 
        self.dhr12l2_reg().ptr()
    }

    #[doc="Read the DHR12L2 register."]
    #[inline] pub fn dhr12l2(&self) -> Dhr12l2 { 
        self.dhr12l2_reg().read()
    }

    #[doc="Write the DHR12L2 register."]
    #[inline] pub fn write_dhr12l2(&self, value: Dhr12l2) -> &Self { 
        self.dhr12l2_reg().write(value);
        self
    }

    #[doc="Set the DHR12L2 register."]
    #[inline] pub fn set_dhr12l2<F: FnOnce(Dhr12l2) -> Dhr12l2>(&self, f: F) -> &Self {
        self.dhr12l2_reg().set(f);
        self
    }

    #[doc="Modify the DHR12L2 register."]
    #[inline] pub fn with_dhr12l2<F: FnOnce(Dhr12l2) -> Dhr12l2>(&self, f: F) -> &Self {
        self.dhr12l2_reg().with(f);
        self
    }

    #[doc="Get the DHR8R2 Register."]
    #[inline] pub fn dhr8r2_reg(&self) -> ::bobbin_mcu::register::Register<Dhr8r2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dhr8r2, 0x1c)
    }

    #[doc="Get the *mut pointer for the DHR8R2 register."]
    #[inline] pub fn dhr8r2_mut(&self) -> *mut Dhr8r2 { 
        self.dhr8r2_reg().ptr()
    }

    #[doc="Get the *const pointer for the DHR8R2 register."]
    #[inline] pub fn dhr8r2_ptr(&self) -> *const Dhr8r2 { 
        self.dhr8r2_reg().ptr()
    }

    #[doc="Read the DHR8R2 register."]
    #[inline] pub fn dhr8r2(&self) -> Dhr8r2 { 
        self.dhr8r2_reg().read()
    }

    #[doc="Write the DHR8R2 register."]
    #[inline] pub fn write_dhr8r2(&self, value: Dhr8r2) -> &Self { 
        self.dhr8r2_reg().write(value);
        self
    }

    #[doc="Set the DHR8R2 register."]
    #[inline] pub fn set_dhr8r2<F: FnOnce(Dhr8r2) -> Dhr8r2>(&self, f: F) -> &Self {
        self.dhr8r2_reg().set(f);
        self
    }

    #[doc="Modify the DHR8R2 register."]
    #[inline] pub fn with_dhr8r2<F: FnOnce(Dhr8r2) -> Dhr8r2>(&self, f: F) -> &Self {
        self.dhr8r2_reg().with(f);
        self
    }

    #[doc="Get the DHR12RD Register."]
    #[inline] pub fn dhr12rd_reg(&self) -> ::bobbin_mcu::register::Register<Dhr12rd> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dhr12rd, 0x20)
    }

    #[doc="Get the *mut pointer for the DHR12RD register."]
    #[inline] pub fn dhr12rd_mut(&self) -> *mut Dhr12rd { 
        self.dhr12rd_reg().ptr()
    }

    #[doc="Get the *const pointer for the DHR12RD register."]
    #[inline] pub fn dhr12rd_ptr(&self) -> *const Dhr12rd { 
        self.dhr12rd_reg().ptr()
    }

    #[doc="Read the DHR12RD register."]
    #[inline] pub fn dhr12rd(&self) -> Dhr12rd { 
        self.dhr12rd_reg().read()
    }

    #[doc="Write the DHR12RD register."]
    #[inline] pub fn write_dhr12rd(&self, value: Dhr12rd) -> &Self { 
        self.dhr12rd_reg().write(value);
        self
    }

    #[doc="Set the DHR12RD register."]
    #[inline] pub fn set_dhr12rd<F: FnOnce(Dhr12rd) -> Dhr12rd>(&self, f: F) -> &Self {
        self.dhr12rd_reg().set(f);
        self
    }

    #[doc="Modify the DHR12RD register."]
    #[inline] pub fn with_dhr12rd<F: FnOnce(Dhr12rd) -> Dhr12rd>(&self, f: F) -> &Self {
        self.dhr12rd_reg().with(f);
        self
    }

    #[doc="Get the DHR12LD Register."]
    #[inline] pub fn dhr12ld_reg(&self) -> ::bobbin_mcu::register::Register<Dhr12ld> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dhr12ld, 0x24)
    }

    #[doc="Get the *mut pointer for the DHR12LD register."]
    #[inline] pub fn dhr12ld_mut(&self) -> *mut Dhr12ld { 
        self.dhr12ld_reg().ptr()
    }

    #[doc="Get the *const pointer for the DHR12LD register."]
    #[inline] pub fn dhr12ld_ptr(&self) -> *const Dhr12ld { 
        self.dhr12ld_reg().ptr()
    }

    #[doc="Read the DHR12LD register."]
    #[inline] pub fn dhr12ld(&self) -> Dhr12ld { 
        self.dhr12ld_reg().read()
    }

    #[doc="Write the DHR12LD register."]
    #[inline] pub fn write_dhr12ld(&self, value: Dhr12ld) -> &Self { 
        self.dhr12ld_reg().write(value);
        self
    }

    #[doc="Set the DHR12LD register."]
    #[inline] pub fn set_dhr12ld<F: FnOnce(Dhr12ld) -> Dhr12ld>(&self, f: F) -> &Self {
        self.dhr12ld_reg().set(f);
        self
    }

    #[doc="Modify the DHR12LD register."]
    #[inline] pub fn with_dhr12ld<F: FnOnce(Dhr12ld) -> Dhr12ld>(&self, f: F) -> &Self {
        self.dhr12ld_reg().with(f);
        self
    }

    #[doc="Get the DHR8RD Register."]
    #[inline] pub fn dhr8rd_reg(&self) -> ::bobbin_mcu::register::Register<Dhr8rd> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dhr8rd, 0x28)
    }

    #[doc="Get the *mut pointer for the DHR8RD register."]
    #[inline] pub fn dhr8rd_mut(&self) -> *mut Dhr8rd { 
        self.dhr8rd_reg().ptr()
    }

    #[doc="Get the *const pointer for the DHR8RD register."]
    #[inline] pub fn dhr8rd_ptr(&self) -> *const Dhr8rd { 
        self.dhr8rd_reg().ptr()
    }

    #[doc="Read the DHR8RD register."]
    #[inline] pub fn dhr8rd(&self) -> Dhr8rd { 
        self.dhr8rd_reg().read()
    }

    #[doc="Write the DHR8RD register."]
    #[inline] pub fn write_dhr8rd(&self, value: Dhr8rd) -> &Self { 
        self.dhr8rd_reg().write(value);
        self
    }

    #[doc="Set the DHR8RD register."]
    #[inline] pub fn set_dhr8rd<F: FnOnce(Dhr8rd) -> Dhr8rd>(&self, f: F) -> &Self {
        self.dhr8rd_reg().set(f);
        self
    }

    #[doc="Modify the DHR8RD register."]
    #[inline] pub fn with_dhr8rd<F: FnOnce(Dhr8rd) -> Dhr8rd>(&self, f: F) -> &Self {
        self.dhr8rd_reg().with(f);
        self
    }

    #[doc="Get the DOR1 Register."]
    #[inline] pub fn dor1_reg(&self) -> ::bobbin_mcu::register::Register<Dor1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dor1, 0x2c)
    }

    #[doc="Get the *mut pointer for the DOR1 register."]
    #[inline] pub fn dor1_mut(&self) -> *mut Dor1 { 
        self.dor1_reg().ptr()
    }

    #[doc="Get the *const pointer for the DOR1 register."]
    #[inline] pub fn dor1_ptr(&self) -> *const Dor1 { 
        self.dor1_reg().ptr()
    }

    #[doc="Read the DOR1 register."]
    #[inline] pub fn dor1(&self) -> Dor1 { 
        self.dor1_reg().read()
    }

    #[doc="Get the DOR2 Register."]
    #[inline] pub fn dor2_reg(&self) -> ::bobbin_mcu::register::Register<Dor2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dor2, 0x30)
    }

    #[doc="Get the *mut pointer for the DOR2 register."]
    #[inline] pub fn dor2_mut(&self) -> *mut Dor2 { 
        self.dor2_reg().ptr()
    }

    #[doc="Get the *const pointer for the DOR2 register."]
    #[inline] pub fn dor2_ptr(&self) -> *const Dor2 { 
        self.dor2_reg().ptr()
    }

    #[doc="Read the DOR2 register."]
    #[inline] pub fn dor2(&self) -> Dor2 { 
        self.dor2_reg().read()
    }

    #[doc="Get the SR Register."]
    #[inline] pub fn sr_reg(&self) -> ::bobbin_mcu::register::Register<Sr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sr, 0x34)
    }

    #[doc="Get the *mut pointer for the SR register."]
    #[inline] pub fn sr_mut(&self) -> *mut Sr { 
        self.sr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SR register."]
    #[inline] pub fn sr_ptr(&self) -> *const Sr { 
        self.sr_reg().ptr()
    }

    #[doc="Read the SR register."]
    #[inline] pub fn sr(&self) -> Sr { 
        self.sr_reg().read()
    }

    #[doc="Write the SR register."]
    #[inline] pub fn write_sr(&self, value: Sr) -> &Self { 
        self.sr_reg().write(value);
        self
    }

    #[doc="Set the SR register."]
    #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        self.sr_reg().set(f);
        self
    }

    #[doc="Modify the SR register."]
    #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        self.sr_reg().with(f);
        self
    }

}

#[doc="control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="DAC channel2 DMA underrun interrupt enable"]
    #[inline] pub fn dmaudrie2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if DMAUDRIE2 != 0"]
    #[inline] pub fn test_dmaudrie2(&self) -> bool {
        self.dmaudrie2() != 0
    }

    #[doc="Sets the DMAUDRIE2 field."]
    #[inline] pub fn set_dmaudrie2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="DAC channel2 DMA enable"]
    #[inline] pub fn dmaen2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if DMAEN2 != 0"]
    #[inline] pub fn test_dmaen2(&self) -> bool {
        self.dmaen2() != 0
    }

    #[doc="Sets the DMAEN2 field."]
    #[inline] pub fn set_dmaen2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="DAC channel2 mask/amplitude selector"]
    #[inline] pub fn mamp2(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if MAMP2 != 0"]
    #[inline] pub fn test_mamp2(&self) -> bool {
        self.mamp2() != 0
    }

    #[doc="Sets the MAMP2 field."]
    #[inline] pub fn set_mamp2<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DAC channel2 noise/triangle wave generation enable"]
    #[inline] pub fn wave2(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
    }

    #[doc="Returns true if WAVE2 != 0"]
    #[inline] pub fn test_wave2(&self) -> bool {
        self.wave2() != 0
    }

    #[doc="Sets the WAVE2 field."]
    #[inline] pub fn set_wave2<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="DAC channel2 trigger selection"]
    #[inline] pub fn tsel2(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x7) as u8) } // [21:19]
    }

    #[doc="Returns true if TSEL2 != 0"]
    #[inline] pub fn test_tsel2(&self) -> bool {
        self.tsel2() != 0
    }

    #[doc="Sets the TSEL2 field."]
    #[inline] pub fn set_tsel2<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="DAC channel2 trigger enable"]
    #[inline] pub fn ten2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if TEN2 != 0"]
    #[inline] pub fn test_ten2(&self) -> bool {
        self.ten2() != 0
    }

    #[doc="Sets the TEN2 field."]
    #[inline] pub fn set_ten2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="DAC channel2 output buffer disable"]
    #[inline] pub fn boff2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if BOFF2 != 0"]
    #[inline] pub fn test_boff2(&self) -> bool {
        self.boff2() != 0
    }

    #[doc="Sets the BOFF2 field."]
    #[inline] pub fn set_boff2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="DAC channel2 enable"]
    #[inline] pub fn en2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if EN2 != 0"]
    #[inline] pub fn test_en2(&self) -> bool {
        self.en2() != 0
    }

    #[doc="Sets the EN2 field."]
    #[inline] pub fn set_en2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DAC channel1 DMA Underrun Interrupt enable"]
    #[inline] pub fn dmaudrie1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if DMAUDRIE1 != 0"]
    #[inline] pub fn test_dmaudrie1(&self) -> bool {
        self.dmaudrie1() != 0
    }

    #[doc="Sets the DMAUDRIE1 field."]
    #[inline] pub fn set_dmaudrie1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="DAC channel1 DMA enable"]
    #[inline] pub fn dmaen1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DMAEN1 != 0"]
    #[inline] pub fn test_dmaen1(&self) -> bool {
        self.dmaen1() != 0
    }

    #[doc="Sets the DMAEN1 field."]
    #[inline] pub fn set_dmaen1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="DAC channel1 mask/amplitude selector"]
    #[inline] pub fn mamp1(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if MAMP1 != 0"]
    #[inline] pub fn test_mamp1(&self) -> bool {
        self.mamp1() != 0
    }

    #[doc="Sets the MAMP1 field."]
    #[inline] pub fn set_mamp1<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DAC channel1 noise/triangle wave generation enable"]
    #[inline] pub fn wave1(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if WAVE1 != 0"]
    #[inline] pub fn test_wave1(&self) -> bool {
        self.wave1() != 0
    }

    #[doc="Sets the WAVE1 field."]
    #[inline] pub fn set_wave1<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="DAC channel1 trigger selection"]
    #[inline] pub fn tsel1(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if TSEL1 != 0"]
    #[inline] pub fn test_tsel1(&self) -> bool {
        self.tsel1() != 0
    }

    #[doc="Sets the TSEL1 field."]
    #[inline] pub fn set_tsel1<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="DAC channel1 trigger enable"]
    #[inline] pub fn ten1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TEN1 != 0"]
    #[inline] pub fn test_ten1(&self) -> bool {
        self.ten1() != 0
    }

    #[doc="Sets the TEN1 field."]
    #[inline] pub fn set_ten1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="DAC channel1 output buffer disable"]
    #[inline] pub fn boff1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if BOFF1 != 0"]
    #[inline] pub fn test_boff1(&self) -> bool {
        self.boff1() != 0
    }

    #[doc="Sets the BOFF1 field."]
    #[inline] pub fn set_boff1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DAC channel1 enable"]
    #[inline] pub fn en1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EN1 != 0"]
    #[inline] pub fn test_en1(&self) -> bool {
        self.en1() != 0
    }

    #[doc="Sets the EN1 field."]
    #[inline] pub fn set_en1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cr {
    #[inline]
    fn from(other: u32) -> Self {
         Cr(other)
    }
}

impl ::core::fmt::Display for Cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dmaudrie2() != 0 { try!(write!(f, " dmaudrie2"))}
        if self.dmaen2() != 0 { try!(write!(f, " dmaen2"))}
        if self.mamp2() != 0 { try!(write!(f, " mamp2=0x{:x}", self.mamp2()))}
        if self.wave2() != 0 { try!(write!(f, " wave2=0x{:x}", self.wave2()))}
        if self.tsel2() != 0 { try!(write!(f, " tsel2=0x{:x}", self.tsel2()))}
        if self.ten2() != 0 { try!(write!(f, " ten2"))}
        if self.boff2() != 0 { try!(write!(f, " boff2"))}
        if self.en2() != 0 { try!(write!(f, " en2"))}
        if self.dmaudrie1() != 0 { try!(write!(f, " dmaudrie1"))}
        if self.dmaen1() != 0 { try!(write!(f, " dmaen1"))}
        if self.mamp1() != 0 { try!(write!(f, " mamp1=0x{:x}", self.mamp1()))}
        if self.wave1() != 0 { try!(write!(f, " wave1=0x{:x}", self.wave1()))}
        if self.tsel1() != 0 { try!(write!(f, " tsel1=0x{:x}", self.tsel1()))}
        if self.ten1() != 0 { try!(write!(f, " ten1"))}
        if self.boff1() != 0 { try!(write!(f, " boff1"))}
        if self.en1() != 0 { try!(write!(f, " en1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="software trigger register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Swtrigr(pub u32);
impl Swtrigr {
    #[doc="DAC channel2 software trigger"]
    #[inline] pub fn swtrig2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SWTRIG2 != 0"]
    #[inline] pub fn test_swtrig2(&self) -> bool {
        self.swtrig2() != 0
    }

    #[doc="Sets the SWTRIG2 field."]
    #[inline] pub fn set_swtrig2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DAC channel1 software trigger"]
    #[inline] pub fn swtrig1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SWTRIG1 != 0"]
    #[inline] pub fn test_swtrig1(&self) -> bool {
        self.swtrig1() != 0
    }

    #[doc="Sets the SWTRIG1 field."]
    #[inline] pub fn set_swtrig1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Swtrigr {
    #[inline]
    fn from(other: u32) -> Self {
         Swtrigr(other)
    }
}

impl ::core::fmt::Display for Swtrigr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Swtrigr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swtrig2() != 0 { try!(write!(f, " swtrig2"))}
        if self.swtrig1() != 0 { try!(write!(f, " swtrig1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="channel1 12-bit right-aligned data holding register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dhr12r1(pub u32);
impl Dhr12r1 {
    #[doc="DAC channel1 12-bit right-aligned data"]
    #[inline] pub fn dacc1dhr(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if DACC1DHR != 0"]
    #[inline] pub fn test_dacc1dhr(&self) -> bool {
        self.dacc1dhr() != 0
    }

    #[doc="Sets the DACC1DHR field."]
    #[inline] pub fn set_dacc1dhr<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dhr12r1 {
    #[inline]
    fn from(other: u32) -> Self {
         Dhr12r1(other)
    }
}

impl ::core::fmt::Display for Dhr12r1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dhr12r1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dacc1dhr() != 0 { try!(write!(f, " dacc1dhr=0x{:x}", self.dacc1dhr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="channel1 12-bit left aligned data holding register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dhr12l1(pub u32);
impl Dhr12l1 {
    #[doc="DAC channel1 12-bit left-aligned data"]
    #[inline] pub fn dacc1dhr(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xfff) as u16) } // [15:4]
    }

    #[doc="Returns true if DACC1DHR != 0"]
    #[inline] pub fn test_dacc1dhr(&self) -> bool {
        self.dacc1dhr() != 0
    }

    #[doc="Sets the DACC1DHR field."]
    #[inline] pub fn set_dacc1dhr<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Dhr12l1 {
    #[inline]
    fn from(other: u32) -> Self {
         Dhr12l1(other)
    }
}

impl ::core::fmt::Display for Dhr12l1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dhr12l1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dacc1dhr() != 0 { try!(write!(f, " dacc1dhr=0x{:x}", self.dacc1dhr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="channel1 8-bit right aligned data holding register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dhr8r1(pub u32);
impl Dhr8r1 {
    #[doc="DAC channel1 8-bit right-aligned data"]
    #[inline] pub fn dacc1dhr(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DACC1DHR != 0"]
    #[inline] pub fn test_dacc1dhr(&self) -> bool {
        self.dacc1dhr() != 0
    }

    #[doc="Sets the DACC1DHR field."]
    #[inline] pub fn set_dacc1dhr<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dhr8r1 {
    #[inline]
    fn from(other: u32) -> Self {
         Dhr8r1(other)
    }
}

impl ::core::fmt::Display for Dhr8r1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dhr8r1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dacc1dhr() != 0 { try!(write!(f, " dacc1dhr=0x{:x}", self.dacc1dhr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="channel2 12-bit right aligned data holding register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dhr12r2(pub u32);
impl Dhr12r2 {
    #[doc="DAC channel2 12-bit right-aligned data"]
    #[inline] pub fn dacc2dhr(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if DACC2DHR != 0"]
    #[inline] pub fn test_dacc2dhr(&self) -> bool {
        self.dacc2dhr() != 0
    }

    #[doc="Sets the DACC2DHR field."]
    #[inline] pub fn set_dacc2dhr<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dhr12r2 {
    #[inline]
    fn from(other: u32) -> Self {
         Dhr12r2(other)
    }
}

impl ::core::fmt::Display for Dhr12r2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dhr12r2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dacc2dhr() != 0 { try!(write!(f, " dacc2dhr=0x{:x}", self.dacc2dhr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="channel2 12-bit left aligned data holding register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dhr12l2(pub u32);
impl Dhr12l2 {
    #[doc="DAC channel2 12-bit left-aligned data"]
    #[inline] pub fn dacc2dhr(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xfff) as u16) } // [15:4]
    }

    #[doc="Returns true if DACC2DHR != 0"]
    #[inline] pub fn test_dacc2dhr(&self) -> bool {
        self.dacc2dhr() != 0
    }

    #[doc="Sets the DACC2DHR field."]
    #[inline] pub fn set_dacc2dhr<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Dhr12l2 {
    #[inline]
    fn from(other: u32) -> Self {
         Dhr12l2(other)
    }
}

impl ::core::fmt::Display for Dhr12l2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dhr12l2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dacc2dhr() != 0 { try!(write!(f, " dacc2dhr=0x{:x}", self.dacc2dhr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="channel2 8-bit right-aligned data holding register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dhr8r2(pub u32);
impl Dhr8r2 {
    #[doc="DAC channel2 8-bit right-aligned data"]
    #[inline] pub fn dacc2dhr(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DACC2DHR != 0"]
    #[inline] pub fn test_dacc2dhr(&self) -> bool {
        self.dacc2dhr() != 0
    }

    #[doc="Sets the DACC2DHR field."]
    #[inline] pub fn set_dacc2dhr<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dhr8r2 {
    #[inline]
    fn from(other: u32) -> Self {
         Dhr8r2(other)
    }
}

impl ::core::fmt::Display for Dhr8r2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dhr8r2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dacc2dhr() != 0 { try!(write!(f, " dacc2dhr=0x{:x}", self.dacc2dhr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Dual DAC 12-bit right-aligned data holding register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dhr12rd(pub u32);
impl Dhr12rd {
    #[doc="DAC channel2 12-bit right-aligned data"]
    #[inline] pub fn dacc2dhr(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xfff) as u16) } // [27:16]
    }

    #[doc="Returns true if DACC2DHR != 0"]
    #[inline] pub fn test_dacc2dhr(&self) -> bool {
        self.dacc2dhr() != 0
    }

    #[doc="Sets the DACC2DHR field."]
    #[inline] pub fn set_dacc2dhr<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DAC channel1 12-bit right-aligned data"]
    #[inline] pub fn dacc1dhr(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if DACC1DHR != 0"]
    #[inline] pub fn test_dacc1dhr(&self) -> bool {
        self.dacc1dhr() != 0
    }

    #[doc="Sets the DACC1DHR field."]
    #[inline] pub fn set_dacc1dhr<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dhr12rd {
    #[inline]
    fn from(other: u32) -> Self {
         Dhr12rd(other)
    }
}

impl ::core::fmt::Display for Dhr12rd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dhr12rd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dacc2dhr() != 0 { try!(write!(f, " dacc2dhr=0x{:x}", self.dacc2dhr()))}
        if self.dacc1dhr() != 0 { try!(write!(f, " dacc1dhr=0x{:x}", self.dacc1dhr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DUAL DAC 12-bit left aligned data holding register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dhr12ld(pub u32);
impl Dhr12ld {
    #[doc="DAC channel2 12-bit left-aligned data"]
    #[inline] pub fn dacc2dhr(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xfff) as u16) } // [31:20]
    }

    #[doc="Returns true if DACC2DHR != 0"]
    #[inline] pub fn test_dacc2dhr(&self) -> bool {
        self.dacc2dhr() != 0
    }

    #[doc="Sets the DACC2DHR field."]
    #[inline] pub fn set_dacc2dhr<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="DAC channel1 12-bit left-aligned data"]
    #[inline] pub fn dacc1dhr(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xfff) as u16) } // [15:4]
    }

    #[doc="Returns true if DACC1DHR != 0"]
    #[inline] pub fn test_dacc1dhr(&self) -> bool {
        self.dacc1dhr() != 0
    }

    #[doc="Sets the DACC1DHR field."]
    #[inline] pub fn set_dacc1dhr<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Dhr12ld {
    #[inline]
    fn from(other: u32) -> Self {
         Dhr12ld(other)
    }
}

impl ::core::fmt::Display for Dhr12ld {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dhr12ld {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dacc2dhr() != 0 { try!(write!(f, " dacc2dhr=0x{:x}", self.dacc2dhr()))}
        if self.dacc1dhr() != 0 { try!(write!(f, " dacc1dhr=0x{:x}", self.dacc1dhr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DUAL DAC 8-bit right aligned data holding register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dhr8rd(pub u32);
impl Dhr8rd {
    #[doc="DAC channel2 8-bit right-aligned data"]
    #[inline] pub fn dacc2dhr(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DACC2DHR != 0"]
    #[inline] pub fn test_dacc2dhr(&self) -> bool {
        self.dacc2dhr() != 0
    }

    #[doc="Sets the DACC2DHR field."]
    #[inline] pub fn set_dacc2dhr<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DAC channel1 8-bit right-aligned data"]
    #[inline] pub fn dacc1dhr(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DACC1DHR != 0"]
    #[inline] pub fn test_dacc1dhr(&self) -> bool {
        self.dacc1dhr() != 0
    }

    #[doc="Sets the DACC1DHR field."]
    #[inline] pub fn set_dacc1dhr<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dhr8rd {
    #[inline]
    fn from(other: u32) -> Self {
         Dhr8rd(other)
    }
}

impl ::core::fmt::Display for Dhr8rd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dhr8rd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dacc2dhr() != 0 { try!(write!(f, " dacc2dhr=0x{:x}", self.dacc2dhr()))}
        if self.dacc1dhr() != 0 { try!(write!(f, " dacc1dhr=0x{:x}", self.dacc1dhr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="channel1 data output register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dor1(pub u32);
impl Dor1 {
    #[doc="DAC channel1 data output"]
    #[inline] pub fn dacc1dor(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if DACC1DOR != 0"]
    #[inline] pub fn test_dacc1dor(&self) -> bool {
        self.dacc1dor() != 0
    }

    #[doc="Sets the DACC1DOR field."]
    #[inline] pub fn set_dacc1dor<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dor1 {
    #[inline]
    fn from(other: u32) -> Self {
         Dor1(other)
    }
}

impl ::core::fmt::Display for Dor1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dor1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dacc1dor() != 0 { try!(write!(f, " dacc1dor=0x{:x}", self.dacc1dor()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="channel2 data output register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dor2(pub u32);
impl Dor2 {
    #[doc="DAC channel2 data output"]
    #[inline] pub fn dacc2dor(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if DACC2DOR != 0"]
    #[inline] pub fn test_dacc2dor(&self) -> bool {
        self.dacc2dor() != 0
    }

    #[doc="Sets the DACC2DOR field."]
    #[inline] pub fn set_dacc2dor<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dor2 {
    #[inline]
    fn from(other: u32) -> Self {
         Dor2(other)
    }
}

impl ::core::fmt::Display for Dor2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dor2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dacc2dor() != 0 { try!(write!(f, " dacc2dor=0x{:x}", self.dacc2dor()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="DAC channel2 DMA underrun flag"]
    #[inline] pub fn dmaudr2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if DMAUDR2 != 0"]
    #[inline] pub fn test_dmaudr2(&self) -> bool {
        self.dmaudr2() != 0
    }

    #[doc="Sets the DMAUDR2 field."]
    #[inline] pub fn set_dmaudr2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="DAC channel1 DMA underrun flag"]
    #[inline] pub fn dmaudr1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if DMAUDR1 != 0"]
    #[inline] pub fn test_dmaudr1(&self) -> bool {
        self.dmaudr1() != 0
    }

    #[doc="Sets the DMAUDR1 field."]
    #[inline] pub fn set_dmaudr1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u32> for Sr {
    #[inline]
    fn from(other: u32) -> Self {
         Sr(other)
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
        if self.dmaudr2() != 0 { try!(write!(f, " dmaudr2"))}
        if self.dmaudr1() != 0 { try!(write!(f, " dmaudr1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

