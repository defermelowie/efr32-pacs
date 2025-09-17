#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1c],
    cfgnstcalib: Cfgnstcalib,
    _reserved1: [u8; 0x05e0],
    rootnsdata0: Rootnsdata0,
    rootnsdata1: Rootnsdata1,
    cfgnsrpuratd0: Cfgnsrpuratd0,
    _reserved4: [u8; 0x2c],
    cfgnsrpuratd12: Cfgnsrpuratd12,
}
impl RegisterBlock {
    #[doc = "0x1c - Configure to define the system tick for the M33."]
    #[inline(always)]
    pub const fn cfgnstcalib(&self) -> &Cfgnstcalib {
        &self.cfgnstcalib
    }
    #[doc = "0x600 - Generic data space for user to pass to root, e.g., address of struct in mem"]
    #[inline(always)]
    pub const fn rootnsdata0(&self) -> &Rootnsdata0 {
        &self.rootnsdata0
    }
    #[doc = "0x604 - Generic data space for user to pass to root, e.g., address of struct in mem"]
    #[inline(always)]
    pub const fn rootnsdata1(&self) -> &Rootnsdata1 {
        &self.rootnsdata1
    }
    #[doc = "0x608 - Protected register address = (RPURATD register index X 32 + RPURATD bit index) X 4."]
    #[inline(always)]
    pub const fn cfgnsrpuratd0(&self) -> &Cfgnsrpuratd0 {
        &self.cfgnsrpuratd0
    }
    #[doc = "0x638 - Protected register address = (RPURATD register index X 32 + RPURATD bit index) X 4."]
    #[inline(always)]
    pub const fn cfgnsrpuratd12(&self) -> &Cfgnsrpuratd12 {
        &self.cfgnsrpuratd12
    }
}
#[doc = "CFGNSTCALIB (rw) register accessor: Configure to define the system tick for the M33.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgnstcalib::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgnstcalib::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgnstcalib`] module"]
#[doc(alias = "CFGNSTCALIB")]
pub type Cfgnstcalib = crate::Reg<cfgnstcalib::CfgnstcalibSpec>;
#[doc = "Configure to define the system tick for the M33."]
pub mod cfgnstcalib;
#[doc = "ROOTNSDATA0 (rw) register accessor: Generic data space for user to pass to root, e.g., address of struct in mem\n\nYou can [`read`](crate::Reg::read) this register and get [`rootnsdata0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rootnsdata0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rootnsdata0`] module"]
#[doc(alias = "ROOTNSDATA0")]
pub type Rootnsdata0 = crate::Reg<rootnsdata0::Rootnsdata0Spec>;
#[doc = "Generic data space for user to pass to root, e.g., address of struct in mem"]
pub mod rootnsdata0;
#[doc = "ROOTNSDATA1 (rw) register accessor: Generic data space for user to pass to root, e.g., address of struct in mem\n\nYou can [`read`](crate::Reg::read) this register and get [`rootnsdata1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rootnsdata1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rootnsdata1`] module"]
#[doc(alias = "ROOTNSDATA1")]
pub type Rootnsdata1 = crate::Reg<rootnsdata1::Rootnsdata1Spec>;
#[doc = "Generic data space for user to pass to root, e.g., address of struct in mem"]
pub mod rootnsdata1;
#[doc = "CFGNSRPURATD0 (rw) register accessor: Protected register address = (RPURATD register index X 32 + RPURATD bit index) X 4.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgnsrpuratd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgnsrpuratd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgnsrpuratd0`] module"]
#[doc(alias = "CFGNSRPURATD0")]
pub type Cfgnsrpuratd0 = crate::Reg<cfgnsrpuratd0::Cfgnsrpuratd0Spec>;
#[doc = "Protected register address = (RPURATD register index X 32 + RPURATD bit index) X 4."]
pub mod cfgnsrpuratd0;
#[doc = "CFGNSRPURATD12 (rw) register accessor: Protected register address = (RPURATD register index X 32 + RPURATD bit index) X 4.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgnsrpuratd12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgnsrpuratd12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgnsrpuratd12`] module"]
#[doc(alias = "CFGNSRPURATD12")]
pub type Cfgnsrpuratd12 = crate::Reg<cfgnsrpuratd12::Cfgnsrpuratd12Spec>;
#[doc = "Protected register address = (RPURATD register index X 32 + RPURATD bit index) X 4."]
pub mod cfgnsrpuratd12;
