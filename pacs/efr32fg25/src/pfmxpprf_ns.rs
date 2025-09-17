#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rfimdcdcctrl0: Rfimdcdcctrl0,
    rfimdcdcctrl1: Rfimdcdcctrl1,
    rfimdcdcctrl2: Rfimdcdcctrl2,
    rfimdcdcstatus: Rfimdcdcstatus,
    rpuratd0: Rpuratd0,
}
impl RegisterBlock {
    #[doc = "0x00 - No Description"]
    #[inline(always)]
    pub const fn rfimdcdcctrl0(&self) -> &Rfimdcdcctrl0 {
        &self.rfimdcdcctrl0
    }
    #[doc = "0x04 - No Description"]
    #[inline(always)]
    pub const fn rfimdcdcctrl1(&self) -> &Rfimdcdcctrl1 {
        &self.rfimdcdcctrl1
    }
    #[doc = "0x08 - No Description"]
    #[inline(always)]
    pub const fn rfimdcdcctrl2(&self) -> &Rfimdcdcctrl2 {
        &self.rfimdcdcctrl2
    }
    #[doc = "0x0c - No Description"]
    #[inline(always)]
    pub const fn rfimdcdcstatus(&self) -> &Rfimdcdcstatus {
        &self.rfimdcdcstatus
    }
    #[doc = "0x10 - Protected register address = (RPURATD register index X 32 + RPURATD bit index) X 4."]
    #[inline(always)]
    pub const fn rpuratd0(&self) -> &Rpuratd0 {
        &self.rpuratd0
    }
}
#[doc = "RFIMDCDCCTRL0 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rfimdcdcctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfimdcdcctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfimdcdcctrl0`] module"]
#[doc(alias = "RFIMDCDCCTRL0")]
pub type Rfimdcdcctrl0 = crate::Reg<rfimdcdcctrl0::Rfimdcdcctrl0Spec>;
#[doc = "No Description"]
pub mod rfimdcdcctrl0;
#[doc = "RFIMDCDCCTRL1 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rfimdcdcctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfimdcdcctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfimdcdcctrl1`] module"]
#[doc(alias = "RFIMDCDCCTRL1")]
pub type Rfimdcdcctrl1 = crate::Reg<rfimdcdcctrl1::Rfimdcdcctrl1Spec>;
#[doc = "No Description"]
pub mod rfimdcdcctrl1;
#[doc = "RFIMDCDCCTRL2 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rfimdcdcctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfimdcdcctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfimdcdcctrl2`] module"]
#[doc(alias = "RFIMDCDCCTRL2")]
pub type Rfimdcdcctrl2 = crate::Reg<rfimdcdcctrl2::Rfimdcdcctrl2Spec>;
#[doc = "No Description"]
pub mod rfimdcdcctrl2;
#[doc = "RFIMDCDCSTATUS (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rfimdcdcstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfimdcdcstatus`] module"]
#[doc(alias = "RFIMDCDCSTATUS")]
pub type Rfimdcdcstatus = crate::Reg<rfimdcdcstatus::RfimdcdcstatusSpec>;
#[doc = "No Description"]
pub mod rfimdcdcstatus;
#[doc = "RPURATD0 (rw) register accessor: Protected register address = (RPURATD register index X 32 + RPURATD bit index) X 4.\n\nYou can [`read`](crate::Reg::read) this register and get [`rpuratd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpuratd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpuratd0`] module"]
#[doc(alias = "RPURATD0")]
pub type Rpuratd0 = crate::Reg<rpuratd0::Rpuratd0Spec>;
#[doc = "Protected register address = (RPURATD register index X 32 + RPURATD bit index) X 4."]
pub mod rpuratd0;
