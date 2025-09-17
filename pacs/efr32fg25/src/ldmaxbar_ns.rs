#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    ch0_reqsel: Ch0Reqsel,
    ch1_reqsel: Ch1Reqsel,
    ch2_reqsel: Ch2Reqsel,
    ch3_reqsel: Ch3Reqsel,
    ch4_reqsel: Ch4Reqsel,
    ch5_reqsel: Ch5Reqsel,
    ch6_reqsel: Ch6Reqsel,
    ch7_reqsel: Ch7Reqsel,
    ch8_reqsel: Ch8Reqsel,
    ch9_reqsel: Ch9Reqsel,
    ch10_reqsel: Ch10Reqsel,
    ch11_reqsel: Ch11Reqsel,
    ch12_reqsel: Ch12Reqsel,
    ch13_reqsel: Ch13Reqsel,
    ch14_reqsel: Ch14Reqsel,
    ch15_reqsel: Ch15Reqsel,
}
impl RegisterBlock {
    #[doc = "0x00 - No Description"]
    #[inline(always)]
    pub const fn ipversion(&self) -> &Ipversion {
        &self.ipversion
    }
    #[doc = "0x04 - No Description"]
    #[inline(always)]
    pub const fn ch0_reqsel(&self) -> &Ch0Reqsel {
        &self.ch0_reqsel
    }
    #[doc = "0x08 - No Description"]
    #[inline(always)]
    pub const fn ch1_reqsel(&self) -> &Ch1Reqsel {
        &self.ch1_reqsel
    }
    #[doc = "0x0c - No Description"]
    #[inline(always)]
    pub const fn ch2_reqsel(&self) -> &Ch2Reqsel {
        &self.ch2_reqsel
    }
    #[doc = "0x10 - No Description"]
    #[inline(always)]
    pub const fn ch3_reqsel(&self) -> &Ch3Reqsel {
        &self.ch3_reqsel
    }
    #[doc = "0x14 - No Description"]
    #[inline(always)]
    pub const fn ch4_reqsel(&self) -> &Ch4Reqsel {
        &self.ch4_reqsel
    }
    #[doc = "0x18 - No Description"]
    #[inline(always)]
    pub const fn ch5_reqsel(&self) -> &Ch5Reqsel {
        &self.ch5_reqsel
    }
    #[doc = "0x1c - No Description"]
    #[inline(always)]
    pub const fn ch6_reqsel(&self) -> &Ch6Reqsel {
        &self.ch6_reqsel
    }
    #[doc = "0x20 - No Description"]
    #[inline(always)]
    pub const fn ch7_reqsel(&self) -> &Ch7Reqsel {
        &self.ch7_reqsel
    }
    #[doc = "0x24 - No Description"]
    #[inline(always)]
    pub const fn ch8_reqsel(&self) -> &Ch8Reqsel {
        &self.ch8_reqsel
    }
    #[doc = "0x28 - No Description"]
    #[inline(always)]
    pub const fn ch9_reqsel(&self) -> &Ch9Reqsel {
        &self.ch9_reqsel
    }
    #[doc = "0x2c - No Description"]
    #[inline(always)]
    pub const fn ch10_reqsel(&self) -> &Ch10Reqsel {
        &self.ch10_reqsel
    }
    #[doc = "0x30 - No Description"]
    #[inline(always)]
    pub const fn ch11_reqsel(&self) -> &Ch11Reqsel {
        &self.ch11_reqsel
    }
    #[doc = "0x34 - No Description"]
    #[inline(always)]
    pub const fn ch12_reqsel(&self) -> &Ch12Reqsel {
        &self.ch12_reqsel
    }
    #[doc = "0x38 - No Description"]
    #[inline(always)]
    pub const fn ch13_reqsel(&self) -> &Ch13Reqsel {
        &self.ch13_reqsel
    }
    #[doc = "0x3c - No Description"]
    #[inline(always)]
    pub const fn ch14_reqsel(&self) -> &Ch14Reqsel {
        &self.ch14_reqsel
    }
    #[doc = "0x40 - No Description"]
    #[inline(always)]
    pub const fn ch15_reqsel(&self) -> &Ch15Reqsel {
        &self.ch15_reqsel
    }
}
#[doc = "IPVERSION (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ipversion::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipversion`] module"]
#[doc(alias = "IPVERSION")]
pub type Ipversion = crate::Reg<ipversion::IpversionSpec>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "CH0_REQSEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_reqsel`] module"]
#[doc(alias = "CH0_REQSEL")]
pub type Ch0Reqsel = crate::Reg<ch0_reqsel::Ch0ReqselSpec>;
#[doc = "No Description"]
pub mod ch0_reqsel;
#[doc = "CH1_REQSEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_reqsel`] module"]
#[doc(alias = "CH1_REQSEL")]
pub type Ch1Reqsel = crate::Reg<ch1_reqsel::Ch1ReqselSpec>;
#[doc = "No Description"]
pub mod ch1_reqsel;
#[doc = "CH2_REQSEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_reqsel`] module"]
#[doc(alias = "CH2_REQSEL")]
pub type Ch2Reqsel = crate::Reg<ch2_reqsel::Ch2ReqselSpec>;
#[doc = "No Description"]
pub mod ch2_reqsel;
#[doc = "CH3_REQSEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_reqsel`] module"]
#[doc(alias = "CH3_REQSEL")]
pub type Ch3Reqsel = crate::Reg<ch3_reqsel::Ch3ReqselSpec>;
#[doc = "No Description"]
pub mod ch3_reqsel;
#[doc = "CH4_REQSEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_reqsel`] module"]
#[doc(alias = "CH4_REQSEL")]
pub type Ch4Reqsel = crate::Reg<ch4_reqsel::Ch4ReqselSpec>;
#[doc = "No Description"]
pub mod ch4_reqsel;
#[doc = "CH5_REQSEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_reqsel`] module"]
#[doc(alias = "CH5_REQSEL")]
pub type Ch5Reqsel = crate::Reg<ch5_reqsel::Ch5ReqselSpec>;
#[doc = "No Description"]
pub mod ch5_reqsel;
#[doc = "CH6_REQSEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_reqsel`] module"]
#[doc(alias = "CH6_REQSEL")]
pub type Ch6Reqsel = crate::Reg<ch6_reqsel::Ch6ReqselSpec>;
#[doc = "No Description"]
pub mod ch6_reqsel;
#[doc = "CH7_REQSEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_reqsel`] module"]
#[doc(alias = "CH7_REQSEL")]
pub type Ch7Reqsel = crate::Reg<ch7_reqsel::Ch7ReqselSpec>;
#[doc = "No Description"]
pub mod ch7_reqsel;
#[doc = "CH8_REQSEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_reqsel`] module"]
#[doc(alias = "CH8_REQSEL")]
pub type Ch8Reqsel = crate::Reg<ch8_reqsel::Ch8ReqselSpec>;
#[doc = "No Description"]
pub mod ch8_reqsel;
#[doc = "CH9_REQSEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_reqsel`] module"]
#[doc(alias = "CH9_REQSEL")]
pub type Ch9Reqsel = crate::Reg<ch9_reqsel::Ch9ReqselSpec>;
#[doc = "No Description"]
pub mod ch9_reqsel;
#[doc = "CH10_REQSEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch10_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_reqsel`] module"]
#[doc(alias = "CH10_REQSEL")]
pub type Ch10Reqsel = crate::Reg<ch10_reqsel::Ch10ReqselSpec>;
#[doc = "No Description"]
pub mod ch10_reqsel;
#[doc = "CH11_REQSEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_reqsel`] module"]
#[doc(alias = "CH11_REQSEL")]
pub type Ch11Reqsel = crate::Reg<ch11_reqsel::Ch11ReqselSpec>;
#[doc = "No Description"]
pub mod ch11_reqsel;
#[doc = "CH12_REQSEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch12_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_reqsel`] module"]
#[doc(alias = "CH12_REQSEL")]
pub type Ch12Reqsel = crate::Reg<ch12_reqsel::Ch12ReqselSpec>;
#[doc = "No Description"]
pub mod ch12_reqsel;
#[doc = "CH13_REQSEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_reqsel`] module"]
#[doc(alias = "CH13_REQSEL")]
pub type Ch13Reqsel = crate::Reg<ch13_reqsel::Ch13ReqselSpec>;
#[doc = "No Description"]
pub mod ch13_reqsel;
#[doc = "CH14_REQSEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch14_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_reqsel`] module"]
#[doc(alias = "CH14_REQSEL")]
pub type Ch14Reqsel = crate::Reg<ch14_reqsel::Ch14ReqselSpec>;
#[doc = "No Description"]
pub mod ch14_reqsel;
#[doc = "CH15_REQSEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_reqsel`] module"]
#[doc(alias = "CH15_REQSEL")]
pub type Ch15Reqsel = crate::Reg<ch15_reqsel::Ch15ReqselSpec>;
#[doc = "No Description"]
pub mod ch15_reqsel;
