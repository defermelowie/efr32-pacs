#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    _reserved1: [u8; 0x04],
    async_swpulse: AsyncSwpulse,
    async_swlevel: AsyncSwlevel,
    async_peek: AsyncPeek,
    sync_peek: SyncPeek,
    async_ch0_ctrl: AsyncCh0Ctrl,
    async_ch1_ctrl: AsyncCh1Ctrl,
    async_ch2_ctrl: AsyncCh2Ctrl,
    async_ch3_ctrl: AsyncCh3Ctrl,
    async_ch4_ctrl: AsyncCh4Ctrl,
    async_ch5_ctrl: AsyncCh5Ctrl,
    async_ch6_ctrl: AsyncCh6Ctrl,
    async_ch7_ctrl: AsyncCh7Ctrl,
    async_ch8_ctrl: AsyncCh8Ctrl,
    async_ch9_ctrl: AsyncCh9Ctrl,
    async_ch10_ctrl: AsyncCh10Ctrl,
    async_ch11_ctrl: AsyncCh11Ctrl,
    sync_ch0_ctrl: SyncCh0Ctrl,
    sync_ch1_ctrl: SyncCh1Ctrl,
    sync_ch2_ctrl: SyncCh2Ctrl,
    sync_ch3_ctrl: SyncCh3Ctrl,
    consumer_cmu_caldn: ConsumerCmuCaldn,
    consumer_cmu_calup: ConsumerCmuCalup,
    consumer_eusart0_clk: ConsumerEusart0Clk,
    consumer_eusart0_rx: ConsumerEusart0Rx,
    consumer_eusart0_trigger: ConsumerEusart0Trigger,
    consumer_eusart1_clk: ConsumerEusart1Clk,
    consumer_eusart1_rx: ConsumerEusart1Rx,
    consumer_eusart1_trigger: ConsumerEusart1Trigger,
    consumer_eusart2_clk: ConsumerEusart2Clk,
    consumer_eusart2_rx: ConsumerEusart2Rx,
    consumer_eusart2_trigger: ConsumerEusart2Trigger,
    consumer_eusart3_clk: ConsumerEusart3Clk,
    consumer_eusart3_rx: ConsumerEusart3Rx,
    consumer_eusart3_trigger: ConsumerEusart3Trigger,
    consumer_eusart4_clk: ConsumerEusart4Clk,
    consumer_eusart4_rx: ConsumerEusart4Rx,
    consumer_eusart4_trigger: ConsumerEusart4Trigger,
    consumer_frc_rxraw: ConsumerFrcRxraw,
    consumer_iadc0_scantrigger: ConsumerIadc0Scantrigger,
    consumer_iadc0_singletrigger: ConsumerIadc0Singletrigger,
    consumer_ldmaxbar_dmareq0: ConsumerLdmaxbarDmareq0,
    consumer_ldmaxbar_dmareq1: ConsumerLdmaxbarDmareq1,
    consumer_lesense_decin0: ConsumerLesenseDecin0,
    consumer_lesense_decin1: ConsumerLesenseDecin1,
    consumer_lesense_decin2: ConsumerLesenseDecin2,
    consumer_lesense_decin3: ConsumerLesenseDecin3,
    consumer_lesense_start: ConsumerLesenseStart,
    consumer_letimer0_clear: ConsumerLetimer0Clear,
    consumer_letimer0_start: ConsumerLetimer0Start,
    consumer_letimer0_stop: ConsumerLetimer0Stop,
    consumer_modem_din: ConsumerModemDin,
    consumer_pcnt0_s0in: ConsumerPcnt0S0in,
    consumer_pcnt0_s1in: ConsumerPcnt0S1in,
    _reserved54: [u8; 0x2c],
    consumer_rac_clr: ConsumerRacClr,
    consumer_rac_ctiin0: ConsumerRacCtiin0,
    consumer_rac_ctiin1: ConsumerRacCtiin1,
    consumer_rac_ctiin2: ConsumerRacCtiin2,
    consumer_rac_ctiin3: ConsumerRacCtiin3,
    consumer_rac_forcetx: ConsumerRacForcetx,
    consumer_rac_rxdis: ConsumerRacRxdis,
    consumer_rac_rxen: ConsumerRacRxen,
    consumer_rac_txen: ConsumerRacTxen,
    consumer_setamper_tampersrc26: ConsumerSetamperTampersrc26,
    consumer_setamper_tampersrc27: ConsumerSetamperTampersrc27,
    consumer_setamper_tampersrc28: ConsumerSetamperTampersrc28,
    consumer_setamper_tampersrc29: ConsumerSetamperTampersrc29,
    consumer_setamper_tampersrc30: ConsumerSetamperTampersrc30,
    consumer_setamper_tampersrc31: ConsumerSetamperTampersrc31,
    consumer_sysrtc0_in0: ConsumerSysrtc0In0,
    consumer_sysrtc0_in1: ConsumerSysrtc0In1,
    consumer_hfxo0_oscreq: ConsumerHfxo0Oscreq,
    consumer_hfxo0_timeout: ConsumerHfxo0Timeout,
    consumer_core_ctiin0: ConsumerCoreCtiin0,
    consumer_core_ctiin1: ConsumerCoreCtiin1,
    consumer_core_ctiin2: ConsumerCoreCtiin2,
    consumer_core_ctiin3: ConsumerCoreCtiin3,
    consumer_core_m33rxev: ConsumerCoreM33rxev,
    consumer_timer0_cc0: ConsumerTimer0Cc0,
    consumer_timer0_cc1: ConsumerTimer0Cc1,
    consumer_timer0_cc2: ConsumerTimer0Cc2,
    consumer_timer0_dti: ConsumerTimer0Dti,
    consumer_timer0_dtifs1: ConsumerTimer0Dtifs1,
    consumer_timer0_dtifs2: ConsumerTimer0Dtifs2,
    consumer_timer1_cc0: ConsumerTimer1Cc0,
    consumer_timer1_cc1: ConsumerTimer1Cc1,
    consumer_timer1_cc2: ConsumerTimer1Cc2,
    consumer_timer1_dti: ConsumerTimer1Dti,
    consumer_timer1_dtifs1: ConsumerTimer1Dtifs1,
    consumer_timer1_dtifs2: ConsumerTimer1Dtifs2,
    consumer_timer2_cc0: ConsumerTimer2Cc0,
    consumer_timer2_cc1: ConsumerTimer2Cc1,
    consumer_timer2_cc2: ConsumerTimer2Cc2,
    consumer_timer2_dti: ConsumerTimer2Dti,
    consumer_timer2_dtifs1: ConsumerTimer2Dtifs1,
    consumer_timer2_dtifs2: ConsumerTimer2Dtifs2,
    consumer_timer3_cc0: ConsumerTimer3Cc0,
    consumer_timer3_cc1: ConsumerTimer3Cc1,
    consumer_timer3_cc2: ConsumerTimer3Cc2,
    consumer_timer3_dti: ConsumerTimer3Dti,
    consumer_timer3_dtifs1: ConsumerTimer3Dtifs1,
    consumer_timer3_dtifs2: ConsumerTimer3Dtifs2,
    consumer_timer4_cc0: ConsumerTimer4Cc0,
    consumer_timer4_cc1: ConsumerTimer4Cc1,
    consumer_timer4_cc2: ConsumerTimer4Cc2,
    consumer_timer4_dti: ConsumerTimer4Dti,
    consumer_timer4_dtifs1: ConsumerTimer4Dtifs1,
    consumer_timer4_dtifs2: ConsumerTimer4Dtifs2,
    consumer_timer5_cc0: ConsumerTimer5Cc0,
    consumer_timer5_cc1: ConsumerTimer5Cc1,
    consumer_timer5_cc2: ConsumerTimer5Cc2,
    consumer_timer5_dti: ConsumerTimer5Dti,
    consumer_timer5_dtifs1: ConsumerTimer5Dtifs1,
    consumer_timer5_dtifs2: ConsumerTimer5Dtifs2,
    consumer_timer6_cc0: ConsumerTimer6Cc0,
    consumer_timer6_cc1: ConsumerTimer6Cc1,
    consumer_timer6_cc2: ConsumerTimer6Cc2,
    consumer_timer6_dti: ConsumerTimer6Dti,
    consumer_timer6_dtifs1: ConsumerTimer6Dtifs1,
    consumer_timer6_dtifs2: ConsumerTimer6Dtifs2,
    consumer_timer7_cc0: ConsumerTimer7Cc0,
    consumer_timer7_cc1: ConsumerTimer7Cc1,
    consumer_timer7_cc2: ConsumerTimer7Cc2,
    consumer_timer7_dti: ConsumerTimer7Dti,
    consumer_timer7_dtifs1: ConsumerTimer7Dtifs1,
    consumer_timer7_dtifs2: ConsumerTimer7Dtifs2,
    consumer_vdac0_asynctrigch0: ConsumerVdac0Asynctrigch0,
    consumer_vdac0_asynctrigch1: ConsumerVdac0Asynctrigch1,
    consumer_vdac0_synctrigch0: ConsumerVdac0Synctrigch0,
    consumer_vdac0_synctrigch1: ConsumerVdac0Synctrigch1,
    consumer_wdog0_src0: ConsumerWdog0Src0,
    consumer_wdog0_src1: ConsumerWdog0Src1,
    consumer_wdog1_src0: ConsumerWdog1Src0,
    consumer_wdog1_src1: ConsumerWdog1Src1,
}
impl RegisterBlock {
    #[doc = "0x00 - No Description"]
    #[inline(always)]
    pub const fn ipversion(&self) -> &Ipversion {
        &self.ipversion
    }
    #[doc = "0x08 - No Description"]
    #[inline(always)]
    pub const fn async_swpulse(&self) -> &AsyncSwpulse {
        &self.async_swpulse
    }
    #[doc = "0x0c - No Description"]
    #[inline(always)]
    pub const fn async_swlevel(&self) -> &AsyncSwlevel {
        &self.async_swlevel
    }
    #[doc = "0x10 - No Description"]
    #[inline(always)]
    pub const fn async_peek(&self) -> &AsyncPeek {
        &self.async_peek
    }
    #[doc = "0x14 - No Description"]
    #[inline(always)]
    pub const fn sync_peek(&self) -> &SyncPeek {
        &self.sync_peek
    }
    #[doc = "0x18 - No Description"]
    #[inline(always)]
    pub const fn async_ch0_ctrl(&self) -> &AsyncCh0Ctrl {
        &self.async_ch0_ctrl
    }
    #[doc = "0x1c - No Description"]
    #[inline(always)]
    pub const fn async_ch1_ctrl(&self) -> &AsyncCh1Ctrl {
        &self.async_ch1_ctrl
    }
    #[doc = "0x20 - No Description"]
    #[inline(always)]
    pub const fn async_ch2_ctrl(&self) -> &AsyncCh2Ctrl {
        &self.async_ch2_ctrl
    }
    #[doc = "0x24 - No Description"]
    #[inline(always)]
    pub const fn async_ch3_ctrl(&self) -> &AsyncCh3Ctrl {
        &self.async_ch3_ctrl
    }
    #[doc = "0x28 - No Description"]
    #[inline(always)]
    pub const fn async_ch4_ctrl(&self) -> &AsyncCh4Ctrl {
        &self.async_ch4_ctrl
    }
    #[doc = "0x2c - No Description"]
    #[inline(always)]
    pub const fn async_ch5_ctrl(&self) -> &AsyncCh5Ctrl {
        &self.async_ch5_ctrl
    }
    #[doc = "0x30 - No Description"]
    #[inline(always)]
    pub const fn async_ch6_ctrl(&self) -> &AsyncCh6Ctrl {
        &self.async_ch6_ctrl
    }
    #[doc = "0x34 - No Description"]
    #[inline(always)]
    pub const fn async_ch7_ctrl(&self) -> &AsyncCh7Ctrl {
        &self.async_ch7_ctrl
    }
    #[doc = "0x38 - No Description"]
    #[inline(always)]
    pub const fn async_ch8_ctrl(&self) -> &AsyncCh8Ctrl {
        &self.async_ch8_ctrl
    }
    #[doc = "0x3c - No Description"]
    #[inline(always)]
    pub const fn async_ch9_ctrl(&self) -> &AsyncCh9Ctrl {
        &self.async_ch9_ctrl
    }
    #[doc = "0x40 - No Description"]
    #[inline(always)]
    pub const fn async_ch10_ctrl(&self) -> &AsyncCh10Ctrl {
        &self.async_ch10_ctrl
    }
    #[doc = "0x44 - No Description"]
    #[inline(always)]
    pub const fn async_ch11_ctrl(&self) -> &AsyncCh11Ctrl {
        &self.async_ch11_ctrl
    }
    #[doc = "0x48 - No Description"]
    #[inline(always)]
    pub const fn sync_ch0_ctrl(&self) -> &SyncCh0Ctrl {
        &self.sync_ch0_ctrl
    }
    #[doc = "0x4c - No Description"]
    #[inline(always)]
    pub const fn sync_ch1_ctrl(&self) -> &SyncCh1Ctrl {
        &self.sync_ch1_ctrl
    }
    #[doc = "0x50 - No Description"]
    #[inline(always)]
    pub const fn sync_ch2_ctrl(&self) -> &SyncCh2Ctrl {
        &self.sync_ch2_ctrl
    }
    #[doc = "0x54 - No Description"]
    #[inline(always)]
    pub const fn sync_ch3_ctrl(&self) -> &SyncCh3Ctrl {
        &self.sync_ch3_ctrl
    }
    #[doc = "0x58 - CALDN consumer register"]
    #[inline(always)]
    pub const fn consumer_cmu_caldn(&self) -> &ConsumerCmuCaldn {
        &self.consumer_cmu_caldn
    }
    #[doc = "0x5c - CALUP Consumer register"]
    #[inline(always)]
    pub const fn consumer_cmu_calup(&self) -> &ConsumerCmuCalup {
        &self.consumer_cmu_calup
    }
    #[doc = "0x60 - CLK consumer register"]
    #[inline(always)]
    pub const fn consumer_eusart0_clk(&self) -> &ConsumerEusart0Clk {
        &self.consumer_eusart0_clk
    }
    #[doc = "0x64 - RX Consumer register"]
    #[inline(always)]
    pub const fn consumer_eusart0_rx(&self) -> &ConsumerEusart0Rx {
        &self.consumer_eusart0_rx
    }
    #[doc = "0x68 - TRIGGER Consumer register"]
    #[inline(always)]
    pub const fn consumer_eusart0_trigger(&self) -> &ConsumerEusart0Trigger {
        &self.consumer_eusart0_trigger
    }
    #[doc = "0x6c - CLK consumer register"]
    #[inline(always)]
    pub const fn consumer_eusart1_clk(&self) -> &ConsumerEusart1Clk {
        &self.consumer_eusart1_clk
    }
    #[doc = "0x70 - RX Consumer register"]
    #[inline(always)]
    pub const fn consumer_eusart1_rx(&self) -> &ConsumerEusart1Rx {
        &self.consumer_eusart1_rx
    }
    #[doc = "0x74 - TRIGGER Consumer register"]
    #[inline(always)]
    pub const fn consumer_eusart1_trigger(&self) -> &ConsumerEusart1Trigger {
        &self.consumer_eusart1_trigger
    }
    #[doc = "0x78 - CLK consumer register"]
    #[inline(always)]
    pub const fn consumer_eusart2_clk(&self) -> &ConsumerEusart2Clk {
        &self.consumer_eusart2_clk
    }
    #[doc = "0x7c - RX Consumer register"]
    #[inline(always)]
    pub const fn consumer_eusart2_rx(&self) -> &ConsumerEusart2Rx {
        &self.consumer_eusart2_rx
    }
    #[doc = "0x80 - TRIGGER Consumer register"]
    #[inline(always)]
    pub const fn consumer_eusart2_trigger(&self) -> &ConsumerEusart2Trigger {
        &self.consumer_eusart2_trigger
    }
    #[doc = "0x84 - CLK consumer register"]
    #[inline(always)]
    pub const fn consumer_eusart3_clk(&self) -> &ConsumerEusart3Clk {
        &self.consumer_eusart3_clk
    }
    #[doc = "0x88 - RX Consumer register"]
    #[inline(always)]
    pub const fn consumer_eusart3_rx(&self) -> &ConsumerEusart3Rx {
        &self.consumer_eusart3_rx
    }
    #[doc = "0x8c - TRIGGER Consumer register"]
    #[inline(always)]
    pub const fn consumer_eusart3_trigger(&self) -> &ConsumerEusart3Trigger {
        &self.consumer_eusart3_trigger
    }
    #[doc = "0x90 - CLK consumer register"]
    #[inline(always)]
    pub const fn consumer_eusart4_clk(&self) -> &ConsumerEusart4Clk {
        &self.consumer_eusart4_clk
    }
    #[doc = "0x94 - RX Consumer register"]
    #[inline(always)]
    pub const fn consumer_eusart4_rx(&self) -> &ConsumerEusart4Rx {
        &self.consumer_eusart4_rx
    }
    #[doc = "0x98 - TRIGGER Consumer register"]
    #[inline(always)]
    pub const fn consumer_eusart4_trigger(&self) -> &ConsumerEusart4Trigger {
        &self.consumer_eusart4_trigger
    }
    #[doc = "0x9c - RXRAW consumer register"]
    #[inline(always)]
    pub const fn consumer_frc_rxraw(&self) -> &ConsumerFrcRxraw {
        &self.consumer_frc_rxraw
    }
    #[doc = "0xa0 - SCAN consumer register"]
    #[inline(always)]
    pub const fn consumer_iadc0_scantrigger(&self) -> &ConsumerIadc0Scantrigger {
        &self.consumer_iadc0_scantrigger
    }
    #[doc = "0xa4 - SINGLE Consumer register"]
    #[inline(always)]
    pub const fn consumer_iadc0_singletrigger(&self) -> &ConsumerIadc0Singletrigger {
        &self.consumer_iadc0_singletrigger
    }
    #[doc = "0xa8 - DMAREQ0 consumer register"]
    #[inline(always)]
    pub const fn consumer_ldmaxbar_dmareq0(&self) -> &ConsumerLdmaxbarDmareq0 {
        &self.consumer_ldmaxbar_dmareq0
    }
    #[doc = "0xac - DMAREQ1 Consumer register"]
    #[inline(always)]
    pub const fn consumer_ldmaxbar_dmareq1(&self) -> &ConsumerLdmaxbarDmareq1 {
        &self.consumer_ldmaxbar_dmareq1
    }
    #[doc = "0xb0 - DECIN0 consumer register"]
    #[inline(always)]
    pub const fn consumer_lesense_decin0(&self) -> &ConsumerLesenseDecin0 {
        &self.consumer_lesense_decin0
    }
    #[doc = "0xb4 - DECIN1 Consumer register"]
    #[inline(always)]
    pub const fn consumer_lesense_decin1(&self) -> &ConsumerLesenseDecin1 {
        &self.consumer_lesense_decin1
    }
    #[doc = "0xb8 - DECIN2 Consumer register"]
    #[inline(always)]
    pub const fn consumer_lesense_decin2(&self) -> &ConsumerLesenseDecin2 {
        &self.consumer_lesense_decin2
    }
    #[doc = "0xbc - DECIN3 Consumer register"]
    #[inline(always)]
    pub const fn consumer_lesense_decin3(&self) -> &ConsumerLesenseDecin3 {
        &self.consumer_lesense_decin3
    }
    #[doc = "0xc0 - START Consumer register"]
    #[inline(always)]
    pub const fn consumer_lesense_start(&self) -> &ConsumerLesenseStart {
        &self.consumer_lesense_start
    }
    #[doc = "0xc4 - CLEAR consumer register"]
    #[inline(always)]
    pub const fn consumer_letimer0_clear(&self) -> &ConsumerLetimer0Clear {
        &self.consumer_letimer0_clear
    }
    #[doc = "0xc8 - START Consumer register"]
    #[inline(always)]
    pub const fn consumer_letimer0_start(&self) -> &ConsumerLetimer0Start {
        &self.consumer_letimer0_start
    }
    #[doc = "0xcc - STOP Consumer register"]
    #[inline(always)]
    pub const fn consumer_letimer0_stop(&self) -> &ConsumerLetimer0Stop {
        &self.consumer_letimer0_stop
    }
    #[doc = "0xd0 - DIN consumer register"]
    #[inline(always)]
    pub const fn consumer_modem_din(&self) -> &ConsumerModemDin {
        &self.consumer_modem_din
    }
    #[doc = "0xd4 - S0IN consumer register"]
    #[inline(always)]
    pub const fn consumer_pcnt0_s0in(&self) -> &ConsumerPcnt0S0in {
        &self.consumer_pcnt0_s0in
    }
    #[doc = "0xd8 - S1IN Consumer register"]
    #[inline(always)]
    pub const fn consumer_pcnt0_s1in(&self) -> &ConsumerPcnt0S1in {
        &self.consumer_pcnt0_s1in
    }
    #[doc = "0x108 - CLR consumer register"]
    #[inline(always)]
    pub const fn consumer_rac_clr(&self) -> &ConsumerRacClr {
        &self.consumer_rac_clr
    }
    #[doc = "0x10c - CTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_rac_ctiin0(&self) -> &ConsumerRacCtiin0 {
        &self.consumer_rac_ctiin0
    }
    #[doc = "0x110 - CTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_rac_ctiin1(&self) -> &ConsumerRacCtiin1 {
        &self.consumer_rac_ctiin1
    }
    #[doc = "0x114 - CTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_rac_ctiin2(&self) -> &ConsumerRacCtiin2 {
        &self.consumer_rac_ctiin2
    }
    #[doc = "0x118 - CTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_rac_ctiin3(&self) -> &ConsumerRacCtiin3 {
        &self.consumer_rac_ctiin3
    }
    #[doc = "0x11c - FORCETX Consumer register"]
    #[inline(always)]
    pub const fn consumer_rac_forcetx(&self) -> &ConsumerRacForcetx {
        &self.consumer_rac_forcetx
    }
    #[doc = "0x120 - RXDIS Consumer register"]
    #[inline(always)]
    pub const fn consumer_rac_rxdis(&self) -> &ConsumerRacRxdis {
        &self.consumer_rac_rxdis
    }
    #[doc = "0x124 - RXEN Consumer register"]
    #[inline(always)]
    pub const fn consumer_rac_rxen(&self) -> &ConsumerRacRxen {
        &self.consumer_rac_rxen
    }
    #[doc = "0x128 - TXEN Consumer register"]
    #[inline(always)]
    pub const fn consumer_rac_txen(&self) -> &ConsumerRacTxen {
        &self.consumer_rac_txen
    }
    #[doc = "0x12c - TAMPERSRC26 consumer register"]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc26(&self) -> &ConsumerSetamperTampersrc26 {
        &self.consumer_setamper_tampersrc26
    }
    #[doc = "0x130 - TAMPERSRC27 Consumer register"]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc27(&self) -> &ConsumerSetamperTampersrc27 {
        &self.consumer_setamper_tampersrc27
    }
    #[doc = "0x134 - TAMPERSRC28 Consumer register"]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc28(&self) -> &ConsumerSetamperTampersrc28 {
        &self.consumer_setamper_tampersrc28
    }
    #[doc = "0x138 - TAMPERSRC29 Consumer register"]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc29(&self) -> &ConsumerSetamperTampersrc29 {
        &self.consumer_setamper_tampersrc29
    }
    #[doc = "0x13c - TAMPERSRC30 Consumer register"]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc30(&self) -> &ConsumerSetamperTampersrc30 {
        &self.consumer_setamper_tampersrc30
    }
    #[doc = "0x140 - TAMPERSRC31 Consumer register"]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc31(&self) -> &ConsumerSetamperTampersrc31 {
        &self.consumer_setamper_tampersrc31
    }
    #[doc = "0x144 - IN0 consumer register"]
    #[inline(always)]
    pub const fn consumer_sysrtc0_in0(&self) -> &ConsumerSysrtc0In0 {
        &self.consumer_sysrtc0_in0
    }
    #[doc = "0x148 - IN1 Consumer register"]
    #[inline(always)]
    pub const fn consumer_sysrtc0_in1(&self) -> &ConsumerSysrtc0In1 {
        &self.consumer_sysrtc0_in1
    }
    #[doc = "0x14c - OSCREQ consumer register"]
    #[inline(always)]
    pub const fn consumer_hfxo0_oscreq(&self) -> &ConsumerHfxo0Oscreq {
        &self.consumer_hfxo0_oscreq
    }
    #[doc = "0x150 - TIMEOUT Consumer register"]
    #[inline(always)]
    pub const fn consumer_hfxo0_timeout(&self) -> &ConsumerHfxo0Timeout {
        &self.consumer_hfxo0_timeout
    }
    #[doc = "0x154 - CTI consumer register"]
    #[inline(always)]
    pub const fn consumer_core_ctiin0(&self) -> &ConsumerCoreCtiin0 {
        &self.consumer_core_ctiin0
    }
    #[doc = "0x158 - CTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_core_ctiin1(&self) -> &ConsumerCoreCtiin1 {
        &self.consumer_core_ctiin1
    }
    #[doc = "0x15c - CTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_core_ctiin2(&self) -> &ConsumerCoreCtiin2 {
        &self.consumer_core_ctiin2
    }
    #[doc = "0x160 - CTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_core_ctiin3(&self) -> &ConsumerCoreCtiin3 {
        &self.consumer_core_ctiin3
    }
    #[doc = "0x164 - M33 Consumer register"]
    #[inline(always)]
    pub const fn consumer_core_m33rxev(&self) -> &ConsumerCoreM33rxev {
        &self.consumer_core_m33rxev
    }
    #[doc = "0x168 - CC0 consumer register"]
    #[inline(always)]
    pub const fn consumer_timer0_cc0(&self) -> &ConsumerTimer0Cc0 {
        &self.consumer_timer0_cc0
    }
    #[doc = "0x16c - CC1 Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer0_cc1(&self) -> &ConsumerTimer0Cc1 {
        &self.consumer_timer0_cc1
    }
    #[doc = "0x170 - CC2 Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer0_cc2(&self) -> &ConsumerTimer0Cc2 {
        &self.consumer_timer0_cc2
    }
    #[doc = "0x174 - DTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer0_dti(&self) -> &ConsumerTimer0Dti {
        &self.consumer_timer0_dti
    }
    #[doc = "0x178 - DTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer0_dtifs1(&self) -> &ConsumerTimer0Dtifs1 {
        &self.consumer_timer0_dtifs1
    }
    #[doc = "0x17c - DTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer0_dtifs2(&self) -> &ConsumerTimer0Dtifs2 {
        &self.consumer_timer0_dtifs2
    }
    #[doc = "0x180 - CC0 consumer register"]
    #[inline(always)]
    pub const fn consumer_timer1_cc0(&self) -> &ConsumerTimer1Cc0 {
        &self.consumer_timer1_cc0
    }
    #[doc = "0x184 - CC1 Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer1_cc1(&self) -> &ConsumerTimer1Cc1 {
        &self.consumer_timer1_cc1
    }
    #[doc = "0x188 - CC2 Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer1_cc2(&self) -> &ConsumerTimer1Cc2 {
        &self.consumer_timer1_cc2
    }
    #[doc = "0x18c - DTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer1_dti(&self) -> &ConsumerTimer1Dti {
        &self.consumer_timer1_dti
    }
    #[doc = "0x190 - DTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer1_dtifs1(&self) -> &ConsumerTimer1Dtifs1 {
        &self.consumer_timer1_dtifs1
    }
    #[doc = "0x194 - DTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer1_dtifs2(&self) -> &ConsumerTimer1Dtifs2 {
        &self.consumer_timer1_dtifs2
    }
    #[doc = "0x198 - CC0 consumer register"]
    #[inline(always)]
    pub const fn consumer_timer2_cc0(&self) -> &ConsumerTimer2Cc0 {
        &self.consumer_timer2_cc0
    }
    #[doc = "0x19c - CC1 Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer2_cc1(&self) -> &ConsumerTimer2Cc1 {
        &self.consumer_timer2_cc1
    }
    #[doc = "0x1a0 - CC2 Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer2_cc2(&self) -> &ConsumerTimer2Cc2 {
        &self.consumer_timer2_cc2
    }
    #[doc = "0x1a4 - DTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer2_dti(&self) -> &ConsumerTimer2Dti {
        &self.consumer_timer2_dti
    }
    #[doc = "0x1a8 - DTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer2_dtifs1(&self) -> &ConsumerTimer2Dtifs1 {
        &self.consumer_timer2_dtifs1
    }
    #[doc = "0x1ac - DTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer2_dtifs2(&self) -> &ConsumerTimer2Dtifs2 {
        &self.consumer_timer2_dtifs2
    }
    #[doc = "0x1b0 - CC0 consumer register"]
    #[inline(always)]
    pub const fn consumer_timer3_cc0(&self) -> &ConsumerTimer3Cc0 {
        &self.consumer_timer3_cc0
    }
    #[doc = "0x1b4 - CC1 Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer3_cc1(&self) -> &ConsumerTimer3Cc1 {
        &self.consumer_timer3_cc1
    }
    #[doc = "0x1b8 - CC2 Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer3_cc2(&self) -> &ConsumerTimer3Cc2 {
        &self.consumer_timer3_cc2
    }
    #[doc = "0x1bc - DTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer3_dti(&self) -> &ConsumerTimer3Dti {
        &self.consumer_timer3_dti
    }
    #[doc = "0x1c0 - DTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer3_dtifs1(&self) -> &ConsumerTimer3Dtifs1 {
        &self.consumer_timer3_dtifs1
    }
    #[doc = "0x1c4 - DTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer3_dtifs2(&self) -> &ConsumerTimer3Dtifs2 {
        &self.consumer_timer3_dtifs2
    }
    #[doc = "0x1c8 - CC0 consumer register"]
    #[inline(always)]
    pub const fn consumer_timer4_cc0(&self) -> &ConsumerTimer4Cc0 {
        &self.consumer_timer4_cc0
    }
    #[doc = "0x1cc - CC1 Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer4_cc1(&self) -> &ConsumerTimer4Cc1 {
        &self.consumer_timer4_cc1
    }
    #[doc = "0x1d0 - CC2 Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer4_cc2(&self) -> &ConsumerTimer4Cc2 {
        &self.consumer_timer4_cc2
    }
    #[doc = "0x1d4 - DTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer4_dti(&self) -> &ConsumerTimer4Dti {
        &self.consumer_timer4_dti
    }
    #[doc = "0x1d8 - DTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer4_dtifs1(&self) -> &ConsumerTimer4Dtifs1 {
        &self.consumer_timer4_dtifs1
    }
    #[doc = "0x1dc - DTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer4_dtifs2(&self) -> &ConsumerTimer4Dtifs2 {
        &self.consumer_timer4_dtifs2
    }
    #[doc = "0x1e0 - CC0 consumer register"]
    #[inline(always)]
    pub const fn consumer_timer5_cc0(&self) -> &ConsumerTimer5Cc0 {
        &self.consumer_timer5_cc0
    }
    #[doc = "0x1e4 - CC1 Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer5_cc1(&self) -> &ConsumerTimer5Cc1 {
        &self.consumer_timer5_cc1
    }
    #[doc = "0x1e8 - CC2 Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer5_cc2(&self) -> &ConsumerTimer5Cc2 {
        &self.consumer_timer5_cc2
    }
    #[doc = "0x1ec - DTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer5_dti(&self) -> &ConsumerTimer5Dti {
        &self.consumer_timer5_dti
    }
    #[doc = "0x1f0 - DTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer5_dtifs1(&self) -> &ConsumerTimer5Dtifs1 {
        &self.consumer_timer5_dtifs1
    }
    #[doc = "0x1f4 - DTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer5_dtifs2(&self) -> &ConsumerTimer5Dtifs2 {
        &self.consumer_timer5_dtifs2
    }
    #[doc = "0x1f8 - CC0 consumer register"]
    #[inline(always)]
    pub const fn consumer_timer6_cc0(&self) -> &ConsumerTimer6Cc0 {
        &self.consumer_timer6_cc0
    }
    #[doc = "0x1fc - CC1 Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer6_cc1(&self) -> &ConsumerTimer6Cc1 {
        &self.consumer_timer6_cc1
    }
    #[doc = "0x200 - CC2 Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer6_cc2(&self) -> &ConsumerTimer6Cc2 {
        &self.consumer_timer6_cc2
    }
    #[doc = "0x204 - DTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer6_dti(&self) -> &ConsumerTimer6Dti {
        &self.consumer_timer6_dti
    }
    #[doc = "0x208 - DTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer6_dtifs1(&self) -> &ConsumerTimer6Dtifs1 {
        &self.consumer_timer6_dtifs1
    }
    #[doc = "0x20c - DTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer6_dtifs2(&self) -> &ConsumerTimer6Dtifs2 {
        &self.consumer_timer6_dtifs2
    }
    #[doc = "0x210 - CC0 consumer register"]
    #[inline(always)]
    pub const fn consumer_timer7_cc0(&self) -> &ConsumerTimer7Cc0 {
        &self.consumer_timer7_cc0
    }
    #[doc = "0x214 - CC1 Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer7_cc1(&self) -> &ConsumerTimer7Cc1 {
        &self.consumer_timer7_cc1
    }
    #[doc = "0x218 - CC2 Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer7_cc2(&self) -> &ConsumerTimer7Cc2 {
        &self.consumer_timer7_cc2
    }
    #[doc = "0x21c - DTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer7_dti(&self) -> &ConsumerTimer7Dti {
        &self.consumer_timer7_dti
    }
    #[doc = "0x220 - DTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer7_dtifs1(&self) -> &ConsumerTimer7Dtifs1 {
        &self.consumer_timer7_dtifs1
    }
    #[doc = "0x224 - DTI Consumer register"]
    #[inline(always)]
    pub const fn consumer_timer7_dtifs2(&self) -> &ConsumerTimer7Dtifs2 {
        &self.consumer_timer7_dtifs2
    }
    #[doc = "0x228 - ASYNCTRIG consumer register"]
    #[inline(always)]
    pub const fn consumer_vdac0_asynctrigch0(&self) -> &ConsumerVdac0Asynctrigch0 {
        &self.consumer_vdac0_asynctrigch0
    }
    #[doc = "0x22c - ASYNCTRIG Consumer register"]
    #[inline(always)]
    pub const fn consumer_vdac0_asynctrigch1(&self) -> &ConsumerVdac0Asynctrigch1 {
        &self.consumer_vdac0_asynctrigch1
    }
    #[doc = "0x230 - SYNCTRIG Consumer register"]
    #[inline(always)]
    pub const fn consumer_vdac0_synctrigch0(&self) -> &ConsumerVdac0Synctrigch0 {
        &self.consumer_vdac0_synctrigch0
    }
    #[doc = "0x234 - SYNCTRIG Consumer register"]
    #[inline(always)]
    pub const fn consumer_vdac0_synctrigch1(&self) -> &ConsumerVdac0Synctrigch1 {
        &self.consumer_vdac0_synctrigch1
    }
    #[doc = "0x238 - SRC0 consumer register"]
    #[inline(always)]
    pub const fn consumer_wdog0_src0(&self) -> &ConsumerWdog0Src0 {
        &self.consumer_wdog0_src0
    }
    #[doc = "0x23c - SRC1 Consumer register"]
    #[inline(always)]
    pub const fn consumer_wdog0_src1(&self) -> &ConsumerWdog0Src1 {
        &self.consumer_wdog0_src1
    }
    #[doc = "0x240 - SRC0 consumer register"]
    #[inline(always)]
    pub const fn consumer_wdog1_src0(&self) -> &ConsumerWdog1Src0 {
        &self.consumer_wdog1_src0
    }
    #[doc = "0x244 - SRC1 Consumer register"]
    #[inline(always)]
    pub const fn consumer_wdog1_src1(&self) -> &ConsumerWdog1Src1 {
        &self.consumer_wdog1_src1
    }
}
#[doc = "IPVERSION (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ipversion::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipversion`] module"]
#[doc(alias = "IPVERSION")]
pub type Ipversion = crate::Reg<ipversion::IpversionSpec>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "ASYNC_SWPULSE (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_swpulse::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_swpulse`] module"]
#[doc(alias = "ASYNC_SWPULSE")]
pub type AsyncSwpulse = crate::Reg<async_swpulse::AsyncSwpulseSpec>;
#[doc = "No Description"]
pub mod async_swpulse;
#[doc = "ASYNC_SWLEVEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_swlevel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_swlevel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_swlevel`] module"]
#[doc(alias = "ASYNC_SWLEVEL")]
pub type AsyncSwlevel = crate::Reg<async_swlevel::AsyncSwlevelSpec>;
#[doc = "No Description"]
pub mod async_swlevel;
#[doc = "ASYNC_PEEK (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_peek::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_peek`] module"]
#[doc(alias = "ASYNC_PEEK")]
pub type AsyncPeek = crate::Reg<async_peek::AsyncPeekSpec>;
#[doc = "No Description"]
pub mod async_peek;
#[doc = "SYNC_PEEK (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_peek::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync_peek`] module"]
#[doc(alias = "SYNC_PEEK")]
pub type SyncPeek = crate::Reg<sync_peek::SyncPeekSpec>;
#[doc = "No Description"]
pub mod sync_peek;
#[doc = "ASYNC_CH0_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_ch0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_ch0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_ch0_ctrl`] module"]
#[doc(alias = "ASYNC_CH0_CTRL")]
pub type AsyncCh0Ctrl = crate::Reg<async_ch0_ctrl::AsyncCh0CtrlSpec>;
#[doc = "No Description"]
pub mod async_ch0_ctrl;
#[doc = "ASYNC_CH1_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_ch1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_ch1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_ch1_ctrl`] module"]
#[doc(alias = "ASYNC_CH1_CTRL")]
pub type AsyncCh1Ctrl = crate::Reg<async_ch1_ctrl::AsyncCh1CtrlSpec>;
#[doc = "No Description"]
pub mod async_ch1_ctrl;
#[doc = "ASYNC_CH2_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_ch2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_ch2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_ch2_ctrl`] module"]
#[doc(alias = "ASYNC_CH2_CTRL")]
pub type AsyncCh2Ctrl = crate::Reg<async_ch2_ctrl::AsyncCh2CtrlSpec>;
#[doc = "No Description"]
pub mod async_ch2_ctrl;
#[doc = "ASYNC_CH3_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_ch3_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_ch3_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_ch3_ctrl`] module"]
#[doc(alias = "ASYNC_CH3_CTRL")]
pub type AsyncCh3Ctrl = crate::Reg<async_ch3_ctrl::AsyncCh3CtrlSpec>;
#[doc = "No Description"]
pub mod async_ch3_ctrl;
#[doc = "ASYNC_CH4_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_ch4_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_ch4_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_ch4_ctrl`] module"]
#[doc(alias = "ASYNC_CH4_CTRL")]
pub type AsyncCh4Ctrl = crate::Reg<async_ch4_ctrl::AsyncCh4CtrlSpec>;
#[doc = "No Description"]
pub mod async_ch4_ctrl;
#[doc = "ASYNC_CH5_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_ch5_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_ch5_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_ch5_ctrl`] module"]
#[doc(alias = "ASYNC_CH5_CTRL")]
pub type AsyncCh5Ctrl = crate::Reg<async_ch5_ctrl::AsyncCh5CtrlSpec>;
#[doc = "No Description"]
pub mod async_ch5_ctrl;
#[doc = "ASYNC_CH6_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_ch6_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_ch6_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_ch6_ctrl`] module"]
#[doc(alias = "ASYNC_CH6_CTRL")]
pub type AsyncCh6Ctrl = crate::Reg<async_ch6_ctrl::AsyncCh6CtrlSpec>;
#[doc = "No Description"]
pub mod async_ch6_ctrl;
#[doc = "ASYNC_CH7_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_ch7_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_ch7_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_ch7_ctrl`] module"]
#[doc(alias = "ASYNC_CH7_CTRL")]
pub type AsyncCh7Ctrl = crate::Reg<async_ch7_ctrl::AsyncCh7CtrlSpec>;
#[doc = "No Description"]
pub mod async_ch7_ctrl;
#[doc = "ASYNC_CH8_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_ch8_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_ch8_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_ch8_ctrl`] module"]
#[doc(alias = "ASYNC_CH8_CTRL")]
pub type AsyncCh8Ctrl = crate::Reg<async_ch8_ctrl::AsyncCh8CtrlSpec>;
#[doc = "No Description"]
pub mod async_ch8_ctrl;
#[doc = "ASYNC_CH9_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_ch9_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_ch9_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_ch9_ctrl`] module"]
#[doc(alias = "ASYNC_CH9_CTRL")]
pub type AsyncCh9Ctrl = crate::Reg<async_ch9_ctrl::AsyncCh9CtrlSpec>;
#[doc = "No Description"]
pub mod async_ch9_ctrl;
#[doc = "ASYNC_CH10_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_ch10_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_ch10_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_ch10_ctrl`] module"]
#[doc(alias = "ASYNC_CH10_CTRL")]
pub type AsyncCh10Ctrl = crate::Reg<async_ch10_ctrl::AsyncCh10CtrlSpec>;
#[doc = "No Description"]
pub mod async_ch10_ctrl;
#[doc = "ASYNC_CH11_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_ch11_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_ch11_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_ch11_ctrl`] module"]
#[doc(alias = "ASYNC_CH11_CTRL")]
pub type AsyncCh11Ctrl = crate::Reg<async_ch11_ctrl::AsyncCh11CtrlSpec>;
#[doc = "No Description"]
pub mod async_ch11_ctrl;
#[doc = "SYNC_CH0_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_ch0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync_ch0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync_ch0_ctrl`] module"]
#[doc(alias = "SYNC_CH0_CTRL")]
pub type SyncCh0Ctrl = crate::Reg<sync_ch0_ctrl::SyncCh0CtrlSpec>;
#[doc = "No Description"]
pub mod sync_ch0_ctrl;
#[doc = "SYNC_CH1_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_ch1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync_ch1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync_ch1_ctrl`] module"]
#[doc(alias = "SYNC_CH1_CTRL")]
pub type SyncCh1Ctrl = crate::Reg<sync_ch1_ctrl::SyncCh1CtrlSpec>;
#[doc = "No Description"]
pub mod sync_ch1_ctrl;
#[doc = "SYNC_CH2_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_ch2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync_ch2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync_ch2_ctrl`] module"]
#[doc(alias = "SYNC_CH2_CTRL")]
pub type SyncCh2Ctrl = crate::Reg<sync_ch2_ctrl::SyncCh2CtrlSpec>;
#[doc = "No Description"]
pub mod sync_ch2_ctrl;
#[doc = "SYNC_CH3_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_ch3_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync_ch3_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync_ch3_ctrl`] module"]
#[doc(alias = "SYNC_CH3_CTRL")]
pub type SyncCh3Ctrl = crate::Reg<sync_ch3_ctrl::SyncCh3CtrlSpec>;
#[doc = "No Description"]
pub mod sync_ch3_ctrl;
#[doc = "CONSUMER_CMU_CALDN (rw) register accessor: CALDN consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_cmu_caldn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_cmu_caldn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_cmu_caldn`] module"]
#[doc(alias = "CONSUMER_CMU_CALDN")]
pub type ConsumerCmuCaldn = crate::Reg<consumer_cmu_caldn::ConsumerCmuCaldnSpec>;
#[doc = "CALDN consumer register"]
pub mod consumer_cmu_caldn;
#[doc = "CONSUMER_CMU_CALUP (rw) register accessor: CALUP Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_cmu_calup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_cmu_calup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_cmu_calup`] module"]
#[doc(alias = "CONSUMER_CMU_CALUP")]
pub type ConsumerCmuCalup = crate::Reg<consumer_cmu_calup::ConsumerCmuCalupSpec>;
#[doc = "CALUP Consumer register"]
pub mod consumer_cmu_calup;
#[doc = "CONSUMER_EUSART0_CLK (rw) register accessor: CLK consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_eusart0_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_eusart0_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_eusart0_clk`] module"]
#[doc(alias = "CONSUMER_EUSART0_CLK")]
pub type ConsumerEusart0Clk = crate::Reg<consumer_eusart0_clk::ConsumerEusart0ClkSpec>;
#[doc = "CLK consumer register"]
pub mod consumer_eusart0_clk;
#[doc = "CONSUMER_EUSART0_RX (rw) register accessor: RX Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_eusart0_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_eusart0_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_eusart0_rx`] module"]
#[doc(alias = "CONSUMER_EUSART0_RX")]
pub type ConsumerEusart0Rx = crate::Reg<consumer_eusart0_rx::ConsumerEusart0RxSpec>;
#[doc = "RX Consumer register"]
pub mod consumer_eusart0_rx;
#[doc = "CONSUMER_EUSART0_TRIGGER (rw) register accessor: TRIGGER Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_eusart0_trigger::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_eusart0_trigger::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_eusart0_trigger`] module"]
#[doc(alias = "CONSUMER_EUSART0_TRIGGER")]
pub type ConsumerEusart0Trigger = crate::Reg<consumer_eusart0_trigger::ConsumerEusart0TriggerSpec>;
#[doc = "TRIGGER Consumer register"]
pub mod consumer_eusart0_trigger;
#[doc = "CONSUMER_EUSART1_CLK (rw) register accessor: CLK consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_eusart1_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_eusart1_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_eusart1_clk`] module"]
#[doc(alias = "CONSUMER_EUSART1_CLK")]
pub type ConsumerEusart1Clk = crate::Reg<consumer_eusart1_clk::ConsumerEusart1ClkSpec>;
#[doc = "CLK consumer register"]
pub mod consumer_eusart1_clk;
#[doc = "CONSUMER_EUSART1_RX (rw) register accessor: RX Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_eusart1_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_eusart1_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_eusart1_rx`] module"]
#[doc(alias = "CONSUMER_EUSART1_RX")]
pub type ConsumerEusart1Rx = crate::Reg<consumer_eusart1_rx::ConsumerEusart1RxSpec>;
#[doc = "RX Consumer register"]
pub mod consumer_eusart1_rx;
#[doc = "CONSUMER_EUSART1_TRIGGER (rw) register accessor: TRIGGER Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_eusart1_trigger::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_eusart1_trigger::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_eusart1_trigger`] module"]
#[doc(alias = "CONSUMER_EUSART1_TRIGGER")]
pub type ConsumerEusart1Trigger = crate::Reg<consumer_eusart1_trigger::ConsumerEusart1TriggerSpec>;
#[doc = "TRIGGER Consumer register"]
pub mod consumer_eusart1_trigger;
#[doc = "CONSUMER_EUSART2_CLK (rw) register accessor: CLK consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_eusart2_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_eusart2_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_eusart2_clk`] module"]
#[doc(alias = "CONSUMER_EUSART2_CLK")]
pub type ConsumerEusart2Clk = crate::Reg<consumer_eusart2_clk::ConsumerEusart2ClkSpec>;
#[doc = "CLK consumer register"]
pub mod consumer_eusart2_clk;
#[doc = "CONSUMER_EUSART2_RX (rw) register accessor: RX Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_eusart2_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_eusart2_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_eusart2_rx`] module"]
#[doc(alias = "CONSUMER_EUSART2_RX")]
pub type ConsumerEusart2Rx = crate::Reg<consumer_eusart2_rx::ConsumerEusart2RxSpec>;
#[doc = "RX Consumer register"]
pub mod consumer_eusart2_rx;
#[doc = "CONSUMER_EUSART2_TRIGGER (rw) register accessor: TRIGGER Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_eusart2_trigger::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_eusart2_trigger::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_eusart2_trigger`] module"]
#[doc(alias = "CONSUMER_EUSART2_TRIGGER")]
pub type ConsumerEusart2Trigger = crate::Reg<consumer_eusart2_trigger::ConsumerEusart2TriggerSpec>;
#[doc = "TRIGGER Consumer register"]
pub mod consumer_eusart2_trigger;
#[doc = "CONSUMER_EUSART3_CLK (rw) register accessor: CLK consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_eusart3_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_eusart3_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_eusart3_clk`] module"]
#[doc(alias = "CONSUMER_EUSART3_CLK")]
pub type ConsumerEusart3Clk = crate::Reg<consumer_eusart3_clk::ConsumerEusart3ClkSpec>;
#[doc = "CLK consumer register"]
pub mod consumer_eusart3_clk;
#[doc = "CONSUMER_EUSART3_RX (rw) register accessor: RX Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_eusart3_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_eusart3_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_eusart3_rx`] module"]
#[doc(alias = "CONSUMER_EUSART3_RX")]
pub type ConsumerEusart3Rx = crate::Reg<consumer_eusart3_rx::ConsumerEusart3RxSpec>;
#[doc = "RX Consumer register"]
pub mod consumer_eusart3_rx;
#[doc = "CONSUMER_EUSART3_TRIGGER (rw) register accessor: TRIGGER Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_eusart3_trigger::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_eusart3_trigger::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_eusart3_trigger`] module"]
#[doc(alias = "CONSUMER_EUSART3_TRIGGER")]
pub type ConsumerEusart3Trigger = crate::Reg<consumer_eusart3_trigger::ConsumerEusart3TriggerSpec>;
#[doc = "TRIGGER Consumer register"]
pub mod consumer_eusart3_trigger;
#[doc = "CONSUMER_EUSART4_CLK (rw) register accessor: CLK consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_eusart4_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_eusart4_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_eusart4_clk`] module"]
#[doc(alias = "CONSUMER_EUSART4_CLK")]
pub type ConsumerEusart4Clk = crate::Reg<consumer_eusart4_clk::ConsumerEusart4ClkSpec>;
#[doc = "CLK consumer register"]
pub mod consumer_eusart4_clk;
#[doc = "CONSUMER_EUSART4_RX (rw) register accessor: RX Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_eusart4_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_eusart4_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_eusart4_rx`] module"]
#[doc(alias = "CONSUMER_EUSART4_RX")]
pub type ConsumerEusart4Rx = crate::Reg<consumer_eusart4_rx::ConsumerEusart4RxSpec>;
#[doc = "RX Consumer register"]
pub mod consumer_eusart4_rx;
#[doc = "CONSUMER_EUSART4_TRIGGER (rw) register accessor: TRIGGER Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_eusart4_trigger::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_eusart4_trigger::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_eusart4_trigger`] module"]
#[doc(alias = "CONSUMER_EUSART4_TRIGGER")]
pub type ConsumerEusart4Trigger = crate::Reg<consumer_eusart4_trigger::ConsumerEusart4TriggerSpec>;
#[doc = "TRIGGER Consumer register"]
pub mod consumer_eusart4_trigger;
#[doc = "CONSUMER_FRC_RXRAW (rw) register accessor: RXRAW consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_frc_rxraw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_frc_rxraw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_frc_rxraw`] module"]
#[doc(alias = "CONSUMER_FRC_RXRAW")]
pub type ConsumerFrcRxraw = crate::Reg<consumer_frc_rxraw::ConsumerFrcRxrawSpec>;
#[doc = "RXRAW consumer register"]
pub mod consumer_frc_rxraw;
#[doc = "CONSUMER_IADC0_SCANTRIGGER (rw) register accessor: SCAN consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_iadc0_scantrigger::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_iadc0_scantrigger::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_iadc0_scantrigger`] module"]
#[doc(alias = "CONSUMER_IADC0_SCANTRIGGER")]
pub type ConsumerIadc0Scantrigger =
    crate::Reg<consumer_iadc0_scantrigger::ConsumerIadc0ScantriggerSpec>;
#[doc = "SCAN consumer register"]
pub mod consumer_iadc0_scantrigger;
#[doc = "CONSUMER_IADC0_SINGLETRIGGER (rw) register accessor: SINGLE Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_iadc0_singletrigger::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_iadc0_singletrigger::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_iadc0_singletrigger`] module"]
#[doc(alias = "CONSUMER_IADC0_SINGLETRIGGER")]
pub type ConsumerIadc0Singletrigger =
    crate::Reg<consumer_iadc0_singletrigger::ConsumerIadc0SingletriggerSpec>;
#[doc = "SINGLE Consumer register"]
pub mod consumer_iadc0_singletrigger;
#[doc = "CONSUMER_LDMAXBAR_DMAREQ0 (rw) register accessor: DMAREQ0 consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_ldmaxbar_dmareq0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_ldmaxbar_dmareq0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_ldmaxbar_dmareq0`] module"]
#[doc(alias = "CONSUMER_LDMAXBAR_DMAREQ0")]
pub type ConsumerLdmaxbarDmareq0 =
    crate::Reg<consumer_ldmaxbar_dmareq0::ConsumerLdmaxbarDmareq0Spec>;
#[doc = "DMAREQ0 consumer register"]
pub mod consumer_ldmaxbar_dmareq0;
#[doc = "CONSUMER_LDMAXBAR_DMAREQ1 (rw) register accessor: DMAREQ1 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_ldmaxbar_dmareq1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_ldmaxbar_dmareq1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_ldmaxbar_dmareq1`] module"]
#[doc(alias = "CONSUMER_LDMAXBAR_DMAREQ1")]
pub type ConsumerLdmaxbarDmareq1 =
    crate::Reg<consumer_ldmaxbar_dmareq1::ConsumerLdmaxbarDmareq1Spec>;
#[doc = "DMAREQ1 Consumer register"]
pub mod consumer_ldmaxbar_dmareq1;
#[doc = "CONSUMER_LESENSE_DECIN0 (rw) register accessor: DECIN0 consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_lesense_decin0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_lesense_decin0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_lesense_decin0`] module"]
#[doc(alias = "CONSUMER_LESENSE_DECIN0")]
pub type ConsumerLesenseDecin0 = crate::Reg<consumer_lesense_decin0::ConsumerLesenseDecin0Spec>;
#[doc = "DECIN0 consumer register"]
pub mod consumer_lesense_decin0;
#[doc = "CONSUMER_LESENSE_DECIN1 (rw) register accessor: DECIN1 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_lesense_decin1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_lesense_decin1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_lesense_decin1`] module"]
#[doc(alias = "CONSUMER_LESENSE_DECIN1")]
pub type ConsumerLesenseDecin1 = crate::Reg<consumer_lesense_decin1::ConsumerLesenseDecin1Spec>;
#[doc = "DECIN1 Consumer register"]
pub mod consumer_lesense_decin1;
#[doc = "CONSUMER_LESENSE_DECIN2 (rw) register accessor: DECIN2 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_lesense_decin2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_lesense_decin2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_lesense_decin2`] module"]
#[doc(alias = "CONSUMER_LESENSE_DECIN2")]
pub type ConsumerLesenseDecin2 = crate::Reg<consumer_lesense_decin2::ConsumerLesenseDecin2Spec>;
#[doc = "DECIN2 Consumer register"]
pub mod consumer_lesense_decin2;
#[doc = "CONSUMER_LESENSE_DECIN3 (rw) register accessor: DECIN3 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_lesense_decin3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_lesense_decin3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_lesense_decin3`] module"]
#[doc(alias = "CONSUMER_LESENSE_DECIN3")]
pub type ConsumerLesenseDecin3 = crate::Reg<consumer_lesense_decin3::ConsumerLesenseDecin3Spec>;
#[doc = "DECIN3 Consumer register"]
pub mod consumer_lesense_decin3;
#[doc = "CONSUMER_LESENSE_START (rw) register accessor: START Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_lesense_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_lesense_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_lesense_start`] module"]
#[doc(alias = "CONSUMER_LESENSE_START")]
pub type ConsumerLesenseStart = crate::Reg<consumer_lesense_start::ConsumerLesenseStartSpec>;
#[doc = "START Consumer register"]
pub mod consumer_lesense_start;
#[doc = "CONSUMER_LETIMER0_CLEAR (rw) register accessor: CLEAR consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_letimer0_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_letimer0_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_letimer0_clear`] module"]
#[doc(alias = "CONSUMER_LETIMER0_CLEAR")]
pub type ConsumerLetimer0Clear = crate::Reg<consumer_letimer0_clear::ConsumerLetimer0ClearSpec>;
#[doc = "CLEAR consumer register"]
pub mod consumer_letimer0_clear;
#[doc = "CONSUMER_LETIMER0_START (rw) register accessor: START Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_letimer0_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_letimer0_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_letimer0_start`] module"]
#[doc(alias = "CONSUMER_LETIMER0_START")]
pub type ConsumerLetimer0Start = crate::Reg<consumer_letimer0_start::ConsumerLetimer0StartSpec>;
#[doc = "START Consumer register"]
pub mod consumer_letimer0_start;
#[doc = "CONSUMER_LETIMER0_STOP (rw) register accessor: STOP Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_letimer0_stop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_letimer0_stop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_letimer0_stop`] module"]
#[doc(alias = "CONSUMER_LETIMER0_STOP")]
pub type ConsumerLetimer0Stop = crate::Reg<consumer_letimer0_stop::ConsumerLetimer0StopSpec>;
#[doc = "STOP Consumer register"]
pub mod consumer_letimer0_stop;
#[doc = "CONSUMER_MODEM_DIN (rw) register accessor: DIN consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_modem_din::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_modem_din::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_modem_din`] module"]
#[doc(alias = "CONSUMER_MODEM_DIN")]
pub type ConsumerModemDin = crate::Reg<consumer_modem_din::ConsumerModemDinSpec>;
#[doc = "DIN consumer register"]
pub mod consumer_modem_din;
#[doc = "CONSUMER_PCNT0_S0IN (rw) register accessor: S0IN consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_pcnt0_s0in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_pcnt0_s0in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_pcnt0_s0in`] module"]
#[doc(alias = "CONSUMER_PCNT0_S0IN")]
pub type ConsumerPcnt0S0in = crate::Reg<consumer_pcnt0_s0in::ConsumerPcnt0S0inSpec>;
#[doc = "S0IN consumer register"]
pub mod consumer_pcnt0_s0in;
#[doc = "CONSUMER_PCNT0_S1IN (rw) register accessor: S1IN Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_pcnt0_s1in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_pcnt0_s1in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_pcnt0_s1in`] module"]
#[doc(alias = "CONSUMER_PCNT0_S1IN")]
pub type ConsumerPcnt0S1in = crate::Reg<consumer_pcnt0_s1in::ConsumerPcnt0S1inSpec>;
#[doc = "S1IN Consumer register"]
pub mod consumer_pcnt0_s1in;
#[doc = "CONSUMER_RAC_CLR (rw) register accessor: CLR consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_rac_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_rac_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_rac_clr`] module"]
#[doc(alias = "CONSUMER_RAC_CLR")]
pub type ConsumerRacClr = crate::Reg<consumer_rac_clr::ConsumerRacClrSpec>;
#[doc = "CLR consumer register"]
pub mod consumer_rac_clr;
#[doc = "CONSUMER_RAC_CTIIN0 (rw) register accessor: CTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_rac_ctiin0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_rac_ctiin0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_rac_ctiin0`] module"]
#[doc(alias = "CONSUMER_RAC_CTIIN0")]
pub type ConsumerRacCtiin0 = crate::Reg<consumer_rac_ctiin0::ConsumerRacCtiin0Spec>;
#[doc = "CTI Consumer register"]
pub mod consumer_rac_ctiin0;
#[doc = "CONSUMER_RAC_CTIIN1 (rw) register accessor: CTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_rac_ctiin1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_rac_ctiin1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_rac_ctiin1`] module"]
#[doc(alias = "CONSUMER_RAC_CTIIN1")]
pub type ConsumerRacCtiin1 = crate::Reg<consumer_rac_ctiin1::ConsumerRacCtiin1Spec>;
#[doc = "CTI Consumer register"]
pub mod consumer_rac_ctiin1;
#[doc = "CONSUMER_RAC_CTIIN2 (rw) register accessor: CTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_rac_ctiin2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_rac_ctiin2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_rac_ctiin2`] module"]
#[doc(alias = "CONSUMER_RAC_CTIIN2")]
pub type ConsumerRacCtiin2 = crate::Reg<consumer_rac_ctiin2::ConsumerRacCtiin2Spec>;
#[doc = "CTI Consumer register"]
pub mod consumer_rac_ctiin2;
#[doc = "CONSUMER_RAC_CTIIN3 (rw) register accessor: CTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_rac_ctiin3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_rac_ctiin3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_rac_ctiin3`] module"]
#[doc(alias = "CONSUMER_RAC_CTIIN3")]
pub type ConsumerRacCtiin3 = crate::Reg<consumer_rac_ctiin3::ConsumerRacCtiin3Spec>;
#[doc = "CTI Consumer register"]
pub mod consumer_rac_ctiin3;
#[doc = "CONSUMER_RAC_FORCETX (rw) register accessor: FORCETX Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_rac_forcetx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_rac_forcetx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_rac_forcetx`] module"]
#[doc(alias = "CONSUMER_RAC_FORCETX")]
pub type ConsumerRacForcetx = crate::Reg<consumer_rac_forcetx::ConsumerRacForcetxSpec>;
#[doc = "FORCETX Consumer register"]
pub mod consumer_rac_forcetx;
#[doc = "CONSUMER_RAC_RXDIS (rw) register accessor: RXDIS Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_rac_rxdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_rac_rxdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_rac_rxdis`] module"]
#[doc(alias = "CONSUMER_RAC_RXDIS")]
pub type ConsumerRacRxdis = crate::Reg<consumer_rac_rxdis::ConsumerRacRxdisSpec>;
#[doc = "RXDIS Consumer register"]
pub mod consumer_rac_rxdis;
#[doc = "CONSUMER_RAC_RXEN (rw) register accessor: RXEN Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_rac_rxen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_rac_rxen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_rac_rxen`] module"]
#[doc(alias = "CONSUMER_RAC_RXEN")]
pub type ConsumerRacRxen = crate::Reg<consumer_rac_rxen::ConsumerRacRxenSpec>;
#[doc = "RXEN Consumer register"]
pub mod consumer_rac_rxen;
#[doc = "CONSUMER_RAC_TXEN (rw) register accessor: TXEN Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_rac_txen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_rac_txen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_rac_txen`] module"]
#[doc(alias = "CONSUMER_RAC_TXEN")]
pub type ConsumerRacTxen = crate::Reg<consumer_rac_txen::ConsumerRacTxenSpec>;
#[doc = "TXEN Consumer register"]
pub mod consumer_rac_txen;
#[doc = "CONSUMER_SETAMPER_TAMPERSRC26 (rw) register accessor: TAMPERSRC26 consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_setamper_tampersrc26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_setamper_tampersrc26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_setamper_tampersrc26`] module"]
#[doc(alias = "CONSUMER_SETAMPER_TAMPERSRC26")]
pub type ConsumerSetamperTampersrc26 =
    crate::Reg<consumer_setamper_tampersrc26::ConsumerSetamperTampersrc26Spec>;
#[doc = "TAMPERSRC26 consumer register"]
pub mod consumer_setamper_tampersrc26;
#[doc = "CONSUMER_SETAMPER_TAMPERSRC27 (rw) register accessor: TAMPERSRC27 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_setamper_tampersrc27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_setamper_tampersrc27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_setamper_tampersrc27`] module"]
#[doc(alias = "CONSUMER_SETAMPER_TAMPERSRC27")]
pub type ConsumerSetamperTampersrc27 =
    crate::Reg<consumer_setamper_tampersrc27::ConsumerSetamperTampersrc27Spec>;
#[doc = "TAMPERSRC27 Consumer register"]
pub mod consumer_setamper_tampersrc27;
#[doc = "CONSUMER_SETAMPER_TAMPERSRC28 (rw) register accessor: TAMPERSRC28 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_setamper_tampersrc28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_setamper_tampersrc28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_setamper_tampersrc28`] module"]
#[doc(alias = "CONSUMER_SETAMPER_TAMPERSRC28")]
pub type ConsumerSetamperTampersrc28 =
    crate::Reg<consumer_setamper_tampersrc28::ConsumerSetamperTampersrc28Spec>;
#[doc = "TAMPERSRC28 Consumer register"]
pub mod consumer_setamper_tampersrc28;
#[doc = "CONSUMER_SETAMPER_TAMPERSRC29 (rw) register accessor: TAMPERSRC29 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_setamper_tampersrc29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_setamper_tampersrc29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_setamper_tampersrc29`] module"]
#[doc(alias = "CONSUMER_SETAMPER_TAMPERSRC29")]
pub type ConsumerSetamperTampersrc29 =
    crate::Reg<consumer_setamper_tampersrc29::ConsumerSetamperTampersrc29Spec>;
#[doc = "TAMPERSRC29 Consumer register"]
pub mod consumer_setamper_tampersrc29;
#[doc = "CONSUMER_SETAMPER_TAMPERSRC30 (rw) register accessor: TAMPERSRC30 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_setamper_tampersrc30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_setamper_tampersrc30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_setamper_tampersrc30`] module"]
#[doc(alias = "CONSUMER_SETAMPER_TAMPERSRC30")]
pub type ConsumerSetamperTampersrc30 =
    crate::Reg<consumer_setamper_tampersrc30::ConsumerSetamperTampersrc30Spec>;
#[doc = "TAMPERSRC30 Consumer register"]
pub mod consumer_setamper_tampersrc30;
#[doc = "CONSUMER_SETAMPER_TAMPERSRC31 (rw) register accessor: TAMPERSRC31 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_setamper_tampersrc31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_setamper_tampersrc31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_setamper_tampersrc31`] module"]
#[doc(alias = "CONSUMER_SETAMPER_TAMPERSRC31")]
pub type ConsumerSetamperTampersrc31 =
    crate::Reg<consumer_setamper_tampersrc31::ConsumerSetamperTampersrc31Spec>;
#[doc = "TAMPERSRC31 Consumer register"]
pub mod consumer_setamper_tampersrc31;
#[doc = "CONSUMER_SYSRTC0_IN0 (rw) register accessor: IN0 consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_sysrtc0_in0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_sysrtc0_in0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_sysrtc0_in0`] module"]
#[doc(alias = "CONSUMER_SYSRTC0_IN0")]
pub type ConsumerSysrtc0In0 = crate::Reg<consumer_sysrtc0_in0::ConsumerSysrtc0In0Spec>;
#[doc = "IN0 consumer register"]
pub mod consumer_sysrtc0_in0;
#[doc = "CONSUMER_SYSRTC0_IN1 (rw) register accessor: IN1 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_sysrtc0_in1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_sysrtc0_in1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_sysrtc0_in1`] module"]
#[doc(alias = "CONSUMER_SYSRTC0_IN1")]
pub type ConsumerSysrtc0In1 = crate::Reg<consumer_sysrtc0_in1::ConsumerSysrtc0In1Spec>;
#[doc = "IN1 Consumer register"]
pub mod consumer_sysrtc0_in1;
#[doc = "CONSUMER_HFXO0_OSCREQ (rw) register accessor: OSCREQ consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_hfxo0_oscreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_hfxo0_oscreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_hfxo0_oscreq`] module"]
#[doc(alias = "CONSUMER_HFXO0_OSCREQ")]
pub type ConsumerHfxo0Oscreq = crate::Reg<consumer_hfxo0_oscreq::ConsumerHfxo0OscreqSpec>;
#[doc = "OSCREQ consumer register"]
pub mod consumer_hfxo0_oscreq;
#[doc = "CONSUMER_HFXO0_TIMEOUT (rw) register accessor: TIMEOUT Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_hfxo0_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_hfxo0_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_hfxo0_timeout`] module"]
#[doc(alias = "CONSUMER_HFXO0_TIMEOUT")]
pub type ConsumerHfxo0Timeout = crate::Reg<consumer_hfxo0_timeout::ConsumerHfxo0TimeoutSpec>;
#[doc = "TIMEOUT Consumer register"]
pub mod consumer_hfxo0_timeout;
#[doc = "CONSUMER_CORE_CTIIN0 (rw) register accessor: CTI consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_core_ctiin0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_core_ctiin0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_core_ctiin0`] module"]
#[doc(alias = "CONSUMER_CORE_CTIIN0")]
pub type ConsumerCoreCtiin0 = crate::Reg<consumer_core_ctiin0::ConsumerCoreCtiin0Spec>;
#[doc = "CTI consumer register"]
pub mod consumer_core_ctiin0;
#[doc = "CONSUMER_CORE_CTIIN1 (rw) register accessor: CTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_core_ctiin1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_core_ctiin1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_core_ctiin1`] module"]
#[doc(alias = "CONSUMER_CORE_CTIIN1")]
pub type ConsumerCoreCtiin1 = crate::Reg<consumer_core_ctiin1::ConsumerCoreCtiin1Spec>;
#[doc = "CTI Consumer register"]
pub mod consumer_core_ctiin1;
#[doc = "CONSUMER_CORE_CTIIN2 (rw) register accessor: CTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_core_ctiin2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_core_ctiin2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_core_ctiin2`] module"]
#[doc(alias = "CONSUMER_CORE_CTIIN2")]
pub type ConsumerCoreCtiin2 = crate::Reg<consumer_core_ctiin2::ConsumerCoreCtiin2Spec>;
#[doc = "CTI Consumer register"]
pub mod consumer_core_ctiin2;
#[doc = "CONSUMER_CORE_CTIIN3 (rw) register accessor: CTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_core_ctiin3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_core_ctiin3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_core_ctiin3`] module"]
#[doc(alias = "CONSUMER_CORE_CTIIN3")]
pub type ConsumerCoreCtiin3 = crate::Reg<consumer_core_ctiin3::ConsumerCoreCtiin3Spec>;
#[doc = "CTI Consumer register"]
pub mod consumer_core_ctiin3;
#[doc = "CONSUMER_CORE_M33RXEV (rw) register accessor: M33 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_core_m33rxev::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_core_m33rxev::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_core_m33rxev`] module"]
#[doc(alias = "CONSUMER_CORE_M33RXEV")]
pub type ConsumerCoreM33rxev = crate::Reg<consumer_core_m33rxev::ConsumerCoreM33rxevSpec>;
#[doc = "M33 Consumer register"]
pub mod consumer_core_m33rxev;
#[doc = "CONSUMER_TIMER0_CC0 (rw) register accessor: CC0 consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer0_cc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer0_cc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer0_cc0`] module"]
#[doc(alias = "CONSUMER_TIMER0_CC0")]
pub type ConsumerTimer0Cc0 = crate::Reg<consumer_timer0_cc0::ConsumerTimer0Cc0Spec>;
#[doc = "CC0 consumer register"]
pub mod consumer_timer0_cc0;
#[doc = "CONSUMER_TIMER0_CC1 (rw) register accessor: CC1 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer0_cc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer0_cc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer0_cc1`] module"]
#[doc(alias = "CONSUMER_TIMER0_CC1")]
pub type ConsumerTimer0Cc1 = crate::Reg<consumer_timer0_cc1::ConsumerTimer0Cc1Spec>;
#[doc = "CC1 Consumer register"]
pub mod consumer_timer0_cc1;
#[doc = "CONSUMER_TIMER0_CC2 (rw) register accessor: CC2 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer0_cc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer0_cc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer0_cc2`] module"]
#[doc(alias = "CONSUMER_TIMER0_CC2")]
pub type ConsumerTimer0Cc2 = crate::Reg<consumer_timer0_cc2::ConsumerTimer0Cc2Spec>;
#[doc = "CC2 Consumer register"]
pub mod consumer_timer0_cc2;
#[doc = "CONSUMER_TIMER0_DTI (rw) register accessor: DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer0_dti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer0_dti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer0_dti`] module"]
#[doc(alias = "CONSUMER_TIMER0_DTI")]
pub type ConsumerTimer0Dti = crate::Reg<consumer_timer0_dti::ConsumerTimer0DtiSpec>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer0_dti;
#[doc = "CONSUMER_TIMER0_DTIFS1 (rw) register accessor: DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer0_dtifs1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer0_dtifs1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer0_dtifs1`] module"]
#[doc(alias = "CONSUMER_TIMER0_DTIFS1")]
pub type ConsumerTimer0Dtifs1 = crate::Reg<consumer_timer0_dtifs1::ConsumerTimer0Dtifs1Spec>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer0_dtifs1;
#[doc = "CONSUMER_TIMER0_DTIFS2 (rw) register accessor: DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer0_dtifs2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer0_dtifs2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer0_dtifs2`] module"]
#[doc(alias = "CONSUMER_TIMER0_DTIFS2")]
pub type ConsumerTimer0Dtifs2 = crate::Reg<consumer_timer0_dtifs2::ConsumerTimer0Dtifs2Spec>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer0_dtifs2;
#[doc = "CONSUMER_TIMER1_CC0 (rw) register accessor: CC0 consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer1_cc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer1_cc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer1_cc0`] module"]
#[doc(alias = "CONSUMER_TIMER1_CC0")]
pub type ConsumerTimer1Cc0 = crate::Reg<consumer_timer1_cc0::ConsumerTimer1Cc0Spec>;
#[doc = "CC0 consumer register"]
pub mod consumer_timer1_cc0;
#[doc = "CONSUMER_TIMER1_CC1 (rw) register accessor: CC1 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer1_cc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer1_cc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer1_cc1`] module"]
#[doc(alias = "CONSUMER_TIMER1_CC1")]
pub type ConsumerTimer1Cc1 = crate::Reg<consumer_timer1_cc1::ConsumerTimer1Cc1Spec>;
#[doc = "CC1 Consumer register"]
pub mod consumer_timer1_cc1;
#[doc = "CONSUMER_TIMER1_CC2 (rw) register accessor: CC2 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer1_cc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer1_cc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer1_cc2`] module"]
#[doc(alias = "CONSUMER_TIMER1_CC2")]
pub type ConsumerTimer1Cc2 = crate::Reg<consumer_timer1_cc2::ConsumerTimer1Cc2Spec>;
#[doc = "CC2 Consumer register"]
pub mod consumer_timer1_cc2;
#[doc = "CONSUMER_TIMER1_DTI (rw) register accessor: DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer1_dti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer1_dti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer1_dti`] module"]
#[doc(alias = "CONSUMER_TIMER1_DTI")]
pub type ConsumerTimer1Dti = crate::Reg<consumer_timer1_dti::ConsumerTimer1DtiSpec>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer1_dti;
#[doc = "CONSUMER_TIMER1_DTIFS1 (rw) register accessor: DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer1_dtifs1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer1_dtifs1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer1_dtifs1`] module"]
#[doc(alias = "CONSUMER_TIMER1_DTIFS1")]
pub type ConsumerTimer1Dtifs1 = crate::Reg<consumer_timer1_dtifs1::ConsumerTimer1Dtifs1Spec>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer1_dtifs1;
#[doc = "CONSUMER_TIMER1_DTIFS2 (rw) register accessor: DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer1_dtifs2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer1_dtifs2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer1_dtifs2`] module"]
#[doc(alias = "CONSUMER_TIMER1_DTIFS2")]
pub type ConsumerTimer1Dtifs2 = crate::Reg<consumer_timer1_dtifs2::ConsumerTimer1Dtifs2Spec>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer1_dtifs2;
#[doc = "CONSUMER_TIMER2_CC0 (rw) register accessor: CC0 consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer2_cc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer2_cc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer2_cc0`] module"]
#[doc(alias = "CONSUMER_TIMER2_CC0")]
pub type ConsumerTimer2Cc0 = crate::Reg<consumer_timer2_cc0::ConsumerTimer2Cc0Spec>;
#[doc = "CC0 consumer register"]
pub mod consumer_timer2_cc0;
#[doc = "CONSUMER_TIMER2_CC1 (rw) register accessor: CC1 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer2_cc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer2_cc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer2_cc1`] module"]
#[doc(alias = "CONSUMER_TIMER2_CC1")]
pub type ConsumerTimer2Cc1 = crate::Reg<consumer_timer2_cc1::ConsumerTimer2Cc1Spec>;
#[doc = "CC1 Consumer register"]
pub mod consumer_timer2_cc1;
#[doc = "CONSUMER_TIMER2_CC2 (rw) register accessor: CC2 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer2_cc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer2_cc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer2_cc2`] module"]
#[doc(alias = "CONSUMER_TIMER2_CC2")]
pub type ConsumerTimer2Cc2 = crate::Reg<consumer_timer2_cc2::ConsumerTimer2Cc2Spec>;
#[doc = "CC2 Consumer register"]
pub mod consumer_timer2_cc2;
#[doc = "CONSUMER_TIMER2_DTI (rw) register accessor: DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer2_dti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer2_dti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer2_dti`] module"]
#[doc(alias = "CONSUMER_TIMER2_DTI")]
pub type ConsumerTimer2Dti = crate::Reg<consumer_timer2_dti::ConsumerTimer2DtiSpec>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer2_dti;
#[doc = "CONSUMER_TIMER2_DTIFS1 (rw) register accessor: DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer2_dtifs1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer2_dtifs1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer2_dtifs1`] module"]
#[doc(alias = "CONSUMER_TIMER2_DTIFS1")]
pub type ConsumerTimer2Dtifs1 = crate::Reg<consumer_timer2_dtifs1::ConsumerTimer2Dtifs1Spec>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer2_dtifs1;
#[doc = "CONSUMER_TIMER2_DTIFS2 (rw) register accessor: DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer2_dtifs2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer2_dtifs2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer2_dtifs2`] module"]
#[doc(alias = "CONSUMER_TIMER2_DTIFS2")]
pub type ConsumerTimer2Dtifs2 = crate::Reg<consumer_timer2_dtifs2::ConsumerTimer2Dtifs2Spec>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer2_dtifs2;
#[doc = "CONSUMER_TIMER3_CC0 (rw) register accessor: CC0 consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer3_cc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer3_cc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer3_cc0`] module"]
#[doc(alias = "CONSUMER_TIMER3_CC0")]
pub type ConsumerTimer3Cc0 = crate::Reg<consumer_timer3_cc0::ConsumerTimer3Cc0Spec>;
#[doc = "CC0 consumer register"]
pub mod consumer_timer3_cc0;
#[doc = "CONSUMER_TIMER3_CC1 (rw) register accessor: CC1 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer3_cc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer3_cc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer3_cc1`] module"]
#[doc(alias = "CONSUMER_TIMER3_CC1")]
pub type ConsumerTimer3Cc1 = crate::Reg<consumer_timer3_cc1::ConsumerTimer3Cc1Spec>;
#[doc = "CC1 Consumer register"]
pub mod consumer_timer3_cc1;
#[doc = "CONSUMER_TIMER3_CC2 (rw) register accessor: CC2 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer3_cc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer3_cc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer3_cc2`] module"]
#[doc(alias = "CONSUMER_TIMER3_CC2")]
pub type ConsumerTimer3Cc2 = crate::Reg<consumer_timer3_cc2::ConsumerTimer3Cc2Spec>;
#[doc = "CC2 Consumer register"]
pub mod consumer_timer3_cc2;
#[doc = "CONSUMER_TIMER3_DTI (rw) register accessor: DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer3_dti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer3_dti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer3_dti`] module"]
#[doc(alias = "CONSUMER_TIMER3_DTI")]
pub type ConsumerTimer3Dti = crate::Reg<consumer_timer3_dti::ConsumerTimer3DtiSpec>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer3_dti;
#[doc = "CONSUMER_TIMER3_DTIFS1 (rw) register accessor: DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer3_dtifs1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer3_dtifs1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer3_dtifs1`] module"]
#[doc(alias = "CONSUMER_TIMER3_DTIFS1")]
pub type ConsumerTimer3Dtifs1 = crate::Reg<consumer_timer3_dtifs1::ConsumerTimer3Dtifs1Spec>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer3_dtifs1;
#[doc = "CONSUMER_TIMER3_DTIFS2 (rw) register accessor: DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer3_dtifs2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer3_dtifs2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer3_dtifs2`] module"]
#[doc(alias = "CONSUMER_TIMER3_DTIFS2")]
pub type ConsumerTimer3Dtifs2 = crate::Reg<consumer_timer3_dtifs2::ConsumerTimer3Dtifs2Spec>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer3_dtifs2;
#[doc = "CONSUMER_TIMER4_CC0 (rw) register accessor: CC0 consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer4_cc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer4_cc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer4_cc0`] module"]
#[doc(alias = "CONSUMER_TIMER4_CC0")]
pub type ConsumerTimer4Cc0 = crate::Reg<consumer_timer4_cc0::ConsumerTimer4Cc0Spec>;
#[doc = "CC0 consumer register"]
pub mod consumer_timer4_cc0;
#[doc = "CONSUMER_TIMER4_CC1 (rw) register accessor: CC1 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer4_cc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer4_cc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer4_cc1`] module"]
#[doc(alias = "CONSUMER_TIMER4_CC1")]
pub type ConsumerTimer4Cc1 = crate::Reg<consumer_timer4_cc1::ConsumerTimer4Cc1Spec>;
#[doc = "CC1 Consumer register"]
pub mod consumer_timer4_cc1;
#[doc = "CONSUMER_TIMER4_CC2 (rw) register accessor: CC2 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer4_cc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer4_cc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer4_cc2`] module"]
#[doc(alias = "CONSUMER_TIMER4_CC2")]
pub type ConsumerTimer4Cc2 = crate::Reg<consumer_timer4_cc2::ConsumerTimer4Cc2Spec>;
#[doc = "CC2 Consumer register"]
pub mod consumer_timer4_cc2;
#[doc = "CONSUMER_TIMER4_DTI (rw) register accessor: DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer4_dti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer4_dti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer4_dti`] module"]
#[doc(alias = "CONSUMER_TIMER4_DTI")]
pub type ConsumerTimer4Dti = crate::Reg<consumer_timer4_dti::ConsumerTimer4DtiSpec>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer4_dti;
#[doc = "CONSUMER_TIMER4_DTIFS1 (rw) register accessor: DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer4_dtifs1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer4_dtifs1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer4_dtifs1`] module"]
#[doc(alias = "CONSUMER_TIMER4_DTIFS1")]
pub type ConsumerTimer4Dtifs1 = crate::Reg<consumer_timer4_dtifs1::ConsumerTimer4Dtifs1Spec>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer4_dtifs1;
#[doc = "CONSUMER_TIMER4_DTIFS2 (rw) register accessor: DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer4_dtifs2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer4_dtifs2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer4_dtifs2`] module"]
#[doc(alias = "CONSUMER_TIMER4_DTIFS2")]
pub type ConsumerTimer4Dtifs2 = crate::Reg<consumer_timer4_dtifs2::ConsumerTimer4Dtifs2Spec>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer4_dtifs2;
#[doc = "CONSUMER_TIMER5_CC0 (rw) register accessor: CC0 consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer5_cc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer5_cc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer5_cc0`] module"]
#[doc(alias = "CONSUMER_TIMER5_CC0")]
pub type ConsumerTimer5Cc0 = crate::Reg<consumer_timer5_cc0::ConsumerTimer5Cc0Spec>;
#[doc = "CC0 consumer register"]
pub mod consumer_timer5_cc0;
#[doc = "CONSUMER_TIMER5_CC1 (rw) register accessor: CC1 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer5_cc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer5_cc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer5_cc1`] module"]
#[doc(alias = "CONSUMER_TIMER5_CC1")]
pub type ConsumerTimer5Cc1 = crate::Reg<consumer_timer5_cc1::ConsumerTimer5Cc1Spec>;
#[doc = "CC1 Consumer register"]
pub mod consumer_timer5_cc1;
#[doc = "CONSUMER_TIMER5_CC2 (rw) register accessor: CC2 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer5_cc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer5_cc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer5_cc2`] module"]
#[doc(alias = "CONSUMER_TIMER5_CC2")]
pub type ConsumerTimer5Cc2 = crate::Reg<consumer_timer5_cc2::ConsumerTimer5Cc2Spec>;
#[doc = "CC2 Consumer register"]
pub mod consumer_timer5_cc2;
#[doc = "CONSUMER_TIMER5_DTI (rw) register accessor: DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer5_dti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer5_dti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer5_dti`] module"]
#[doc(alias = "CONSUMER_TIMER5_DTI")]
pub type ConsumerTimer5Dti = crate::Reg<consumer_timer5_dti::ConsumerTimer5DtiSpec>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer5_dti;
#[doc = "CONSUMER_TIMER5_DTIFS1 (rw) register accessor: DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer5_dtifs1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer5_dtifs1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer5_dtifs1`] module"]
#[doc(alias = "CONSUMER_TIMER5_DTIFS1")]
pub type ConsumerTimer5Dtifs1 = crate::Reg<consumer_timer5_dtifs1::ConsumerTimer5Dtifs1Spec>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer5_dtifs1;
#[doc = "CONSUMER_TIMER5_DTIFS2 (rw) register accessor: DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer5_dtifs2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer5_dtifs2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer5_dtifs2`] module"]
#[doc(alias = "CONSUMER_TIMER5_DTIFS2")]
pub type ConsumerTimer5Dtifs2 = crate::Reg<consumer_timer5_dtifs2::ConsumerTimer5Dtifs2Spec>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer5_dtifs2;
#[doc = "CONSUMER_TIMER6_CC0 (rw) register accessor: CC0 consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer6_cc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer6_cc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer6_cc0`] module"]
#[doc(alias = "CONSUMER_TIMER6_CC0")]
pub type ConsumerTimer6Cc0 = crate::Reg<consumer_timer6_cc0::ConsumerTimer6Cc0Spec>;
#[doc = "CC0 consumer register"]
pub mod consumer_timer6_cc0;
#[doc = "CONSUMER_TIMER6_CC1 (rw) register accessor: CC1 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer6_cc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer6_cc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer6_cc1`] module"]
#[doc(alias = "CONSUMER_TIMER6_CC1")]
pub type ConsumerTimer6Cc1 = crate::Reg<consumer_timer6_cc1::ConsumerTimer6Cc1Spec>;
#[doc = "CC1 Consumer register"]
pub mod consumer_timer6_cc1;
#[doc = "CONSUMER_TIMER6_CC2 (rw) register accessor: CC2 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer6_cc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer6_cc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer6_cc2`] module"]
#[doc(alias = "CONSUMER_TIMER6_CC2")]
pub type ConsumerTimer6Cc2 = crate::Reg<consumer_timer6_cc2::ConsumerTimer6Cc2Spec>;
#[doc = "CC2 Consumer register"]
pub mod consumer_timer6_cc2;
#[doc = "CONSUMER_TIMER6_DTI (rw) register accessor: DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer6_dti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer6_dti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer6_dti`] module"]
#[doc(alias = "CONSUMER_TIMER6_DTI")]
pub type ConsumerTimer6Dti = crate::Reg<consumer_timer6_dti::ConsumerTimer6DtiSpec>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer6_dti;
#[doc = "CONSUMER_TIMER6_DTIFS1 (rw) register accessor: DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer6_dtifs1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer6_dtifs1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer6_dtifs1`] module"]
#[doc(alias = "CONSUMER_TIMER6_DTIFS1")]
pub type ConsumerTimer6Dtifs1 = crate::Reg<consumer_timer6_dtifs1::ConsumerTimer6Dtifs1Spec>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer6_dtifs1;
#[doc = "CONSUMER_TIMER6_DTIFS2 (rw) register accessor: DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer6_dtifs2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer6_dtifs2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer6_dtifs2`] module"]
#[doc(alias = "CONSUMER_TIMER6_DTIFS2")]
pub type ConsumerTimer6Dtifs2 = crate::Reg<consumer_timer6_dtifs2::ConsumerTimer6Dtifs2Spec>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer6_dtifs2;
#[doc = "CONSUMER_TIMER7_CC0 (rw) register accessor: CC0 consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer7_cc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer7_cc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer7_cc0`] module"]
#[doc(alias = "CONSUMER_TIMER7_CC0")]
pub type ConsumerTimer7Cc0 = crate::Reg<consumer_timer7_cc0::ConsumerTimer7Cc0Spec>;
#[doc = "CC0 consumer register"]
pub mod consumer_timer7_cc0;
#[doc = "CONSUMER_TIMER7_CC1 (rw) register accessor: CC1 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer7_cc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer7_cc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer7_cc1`] module"]
#[doc(alias = "CONSUMER_TIMER7_CC1")]
pub type ConsumerTimer7Cc1 = crate::Reg<consumer_timer7_cc1::ConsumerTimer7Cc1Spec>;
#[doc = "CC1 Consumer register"]
pub mod consumer_timer7_cc1;
#[doc = "CONSUMER_TIMER7_CC2 (rw) register accessor: CC2 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer7_cc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer7_cc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer7_cc2`] module"]
#[doc(alias = "CONSUMER_TIMER7_CC2")]
pub type ConsumerTimer7Cc2 = crate::Reg<consumer_timer7_cc2::ConsumerTimer7Cc2Spec>;
#[doc = "CC2 Consumer register"]
pub mod consumer_timer7_cc2;
#[doc = "CONSUMER_TIMER7_DTI (rw) register accessor: DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer7_dti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer7_dti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer7_dti`] module"]
#[doc(alias = "CONSUMER_TIMER7_DTI")]
pub type ConsumerTimer7Dti = crate::Reg<consumer_timer7_dti::ConsumerTimer7DtiSpec>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer7_dti;
#[doc = "CONSUMER_TIMER7_DTIFS1 (rw) register accessor: DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer7_dtifs1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer7_dtifs1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer7_dtifs1`] module"]
#[doc(alias = "CONSUMER_TIMER7_DTIFS1")]
pub type ConsumerTimer7Dtifs1 = crate::Reg<consumer_timer7_dtifs1::ConsumerTimer7Dtifs1Spec>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer7_dtifs1;
#[doc = "CONSUMER_TIMER7_DTIFS2 (rw) register accessor: DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer7_dtifs2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer7_dtifs2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer7_dtifs2`] module"]
#[doc(alias = "CONSUMER_TIMER7_DTIFS2")]
pub type ConsumerTimer7Dtifs2 = crate::Reg<consumer_timer7_dtifs2::ConsumerTimer7Dtifs2Spec>;
#[doc = "DTI Consumer register"]
pub mod consumer_timer7_dtifs2;
#[doc = "CONSUMER_VDAC0_ASYNCTRIGCH0 (rw) register accessor: ASYNCTRIG consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_vdac0_asynctrigch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_vdac0_asynctrigch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_vdac0_asynctrigch0`] module"]
#[doc(alias = "CONSUMER_VDAC0_ASYNCTRIGCH0")]
pub type ConsumerVdac0Asynctrigch0 =
    crate::Reg<consumer_vdac0_asynctrigch0::ConsumerVdac0Asynctrigch0Spec>;
#[doc = "ASYNCTRIG consumer register"]
pub mod consumer_vdac0_asynctrigch0;
#[doc = "CONSUMER_VDAC0_ASYNCTRIGCH1 (rw) register accessor: ASYNCTRIG Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_vdac0_asynctrigch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_vdac0_asynctrigch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_vdac0_asynctrigch1`] module"]
#[doc(alias = "CONSUMER_VDAC0_ASYNCTRIGCH1")]
pub type ConsumerVdac0Asynctrigch1 =
    crate::Reg<consumer_vdac0_asynctrigch1::ConsumerVdac0Asynctrigch1Spec>;
#[doc = "ASYNCTRIG Consumer register"]
pub mod consumer_vdac0_asynctrigch1;
#[doc = "CONSUMER_VDAC0_SYNCTRIGCH0 (rw) register accessor: SYNCTRIG Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_vdac0_synctrigch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_vdac0_synctrigch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_vdac0_synctrigch0`] module"]
#[doc(alias = "CONSUMER_VDAC0_SYNCTRIGCH0")]
pub type ConsumerVdac0Synctrigch0 =
    crate::Reg<consumer_vdac0_synctrigch0::ConsumerVdac0Synctrigch0Spec>;
#[doc = "SYNCTRIG Consumer register"]
pub mod consumer_vdac0_synctrigch0;
#[doc = "CONSUMER_VDAC0_SYNCTRIGCH1 (rw) register accessor: SYNCTRIG Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_vdac0_synctrigch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_vdac0_synctrigch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_vdac0_synctrigch1`] module"]
#[doc(alias = "CONSUMER_VDAC0_SYNCTRIGCH1")]
pub type ConsumerVdac0Synctrigch1 =
    crate::Reg<consumer_vdac0_synctrigch1::ConsumerVdac0Synctrigch1Spec>;
#[doc = "SYNCTRIG Consumer register"]
pub mod consumer_vdac0_synctrigch1;
#[doc = "CONSUMER_WDOG0_SRC0 (rw) register accessor: SRC0 consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_wdog0_src0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_wdog0_src0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_wdog0_src0`] module"]
#[doc(alias = "CONSUMER_WDOG0_SRC0")]
pub type ConsumerWdog0Src0 = crate::Reg<consumer_wdog0_src0::ConsumerWdog0Src0Spec>;
#[doc = "SRC0 consumer register"]
pub mod consumer_wdog0_src0;
#[doc = "CONSUMER_WDOG0_SRC1 (rw) register accessor: SRC1 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_wdog0_src1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_wdog0_src1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_wdog0_src1`] module"]
#[doc(alias = "CONSUMER_WDOG0_SRC1")]
pub type ConsumerWdog0Src1 = crate::Reg<consumer_wdog0_src1::ConsumerWdog0Src1Spec>;
#[doc = "SRC1 Consumer register"]
pub mod consumer_wdog0_src1;
#[doc = "CONSUMER_WDOG1_SRC0 (rw) register accessor: SRC0 consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_wdog1_src0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_wdog1_src0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_wdog1_src0`] module"]
#[doc(alias = "CONSUMER_WDOG1_SRC0")]
pub type ConsumerWdog1Src0 = crate::Reg<consumer_wdog1_src0::ConsumerWdog1Src0Spec>;
#[doc = "SRC0 consumer register"]
pub mod consumer_wdog1_src0;
#[doc = "CONSUMER_WDOG1_SRC1 (rw) register accessor: SRC1 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_wdog1_src1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_wdog1_src1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_wdog1_src1`] module"]
#[doc(alias = "CONSUMER_WDOG1_SRC1")]
pub type ConsumerWdog1Src1 = crate::Reg<consumer_wdog1_src1::ConsumerWdog1Src1Spec>;
#[doc = "SRC1 Consumer register"]
pub mod consumer_wdog1_src1;
