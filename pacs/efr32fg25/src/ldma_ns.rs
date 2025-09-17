#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    en: En,
    swrst: Swrst,
    ctrl: Ctrl,
    status: Status,
    syncswset: Syncswset,
    syncswclr: Syncswclr,
    synchwen: Synchwen,
    synchwsel: Synchwsel,
    syncstatus: Syncstatus,
    chen: Chen,
    chdis: Chdis,
    chstatus: Chstatus,
    chbusy: Chbusy,
    chdone: Chdone,
    dbghalt: Dbghalt,
    swreq: Swreq,
    reqdis: Reqdis,
    reqpend: Reqpend,
    linkload: Linkload,
    reqclear: Reqclear,
    if_: If,
    ien: Ien,
    _reserved23: [u8; 0x04],
    ch0_cfg: Ch0Cfg,
    ch0_loop: Ch0Loop,
    ch0_ctrl: Ch0Ctrl,
    ch0_src: Ch0Src,
    ch0_dst: Ch0Dst,
    ch0_link: Ch0Link,
    ch0_xctrl: Ch0Xctrl,
    _reserved30: [u8; 0x04],
    ch0_ilsrc: Ch0Ilsrc,
    _reserved31: [u8; 0x0c],
    ch1_cfg: Ch1Cfg,
    ch1_loop: Ch1Loop,
    ch1_ctrl: Ch1Ctrl,
    ch1_src: Ch1Src,
    ch1_dst: Ch1Dst,
    ch1_link: Ch1Link,
    ch1_xctrl: Ch1Xctrl,
    _reserved38: [u8; 0x04],
    ch1_ilsrc: Ch1Ilsrc,
    _reserved39: [u8; 0x0c],
    ch2_cfg: Ch2Cfg,
    ch2_loop: Ch2Loop,
    ch2_ctrl: Ch2Ctrl,
    ch2_src: Ch2Src,
    ch2_dst: Ch2Dst,
    ch2_link: Ch2Link,
    ch2_xctrl: Ch2Xctrl,
    _reserved46: [u8; 0x04],
    ch2_ilsrc: Ch2Ilsrc,
    _reserved47: [u8; 0x0c],
    ch3_cfg: Ch3Cfg,
    ch3_loop: Ch3Loop,
    ch3_ctrl: Ch3Ctrl,
    ch3_src: Ch3Src,
    ch3_dst: Ch3Dst,
    ch3_link: Ch3Link,
    ch3_xctrl: Ch3Xctrl,
    _reserved54: [u8; 0x04],
    ch3_ilsrc: Ch3Ilsrc,
    _reserved55: [u8; 0x0c],
    ch4_cfg: Ch4Cfg,
    ch4_loop: Ch4Loop,
    ch4_ctrl: Ch4Ctrl,
    ch4_src: Ch4Src,
    ch4_dst: Ch4Dst,
    ch4_link: Ch4Link,
    ch4_xctrl: Ch4Xctrl,
    _reserved62: [u8; 0x04],
    ch4_ilsrc: Ch4Ilsrc,
    _reserved63: [u8; 0x0c],
    ch5_cfg: Ch5Cfg,
    ch5_loop: Ch5Loop,
    ch5_ctrl: Ch5Ctrl,
    ch5_src: Ch5Src,
    ch5_dst: Ch5Dst,
    ch5_link: Ch5Link,
    ch5_xctrl: Ch5Xctrl,
    _reserved70: [u8; 0x04],
    ch5_ilsrc: Ch5Ilsrc,
    _reserved71: [u8; 0x0c],
    ch6_cfg: Ch6Cfg,
    ch6_loop: Ch6Loop,
    ch6_ctrl: Ch6Ctrl,
    ch6_src: Ch6Src,
    ch6_dst: Ch6Dst,
    ch6_link: Ch6Link,
    ch6_xctrl: Ch6Xctrl,
    _reserved78: [u8; 0x04],
    ch6_ilsrc: Ch6Ilsrc,
    _reserved79: [u8; 0x0c],
    ch7_cfg: Ch7Cfg,
    ch7_loop: Ch7Loop,
    ch7_ctrl: Ch7Ctrl,
    ch7_src: Ch7Src,
    ch7_dst: Ch7Dst,
    ch7_link: Ch7Link,
    ch7_xctrl: Ch7Xctrl,
    _reserved86: [u8; 0x04],
    ch7_ilsrc: Ch7Ilsrc,
    _reserved87: [u8; 0x0c],
    ch8_cfg: Ch8Cfg,
    ch8_loop: Ch8Loop,
    ch8_ctrl: Ch8Ctrl,
    ch8_src: Ch8Src,
    ch8_dst: Ch8Dst,
    ch8_link: Ch8Link,
    ch8_xctrl: Ch8Xctrl,
    _reserved94: [u8; 0x04],
    ch8_ilsrc: Ch8Ilsrc,
    _reserved95: [u8; 0x0c],
    ch9_cfg: Ch9Cfg,
    ch9_loop: Ch9Loop,
    ch9_ctrl: Ch9Ctrl,
    ch9_src: Ch9Src,
    ch9_dst: Ch9Dst,
    ch9_link: Ch9Link,
    ch9_xctrl: Ch9Xctrl,
    _reserved102: [u8; 0x04],
    ch9_ilsrc: Ch9Ilsrc,
    _reserved103: [u8; 0x0c],
    ch10_cfg: Ch10Cfg,
    ch10_loop: Ch10Loop,
    ch10_ctrl: Ch10Ctrl,
    ch10_src: Ch10Src,
    ch10_dst: Ch10Dst,
    ch10_link: Ch10Link,
    ch10_xctrl: Ch10Xctrl,
    _reserved110: [u8; 0x04],
    ch10_ilsrc: Ch10Ilsrc,
    _reserved111: [u8; 0x0c],
    ch11_cfg: Ch11Cfg,
    ch11_loop: Ch11Loop,
    ch11_ctrl: Ch11Ctrl,
    ch11_src: Ch11Src,
    ch11_dst: Ch11Dst,
    ch11_link: Ch11Link,
    ch11_xctrl: Ch11Xctrl,
    _reserved118: [u8; 0x04],
    ch11_ilsrc: Ch11Ilsrc,
    _reserved119: [u8; 0x0c],
    ch12_cfg: Ch12Cfg,
    ch12_loop: Ch12Loop,
    ch12_ctrl: Ch12Ctrl,
    ch12_src: Ch12Src,
    ch12_dst: Ch12Dst,
    ch12_link: Ch12Link,
    ch12_xctrl: Ch12Xctrl,
    _reserved126: [u8; 0x04],
    ch12_ilsrc: Ch12Ilsrc,
    _reserved127: [u8; 0x0c],
    ch13_cfg: Ch13Cfg,
    ch13_loop: Ch13Loop,
    ch13_ctrl: Ch13Ctrl,
    ch13_src: Ch13Src,
    ch13_dst: Ch13Dst,
    ch13_link: Ch13Link,
    ch13_xctrl: Ch13Xctrl,
    _reserved134: [u8; 0x04],
    ch13_ilsrc: Ch13Ilsrc,
    _reserved135: [u8; 0x0c],
    ch14_cfg: Ch14Cfg,
    ch14_loop: Ch14Loop,
    ch14_ctrl: Ch14Ctrl,
    ch14_src: Ch14Src,
    ch14_dst: Ch14Dst,
    ch14_link: Ch14Link,
    ch14_xctrl: Ch14Xctrl,
    _reserved142: [u8; 0x04],
    ch14_ilsrc: Ch14Ilsrc,
    _reserved143: [u8; 0x0c],
    ch15_cfg: Ch15Cfg,
    ch15_loop: Ch15Loop,
    ch15_ctrl: Ch15Ctrl,
    ch15_src: Ch15Src,
    ch15_dst: Ch15Dst,
    ch15_link: Ch15Link,
    ch15_xctrl: Ch15Xctrl,
    _reserved150: [u8; 0x04],
    ch15_ilsrc: Ch15Ilsrc,
}
impl RegisterBlock {
    #[doc = "0x00 - IP version register"]
    #[inline(always)]
    pub const fn ipversion(&self) -> &Ipversion {
        &self.ipversion
    }
    #[doc = "0x04 - Module enable disable Register"]
    #[inline(always)]
    pub const fn en(&self) -> &En {
        &self.en
    }
    #[doc = "0x08 - Software Reset Register"]
    #[inline(always)]
    pub const fn swrst(&self) -> &Swrst {
        &self.swrst
    }
    #[doc = "0x0c - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x10 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x14 - Sync Trig Sw Set Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn syncswset(&self) -> &Syncswset {
        &self.syncswset
    }
    #[doc = "0x18 - Sync Trig Sw Clear register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn syncswclr(&self) -> &Syncswclr {
        &self.syncswclr
    }
    #[doc = "0x1c - Sync HW trigger enable register"]
    #[inline(always)]
    pub const fn synchwen(&self) -> &Synchwen {
        &self.synchwen
    }
    #[doc = "0x20 - Sync HW trigger selection register"]
    #[inline(always)]
    pub const fn synchwsel(&self) -> &Synchwsel {
        &self.synchwsel
    }
    #[doc = "0x24 - Sync Trigger Status Register"]
    #[inline(always)]
    pub const fn syncstatus(&self) -> &Syncstatus {
        &self.syncstatus
    }
    #[doc = "0x28 - Channel Enable Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn chen(&self) -> &Chen {
        &self.chen
    }
    #[doc = "0x2c - Channel Disable Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn chdis(&self) -> &Chdis {
        &self.chdis
    }
    #[doc = "0x30 - Channel Status Register"]
    #[inline(always)]
    pub const fn chstatus(&self) -> &Chstatus {
        &self.chstatus
    }
    #[doc = "0x34 - Channel Busy Register"]
    #[inline(always)]
    pub const fn chbusy(&self) -> &Chbusy {
        &self.chbusy
    }
    #[doc = "0x38 - Channel Linking Done Status Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn chdone(&self) -> &Chdone {
        &self.chdone
    }
    #[doc = "0x3c - Channel Debug Halt Register"]
    #[inline(always)]
    pub const fn dbghalt(&self) -> &Dbghalt {
        &self.dbghalt
    }
    #[doc = "0x40 - Channel Software Transfer Request (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn swreq(&self) -> &Swreq {
        &self.swreq
    }
    #[doc = "0x44 - Channel Request Disable Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn reqdis(&self) -> &Reqdis {
        &self.reqdis
    }
    #[doc = "0x48 - Channel Requests Pending Register"]
    #[inline(always)]
    pub const fn reqpend(&self) -> &Reqpend {
        &self.reqpend
    }
    #[doc = "0x4c - Channel Link Load Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn linkload(&self) -> &Linkload {
        &self.linkload
    }
    #[doc = "0x50 - Channel Request Clear Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn reqclear(&self) -> &Reqclear {
        &self.reqclear
    }
    #[doc = "0x54 - Interrupt Flag Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x58 - Done Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x60 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch0_cfg(&self) -> &Ch0Cfg {
        &self.ch0_cfg
    }
    #[doc = "0x64 - Channel Loop Counter Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch0_loop(&self) -> &Ch0Loop {
        &self.ch0_loop
    }
    #[doc = "0x68 - Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch0_ctrl(&self) -> &Ch0Ctrl {
        &self.ch0_ctrl
    }
    #[doc = "0x6c - Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch0_src(&self) -> &Ch0Src {
        &self.ch0_src
    }
    #[doc = "0x70 - Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch0_dst(&self) -> &Ch0Dst {
        &self.ch0_dst
    }
    #[doc = "0x74 - Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch0_link(&self) -> &Ch0Link {
        &self.ch0_link
    }
    #[doc = "0x78 - Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch0_xctrl(&self) -> &Ch0Xctrl {
        &self.ch0_xctrl
    }
    #[doc = "0x80 - Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch0_ilsrc(&self) -> &Ch0Ilsrc {
        &self.ch0_ilsrc
    }
    #[doc = "0x90 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch1_cfg(&self) -> &Ch1Cfg {
        &self.ch1_cfg
    }
    #[doc = "0x94 - Channel Loop Counter Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch1_loop(&self) -> &Ch1Loop {
        &self.ch1_loop
    }
    #[doc = "0x98 - Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch1_ctrl(&self) -> &Ch1Ctrl {
        &self.ch1_ctrl
    }
    #[doc = "0x9c - Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch1_src(&self) -> &Ch1Src {
        &self.ch1_src
    }
    #[doc = "0xa0 - Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch1_dst(&self) -> &Ch1Dst {
        &self.ch1_dst
    }
    #[doc = "0xa4 - Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch1_link(&self) -> &Ch1Link {
        &self.ch1_link
    }
    #[doc = "0xa8 - Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch1_xctrl(&self) -> &Ch1Xctrl {
        &self.ch1_xctrl
    }
    #[doc = "0xb0 - Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch1_ilsrc(&self) -> &Ch1Ilsrc {
        &self.ch1_ilsrc
    }
    #[doc = "0xc0 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch2_cfg(&self) -> &Ch2Cfg {
        &self.ch2_cfg
    }
    #[doc = "0xc4 - Channel Loop Counter Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch2_loop(&self) -> &Ch2Loop {
        &self.ch2_loop
    }
    #[doc = "0xc8 - Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch2_ctrl(&self) -> &Ch2Ctrl {
        &self.ch2_ctrl
    }
    #[doc = "0xcc - Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch2_src(&self) -> &Ch2Src {
        &self.ch2_src
    }
    #[doc = "0xd0 - Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch2_dst(&self) -> &Ch2Dst {
        &self.ch2_dst
    }
    #[doc = "0xd4 - Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch2_link(&self) -> &Ch2Link {
        &self.ch2_link
    }
    #[doc = "0xd8 - Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch2_xctrl(&self) -> &Ch2Xctrl {
        &self.ch2_xctrl
    }
    #[doc = "0xe0 - Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch2_ilsrc(&self) -> &Ch2Ilsrc {
        &self.ch2_ilsrc
    }
    #[doc = "0xf0 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch3_cfg(&self) -> &Ch3Cfg {
        &self.ch3_cfg
    }
    #[doc = "0xf4 - Channel Loop Counter Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch3_loop(&self) -> &Ch3Loop {
        &self.ch3_loop
    }
    #[doc = "0xf8 - Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch3_ctrl(&self) -> &Ch3Ctrl {
        &self.ch3_ctrl
    }
    #[doc = "0xfc - Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch3_src(&self) -> &Ch3Src {
        &self.ch3_src
    }
    #[doc = "0x100 - Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch3_dst(&self) -> &Ch3Dst {
        &self.ch3_dst
    }
    #[doc = "0x104 - Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch3_link(&self) -> &Ch3Link {
        &self.ch3_link
    }
    #[doc = "0x108 - Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch3_xctrl(&self) -> &Ch3Xctrl {
        &self.ch3_xctrl
    }
    #[doc = "0x110 - Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch3_ilsrc(&self) -> &Ch3Ilsrc {
        &self.ch3_ilsrc
    }
    #[doc = "0x120 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch4_cfg(&self) -> &Ch4Cfg {
        &self.ch4_cfg
    }
    #[doc = "0x124 - Channel Loop Counter Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch4_loop(&self) -> &Ch4Loop {
        &self.ch4_loop
    }
    #[doc = "0x128 - Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch4_ctrl(&self) -> &Ch4Ctrl {
        &self.ch4_ctrl
    }
    #[doc = "0x12c - Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch4_src(&self) -> &Ch4Src {
        &self.ch4_src
    }
    #[doc = "0x130 - Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch4_dst(&self) -> &Ch4Dst {
        &self.ch4_dst
    }
    #[doc = "0x134 - Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch4_link(&self) -> &Ch4Link {
        &self.ch4_link
    }
    #[doc = "0x138 - Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch4_xctrl(&self) -> &Ch4Xctrl {
        &self.ch4_xctrl
    }
    #[doc = "0x140 - Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch4_ilsrc(&self) -> &Ch4Ilsrc {
        &self.ch4_ilsrc
    }
    #[doc = "0x150 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch5_cfg(&self) -> &Ch5Cfg {
        &self.ch5_cfg
    }
    #[doc = "0x154 - Channel Loop Counter Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch5_loop(&self) -> &Ch5Loop {
        &self.ch5_loop
    }
    #[doc = "0x158 - Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch5_ctrl(&self) -> &Ch5Ctrl {
        &self.ch5_ctrl
    }
    #[doc = "0x15c - Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch5_src(&self) -> &Ch5Src {
        &self.ch5_src
    }
    #[doc = "0x160 - Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch5_dst(&self) -> &Ch5Dst {
        &self.ch5_dst
    }
    #[doc = "0x164 - Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch5_link(&self) -> &Ch5Link {
        &self.ch5_link
    }
    #[doc = "0x168 - Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch5_xctrl(&self) -> &Ch5Xctrl {
        &self.ch5_xctrl
    }
    #[doc = "0x170 - Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch5_ilsrc(&self) -> &Ch5Ilsrc {
        &self.ch5_ilsrc
    }
    #[doc = "0x180 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch6_cfg(&self) -> &Ch6Cfg {
        &self.ch6_cfg
    }
    #[doc = "0x184 - Channel Loop Counter Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch6_loop(&self) -> &Ch6Loop {
        &self.ch6_loop
    }
    #[doc = "0x188 - Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch6_ctrl(&self) -> &Ch6Ctrl {
        &self.ch6_ctrl
    }
    #[doc = "0x18c - Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch6_src(&self) -> &Ch6Src {
        &self.ch6_src
    }
    #[doc = "0x190 - Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch6_dst(&self) -> &Ch6Dst {
        &self.ch6_dst
    }
    #[doc = "0x194 - Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch6_link(&self) -> &Ch6Link {
        &self.ch6_link
    }
    #[doc = "0x198 - Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch6_xctrl(&self) -> &Ch6Xctrl {
        &self.ch6_xctrl
    }
    #[doc = "0x1a0 - Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch6_ilsrc(&self) -> &Ch6Ilsrc {
        &self.ch6_ilsrc
    }
    #[doc = "0x1b0 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch7_cfg(&self) -> &Ch7Cfg {
        &self.ch7_cfg
    }
    #[doc = "0x1b4 - Channel Loop Counter Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch7_loop(&self) -> &Ch7Loop {
        &self.ch7_loop
    }
    #[doc = "0x1b8 - Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch7_ctrl(&self) -> &Ch7Ctrl {
        &self.ch7_ctrl
    }
    #[doc = "0x1bc - Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch7_src(&self) -> &Ch7Src {
        &self.ch7_src
    }
    #[doc = "0x1c0 - Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch7_dst(&self) -> &Ch7Dst {
        &self.ch7_dst
    }
    #[doc = "0x1c4 - Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch7_link(&self) -> &Ch7Link {
        &self.ch7_link
    }
    #[doc = "0x1c8 - Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch7_xctrl(&self) -> &Ch7Xctrl {
        &self.ch7_xctrl
    }
    #[doc = "0x1d0 - Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch7_ilsrc(&self) -> &Ch7Ilsrc {
        &self.ch7_ilsrc
    }
    #[doc = "0x1e0 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch8_cfg(&self) -> &Ch8Cfg {
        &self.ch8_cfg
    }
    #[doc = "0x1e4 - Channel Loop Counter Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch8_loop(&self) -> &Ch8Loop {
        &self.ch8_loop
    }
    #[doc = "0x1e8 - Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch8_ctrl(&self) -> &Ch8Ctrl {
        &self.ch8_ctrl
    }
    #[doc = "0x1ec - Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch8_src(&self) -> &Ch8Src {
        &self.ch8_src
    }
    #[doc = "0x1f0 - Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch8_dst(&self) -> &Ch8Dst {
        &self.ch8_dst
    }
    #[doc = "0x1f4 - Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch8_link(&self) -> &Ch8Link {
        &self.ch8_link
    }
    #[doc = "0x1f8 - Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch8_xctrl(&self) -> &Ch8Xctrl {
        &self.ch8_xctrl
    }
    #[doc = "0x200 - Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch8_ilsrc(&self) -> &Ch8Ilsrc {
        &self.ch8_ilsrc
    }
    #[doc = "0x210 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch9_cfg(&self) -> &Ch9Cfg {
        &self.ch9_cfg
    }
    #[doc = "0x214 - Channel Loop Counter Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch9_loop(&self) -> &Ch9Loop {
        &self.ch9_loop
    }
    #[doc = "0x218 - Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch9_ctrl(&self) -> &Ch9Ctrl {
        &self.ch9_ctrl
    }
    #[doc = "0x21c - Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch9_src(&self) -> &Ch9Src {
        &self.ch9_src
    }
    #[doc = "0x220 - Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch9_dst(&self) -> &Ch9Dst {
        &self.ch9_dst
    }
    #[doc = "0x224 - Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch9_link(&self) -> &Ch9Link {
        &self.ch9_link
    }
    #[doc = "0x228 - Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch9_xctrl(&self) -> &Ch9Xctrl {
        &self.ch9_xctrl
    }
    #[doc = "0x230 - Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch9_ilsrc(&self) -> &Ch9Ilsrc {
        &self.ch9_ilsrc
    }
    #[doc = "0x240 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch10_cfg(&self) -> &Ch10Cfg {
        &self.ch10_cfg
    }
    #[doc = "0x244 - Channel Loop Counter Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch10_loop(&self) -> &Ch10Loop {
        &self.ch10_loop
    }
    #[doc = "0x248 - Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch10_ctrl(&self) -> &Ch10Ctrl {
        &self.ch10_ctrl
    }
    #[doc = "0x24c - Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch10_src(&self) -> &Ch10Src {
        &self.ch10_src
    }
    #[doc = "0x250 - Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch10_dst(&self) -> &Ch10Dst {
        &self.ch10_dst
    }
    #[doc = "0x254 - Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch10_link(&self) -> &Ch10Link {
        &self.ch10_link
    }
    #[doc = "0x258 - Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch10_xctrl(&self) -> &Ch10Xctrl {
        &self.ch10_xctrl
    }
    #[doc = "0x260 - Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch10_ilsrc(&self) -> &Ch10Ilsrc {
        &self.ch10_ilsrc
    }
    #[doc = "0x270 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch11_cfg(&self) -> &Ch11Cfg {
        &self.ch11_cfg
    }
    #[doc = "0x274 - Channel Loop Counter Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch11_loop(&self) -> &Ch11Loop {
        &self.ch11_loop
    }
    #[doc = "0x278 - Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch11_ctrl(&self) -> &Ch11Ctrl {
        &self.ch11_ctrl
    }
    #[doc = "0x27c - Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch11_src(&self) -> &Ch11Src {
        &self.ch11_src
    }
    #[doc = "0x280 - Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch11_dst(&self) -> &Ch11Dst {
        &self.ch11_dst
    }
    #[doc = "0x284 - Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch11_link(&self) -> &Ch11Link {
        &self.ch11_link
    }
    #[doc = "0x288 - Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch11_xctrl(&self) -> &Ch11Xctrl {
        &self.ch11_xctrl
    }
    #[doc = "0x290 - Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch11_ilsrc(&self) -> &Ch11Ilsrc {
        &self.ch11_ilsrc
    }
    #[doc = "0x2a0 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch12_cfg(&self) -> &Ch12Cfg {
        &self.ch12_cfg
    }
    #[doc = "0x2a4 - Channel Loop Counter Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch12_loop(&self) -> &Ch12Loop {
        &self.ch12_loop
    }
    #[doc = "0x2a8 - Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch12_ctrl(&self) -> &Ch12Ctrl {
        &self.ch12_ctrl
    }
    #[doc = "0x2ac - Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch12_src(&self) -> &Ch12Src {
        &self.ch12_src
    }
    #[doc = "0x2b0 - Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch12_dst(&self) -> &Ch12Dst {
        &self.ch12_dst
    }
    #[doc = "0x2b4 - Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch12_link(&self) -> &Ch12Link {
        &self.ch12_link
    }
    #[doc = "0x2b8 - Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch12_xctrl(&self) -> &Ch12Xctrl {
        &self.ch12_xctrl
    }
    #[doc = "0x2c0 - Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch12_ilsrc(&self) -> &Ch12Ilsrc {
        &self.ch12_ilsrc
    }
    #[doc = "0x2d0 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch13_cfg(&self) -> &Ch13Cfg {
        &self.ch13_cfg
    }
    #[doc = "0x2d4 - Channel Loop Counter Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch13_loop(&self) -> &Ch13Loop {
        &self.ch13_loop
    }
    #[doc = "0x2d8 - Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch13_ctrl(&self) -> &Ch13Ctrl {
        &self.ch13_ctrl
    }
    #[doc = "0x2dc - Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch13_src(&self) -> &Ch13Src {
        &self.ch13_src
    }
    #[doc = "0x2e0 - Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch13_dst(&self) -> &Ch13Dst {
        &self.ch13_dst
    }
    #[doc = "0x2e4 - Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch13_link(&self) -> &Ch13Link {
        &self.ch13_link
    }
    #[doc = "0x2e8 - Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch13_xctrl(&self) -> &Ch13Xctrl {
        &self.ch13_xctrl
    }
    #[doc = "0x2f0 - Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch13_ilsrc(&self) -> &Ch13Ilsrc {
        &self.ch13_ilsrc
    }
    #[doc = "0x300 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch14_cfg(&self) -> &Ch14Cfg {
        &self.ch14_cfg
    }
    #[doc = "0x304 - Channel Loop Counter Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch14_loop(&self) -> &Ch14Loop {
        &self.ch14_loop
    }
    #[doc = "0x308 - Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch14_ctrl(&self) -> &Ch14Ctrl {
        &self.ch14_ctrl
    }
    #[doc = "0x30c - Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch14_src(&self) -> &Ch14Src {
        &self.ch14_src
    }
    #[doc = "0x310 - Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch14_dst(&self) -> &Ch14Dst {
        &self.ch14_dst
    }
    #[doc = "0x314 - Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch14_link(&self) -> &Ch14Link {
        &self.ch14_link
    }
    #[doc = "0x318 - Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch14_xctrl(&self) -> &Ch14Xctrl {
        &self.ch14_xctrl
    }
    #[doc = "0x320 - Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch14_ilsrc(&self) -> &Ch14Ilsrc {
        &self.ch14_ilsrc
    }
    #[doc = "0x330 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn ch15_cfg(&self) -> &Ch15Cfg {
        &self.ch15_cfg
    }
    #[doc = "0x334 - Channel Loop Counter Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch15_loop(&self) -> &Ch15Loop {
        &self.ch15_loop
    }
    #[doc = "0x338 - Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch15_ctrl(&self) -> &Ch15Ctrl {
        &self.ch15_ctrl
    }
    #[doc = "0x33c - Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch15_src(&self) -> &Ch15Src {
        &self.ch15_src
    }
    #[doc = "0x340 - Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch15_dst(&self) -> &Ch15Dst {
        &self.ch15_dst
    }
    #[doc = "0x344 - Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch15_link(&self) -> &Ch15Link {
        &self.ch15_link
    }
    #[doc = "0x348 - Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch15_xctrl(&self) -> &Ch15Xctrl {
        &self.ch15_xctrl
    }
    #[doc = "0x350 - Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
    #[inline(always)]
    pub const fn ch15_ilsrc(&self) -> &Ch15Ilsrc {
        &self.ch15_ilsrc
    }
}
#[doc = "IPVERSION (r) register accessor: IP version register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipversion::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipversion`] module"]
#[doc(alias = "IPVERSION")]
pub type Ipversion = crate::Reg<ipversion::IpversionSpec>;
#[doc = "IP version register"]
pub mod ipversion;
#[doc = "EN (rw) register accessor: Module enable disable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en`] module"]
#[doc(alias = "EN")]
pub type En = crate::Reg<en::EnSpec>;
#[doc = "Module enable disable Register"]
pub mod en;
#[doc = "SWRST (rw) register accessor: Software Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrst`] module"]
#[doc(alias = "SWRST")]
pub type Swrst = crate::Reg<swrst::SwrstSpec>;
#[doc = "Software Reset Register"]
pub mod swrst;
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "SYNCSWSET (w) register accessor: Sync Trig Sw Set Register (Writes will only take effect when EN=1)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syncswset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncswset`] module"]
#[doc(alias = "SYNCSWSET")]
pub type Syncswset = crate::Reg<syncswset::SyncswsetSpec>;
#[doc = "Sync Trig Sw Set Register (Writes will only take effect when EN=1)"]
pub mod syncswset;
#[doc = "SYNCSWCLR (w) register accessor: Sync Trig Sw Clear register (Writes will only take effect when EN=1)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syncswclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncswclr`] module"]
#[doc(alias = "SYNCSWCLR")]
pub type Syncswclr = crate::Reg<syncswclr::SyncswclrSpec>;
#[doc = "Sync Trig Sw Clear register (Writes will only take effect when EN=1)"]
pub mod syncswclr;
#[doc = "SYNCHWEN (rw) register accessor: Sync HW trigger enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`synchwen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synchwen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synchwen`] module"]
#[doc(alias = "SYNCHWEN")]
pub type Synchwen = crate::Reg<synchwen::SynchwenSpec>;
#[doc = "Sync HW trigger enable register"]
pub mod synchwen;
#[doc = "SYNCHWSEL (rw) register accessor: Sync HW trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`synchwsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synchwsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synchwsel`] module"]
#[doc(alias = "SYNCHWSEL")]
pub type Synchwsel = crate::Reg<synchwsel::SynchwselSpec>;
#[doc = "Sync HW trigger selection register"]
pub mod synchwsel;
#[doc = "SYNCSTATUS (r) register accessor: Sync Trigger Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncstatus`] module"]
#[doc(alias = "SYNCSTATUS")]
pub type Syncstatus = crate::Reg<syncstatus::SyncstatusSpec>;
#[doc = "Sync Trigger Status Register"]
pub mod syncstatus;
#[doc = "CHEN (w) register accessor: Channel Enable Register (Writes will only take effect when EN=1)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chen::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chen`] module"]
#[doc(alias = "CHEN")]
pub type Chen = crate::Reg<chen::ChenSpec>;
#[doc = "Channel Enable Register (Writes will only take effect when EN=1)"]
pub mod chen;
#[doc = "CHDIS (w) register accessor: Channel Disable Register (Writes will only take effect when EN=1)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chdis::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chdis`] module"]
#[doc(alias = "CHDIS")]
pub type Chdis = crate::Reg<chdis::ChdisSpec>;
#[doc = "Channel Disable Register (Writes will only take effect when EN=1)"]
pub mod chdis;
#[doc = "CHSTATUS (r) register accessor: Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chstatus`] module"]
#[doc(alias = "CHSTATUS")]
pub type Chstatus = crate::Reg<chstatus::ChstatusSpec>;
#[doc = "Channel Status Register"]
pub mod chstatus;
#[doc = "CHBUSY (r) register accessor: Channel Busy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chbusy`] module"]
#[doc(alias = "CHBUSY")]
pub type Chbusy = crate::Reg<chbusy::ChbusySpec>;
#[doc = "Channel Busy Register"]
pub mod chbusy;
#[doc = "CHDONE (rw) register accessor: Channel Linking Done Status Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`chdone::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chdone::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chdone`] module"]
#[doc(alias = "CHDONE")]
pub type Chdone = crate::Reg<chdone::ChdoneSpec>;
#[doc = "Channel Linking Done Status Register (Writes will only take effect when EN=1)"]
pub mod chdone;
#[doc = "DBGHALT (rw) register accessor: Channel Debug Halt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbghalt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbghalt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbghalt`] module"]
#[doc(alias = "DBGHALT")]
pub type Dbghalt = crate::Reg<dbghalt::DbghaltSpec>;
#[doc = "Channel Debug Halt Register"]
pub mod dbghalt;
#[doc = "SWREQ (w) register accessor: Channel Software Transfer Request (Writes will only take effect when EN=1)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreq`] module"]
#[doc(alias = "SWREQ")]
pub type Swreq = crate::Reg<swreq::SwreqSpec>;
#[doc = "Channel Software Transfer Request (Writes will only take effect when EN=1)"]
pub mod swreq;
#[doc = "REQDIS (rw) register accessor: Channel Request Disable Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`reqdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reqdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqdis`] module"]
#[doc(alias = "REQDIS")]
pub type Reqdis = crate::Reg<reqdis::ReqdisSpec>;
#[doc = "Channel Request Disable Register (Writes will only take effect when EN=1)"]
pub mod reqdis;
#[doc = "REQPEND (r) register accessor: Channel Requests Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reqpend::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqpend`] module"]
#[doc(alias = "REQPEND")]
pub type Reqpend = crate::Reg<reqpend::ReqpendSpec>;
#[doc = "Channel Requests Pending Register"]
pub mod reqpend;
#[doc = "LINKLOAD (w) register accessor: Channel Link Load Register (Writes will only take effect when EN=1)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linkload::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linkload`] module"]
#[doc(alias = "LINKLOAD")]
pub type Linkload = crate::Reg<linkload::LinkloadSpec>;
#[doc = "Channel Link Load Register (Writes will only take effect when EN=1)"]
pub mod linkload;
#[doc = "REQCLEAR (w) register accessor: Channel Request Clear Register (Writes will only take effect when EN=1)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reqclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqclear`] module"]
#[doc(alias = "REQCLEAR")]
pub type Reqclear = crate::Reg<reqclear::ReqclearSpec>;
#[doc = "Channel Request Clear Register (Writes will only take effect when EN=1)"]
pub mod reqclear;
#[doc = "IF (rw) register accessor: Interrupt Flag Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`] module"]
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IfSpec>;
#[doc = "Interrupt Flag Register (Writes will only take effect when EN=1)"]
pub mod if_;
#[doc = "IEN (rw) register accessor: Done Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`] module"]
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IenSpec>;
#[doc = "Done Interrupt Enable Register"]
pub mod ien;
#[doc = "CH0_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_cfg`] module"]
#[doc(alias = "CH0_CFG")]
pub type Ch0Cfg = crate::Reg<ch0_cfg::Ch0CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch0_cfg;
#[doc = "CH0_LOOP (rw) register accessor: Channel Loop Counter Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_loop`] module"]
#[doc(alias = "CH0_LOOP")]
pub type Ch0Loop = crate::Reg<ch0_loop::Ch0LoopSpec>;
#[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)"]
pub mod ch0_loop;
#[doc = "CH0_CTRL (rw) register accessor: Channel Descriptor Control Word Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_ctrl`] module"]
#[doc(alias = "CH0_CTRL")]
pub type Ch0Ctrl = crate::Reg<ch0_ctrl::Ch0CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
pub mod ch0_ctrl;
#[doc = "CH0_SRC (rw) register accessor: Channel Descriptor Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_src`] module"]
#[doc(alias = "CH0_SRC")]
pub type Ch0Src = crate::Reg<ch0_src::Ch0SrcSpec>;
#[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch0_src;
#[doc = "CH0_DST (rw) register accessor: Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_dst`] module"]
#[doc(alias = "CH0_DST")]
pub type Ch0Dst = crate::Reg<ch0_dst::Ch0DstSpec>;
#[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
pub mod ch0_dst;
#[doc = "CH0_LINK (rw) register accessor: Channel Descriptor Link Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_link`] module"]
#[doc(alias = "CH0_LINK")]
pub type Ch0Link = crate::Reg<ch0_link::Ch0LinkSpec>;
#[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
pub mod ch0_link;
#[doc = "CH0_XCTRL (rw) register accessor: Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_xctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_xctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_xctrl`] module"]
#[doc(alias = "CH0_XCTRL")]
pub type Ch0Xctrl = crate::Reg<ch0_xctrl::Ch0XctrlSpec>;
#[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
pub mod ch0_xctrl;
#[doc = "CH0_ILSRC (rw) register accessor: Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_ilsrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_ilsrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_ilsrc`] module"]
#[doc(alias = "CH0_ILSRC")]
pub type Ch0Ilsrc = crate::Reg<ch0_ilsrc::Ch0IlsrcSpec>;
#[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch0_ilsrc;
#[doc = "CH1_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_cfg`] module"]
#[doc(alias = "CH1_CFG")]
pub type Ch1Cfg = crate::Reg<ch1_cfg::Ch1CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch1_cfg;
#[doc = "CH1_LOOP (rw) register accessor: Channel Loop Counter Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_loop`] module"]
#[doc(alias = "CH1_LOOP")]
pub type Ch1Loop = crate::Reg<ch1_loop::Ch1LoopSpec>;
#[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)"]
pub mod ch1_loop;
#[doc = "CH1_CTRL (rw) register accessor: Channel Descriptor Control Word Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_ctrl`] module"]
#[doc(alias = "CH1_CTRL")]
pub type Ch1Ctrl = crate::Reg<ch1_ctrl::Ch1CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
pub mod ch1_ctrl;
#[doc = "CH1_SRC (rw) register accessor: Channel Descriptor Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_src`] module"]
#[doc(alias = "CH1_SRC")]
pub type Ch1Src = crate::Reg<ch1_src::Ch1SrcSpec>;
#[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch1_src;
#[doc = "CH1_DST (rw) register accessor: Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_dst`] module"]
#[doc(alias = "CH1_DST")]
pub type Ch1Dst = crate::Reg<ch1_dst::Ch1DstSpec>;
#[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
pub mod ch1_dst;
#[doc = "CH1_LINK (rw) register accessor: Channel Descriptor Link Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_link`] module"]
#[doc(alias = "CH1_LINK")]
pub type Ch1Link = crate::Reg<ch1_link::Ch1LinkSpec>;
#[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
pub mod ch1_link;
#[doc = "CH1_XCTRL (rw) register accessor: Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_xctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_xctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_xctrl`] module"]
#[doc(alias = "CH1_XCTRL")]
pub type Ch1Xctrl = crate::Reg<ch1_xctrl::Ch1XctrlSpec>;
#[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
pub mod ch1_xctrl;
#[doc = "CH1_ILSRC (rw) register accessor: Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_ilsrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_ilsrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_ilsrc`] module"]
#[doc(alias = "CH1_ILSRC")]
pub type Ch1Ilsrc = crate::Reg<ch1_ilsrc::Ch1IlsrcSpec>;
#[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch1_ilsrc;
#[doc = "CH2_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_cfg`] module"]
#[doc(alias = "CH2_CFG")]
pub type Ch2Cfg = crate::Reg<ch2_cfg::Ch2CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch2_cfg;
#[doc = "CH2_LOOP (rw) register accessor: Channel Loop Counter Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_loop`] module"]
#[doc(alias = "CH2_LOOP")]
pub type Ch2Loop = crate::Reg<ch2_loop::Ch2LoopSpec>;
#[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)"]
pub mod ch2_loop;
#[doc = "CH2_CTRL (rw) register accessor: Channel Descriptor Control Word Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_ctrl`] module"]
#[doc(alias = "CH2_CTRL")]
pub type Ch2Ctrl = crate::Reg<ch2_ctrl::Ch2CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
pub mod ch2_ctrl;
#[doc = "CH2_SRC (rw) register accessor: Channel Descriptor Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_src`] module"]
#[doc(alias = "CH2_SRC")]
pub type Ch2Src = crate::Reg<ch2_src::Ch2SrcSpec>;
#[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch2_src;
#[doc = "CH2_DST (rw) register accessor: Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_dst`] module"]
#[doc(alias = "CH2_DST")]
pub type Ch2Dst = crate::Reg<ch2_dst::Ch2DstSpec>;
#[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
pub mod ch2_dst;
#[doc = "CH2_LINK (rw) register accessor: Channel Descriptor Link Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_link`] module"]
#[doc(alias = "CH2_LINK")]
pub type Ch2Link = crate::Reg<ch2_link::Ch2LinkSpec>;
#[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
pub mod ch2_link;
#[doc = "CH2_XCTRL (rw) register accessor: Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_xctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_xctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_xctrl`] module"]
#[doc(alias = "CH2_XCTRL")]
pub type Ch2Xctrl = crate::Reg<ch2_xctrl::Ch2XctrlSpec>;
#[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
pub mod ch2_xctrl;
#[doc = "CH2_ILSRC (rw) register accessor: Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_ilsrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_ilsrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_ilsrc`] module"]
#[doc(alias = "CH2_ILSRC")]
pub type Ch2Ilsrc = crate::Reg<ch2_ilsrc::Ch2IlsrcSpec>;
#[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch2_ilsrc;
#[doc = "CH3_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_cfg`] module"]
#[doc(alias = "CH3_CFG")]
pub type Ch3Cfg = crate::Reg<ch3_cfg::Ch3CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch3_cfg;
#[doc = "CH3_LOOP (rw) register accessor: Channel Loop Counter Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_loop`] module"]
#[doc(alias = "CH3_LOOP")]
pub type Ch3Loop = crate::Reg<ch3_loop::Ch3LoopSpec>;
#[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)"]
pub mod ch3_loop;
#[doc = "CH3_CTRL (rw) register accessor: Channel Descriptor Control Word Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_ctrl`] module"]
#[doc(alias = "CH3_CTRL")]
pub type Ch3Ctrl = crate::Reg<ch3_ctrl::Ch3CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
pub mod ch3_ctrl;
#[doc = "CH3_SRC (rw) register accessor: Channel Descriptor Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_src`] module"]
#[doc(alias = "CH3_SRC")]
pub type Ch3Src = crate::Reg<ch3_src::Ch3SrcSpec>;
#[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch3_src;
#[doc = "CH3_DST (rw) register accessor: Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_dst`] module"]
#[doc(alias = "CH3_DST")]
pub type Ch3Dst = crate::Reg<ch3_dst::Ch3DstSpec>;
#[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
pub mod ch3_dst;
#[doc = "CH3_LINK (rw) register accessor: Channel Descriptor Link Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_link`] module"]
#[doc(alias = "CH3_LINK")]
pub type Ch3Link = crate::Reg<ch3_link::Ch3LinkSpec>;
#[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
pub mod ch3_link;
#[doc = "CH3_XCTRL (rw) register accessor: Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_xctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_xctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_xctrl`] module"]
#[doc(alias = "CH3_XCTRL")]
pub type Ch3Xctrl = crate::Reg<ch3_xctrl::Ch3XctrlSpec>;
#[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
pub mod ch3_xctrl;
#[doc = "CH3_ILSRC (rw) register accessor: Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_ilsrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_ilsrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_ilsrc`] module"]
#[doc(alias = "CH3_ILSRC")]
pub type Ch3Ilsrc = crate::Reg<ch3_ilsrc::Ch3IlsrcSpec>;
#[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch3_ilsrc;
#[doc = "CH4_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_cfg`] module"]
#[doc(alias = "CH4_CFG")]
pub type Ch4Cfg = crate::Reg<ch4_cfg::Ch4CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch4_cfg;
#[doc = "CH4_LOOP (rw) register accessor: Channel Loop Counter Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_loop`] module"]
#[doc(alias = "CH4_LOOP")]
pub type Ch4Loop = crate::Reg<ch4_loop::Ch4LoopSpec>;
#[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)"]
pub mod ch4_loop;
#[doc = "CH4_CTRL (rw) register accessor: Channel Descriptor Control Word Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_ctrl`] module"]
#[doc(alias = "CH4_CTRL")]
pub type Ch4Ctrl = crate::Reg<ch4_ctrl::Ch4CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
pub mod ch4_ctrl;
#[doc = "CH4_SRC (rw) register accessor: Channel Descriptor Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_src`] module"]
#[doc(alias = "CH4_SRC")]
pub type Ch4Src = crate::Reg<ch4_src::Ch4SrcSpec>;
#[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch4_src;
#[doc = "CH4_DST (rw) register accessor: Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_dst`] module"]
#[doc(alias = "CH4_DST")]
pub type Ch4Dst = crate::Reg<ch4_dst::Ch4DstSpec>;
#[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
pub mod ch4_dst;
#[doc = "CH4_LINK (rw) register accessor: Channel Descriptor Link Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_link`] module"]
#[doc(alias = "CH4_LINK")]
pub type Ch4Link = crate::Reg<ch4_link::Ch4LinkSpec>;
#[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
pub mod ch4_link;
#[doc = "CH4_XCTRL (rw) register accessor: Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_xctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_xctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_xctrl`] module"]
#[doc(alias = "CH4_XCTRL")]
pub type Ch4Xctrl = crate::Reg<ch4_xctrl::Ch4XctrlSpec>;
#[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
pub mod ch4_xctrl;
#[doc = "CH4_ILSRC (rw) register accessor: Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_ilsrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_ilsrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_ilsrc`] module"]
#[doc(alias = "CH4_ILSRC")]
pub type Ch4Ilsrc = crate::Reg<ch4_ilsrc::Ch4IlsrcSpec>;
#[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch4_ilsrc;
#[doc = "CH5_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_cfg`] module"]
#[doc(alias = "CH5_CFG")]
pub type Ch5Cfg = crate::Reg<ch5_cfg::Ch5CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch5_cfg;
#[doc = "CH5_LOOP (rw) register accessor: Channel Loop Counter Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_loop`] module"]
#[doc(alias = "CH5_LOOP")]
pub type Ch5Loop = crate::Reg<ch5_loop::Ch5LoopSpec>;
#[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)"]
pub mod ch5_loop;
#[doc = "CH5_CTRL (rw) register accessor: Channel Descriptor Control Word Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_ctrl`] module"]
#[doc(alias = "CH5_CTRL")]
pub type Ch5Ctrl = crate::Reg<ch5_ctrl::Ch5CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
pub mod ch5_ctrl;
#[doc = "CH5_SRC (rw) register accessor: Channel Descriptor Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_src`] module"]
#[doc(alias = "CH5_SRC")]
pub type Ch5Src = crate::Reg<ch5_src::Ch5SrcSpec>;
#[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch5_src;
#[doc = "CH5_DST (rw) register accessor: Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_dst`] module"]
#[doc(alias = "CH5_DST")]
pub type Ch5Dst = crate::Reg<ch5_dst::Ch5DstSpec>;
#[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
pub mod ch5_dst;
#[doc = "CH5_LINK (rw) register accessor: Channel Descriptor Link Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_link`] module"]
#[doc(alias = "CH5_LINK")]
pub type Ch5Link = crate::Reg<ch5_link::Ch5LinkSpec>;
#[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
pub mod ch5_link;
#[doc = "CH5_XCTRL (rw) register accessor: Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_xctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_xctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_xctrl`] module"]
#[doc(alias = "CH5_XCTRL")]
pub type Ch5Xctrl = crate::Reg<ch5_xctrl::Ch5XctrlSpec>;
#[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
pub mod ch5_xctrl;
#[doc = "CH5_ILSRC (rw) register accessor: Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_ilsrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_ilsrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_ilsrc`] module"]
#[doc(alias = "CH5_ILSRC")]
pub type Ch5Ilsrc = crate::Reg<ch5_ilsrc::Ch5IlsrcSpec>;
#[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch5_ilsrc;
#[doc = "CH6_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_cfg`] module"]
#[doc(alias = "CH6_CFG")]
pub type Ch6Cfg = crate::Reg<ch6_cfg::Ch6CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch6_cfg;
#[doc = "CH6_LOOP (rw) register accessor: Channel Loop Counter Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_loop`] module"]
#[doc(alias = "CH6_LOOP")]
pub type Ch6Loop = crate::Reg<ch6_loop::Ch6LoopSpec>;
#[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)"]
pub mod ch6_loop;
#[doc = "CH6_CTRL (rw) register accessor: Channel Descriptor Control Word Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_ctrl`] module"]
#[doc(alias = "CH6_CTRL")]
pub type Ch6Ctrl = crate::Reg<ch6_ctrl::Ch6CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
pub mod ch6_ctrl;
#[doc = "CH6_SRC (rw) register accessor: Channel Descriptor Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_src`] module"]
#[doc(alias = "CH6_SRC")]
pub type Ch6Src = crate::Reg<ch6_src::Ch6SrcSpec>;
#[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch6_src;
#[doc = "CH6_DST (rw) register accessor: Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_dst`] module"]
#[doc(alias = "CH6_DST")]
pub type Ch6Dst = crate::Reg<ch6_dst::Ch6DstSpec>;
#[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
pub mod ch6_dst;
#[doc = "CH6_LINK (rw) register accessor: Channel Descriptor Link Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_link`] module"]
#[doc(alias = "CH6_LINK")]
pub type Ch6Link = crate::Reg<ch6_link::Ch6LinkSpec>;
#[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
pub mod ch6_link;
#[doc = "CH6_XCTRL (rw) register accessor: Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_xctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_xctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_xctrl`] module"]
#[doc(alias = "CH6_XCTRL")]
pub type Ch6Xctrl = crate::Reg<ch6_xctrl::Ch6XctrlSpec>;
#[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
pub mod ch6_xctrl;
#[doc = "CH6_ILSRC (rw) register accessor: Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_ilsrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_ilsrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_ilsrc`] module"]
#[doc(alias = "CH6_ILSRC")]
pub type Ch6Ilsrc = crate::Reg<ch6_ilsrc::Ch6IlsrcSpec>;
#[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch6_ilsrc;
#[doc = "CH7_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_cfg`] module"]
#[doc(alias = "CH7_CFG")]
pub type Ch7Cfg = crate::Reg<ch7_cfg::Ch7CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch7_cfg;
#[doc = "CH7_LOOP (rw) register accessor: Channel Loop Counter Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_loop`] module"]
#[doc(alias = "CH7_LOOP")]
pub type Ch7Loop = crate::Reg<ch7_loop::Ch7LoopSpec>;
#[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)"]
pub mod ch7_loop;
#[doc = "CH7_CTRL (rw) register accessor: Channel Descriptor Control Word Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_ctrl`] module"]
#[doc(alias = "CH7_CTRL")]
pub type Ch7Ctrl = crate::Reg<ch7_ctrl::Ch7CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
pub mod ch7_ctrl;
#[doc = "CH7_SRC (rw) register accessor: Channel Descriptor Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_src`] module"]
#[doc(alias = "CH7_SRC")]
pub type Ch7Src = crate::Reg<ch7_src::Ch7SrcSpec>;
#[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch7_src;
#[doc = "CH7_DST (rw) register accessor: Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_dst`] module"]
#[doc(alias = "CH7_DST")]
pub type Ch7Dst = crate::Reg<ch7_dst::Ch7DstSpec>;
#[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
pub mod ch7_dst;
#[doc = "CH7_LINK (rw) register accessor: Channel Descriptor Link Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_link`] module"]
#[doc(alias = "CH7_LINK")]
pub type Ch7Link = crate::Reg<ch7_link::Ch7LinkSpec>;
#[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
pub mod ch7_link;
#[doc = "CH7_XCTRL (rw) register accessor: Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_xctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_xctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_xctrl`] module"]
#[doc(alias = "CH7_XCTRL")]
pub type Ch7Xctrl = crate::Reg<ch7_xctrl::Ch7XctrlSpec>;
#[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
pub mod ch7_xctrl;
#[doc = "CH7_ILSRC (rw) register accessor: Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_ilsrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_ilsrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_ilsrc`] module"]
#[doc(alias = "CH7_ILSRC")]
pub type Ch7Ilsrc = crate::Reg<ch7_ilsrc::Ch7IlsrcSpec>;
#[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch7_ilsrc;
#[doc = "CH8_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_cfg`] module"]
#[doc(alias = "CH8_CFG")]
pub type Ch8Cfg = crate::Reg<ch8_cfg::Ch8CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch8_cfg;
#[doc = "CH8_LOOP (rw) register accessor: Channel Loop Counter Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_loop`] module"]
#[doc(alias = "CH8_LOOP")]
pub type Ch8Loop = crate::Reg<ch8_loop::Ch8LoopSpec>;
#[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)"]
pub mod ch8_loop;
#[doc = "CH8_CTRL (rw) register accessor: Channel Descriptor Control Word Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_ctrl`] module"]
#[doc(alias = "CH8_CTRL")]
pub type Ch8Ctrl = crate::Reg<ch8_ctrl::Ch8CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
pub mod ch8_ctrl;
#[doc = "CH8_SRC (rw) register accessor: Channel Descriptor Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_src`] module"]
#[doc(alias = "CH8_SRC")]
pub type Ch8Src = crate::Reg<ch8_src::Ch8SrcSpec>;
#[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch8_src;
#[doc = "CH8_DST (rw) register accessor: Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_dst`] module"]
#[doc(alias = "CH8_DST")]
pub type Ch8Dst = crate::Reg<ch8_dst::Ch8DstSpec>;
#[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
pub mod ch8_dst;
#[doc = "CH8_LINK (rw) register accessor: Channel Descriptor Link Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_link`] module"]
#[doc(alias = "CH8_LINK")]
pub type Ch8Link = crate::Reg<ch8_link::Ch8LinkSpec>;
#[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
pub mod ch8_link;
#[doc = "CH8_XCTRL (rw) register accessor: Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_xctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_xctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_xctrl`] module"]
#[doc(alias = "CH8_XCTRL")]
pub type Ch8Xctrl = crate::Reg<ch8_xctrl::Ch8XctrlSpec>;
#[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
pub mod ch8_xctrl;
#[doc = "CH8_ILSRC (rw) register accessor: Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_ilsrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_ilsrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_ilsrc`] module"]
#[doc(alias = "CH8_ILSRC")]
pub type Ch8Ilsrc = crate::Reg<ch8_ilsrc::Ch8IlsrcSpec>;
#[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch8_ilsrc;
#[doc = "CH9_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_cfg`] module"]
#[doc(alias = "CH9_CFG")]
pub type Ch9Cfg = crate::Reg<ch9_cfg::Ch9CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch9_cfg;
#[doc = "CH9_LOOP (rw) register accessor: Channel Loop Counter Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_loop`] module"]
#[doc(alias = "CH9_LOOP")]
pub type Ch9Loop = crate::Reg<ch9_loop::Ch9LoopSpec>;
#[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)"]
pub mod ch9_loop;
#[doc = "CH9_CTRL (rw) register accessor: Channel Descriptor Control Word Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_ctrl`] module"]
#[doc(alias = "CH9_CTRL")]
pub type Ch9Ctrl = crate::Reg<ch9_ctrl::Ch9CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
pub mod ch9_ctrl;
#[doc = "CH9_SRC (rw) register accessor: Channel Descriptor Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_src`] module"]
#[doc(alias = "CH9_SRC")]
pub type Ch9Src = crate::Reg<ch9_src::Ch9SrcSpec>;
#[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch9_src;
#[doc = "CH9_DST (rw) register accessor: Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_dst`] module"]
#[doc(alias = "CH9_DST")]
pub type Ch9Dst = crate::Reg<ch9_dst::Ch9DstSpec>;
#[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
pub mod ch9_dst;
#[doc = "CH9_LINK (rw) register accessor: Channel Descriptor Link Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_link`] module"]
#[doc(alias = "CH9_LINK")]
pub type Ch9Link = crate::Reg<ch9_link::Ch9LinkSpec>;
#[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
pub mod ch9_link;
#[doc = "CH9_XCTRL (rw) register accessor: Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_xctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_xctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_xctrl`] module"]
#[doc(alias = "CH9_XCTRL")]
pub type Ch9Xctrl = crate::Reg<ch9_xctrl::Ch9XctrlSpec>;
#[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
pub mod ch9_xctrl;
#[doc = "CH9_ILSRC (rw) register accessor: Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_ilsrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_ilsrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_ilsrc`] module"]
#[doc(alias = "CH9_ILSRC")]
pub type Ch9Ilsrc = crate::Reg<ch9_ilsrc::Ch9IlsrcSpec>;
#[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch9_ilsrc;
#[doc = "CH10_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch10_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_cfg`] module"]
#[doc(alias = "CH10_CFG")]
pub type Ch10Cfg = crate::Reg<ch10_cfg::Ch10CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch10_cfg;
#[doc = "CH10_LOOP (rw) register accessor: Channel Loop Counter Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch10_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_loop`] module"]
#[doc(alias = "CH10_LOOP")]
pub type Ch10Loop = crate::Reg<ch10_loop::Ch10LoopSpec>;
#[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)"]
pub mod ch10_loop;
#[doc = "CH10_CTRL (rw) register accessor: Channel Descriptor Control Word Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch10_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_ctrl`] module"]
#[doc(alias = "CH10_CTRL")]
pub type Ch10Ctrl = crate::Reg<ch10_ctrl::Ch10CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
pub mod ch10_ctrl;
#[doc = "CH10_SRC (rw) register accessor: Channel Descriptor Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch10_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_src`] module"]
#[doc(alias = "CH10_SRC")]
pub type Ch10Src = crate::Reg<ch10_src::Ch10SrcSpec>;
#[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch10_src;
#[doc = "CH10_DST (rw) register accessor: Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch10_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_dst`] module"]
#[doc(alias = "CH10_DST")]
pub type Ch10Dst = crate::Reg<ch10_dst::Ch10DstSpec>;
#[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
pub mod ch10_dst;
#[doc = "CH10_LINK (rw) register accessor: Channel Descriptor Link Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch10_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_link`] module"]
#[doc(alias = "CH10_LINK")]
pub type Ch10Link = crate::Reg<ch10_link::Ch10LinkSpec>;
#[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
pub mod ch10_link;
#[doc = "CH10_XCTRL (rw) register accessor: Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch10_xctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_xctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_xctrl`] module"]
#[doc(alias = "CH10_XCTRL")]
pub type Ch10Xctrl = crate::Reg<ch10_xctrl::Ch10XctrlSpec>;
#[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
pub mod ch10_xctrl;
#[doc = "CH10_ILSRC (rw) register accessor: Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch10_ilsrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_ilsrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_ilsrc`] module"]
#[doc(alias = "CH10_ILSRC")]
pub type Ch10Ilsrc = crate::Reg<ch10_ilsrc::Ch10IlsrcSpec>;
#[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch10_ilsrc;
#[doc = "CH11_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_cfg`] module"]
#[doc(alias = "CH11_CFG")]
pub type Ch11Cfg = crate::Reg<ch11_cfg::Ch11CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch11_cfg;
#[doc = "CH11_LOOP (rw) register accessor: Channel Loop Counter Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_loop`] module"]
#[doc(alias = "CH11_LOOP")]
pub type Ch11Loop = crate::Reg<ch11_loop::Ch11LoopSpec>;
#[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)"]
pub mod ch11_loop;
#[doc = "CH11_CTRL (rw) register accessor: Channel Descriptor Control Word Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_ctrl`] module"]
#[doc(alias = "CH11_CTRL")]
pub type Ch11Ctrl = crate::Reg<ch11_ctrl::Ch11CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
pub mod ch11_ctrl;
#[doc = "CH11_SRC (rw) register accessor: Channel Descriptor Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_src`] module"]
#[doc(alias = "CH11_SRC")]
pub type Ch11Src = crate::Reg<ch11_src::Ch11SrcSpec>;
#[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch11_src;
#[doc = "CH11_DST (rw) register accessor: Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_dst`] module"]
#[doc(alias = "CH11_DST")]
pub type Ch11Dst = crate::Reg<ch11_dst::Ch11DstSpec>;
#[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
pub mod ch11_dst;
#[doc = "CH11_LINK (rw) register accessor: Channel Descriptor Link Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_link`] module"]
#[doc(alias = "CH11_LINK")]
pub type Ch11Link = crate::Reg<ch11_link::Ch11LinkSpec>;
#[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
pub mod ch11_link;
#[doc = "CH11_XCTRL (rw) register accessor: Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_xctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_xctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_xctrl`] module"]
#[doc(alias = "CH11_XCTRL")]
pub type Ch11Xctrl = crate::Reg<ch11_xctrl::Ch11XctrlSpec>;
#[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
pub mod ch11_xctrl;
#[doc = "CH11_ILSRC (rw) register accessor: Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_ilsrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_ilsrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_ilsrc`] module"]
#[doc(alias = "CH11_ILSRC")]
pub type Ch11Ilsrc = crate::Reg<ch11_ilsrc::Ch11IlsrcSpec>;
#[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch11_ilsrc;
#[doc = "CH12_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch12_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_cfg`] module"]
#[doc(alias = "CH12_CFG")]
pub type Ch12Cfg = crate::Reg<ch12_cfg::Ch12CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch12_cfg;
#[doc = "CH12_LOOP (rw) register accessor: Channel Loop Counter Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch12_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_loop`] module"]
#[doc(alias = "CH12_LOOP")]
pub type Ch12Loop = crate::Reg<ch12_loop::Ch12LoopSpec>;
#[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)"]
pub mod ch12_loop;
#[doc = "CH12_CTRL (rw) register accessor: Channel Descriptor Control Word Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch12_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_ctrl`] module"]
#[doc(alias = "CH12_CTRL")]
pub type Ch12Ctrl = crate::Reg<ch12_ctrl::Ch12CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
pub mod ch12_ctrl;
#[doc = "CH12_SRC (rw) register accessor: Channel Descriptor Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch12_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_src`] module"]
#[doc(alias = "CH12_SRC")]
pub type Ch12Src = crate::Reg<ch12_src::Ch12SrcSpec>;
#[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch12_src;
#[doc = "CH12_DST (rw) register accessor: Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch12_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_dst`] module"]
#[doc(alias = "CH12_DST")]
pub type Ch12Dst = crate::Reg<ch12_dst::Ch12DstSpec>;
#[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
pub mod ch12_dst;
#[doc = "CH12_LINK (rw) register accessor: Channel Descriptor Link Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch12_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_link`] module"]
#[doc(alias = "CH12_LINK")]
pub type Ch12Link = crate::Reg<ch12_link::Ch12LinkSpec>;
#[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
pub mod ch12_link;
#[doc = "CH12_XCTRL (rw) register accessor: Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch12_xctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_xctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_xctrl`] module"]
#[doc(alias = "CH12_XCTRL")]
pub type Ch12Xctrl = crate::Reg<ch12_xctrl::Ch12XctrlSpec>;
#[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
pub mod ch12_xctrl;
#[doc = "CH12_ILSRC (rw) register accessor: Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch12_ilsrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_ilsrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_ilsrc`] module"]
#[doc(alias = "CH12_ILSRC")]
pub type Ch12Ilsrc = crate::Reg<ch12_ilsrc::Ch12IlsrcSpec>;
#[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch12_ilsrc;
#[doc = "CH13_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_cfg`] module"]
#[doc(alias = "CH13_CFG")]
pub type Ch13Cfg = crate::Reg<ch13_cfg::Ch13CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch13_cfg;
#[doc = "CH13_LOOP (rw) register accessor: Channel Loop Counter Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_loop`] module"]
#[doc(alias = "CH13_LOOP")]
pub type Ch13Loop = crate::Reg<ch13_loop::Ch13LoopSpec>;
#[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)"]
pub mod ch13_loop;
#[doc = "CH13_CTRL (rw) register accessor: Channel Descriptor Control Word Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_ctrl`] module"]
#[doc(alias = "CH13_CTRL")]
pub type Ch13Ctrl = crate::Reg<ch13_ctrl::Ch13CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
pub mod ch13_ctrl;
#[doc = "CH13_SRC (rw) register accessor: Channel Descriptor Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_src`] module"]
#[doc(alias = "CH13_SRC")]
pub type Ch13Src = crate::Reg<ch13_src::Ch13SrcSpec>;
#[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch13_src;
#[doc = "CH13_DST (rw) register accessor: Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_dst`] module"]
#[doc(alias = "CH13_DST")]
pub type Ch13Dst = crate::Reg<ch13_dst::Ch13DstSpec>;
#[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
pub mod ch13_dst;
#[doc = "CH13_LINK (rw) register accessor: Channel Descriptor Link Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_link`] module"]
#[doc(alias = "CH13_LINK")]
pub type Ch13Link = crate::Reg<ch13_link::Ch13LinkSpec>;
#[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
pub mod ch13_link;
#[doc = "CH13_XCTRL (rw) register accessor: Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_xctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_xctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_xctrl`] module"]
#[doc(alias = "CH13_XCTRL")]
pub type Ch13Xctrl = crate::Reg<ch13_xctrl::Ch13XctrlSpec>;
#[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
pub mod ch13_xctrl;
#[doc = "CH13_ILSRC (rw) register accessor: Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_ilsrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_ilsrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_ilsrc`] module"]
#[doc(alias = "CH13_ILSRC")]
pub type Ch13Ilsrc = crate::Reg<ch13_ilsrc::Ch13IlsrcSpec>;
#[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch13_ilsrc;
#[doc = "CH14_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch14_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_cfg`] module"]
#[doc(alias = "CH14_CFG")]
pub type Ch14Cfg = crate::Reg<ch14_cfg::Ch14CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch14_cfg;
#[doc = "CH14_LOOP (rw) register accessor: Channel Loop Counter Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch14_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_loop`] module"]
#[doc(alias = "CH14_LOOP")]
pub type Ch14Loop = crate::Reg<ch14_loop::Ch14LoopSpec>;
#[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)"]
pub mod ch14_loop;
#[doc = "CH14_CTRL (rw) register accessor: Channel Descriptor Control Word Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch14_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_ctrl`] module"]
#[doc(alias = "CH14_CTRL")]
pub type Ch14Ctrl = crate::Reg<ch14_ctrl::Ch14CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
pub mod ch14_ctrl;
#[doc = "CH14_SRC (rw) register accessor: Channel Descriptor Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch14_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_src`] module"]
#[doc(alias = "CH14_SRC")]
pub type Ch14Src = crate::Reg<ch14_src::Ch14SrcSpec>;
#[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch14_src;
#[doc = "CH14_DST (rw) register accessor: Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch14_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_dst`] module"]
#[doc(alias = "CH14_DST")]
pub type Ch14Dst = crate::Reg<ch14_dst::Ch14DstSpec>;
#[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
pub mod ch14_dst;
#[doc = "CH14_LINK (rw) register accessor: Channel Descriptor Link Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch14_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_link`] module"]
#[doc(alias = "CH14_LINK")]
pub type Ch14Link = crate::Reg<ch14_link::Ch14LinkSpec>;
#[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
pub mod ch14_link;
#[doc = "CH14_XCTRL (rw) register accessor: Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch14_xctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_xctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_xctrl`] module"]
#[doc(alias = "CH14_XCTRL")]
pub type Ch14Xctrl = crate::Reg<ch14_xctrl::Ch14XctrlSpec>;
#[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
pub mod ch14_xctrl;
#[doc = "CH14_ILSRC (rw) register accessor: Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch14_ilsrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_ilsrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_ilsrc`] module"]
#[doc(alias = "CH14_ILSRC")]
pub type Ch14Ilsrc = crate::Reg<ch14_ilsrc::Ch14IlsrcSpec>;
#[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch14_ilsrc;
#[doc = "CH15_CFG (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_cfg`] module"]
#[doc(alias = "CH15_CFG")]
pub type Ch15Cfg = crate::Reg<ch15_cfg::Ch15CfgSpec>;
#[doc = "Channel Configuration Register"]
pub mod ch15_cfg;
#[doc = "CH15_LOOP (rw) register accessor: Channel Loop Counter Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_loop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_loop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_loop`] module"]
#[doc(alias = "CH15_LOOP")]
pub type Ch15Loop = crate::Reg<ch15_loop::Ch15LoopSpec>;
#[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)"]
pub mod ch15_loop;
#[doc = "CH15_CTRL (rw) register accessor: Channel Descriptor Control Word Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_ctrl`] module"]
#[doc(alias = "CH15_CTRL")]
pub type Ch15Ctrl = crate::Reg<ch15_ctrl::Ch15CtrlSpec>;
#[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)"]
pub mod ch15_ctrl;
#[doc = "CH15_SRC (rw) register accessor: Channel Descriptor Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_src`] module"]
#[doc(alias = "CH15_SRC")]
pub type Ch15Src = crate::Reg<ch15_src::Ch15SrcSpec>;
#[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch15_src;
#[doc = "CH15_DST (rw) register accessor: Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_dst`] module"]
#[doc(alias = "CH15_DST")]
pub type Ch15Dst = crate::Reg<ch15_dst::Ch15DstSpec>;
#[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)"]
pub mod ch15_dst;
#[doc = "CH15_LINK (rw) register accessor: Channel Descriptor Link Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_link`] module"]
#[doc(alias = "CH15_LINK")]
pub type Ch15Link = crate::Reg<ch15_link::Ch15LinkSpec>;
#[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)"]
pub mod ch15_link;
#[doc = "CH15_XCTRL (rw) register accessor: Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_xctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_xctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_xctrl`] module"]
#[doc(alias = "CH15_XCTRL")]
pub type Ch15Xctrl = crate::Reg<ch15_xctrl::Ch15XctrlSpec>;
#[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)"]
pub mod ch15_xctrl;
#[doc = "CH15_ILSRC (rw) register accessor: Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_ilsrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_ilsrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_ilsrc`] module"]
#[doc(alias = "CH15_ILSRC")]
pub type Ch15Ilsrc = crate::Reg<ch15_ilsrc::Ch15IlsrcSpec>;
#[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)"]
pub mod ch15_ilsrc;
