::bobbin_mcu::periph!( USB_FS_GLOBAL, UsbFsGlobal, USB_FS_GLOBAL_PERIPH, UsbFsGlobalPeriph, USB_FS_GLOBAL_OWNED, USB_FS_GLOBAL_REF_COUNT, 0x50000000, 0x00, 0x14);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="USB_FS_GLOBAL Peripheral"]
pub struct UsbFsGlobalPeriph(pub usize); 

impl UsbFsGlobalPeriph {
    #[doc="Get the GOTGCTL Register."]
    #[inline] pub fn gotgctl_reg(&self) -> ::bobbin_mcu::register::Register<Gotgctl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Gotgctl, 0x0)
    }

    #[doc="Get the *mut pointer for the GOTGCTL register."]
    #[inline] pub fn gotgctl_mut(&self) -> *mut Gotgctl { 
        self.gotgctl_reg().ptr()
    }

    #[doc="Get the *const pointer for the GOTGCTL register."]
    #[inline] pub fn gotgctl_ptr(&self) -> *const Gotgctl { 
        self.gotgctl_reg().ptr()
    }

    #[doc="Read the GOTGCTL register."]
    #[inline] pub fn gotgctl(&self) -> Gotgctl { 
        self.gotgctl_reg().read()
    }

    #[doc="Write the GOTGCTL register."]
    #[inline] pub fn write_gotgctl(&self, value: Gotgctl) -> &Self { 
        self.gotgctl_reg().write(value);
        self
    }

    #[doc="Set the GOTGCTL register."]
    #[inline] pub fn set_gotgctl<F: FnOnce(Gotgctl) -> Gotgctl>(&self, f: F) -> &Self {
        self.gotgctl_reg().set(f);
        self
    }

    #[doc="Modify the GOTGCTL register."]
    #[inline] pub fn with_gotgctl<F: FnOnce(Gotgctl) -> Gotgctl>(&self, f: F) -> &Self {
        self.gotgctl_reg().with(f);
        self
    }

    #[doc="Get the GOTGINT Register."]
    #[inline] pub fn gotgint_reg(&self) -> ::bobbin_mcu::register::Register<Gotgint> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Gotgint, 0x4)
    }

    #[doc="Get the *mut pointer for the GOTGINT register."]
    #[inline] pub fn gotgint_mut(&self) -> *mut Gotgint { 
        self.gotgint_reg().ptr()
    }

    #[doc="Get the *const pointer for the GOTGINT register."]
    #[inline] pub fn gotgint_ptr(&self) -> *const Gotgint { 
        self.gotgint_reg().ptr()
    }

    #[doc="Read the GOTGINT register."]
    #[inline] pub fn gotgint(&self) -> Gotgint { 
        self.gotgint_reg().read()
    }

    #[doc="Write the GOTGINT register."]
    #[inline] pub fn write_gotgint(&self, value: Gotgint) -> &Self { 
        self.gotgint_reg().write(value);
        self
    }

    #[doc="Set the GOTGINT register."]
    #[inline] pub fn set_gotgint<F: FnOnce(Gotgint) -> Gotgint>(&self, f: F) -> &Self {
        self.gotgint_reg().set(f);
        self
    }

    #[doc="Modify the GOTGINT register."]
    #[inline] pub fn with_gotgint<F: FnOnce(Gotgint) -> Gotgint>(&self, f: F) -> &Self {
        self.gotgint_reg().with(f);
        self
    }

    #[doc="Get the GAHBCFG Register."]
    #[inline] pub fn gahbcfg_reg(&self) -> ::bobbin_mcu::register::Register<Gahbcfg> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Gahbcfg, 0x8)
    }

    #[doc="Get the *mut pointer for the GAHBCFG register."]
    #[inline] pub fn gahbcfg_mut(&self) -> *mut Gahbcfg { 
        self.gahbcfg_reg().ptr()
    }

    #[doc="Get the *const pointer for the GAHBCFG register."]
    #[inline] pub fn gahbcfg_ptr(&self) -> *const Gahbcfg { 
        self.gahbcfg_reg().ptr()
    }

    #[doc="Read the GAHBCFG register."]
    #[inline] pub fn gahbcfg(&self) -> Gahbcfg { 
        self.gahbcfg_reg().read()
    }

    #[doc="Write the GAHBCFG register."]
    #[inline] pub fn write_gahbcfg(&self, value: Gahbcfg) -> &Self { 
        self.gahbcfg_reg().write(value);
        self
    }

    #[doc="Set the GAHBCFG register."]
    #[inline] pub fn set_gahbcfg<F: FnOnce(Gahbcfg) -> Gahbcfg>(&self, f: F) -> &Self {
        self.gahbcfg_reg().set(f);
        self
    }

    #[doc="Modify the GAHBCFG register."]
    #[inline] pub fn with_gahbcfg<F: FnOnce(Gahbcfg) -> Gahbcfg>(&self, f: F) -> &Self {
        self.gahbcfg_reg().with(f);
        self
    }

    #[doc="Get the GUSBCFG Register."]
    #[inline] pub fn gusbcfg_reg(&self) -> ::bobbin_mcu::register::Register<Gusbcfg> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Gusbcfg, 0xc)
    }

    #[doc="Get the *mut pointer for the GUSBCFG register."]
    #[inline] pub fn gusbcfg_mut(&self) -> *mut Gusbcfg { 
        self.gusbcfg_reg().ptr()
    }

    #[doc="Get the *const pointer for the GUSBCFG register."]
    #[inline] pub fn gusbcfg_ptr(&self) -> *const Gusbcfg { 
        self.gusbcfg_reg().ptr()
    }

    #[doc="Read the GUSBCFG register."]
    #[inline] pub fn gusbcfg(&self) -> Gusbcfg { 
        self.gusbcfg_reg().read()
    }

    #[doc="Write the GUSBCFG register."]
    #[inline] pub fn write_gusbcfg(&self, value: Gusbcfg) -> &Self { 
        self.gusbcfg_reg().write(value);
        self
    }

    #[doc="Set the GUSBCFG register."]
    #[inline] pub fn set_gusbcfg<F: FnOnce(Gusbcfg) -> Gusbcfg>(&self, f: F) -> &Self {
        self.gusbcfg_reg().set(f);
        self
    }

    #[doc="Modify the GUSBCFG register."]
    #[inline] pub fn with_gusbcfg<F: FnOnce(Gusbcfg) -> Gusbcfg>(&self, f: F) -> &Self {
        self.gusbcfg_reg().with(f);
        self
    }

    #[doc="Get the GRSTCTL Register."]
    #[inline] pub fn grstctl_reg(&self) -> ::bobbin_mcu::register::Register<Grstctl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Grstctl, 0x10)
    }

    #[doc="Get the *mut pointer for the GRSTCTL register."]
    #[inline] pub fn grstctl_mut(&self) -> *mut Grstctl { 
        self.grstctl_reg().ptr()
    }

    #[doc="Get the *const pointer for the GRSTCTL register."]
    #[inline] pub fn grstctl_ptr(&self) -> *const Grstctl { 
        self.grstctl_reg().ptr()
    }

    #[doc="Read the GRSTCTL register."]
    #[inline] pub fn grstctl(&self) -> Grstctl { 
        self.grstctl_reg().read()
    }

    #[doc="Write the GRSTCTL register."]
    #[inline] pub fn write_grstctl(&self, value: Grstctl) -> &Self { 
        self.grstctl_reg().write(value);
        self
    }

    #[doc="Set the GRSTCTL register."]
    #[inline] pub fn set_grstctl<F: FnOnce(Grstctl) -> Grstctl>(&self, f: F) -> &Self {
        self.grstctl_reg().set(f);
        self
    }

    #[doc="Modify the GRSTCTL register."]
    #[inline] pub fn with_grstctl<F: FnOnce(Grstctl) -> Grstctl>(&self, f: F) -> &Self {
        self.grstctl_reg().with(f);
        self
    }

    #[doc="Get the GINTSTS Register."]
    #[inline] pub fn gintsts_reg(&self) -> ::bobbin_mcu::register::Register<Gintsts> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Gintsts, 0x14)
    }

    #[doc="Get the *mut pointer for the GINTSTS register."]
    #[inline] pub fn gintsts_mut(&self) -> *mut Gintsts { 
        self.gintsts_reg().ptr()
    }

    #[doc="Get the *const pointer for the GINTSTS register."]
    #[inline] pub fn gintsts_ptr(&self) -> *const Gintsts { 
        self.gintsts_reg().ptr()
    }

    #[doc="Read the GINTSTS register."]
    #[inline] pub fn gintsts(&self) -> Gintsts { 
        self.gintsts_reg().read()
    }

    #[doc="Write the GINTSTS register."]
    #[inline] pub fn write_gintsts(&self, value: Gintsts) -> &Self { 
        self.gintsts_reg().write(value);
        self
    }

    #[doc="Set the GINTSTS register."]
    #[inline] pub fn set_gintsts<F: FnOnce(Gintsts) -> Gintsts>(&self, f: F) -> &Self {
        self.gintsts_reg().set(f);
        self
    }

    #[doc="Modify the GINTSTS register."]
    #[inline] pub fn with_gintsts<F: FnOnce(Gintsts) -> Gintsts>(&self, f: F) -> &Self {
        self.gintsts_reg().with(f);
        self
    }

    #[doc="Get the GINTMSK Register."]
    #[inline] pub fn gintmsk_reg(&self) -> ::bobbin_mcu::register::Register<Gintmsk> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Gintmsk, 0x18)
    }

    #[doc="Get the *mut pointer for the GINTMSK register."]
    #[inline] pub fn gintmsk_mut(&self) -> *mut Gintmsk { 
        self.gintmsk_reg().ptr()
    }

    #[doc="Get the *const pointer for the GINTMSK register."]
    #[inline] pub fn gintmsk_ptr(&self) -> *const Gintmsk { 
        self.gintmsk_reg().ptr()
    }

    #[doc="Read the GINTMSK register."]
    #[inline] pub fn gintmsk(&self) -> Gintmsk { 
        self.gintmsk_reg().read()
    }

    #[doc="Write the GINTMSK register."]
    #[inline] pub fn write_gintmsk(&self, value: Gintmsk) -> &Self { 
        self.gintmsk_reg().write(value);
        self
    }

    #[doc="Set the GINTMSK register."]
    #[inline] pub fn set_gintmsk<F: FnOnce(Gintmsk) -> Gintmsk>(&self, f: F) -> &Self {
        self.gintmsk_reg().set(f);
        self
    }

    #[doc="Modify the GINTMSK register."]
    #[inline] pub fn with_gintmsk<F: FnOnce(Gintmsk) -> Gintmsk>(&self, f: F) -> &Self {
        self.gintmsk_reg().with(f);
        self
    }

    #[doc="Get the GRXSTSR_DEVICE Register."]
    #[inline] pub fn grxstsr_device_reg(&self) -> ::bobbin_mcu::register::Register<GrxstsrDevice> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut GrxstsrDevice, 0x1c)
    }

    #[doc="Get the *mut pointer for the GRXSTSR_DEVICE register."]
    #[inline] pub fn grxstsr_device_mut(&self) -> *mut GrxstsrDevice { 
        self.grxstsr_device_reg().ptr()
    }

    #[doc="Get the *const pointer for the GRXSTSR_DEVICE register."]
    #[inline] pub fn grxstsr_device_ptr(&self) -> *const GrxstsrDevice { 
        self.grxstsr_device_reg().ptr()
    }

    #[doc="Read the GRXSTSR_DEVICE register."]
    #[inline] pub fn grxstsr_device(&self) -> GrxstsrDevice { 
        self.grxstsr_device_reg().read()
    }

    #[doc="Get the GRXSTSR_HOST Register."]
    #[inline] pub fn grxstsr_host_reg(&self) -> ::bobbin_mcu::register::Register<GrxstsrHost> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut GrxstsrHost, 0x1c)
    }

    #[doc="Get the *mut pointer for the GRXSTSR_HOST register."]
    #[inline] pub fn grxstsr_host_mut(&self) -> *mut GrxstsrHost { 
        self.grxstsr_host_reg().ptr()
    }

    #[doc="Get the *const pointer for the GRXSTSR_HOST register."]
    #[inline] pub fn grxstsr_host_ptr(&self) -> *const GrxstsrHost { 
        self.grxstsr_host_reg().ptr()
    }

    #[doc="Read the GRXSTSR_HOST register."]
    #[inline] pub fn grxstsr_host(&self) -> GrxstsrHost { 
        self.grxstsr_host_reg().read()
    }

    #[doc="Get the GRXFSIZ Register."]
    #[inline] pub fn grxfsiz_reg(&self) -> ::bobbin_mcu::register::Register<Grxfsiz> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Grxfsiz, 0x24)
    }

    #[doc="Get the *mut pointer for the GRXFSIZ register."]
    #[inline] pub fn grxfsiz_mut(&self) -> *mut Grxfsiz { 
        self.grxfsiz_reg().ptr()
    }

    #[doc="Get the *const pointer for the GRXFSIZ register."]
    #[inline] pub fn grxfsiz_ptr(&self) -> *const Grxfsiz { 
        self.grxfsiz_reg().ptr()
    }

    #[doc="Read the GRXFSIZ register."]
    #[inline] pub fn grxfsiz(&self) -> Grxfsiz { 
        self.grxfsiz_reg().read()
    }

    #[doc="Write the GRXFSIZ register."]
    #[inline] pub fn write_grxfsiz(&self, value: Grxfsiz) -> &Self { 
        self.grxfsiz_reg().write(value);
        self
    }

    #[doc="Set the GRXFSIZ register."]
    #[inline] pub fn set_grxfsiz<F: FnOnce(Grxfsiz) -> Grxfsiz>(&self, f: F) -> &Self {
        self.grxfsiz_reg().set(f);
        self
    }

    #[doc="Modify the GRXFSIZ register."]
    #[inline] pub fn with_grxfsiz<F: FnOnce(Grxfsiz) -> Grxfsiz>(&self, f: F) -> &Self {
        self.grxfsiz_reg().with(f);
        self
    }

    #[doc="Get the DIEPTXF0 Register."]
    #[inline] pub fn dieptxf0_reg(&self) -> ::bobbin_mcu::register::Register<Dieptxf0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dieptxf0, 0x28)
    }

    #[doc="Get the *mut pointer for the DIEPTXF0 register."]
    #[inline] pub fn dieptxf0_mut(&self) -> *mut Dieptxf0 { 
        self.dieptxf0_reg().ptr()
    }

    #[doc="Get the *const pointer for the DIEPTXF0 register."]
    #[inline] pub fn dieptxf0_ptr(&self) -> *const Dieptxf0 { 
        self.dieptxf0_reg().ptr()
    }

    #[doc="Read the DIEPTXF0 register."]
    #[inline] pub fn dieptxf0(&self) -> Dieptxf0 { 
        self.dieptxf0_reg().read()
    }

    #[doc="Write the DIEPTXF0 register."]
    #[inline] pub fn write_dieptxf0(&self, value: Dieptxf0) -> &Self { 
        self.dieptxf0_reg().write(value);
        self
    }

    #[doc="Set the DIEPTXF0 register."]
    #[inline] pub fn set_dieptxf0<F: FnOnce(Dieptxf0) -> Dieptxf0>(&self, f: F) -> &Self {
        self.dieptxf0_reg().set(f);
        self
    }

    #[doc="Modify the DIEPTXF0 register."]
    #[inline] pub fn with_dieptxf0<F: FnOnce(Dieptxf0) -> Dieptxf0>(&self, f: F) -> &Self {
        self.dieptxf0_reg().with(f);
        self
    }

    #[doc="Get the HNPTXFSIZ_HOST Register."]
    #[inline] pub fn hnptxfsiz_host_reg(&self) -> ::bobbin_mcu::register::Register<HnptxfsizHost> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut HnptxfsizHost, 0x28)
    }

    #[doc="Get the *mut pointer for the HNPTXFSIZ_HOST register."]
    #[inline] pub fn hnptxfsiz_host_mut(&self) -> *mut HnptxfsizHost { 
        self.hnptxfsiz_host_reg().ptr()
    }

    #[doc="Get the *const pointer for the HNPTXFSIZ_HOST register."]
    #[inline] pub fn hnptxfsiz_host_ptr(&self) -> *const HnptxfsizHost { 
        self.hnptxfsiz_host_reg().ptr()
    }

    #[doc="Read the HNPTXFSIZ_HOST register."]
    #[inline] pub fn hnptxfsiz_host(&self) -> HnptxfsizHost { 
        self.hnptxfsiz_host_reg().read()
    }

    #[doc="Write the HNPTXFSIZ_HOST register."]
    #[inline] pub fn write_hnptxfsiz_host(&self, value: HnptxfsizHost) -> &Self { 
        self.hnptxfsiz_host_reg().write(value);
        self
    }

    #[doc="Set the HNPTXFSIZ_HOST register."]
    #[inline] pub fn set_hnptxfsiz_host<F: FnOnce(HnptxfsizHost) -> HnptxfsizHost>(&self, f: F) -> &Self {
        self.hnptxfsiz_host_reg().set(f);
        self
    }

    #[doc="Modify the HNPTXFSIZ_HOST register."]
    #[inline] pub fn with_hnptxfsiz_host<F: FnOnce(HnptxfsizHost) -> HnptxfsizHost>(&self, f: F) -> &Self {
        self.hnptxfsiz_host_reg().with(f);
        self
    }

    #[doc="Get the HNPTXSTS Register."]
    #[inline] pub fn hnptxsts_reg(&self) -> ::bobbin_mcu::register::Register<Hnptxsts> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hnptxsts, 0x2c)
    }

    #[doc="Get the *mut pointer for the HNPTXSTS register."]
    #[inline] pub fn hnptxsts_mut(&self) -> *mut Hnptxsts { 
        self.hnptxsts_reg().ptr()
    }

    #[doc="Get the *const pointer for the HNPTXSTS register."]
    #[inline] pub fn hnptxsts_ptr(&self) -> *const Hnptxsts { 
        self.hnptxsts_reg().ptr()
    }

    #[doc="Read the HNPTXSTS register."]
    #[inline] pub fn hnptxsts(&self) -> Hnptxsts { 
        self.hnptxsts_reg().read()
    }

    #[doc="Get the GCCFG Register."]
    #[inline] pub fn gccfg_reg(&self) -> ::bobbin_mcu::register::Register<Gccfg> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Gccfg, 0x38)
    }

    #[doc="Get the *mut pointer for the GCCFG register."]
    #[inline] pub fn gccfg_mut(&self) -> *mut Gccfg { 
        self.gccfg_reg().ptr()
    }

    #[doc="Get the *const pointer for the GCCFG register."]
    #[inline] pub fn gccfg_ptr(&self) -> *const Gccfg { 
        self.gccfg_reg().ptr()
    }

    #[doc="Read the GCCFG register."]
    #[inline] pub fn gccfg(&self) -> Gccfg { 
        self.gccfg_reg().read()
    }

    #[doc="Write the GCCFG register."]
    #[inline] pub fn write_gccfg(&self, value: Gccfg) -> &Self { 
        self.gccfg_reg().write(value);
        self
    }

    #[doc="Set the GCCFG register."]
    #[inline] pub fn set_gccfg<F: FnOnce(Gccfg) -> Gccfg>(&self, f: F) -> &Self {
        self.gccfg_reg().set(f);
        self
    }

    #[doc="Modify the GCCFG register."]
    #[inline] pub fn with_gccfg<F: FnOnce(Gccfg) -> Gccfg>(&self, f: F) -> &Self {
        self.gccfg_reg().with(f);
        self
    }

    #[doc="Get the CID Register."]
    #[inline] pub fn cid_reg(&self) -> ::bobbin_mcu::register::Register<Cid> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cid, 0x3c)
    }

    #[doc="Get the *mut pointer for the CID register."]
    #[inline] pub fn cid_mut(&self) -> *mut Cid { 
        self.cid_reg().ptr()
    }

    #[doc="Get the *const pointer for the CID register."]
    #[inline] pub fn cid_ptr(&self) -> *const Cid { 
        self.cid_reg().ptr()
    }

    #[doc="Read the CID register."]
    #[inline] pub fn cid(&self) -> Cid { 
        self.cid_reg().read()
    }

    #[doc="Write the CID register."]
    #[inline] pub fn write_cid(&self, value: Cid) -> &Self { 
        self.cid_reg().write(value);
        self
    }

    #[doc="Set the CID register."]
    #[inline] pub fn set_cid<F: FnOnce(Cid) -> Cid>(&self, f: F) -> &Self {
        self.cid_reg().set(f);
        self
    }

    #[doc="Modify the CID register."]
    #[inline] pub fn with_cid<F: FnOnce(Cid) -> Cid>(&self, f: F) -> &Self {
        self.cid_reg().with(f);
        self
    }

    #[doc="Get the HPTXFSIZ Register."]
    #[inline] pub fn hptxfsiz_reg(&self) -> ::bobbin_mcu::register::Register<Hptxfsiz> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hptxfsiz, 0x100)
    }

    #[doc="Get the *mut pointer for the HPTXFSIZ register."]
    #[inline] pub fn hptxfsiz_mut(&self) -> *mut Hptxfsiz { 
        self.hptxfsiz_reg().ptr()
    }

    #[doc="Get the *const pointer for the HPTXFSIZ register."]
    #[inline] pub fn hptxfsiz_ptr(&self) -> *const Hptxfsiz { 
        self.hptxfsiz_reg().ptr()
    }

    #[doc="Read the HPTXFSIZ register."]
    #[inline] pub fn hptxfsiz(&self) -> Hptxfsiz { 
        self.hptxfsiz_reg().read()
    }

    #[doc="Write the HPTXFSIZ register."]
    #[inline] pub fn write_hptxfsiz(&self, value: Hptxfsiz) -> &Self { 
        self.hptxfsiz_reg().write(value);
        self
    }

    #[doc="Set the HPTXFSIZ register."]
    #[inline] pub fn set_hptxfsiz<F: FnOnce(Hptxfsiz) -> Hptxfsiz>(&self, f: F) -> &Self {
        self.hptxfsiz_reg().set(f);
        self
    }

    #[doc="Modify the HPTXFSIZ register."]
    #[inline] pub fn with_hptxfsiz<F: FnOnce(Hptxfsiz) -> Hptxfsiz>(&self, f: F) -> &Self {
        self.hptxfsiz_reg().with(f);
        self
    }

    #[doc="Get the DIEPTXF Register."]
    #[inline] pub fn dieptxf_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Dieptxf, ::bobbin_bits::R6> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Dieptxf, 0x100, 0x4)
    }

    #[doc="Get the *mut pointer for the DIEPTXF register."]
    #[inline] pub fn dieptxf_mut<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *mut Dieptxf { 
        self.dieptxf_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DIEPTXF register."]
    #[inline] pub fn dieptxf_ptr<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *const Dieptxf { 
        self.dieptxf_reg().ptr(index.into())
    }

    #[doc="Read the DIEPTXF register."]
    #[inline] pub fn dieptxf<I: Into<::bobbin_bits::R6>>(&self, index: I) -> Dieptxf { 
        self.dieptxf_reg().read(index.into())
    }

    #[doc="Write the DIEPTXF register."]
    #[inline] pub fn write_dieptxf<I: Into<::bobbin_bits::R6>>(&self, index: I, value: Dieptxf) -> &Self {
        self.dieptxf_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DIEPTXF register."]
    #[inline] pub fn set_dieptxf<I: Into<::bobbin_bits::R6>, F: FnOnce(Dieptxf) -> Dieptxf>(&self, index: I, f: F) -> &Self {
        self.dieptxf_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DIEPTXF register."]
    #[inline] pub fn with_dieptxf<I: Into<::bobbin_bits::R6> + Copy, F: FnOnce(Dieptxf) -> Dieptxf>(&self, index: I, f: F) -> &Self {
        self.dieptxf_reg().with(index.into(), f);
        self
    }

    #[doc="Get the GRXSTSP_DEVICE Register."]
    #[inline] pub fn grxstsp_device_reg(&self) -> ::bobbin_mcu::register::Register<GrxstspDevice> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut GrxstspDevice, 0x20)
    }

    #[doc="Get the *mut pointer for the GRXSTSP_DEVICE register."]
    #[inline] pub fn grxstsp_device_mut(&self) -> *mut GrxstspDevice { 
        self.grxstsp_device_reg().ptr()
    }

    #[doc="Get the *const pointer for the GRXSTSP_DEVICE register."]
    #[inline] pub fn grxstsp_device_ptr(&self) -> *const GrxstspDevice { 
        self.grxstsp_device_reg().ptr()
    }

    #[doc="Read the GRXSTSP_DEVICE register."]
    #[inline] pub fn grxstsp_device(&self) -> GrxstspDevice { 
        self.grxstsp_device_reg().read()
    }

    #[doc="Get the GRXSTSP_HOST Register."]
    #[inline] pub fn grxstsp_host_reg(&self) -> ::bobbin_mcu::register::Register<GrxstspHost> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut GrxstspHost, 0x20)
    }

    #[doc="Get the *mut pointer for the GRXSTSP_HOST register."]
    #[inline] pub fn grxstsp_host_mut(&self) -> *mut GrxstspHost { 
        self.grxstsp_host_reg().ptr()
    }

    #[doc="Get the *const pointer for the GRXSTSP_HOST register."]
    #[inline] pub fn grxstsp_host_ptr(&self) -> *const GrxstspHost { 
        self.grxstsp_host_reg().ptr()
    }

    #[doc="Read the GRXSTSP_HOST register."]
    #[inline] pub fn grxstsp_host(&self) -> GrxstspHost { 
        self.grxstsp_host_reg().read()
    }

    #[doc="Get the GI2CCTL Register."]
    #[inline] pub fn gi2cctl_reg(&self) -> ::bobbin_mcu::register::Register<Gi2cctl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Gi2cctl, 0x30)
    }

    #[doc="Get the *mut pointer for the GI2CCTL register."]
    #[inline] pub fn gi2cctl_mut(&self) -> *mut Gi2cctl { 
        self.gi2cctl_reg().ptr()
    }

    #[doc="Get the *const pointer for the GI2CCTL register."]
    #[inline] pub fn gi2cctl_ptr(&self) -> *const Gi2cctl { 
        self.gi2cctl_reg().ptr()
    }

    #[doc="Read the GI2CCTL register."]
    #[inline] pub fn gi2cctl(&self) -> Gi2cctl { 
        self.gi2cctl_reg().read()
    }

    #[doc="Write the GI2CCTL register."]
    #[inline] pub fn write_gi2cctl(&self, value: Gi2cctl) -> &Self { 
        self.gi2cctl_reg().write(value);
        self
    }

    #[doc="Set the GI2CCTL register."]
    #[inline] pub fn set_gi2cctl<F: FnOnce(Gi2cctl) -> Gi2cctl>(&self, f: F) -> &Self {
        self.gi2cctl_reg().set(f);
        self
    }

    #[doc="Modify the GI2CCTL register."]
    #[inline] pub fn with_gi2cctl<F: FnOnce(Gi2cctl) -> Gi2cctl>(&self, f: F) -> &Self {
        self.gi2cctl_reg().with(f);
        self
    }

    #[doc="Get the GPWRDN Register."]
    #[inline] pub fn gpwrdn_reg(&self) -> ::bobbin_mcu::register::Register<Gpwrdn> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Gpwrdn, 0x58)
    }

    #[doc="Get the *mut pointer for the GPWRDN register."]
    #[inline] pub fn gpwrdn_mut(&self) -> *mut Gpwrdn { 
        self.gpwrdn_reg().ptr()
    }

    #[doc="Get the *const pointer for the GPWRDN register."]
    #[inline] pub fn gpwrdn_ptr(&self) -> *const Gpwrdn { 
        self.gpwrdn_reg().ptr()
    }

    #[doc="Read the GPWRDN register."]
    #[inline] pub fn gpwrdn(&self) -> Gpwrdn { 
        self.gpwrdn_reg().read()
    }

    #[doc="Write the GPWRDN register."]
    #[inline] pub fn write_gpwrdn(&self, value: Gpwrdn) -> &Self { 
        self.gpwrdn_reg().write(value);
        self
    }

    #[doc="Set the GPWRDN register."]
    #[inline] pub fn set_gpwrdn<F: FnOnce(Gpwrdn) -> Gpwrdn>(&self, f: F) -> &Self {
        self.gpwrdn_reg().set(f);
        self
    }

    #[doc="Modify the GPWRDN register."]
    #[inline] pub fn with_gpwrdn<F: FnOnce(Gpwrdn) -> Gpwrdn>(&self, f: F) -> &Self {
        self.gpwrdn_reg().with(f);
        self
    }

    #[doc="Get the GADPCTL Register."]
    #[inline] pub fn gadpctl_reg(&self) -> ::bobbin_mcu::register::Register<Gadpctl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Gadpctl, 0x60)
    }

    #[doc="Get the *mut pointer for the GADPCTL register."]
    #[inline] pub fn gadpctl_mut(&self) -> *mut Gadpctl { 
        self.gadpctl_reg().ptr()
    }

    #[doc="Get the *const pointer for the GADPCTL register."]
    #[inline] pub fn gadpctl_ptr(&self) -> *const Gadpctl { 
        self.gadpctl_reg().ptr()
    }

    #[doc="Read the GADPCTL register."]
    #[inline] pub fn gadpctl(&self) -> Gadpctl { 
        self.gadpctl_reg().read()
    }

    #[doc="Write the GADPCTL register."]
    #[inline] pub fn write_gadpctl(&self, value: Gadpctl) -> &Self { 
        self.gadpctl_reg().write(value);
        self
    }

    #[doc="Set the GADPCTL register."]
    #[inline] pub fn set_gadpctl<F: FnOnce(Gadpctl) -> Gadpctl>(&self, f: F) -> &Self {
        self.gadpctl_reg().set(f);
        self
    }

    #[doc="Modify the GADPCTL register."]
    #[inline] pub fn with_gadpctl<F: FnOnce(Gadpctl) -> Gadpctl>(&self, f: F) -> &Self {
        self.gadpctl_reg().with(f);
        self
    }

    #[doc="Get the GLPMCFG Register."]
    #[inline] pub fn glpmcfg_reg(&self) -> ::bobbin_mcu::register::Register<Glpmcfg> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Glpmcfg, 0x54)
    }

    #[doc="Get the *mut pointer for the GLPMCFG register."]
    #[inline] pub fn glpmcfg_mut(&self) -> *mut Glpmcfg { 
        self.glpmcfg_reg().ptr()
    }

    #[doc="Get the *const pointer for the GLPMCFG register."]
    #[inline] pub fn glpmcfg_ptr(&self) -> *const Glpmcfg { 
        self.glpmcfg_reg().ptr()
    }

    #[doc="Read the GLPMCFG register."]
    #[inline] pub fn glpmcfg(&self) -> Glpmcfg { 
        self.glpmcfg_reg().read()
    }

    #[doc="Write the GLPMCFG register."]
    #[inline] pub fn write_glpmcfg(&self, value: Glpmcfg) -> &Self { 
        self.glpmcfg_reg().write(value);
        self
    }

    #[doc="Set the GLPMCFG register."]
    #[inline] pub fn set_glpmcfg<F: FnOnce(Glpmcfg) -> Glpmcfg>(&self, f: F) -> &Self {
        self.glpmcfg_reg().set(f);
        self
    }

    #[doc="Modify the GLPMCFG register."]
    #[inline] pub fn with_glpmcfg<F: FnOnce(Glpmcfg) -> Glpmcfg>(&self, f: F) -> &Self {
        self.glpmcfg_reg().with(f);
        self
    }

}

#[doc="OTG_FS control and status register (OTG_FS_GOTGCTL)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gotgctl(pub u32);
impl Gotgctl {
    #[doc="Session request success"]
    #[inline] pub fn srqscs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SRQSCS != 0"]
    #[inline] pub fn test_srqscs(&self) -> bool {
        self.srqscs() != 0
    }

    #[doc="Sets the SRQSCS field."]
    #[inline] pub fn set_srqscs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Session request"]
    #[inline] pub fn srq(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SRQ != 0"]
    #[inline] pub fn test_srq(&self) -> bool {
        self.srq() != 0
    }

    #[doc="Sets the SRQ field."]
    #[inline] pub fn set_srq<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Host negotiation success"]
    #[inline] pub fn hngscs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if HNGSCS != 0"]
    #[inline] pub fn test_hngscs(&self) -> bool {
        self.hngscs() != 0
    }

    #[doc="Sets the HNGSCS field."]
    #[inline] pub fn set_hngscs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="HNP request"]
    #[inline] pub fn hnprq(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if HNPRQ != 0"]
    #[inline] pub fn test_hnprq(&self) -> bool {
        self.hnprq() != 0
    }

    #[doc="Sets the HNPRQ field."]
    #[inline] pub fn set_hnprq<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Host set HNP enable"]
    #[inline] pub fn hshnpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if HSHNPEN != 0"]
    #[inline] pub fn test_hshnpen(&self) -> bool {
        self.hshnpen() != 0
    }

    #[doc="Sets the HSHNPEN field."]
    #[inline] pub fn set_hshnpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Device HNP enabled"]
    #[inline] pub fn dhnpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DHNPEN != 0"]
    #[inline] pub fn test_dhnpen(&self) -> bool {
        self.dhnpen() != 0
    }

    #[doc="Sets the DHNPEN field."]
    #[inline] pub fn set_dhnpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Connector ID status"]
    #[inline] pub fn cidsts(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if CIDSTS != 0"]
    #[inline] pub fn test_cidsts(&self) -> bool {
        self.cidsts() != 0
    }

    #[doc="Sets the CIDSTS field."]
    #[inline] pub fn set_cidsts<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Long/short debounce time"]
    #[inline] pub fn dbct(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DBCT != 0"]
    #[inline] pub fn test_dbct(&self) -> bool {
        self.dbct() != 0
    }

    #[doc="Sets the DBCT field."]
    #[inline] pub fn set_dbct<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="A-session valid"]
    #[inline] pub fn asvld(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if ASVLD != 0"]
    #[inline] pub fn test_asvld(&self) -> bool {
        self.asvld() != 0
    }

    #[doc="Sets the ASVLD field."]
    #[inline] pub fn set_asvld<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="B-session valid"]
    #[inline] pub fn bsvld(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if BSVLD != 0"]
    #[inline] pub fn test_bsvld(&self) -> bool {
        self.bsvld() != 0
    }

    #[doc="Sets the BSVLD field."]
    #[inline] pub fn set_bsvld<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="VBUS valid override enable"]
    #[inline] pub fn vbvaloen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if VBVALOEN != 0"]
    #[inline] pub fn test_vbvaloen(&self) -> bool {
        self.vbvaloen() != 0
    }

    #[doc="Sets the VBVALOEN field."]
    #[inline] pub fn set_vbvaloen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="VBUS valid override value"]
    #[inline] pub fn vbvaloval(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if VBVALOVAL != 0"]
    #[inline] pub fn test_vbvaloval(&self) -> bool {
        self.vbvaloval() != 0
    }

    #[doc="Sets the VBVALOVAL field."]
    #[inline] pub fn set_vbvaloval<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="A-peripheral session valid override enable"]
    #[inline] pub fn avaloen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if AVALOEN != 0"]
    #[inline] pub fn test_avaloen(&self) -> bool {
        self.avaloen() != 0
    }

    #[doc="Sets the AVALOEN field."]
    #[inline] pub fn set_avaloen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="A-peripheral session valid override value"]
    #[inline] pub fn avaloval(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if AVALOVAL != 0"]
    #[inline] pub fn test_avaloval(&self) -> bool {
        self.avaloval() != 0
    }

    #[doc="Sets the AVALOVAL field."]
    #[inline] pub fn set_avaloval<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="B-peripheral session valid override enable"]
    #[inline] pub fn bvaloen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if BVALOEN != 0"]
    #[inline] pub fn test_bvaloen(&self) -> bool {
        self.bvaloen() != 0
    }

    #[doc="Sets the BVALOEN field."]
    #[inline] pub fn set_bvaloen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="B-peripheral session valid override value"]
    #[inline] pub fn bvaloval(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if BVALOVAL != 0"]
    #[inline] pub fn test_bvaloval(&self) -> bool {
        self.bvaloval() != 0
    }

    #[doc="Sets the BVALOVAL field."]
    #[inline] pub fn set_bvaloval<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Embedded host enable"]
    #[inline] pub fn ehen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if EHEN != 0"]
    #[inline] pub fn test_ehen(&self) -> bool {
        self.ehen() != 0
    }

    #[doc="Sets the EHEN field."]
    #[inline] pub fn set_ehen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="OTG version"]
    #[inline] pub fn otgver(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if OTGVER != 0"]
    #[inline] pub fn test_otgver(&self) -> bool {
        self.otgver() != 0
    }

    #[doc="Sets the OTGVER field."]
    #[inline] pub fn set_otgver<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

}

impl From<u32> for Gotgctl {
    #[inline]
    fn from(other: u32) -> Self {
         Gotgctl(other)
    }
}

impl ::core::fmt::Display for Gotgctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gotgctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.srqscs() != 0 { try!(write!(f, " srqscs"))}
        if self.srq() != 0 { try!(write!(f, " srq"))}
        if self.hngscs() != 0 { try!(write!(f, " hngscs"))}
        if self.hnprq() != 0 { try!(write!(f, " hnprq"))}
        if self.hshnpen() != 0 { try!(write!(f, " hshnpen"))}
        if self.dhnpen() != 0 { try!(write!(f, " dhnpen"))}
        if self.cidsts() != 0 { try!(write!(f, " cidsts"))}
        if self.dbct() != 0 { try!(write!(f, " dbct"))}
        if self.asvld() != 0 { try!(write!(f, " asvld"))}
        if self.bsvld() != 0 { try!(write!(f, " bsvld"))}
        if self.vbvaloen() != 0 { try!(write!(f, " vbvaloen"))}
        if self.vbvaloval() != 0 { try!(write!(f, " vbvaloval"))}
        if self.avaloen() != 0 { try!(write!(f, " avaloen"))}
        if self.avaloval() != 0 { try!(write!(f, " avaloval"))}
        if self.bvaloen() != 0 { try!(write!(f, " bvaloen"))}
        if self.bvaloval() != 0 { try!(write!(f, " bvaloval"))}
        if self.ehen() != 0 { try!(write!(f, " ehen"))}
        if self.otgver() != 0 { try!(write!(f, " otgver"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS interrupt register (OTG_FS_GOTGINT)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gotgint(pub u32);
impl Gotgint {
    #[doc="Session end detected"]
    #[inline] pub fn sedet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SEDET != 0"]
    #[inline] pub fn test_sedet(&self) -> bool {
        self.sedet() != 0
    }

    #[doc="Sets the SEDET field."]
    #[inline] pub fn set_sedet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Session request success status change"]
    #[inline] pub fn srsschg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SRSSCHG != 0"]
    #[inline] pub fn test_srsschg(&self) -> bool {
        self.srsschg() != 0
    }

    #[doc="Sets the SRSSCHG field."]
    #[inline] pub fn set_srsschg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Host negotiation success status change"]
    #[inline] pub fn hnsschg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if HNSSCHG != 0"]
    #[inline] pub fn test_hnsschg(&self) -> bool {
        self.hnsschg() != 0
    }

    #[doc="Sets the HNSSCHG field."]
    #[inline] pub fn set_hnsschg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Host negotiation detected"]
    #[inline] pub fn hngdet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if HNGDET != 0"]
    #[inline] pub fn test_hngdet(&self) -> bool {
        self.hngdet() != 0
    }

    #[doc="Sets the HNGDET field."]
    #[inline] pub fn set_hngdet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="A-device timeout change"]
    #[inline] pub fn adtochg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if ADTOCHG != 0"]
    #[inline] pub fn test_adtochg(&self) -> bool {
        self.adtochg() != 0
    }

    #[doc="Sets the ADTOCHG field."]
    #[inline] pub fn set_adtochg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Debounce done"]
    #[inline] pub fn dbcdne(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if DBCDNE != 0"]
    #[inline] pub fn test_dbcdne(&self) -> bool {
        self.dbcdne() != 0
    }

    #[doc="Sets the DBCDNE field."]
    #[inline] pub fn set_dbcdne<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="ID input pin changed"]
    #[inline] pub fn idchng(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if IDCHNG != 0"]
    #[inline] pub fn test_idchng(&self) -> bool {
        self.idchng() != 0
    }

    #[doc="Sets the IDCHNG field."]
    #[inline] pub fn set_idchng<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

}

impl From<u32> for Gotgint {
    #[inline]
    fn from(other: u32) -> Self {
         Gotgint(other)
    }
}

impl ::core::fmt::Display for Gotgint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gotgint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sedet() != 0 { try!(write!(f, " sedet"))}
        if self.srsschg() != 0 { try!(write!(f, " srsschg"))}
        if self.hnsschg() != 0 { try!(write!(f, " hnsschg"))}
        if self.hngdet() != 0 { try!(write!(f, " hngdet"))}
        if self.adtochg() != 0 { try!(write!(f, " adtochg"))}
        if self.dbcdne() != 0 { try!(write!(f, " dbcdne"))}
        if self.idchng() != 0 { try!(write!(f, " idchng"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gahbcfg(pub u32);
impl Gahbcfg {
    #[doc="Global interrupt mask"]
    #[inline] pub fn gint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if GINT != 0"]
    #[inline] pub fn test_gint(&self) -> bool {
        self.gint() != 0
    }

    #[doc="Sets the GINT field."]
    #[inline] pub fn set_gint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="TxFIFO empty level"]
    #[inline] pub fn txfelvl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXFELVL != 0"]
    #[inline] pub fn test_txfelvl(&self) -> bool {
        self.txfelvl() != 0
    }

    #[doc="Sets the TXFELVL field."]
    #[inline] pub fn set_txfelvl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Periodic TxFIFO empty level"]
    #[inline] pub fn ptxfelvl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PTXFELVL != 0"]
    #[inline] pub fn test_ptxfelvl(&self) -> bool {
        self.ptxfelvl() != 0
    }

    #[doc="Sets the PTXFELVL field."]
    #[inline] pub fn set_ptxfelvl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Gahbcfg {
    #[inline]
    fn from(other: u32) -> Self {
         Gahbcfg(other)
    }
}

impl ::core::fmt::Display for Gahbcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gahbcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gint() != 0 { try!(write!(f, " gint"))}
        if self.txfelvl() != 0 { try!(write!(f, " txfelvl"))}
        if self.ptxfelvl() != 0 { try!(write!(f, " ptxfelvl"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS USB configuration register (OTG_FS_GUSBCFG)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gusbcfg(pub u32);
impl Gusbcfg {
    #[doc="FS timeout calibration"]
    #[inline] pub fn tocal(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if TOCAL != 0"]
    #[inline] pub fn test_tocal(&self) -> bool {
        self.tocal() != 0
    }

    #[doc="Sets the TOCAL field."]
    #[inline] pub fn set_tocal<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Full Speed serial transceiver select"]
    #[inline] pub fn physel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PHYSEL != 0"]
    #[inline] pub fn test_physel(&self) -> bool {
        self.physel() != 0
    }

    #[doc="Sets the PHYSEL field."]
    #[inline] pub fn set_physel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="SRP-capable"]
    #[inline] pub fn srpcap(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SRPCAP != 0"]
    #[inline] pub fn test_srpcap(&self) -> bool {
        self.srpcap() != 0
    }

    #[doc="Sets the SRPCAP field."]
    #[inline] pub fn set_srpcap<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="HNP-capable"]
    #[inline] pub fn hnpcap(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if HNPCAP != 0"]
    #[inline] pub fn test_hnpcap(&self) -> bool {
        self.hnpcap() != 0
    }

    #[doc="Sets the HNPCAP field."]
    #[inline] pub fn set_hnpcap<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="USB turnaround time"]
    #[inline] pub fn trdt(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0xf) as u8) } // [13:10]
    }

    #[doc="Returns true if TRDT != 0"]
    #[inline] pub fn test_trdt(&self) -> bool {
        self.trdt() != 0
    }

    #[doc="Sets the TRDT field."]
    #[inline] pub fn set_trdt<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Force host mode"]
    #[inline] pub fn fhmod(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if FHMOD != 0"]
    #[inline] pub fn test_fhmod(&self) -> bool {
        self.fhmod() != 0
    }

    #[doc="Sets the FHMOD field."]
    #[inline] pub fn set_fhmod<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Force device mode"]
    #[inline] pub fn fdmod(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if FDMOD != 0"]
    #[inline] pub fn test_fdmod(&self) -> bool {
        self.fdmod() != 0
    }

    #[doc="Sets the FDMOD field."]
    #[inline] pub fn set_fdmod<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

}

impl From<u32> for Gusbcfg {
    #[inline]
    fn from(other: u32) -> Self {
         Gusbcfg(other)
    }
}

impl ::core::fmt::Display for Gusbcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gusbcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tocal() != 0 { try!(write!(f, " tocal=0x{:x}", self.tocal()))}
        if self.physel() != 0 { try!(write!(f, " physel"))}
        if self.srpcap() != 0 { try!(write!(f, " srpcap"))}
        if self.hnpcap() != 0 { try!(write!(f, " hnpcap"))}
        if self.trdt() != 0 { try!(write!(f, " trdt=0x{:x}", self.trdt()))}
        if self.fhmod() != 0 { try!(write!(f, " fhmod"))}
        if self.fdmod() != 0 { try!(write!(f, " fdmod"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS reset register (OTG_FS_GRSTCTL)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Grstctl(pub u32);
impl Grstctl {
    #[doc="Core soft reset"]
    #[inline] pub fn csrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CSRST != 0"]
    #[inline] pub fn test_csrst(&self) -> bool {
        self.csrst() != 0
    }

    #[doc="Sets the CSRST field."]
    #[inline] pub fn set_csrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="HCLK soft reset"]
    #[inline] pub fn hsrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if HSRST != 0"]
    #[inline] pub fn test_hsrst(&self) -> bool {
        self.hsrst() != 0
    }

    #[doc="Sets the HSRST field."]
    #[inline] pub fn set_hsrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Host frame counter reset"]
    #[inline] pub fn fcrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FCRST != 0"]
    #[inline] pub fn test_fcrst(&self) -> bool {
        self.fcrst() != 0
    }

    #[doc="Sets the FCRST field."]
    #[inline] pub fn set_fcrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="RxFIFO flush"]
    #[inline] pub fn rxfflsh(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXFFLSH != 0"]
    #[inline] pub fn test_rxfflsh(&self) -> bool {
        self.rxfflsh() != 0
    }

    #[doc="Sets the RXFFLSH field."]
    #[inline] pub fn set_rxfflsh<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TxFIFO flush"]
    #[inline] pub fn txfflsh(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXFFLSH != 0"]
    #[inline] pub fn test_txfflsh(&self) -> bool {
        self.txfflsh() != 0
    }

    #[doc="Sets the TXFFLSH field."]
    #[inline] pub fn set_txfflsh<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="TxFIFO number"]
    #[inline] pub fn txfnum(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1f) as u8) } // [10:6]
    }

    #[doc="Returns true if TXFNUM != 0"]
    #[inline] pub fn test_txfnum(&self) -> bool {
        self.txfnum() != 0
    }

    #[doc="Sets the TXFNUM field."]
    #[inline] pub fn set_txfnum<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="AHB master idle"]
    #[inline] pub fn ahbidl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if AHBIDL != 0"]
    #[inline] pub fn test_ahbidl(&self) -> bool {
        self.ahbidl() != 0
    }

    #[doc="Sets the AHBIDL field."]
    #[inline] pub fn set_ahbidl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Grstctl {
    #[inline]
    fn from(other: u32) -> Self {
         Grstctl(other)
    }
}

impl ::core::fmt::Display for Grstctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Grstctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.csrst() != 0 { try!(write!(f, " csrst"))}
        if self.hsrst() != 0 { try!(write!(f, " hsrst"))}
        if self.fcrst() != 0 { try!(write!(f, " fcrst"))}
        if self.rxfflsh() != 0 { try!(write!(f, " rxfflsh"))}
        if self.txfflsh() != 0 { try!(write!(f, " txfflsh"))}
        if self.txfnum() != 0 { try!(write!(f, " txfnum=0x{:x}", self.txfnum()))}
        if self.ahbidl() != 0 { try!(write!(f, " ahbidl"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS core interrupt register (OTG_FS_GINTSTS)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gintsts(pub u32);
impl Gintsts {
    #[doc="Current mode of operation"]
    #[inline] pub fn cmod(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CMOD != 0"]
    #[inline] pub fn test_cmod(&self) -> bool {
        self.cmod() != 0
    }

    #[doc="Sets the CMOD field."]
    #[inline] pub fn set_cmod<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Mode mismatch interrupt"]
    #[inline] pub fn mmis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MMIS != 0"]
    #[inline] pub fn test_mmis(&self) -> bool {
        self.mmis() != 0
    }

    #[doc="Sets the MMIS field."]
    #[inline] pub fn set_mmis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="OTG interrupt"]
    #[inline] pub fn otgint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if OTGINT != 0"]
    #[inline] pub fn test_otgint(&self) -> bool {
        self.otgint() != 0
    }

    #[doc="Sets the OTGINT field."]
    #[inline] pub fn set_otgint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Start of frame"]
    #[inline] pub fn sof(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SOF != 0"]
    #[inline] pub fn test_sof(&self) -> bool {
        self.sof() != 0
    }

    #[doc="Sets the SOF field."]
    #[inline] pub fn set_sof<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="RxFIFO non-empty"]
    #[inline] pub fn rxflvl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXFLVL != 0"]
    #[inline] pub fn test_rxflvl(&self) -> bool {
        self.rxflvl() != 0
    }

    #[doc="Sets the RXFLVL field."]
    #[inline] pub fn set_rxflvl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Non-periodic TxFIFO empty"]
    #[inline] pub fn nptxfe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if NPTXFE != 0"]
    #[inline] pub fn test_nptxfe(&self) -> bool {
        self.nptxfe() != 0
    }

    #[doc="Sets the NPTXFE field."]
    #[inline] pub fn set_nptxfe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Global IN non-periodic NAK effective"]
    #[inline] pub fn ginakeff(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if GINAKEFF != 0"]
    #[inline] pub fn test_ginakeff(&self) -> bool {
        self.ginakeff() != 0
    }

    #[doc="Sets the GINAKEFF field."]
    #[inline] pub fn set_ginakeff<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Global OUT NAK effective"]
    #[inline] pub fn goutnakeff(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GOUTNAKEFF != 0"]
    #[inline] pub fn test_goutnakeff(&self) -> bool {
        self.goutnakeff() != 0
    }

    #[doc="Sets the GOUTNAKEFF field."]
    #[inline] pub fn set_goutnakeff<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Early suspend"]
    #[inline] pub fn esusp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if ESUSP != 0"]
    #[inline] pub fn test_esusp(&self) -> bool {
        self.esusp() != 0
    }

    #[doc="Sets the ESUSP field."]
    #[inline] pub fn set_esusp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="USB suspend"]
    #[inline] pub fn usbsusp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if USBSUSP != 0"]
    #[inline] pub fn test_usbsusp(&self) -> bool {
        self.usbsusp() != 0
    }

    #[doc="Sets the USBSUSP field."]
    #[inline] pub fn set_usbsusp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="USB reset"]
    #[inline] pub fn usbrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if USBRST != 0"]
    #[inline] pub fn test_usbrst(&self) -> bool {
        self.usbrst() != 0
    }

    #[doc="Sets the USBRST field."]
    #[inline] pub fn set_usbrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Enumeration done"]
    #[inline] pub fn enumdne(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if ENUMDNE != 0"]
    #[inline] pub fn test_enumdne(&self) -> bool {
        self.enumdne() != 0
    }

    #[doc="Sets the ENUMDNE field."]
    #[inline] pub fn set_enumdne<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Isochronous OUT packet dropped interrupt"]
    #[inline] pub fn isoodrp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if ISOODRP != 0"]
    #[inline] pub fn test_isoodrp(&self) -> bool {
        self.isoodrp() != 0
    }

    #[doc="Sets the ISOODRP field."]
    #[inline] pub fn set_isoodrp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="End of periodic frame interrupt"]
    #[inline] pub fn eopf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if EOPF != 0"]
    #[inline] pub fn test_eopf(&self) -> bool {
        self.eopf() != 0
    }

    #[doc="Sets the EOPF field."]
    #[inline] pub fn set_eopf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="IN endpoint interrupt"]
    #[inline] pub fn iepint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if IEPINT != 0"]
    #[inline] pub fn test_iepint(&self) -> bool {
        self.iepint() != 0
    }

    #[doc="Sets the IEPINT field."]
    #[inline] pub fn set_iepint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="OUT endpoint interrupt"]
    #[inline] pub fn oepint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if OEPINT != 0"]
    #[inline] pub fn test_oepint(&self) -> bool {
        self.oepint() != 0
    }

    #[doc="Sets the OEPINT field."]
    #[inline] pub fn set_oepint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Incomplete isochronous IN transfer"]
    #[inline] pub fn iisoixfr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if IISOIXFR != 0"]
    #[inline] pub fn test_iisoixfr(&self) -> bool {
        self.iisoixfr() != 0
    }

    #[doc="Sets the IISOIXFR field."]
    #[inline] pub fn set_iisoixfr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Incomplete periodic transfer(Host mode)/Incomplete isochronous OUT transfer(Device mode)"]
    #[inline] pub fn ipxfr_incompisoout(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if IPXFR_INCOMPISOOUT != 0"]
    #[inline] pub fn test_ipxfr_incompisoout(&self) -> bool {
        self.ipxfr_incompisoout() != 0
    }

    #[doc="Sets the IPXFR_INCOMPISOOUT field."]
    #[inline] pub fn set_ipxfr_incompisoout<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Host port interrupt"]
    #[inline] pub fn hprtint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if HPRTINT != 0"]
    #[inline] pub fn test_hprtint(&self) -> bool {
        self.hprtint() != 0
    }

    #[doc="Sets the HPRTINT field."]
    #[inline] pub fn set_hprtint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Host channels interrupt"]
    #[inline] pub fn hcint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if HCINT != 0"]
    #[inline] pub fn test_hcint(&self) -> bool {
        self.hcint() != 0
    }

    #[doc="Sets the HCINT field."]
    #[inline] pub fn set_hcint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Periodic TxFIFO empty"]
    #[inline] pub fn ptxfe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PTXFE != 0"]
    #[inline] pub fn test_ptxfe(&self) -> bool {
        self.ptxfe() != 0
    }

    #[doc="Sets the PTXFE field."]
    #[inline] pub fn set_ptxfe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Connector ID status change"]
    #[inline] pub fn cidschg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if CIDSCHG != 0"]
    #[inline] pub fn test_cidschg(&self) -> bool {
        self.cidschg() != 0
    }

    #[doc="Sets the CIDSCHG field."]
    #[inline] pub fn set_cidschg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Disconnect detected interrupt"]
    #[inline] pub fn discint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if DISCINT != 0"]
    #[inline] pub fn test_discint(&self) -> bool {
        self.discint() != 0
    }

    #[doc="Sets the DISCINT field."]
    #[inline] pub fn set_discint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Session request/new session detected interrupt"]
    #[inline] pub fn srqint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if SRQINT != 0"]
    #[inline] pub fn test_srqint(&self) -> bool {
        self.srqint() != 0
    }

    #[doc="Sets the SRQINT field."]
    #[inline] pub fn set_srqint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Resume/remote wakeup detected interrupt"]
    #[inline] pub fn wkupint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if WKUPINT != 0"]
    #[inline] pub fn test_wkupint(&self) -> bool {
        self.wkupint() != 0
    }

    #[doc="Sets the WKUPINT field."]
    #[inline] pub fn set_wkupint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Reset detected interrupt"]
    #[inline] pub fn rstdet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if RSTDET != 0"]
    #[inline] pub fn test_rstdet(&self) -> bool {
        self.rstdet() != 0
    }

    #[doc="Sets the RSTDET field."]
    #[inline] pub fn set_rstdet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

}

impl From<u32> for Gintsts {
    #[inline]
    fn from(other: u32) -> Self {
         Gintsts(other)
    }
}

impl ::core::fmt::Display for Gintsts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gintsts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmod() != 0 { try!(write!(f, " cmod"))}
        if self.mmis() != 0 { try!(write!(f, " mmis"))}
        if self.otgint() != 0 { try!(write!(f, " otgint"))}
        if self.sof() != 0 { try!(write!(f, " sof"))}
        if self.rxflvl() != 0 { try!(write!(f, " rxflvl"))}
        if self.nptxfe() != 0 { try!(write!(f, " nptxfe"))}
        if self.ginakeff() != 0 { try!(write!(f, " ginakeff"))}
        if self.goutnakeff() != 0 { try!(write!(f, " goutnakeff"))}
        if self.esusp() != 0 { try!(write!(f, " esusp"))}
        if self.usbsusp() != 0 { try!(write!(f, " usbsusp"))}
        if self.usbrst() != 0 { try!(write!(f, " usbrst"))}
        if self.enumdne() != 0 { try!(write!(f, " enumdne"))}
        if self.isoodrp() != 0 { try!(write!(f, " isoodrp"))}
        if self.eopf() != 0 { try!(write!(f, " eopf"))}
        if self.iepint() != 0 { try!(write!(f, " iepint"))}
        if self.oepint() != 0 { try!(write!(f, " oepint"))}
        if self.iisoixfr() != 0 { try!(write!(f, " iisoixfr"))}
        if self.ipxfr_incompisoout() != 0 { try!(write!(f, " ipxfr_incompisoout"))}
        if self.hprtint() != 0 { try!(write!(f, " hprtint"))}
        if self.hcint() != 0 { try!(write!(f, " hcint"))}
        if self.ptxfe() != 0 { try!(write!(f, " ptxfe"))}
        if self.cidschg() != 0 { try!(write!(f, " cidschg"))}
        if self.discint() != 0 { try!(write!(f, " discint"))}
        if self.srqint() != 0 { try!(write!(f, " srqint"))}
        if self.wkupint() != 0 { try!(write!(f, " wkupint"))}
        if self.rstdet() != 0 { try!(write!(f, " rstdet"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gintmsk(pub u32);
impl Gintmsk {
    #[doc="Mode mismatch interrupt mask"]
    #[inline] pub fn mmism(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if MMISM != 0"]
    #[inline] pub fn test_mmism(&self) -> bool {
        self.mmism() != 0
    }

    #[doc="Sets the MMISM field."]
    #[inline] pub fn set_mmism<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="OTG interrupt mask"]
    #[inline] pub fn otgint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if OTGINT != 0"]
    #[inline] pub fn test_otgint(&self) -> bool {
        self.otgint() != 0
    }

    #[doc="Sets the OTGINT field."]
    #[inline] pub fn set_otgint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Start of frame mask"]
    #[inline] pub fn sofm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SOFM != 0"]
    #[inline] pub fn test_sofm(&self) -> bool {
        self.sofm() != 0
    }

    #[doc="Sets the SOFM field."]
    #[inline] pub fn set_sofm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Receive FIFO non-empty mask"]
    #[inline] pub fn rxflvlm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXFLVLM != 0"]
    #[inline] pub fn test_rxflvlm(&self) -> bool {
        self.rxflvlm() != 0
    }

    #[doc="Sets the RXFLVLM field."]
    #[inline] pub fn set_rxflvlm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Non-periodic TxFIFO empty mask"]
    #[inline] pub fn nptxfem(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if NPTXFEM != 0"]
    #[inline] pub fn test_nptxfem(&self) -> bool {
        self.nptxfem() != 0
    }

    #[doc="Sets the NPTXFEM field."]
    #[inline] pub fn set_nptxfem<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Global non-periodic IN NAK effective mask"]
    #[inline] pub fn ginakeffm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if GINAKEFFM != 0"]
    #[inline] pub fn test_ginakeffm(&self) -> bool {
        self.ginakeffm() != 0
    }

    #[doc="Sets the GINAKEFFM field."]
    #[inline] pub fn set_ginakeffm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Global OUT NAK effective mask"]
    #[inline] pub fn gonakeffm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if GONAKEFFM != 0"]
    #[inline] pub fn test_gonakeffm(&self) -> bool {
        self.gonakeffm() != 0
    }

    #[doc="Sets the GONAKEFFM field."]
    #[inline] pub fn set_gonakeffm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Early suspend mask"]
    #[inline] pub fn esuspm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if ESUSPM != 0"]
    #[inline] pub fn test_esuspm(&self) -> bool {
        self.esuspm() != 0
    }

    #[doc="Sets the ESUSPM field."]
    #[inline] pub fn set_esuspm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="USB suspend mask"]
    #[inline] pub fn usbsuspm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if USBSUSPM != 0"]
    #[inline] pub fn test_usbsuspm(&self) -> bool {
        self.usbsuspm() != 0
    }

    #[doc="Sets the USBSUSPM field."]
    #[inline] pub fn set_usbsuspm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="USB reset mask"]
    #[inline] pub fn usbrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if USBRST != 0"]
    #[inline] pub fn test_usbrst(&self) -> bool {
        self.usbrst() != 0
    }

    #[doc="Sets the USBRST field."]
    #[inline] pub fn set_usbrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Enumeration done mask"]
    #[inline] pub fn enumdnem(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if ENUMDNEM != 0"]
    #[inline] pub fn test_enumdnem(&self) -> bool {
        self.enumdnem() != 0
    }

    #[doc="Sets the ENUMDNEM field."]
    #[inline] pub fn set_enumdnem<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Isochronous OUT packet dropped interrupt mask"]
    #[inline] pub fn isoodrpm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if ISOODRPM != 0"]
    #[inline] pub fn test_isoodrpm(&self) -> bool {
        self.isoodrpm() != 0
    }

    #[doc="Sets the ISOODRPM field."]
    #[inline] pub fn set_isoodrpm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="End of periodic frame interrupt mask"]
    #[inline] pub fn eopfm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if EOPFM != 0"]
    #[inline] pub fn test_eopfm(&self) -> bool {
        self.eopfm() != 0
    }

    #[doc="Sets the EOPFM field."]
    #[inline] pub fn set_eopfm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="IN endpoints interrupt mask"]
    #[inline] pub fn iepint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if IEPINT != 0"]
    #[inline] pub fn test_iepint(&self) -> bool {
        self.iepint() != 0
    }

    #[doc="Sets the IEPINT field."]
    #[inline] pub fn set_iepint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="OUT endpoints interrupt mask"]
    #[inline] pub fn oepint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if OEPINT != 0"]
    #[inline] pub fn test_oepint(&self) -> bool {
        self.oepint() != 0
    }

    #[doc="Sets the OEPINT field."]
    #[inline] pub fn set_oepint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Incomplete isochronous IN transfer mask"]
    #[inline] pub fn iisoixfrm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if IISOIXFRM != 0"]
    #[inline] pub fn test_iisoixfrm(&self) -> bool {
        self.iisoixfrm() != 0
    }

    #[doc="Sets the IISOIXFRM field."]
    #[inline] pub fn set_iisoixfrm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)"]
    #[inline] pub fn ipxfrm_iisooxfrm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if IPXFRM_IISOOXFRM != 0"]
    #[inline] pub fn test_ipxfrm_iisooxfrm(&self) -> bool {
        self.ipxfrm_iisooxfrm() != 0
    }

    #[doc="Sets the IPXFRM_IISOOXFRM field."]
    #[inline] pub fn set_ipxfrm_iisooxfrm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Host port interrupt mask"]
    #[inline] pub fn prtim(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PRTIM != 0"]
    #[inline] pub fn test_prtim(&self) -> bool {
        self.prtim() != 0
    }

    #[doc="Sets the PRTIM field."]
    #[inline] pub fn set_prtim<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Host channels interrupt mask"]
    #[inline] pub fn hcim(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if HCIM != 0"]
    #[inline] pub fn test_hcim(&self) -> bool {
        self.hcim() != 0
    }

    #[doc="Sets the HCIM field."]
    #[inline] pub fn set_hcim<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Periodic TxFIFO empty mask"]
    #[inline] pub fn ptxfem(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PTXFEM != 0"]
    #[inline] pub fn test_ptxfem(&self) -> bool {
        self.ptxfem() != 0
    }

    #[doc="Sets the PTXFEM field."]
    #[inline] pub fn set_ptxfem<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Connector ID status change mask"]
    #[inline] pub fn cidschgm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if CIDSCHGM != 0"]
    #[inline] pub fn test_cidschgm(&self) -> bool {
        self.cidschgm() != 0
    }

    #[doc="Sets the CIDSCHGM field."]
    #[inline] pub fn set_cidschgm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Disconnect detected interrupt mask"]
    #[inline] pub fn discint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if DISCINT != 0"]
    #[inline] pub fn test_discint(&self) -> bool {
        self.discint() != 0
    }

    #[doc="Sets the DISCINT field."]
    #[inline] pub fn set_discint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Session request/new session detected interrupt mask"]
    #[inline] pub fn srqim(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if SRQIM != 0"]
    #[inline] pub fn test_srqim(&self) -> bool {
        self.srqim() != 0
    }

    #[doc="Sets the SRQIM field."]
    #[inline] pub fn set_srqim<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Resume/remote wakeup detected interrupt mask"]
    #[inline] pub fn wuim(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if WUIM != 0"]
    #[inline] pub fn test_wuim(&self) -> bool {
        self.wuim() != 0
    }

    #[doc="Sets the WUIM field."]
    #[inline] pub fn set_wuim<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Reset detected interrupt mask"]
    #[inline] pub fn rstdetm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if RSTDETM != 0"]
    #[inline] pub fn test_rstdetm(&self) -> bool {
        self.rstdetm() != 0
    }

    #[doc="Sets the RSTDETM field."]
    #[inline] pub fn set_rstdetm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="LPM interrupt mask"]
    #[inline] pub fn lpmin(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if LPMIN != 0"]
    #[inline] pub fn test_lpmin(&self) -> bool {
        self.lpmin() != 0
    }

    #[doc="Sets the LPMIN field."]
    #[inline] pub fn set_lpmin<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

}

impl From<u32> for Gintmsk {
    #[inline]
    fn from(other: u32) -> Self {
         Gintmsk(other)
    }
}

impl ::core::fmt::Display for Gintmsk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gintmsk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mmism() != 0 { try!(write!(f, " mmism"))}
        if self.otgint() != 0 { try!(write!(f, " otgint"))}
        if self.sofm() != 0 { try!(write!(f, " sofm"))}
        if self.rxflvlm() != 0 { try!(write!(f, " rxflvlm"))}
        if self.nptxfem() != 0 { try!(write!(f, " nptxfem"))}
        if self.ginakeffm() != 0 { try!(write!(f, " ginakeffm"))}
        if self.gonakeffm() != 0 { try!(write!(f, " gonakeffm"))}
        if self.esuspm() != 0 { try!(write!(f, " esuspm"))}
        if self.usbsuspm() != 0 { try!(write!(f, " usbsuspm"))}
        if self.usbrst() != 0 { try!(write!(f, " usbrst"))}
        if self.enumdnem() != 0 { try!(write!(f, " enumdnem"))}
        if self.isoodrpm() != 0 { try!(write!(f, " isoodrpm"))}
        if self.eopfm() != 0 { try!(write!(f, " eopfm"))}
        if self.iepint() != 0 { try!(write!(f, " iepint"))}
        if self.oepint() != 0 { try!(write!(f, " oepint"))}
        if self.iisoixfrm() != 0 { try!(write!(f, " iisoixfrm"))}
        if self.ipxfrm_iisooxfrm() != 0 { try!(write!(f, " ipxfrm_iisooxfrm"))}
        if self.prtim() != 0 { try!(write!(f, " prtim"))}
        if self.hcim() != 0 { try!(write!(f, " hcim"))}
        if self.ptxfem() != 0 { try!(write!(f, " ptxfem"))}
        if self.cidschgm() != 0 { try!(write!(f, " cidschgm"))}
        if self.discint() != 0 { try!(write!(f, " discint"))}
        if self.srqim() != 0 { try!(write!(f, " srqim"))}
        if self.wuim() != 0 { try!(write!(f, " wuim"))}
        if self.rstdetm() != 0 { try!(write!(f, " rstdetm"))}
        if self.lpmin() != 0 { try!(write!(f, " lpmin"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS Receive status debug read(Device mode)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct GrxstsrDevice(pub u32);
impl GrxstsrDevice {
    #[doc="Endpoint number"]
    #[inline] pub fn epnum(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EPNUM != 0"]
    #[inline] pub fn test_epnum(&self) -> bool {
        self.epnum() != 0
    }

    #[doc="Sets the EPNUM field."]
    #[inline] pub fn set_epnum<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Byte count"]
    #[inline] pub fn bcnt(&self) -> ::bobbin_bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7ff) as u16) } // [14:4]
    }

    #[doc="Returns true if BCNT != 0"]
    #[inline] pub fn test_bcnt(&self) -> bool {
        self.bcnt() != 0
    }

    #[doc="Sets the BCNT field."]
    #[inline] pub fn set_bcnt<V: Into<::bobbin_bits::U11>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data PID"]
    #[inline] pub fn dpid(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x3) as u8) } // [16:15]
    }

    #[doc="Returns true if DPID != 0"]
    #[inline] pub fn test_dpid(&self) -> bool {
        self.dpid() != 0
    }

    #[doc="Sets the DPID field."]
    #[inline] pub fn set_dpid<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Packet status"]
    #[inline] pub fn pktsts(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0xf) as u8) } // [20:17]
    }

    #[doc="Returns true if PKTSTS != 0"]
    #[inline] pub fn test_pktsts(&self) -> bool {
        self.pktsts() != 0
    }

    #[doc="Sets the PKTSTS field."]
    #[inline] pub fn set_pktsts<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Frame number"]
    #[inline] pub fn frmnum(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0xf) as u8) } // [24:21]
    }

    #[doc="Returns true if FRMNUM != 0"]
    #[inline] pub fn test_frmnum(&self) -> bool {
        self.frmnum() != 0
    }

    #[doc="Sets the FRMNUM field."]
    #[inline] pub fn set_frmnum<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 21);
        self.0 |= value << 21;
        self
    }

}

impl From<u32> for GrxstsrDevice {
    #[inline]
    fn from(other: u32) -> Self {
         GrxstsrDevice(other)
    }
}

impl ::core::fmt::Display for GrxstsrDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for GrxstsrDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.epnum() != 0 { try!(write!(f, " epnum=0x{:x}", self.epnum()))}
        if self.bcnt() != 0 { try!(write!(f, " bcnt=0x{:x}", self.bcnt()))}
        if self.dpid() != 0 { try!(write!(f, " dpid=0x{:x}", self.dpid()))}
        if self.pktsts() != 0 { try!(write!(f, " pktsts=0x{:x}", self.pktsts()))}
        if self.frmnum() != 0 { try!(write!(f, " frmnum=0x{:x}", self.frmnum()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS Receive status debug read(Host mode)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct GrxstsrHost(pub u32);
impl GrxstsrHost {
    #[doc="Endpoint number"]
    #[inline] pub fn chnum(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CHNUM != 0"]
    #[inline] pub fn test_chnum(&self) -> bool {
        self.chnum() != 0
    }

    #[doc="Sets the CHNUM field."]
    #[inline] pub fn set_chnum<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Byte count"]
    #[inline] pub fn bcnt(&self) -> ::bobbin_bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7ff) as u16) } // [14:4]
    }

    #[doc="Returns true if BCNT != 0"]
    #[inline] pub fn test_bcnt(&self) -> bool {
        self.bcnt() != 0
    }

    #[doc="Sets the BCNT field."]
    #[inline] pub fn set_bcnt<V: Into<::bobbin_bits::U11>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data PID"]
    #[inline] pub fn dpid(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x3) as u8) } // [16:15]
    }

    #[doc="Returns true if DPID != 0"]
    #[inline] pub fn test_dpid(&self) -> bool {
        self.dpid() != 0
    }

    #[doc="Sets the DPID field."]
    #[inline] pub fn set_dpid<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Packet status"]
    #[inline] pub fn pktsts(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0xf) as u8) } // [20:17]
    }

    #[doc="Returns true if PKTSTS != 0"]
    #[inline] pub fn test_pktsts(&self) -> bool {
        self.pktsts() != 0
    }

    #[doc="Sets the PKTSTS field."]
    #[inline] pub fn set_pktsts<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 17);
        self.0 |= value << 17;
        self
    }

}

impl From<u32> for GrxstsrHost {
    #[inline]
    fn from(other: u32) -> Self {
         GrxstsrHost(other)
    }
}

impl ::core::fmt::Display for GrxstsrHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for GrxstsrHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.chnum() != 0 { try!(write!(f, " chnum=0x{:x}", self.chnum()))}
        if self.bcnt() != 0 { try!(write!(f, " bcnt=0x{:x}", self.bcnt()))}
        if self.dpid() != 0 { try!(write!(f, " dpid=0x{:x}", self.dpid()))}
        if self.pktsts() != 0 { try!(write!(f, " pktsts=0x{:x}", self.pktsts()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Grxfsiz(pub u32);
impl Grxfsiz {
    #[doc="RxFIFO depth"]
    #[inline] pub fn rxfd(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if RXFD != 0"]
    #[inline] pub fn test_rxfd(&self) -> bool {
        self.rxfd() != 0
    }

    #[doc="Sets the RXFD field."]
    #[inline] pub fn set_rxfd<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Grxfsiz {
    #[inline]
    fn from(other: u32) -> Self {
         Grxfsiz(other)
    }
}

impl ::core::fmt::Display for Grxfsiz {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Grxfsiz {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxfd() != 0 { try!(write!(f, " rxfd=0x{:x}", self.rxfd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS Endpoint 0 Transmit FIFO size"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dieptxf0(pub u32);
impl Dieptxf0 {
    #[doc="Endpoint 0 transmit RAM start address"]
    #[inline] pub fn tx0fsa(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if TX0FSA != 0"]
    #[inline] pub fn test_tx0fsa(&self) -> bool {
        self.tx0fsa() != 0
    }

    #[doc="Sets the TX0FSA field."]
    #[inline] pub fn set_tx0fsa<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Endpoint 0 TxFIFO depth"]
    #[inline] pub fn tx0fd(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if TX0FD != 0"]
    #[inline] pub fn test_tx0fd(&self) -> bool {
        self.tx0fd() != 0
    }

    #[doc="Sets the TX0FD field."]
    #[inline] pub fn set_tx0fd<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Dieptxf0 {
    #[inline]
    fn from(other: u32) -> Self {
         Dieptxf0(other)
    }
}

impl ::core::fmt::Display for Dieptxf0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dieptxf0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tx0fsa() != 0 { try!(write!(f, " tx0fsa=0x{:x}", self.tx0fsa()))}
        if self.tx0fd() != 0 { try!(write!(f, " tx0fd=0x{:x}", self.tx0fd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS Host non-periodic transmit FIFO size register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct HnptxfsizHost(pub u32);
impl HnptxfsizHost {
    #[doc="Non-periodic transmit RAM start address"]
    #[inline] pub fn nptxfsa(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if NPTXFSA != 0"]
    #[inline] pub fn test_nptxfsa(&self) -> bool {
        self.nptxfsa() != 0
    }

    #[doc="Sets the NPTXFSA field."]
    #[inline] pub fn set_nptxfsa<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Non-periodic TxFIFO depth"]
    #[inline] pub fn nptxfd(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if NPTXFD != 0"]
    #[inline] pub fn test_nptxfd(&self) -> bool {
        self.nptxfd() != 0
    }

    #[doc="Sets the NPTXFD field."]
    #[inline] pub fn set_nptxfd<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for HnptxfsizHost {
    #[inline]
    fn from(other: u32) -> Self {
         HnptxfsizHost(other)
    }
}

impl ::core::fmt::Display for HnptxfsizHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for HnptxfsizHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.nptxfsa() != 0 { try!(write!(f, " nptxfsa=0x{:x}", self.nptxfsa()))}
        if self.nptxfd() != 0 { try!(write!(f, " nptxfd=0x{:x}", self.nptxfd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hnptxsts(pub u32);
impl Hnptxsts {
    #[doc="Non-periodic TxFIFO space available"]
    #[inline] pub fn nptxfsav(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if NPTXFSAV != 0"]
    #[inline] pub fn test_nptxfsav(&self) -> bool {
        self.nptxfsav() != 0
    }

    #[doc="Sets the NPTXFSAV field."]
    #[inline] pub fn set_nptxfsav<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Non-periodic transmit request queue space available"]
    #[inline] pub fn nptqxsav(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if NPTQXSAV != 0"]
    #[inline] pub fn test_nptqxsav(&self) -> bool {
        self.nptqxsav() != 0
    }

    #[doc="Sets the NPTQXSAV field."]
    #[inline] pub fn set_nptqxsav<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Top of the non-periodic transmit request queue"]
    #[inline] pub fn nptxqtop(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7f) as u8) } // [30:24]
    }

    #[doc="Returns true if NPTXQTOP != 0"]
    #[inline] pub fn test_nptxqtop(&self) -> bool {
        self.nptxqtop() != 0
    }

    #[doc="Sets the NPTXQTOP field."]
    #[inline] pub fn set_nptxqtop<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Hnptxsts {
    #[inline]
    fn from(other: u32) -> Self {
         Hnptxsts(other)
    }
}

impl ::core::fmt::Display for Hnptxsts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hnptxsts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.nptxfsav() != 0 { try!(write!(f, " nptxfsav=0x{:x}", self.nptxfsav()))}
        if self.nptqxsav() != 0 { try!(write!(f, " nptqxsav=0x{:x}", self.nptqxsav()))}
        if self.nptxqtop() != 0 { try!(write!(f, " nptxqtop=0x{:x}", self.nptxqtop()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS general core configuration register (OTG_FS_GCCFG)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gccfg(pub u32);
impl Gccfg {
    #[doc="Power down"]
    #[inline] pub fn pwrdwn(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PWRDWN != 0"]
    #[inline] pub fn test_pwrdwn(&self) -> bool {
        self.pwrdwn() != 0
    }

    #[doc="Sets the PWRDWN field."]
    #[inline] pub fn set_pwrdwn<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Battery charging detector (BCD) enable"]
    #[inline] pub fn bcden(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if BCDEN != 0"]
    #[inline] pub fn test_bcden(&self) -> bool {
        self.bcden() != 0
    }

    #[doc="Sets the BCDEN field."]
    #[inline] pub fn set_bcden<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Data contact detection (DCD) mode enable"]
    #[inline] pub fn dcden(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if DCDEN != 0"]
    #[inline] pub fn test_dcden(&self) -> bool {
        self.dcden() != 0
    }

    #[doc="Sets the DCDEN field."]
    #[inline] pub fn set_dcden<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Primary detection (PD) mode enable"]
    #[inline] pub fn pden(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if PDEN != 0"]
    #[inline] pub fn test_pden(&self) -> bool {
        self.pden() != 0
    }

    #[doc="Sets the PDEN field."]
    #[inline] pub fn set_pden<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Secondary detection (SD) mode enable"]
    #[inline] pub fn sden(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if SDEN != 0"]
    #[inline] pub fn test_sden(&self) -> bool {
        self.sden() != 0
    }

    #[doc="Sets the SDEN field."]
    #[inline] pub fn set_sden<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="USB VBUS detection enable"]
    #[inline] pub fn vbden(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if VBDEN != 0"]
    #[inline] pub fn test_vbden(&self) -> bool {
        self.vbden() != 0
    }

    #[doc="Sets the VBDEN field."]
    #[inline] pub fn set_vbden<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Data contact detection (DCD) status"]
    #[inline] pub fn dcdet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DCDET != 0"]
    #[inline] pub fn test_dcdet(&self) -> bool {
        self.dcdet() != 0
    }

    #[doc="Sets the DCDET field."]
    #[inline] pub fn set_dcdet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Primary detection (PD) status"]
    #[inline] pub fn pdet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PDET != 0"]
    #[inline] pub fn test_pdet(&self) -> bool {
        self.pdet() != 0
    }

    #[doc="Sets the PDET field."]
    #[inline] pub fn set_pdet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Secondary detection (SD) status"]
    #[inline] pub fn sdet(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SDET != 0"]
    #[inline] pub fn test_sdet(&self) -> bool {
        self.sdet() != 0
    }

    #[doc="Sets the SDET field."]
    #[inline] pub fn set_sdet<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="DM pull-up detection status"]
    #[inline] pub fn ps2det(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PS2DET != 0"]
    #[inline] pub fn test_ps2det(&self) -> bool {
        self.ps2det() != 0
    }

    #[doc="Sets the PS2DET field."]
    #[inline] pub fn set_ps2det<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Gccfg {
    #[inline]
    fn from(other: u32) -> Self {
         Gccfg(other)
    }
}

impl ::core::fmt::Display for Gccfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gccfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pwrdwn() != 0 { try!(write!(f, " pwrdwn"))}
        if self.bcden() != 0 { try!(write!(f, " bcden"))}
        if self.dcden() != 0 { try!(write!(f, " dcden"))}
        if self.pden() != 0 { try!(write!(f, " pden"))}
        if self.sden() != 0 { try!(write!(f, " sden"))}
        if self.vbden() != 0 { try!(write!(f, " vbden"))}
        if self.dcdet() != 0 { try!(write!(f, " dcdet"))}
        if self.pdet() != 0 { try!(write!(f, " pdet"))}
        if self.sdet() != 0 { try!(write!(f, " sdet"))}
        if self.ps2det() != 0 { try!(write!(f, " ps2det"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="core ID register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cid(pub u32);
impl Cid {
    #[doc="Product ID field"]
    #[inline] pub fn product_id(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if PRODUCT_ID != 0"]
    #[inline] pub fn test_product_id(&self) -> bool {
        self.product_id() != 0
    }

    #[doc="Sets the PRODUCT_ID field."]
    #[inline] pub fn set_product_id<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cid {
    #[inline]
    fn from(other: u32) -> Self {
         Cid(other)
    }
}

impl ::core::fmt::Display for Cid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hptxfsiz(pub u32);
impl Hptxfsiz {
    #[doc="Host periodic TxFIFO start address"]
    #[inline] pub fn ptxsa(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if PTXSA != 0"]
    #[inline] pub fn test_ptxsa(&self) -> bool {
        self.ptxsa() != 0
    }

    #[doc="Sets the PTXSA field."]
    #[inline] pub fn set_ptxsa<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Host periodic TxFIFO depth"]
    #[inline] pub fn ptxfsiz(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if PTXFSIZ != 0"]
    #[inline] pub fn test_ptxfsiz(&self) -> bool {
        self.ptxfsiz() != 0
    }

    #[doc="Sets the PTXFSIZ field."]
    #[inline] pub fn set_ptxfsiz<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Hptxfsiz {
    #[inline]
    fn from(other: u32) -> Self {
         Hptxfsiz(other)
    }
}

impl ::core::fmt::Display for Hptxfsiz {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hptxfsiz {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ptxsa() != 0 { try!(write!(f, " ptxsa=0x{:x}", self.ptxsa()))}
        if self.ptxfsiz() != 0 { try!(write!(f, " ptxfsiz=0x{:x}", self.ptxfsiz()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXFn)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dieptxf(pub u32);
impl Dieptxf {
    #[doc="IN endpoint FIFO2 transmit RAM start address"]
    #[inline] pub fn ineptxsa(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if INEPTXSA != 0"]
    #[inline] pub fn test_ineptxsa(&self) -> bool {
        self.ineptxsa() != 0
    }

    #[doc="Sets the INEPTXSA field."]
    #[inline] pub fn set_ineptxsa<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IN endpoint TxFIFO depth"]
    #[inline] pub fn ineptxfd(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if INEPTXFD != 0"]
    #[inline] pub fn test_ineptxfd(&self) -> bool {
        self.ineptxfd() != 0
    }

    #[doc="Sets the INEPTXFD field."]
    #[inline] pub fn set_ineptxfd<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Dieptxf {
    #[inline]
    fn from(other: u32) -> Self {
         Dieptxf(other)
    }
}

impl ::core::fmt::Display for Dieptxf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dieptxf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ineptxsa() != 0 { try!(write!(f, " ineptxsa=0x{:x}", self.ineptxsa()))}
        if self.ineptxfd() != 0 { try!(write!(f, " ineptxfd=0x{:x}", self.ineptxfd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG status read and pop register (Device mode)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct GrxstspDevice(pub u32);
impl GrxstspDevice {
    #[doc="Endpoint number"]
    #[inline] pub fn epnum(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EPNUM != 0"]
    #[inline] pub fn test_epnum(&self) -> bool {
        self.epnum() != 0
    }

    #[doc="Sets the EPNUM field."]
    #[inline] pub fn set_epnum<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Byte count"]
    #[inline] pub fn bcnt(&self) -> ::bobbin_bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7ff) as u16) } // [14:4]
    }

    #[doc="Returns true if BCNT != 0"]
    #[inline] pub fn test_bcnt(&self) -> bool {
        self.bcnt() != 0
    }

    #[doc="Sets the BCNT field."]
    #[inline] pub fn set_bcnt<V: Into<::bobbin_bits::U11>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data PID"]
    #[inline] pub fn dpid(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x3) as u8) } // [16:15]
    }

    #[doc="Returns true if DPID != 0"]
    #[inline] pub fn test_dpid(&self) -> bool {
        self.dpid() != 0
    }

    #[doc="Sets the DPID field."]
    #[inline] pub fn set_dpid<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Packet status"]
    #[inline] pub fn pktsts(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0xf) as u8) } // [20:17]
    }

    #[doc="Returns true if PKTSTS != 0"]
    #[inline] pub fn test_pktsts(&self) -> bool {
        self.pktsts() != 0
    }

    #[doc="Sets the PKTSTS field."]
    #[inline] pub fn set_pktsts<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Frame number"]
    #[inline] pub fn frmnum(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0xf) as u8) } // [24:21]
    }

    #[doc="Returns true if FRMNUM != 0"]
    #[inline] pub fn test_frmnum(&self) -> bool {
        self.frmnum() != 0
    }

    #[doc="Sets the FRMNUM field."]
    #[inline] pub fn set_frmnum<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 21);
        self.0 |= value << 21;
        self
    }

}

impl From<u32> for GrxstspDevice {
    #[inline]
    fn from(other: u32) -> Self {
         GrxstspDevice(other)
    }
}

impl ::core::fmt::Display for GrxstspDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for GrxstspDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.epnum() != 0 { try!(write!(f, " epnum=0x{:x}", self.epnum()))}
        if self.bcnt() != 0 { try!(write!(f, " bcnt=0x{:x}", self.bcnt()))}
        if self.dpid() != 0 { try!(write!(f, " dpid=0x{:x}", self.dpid()))}
        if self.pktsts() != 0 { try!(write!(f, " pktsts=0x{:x}", self.pktsts()))}
        if self.frmnum() != 0 { try!(write!(f, " frmnum=0x{:x}", self.frmnum()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG status read and pop register (Host mode)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct GrxstspHost(pub u32);
impl GrxstspHost {
    #[doc="Channel number"]
    #[inline] pub fn chnum(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CHNUM != 0"]
    #[inline] pub fn test_chnum(&self) -> bool {
        self.chnum() != 0
    }

    #[doc="Sets the CHNUM field."]
    #[inline] pub fn set_chnum<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Byte count"]
    #[inline] pub fn bcnt(&self) -> ::bobbin_bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7ff) as u16) } // [14:4]
    }

    #[doc="Returns true if BCNT != 0"]
    #[inline] pub fn test_bcnt(&self) -> bool {
        self.bcnt() != 0
    }

    #[doc="Sets the BCNT field."]
    #[inline] pub fn set_bcnt<V: Into<::bobbin_bits::U11>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Data PID"]
    #[inline] pub fn dpid(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x3) as u8) } // [16:15]
    }

    #[doc="Returns true if DPID != 0"]
    #[inline] pub fn test_dpid(&self) -> bool {
        self.dpid() != 0
    }

    #[doc="Sets the DPID field."]
    #[inline] pub fn set_dpid<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Packet status"]
    #[inline] pub fn pktsts(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0xf) as u8) } // [20:17]
    }

    #[doc="Returns true if PKTSTS != 0"]
    #[inline] pub fn test_pktsts(&self) -> bool {
        self.pktsts() != 0
    }

    #[doc="Sets the PKTSTS field."]
    #[inline] pub fn set_pktsts<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 17);
        self.0 |= value << 17;
        self
    }

}

impl From<u32> for GrxstspHost {
    #[inline]
    fn from(other: u32) -> Self {
         GrxstspHost(other)
    }
}

impl ::core::fmt::Display for GrxstspHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for GrxstspHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.chnum() != 0 { try!(write!(f, " chnum=0x{:x}", self.chnum()))}
        if self.bcnt() != 0 { try!(write!(f, " bcnt=0x{:x}", self.bcnt()))}
        if self.dpid() != 0 { try!(write!(f, " dpid=0x{:x}", self.dpid()))}
        if self.pktsts() != 0 { try!(write!(f, " pktsts=0x{:x}", self.pktsts()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG I2C access register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gi2cctl(pub u32);
impl Gi2cctl {
    #[doc="I2C Read/Write Data"]
    #[inline] pub fn rwdata(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if RWDATA != 0"]
    #[inline] pub fn test_rwdata(&self) -> bool {
        self.rwdata() != 0
    }

    #[doc="Sets the RWDATA field."]
    #[inline] pub fn set_rwdata<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="I2C Register Address"]
    #[inline] pub fn regaddr(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if REGADDR != 0"]
    #[inline] pub fn test_regaddr(&self) -> bool {
        self.regaddr() != 0
    }

    #[doc="Sets the REGADDR field."]
    #[inline] pub fn set_regaddr<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="I2C Address"]
    #[inline] pub fn addr(&self) -> ::bobbin_bits::U7 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7f) as u8) } // [22:16]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<::bobbin_bits::U7>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U7 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="I2C Enable"]
    #[inline] pub fn i2cen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if I2CEN != 0"]
    #[inline] pub fn test_i2cen(&self) -> bool {
        self.i2cen() != 0
    }

    #[doc="Sets the I2CEN field."]
    #[inline] pub fn set_i2cen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="I2C ACK"]
    #[inline] pub fn ack(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if ACK != 0"]
    #[inline] pub fn test_ack(&self) -> bool {
        self.ack() != 0
    }

    #[doc="Sets the ACK field."]
    #[inline] pub fn set_ack<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="I2C Device Address"]
    #[inline] pub fn i2cdevadr(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x3) as u8) } // [27:26]
    }

    #[doc="Returns true if I2CDEVADR != 0"]
    #[inline] pub fn test_i2cdevadr(&self) -> bool {
        self.i2cdevadr() != 0
    }

    #[doc="Sets the I2CDEVADR field."]
    #[inline] pub fn set_i2cdevadr<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="I2C DatSe0 USB mode"]
    #[inline] pub fn i2cdatse0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if I2CDATSE0 != 0"]
    #[inline] pub fn test_i2cdatse0(&self) -> bool {
        self.i2cdatse0() != 0
    }

    #[doc="Sets the I2CDATSE0 field."]
    #[inline] pub fn set_i2cdatse0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Read/Write Indicator"]
    #[inline] pub fn rw(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if RW != 0"]
    #[inline] pub fn test_rw(&self) -> bool {
        self.rw() != 0
    }

    #[doc="Sets the RW field."]
    #[inline] pub fn set_rw<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="I2C Busy/Done"]
    #[inline] pub fn bsydne(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if BSYDNE != 0"]
    #[inline] pub fn test_bsydne(&self) -> bool {
        self.bsydne() != 0
    }

    #[doc="Sets the BSYDNE field."]
    #[inline] pub fn set_bsydne<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Gi2cctl {
    #[inline]
    fn from(other: u32) -> Self {
         Gi2cctl(other)
    }
}

impl ::core::fmt::Display for Gi2cctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gi2cctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rwdata() != 0 { try!(write!(f, " rwdata=0x{:x}", self.rwdata()))}
        if self.regaddr() != 0 { try!(write!(f, " regaddr=0x{:x}", self.regaddr()))}
        if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
        if self.i2cen() != 0 { try!(write!(f, " i2cen"))}
        if self.ack() != 0 { try!(write!(f, " ack"))}
        if self.i2cdevadr() != 0 { try!(write!(f, " i2cdevadr=0x{:x}", self.i2cdevadr()))}
        if self.i2cdatse0() != 0 { try!(write!(f, " i2cdatse0"))}
        if self.rw() != 0 { try!(write!(f, " rw"))}
        if self.bsydne() != 0 { try!(write!(f, " bsydne"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG power down register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gpwrdn(pub u32);
impl Gpwrdn {
    #[doc="ADP module enable"]
    #[inline] pub fn adpmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ADPMEN != 0"]
    #[inline] pub fn test_adpmen(&self) -> bool {
        self.adpmen() != 0
    }

    #[doc="Sets the ADPMEN field."]
    #[inline] pub fn set_adpmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="ADP interrupt flag"]
    #[inline] pub fn adpif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if ADPIF != 0"]
    #[inline] pub fn test_adpif(&self) -> bool {
        self.adpif() != 0
    }

    #[doc="Sets the ADPIF field."]
    #[inline] pub fn set_adpif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

}

impl From<u32> for Gpwrdn {
    #[inline]
    fn from(other: u32) -> Self {
         Gpwrdn(other)
    }
}

impl ::core::fmt::Display for Gpwrdn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gpwrdn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adpmen() != 0 { try!(write!(f, " adpmen"))}
        if self.adpif() != 0 { try!(write!(f, " adpif"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG ADP timer, control and status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gadpctl(pub u32);
impl Gadpctl {
    #[doc="Probe discharge"]
    #[inline] pub fn prbdschg(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if PRBDSCHG != 0"]
    #[inline] pub fn test_prbdschg(&self) -> bool {
        self.prbdschg() != 0
    }

    #[doc="Sets the PRBDSCHG field."]
    #[inline] pub fn set_prbdschg<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Probe delta"]
    #[inline] pub fn prbdelta(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if PRBDELTA != 0"]
    #[inline] pub fn test_prbdelta(&self) -> bool {
        self.prbdelta() != 0
    }

    #[doc="Sets the PRBDELTA field."]
    #[inline] pub fn set_prbdelta<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Probe period"]
    #[inline] pub fn prbper(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if PRBPER != 0"]
    #[inline] pub fn test_prbper(&self) -> bool {
        self.prbper() != 0
    }

    #[doc="Sets the PRBPER field."]
    #[inline] pub fn set_prbper<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Ramp time"]
    #[inline] pub fn rtim(&self) -> ::bobbin_bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x7ff) as u16) } // [16:6]
    }

    #[doc="Returns true if RTIM != 0"]
    #[inline] pub fn test_rtim(&self) -> bool {
        self.rtim() != 0
    }

    #[doc="Sets the RTIM field."]
    #[inline] pub fn set_rtim<V: Into<::bobbin_bits::U11>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Enable probe"]
    #[inline] pub fn enaprb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if ENAPRB != 0"]
    #[inline] pub fn test_enaprb(&self) -> bool {
        self.enaprb() != 0
    }

    #[doc="Sets the ENAPRB field."]
    #[inline] pub fn set_enaprb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Enable sense"]
    #[inline] pub fn enasns(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if ENASNS != 0"]
    #[inline] pub fn test_enasns(&self) -> bool {
        self.enasns() != 0
    }

    #[doc="Sets the ENASNS field."]
    #[inline] pub fn set_enasns<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="ADP reset"]
    #[inline] pub fn adprst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if ADPRST != 0"]
    #[inline] pub fn test_adprst(&self) -> bool {
        self.adprst() != 0
    }

    #[doc="Sets the ADPRST field."]
    #[inline] pub fn set_adprst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="ADP enable"]
    #[inline] pub fn adpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if ADPEN != 0"]
    #[inline] pub fn test_adpen(&self) -> bool {
        self.adpen() != 0
    }

    #[doc="Sets the ADPEN field."]
    #[inline] pub fn set_adpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="ADP probe interrupt flag"]
    #[inline] pub fn adpprbif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if ADPPRBIF != 0"]
    #[inline] pub fn test_adpprbif(&self) -> bool {
        self.adpprbif() != 0
    }

    #[doc="Sets the ADPPRBIF field."]
    #[inline] pub fn set_adpprbif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="ADP sense interrupt flag"]
    #[inline] pub fn adpsnsif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if ADPSNSIF != 0"]
    #[inline] pub fn test_adpsnsif(&self) -> bool {
        self.adpsnsif() != 0
    }

    #[doc="Sets the ADPSNSIF field."]
    #[inline] pub fn set_adpsnsif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="ADP timeout interrupt flag"]
    #[inline] pub fn adptoif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if ADPTOIF != 0"]
    #[inline] pub fn test_adptoif(&self) -> bool {
        self.adptoif() != 0
    }

    #[doc="Sets the ADPTOIF field."]
    #[inline] pub fn set_adptoif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="ADP probe interrupt mask"]
    #[inline] pub fn adpprbim(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if ADPPRBIM != 0"]
    #[inline] pub fn test_adpprbim(&self) -> bool {
        self.adpprbim() != 0
    }

    #[doc="Sets the ADPPRBIM field."]
    #[inline] pub fn set_adpprbim<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="ADP sense interrupt mask"]
    #[inline] pub fn adpsnsim(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if ADPSNSIM != 0"]
    #[inline] pub fn test_adpsnsim(&self) -> bool {
        self.adpsnsim() != 0
    }

    #[doc="Sets the ADPSNSIM field."]
    #[inline] pub fn set_adpsnsim<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="ADP timeout interrupt mask"]
    #[inline] pub fn adptoim(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if ADPTOIM != 0"]
    #[inline] pub fn test_adptoim(&self) -> bool {
        self.adptoim() != 0
    }

    #[doc="Sets the ADPTOIM field."]
    #[inline] pub fn set_adptoim<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Access request"]
    #[inline] pub fn ar(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x3) as u8) } // [28:27]
    }

    #[doc="Returns true if AR != 0"]
    #[inline] pub fn test_ar(&self) -> bool {
        self.ar() != 0
    }

    #[doc="Sets the AR field."]
    #[inline] pub fn set_ar<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 27);
        self.0 |= value << 27;
        self
    }

}

impl From<u32> for Gadpctl {
    #[inline]
    fn from(other: u32) -> Self {
         Gadpctl(other)
    }
}

impl ::core::fmt::Display for Gadpctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gadpctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.prbdschg() != 0 { try!(write!(f, " prbdschg=0x{:x}", self.prbdschg()))}
        if self.prbdelta() != 0 { try!(write!(f, " prbdelta=0x{:x}", self.prbdelta()))}
        if self.prbper() != 0 { try!(write!(f, " prbper=0x{:x}", self.prbper()))}
        if self.rtim() != 0 { try!(write!(f, " rtim=0x{:x}", self.rtim()))}
        if self.enaprb() != 0 { try!(write!(f, " enaprb"))}
        if self.enasns() != 0 { try!(write!(f, " enasns"))}
        if self.adprst() != 0 { try!(write!(f, " adprst"))}
        if self.adpen() != 0 { try!(write!(f, " adpen"))}
        if self.adpprbif() != 0 { try!(write!(f, " adpprbif"))}
        if self.adpsnsif() != 0 { try!(write!(f, " adpsnsif"))}
        if self.adptoif() != 0 { try!(write!(f, " adptoif"))}
        if self.adpprbim() != 0 { try!(write!(f, " adpprbim"))}
        if self.adpsnsim() != 0 { try!(write!(f, " adpsnsim"))}
        if self.adptoim() != 0 { try!(write!(f, " adptoim"))}
        if self.ar() != 0 { try!(write!(f, " ar=0x{:x}", self.ar()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="OTG core LPM configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Glpmcfg(pub u32);
impl Glpmcfg {
    #[doc="LPM support enable"]
    #[inline] pub fn lpmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LPMEN != 0"]
    #[inline] pub fn test_lpmen(&self) -> bool {
        self.lpmen() != 0
    }

    #[doc="Sets the LPMEN field."]
    #[inline] pub fn set_lpmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="LPM token acknowledge enable"]
    #[inline] pub fn lpmack(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LPMACK != 0"]
    #[inline] pub fn test_lpmack(&self) -> bool {
        self.lpmack() != 0
    }

    #[doc="Sets the LPMACK field."]
    #[inline] pub fn set_lpmack<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Best effort service latency"]
    #[inline] pub fn besl(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0xf) as u8) } // [5:2]
    }

    #[doc="Returns true if BESL != 0"]
    #[inline] pub fn test_besl(&self) -> bool {
        self.besl() != 0
    }

    #[doc="Sets the BESL field."]
    #[inline] pub fn set_besl<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="bRemoteWake value"]
    #[inline] pub fn remwake(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if REMWAKE != 0"]
    #[inline] pub fn test_remwake(&self) -> bool {
        self.remwake() != 0
    }

    #[doc="Sets the REMWAKE field."]
    #[inline] pub fn set_remwake<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="L1 Shallow Sleep enable"]
    #[inline] pub fn l1ssen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if L1SSEN != 0"]
    #[inline] pub fn test_l1ssen(&self) -> bool {
        self.l1ssen() != 0
    }

    #[doc="Sets the L1SSEN field."]
    #[inline] pub fn set_l1ssen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="BESL threshold"]
    #[inline] pub fn beslthrs(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if BESLTHRS != 0"]
    #[inline] pub fn test_beslthrs(&self) -> bool {
        self.beslthrs() != 0
    }

    #[doc="Sets the BESLTHRS field."]
    #[inline] pub fn set_beslthrs<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="L1 deep sleep enable"]
    #[inline] pub fn l1dsen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if L1DSEN != 0"]
    #[inline] pub fn test_l1dsen(&self) -> bool {
        self.l1dsen() != 0
    }

    #[doc="Sets the L1DSEN field."]
    #[inline] pub fn set_l1dsen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="LPM response"]
    #[inline] pub fn lpmrst(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x3) as u8) } // [14:13]
    }

    #[doc="Returns true if LPMRST != 0"]
    #[inline] pub fn test_lpmrst(&self) -> bool {
        self.lpmrst() != 0
    }

    #[doc="Sets the LPMRST field."]
    #[inline] pub fn set_lpmrst<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port sleep status"]
    #[inline] pub fn slpsts(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if SLPSTS != 0"]
    #[inline] pub fn test_slpsts(&self) -> bool {
        self.slpsts() != 0
    }

    #[doc="Sets the SLPSTS field."]
    #[inline] pub fn set_slpsts<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Sleep State Resume OK"]
    #[inline] pub fn l1rsmok(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if L1RSMOK != 0"]
    #[inline] pub fn test_l1rsmok(&self) -> bool {
        self.l1rsmok() != 0
    }

    #[doc="Sets the L1RSMOK field."]
    #[inline] pub fn set_l1rsmok<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="LPM Channel Index"]
    #[inline] pub fn lpmchidx(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0xf) as u8) } // [20:17]
    }

    #[doc="Returns true if LPMCHIDX != 0"]
    #[inline] pub fn test_lpmchidx(&self) -> bool {
        self.lpmchidx() != 0
    }

    #[doc="Sets the LPMCHIDX field."]
    #[inline] pub fn set_lpmchidx<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="LPM retry count"]
    #[inline] pub fn lpmrcnt(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x7) as u8) } // [23:21]
    }

    #[doc="Returns true if LPMRCNT != 0"]
    #[inline] pub fn test_lpmrcnt(&self) -> bool {
        self.lpmrcnt() != 0
    }

    #[doc="Sets the LPMRCNT field."]
    #[inline] pub fn set_lpmrcnt<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Send LPM transaction"]
    #[inline] pub fn sndlpm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if SNDLPM != 0"]
    #[inline] pub fn test_sndlpm(&self) -> bool {
        self.sndlpm() != 0
    }

    #[doc="Sets the SNDLPM field."]
    #[inline] pub fn set_sndlpm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="LPM retry count status"]
    #[inline] pub fn lpmrcntsts(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x7) as u8) } // [27:25]
    }

    #[doc="Returns true if LPMRCNTSTS != 0"]
    #[inline] pub fn test_lpmrcntsts(&self) -> bool {
        self.lpmrcntsts() != 0
    }

    #[doc="Sets the LPMRCNTSTS field."]
    #[inline] pub fn set_lpmrcntsts<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Enable best effort service latency"]
    #[inline] pub fn enbesl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if ENBESL != 0"]
    #[inline] pub fn test_enbesl(&self) -> bool {
        self.enbesl() != 0
    }

    #[doc="Sets the ENBESL field."]
    #[inline] pub fn set_enbesl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Glpmcfg {
    #[inline]
    fn from(other: u32) -> Self {
         Glpmcfg(other)
    }
}

impl ::core::fmt::Display for Glpmcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Glpmcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lpmen() != 0 { try!(write!(f, " lpmen"))}
        if self.lpmack() != 0 { try!(write!(f, " lpmack"))}
        if self.besl() != 0 { try!(write!(f, " besl=0x{:x}", self.besl()))}
        if self.remwake() != 0 { try!(write!(f, " remwake"))}
        if self.l1ssen() != 0 { try!(write!(f, " l1ssen"))}
        if self.beslthrs() != 0 { try!(write!(f, " beslthrs=0x{:x}", self.beslthrs()))}
        if self.l1dsen() != 0 { try!(write!(f, " l1dsen"))}
        if self.lpmrst() != 0 { try!(write!(f, " lpmrst=0x{:x}", self.lpmrst()))}
        if self.slpsts() != 0 { try!(write!(f, " slpsts"))}
        if self.l1rsmok() != 0 { try!(write!(f, " l1rsmok"))}
        if self.lpmchidx() != 0 { try!(write!(f, " lpmchidx=0x{:x}", self.lpmchidx()))}
        if self.lpmrcnt() != 0 { try!(write!(f, " lpmrcnt=0x{:x}", self.lpmrcnt()))}
        if self.sndlpm() != 0 { try!(write!(f, " sndlpm"))}
        if self.lpmrcntsts() != 0 { try!(write!(f, " lpmrcntsts=0x{:x}", self.lpmrcntsts()))}
        if self.enbesl() != 0 { try!(write!(f, " enbesl"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

