#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    en: En,
    ctrl: Ctrl,
    if_: If,
    _reserved4: [u8; 0x08],
    ien: Ien,
    _reserved5: [u8; 0x08],
    cal: Cal,
    status: Status,
}
impl RegisterBlock {
    #[doc = "0x00 - No Description"]
    #[inline(always)]
    pub const fn ipversion(&self) -> &Ipversion {
        &self.ipversion
    }
    #[doc = "0x04 - No Description"]
    #[inline(always)]
    pub const fn en(&self) -> &En {
        &self.en
    }
    #[doc = "0x08 - No Description"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x0c - No Description"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x18 - No Description"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x24 - No Description"]
    #[inline(always)]
    pub const fn cal(&self) -> &Cal {
        &self.cal
    }
    #[doc = "0x28 - No Description"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
}
#[doc = "IPVERSION (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ipversion::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipversion`] module"]
#[doc(alias = "IPVERSION")]
pub type Ipversion = crate::Reg<ipversion::IpversionSpec>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "EN (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en`] module"]
#[doc(alias = "EN")]
pub type En = crate::Reg<en::EnSpec>;
#[doc = "No Description"]
pub mod en;
#[doc = "CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "No Description"]
pub mod ctrl;
#[doc = "IF (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`] module"]
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IfSpec>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`] module"]
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IenSpec>;
#[doc = "No Description"]
pub mod ien;
#[doc = "CAL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal`] module"]
#[doc(alias = "CAL")]
pub type Cal = crate::Reg<cal::CalSpec>;
#[doc = "No Description"]
pub mod cal;
#[doc = "STATUS (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "No Description"]
pub mod status;
