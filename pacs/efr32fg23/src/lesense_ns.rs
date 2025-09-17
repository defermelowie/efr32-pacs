#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    en: En,
    swrst: Swrst,
    cfg: Cfg,
    timctrl: Timctrl,
    perctrl: Perctrl,
    decctrl: Decctrl,
    evalctrl: Evalctrl,
    prsctrl: Prsctrl,
    cmd: Cmd,
    chen: Chen,
    scanres: Scanres,
    status: Status,
    rescount: Rescount,
    resfifo: Resfifo,
    curch: Curch,
    decstate: Decstate,
    sensorstate: Sensorstate,
    idleconf: Idleconf,
    _reserved19: [u8; 0x04],
    syncbusy: Syncbusy,
    _reserved20: [u8; 0x0c],
    if_: If,
    ien: Ien,
    _reserved22: [u8; 0x98],
    ch0_timing: Ch0Timing,
    ch0_interact: Ch0Interact,
    ch0_evalcfg: Ch0Evalcfg,
    ch0_evalthres: Ch0Evalthres,
    ch1_timing: Ch1Timing,
    ch1_interact: Ch1Interact,
    ch1_evalcfg: Ch1Evalcfg,
    ch1_evalthres: Ch1Evalthres,
    ch2_timing: Ch2Timing,
    ch2_interact: Ch2Interact,
    ch2_evalcfg: Ch2Evalcfg,
    ch2_evalthres: Ch2Evalthres,
    ch3_timing: Ch3Timing,
    ch3_interact: Ch3Interact,
    ch3_evalcfg: Ch3Evalcfg,
    ch3_evalthres: Ch3Evalthres,
    ch4_timing: Ch4Timing,
    ch4_interact: Ch4Interact,
    ch4_evalcfg: Ch4Evalcfg,
    ch4_evalthres: Ch4Evalthres,
    ch5_timing: Ch5Timing,
    ch5_interact: Ch5Interact,
    ch5_evalcfg: Ch5Evalcfg,
    ch5_evalthres: Ch5Evalthres,
    ch6_timing: Ch6Timing,
    ch6_interact: Ch6Interact,
    ch6_evalcfg: Ch6Evalcfg,
    ch6_evalthres: Ch6Evalthres,
    ch7_timing: Ch7Timing,
    ch7_interact: Ch7Interact,
    ch7_evalcfg: Ch7Evalcfg,
    ch7_evalthres: Ch7Evalthres,
    ch8_timing: Ch8Timing,
    ch8_interact: Ch8Interact,
    ch8_evalcfg: Ch8Evalcfg,
    ch8_evalthres: Ch8Evalthres,
    ch9_timing: Ch9Timing,
    ch9_interact: Ch9Interact,
    ch9_evalcfg: Ch9Evalcfg,
    ch9_evalthres: Ch9Evalthres,
    ch10_timing: Ch10Timing,
    ch10_interact: Ch10Interact,
    ch10_evalcfg: Ch10Evalcfg,
    ch10_evalthres: Ch10Evalthres,
    ch11_timing: Ch11Timing,
    ch11_interact: Ch11Interact,
    ch11_evalcfg: Ch11Evalcfg,
    ch11_evalthres: Ch11Evalthres,
    ch12_timing: Ch12Timing,
    ch12_interact: Ch12Interact,
    ch12_evalcfg: Ch12Evalcfg,
    ch12_evalthres: Ch12Evalthres,
    ch13_timing: Ch13Timing,
    ch13_interact: Ch13Interact,
    ch13_evalcfg: Ch13Evalcfg,
    ch13_evalthres: Ch13Evalthres,
    ch14_timing: Ch14Timing,
    ch14_interact: Ch14Interact,
    ch14_evalcfg: Ch14Evalcfg,
    ch14_evalthres: Ch14Evalthres,
    ch15_timing: Ch15Timing,
    ch15_interact: Ch15Interact,
    ch15_evalcfg: Ch15Evalcfg,
    ch15_evalthres: Ch15Evalthres,
    st0_arc: St0Arc,
    st1_arc: St1Arc,
    st2_arc: St2Arc,
    st3_arc: St3Arc,
    st4_arc: St4Arc,
    st5_arc: St5Arc,
    st6_arc: St6Arc,
    st7_arc: St7Arc,
    st8_arc: St8Arc,
    st9_arc: St9Arc,
    st10_arc: St10Arc,
    st11_arc: St11Arc,
    st12_arc: St12Arc,
    st13_arc: St13Arc,
    st14_arc: St14Arc,
    st15_arc: St15Arc,
    st16_arc: St16Arc,
    st17_arc: St17Arc,
    st18_arc: St18Arc,
    st19_arc: St19Arc,
    st20_arc: St20Arc,
    st21_arc: St21Arc,
    st22_arc: St22Arc,
    st23_arc: St23Arc,
    st24_arc: St24Arc,
    st25_arc: St25Arc,
    st26_arc: St26Arc,
    st27_arc: St27Arc,
    st28_arc: St28Arc,
    st29_arc: St29Arc,
    st30_arc: St30Arc,
    st31_arc: St31Arc,
    st32_arc: St32Arc,
    st33_arc: St33Arc,
    st34_arc: St34Arc,
    st35_arc: St35Arc,
    st36_arc: St36Arc,
    st37_arc: St37Arc,
    st38_arc: St38Arc,
    st39_arc: St39Arc,
    st40_arc: St40Arc,
    st41_arc: St41Arc,
    st42_arc: St42Arc,
    st43_arc: St43Arc,
    st44_arc: St44Arc,
    st45_arc: St45Arc,
    st46_arc: St46Arc,
    st47_arc: St47Arc,
    st48_arc: St48Arc,
    st49_arc: St49Arc,
    st50_arc: St50Arc,
    st51_arc: St51Arc,
    st52_arc: St52Arc,
    st53_arc: St53Arc,
    st54_arc: St54Arc,
    st55_arc: St55Arc,
    st56_arc: St56Arc,
    st57_arc: St57Arc,
    st58_arc: St58Arc,
    st59_arc: St59Arc,
    st60_arc: St60Arc,
    st61_arc: St61Arc,
    st62_arc: St62Arc,
    st63_arc: St63Arc,
}
impl RegisterBlock {
    #[doc = "0x00 - IPVERSION"]
    #[inline(always)]
    pub const fn ipversion(&self) -> &Ipversion {
        &self.ipversion
    }
    #[doc = "0x04 - Global Enable of LESENSE functions"]
    #[inline(always)]
    pub const fn en(&self) -> &En {
        &self.en
    }
    #[doc = "0x08 - No Description"]
    #[inline(always)]
    pub const fn swrst(&self) -> &Swrst {
        &self.swrst
    }
    #[doc = "0x0c - Configuration Register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x10 - Timing Control Register"]
    #[inline(always)]
    pub const fn timctrl(&self) -> &Timctrl {
        &self.timctrl
    }
    #[doc = "0x14 - Peripheral Control Register"]
    #[inline(always)]
    pub const fn perctrl(&self) -> &Perctrl {
        &self.perctrl
    }
    #[doc = "0x18 - Decoder control Register"]
    #[inline(always)]
    pub const fn decctrl(&self) -> &Decctrl {
        &self.decctrl
    }
    #[doc = "0x1c - LESENSE evaluation control"]
    #[inline(always)]
    pub const fn evalctrl(&self) -> &Evalctrl {
        &self.evalctrl
    }
    #[doc = "0x20 - PRS control register"]
    #[inline(always)]
    pub const fn prsctrl(&self) -> &Prsctrl {
        &self.prsctrl
    }
    #[doc = "0x24 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x28 - Channel enable Register"]
    #[inline(always)]
    pub const fn chen(&self) -> &Chen {
        &self.chen
    }
    #[doc = "0x2c - Scan result register"]
    #[inline(always)]
    pub const fn scanres(&self) -> &Scanres {
        &self.scanres
    }
    #[doc = "0x30 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x34 - Result FIFO Count"]
    #[inline(always)]
    pub const fn rescount(&self) -> &Rescount {
        &self.rescount
    }
    #[doc = "0x38 - Result Fifo"]
    #[inline(always)]
    pub const fn resfifo(&self) -> &Resfifo {
        &self.resfifo
    }
    #[doc = "0x3c - Current channel index"]
    #[inline(always)]
    pub const fn curch(&self) -> &Curch {
        &self.curch
    }
    #[doc = "0x40 - Current decoder state"]
    #[inline(always)]
    pub const fn decstate(&self) -> &Decstate {
        &self.decstate
    }
    #[doc = "0x44 - Decoder input register"]
    #[inline(always)]
    pub const fn sensorstate(&self) -> &Sensorstate {
        &self.sensorstate
    }
    #[doc = "0x48 - GPIO Idle phase configuration"]
    #[inline(always)]
    pub const fn idleconf(&self) -> &Idleconf {
        &self.idleconf
    }
    #[doc = "0x50 - Synchronization Busy Register"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x60 - Interrupt Flags"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x64 - Interrupt Enables"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x100 - No Description"]
    #[inline(always)]
    pub const fn ch0_timing(&self) -> &Ch0Timing {
        &self.ch0_timing
    }
    #[doc = "0x104 - No Description"]
    #[inline(always)]
    pub const fn ch0_interact(&self) -> &Ch0Interact {
        &self.ch0_interact
    }
    #[doc = "0x108 - No Description"]
    #[inline(always)]
    pub const fn ch0_evalcfg(&self) -> &Ch0Evalcfg {
        &self.ch0_evalcfg
    }
    #[doc = "0x10c - No Description"]
    #[inline(always)]
    pub const fn ch0_evalthres(&self) -> &Ch0Evalthres {
        &self.ch0_evalthres
    }
    #[doc = "0x110 - No Description"]
    #[inline(always)]
    pub const fn ch1_timing(&self) -> &Ch1Timing {
        &self.ch1_timing
    }
    #[doc = "0x114 - No Description"]
    #[inline(always)]
    pub const fn ch1_interact(&self) -> &Ch1Interact {
        &self.ch1_interact
    }
    #[doc = "0x118 - No Description"]
    #[inline(always)]
    pub const fn ch1_evalcfg(&self) -> &Ch1Evalcfg {
        &self.ch1_evalcfg
    }
    #[doc = "0x11c - No Description"]
    #[inline(always)]
    pub const fn ch1_evalthres(&self) -> &Ch1Evalthres {
        &self.ch1_evalthres
    }
    #[doc = "0x120 - No Description"]
    #[inline(always)]
    pub const fn ch2_timing(&self) -> &Ch2Timing {
        &self.ch2_timing
    }
    #[doc = "0x124 - No Description"]
    #[inline(always)]
    pub const fn ch2_interact(&self) -> &Ch2Interact {
        &self.ch2_interact
    }
    #[doc = "0x128 - No Description"]
    #[inline(always)]
    pub const fn ch2_evalcfg(&self) -> &Ch2Evalcfg {
        &self.ch2_evalcfg
    }
    #[doc = "0x12c - No Description"]
    #[inline(always)]
    pub const fn ch2_evalthres(&self) -> &Ch2Evalthres {
        &self.ch2_evalthres
    }
    #[doc = "0x130 - No Description"]
    #[inline(always)]
    pub const fn ch3_timing(&self) -> &Ch3Timing {
        &self.ch3_timing
    }
    #[doc = "0x134 - No Description"]
    #[inline(always)]
    pub const fn ch3_interact(&self) -> &Ch3Interact {
        &self.ch3_interact
    }
    #[doc = "0x138 - No Description"]
    #[inline(always)]
    pub const fn ch3_evalcfg(&self) -> &Ch3Evalcfg {
        &self.ch3_evalcfg
    }
    #[doc = "0x13c - No Description"]
    #[inline(always)]
    pub const fn ch3_evalthres(&self) -> &Ch3Evalthres {
        &self.ch3_evalthres
    }
    #[doc = "0x140 - No Description"]
    #[inline(always)]
    pub const fn ch4_timing(&self) -> &Ch4Timing {
        &self.ch4_timing
    }
    #[doc = "0x144 - No Description"]
    #[inline(always)]
    pub const fn ch4_interact(&self) -> &Ch4Interact {
        &self.ch4_interact
    }
    #[doc = "0x148 - No Description"]
    #[inline(always)]
    pub const fn ch4_evalcfg(&self) -> &Ch4Evalcfg {
        &self.ch4_evalcfg
    }
    #[doc = "0x14c - No Description"]
    #[inline(always)]
    pub const fn ch4_evalthres(&self) -> &Ch4Evalthres {
        &self.ch4_evalthres
    }
    #[doc = "0x150 - No Description"]
    #[inline(always)]
    pub const fn ch5_timing(&self) -> &Ch5Timing {
        &self.ch5_timing
    }
    #[doc = "0x154 - No Description"]
    #[inline(always)]
    pub const fn ch5_interact(&self) -> &Ch5Interact {
        &self.ch5_interact
    }
    #[doc = "0x158 - No Description"]
    #[inline(always)]
    pub const fn ch5_evalcfg(&self) -> &Ch5Evalcfg {
        &self.ch5_evalcfg
    }
    #[doc = "0x15c - No Description"]
    #[inline(always)]
    pub const fn ch5_evalthres(&self) -> &Ch5Evalthres {
        &self.ch5_evalthres
    }
    #[doc = "0x160 - No Description"]
    #[inline(always)]
    pub const fn ch6_timing(&self) -> &Ch6Timing {
        &self.ch6_timing
    }
    #[doc = "0x164 - No Description"]
    #[inline(always)]
    pub const fn ch6_interact(&self) -> &Ch6Interact {
        &self.ch6_interact
    }
    #[doc = "0x168 - No Description"]
    #[inline(always)]
    pub const fn ch6_evalcfg(&self) -> &Ch6Evalcfg {
        &self.ch6_evalcfg
    }
    #[doc = "0x16c - No Description"]
    #[inline(always)]
    pub const fn ch6_evalthres(&self) -> &Ch6Evalthres {
        &self.ch6_evalthres
    }
    #[doc = "0x170 - No Description"]
    #[inline(always)]
    pub const fn ch7_timing(&self) -> &Ch7Timing {
        &self.ch7_timing
    }
    #[doc = "0x174 - No Description"]
    #[inline(always)]
    pub const fn ch7_interact(&self) -> &Ch7Interact {
        &self.ch7_interact
    }
    #[doc = "0x178 - No Description"]
    #[inline(always)]
    pub const fn ch7_evalcfg(&self) -> &Ch7Evalcfg {
        &self.ch7_evalcfg
    }
    #[doc = "0x17c - No Description"]
    #[inline(always)]
    pub const fn ch7_evalthres(&self) -> &Ch7Evalthres {
        &self.ch7_evalthres
    }
    #[doc = "0x180 - No Description"]
    #[inline(always)]
    pub const fn ch8_timing(&self) -> &Ch8Timing {
        &self.ch8_timing
    }
    #[doc = "0x184 - No Description"]
    #[inline(always)]
    pub const fn ch8_interact(&self) -> &Ch8Interact {
        &self.ch8_interact
    }
    #[doc = "0x188 - No Description"]
    #[inline(always)]
    pub const fn ch8_evalcfg(&self) -> &Ch8Evalcfg {
        &self.ch8_evalcfg
    }
    #[doc = "0x18c - No Description"]
    #[inline(always)]
    pub const fn ch8_evalthres(&self) -> &Ch8Evalthres {
        &self.ch8_evalthres
    }
    #[doc = "0x190 - No Description"]
    #[inline(always)]
    pub const fn ch9_timing(&self) -> &Ch9Timing {
        &self.ch9_timing
    }
    #[doc = "0x194 - No Description"]
    #[inline(always)]
    pub const fn ch9_interact(&self) -> &Ch9Interact {
        &self.ch9_interact
    }
    #[doc = "0x198 - No Description"]
    #[inline(always)]
    pub const fn ch9_evalcfg(&self) -> &Ch9Evalcfg {
        &self.ch9_evalcfg
    }
    #[doc = "0x19c - No Description"]
    #[inline(always)]
    pub const fn ch9_evalthres(&self) -> &Ch9Evalthres {
        &self.ch9_evalthres
    }
    #[doc = "0x1a0 - No Description"]
    #[inline(always)]
    pub const fn ch10_timing(&self) -> &Ch10Timing {
        &self.ch10_timing
    }
    #[doc = "0x1a4 - No Description"]
    #[inline(always)]
    pub const fn ch10_interact(&self) -> &Ch10Interact {
        &self.ch10_interact
    }
    #[doc = "0x1a8 - No Description"]
    #[inline(always)]
    pub const fn ch10_evalcfg(&self) -> &Ch10Evalcfg {
        &self.ch10_evalcfg
    }
    #[doc = "0x1ac - No Description"]
    #[inline(always)]
    pub const fn ch10_evalthres(&self) -> &Ch10Evalthres {
        &self.ch10_evalthres
    }
    #[doc = "0x1b0 - No Description"]
    #[inline(always)]
    pub const fn ch11_timing(&self) -> &Ch11Timing {
        &self.ch11_timing
    }
    #[doc = "0x1b4 - No Description"]
    #[inline(always)]
    pub const fn ch11_interact(&self) -> &Ch11Interact {
        &self.ch11_interact
    }
    #[doc = "0x1b8 - No Description"]
    #[inline(always)]
    pub const fn ch11_evalcfg(&self) -> &Ch11Evalcfg {
        &self.ch11_evalcfg
    }
    #[doc = "0x1bc - No Description"]
    #[inline(always)]
    pub const fn ch11_evalthres(&self) -> &Ch11Evalthres {
        &self.ch11_evalthres
    }
    #[doc = "0x1c0 - No Description"]
    #[inline(always)]
    pub const fn ch12_timing(&self) -> &Ch12Timing {
        &self.ch12_timing
    }
    #[doc = "0x1c4 - No Description"]
    #[inline(always)]
    pub const fn ch12_interact(&self) -> &Ch12Interact {
        &self.ch12_interact
    }
    #[doc = "0x1c8 - No Description"]
    #[inline(always)]
    pub const fn ch12_evalcfg(&self) -> &Ch12Evalcfg {
        &self.ch12_evalcfg
    }
    #[doc = "0x1cc - No Description"]
    #[inline(always)]
    pub const fn ch12_evalthres(&self) -> &Ch12Evalthres {
        &self.ch12_evalthres
    }
    #[doc = "0x1d0 - No Description"]
    #[inline(always)]
    pub const fn ch13_timing(&self) -> &Ch13Timing {
        &self.ch13_timing
    }
    #[doc = "0x1d4 - No Description"]
    #[inline(always)]
    pub const fn ch13_interact(&self) -> &Ch13Interact {
        &self.ch13_interact
    }
    #[doc = "0x1d8 - No Description"]
    #[inline(always)]
    pub const fn ch13_evalcfg(&self) -> &Ch13Evalcfg {
        &self.ch13_evalcfg
    }
    #[doc = "0x1dc - No Description"]
    #[inline(always)]
    pub const fn ch13_evalthres(&self) -> &Ch13Evalthres {
        &self.ch13_evalthres
    }
    #[doc = "0x1e0 - No Description"]
    #[inline(always)]
    pub const fn ch14_timing(&self) -> &Ch14Timing {
        &self.ch14_timing
    }
    #[doc = "0x1e4 - No Description"]
    #[inline(always)]
    pub const fn ch14_interact(&self) -> &Ch14Interact {
        &self.ch14_interact
    }
    #[doc = "0x1e8 - No Description"]
    #[inline(always)]
    pub const fn ch14_evalcfg(&self) -> &Ch14Evalcfg {
        &self.ch14_evalcfg
    }
    #[doc = "0x1ec - No Description"]
    #[inline(always)]
    pub const fn ch14_evalthres(&self) -> &Ch14Evalthres {
        &self.ch14_evalthres
    }
    #[doc = "0x1f0 - No Description"]
    #[inline(always)]
    pub const fn ch15_timing(&self) -> &Ch15Timing {
        &self.ch15_timing
    }
    #[doc = "0x1f4 - No Description"]
    #[inline(always)]
    pub const fn ch15_interact(&self) -> &Ch15Interact {
        &self.ch15_interact
    }
    #[doc = "0x1f8 - No Description"]
    #[inline(always)]
    pub const fn ch15_evalcfg(&self) -> &Ch15Evalcfg {
        &self.ch15_evalcfg
    }
    #[doc = "0x1fc - No Description"]
    #[inline(always)]
    pub const fn ch15_evalthres(&self) -> &Ch15Evalthres {
        &self.ch15_evalthres
    }
    #[doc = "0x200 - No Description"]
    #[inline(always)]
    pub const fn st0_arc(&self) -> &St0Arc {
        &self.st0_arc
    }
    #[doc = "0x204 - No Description"]
    #[inline(always)]
    pub const fn st1_arc(&self) -> &St1Arc {
        &self.st1_arc
    }
    #[doc = "0x208 - No Description"]
    #[inline(always)]
    pub const fn st2_arc(&self) -> &St2Arc {
        &self.st2_arc
    }
    #[doc = "0x20c - No Description"]
    #[inline(always)]
    pub const fn st3_arc(&self) -> &St3Arc {
        &self.st3_arc
    }
    #[doc = "0x210 - No Description"]
    #[inline(always)]
    pub const fn st4_arc(&self) -> &St4Arc {
        &self.st4_arc
    }
    #[doc = "0x214 - No Description"]
    #[inline(always)]
    pub const fn st5_arc(&self) -> &St5Arc {
        &self.st5_arc
    }
    #[doc = "0x218 - No Description"]
    #[inline(always)]
    pub const fn st6_arc(&self) -> &St6Arc {
        &self.st6_arc
    }
    #[doc = "0x21c - No Description"]
    #[inline(always)]
    pub const fn st7_arc(&self) -> &St7Arc {
        &self.st7_arc
    }
    #[doc = "0x220 - No Description"]
    #[inline(always)]
    pub const fn st8_arc(&self) -> &St8Arc {
        &self.st8_arc
    }
    #[doc = "0x224 - No Description"]
    #[inline(always)]
    pub const fn st9_arc(&self) -> &St9Arc {
        &self.st9_arc
    }
    #[doc = "0x228 - No Description"]
    #[inline(always)]
    pub const fn st10_arc(&self) -> &St10Arc {
        &self.st10_arc
    }
    #[doc = "0x22c - No Description"]
    #[inline(always)]
    pub const fn st11_arc(&self) -> &St11Arc {
        &self.st11_arc
    }
    #[doc = "0x230 - No Description"]
    #[inline(always)]
    pub const fn st12_arc(&self) -> &St12Arc {
        &self.st12_arc
    }
    #[doc = "0x234 - No Description"]
    #[inline(always)]
    pub const fn st13_arc(&self) -> &St13Arc {
        &self.st13_arc
    }
    #[doc = "0x238 - No Description"]
    #[inline(always)]
    pub const fn st14_arc(&self) -> &St14Arc {
        &self.st14_arc
    }
    #[doc = "0x23c - No Description"]
    #[inline(always)]
    pub const fn st15_arc(&self) -> &St15Arc {
        &self.st15_arc
    }
    #[doc = "0x240 - No Description"]
    #[inline(always)]
    pub const fn st16_arc(&self) -> &St16Arc {
        &self.st16_arc
    }
    #[doc = "0x244 - No Description"]
    #[inline(always)]
    pub const fn st17_arc(&self) -> &St17Arc {
        &self.st17_arc
    }
    #[doc = "0x248 - No Description"]
    #[inline(always)]
    pub const fn st18_arc(&self) -> &St18Arc {
        &self.st18_arc
    }
    #[doc = "0x24c - No Description"]
    #[inline(always)]
    pub const fn st19_arc(&self) -> &St19Arc {
        &self.st19_arc
    }
    #[doc = "0x250 - No Description"]
    #[inline(always)]
    pub const fn st20_arc(&self) -> &St20Arc {
        &self.st20_arc
    }
    #[doc = "0x254 - No Description"]
    #[inline(always)]
    pub const fn st21_arc(&self) -> &St21Arc {
        &self.st21_arc
    }
    #[doc = "0x258 - No Description"]
    #[inline(always)]
    pub const fn st22_arc(&self) -> &St22Arc {
        &self.st22_arc
    }
    #[doc = "0x25c - No Description"]
    #[inline(always)]
    pub const fn st23_arc(&self) -> &St23Arc {
        &self.st23_arc
    }
    #[doc = "0x260 - No Description"]
    #[inline(always)]
    pub const fn st24_arc(&self) -> &St24Arc {
        &self.st24_arc
    }
    #[doc = "0x264 - No Description"]
    #[inline(always)]
    pub const fn st25_arc(&self) -> &St25Arc {
        &self.st25_arc
    }
    #[doc = "0x268 - No Description"]
    #[inline(always)]
    pub const fn st26_arc(&self) -> &St26Arc {
        &self.st26_arc
    }
    #[doc = "0x26c - No Description"]
    #[inline(always)]
    pub const fn st27_arc(&self) -> &St27Arc {
        &self.st27_arc
    }
    #[doc = "0x270 - No Description"]
    #[inline(always)]
    pub const fn st28_arc(&self) -> &St28Arc {
        &self.st28_arc
    }
    #[doc = "0x274 - No Description"]
    #[inline(always)]
    pub const fn st29_arc(&self) -> &St29Arc {
        &self.st29_arc
    }
    #[doc = "0x278 - No Description"]
    #[inline(always)]
    pub const fn st30_arc(&self) -> &St30Arc {
        &self.st30_arc
    }
    #[doc = "0x27c - No Description"]
    #[inline(always)]
    pub const fn st31_arc(&self) -> &St31Arc {
        &self.st31_arc
    }
    #[doc = "0x280 - No Description"]
    #[inline(always)]
    pub const fn st32_arc(&self) -> &St32Arc {
        &self.st32_arc
    }
    #[doc = "0x284 - No Description"]
    #[inline(always)]
    pub const fn st33_arc(&self) -> &St33Arc {
        &self.st33_arc
    }
    #[doc = "0x288 - No Description"]
    #[inline(always)]
    pub const fn st34_arc(&self) -> &St34Arc {
        &self.st34_arc
    }
    #[doc = "0x28c - No Description"]
    #[inline(always)]
    pub const fn st35_arc(&self) -> &St35Arc {
        &self.st35_arc
    }
    #[doc = "0x290 - No Description"]
    #[inline(always)]
    pub const fn st36_arc(&self) -> &St36Arc {
        &self.st36_arc
    }
    #[doc = "0x294 - No Description"]
    #[inline(always)]
    pub const fn st37_arc(&self) -> &St37Arc {
        &self.st37_arc
    }
    #[doc = "0x298 - No Description"]
    #[inline(always)]
    pub const fn st38_arc(&self) -> &St38Arc {
        &self.st38_arc
    }
    #[doc = "0x29c - No Description"]
    #[inline(always)]
    pub const fn st39_arc(&self) -> &St39Arc {
        &self.st39_arc
    }
    #[doc = "0x2a0 - No Description"]
    #[inline(always)]
    pub const fn st40_arc(&self) -> &St40Arc {
        &self.st40_arc
    }
    #[doc = "0x2a4 - No Description"]
    #[inline(always)]
    pub const fn st41_arc(&self) -> &St41Arc {
        &self.st41_arc
    }
    #[doc = "0x2a8 - No Description"]
    #[inline(always)]
    pub const fn st42_arc(&self) -> &St42Arc {
        &self.st42_arc
    }
    #[doc = "0x2ac - No Description"]
    #[inline(always)]
    pub const fn st43_arc(&self) -> &St43Arc {
        &self.st43_arc
    }
    #[doc = "0x2b0 - No Description"]
    #[inline(always)]
    pub const fn st44_arc(&self) -> &St44Arc {
        &self.st44_arc
    }
    #[doc = "0x2b4 - No Description"]
    #[inline(always)]
    pub const fn st45_arc(&self) -> &St45Arc {
        &self.st45_arc
    }
    #[doc = "0x2b8 - No Description"]
    #[inline(always)]
    pub const fn st46_arc(&self) -> &St46Arc {
        &self.st46_arc
    }
    #[doc = "0x2bc - No Description"]
    #[inline(always)]
    pub const fn st47_arc(&self) -> &St47Arc {
        &self.st47_arc
    }
    #[doc = "0x2c0 - No Description"]
    #[inline(always)]
    pub const fn st48_arc(&self) -> &St48Arc {
        &self.st48_arc
    }
    #[doc = "0x2c4 - No Description"]
    #[inline(always)]
    pub const fn st49_arc(&self) -> &St49Arc {
        &self.st49_arc
    }
    #[doc = "0x2c8 - No Description"]
    #[inline(always)]
    pub const fn st50_arc(&self) -> &St50Arc {
        &self.st50_arc
    }
    #[doc = "0x2cc - No Description"]
    #[inline(always)]
    pub const fn st51_arc(&self) -> &St51Arc {
        &self.st51_arc
    }
    #[doc = "0x2d0 - No Description"]
    #[inline(always)]
    pub const fn st52_arc(&self) -> &St52Arc {
        &self.st52_arc
    }
    #[doc = "0x2d4 - No Description"]
    #[inline(always)]
    pub const fn st53_arc(&self) -> &St53Arc {
        &self.st53_arc
    }
    #[doc = "0x2d8 - No Description"]
    #[inline(always)]
    pub const fn st54_arc(&self) -> &St54Arc {
        &self.st54_arc
    }
    #[doc = "0x2dc - No Description"]
    #[inline(always)]
    pub const fn st55_arc(&self) -> &St55Arc {
        &self.st55_arc
    }
    #[doc = "0x2e0 - No Description"]
    #[inline(always)]
    pub const fn st56_arc(&self) -> &St56Arc {
        &self.st56_arc
    }
    #[doc = "0x2e4 - No Description"]
    #[inline(always)]
    pub const fn st57_arc(&self) -> &St57Arc {
        &self.st57_arc
    }
    #[doc = "0x2e8 - No Description"]
    #[inline(always)]
    pub const fn st58_arc(&self) -> &St58Arc {
        &self.st58_arc
    }
    #[doc = "0x2ec - No Description"]
    #[inline(always)]
    pub const fn st59_arc(&self) -> &St59Arc {
        &self.st59_arc
    }
    #[doc = "0x2f0 - No Description"]
    #[inline(always)]
    pub const fn st60_arc(&self) -> &St60Arc {
        &self.st60_arc
    }
    #[doc = "0x2f4 - No Description"]
    #[inline(always)]
    pub const fn st61_arc(&self) -> &St61Arc {
        &self.st61_arc
    }
    #[doc = "0x2f8 - No Description"]
    #[inline(always)]
    pub const fn st62_arc(&self) -> &St62Arc {
        &self.st62_arc
    }
    #[doc = "0x2fc - No Description"]
    #[inline(always)]
    pub const fn st63_arc(&self) -> &St63Arc {
        &self.st63_arc
    }
}
#[doc = "IPVERSION (r) register accessor: IPVERSION\n\nYou can [`read`](crate::Reg::read) this register and get [`ipversion::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipversion`] module"]
#[doc(alias = "IPVERSION")]
pub type Ipversion = crate::Reg<ipversion::IpversionSpec>;
#[doc = "IPVERSION"]
pub mod ipversion;
#[doc = "EN (rw) register accessor: Global Enable of LESENSE functions\n\nYou can [`read`](crate::Reg::read) this register and get [`en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en`] module"]
#[doc(alias = "EN")]
pub type En = crate::Reg<en::EnSpec>;
#[doc = "Global Enable of LESENSE functions"]
pub mod en;
#[doc = "SWRST (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrst`] module"]
#[doc(alias = "SWRST")]
pub type Swrst = crate::Reg<swrst::SwrstSpec>;
#[doc = "No Description"]
pub mod swrst;
#[doc = "CFG (rw) register accessor: Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "TIMCTRL (rw) register accessor: Timing Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timctrl`] module"]
#[doc(alias = "TIMCTRL")]
pub type Timctrl = crate::Reg<timctrl::TimctrlSpec>;
#[doc = "Timing Control Register"]
pub mod timctrl;
#[doc = "PERCTRL (rw) register accessor: Peripheral Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`perctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perctrl`] module"]
#[doc(alias = "PERCTRL")]
pub type Perctrl = crate::Reg<perctrl::PerctrlSpec>;
#[doc = "Peripheral Control Register"]
pub mod perctrl;
#[doc = "DECCTRL (rw) register accessor: Decoder control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`decctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`decctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@decctrl`] module"]
#[doc(alias = "DECCTRL")]
pub type Decctrl = crate::Reg<decctrl::DecctrlSpec>;
#[doc = "Decoder control Register"]
pub mod decctrl;
#[doc = "EVALCTRL (rw) register accessor: LESENSE evaluation control\n\nYou can [`read`](crate::Reg::read) this register and get [`evalctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evalctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evalctrl`] module"]
#[doc(alias = "EVALCTRL")]
pub type Evalctrl = crate::Reg<evalctrl::EvalctrlSpec>;
#[doc = "LESENSE evaluation control"]
pub mod evalctrl;
#[doc = "PRSCTRL (rw) register accessor: PRS control register\n\nYou can [`read`](crate::Reg::read) this register and get [`prsctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prsctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prsctrl`] module"]
#[doc(alias = "PRSCTRL")]
pub type Prsctrl = crate::Reg<prsctrl::PrsctrlSpec>;
#[doc = "PRS control register"]
pub mod prsctrl;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "CHEN (rw) register accessor: Channel enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chen`] module"]
#[doc(alias = "CHEN")]
pub type Chen = crate::Reg<chen::ChenSpec>;
#[doc = "Channel enable Register"]
pub mod chen;
#[doc = "SCANRES (r) register accessor: Scan result register\n\nYou can [`read`](crate::Reg::read) this register and get [`scanres::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scanres`] module"]
#[doc(alias = "SCANRES")]
pub type Scanres = crate::Reg<scanres::ScanresSpec>;
#[doc = "Scan result register"]
pub mod scanres;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "RESCOUNT (r) register accessor: Result FIFO Count\n\nYou can [`read`](crate::Reg::read) this register and get [`rescount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rescount`] module"]
#[doc(alias = "RESCOUNT")]
pub type Rescount = crate::Reg<rescount::RescountSpec>;
#[doc = "Result FIFO Count"]
pub mod rescount;
#[doc = "RESFIFO (r) register accessor: Result Fifo\n\nYou can [`read`](crate::Reg::read) this register and get [`resfifo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resfifo`] module"]
#[doc(alias = "RESFIFO")]
pub type Resfifo = crate::Reg<resfifo::ResfifoSpec>;
#[doc = "Result Fifo"]
pub mod resfifo;
#[doc = "CURCH (r) register accessor: Current channel index\n\nYou can [`read`](crate::Reg::read) this register and get [`curch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@curch`] module"]
#[doc(alias = "CURCH")]
pub type Curch = crate::Reg<curch::CurchSpec>;
#[doc = "Current channel index"]
pub mod curch;
#[doc = "DECSTATE (r) register accessor: Current decoder state\n\nYou can [`read`](crate::Reg::read) this register and get [`decstate::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@decstate`] module"]
#[doc(alias = "DECSTATE")]
pub type Decstate = crate::Reg<decstate::DecstateSpec>;
#[doc = "Current decoder state"]
pub mod decstate;
#[doc = "SENSORSTATE (r) register accessor: Decoder input register\n\nYou can [`read`](crate::Reg::read) this register and get [`sensorstate::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sensorstate`] module"]
#[doc(alias = "SENSORSTATE")]
pub type Sensorstate = crate::Reg<sensorstate::SensorstateSpec>;
#[doc = "Decoder input register"]
pub mod sensorstate;
#[doc = "IDLECONF (rw) register accessor: GPIO Idle phase configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`idleconf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idleconf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idleconf`] module"]
#[doc(alias = "IDLECONF")]
pub type Idleconf = crate::Reg<idleconf::IdleconfSpec>;
#[doc = "GPIO Idle phase configuration"]
pub mod idleconf;
#[doc = "SYNCBUSY (r) register accessor: Synchronization Busy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "IF (rw) register accessor: Interrupt Flags\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`] module"]
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IfSpec>;
#[doc = "Interrupt Flags"]
pub mod if_;
#[doc = "IEN (rw) register accessor: Interrupt Enables\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`] module"]
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IenSpec>;
#[doc = "Interrupt Enables"]
pub mod ien;
#[doc = "CH0_TIMING (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_timing`] module"]
#[doc(alias = "CH0_TIMING")]
pub type Ch0Timing = crate::Reg<ch0_timing::Ch0TimingSpec>;
#[doc = "No Description"]
pub mod ch0_timing;
#[doc = "CH0_INTERACT (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_interact`] module"]
#[doc(alias = "CH0_INTERACT")]
pub type Ch0Interact = crate::Reg<ch0_interact::Ch0InteractSpec>;
#[doc = "No Description"]
pub mod ch0_interact;
#[doc = "CH0_EVALCFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_evalcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_evalcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_evalcfg`] module"]
#[doc(alias = "CH0_EVALCFG")]
pub type Ch0Evalcfg = crate::Reg<ch0_evalcfg::Ch0EvalcfgSpec>;
#[doc = "No Description"]
pub mod ch0_evalcfg;
#[doc = "CH0_EVALTHRES (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_evalthres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_evalthres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_evalthres`] module"]
#[doc(alias = "CH0_EVALTHRES")]
pub type Ch0Evalthres = crate::Reg<ch0_evalthres::Ch0EvalthresSpec>;
#[doc = "No Description"]
pub mod ch0_evalthres;
#[doc = "CH1_TIMING (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_timing`] module"]
#[doc(alias = "CH1_TIMING")]
pub type Ch1Timing = crate::Reg<ch1_timing::Ch1TimingSpec>;
#[doc = "No Description"]
pub mod ch1_timing;
#[doc = "CH1_INTERACT (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_interact`] module"]
#[doc(alias = "CH1_INTERACT")]
pub type Ch1Interact = crate::Reg<ch1_interact::Ch1InteractSpec>;
#[doc = "No Description"]
pub mod ch1_interact;
#[doc = "CH1_EVALCFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_evalcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_evalcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_evalcfg`] module"]
#[doc(alias = "CH1_EVALCFG")]
pub type Ch1Evalcfg = crate::Reg<ch1_evalcfg::Ch1EvalcfgSpec>;
#[doc = "No Description"]
pub mod ch1_evalcfg;
#[doc = "CH1_EVALTHRES (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_evalthres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_evalthres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_evalthres`] module"]
#[doc(alias = "CH1_EVALTHRES")]
pub type Ch1Evalthres = crate::Reg<ch1_evalthres::Ch1EvalthresSpec>;
#[doc = "No Description"]
pub mod ch1_evalthres;
#[doc = "CH2_TIMING (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_timing`] module"]
#[doc(alias = "CH2_TIMING")]
pub type Ch2Timing = crate::Reg<ch2_timing::Ch2TimingSpec>;
#[doc = "No Description"]
pub mod ch2_timing;
#[doc = "CH2_INTERACT (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_interact`] module"]
#[doc(alias = "CH2_INTERACT")]
pub type Ch2Interact = crate::Reg<ch2_interact::Ch2InteractSpec>;
#[doc = "No Description"]
pub mod ch2_interact;
#[doc = "CH2_EVALCFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_evalcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_evalcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_evalcfg`] module"]
#[doc(alias = "CH2_EVALCFG")]
pub type Ch2Evalcfg = crate::Reg<ch2_evalcfg::Ch2EvalcfgSpec>;
#[doc = "No Description"]
pub mod ch2_evalcfg;
#[doc = "CH2_EVALTHRES (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_evalthres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_evalthres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_evalthres`] module"]
#[doc(alias = "CH2_EVALTHRES")]
pub type Ch2Evalthres = crate::Reg<ch2_evalthres::Ch2EvalthresSpec>;
#[doc = "No Description"]
pub mod ch2_evalthres;
#[doc = "CH3_TIMING (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_timing`] module"]
#[doc(alias = "CH3_TIMING")]
pub type Ch3Timing = crate::Reg<ch3_timing::Ch3TimingSpec>;
#[doc = "No Description"]
pub mod ch3_timing;
#[doc = "CH3_INTERACT (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_interact`] module"]
#[doc(alias = "CH3_INTERACT")]
pub type Ch3Interact = crate::Reg<ch3_interact::Ch3InteractSpec>;
#[doc = "No Description"]
pub mod ch3_interact;
#[doc = "CH3_EVALCFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_evalcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_evalcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_evalcfg`] module"]
#[doc(alias = "CH3_EVALCFG")]
pub type Ch3Evalcfg = crate::Reg<ch3_evalcfg::Ch3EvalcfgSpec>;
#[doc = "No Description"]
pub mod ch3_evalcfg;
#[doc = "CH3_EVALTHRES (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_evalthres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_evalthres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_evalthres`] module"]
#[doc(alias = "CH3_EVALTHRES")]
pub type Ch3Evalthres = crate::Reg<ch3_evalthres::Ch3EvalthresSpec>;
#[doc = "No Description"]
pub mod ch3_evalthres;
#[doc = "CH4_TIMING (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_timing`] module"]
#[doc(alias = "CH4_TIMING")]
pub type Ch4Timing = crate::Reg<ch4_timing::Ch4TimingSpec>;
#[doc = "No Description"]
pub mod ch4_timing;
#[doc = "CH4_INTERACT (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_interact`] module"]
#[doc(alias = "CH4_INTERACT")]
pub type Ch4Interact = crate::Reg<ch4_interact::Ch4InteractSpec>;
#[doc = "No Description"]
pub mod ch4_interact;
#[doc = "CH4_EVALCFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_evalcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_evalcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_evalcfg`] module"]
#[doc(alias = "CH4_EVALCFG")]
pub type Ch4Evalcfg = crate::Reg<ch4_evalcfg::Ch4EvalcfgSpec>;
#[doc = "No Description"]
pub mod ch4_evalcfg;
#[doc = "CH4_EVALTHRES (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_evalthres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_evalthres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_evalthres`] module"]
#[doc(alias = "CH4_EVALTHRES")]
pub type Ch4Evalthres = crate::Reg<ch4_evalthres::Ch4EvalthresSpec>;
#[doc = "No Description"]
pub mod ch4_evalthres;
#[doc = "CH5_TIMING (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_timing`] module"]
#[doc(alias = "CH5_TIMING")]
pub type Ch5Timing = crate::Reg<ch5_timing::Ch5TimingSpec>;
#[doc = "No Description"]
pub mod ch5_timing;
#[doc = "CH5_INTERACT (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_interact`] module"]
#[doc(alias = "CH5_INTERACT")]
pub type Ch5Interact = crate::Reg<ch5_interact::Ch5InteractSpec>;
#[doc = "No Description"]
pub mod ch5_interact;
#[doc = "CH5_EVALCFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_evalcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_evalcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_evalcfg`] module"]
#[doc(alias = "CH5_EVALCFG")]
pub type Ch5Evalcfg = crate::Reg<ch5_evalcfg::Ch5EvalcfgSpec>;
#[doc = "No Description"]
pub mod ch5_evalcfg;
#[doc = "CH5_EVALTHRES (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_evalthres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_evalthres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_evalthres`] module"]
#[doc(alias = "CH5_EVALTHRES")]
pub type Ch5Evalthres = crate::Reg<ch5_evalthres::Ch5EvalthresSpec>;
#[doc = "No Description"]
pub mod ch5_evalthres;
#[doc = "CH6_TIMING (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_timing`] module"]
#[doc(alias = "CH6_TIMING")]
pub type Ch6Timing = crate::Reg<ch6_timing::Ch6TimingSpec>;
#[doc = "No Description"]
pub mod ch6_timing;
#[doc = "CH6_INTERACT (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_interact`] module"]
#[doc(alias = "CH6_INTERACT")]
pub type Ch6Interact = crate::Reg<ch6_interact::Ch6InteractSpec>;
#[doc = "No Description"]
pub mod ch6_interact;
#[doc = "CH6_EVALCFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_evalcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_evalcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_evalcfg`] module"]
#[doc(alias = "CH6_EVALCFG")]
pub type Ch6Evalcfg = crate::Reg<ch6_evalcfg::Ch6EvalcfgSpec>;
#[doc = "No Description"]
pub mod ch6_evalcfg;
#[doc = "CH6_EVALTHRES (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_evalthres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_evalthres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_evalthres`] module"]
#[doc(alias = "CH6_EVALTHRES")]
pub type Ch6Evalthres = crate::Reg<ch6_evalthres::Ch6EvalthresSpec>;
#[doc = "No Description"]
pub mod ch6_evalthres;
#[doc = "CH7_TIMING (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_timing`] module"]
#[doc(alias = "CH7_TIMING")]
pub type Ch7Timing = crate::Reg<ch7_timing::Ch7TimingSpec>;
#[doc = "No Description"]
pub mod ch7_timing;
#[doc = "CH7_INTERACT (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_interact`] module"]
#[doc(alias = "CH7_INTERACT")]
pub type Ch7Interact = crate::Reg<ch7_interact::Ch7InteractSpec>;
#[doc = "No Description"]
pub mod ch7_interact;
#[doc = "CH7_EVALCFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_evalcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_evalcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_evalcfg`] module"]
#[doc(alias = "CH7_EVALCFG")]
pub type Ch7Evalcfg = crate::Reg<ch7_evalcfg::Ch7EvalcfgSpec>;
#[doc = "No Description"]
pub mod ch7_evalcfg;
#[doc = "CH7_EVALTHRES (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_evalthres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_evalthres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_evalthres`] module"]
#[doc(alias = "CH7_EVALTHRES")]
pub type Ch7Evalthres = crate::Reg<ch7_evalthres::Ch7EvalthresSpec>;
#[doc = "No Description"]
pub mod ch7_evalthres;
#[doc = "CH8_TIMING (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_timing`] module"]
#[doc(alias = "CH8_TIMING")]
pub type Ch8Timing = crate::Reg<ch8_timing::Ch8TimingSpec>;
#[doc = "No Description"]
pub mod ch8_timing;
#[doc = "CH8_INTERACT (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_interact`] module"]
#[doc(alias = "CH8_INTERACT")]
pub type Ch8Interact = crate::Reg<ch8_interact::Ch8InteractSpec>;
#[doc = "No Description"]
pub mod ch8_interact;
#[doc = "CH8_EVALCFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_evalcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_evalcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_evalcfg`] module"]
#[doc(alias = "CH8_EVALCFG")]
pub type Ch8Evalcfg = crate::Reg<ch8_evalcfg::Ch8EvalcfgSpec>;
#[doc = "No Description"]
pub mod ch8_evalcfg;
#[doc = "CH8_EVALTHRES (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_evalthres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_evalthres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8_evalthres`] module"]
#[doc(alias = "CH8_EVALTHRES")]
pub type Ch8Evalthres = crate::Reg<ch8_evalthres::Ch8EvalthresSpec>;
#[doc = "No Description"]
pub mod ch8_evalthres;
#[doc = "CH9_TIMING (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_timing`] module"]
#[doc(alias = "CH9_TIMING")]
pub type Ch9Timing = crate::Reg<ch9_timing::Ch9TimingSpec>;
#[doc = "No Description"]
pub mod ch9_timing;
#[doc = "CH9_INTERACT (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_interact`] module"]
#[doc(alias = "CH9_INTERACT")]
pub type Ch9Interact = crate::Reg<ch9_interact::Ch9InteractSpec>;
#[doc = "No Description"]
pub mod ch9_interact;
#[doc = "CH9_EVALCFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_evalcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_evalcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_evalcfg`] module"]
#[doc(alias = "CH9_EVALCFG")]
pub type Ch9Evalcfg = crate::Reg<ch9_evalcfg::Ch9EvalcfgSpec>;
#[doc = "No Description"]
pub mod ch9_evalcfg;
#[doc = "CH9_EVALTHRES (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_evalthres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_evalthres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9_evalthres`] module"]
#[doc(alias = "CH9_EVALTHRES")]
pub type Ch9Evalthres = crate::Reg<ch9_evalthres::Ch9EvalthresSpec>;
#[doc = "No Description"]
pub mod ch9_evalthres;
#[doc = "CH10_TIMING (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch10_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_timing`] module"]
#[doc(alias = "CH10_TIMING")]
pub type Ch10Timing = crate::Reg<ch10_timing::Ch10TimingSpec>;
#[doc = "No Description"]
pub mod ch10_timing;
#[doc = "CH10_INTERACT (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch10_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_interact`] module"]
#[doc(alias = "CH10_INTERACT")]
pub type Ch10Interact = crate::Reg<ch10_interact::Ch10InteractSpec>;
#[doc = "No Description"]
pub mod ch10_interact;
#[doc = "CH10_EVALCFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch10_evalcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_evalcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_evalcfg`] module"]
#[doc(alias = "CH10_EVALCFG")]
pub type Ch10Evalcfg = crate::Reg<ch10_evalcfg::Ch10EvalcfgSpec>;
#[doc = "No Description"]
pub mod ch10_evalcfg;
#[doc = "CH10_EVALTHRES (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch10_evalthres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch10_evalthres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10_evalthres`] module"]
#[doc(alias = "CH10_EVALTHRES")]
pub type Ch10Evalthres = crate::Reg<ch10_evalthres::Ch10EvalthresSpec>;
#[doc = "No Description"]
pub mod ch10_evalthres;
#[doc = "CH11_TIMING (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_timing`] module"]
#[doc(alias = "CH11_TIMING")]
pub type Ch11Timing = crate::Reg<ch11_timing::Ch11TimingSpec>;
#[doc = "No Description"]
pub mod ch11_timing;
#[doc = "CH11_INTERACT (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_interact`] module"]
#[doc(alias = "CH11_INTERACT")]
pub type Ch11Interact = crate::Reg<ch11_interact::Ch11InteractSpec>;
#[doc = "No Description"]
pub mod ch11_interact;
#[doc = "CH11_EVALCFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_evalcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_evalcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_evalcfg`] module"]
#[doc(alias = "CH11_EVALCFG")]
pub type Ch11Evalcfg = crate::Reg<ch11_evalcfg::Ch11EvalcfgSpec>;
#[doc = "No Description"]
pub mod ch11_evalcfg;
#[doc = "CH11_EVALTHRES (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_evalthres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_evalthres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11_evalthres`] module"]
#[doc(alias = "CH11_EVALTHRES")]
pub type Ch11Evalthres = crate::Reg<ch11_evalthres::Ch11EvalthresSpec>;
#[doc = "No Description"]
pub mod ch11_evalthres;
#[doc = "CH12_TIMING (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch12_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_timing`] module"]
#[doc(alias = "CH12_TIMING")]
pub type Ch12Timing = crate::Reg<ch12_timing::Ch12TimingSpec>;
#[doc = "No Description"]
pub mod ch12_timing;
#[doc = "CH12_INTERACT (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch12_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_interact`] module"]
#[doc(alias = "CH12_INTERACT")]
pub type Ch12Interact = crate::Reg<ch12_interact::Ch12InteractSpec>;
#[doc = "No Description"]
pub mod ch12_interact;
#[doc = "CH12_EVALCFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch12_evalcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_evalcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_evalcfg`] module"]
#[doc(alias = "CH12_EVALCFG")]
pub type Ch12Evalcfg = crate::Reg<ch12_evalcfg::Ch12EvalcfgSpec>;
#[doc = "No Description"]
pub mod ch12_evalcfg;
#[doc = "CH12_EVALTHRES (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch12_evalthres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_evalthres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch12_evalthres`] module"]
#[doc(alias = "CH12_EVALTHRES")]
pub type Ch12Evalthres = crate::Reg<ch12_evalthres::Ch12EvalthresSpec>;
#[doc = "No Description"]
pub mod ch12_evalthres;
#[doc = "CH13_TIMING (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_timing`] module"]
#[doc(alias = "CH13_TIMING")]
pub type Ch13Timing = crate::Reg<ch13_timing::Ch13TimingSpec>;
#[doc = "No Description"]
pub mod ch13_timing;
#[doc = "CH13_INTERACT (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_interact`] module"]
#[doc(alias = "CH13_INTERACT")]
pub type Ch13Interact = crate::Reg<ch13_interact::Ch13InteractSpec>;
#[doc = "No Description"]
pub mod ch13_interact;
#[doc = "CH13_EVALCFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_evalcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_evalcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_evalcfg`] module"]
#[doc(alias = "CH13_EVALCFG")]
pub type Ch13Evalcfg = crate::Reg<ch13_evalcfg::Ch13EvalcfgSpec>;
#[doc = "No Description"]
pub mod ch13_evalcfg;
#[doc = "CH13_EVALTHRES (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_evalthres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_evalthres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch13_evalthres`] module"]
#[doc(alias = "CH13_EVALTHRES")]
pub type Ch13Evalthres = crate::Reg<ch13_evalthres::Ch13EvalthresSpec>;
#[doc = "No Description"]
pub mod ch13_evalthres;
#[doc = "CH14_TIMING (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch14_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_timing`] module"]
#[doc(alias = "CH14_TIMING")]
pub type Ch14Timing = crate::Reg<ch14_timing::Ch14TimingSpec>;
#[doc = "No Description"]
pub mod ch14_timing;
#[doc = "CH14_INTERACT (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch14_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_interact`] module"]
#[doc(alias = "CH14_INTERACT")]
pub type Ch14Interact = crate::Reg<ch14_interact::Ch14InteractSpec>;
#[doc = "No Description"]
pub mod ch14_interact;
#[doc = "CH14_EVALCFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch14_evalcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_evalcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_evalcfg`] module"]
#[doc(alias = "CH14_EVALCFG")]
pub type Ch14Evalcfg = crate::Reg<ch14_evalcfg::Ch14EvalcfgSpec>;
#[doc = "No Description"]
pub mod ch14_evalcfg;
#[doc = "CH14_EVALTHRES (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch14_evalthres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch14_evalthres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14_evalthres`] module"]
#[doc(alias = "CH14_EVALTHRES")]
pub type Ch14Evalthres = crate::Reg<ch14_evalthres::Ch14EvalthresSpec>;
#[doc = "No Description"]
pub mod ch14_evalthres;
#[doc = "CH15_TIMING (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_timing`] module"]
#[doc(alias = "CH15_TIMING")]
pub type Ch15Timing = crate::Reg<ch15_timing::Ch15TimingSpec>;
#[doc = "No Description"]
pub mod ch15_timing;
#[doc = "CH15_INTERACT (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_interact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_interact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_interact`] module"]
#[doc(alias = "CH15_INTERACT")]
pub type Ch15Interact = crate::Reg<ch15_interact::Ch15InteractSpec>;
#[doc = "No Description"]
pub mod ch15_interact;
#[doc = "CH15_EVALCFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_evalcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_evalcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_evalcfg`] module"]
#[doc(alias = "CH15_EVALCFG")]
pub type Ch15Evalcfg = crate::Reg<ch15_evalcfg::Ch15EvalcfgSpec>;
#[doc = "No Description"]
pub mod ch15_evalcfg;
#[doc = "CH15_EVALTHRES (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_evalthres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_evalthres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15_evalthres`] module"]
#[doc(alias = "CH15_EVALTHRES")]
pub type Ch15Evalthres = crate::Reg<ch15_evalthres::Ch15EvalthresSpec>;
#[doc = "No Description"]
pub mod ch15_evalthres;
#[doc = "ST0_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st0_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st0_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st0_arc`] module"]
#[doc(alias = "ST0_ARC")]
pub type St0Arc = crate::Reg<st0_arc::St0ArcSpec>;
#[doc = "No Description"]
pub mod st0_arc;
#[doc = "ST1_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st1_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st1_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st1_arc`] module"]
#[doc(alias = "ST1_ARC")]
pub type St1Arc = crate::Reg<st1_arc::St1ArcSpec>;
#[doc = "No Description"]
pub mod st1_arc;
#[doc = "ST2_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st2_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st2_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st2_arc`] module"]
#[doc(alias = "ST2_ARC")]
pub type St2Arc = crate::Reg<st2_arc::St2ArcSpec>;
#[doc = "No Description"]
pub mod st2_arc;
#[doc = "ST3_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st3_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st3_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st3_arc`] module"]
#[doc(alias = "ST3_ARC")]
pub type St3Arc = crate::Reg<st3_arc::St3ArcSpec>;
#[doc = "No Description"]
pub mod st3_arc;
#[doc = "ST4_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st4_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st4_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st4_arc`] module"]
#[doc(alias = "ST4_ARC")]
pub type St4Arc = crate::Reg<st4_arc::St4ArcSpec>;
#[doc = "No Description"]
pub mod st4_arc;
#[doc = "ST5_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st5_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st5_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st5_arc`] module"]
#[doc(alias = "ST5_ARC")]
pub type St5Arc = crate::Reg<st5_arc::St5ArcSpec>;
#[doc = "No Description"]
pub mod st5_arc;
#[doc = "ST6_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st6_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st6_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st6_arc`] module"]
#[doc(alias = "ST6_ARC")]
pub type St6Arc = crate::Reg<st6_arc::St6ArcSpec>;
#[doc = "No Description"]
pub mod st6_arc;
#[doc = "ST7_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st7_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st7_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st7_arc`] module"]
#[doc(alias = "ST7_ARC")]
pub type St7Arc = crate::Reg<st7_arc::St7ArcSpec>;
#[doc = "No Description"]
pub mod st7_arc;
#[doc = "ST8_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st8_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st8_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st8_arc`] module"]
#[doc(alias = "ST8_ARC")]
pub type St8Arc = crate::Reg<st8_arc::St8ArcSpec>;
#[doc = "No Description"]
pub mod st8_arc;
#[doc = "ST9_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st9_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st9_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st9_arc`] module"]
#[doc(alias = "ST9_ARC")]
pub type St9Arc = crate::Reg<st9_arc::St9ArcSpec>;
#[doc = "No Description"]
pub mod st9_arc;
#[doc = "ST10_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st10_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st10_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st10_arc`] module"]
#[doc(alias = "ST10_ARC")]
pub type St10Arc = crate::Reg<st10_arc::St10ArcSpec>;
#[doc = "No Description"]
pub mod st10_arc;
#[doc = "ST11_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st11_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st11_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st11_arc`] module"]
#[doc(alias = "ST11_ARC")]
pub type St11Arc = crate::Reg<st11_arc::St11ArcSpec>;
#[doc = "No Description"]
pub mod st11_arc;
#[doc = "ST12_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st12_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st12_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st12_arc`] module"]
#[doc(alias = "ST12_ARC")]
pub type St12Arc = crate::Reg<st12_arc::St12ArcSpec>;
#[doc = "No Description"]
pub mod st12_arc;
#[doc = "ST13_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st13_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st13_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st13_arc`] module"]
#[doc(alias = "ST13_ARC")]
pub type St13Arc = crate::Reg<st13_arc::St13ArcSpec>;
#[doc = "No Description"]
pub mod st13_arc;
#[doc = "ST14_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st14_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st14_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st14_arc`] module"]
#[doc(alias = "ST14_ARC")]
pub type St14Arc = crate::Reg<st14_arc::St14ArcSpec>;
#[doc = "No Description"]
pub mod st14_arc;
#[doc = "ST15_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st15_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st15_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st15_arc`] module"]
#[doc(alias = "ST15_ARC")]
pub type St15Arc = crate::Reg<st15_arc::St15ArcSpec>;
#[doc = "No Description"]
pub mod st15_arc;
#[doc = "ST16_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st16_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st16_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st16_arc`] module"]
#[doc(alias = "ST16_ARC")]
pub type St16Arc = crate::Reg<st16_arc::St16ArcSpec>;
#[doc = "No Description"]
pub mod st16_arc;
#[doc = "ST17_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st17_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st17_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st17_arc`] module"]
#[doc(alias = "ST17_ARC")]
pub type St17Arc = crate::Reg<st17_arc::St17ArcSpec>;
#[doc = "No Description"]
pub mod st17_arc;
#[doc = "ST18_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st18_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st18_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st18_arc`] module"]
#[doc(alias = "ST18_ARC")]
pub type St18Arc = crate::Reg<st18_arc::St18ArcSpec>;
#[doc = "No Description"]
pub mod st18_arc;
#[doc = "ST19_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st19_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st19_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st19_arc`] module"]
#[doc(alias = "ST19_ARC")]
pub type St19Arc = crate::Reg<st19_arc::St19ArcSpec>;
#[doc = "No Description"]
pub mod st19_arc;
#[doc = "ST20_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st20_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st20_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st20_arc`] module"]
#[doc(alias = "ST20_ARC")]
pub type St20Arc = crate::Reg<st20_arc::St20ArcSpec>;
#[doc = "No Description"]
pub mod st20_arc;
#[doc = "ST21_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st21_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st21_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st21_arc`] module"]
#[doc(alias = "ST21_ARC")]
pub type St21Arc = crate::Reg<st21_arc::St21ArcSpec>;
#[doc = "No Description"]
pub mod st21_arc;
#[doc = "ST22_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st22_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st22_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st22_arc`] module"]
#[doc(alias = "ST22_ARC")]
pub type St22Arc = crate::Reg<st22_arc::St22ArcSpec>;
#[doc = "No Description"]
pub mod st22_arc;
#[doc = "ST23_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st23_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st23_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st23_arc`] module"]
#[doc(alias = "ST23_ARC")]
pub type St23Arc = crate::Reg<st23_arc::St23ArcSpec>;
#[doc = "No Description"]
pub mod st23_arc;
#[doc = "ST24_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st24_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st24_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st24_arc`] module"]
#[doc(alias = "ST24_ARC")]
pub type St24Arc = crate::Reg<st24_arc::St24ArcSpec>;
#[doc = "No Description"]
pub mod st24_arc;
#[doc = "ST25_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st25_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st25_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st25_arc`] module"]
#[doc(alias = "ST25_ARC")]
pub type St25Arc = crate::Reg<st25_arc::St25ArcSpec>;
#[doc = "No Description"]
pub mod st25_arc;
#[doc = "ST26_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st26_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st26_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st26_arc`] module"]
#[doc(alias = "ST26_ARC")]
pub type St26Arc = crate::Reg<st26_arc::St26ArcSpec>;
#[doc = "No Description"]
pub mod st26_arc;
#[doc = "ST27_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st27_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st27_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st27_arc`] module"]
#[doc(alias = "ST27_ARC")]
pub type St27Arc = crate::Reg<st27_arc::St27ArcSpec>;
#[doc = "No Description"]
pub mod st27_arc;
#[doc = "ST28_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st28_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st28_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st28_arc`] module"]
#[doc(alias = "ST28_ARC")]
pub type St28Arc = crate::Reg<st28_arc::St28ArcSpec>;
#[doc = "No Description"]
pub mod st28_arc;
#[doc = "ST29_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st29_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st29_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st29_arc`] module"]
#[doc(alias = "ST29_ARC")]
pub type St29Arc = crate::Reg<st29_arc::St29ArcSpec>;
#[doc = "No Description"]
pub mod st29_arc;
#[doc = "ST30_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st30_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st30_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st30_arc`] module"]
#[doc(alias = "ST30_ARC")]
pub type St30Arc = crate::Reg<st30_arc::St30ArcSpec>;
#[doc = "No Description"]
pub mod st30_arc;
#[doc = "ST31_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st31_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st31_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st31_arc`] module"]
#[doc(alias = "ST31_ARC")]
pub type St31Arc = crate::Reg<st31_arc::St31ArcSpec>;
#[doc = "No Description"]
pub mod st31_arc;
#[doc = "ST32_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st32_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st32_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st32_arc`] module"]
#[doc(alias = "ST32_ARC")]
pub type St32Arc = crate::Reg<st32_arc::St32ArcSpec>;
#[doc = "No Description"]
pub mod st32_arc;
#[doc = "ST33_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st33_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st33_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st33_arc`] module"]
#[doc(alias = "ST33_ARC")]
pub type St33Arc = crate::Reg<st33_arc::St33ArcSpec>;
#[doc = "No Description"]
pub mod st33_arc;
#[doc = "ST34_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st34_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st34_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st34_arc`] module"]
#[doc(alias = "ST34_ARC")]
pub type St34Arc = crate::Reg<st34_arc::St34ArcSpec>;
#[doc = "No Description"]
pub mod st34_arc;
#[doc = "ST35_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st35_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st35_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st35_arc`] module"]
#[doc(alias = "ST35_ARC")]
pub type St35Arc = crate::Reg<st35_arc::St35ArcSpec>;
#[doc = "No Description"]
pub mod st35_arc;
#[doc = "ST36_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st36_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st36_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st36_arc`] module"]
#[doc(alias = "ST36_ARC")]
pub type St36Arc = crate::Reg<st36_arc::St36ArcSpec>;
#[doc = "No Description"]
pub mod st36_arc;
#[doc = "ST37_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st37_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st37_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st37_arc`] module"]
#[doc(alias = "ST37_ARC")]
pub type St37Arc = crate::Reg<st37_arc::St37ArcSpec>;
#[doc = "No Description"]
pub mod st37_arc;
#[doc = "ST38_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st38_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st38_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st38_arc`] module"]
#[doc(alias = "ST38_ARC")]
pub type St38Arc = crate::Reg<st38_arc::St38ArcSpec>;
#[doc = "No Description"]
pub mod st38_arc;
#[doc = "ST39_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st39_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st39_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st39_arc`] module"]
#[doc(alias = "ST39_ARC")]
pub type St39Arc = crate::Reg<st39_arc::St39ArcSpec>;
#[doc = "No Description"]
pub mod st39_arc;
#[doc = "ST40_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st40_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st40_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st40_arc`] module"]
#[doc(alias = "ST40_ARC")]
pub type St40Arc = crate::Reg<st40_arc::St40ArcSpec>;
#[doc = "No Description"]
pub mod st40_arc;
#[doc = "ST41_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st41_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st41_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st41_arc`] module"]
#[doc(alias = "ST41_ARC")]
pub type St41Arc = crate::Reg<st41_arc::St41ArcSpec>;
#[doc = "No Description"]
pub mod st41_arc;
#[doc = "ST42_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st42_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st42_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st42_arc`] module"]
#[doc(alias = "ST42_ARC")]
pub type St42Arc = crate::Reg<st42_arc::St42ArcSpec>;
#[doc = "No Description"]
pub mod st42_arc;
#[doc = "ST43_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st43_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st43_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st43_arc`] module"]
#[doc(alias = "ST43_ARC")]
pub type St43Arc = crate::Reg<st43_arc::St43ArcSpec>;
#[doc = "No Description"]
pub mod st43_arc;
#[doc = "ST44_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st44_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st44_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st44_arc`] module"]
#[doc(alias = "ST44_ARC")]
pub type St44Arc = crate::Reg<st44_arc::St44ArcSpec>;
#[doc = "No Description"]
pub mod st44_arc;
#[doc = "ST45_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st45_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st45_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st45_arc`] module"]
#[doc(alias = "ST45_ARC")]
pub type St45Arc = crate::Reg<st45_arc::St45ArcSpec>;
#[doc = "No Description"]
pub mod st45_arc;
#[doc = "ST46_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st46_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st46_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st46_arc`] module"]
#[doc(alias = "ST46_ARC")]
pub type St46Arc = crate::Reg<st46_arc::St46ArcSpec>;
#[doc = "No Description"]
pub mod st46_arc;
#[doc = "ST47_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st47_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st47_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st47_arc`] module"]
#[doc(alias = "ST47_ARC")]
pub type St47Arc = crate::Reg<st47_arc::St47ArcSpec>;
#[doc = "No Description"]
pub mod st47_arc;
#[doc = "ST48_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st48_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st48_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st48_arc`] module"]
#[doc(alias = "ST48_ARC")]
pub type St48Arc = crate::Reg<st48_arc::St48ArcSpec>;
#[doc = "No Description"]
pub mod st48_arc;
#[doc = "ST49_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st49_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st49_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st49_arc`] module"]
#[doc(alias = "ST49_ARC")]
pub type St49Arc = crate::Reg<st49_arc::St49ArcSpec>;
#[doc = "No Description"]
pub mod st49_arc;
#[doc = "ST50_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st50_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st50_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st50_arc`] module"]
#[doc(alias = "ST50_ARC")]
pub type St50Arc = crate::Reg<st50_arc::St50ArcSpec>;
#[doc = "No Description"]
pub mod st50_arc;
#[doc = "ST51_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st51_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st51_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st51_arc`] module"]
#[doc(alias = "ST51_ARC")]
pub type St51Arc = crate::Reg<st51_arc::St51ArcSpec>;
#[doc = "No Description"]
pub mod st51_arc;
#[doc = "ST52_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st52_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st52_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st52_arc`] module"]
#[doc(alias = "ST52_ARC")]
pub type St52Arc = crate::Reg<st52_arc::St52ArcSpec>;
#[doc = "No Description"]
pub mod st52_arc;
#[doc = "ST53_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st53_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st53_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st53_arc`] module"]
#[doc(alias = "ST53_ARC")]
pub type St53Arc = crate::Reg<st53_arc::St53ArcSpec>;
#[doc = "No Description"]
pub mod st53_arc;
#[doc = "ST54_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st54_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st54_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st54_arc`] module"]
#[doc(alias = "ST54_ARC")]
pub type St54Arc = crate::Reg<st54_arc::St54ArcSpec>;
#[doc = "No Description"]
pub mod st54_arc;
#[doc = "ST55_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st55_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st55_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st55_arc`] module"]
#[doc(alias = "ST55_ARC")]
pub type St55Arc = crate::Reg<st55_arc::St55ArcSpec>;
#[doc = "No Description"]
pub mod st55_arc;
#[doc = "ST56_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st56_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st56_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st56_arc`] module"]
#[doc(alias = "ST56_ARC")]
pub type St56Arc = crate::Reg<st56_arc::St56ArcSpec>;
#[doc = "No Description"]
pub mod st56_arc;
#[doc = "ST57_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st57_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st57_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st57_arc`] module"]
#[doc(alias = "ST57_ARC")]
pub type St57Arc = crate::Reg<st57_arc::St57ArcSpec>;
#[doc = "No Description"]
pub mod st57_arc;
#[doc = "ST58_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st58_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st58_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st58_arc`] module"]
#[doc(alias = "ST58_ARC")]
pub type St58Arc = crate::Reg<st58_arc::St58ArcSpec>;
#[doc = "No Description"]
pub mod st58_arc;
#[doc = "ST59_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st59_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st59_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st59_arc`] module"]
#[doc(alias = "ST59_ARC")]
pub type St59Arc = crate::Reg<st59_arc::St59ArcSpec>;
#[doc = "No Description"]
pub mod st59_arc;
#[doc = "ST60_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st60_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st60_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st60_arc`] module"]
#[doc(alias = "ST60_ARC")]
pub type St60Arc = crate::Reg<st60_arc::St60ArcSpec>;
#[doc = "No Description"]
pub mod st60_arc;
#[doc = "ST61_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st61_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st61_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st61_arc`] module"]
#[doc(alias = "ST61_ARC")]
pub type St61Arc = crate::Reg<st61_arc::St61ArcSpec>;
#[doc = "No Description"]
pub mod st61_arc;
#[doc = "ST62_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st62_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st62_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st62_arc`] module"]
#[doc(alias = "ST62_ARC")]
pub type St62Arc = crate::Reg<st62_arc::St62ArcSpec>;
#[doc = "No Description"]
pub mod st62_arc;
#[doc = "ST63_ARC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st63_arc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st63_arc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st63_arc`] module"]
#[doc(alias = "ST63_ARC")]
pub type St63Arc = crate::Reg<st63_arc::St63ArcSpec>;
#[doc = "No Description"]
pub mod st63_arc;
