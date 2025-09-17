#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    en: En,
    _reserved2: [u8; 0x04],
    cfg: Cfg,
    cntmismatchmax: Cntmismatchmax,
    chnlfiltwinsize: Chnlfiltwinsize,
    cmd: Cmd,
    syncbusy: Syncbusy,
    ien: Ien,
    if_: If,
    status: Status,
    em4wuen: Em4wuen,
    chnlseedval0: Chnlseedval0,
    chnlseedval1: Chnlseedval1,
    clkprescval: Clkprescval,
    _reserved14: [u8; 0x0c],
    lock: Lock,
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
    #[doc = "0x0c - No Description"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x10 - No Description"]
    #[inline(always)]
    pub const fn cntmismatchmax(&self) -> &Cntmismatchmax {
        &self.cntmismatchmax
    }
    #[doc = "0x14 - No Description"]
    #[inline(always)]
    pub const fn chnlfiltwinsize(&self) -> &Chnlfiltwinsize {
        &self.chnlfiltwinsize
    }
    #[doc = "0x18 - No Description"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x1c - No Description"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x20 - No Description"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x24 - No Description"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x28 - No Description"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x2c - No Description"]
    #[inline(always)]
    pub const fn em4wuen(&self) -> &Em4wuen {
        &self.em4wuen
    }
    #[doc = "0x30 - No Description"]
    #[inline(always)]
    pub const fn chnlseedval0(&self) -> &Chnlseedval0 {
        &self.chnlseedval0
    }
    #[doc = "0x34 - No Description"]
    #[inline(always)]
    pub const fn chnlseedval1(&self) -> &Chnlseedval1 {
        &self.chnlseedval1
    }
    #[doc = "0x38 - Finial dividing factor = RIPPLECNTUPPER * CNTLOWER"]
    #[inline(always)]
    pub const fn clkprescval(&self) -> &Clkprescval {
        &self.clkprescval
    }
    #[doc = "0x48 - No Description"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
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
#[doc = "CFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "No Description"]
pub mod cfg;
#[doc = "CNTMISMATCHMAX (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cntmismatchmax::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntmismatchmax::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntmismatchmax`] module"]
#[doc(alias = "CNTMISMATCHMAX")]
pub type Cntmismatchmax = crate::Reg<cntmismatchmax::CntmismatchmaxSpec>;
#[doc = "No Description"]
pub mod cntmismatchmax;
#[doc = "CHNLFILTWINSIZE (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`chnlfiltwinsize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnlfiltwinsize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnlfiltwinsize`] module"]
#[doc(alias = "CHNLFILTWINSIZE")]
pub type Chnlfiltwinsize = crate::Reg<chnlfiltwinsize::ChnlfiltwinsizeSpec>;
#[doc = "No Description"]
pub mod chnlfiltwinsize;
#[doc = "CMD (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "SYNCBUSY (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "No Description"]
pub mod syncbusy;
#[doc = "IEN (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`] module"]
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IenSpec>;
#[doc = "No Description"]
pub mod ien;
#[doc = "IF (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`] module"]
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IfSpec>;
#[doc = "No Description"]
pub mod if_;
#[doc = "STATUS (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "No Description"]
pub mod status;
#[doc = "EM4WUEN (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`em4wuen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em4wuen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em4wuen`] module"]
#[doc(alias = "EM4WUEN")]
pub type Em4wuen = crate::Reg<em4wuen::Em4wuenSpec>;
#[doc = "No Description"]
pub mod em4wuen;
#[doc = "CHNLSEEDVAL0 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`chnlseedval0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnlseedval0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnlseedval0`] module"]
#[doc(alias = "CHNLSEEDVAL0")]
pub type Chnlseedval0 = crate::Reg<chnlseedval0::Chnlseedval0Spec>;
#[doc = "No Description"]
pub mod chnlseedval0;
#[doc = "CHNLSEEDVAL1 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`chnlseedval1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnlseedval1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnlseedval1`] module"]
#[doc(alias = "CHNLSEEDVAL1")]
pub type Chnlseedval1 = crate::Reg<chnlseedval1::Chnlseedval1Spec>;
#[doc = "No Description"]
pub mod chnlseedval1;
#[doc = "CLKPRESCVAL (rw) register accessor: Finial dividing factor = RIPPLECNTUPPER * CNTLOWER\n\nYou can [`read`](crate::Reg::read) this register and get [`clkprescval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkprescval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkprescval`] module"]
#[doc(alias = "CLKPRESCVAL")]
pub type Clkprescval = crate::Reg<clkprescval::ClkprescvalSpec>;
#[doc = "Finial dividing factor = RIPPLECNTUPPER * CNTLOWER"]
pub mod clkprescval;
#[doc = "LOCK (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "No Description"]
pub mod lock;
