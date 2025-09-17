#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    en: En,
    lpmode: Lpmode,
    buf0_ctrl: Buf0Ctrl,
    buf0_addr: Buf0Addr,
    buf0_writeoffset: Buf0Writeoffset,
    buf0_readoffset: Buf0Readoffset,
    _reserved7: [u8; 0x04],
    buf0_readdata: Buf0Readdata,
    buf0_writedata: Buf0Writedata,
    buf0_xwrite: Buf0Xwrite,
    buf0_status: Buf0Status,
    buf0_thresholdctrl: Buf0Thresholdctrl,
    buf0_cmd: Buf0Cmd,
    _reserved13: [u8; 0x04],
    buf0_readdata32: Buf0Readdata32,
    buf0_writedata32: Buf0Writedata32,
    buf0_xwrite32: Buf0Xwrite32,
    _reserved16: [u8; 0x04],
    buf1_ctrl: Buf1Ctrl,
    buf1_addr: Buf1Addr,
    buf1_writeoffset: Buf1Writeoffset,
    buf1_readoffset: Buf1Readoffset,
    _reserved20: [u8; 0x04],
    buf1_readdata: Buf1Readdata,
    buf1_writedata: Buf1Writedata,
    buf1_xwrite: Buf1Xwrite,
    buf1_status: Buf1Status,
    buf1_thresholdctrl: Buf1Thresholdctrl,
    buf1_cmd: Buf1Cmd,
    _reserved26: [u8; 0x04],
    buf1_readdata32: Buf1Readdata32,
    buf1_writedata32: Buf1Writedata32,
    buf1_xwrite32: Buf1Xwrite32,
    _reserved29: [u8; 0x04],
    buf2_ctrl: Buf2Ctrl,
    buf2_addr: Buf2Addr,
    buf2_writeoffset: Buf2Writeoffset,
    buf2_readoffset: Buf2Readoffset,
    _reserved33: [u8; 0x04],
    buf2_readdata: Buf2Readdata,
    buf2_writedata: Buf2Writedata,
    buf2_xwrite: Buf2Xwrite,
    buf2_status: Buf2Status,
    buf2_thresholdctrl: Buf2Thresholdctrl,
    buf2_cmd: Buf2Cmd,
    _reserved39: [u8; 0x04],
    buf2_readdata32: Buf2Readdata32,
    buf2_writedata32: Buf2Writedata32,
    buf2_xwrite32: Buf2Xwrite32,
    _reserved42: [u8; 0x04],
    buf3_ctrl: Buf3Ctrl,
    buf3_addr: Buf3Addr,
    buf3_writeoffset: Buf3Writeoffset,
    buf3_readoffset: Buf3Readoffset,
    _reserved46: [u8; 0x04],
    buf3_readdata: Buf3Readdata,
    buf3_writedata: Buf3Writedata,
    buf3_xwrite: Buf3Xwrite,
    buf3_status: Buf3Status,
    buf3_thresholdctrl: Buf3Thresholdctrl,
    buf3_cmd: Buf3Cmd,
    _reserved52: [u8; 0x04],
    buf3_readdata32: Buf3Readdata32,
    buf3_writedata32: Buf3Writedata32,
    buf3_xwrite32: Buf3Xwrite32,
    _reserved55: [u8; 0x0c],
    if_: If,
    ien: Ien,
    seqif: Seqif,
    seqien: Seqien,
    sfmif: Sfmif,
    sfmien: Sfmien,
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
    pub const fn lpmode(&self) -> &Lpmode {
        &self.lpmode
    }
    #[doc = "0x0c - No Description"]
    #[inline(always)]
    pub const fn buf0_ctrl(&self) -> &Buf0Ctrl {
        &self.buf0_ctrl
    }
    #[doc = "0x10 - No Description"]
    #[inline(always)]
    pub const fn buf0_addr(&self) -> &Buf0Addr {
        &self.buf0_addr
    }
    #[doc = "0x14 - No Description"]
    #[inline(always)]
    pub const fn buf0_writeoffset(&self) -> &Buf0Writeoffset {
        &self.buf0_writeoffset
    }
    #[doc = "0x18 - No Description"]
    #[inline(always)]
    pub const fn buf0_readoffset(&self) -> &Buf0Readoffset {
        &self.buf0_readoffset
    }
    #[doc = "0x20 - No Description"]
    #[inline(always)]
    pub const fn buf0_readdata(&self) -> &Buf0Readdata {
        &self.buf0_readdata
    }
    #[doc = "0x24 - No Description"]
    #[inline(always)]
    pub const fn buf0_writedata(&self) -> &Buf0Writedata {
        &self.buf0_writedata
    }
    #[doc = "0x28 - No Description"]
    #[inline(always)]
    pub const fn buf0_xwrite(&self) -> &Buf0Xwrite {
        &self.buf0_xwrite
    }
    #[doc = "0x2c - No Description"]
    #[inline(always)]
    pub const fn buf0_status(&self) -> &Buf0Status {
        &self.buf0_status
    }
    #[doc = "0x30 - No Description"]
    #[inline(always)]
    pub const fn buf0_thresholdctrl(&self) -> &Buf0Thresholdctrl {
        &self.buf0_thresholdctrl
    }
    #[doc = "0x34 - No Description"]
    #[inline(always)]
    pub const fn buf0_cmd(&self) -> &Buf0Cmd {
        &self.buf0_cmd
    }
    #[doc = "0x3c - No Description"]
    #[inline(always)]
    pub const fn buf0_readdata32(&self) -> &Buf0Readdata32 {
        &self.buf0_readdata32
    }
    #[doc = "0x40 - No Description"]
    #[inline(always)]
    pub const fn buf0_writedata32(&self) -> &Buf0Writedata32 {
        &self.buf0_writedata32
    }
    #[doc = "0x44 - No Description"]
    #[inline(always)]
    pub const fn buf0_xwrite32(&self) -> &Buf0Xwrite32 {
        &self.buf0_xwrite32
    }
    #[doc = "0x4c - No Description"]
    #[inline(always)]
    pub const fn buf1_ctrl(&self) -> &Buf1Ctrl {
        &self.buf1_ctrl
    }
    #[doc = "0x50 - No Description"]
    #[inline(always)]
    pub const fn buf1_addr(&self) -> &Buf1Addr {
        &self.buf1_addr
    }
    #[doc = "0x54 - No Description"]
    #[inline(always)]
    pub const fn buf1_writeoffset(&self) -> &Buf1Writeoffset {
        &self.buf1_writeoffset
    }
    #[doc = "0x58 - No Description"]
    #[inline(always)]
    pub const fn buf1_readoffset(&self) -> &Buf1Readoffset {
        &self.buf1_readoffset
    }
    #[doc = "0x60 - No Description"]
    #[inline(always)]
    pub const fn buf1_readdata(&self) -> &Buf1Readdata {
        &self.buf1_readdata
    }
    #[doc = "0x64 - No Description"]
    #[inline(always)]
    pub const fn buf1_writedata(&self) -> &Buf1Writedata {
        &self.buf1_writedata
    }
    #[doc = "0x68 - No Description"]
    #[inline(always)]
    pub const fn buf1_xwrite(&self) -> &Buf1Xwrite {
        &self.buf1_xwrite
    }
    #[doc = "0x6c - No Description"]
    #[inline(always)]
    pub const fn buf1_status(&self) -> &Buf1Status {
        &self.buf1_status
    }
    #[doc = "0x70 - No Description"]
    #[inline(always)]
    pub const fn buf1_thresholdctrl(&self) -> &Buf1Thresholdctrl {
        &self.buf1_thresholdctrl
    }
    #[doc = "0x74 - No Description"]
    #[inline(always)]
    pub const fn buf1_cmd(&self) -> &Buf1Cmd {
        &self.buf1_cmd
    }
    #[doc = "0x7c - No Description"]
    #[inline(always)]
    pub const fn buf1_readdata32(&self) -> &Buf1Readdata32 {
        &self.buf1_readdata32
    }
    #[doc = "0x80 - No Description"]
    #[inline(always)]
    pub const fn buf1_writedata32(&self) -> &Buf1Writedata32 {
        &self.buf1_writedata32
    }
    #[doc = "0x84 - No Description"]
    #[inline(always)]
    pub const fn buf1_xwrite32(&self) -> &Buf1Xwrite32 {
        &self.buf1_xwrite32
    }
    #[doc = "0x8c - No Description"]
    #[inline(always)]
    pub const fn buf2_ctrl(&self) -> &Buf2Ctrl {
        &self.buf2_ctrl
    }
    #[doc = "0x90 - No Description"]
    #[inline(always)]
    pub const fn buf2_addr(&self) -> &Buf2Addr {
        &self.buf2_addr
    }
    #[doc = "0x94 - No Description"]
    #[inline(always)]
    pub const fn buf2_writeoffset(&self) -> &Buf2Writeoffset {
        &self.buf2_writeoffset
    }
    #[doc = "0x98 - No Description"]
    #[inline(always)]
    pub const fn buf2_readoffset(&self) -> &Buf2Readoffset {
        &self.buf2_readoffset
    }
    #[doc = "0xa0 - No Description"]
    #[inline(always)]
    pub const fn buf2_readdata(&self) -> &Buf2Readdata {
        &self.buf2_readdata
    }
    #[doc = "0xa4 - No Description"]
    #[inline(always)]
    pub const fn buf2_writedata(&self) -> &Buf2Writedata {
        &self.buf2_writedata
    }
    #[doc = "0xa8 - No Description"]
    #[inline(always)]
    pub const fn buf2_xwrite(&self) -> &Buf2Xwrite {
        &self.buf2_xwrite
    }
    #[doc = "0xac - No Description"]
    #[inline(always)]
    pub const fn buf2_status(&self) -> &Buf2Status {
        &self.buf2_status
    }
    #[doc = "0xb0 - No Description"]
    #[inline(always)]
    pub const fn buf2_thresholdctrl(&self) -> &Buf2Thresholdctrl {
        &self.buf2_thresholdctrl
    }
    #[doc = "0xb4 - No Description"]
    #[inline(always)]
    pub const fn buf2_cmd(&self) -> &Buf2Cmd {
        &self.buf2_cmd
    }
    #[doc = "0xbc - No Description"]
    #[inline(always)]
    pub const fn buf2_readdata32(&self) -> &Buf2Readdata32 {
        &self.buf2_readdata32
    }
    #[doc = "0xc0 - No Description"]
    #[inline(always)]
    pub const fn buf2_writedata32(&self) -> &Buf2Writedata32 {
        &self.buf2_writedata32
    }
    #[doc = "0xc4 - No Description"]
    #[inline(always)]
    pub const fn buf2_xwrite32(&self) -> &Buf2Xwrite32 {
        &self.buf2_xwrite32
    }
    #[doc = "0xcc - No Description"]
    #[inline(always)]
    pub const fn buf3_ctrl(&self) -> &Buf3Ctrl {
        &self.buf3_ctrl
    }
    #[doc = "0xd0 - No Description"]
    #[inline(always)]
    pub const fn buf3_addr(&self) -> &Buf3Addr {
        &self.buf3_addr
    }
    #[doc = "0xd4 - No Description"]
    #[inline(always)]
    pub const fn buf3_writeoffset(&self) -> &Buf3Writeoffset {
        &self.buf3_writeoffset
    }
    #[doc = "0xd8 - No Description"]
    #[inline(always)]
    pub const fn buf3_readoffset(&self) -> &Buf3Readoffset {
        &self.buf3_readoffset
    }
    #[doc = "0xe0 - No Description"]
    #[inline(always)]
    pub const fn buf3_readdata(&self) -> &Buf3Readdata {
        &self.buf3_readdata
    }
    #[doc = "0xe4 - No Description"]
    #[inline(always)]
    pub const fn buf3_writedata(&self) -> &Buf3Writedata {
        &self.buf3_writedata
    }
    #[doc = "0xe8 - No Description"]
    #[inline(always)]
    pub const fn buf3_xwrite(&self) -> &Buf3Xwrite {
        &self.buf3_xwrite
    }
    #[doc = "0xec - No Description"]
    #[inline(always)]
    pub const fn buf3_status(&self) -> &Buf3Status {
        &self.buf3_status
    }
    #[doc = "0xf0 - No Description"]
    #[inline(always)]
    pub const fn buf3_thresholdctrl(&self) -> &Buf3Thresholdctrl {
        &self.buf3_thresholdctrl
    }
    #[doc = "0xf4 - No Description"]
    #[inline(always)]
    pub const fn buf3_cmd(&self) -> &Buf3Cmd {
        &self.buf3_cmd
    }
    #[doc = "0xfc - No Description"]
    #[inline(always)]
    pub const fn buf3_readdata32(&self) -> &Buf3Readdata32 {
        &self.buf3_readdata32
    }
    #[doc = "0x100 - No Description"]
    #[inline(always)]
    pub const fn buf3_writedata32(&self) -> &Buf3Writedata32 {
        &self.buf3_writedata32
    }
    #[doc = "0x104 - No Description"]
    #[inline(always)]
    pub const fn buf3_xwrite32(&self) -> &Buf3Xwrite32 {
        &self.buf3_xwrite32
    }
    #[doc = "0x114 - No Description"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x118 - No Description"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x11c - No Description"]
    #[inline(always)]
    pub const fn seqif(&self) -> &Seqif {
        &self.seqif
    }
    #[doc = "0x120 - No Description"]
    #[inline(always)]
    pub const fn seqien(&self) -> &Seqien {
        &self.seqien
    }
    #[doc = "0x124 - No Description"]
    #[inline(always)]
    pub const fn sfmif(&self) -> &Sfmif {
        &self.sfmif
    }
    #[doc = "0x128 - No Description"]
    #[inline(always)]
    pub const fn sfmien(&self) -> &Sfmien {
        &self.sfmien
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
#[doc = "LPMODE (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`lpmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpmode`] module"]
#[doc(alias = "LPMODE")]
pub type Lpmode = crate::Reg<lpmode::LpmodeSpec>;
#[doc = "No Description"]
pub mod lpmode;
#[doc = "BUF0_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf0_ctrl`] module"]
#[doc(alias = "BUF0_CTRL")]
pub type Buf0Ctrl = crate::Reg<buf0_ctrl::Buf0CtrlSpec>;
#[doc = "No Description"]
pub mod buf0_ctrl;
#[doc = "BUF0_ADDR (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf0_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf0_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf0_addr`] module"]
#[doc(alias = "BUF0_ADDR")]
pub type Buf0Addr = crate::Reg<buf0_addr::Buf0AddrSpec>;
#[doc = "No Description"]
pub mod buf0_addr;
#[doc = "BUF0_WRITEOFFSET (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf0_writeoffset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf0_writeoffset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf0_writeoffset`] module"]
#[doc(alias = "BUF0_WRITEOFFSET")]
pub type Buf0Writeoffset = crate::Reg<buf0_writeoffset::Buf0WriteoffsetSpec>;
#[doc = "No Description"]
pub mod buf0_writeoffset;
#[doc = "BUF0_READOFFSET (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf0_readoffset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf0_readoffset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf0_readoffset`] module"]
#[doc(alias = "BUF0_READOFFSET")]
pub type Buf0Readoffset = crate::Reg<buf0_readoffset::Buf0ReadoffsetSpec>;
#[doc = "No Description"]
pub mod buf0_readoffset;
#[doc = "BUF0_READDATA (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf0_readdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf0_readdata`] module"]
#[doc(alias = "BUF0_READDATA")]
pub type Buf0Readdata = crate::Reg<buf0_readdata::Buf0ReaddataSpec>;
#[doc = "No Description"]
pub mod buf0_readdata;
#[doc = "BUF0_WRITEDATA (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf0_writedata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf0_writedata`] module"]
#[doc(alias = "BUF0_WRITEDATA")]
pub type Buf0Writedata = crate::Reg<buf0_writedata::Buf0WritedataSpec>;
#[doc = "No Description"]
pub mod buf0_writedata;
#[doc = "BUF0_XWRITE (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf0_xwrite::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf0_xwrite`] module"]
#[doc(alias = "BUF0_XWRITE")]
pub type Buf0Xwrite = crate::Reg<buf0_xwrite::Buf0XwriteSpec>;
#[doc = "No Description"]
pub mod buf0_xwrite;
#[doc = "BUF0_STATUS (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf0_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf0_status`] module"]
#[doc(alias = "BUF0_STATUS")]
pub type Buf0Status = crate::Reg<buf0_status::Buf0StatusSpec>;
#[doc = "No Description"]
pub mod buf0_status;
#[doc = "BUF0_THRESHOLDCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf0_thresholdctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf0_thresholdctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf0_thresholdctrl`] module"]
#[doc(alias = "BUF0_THRESHOLDCTRL")]
pub type Buf0Thresholdctrl = crate::Reg<buf0_thresholdctrl::Buf0ThresholdctrlSpec>;
#[doc = "No Description"]
pub mod buf0_thresholdctrl;
#[doc = "BUF0_CMD (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf0_cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf0_cmd`] module"]
#[doc(alias = "BUF0_CMD")]
pub type Buf0Cmd = crate::Reg<buf0_cmd::Buf0CmdSpec>;
#[doc = "No Description"]
pub mod buf0_cmd;
#[doc = "BUF0_READDATA32 (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf0_readdata32::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf0_readdata32`] module"]
#[doc(alias = "BUF0_READDATA32")]
pub type Buf0Readdata32 = crate::Reg<buf0_readdata32::Buf0Readdata32Spec>;
#[doc = "No Description"]
pub mod buf0_readdata32;
#[doc = "BUF0_WRITEDATA32 (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf0_writedata32::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf0_writedata32`] module"]
#[doc(alias = "BUF0_WRITEDATA32")]
pub type Buf0Writedata32 = crate::Reg<buf0_writedata32::Buf0Writedata32Spec>;
#[doc = "No Description"]
pub mod buf0_writedata32;
#[doc = "BUF0_XWRITE32 (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf0_xwrite32::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf0_xwrite32`] module"]
#[doc(alias = "BUF0_XWRITE32")]
pub type Buf0Xwrite32 = crate::Reg<buf0_xwrite32::Buf0Xwrite32Spec>;
#[doc = "No Description"]
pub mod buf0_xwrite32;
#[doc = "BUF1_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf1_ctrl`] module"]
#[doc(alias = "BUF1_CTRL")]
pub type Buf1Ctrl = crate::Reg<buf1_ctrl::Buf1CtrlSpec>;
#[doc = "No Description"]
pub mod buf1_ctrl;
#[doc = "BUF1_ADDR (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf1_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf1_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf1_addr`] module"]
#[doc(alias = "BUF1_ADDR")]
pub type Buf1Addr = crate::Reg<buf1_addr::Buf1AddrSpec>;
#[doc = "No Description"]
pub mod buf1_addr;
#[doc = "BUF1_WRITEOFFSET (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf1_writeoffset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf1_writeoffset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf1_writeoffset`] module"]
#[doc(alias = "BUF1_WRITEOFFSET")]
pub type Buf1Writeoffset = crate::Reg<buf1_writeoffset::Buf1WriteoffsetSpec>;
#[doc = "No Description"]
pub mod buf1_writeoffset;
#[doc = "BUF1_READOFFSET (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf1_readoffset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf1_readoffset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf1_readoffset`] module"]
#[doc(alias = "BUF1_READOFFSET")]
pub type Buf1Readoffset = crate::Reg<buf1_readoffset::Buf1ReadoffsetSpec>;
#[doc = "No Description"]
pub mod buf1_readoffset;
#[doc = "BUF1_READDATA (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf1_readdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf1_readdata`] module"]
#[doc(alias = "BUF1_READDATA")]
pub type Buf1Readdata = crate::Reg<buf1_readdata::Buf1ReaddataSpec>;
#[doc = "No Description"]
pub mod buf1_readdata;
#[doc = "BUF1_WRITEDATA (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf1_writedata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf1_writedata`] module"]
#[doc(alias = "BUF1_WRITEDATA")]
pub type Buf1Writedata = crate::Reg<buf1_writedata::Buf1WritedataSpec>;
#[doc = "No Description"]
pub mod buf1_writedata;
#[doc = "BUF1_XWRITE (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf1_xwrite::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf1_xwrite`] module"]
#[doc(alias = "BUF1_XWRITE")]
pub type Buf1Xwrite = crate::Reg<buf1_xwrite::Buf1XwriteSpec>;
#[doc = "No Description"]
pub mod buf1_xwrite;
#[doc = "BUF1_STATUS (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf1_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf1_status`] module"]
#[doc(alias = "BUF1_STATUS")]
pub type Buf1Status = crate::Reg<buf1_status::Buf1StatusSpec>;
#[doc = "No Description"]
pub mod buf1_status;
#[doc = "BUF1_THRESHOLDCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf1_thresholdctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf1_thresholdctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf1_thresholdctrl`] module"]
#[doc(alias = "BUF1_THRESHOLDCTRL")]
pub type Buf1Thresholdctrl = crate::Reg<buf1_thresholdctrl::Buf1ThresholdctrlSpec>;
#[doc = "No Description"]
pub mod buf1_thresholdctrl;
#[doc = "BUF1_CMD (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf1_cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf1_cmd`] module"]
#[doc(alias = "BUF1_CMD")]
pub type Buf1Cmd = crate::Reg<buf1_cmd::Buf1CmdSpec>;
#[doc = "No Description"]
pub mod buf1_cmd;
#[doc = "BUF1_READDATA32 (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf1_readdata32::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf1_readdata32`] module"]
#[doc(alias = "BUF1_READDATA32")]
pub type Buf1Readdata32 = crate::Reg<buf1_readdata32::Buf1Readdata32Spec>;
#[doc = "No Description"]
pub mod buf1_readdata32;
#[doc = "BUF1_WRITEDATA32 (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf1_writedata32::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf1_writedata32`] module"]
#[doc(alias = "BUF1_WRITEDATA32")]
pub type Buf1Writedata32 = crate::Reg<buf1_writedata32::Buf1Writedata32Spec>;
#[doc = "No Description"]
pub mod buf1_writedata32;
#[doc = "BUF1_XWRITE32 (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf1_xwrite32::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf1_xwrite32`] module"]
#[doc(alias = "BUF1_XWRITE32")]
pub type Buf1Xwrite32 = crate::Reg<buf1_xwrite32::Buf1Xwrite32Spec>;
#[doc = "No Description"]
pub mod buf1_xwrite32;
#[doc = "BUF2_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf2_ctrl`] module"]
#[doc(alias = "BUF2_CTRL")]
pub type Buf2Ctrl = crate::Reg<buf2_ctrl::Buf2CtrlSpec>;
#[doc = "No Description"]
pub mod buf2_ctrl;
#[doc = "BUF2_ADDR (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf2_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf2_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf2_addr`] module"]
#[doc(alias = "BUF2_ADDR")]
pub type Buf2Addr = crate::Reg<buf2_addr::Buf2AddrSpec>;
#[doc = "No Description"]
pub mod buf2_addr;
#[doc = "BUF2_WRITEOFFSET (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf2_writeoffset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf2_writeoffset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf2_writeoffset`] module"]
#[doc(alias = "BUF2_WRITEOFFSET")]
pub type Buf2Writeoffset = crate::Reg<buf2_writeoffset::Buf2WriteoffsetSpec>;
#[doc = "No Description"]
pub mod buf2_writeoffset;
#[doc = "BUF2_READOFFSET (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf2_readoffset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf2_readoffset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf2_readoffset`] module"]
#[doc(alias = "BUF2_READOFFSET")]
pub type Buf2Readoffset = crate::Reg<buf2_readoffset::Buf2ReadoffsetSpec>;
#[doc = "No Description"]
pub mod buf2_readoffset;
#[doc = "BUF2_READDATA (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf2_readdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf2_readdata`] module"]
#[doc(alias = "BUF2_READDATA")]
pub type Buf2Readdata = crate::Reg<buf2_readdata::Buf2ReaddataSpec>;
#[doc = "No Description"]
pub mod buf2_readdata;
#[doc = "BUF2_WRITEDATA (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf2_writedata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf2_writedata`] module"]
#[doc(alias = "BUF2_WRITEDATA")]
pub type Buf2Writedata = crate::Reg<buf2_writedata::Buf2WritedataSpec>;
#[doc = "No Description"]
pub mod buf2_writedata;
#[doc = "BUF2_XWRITE (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf2_xwrite::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf2_xwrite`] module"]
#[doc(alias = "BUF2_XWRITE")]
pub type Buf2Xwrite = crate::Reg<buf2_xwrite::Buf2XwriteSpec>;
#[doc = "No Description"]
pub mod buf2_xwrite;
#[doc = "BUF2_STATUS (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf2_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf2_status`] module"]
#[doc(alias = "BUF2_STATUS")]
pub type Buf2Status = crate::Reg<buf2_status::Buf2StatusSpec>;
#[doc = "No Description"]
pub mod buf2_status;
#[doc = "BUF2_THRESHOLDCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf2_thresholdctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf2_thresholdctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf2_thresholdctrl`] module"]
#[doc(alias = "BUF2_THRESHOLDCTRL")]
pub type Buf2Thresholdctrl = crate::Reg<buf2_thresholdctrl::Buf2ThresholdctrlSpec>;
#[doc = "No Description"]
pub mod buf2_thresholdctrl;
#[doc = "BUF2_CMD (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf2_cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf2_cmd`] module"]
#[doc(alias = "BUF2_CMD")]
pub type Buf2Cmd = crate::Reg<buf2_cmd::Buf2CmdSpec>;
#[doc = "No Description"]
pub mod buf2_cmd;
#[doc = "BUF2_READDATA32 (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf2_readdata32::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf2_readdata32`] module"]
#[doc(alias = "BUF2_READDATA32")]
pub type Buf2Readdata32 = crate::Reg<buf2_readdata32::Buf2Readdata32Spec>;
#[doc = "No Description"]
pub mod buf2_readdata32;
#[doc = "BUF2_WRITEDATA32 (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf2_writedata32::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf2_writedata32`] module"]
#[doc(alias = "BUF2_WRITEDATA32")]
pub type Buf2Writedata32 = crate::Reg<buf2_writedata32::Buf2Writedata32Spec>;
#[doc = "No Description"]
pub mod buf2_writedata32;
#[doc = "BUF2_XWRITE32 (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf2_xwrite32::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf2_xwrite32`] module"]
#[doc(alias = "BUF2_XWRITE32")]
pub type Buf2Xwrite32 = crate::Reg<buf2_xwrite32::Buf2Xwrite32Spec>;
#[doc = "No Description"]
pub mod buf2_xwrite32;
#[doc = "BUF3_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf3_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf3_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf3_ctrl`] module"]
#[doc(alias = "BUF3_CTRL")]
pub type Buf3Ctrl = crate::Reg<buf3_ctrl::Buf3CtrlSpec>;
#[doc = "No Description"]
pub mod buf3_ctrl;
#[doc = "BUF3_ADDR (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf3_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf3_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf3_addr`] module"]
#[doc(alias = "BUF3_ADDR")]
pub type Buf3Addr = crate::Reg<buf3_addr::Buf3AddrSpec>;
#[doc = "No Description"]
pub mod buf3_addr;
#[doc = "BUF3_WRITEOFFSET (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf3_writeoffset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf3_writeoffset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf3_writeoffset`] module"]
#[doc(alias = "BUF3_WRITEOFFSET")]
pub type Buf3Writeoffset = crate::Reg<buf3_writeoffset::Buf3WriteoffsetSpec>;
#[doc = "No Description"]
pub mod buf3_writeoffset;
#[doc = "BUF3_READOFFSET (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf3_readoffset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf3_readoffset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf3_readoffset`] module"]
#[doc(alias = "BUF3_READOFFSET")]
pub type Buf3Readoffset = crate::Reg<buf3_readoffset::Buf3ReadoffsetSpec>;
#[doc = "No Description"]
pub mod buf3_readoffset;
#[doc = "BUF3_READDATA (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf3_readdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf3_readdata`] module"]
#[doc(alias = "BUF3_READDATA")]
pub type Buf3Readdata = crate::Reg<buf3_readdata::Buf3ReaddataSpec>;
#[doc = "No Description"]
pub mod buf3_readdata;
#[doc = "BUF3_WRITEDATA (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf3_writedata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf3_writedata`] module"]
#[doc(alias = "BUF3_WRITEDATA")]
pub type Buf3Writedata = crate::Reg<buf3_writedata::Buf3WritedataSpec>;
#[doc = "No Description"]
pub mod buf3_writedata;
#[doc = "BUF3_XWRITE (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf3_xwrite::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf3_xwrite`] module"]
#[doc(alias = "BUF3_XWRITE")]
pub type Buf3Xwrite = crate::Reg<buf3_xwrite::Buf3XwriteSpec>;
#[doc = "No Description"]
pub mod buf3_xwrite;
#[doc = "BUF3_STATUS (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf3_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf3_status`] module"]
#[doc(alias = "BUF3_STATUS")]
pub type Buf3Status = crate::Reg<buf3_status::Buf3StatusSpec>;
#[doc = "No Description"]
pub mod buf3_status;
#[doc = "BUF3_THRESHOLDCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf3_thresholdctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf3_thresholdctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf3_thresholdctrl`] module"]
#[doc(alias = "BUF3_THRESHOLDCTRL")]
pub type Buf3Thresholdctrl = crate::Reg<buf3_thresholdctrl::Buf3ThresholdctrlSpec>;
#[doc = "No Description"]
pub mod buf3_thresholdctrl;
#[doc = "BUF3_CMD (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf3_cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf3_cmd`] module"]
#[doc(alias = "BUF3_CMD")]
pub type Buf3Cmd = crate::Reg<buf3_cmd::Buf3CmdSpec>;
#[doc = "No Description"]
pub mod buf3_cmd;
#[doc = "BUF3_READDATA32 (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf3_readdata32::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf3_readdata32`] module"]
#[doc(alias = "BUF3_READDATA32")]
pub type Buf3Readdata32 = crate::Reg<buf3_readdata32::Buf3Readdata32Spec>;
#[doc = "No Description"]
pub mod buf3_readdata32;
#[doc = "BUF3_WRITEDATA32 (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf3_writedata32::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf3_writedata32`] module"]
#[doc(alias = "BUF3_WRITEDATA32")]
pub type Buf3Writedata32 = crate::Reg<buf3_writedata32::Buf3Writedata32Spec>;
#[doc = "No Description"]
pub mod buf3_writedata32;
#[doc = "BUF3_XWRITE32 (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf3_xwrite32::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf3_xwrite32`] module"]
#[doc(alias = "BUF3_XWRITE32")]
pub type Buf3Xwrite32 = crate::Reg<buf3_xwrite32::Buf3Xwrite32Spec>;
#[doc = "No Description"]
pub mod buf3_xwrite32;
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
#[doc = "SEQIF (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`seqif::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqif::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seqif`] module"]
#[doc(alias = "SEQIF")]
pub type Seqif = crate::Reg<seqif::SeqifSpec>;
#[doc = "No Description"]
pub mod seqif;
#[doc = "SEQIEN (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`seqien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seqien`] module"]
#[doc(alias = "SEQIEN")]
pub type Seqien = crate::Reg<seqien::SeqienSpec>;
#[doc = "No Description"]
pub mod seqien;
#[doc = "SFMIF (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`sfmif::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmif::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfmif`] module"]
#[doc(alias = "SFMIF")]
pub type Sfmif = crate::Reg<sfmif::SfmifSpec>;
#[doc = "No Description"]
pub mod sfmif;
#[doc = "SFMIEN (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`sfmien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfmien`] module"]
#[doc(alias = "SFMIEN")]
pub type Sfmien = crate::Reg<sfmien::SfmienSpec>;
#[doc = "No Description"]
pub mod sfmien;
