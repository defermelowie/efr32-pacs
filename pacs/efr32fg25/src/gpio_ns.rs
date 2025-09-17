#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    _reserved1: [u8; 0x2c],
    porta_ctrl: PortaCtrl,
    porta_model: PortaModel,
    _reserved3: [u8; 0x04],
    porta_modeh: PortaModeh,
    porta_dout: PortaDout,
    porta_din: PortaDin,
    _reserved6: [u8; 0x18],
    portb_ctrl: PortbCtrl,
    portb_model: PortbModel,
    _reserved8: [u8; 0x04],
    portb_modeh: PortbModeh,
    portb_dout: PortbDout,
    portb_din: PortbDin,
    _reserved11: [u8; 0x18],
    portc_ctrl: PortcCtrl,
    portc_model: PortcModel,
    _reserved13: [u8; 0x04],
    portc_modeh: PortcModeh,
    portc_dout: PortcDout,
    portc_din: PortcDin,
    _reserved16: [u8; 0x18],
    portd_ctrl: PortdCtrl,
    portd_model: PortdModel,
    _reserved18: [u8; 0x08],
    portd_dout: PortdDout,
    portd_din: PortdDin,
    _reserved20: [u8; 0x0128],
    lock: Lock,
    _reserved21: [u8; 0x0c],
    gpiolockstatus: Gpiolockstatus,
    _reserved22: [u8; 0x0c],
    abusalloc: Abusalloc,
    bbusalloc: Bbusalloc,
    cdbusalloc: Cdbusalloc,
    _reserved25: [u8; 0x04],
    aodd0switch: Aodd0switch,
    aodd1switch: Aodd1switch,
    aeven0switch: Aeven0switch,
    aeven1switch: Aeven1switch,
    bodd0switch: Bodd0switch,
    bodd1switch: Bodd1switch,
    beven0switch: Beven0switch,
    beven1switch: Beven1switch,
    cdodd0switch: Cdodd0switch,
    cdodd1switch: Cdodd1switch,
    cdeven0switch: Cdeven0switch,
    cdeven1switch: Cdeven1switch,
    _reserved37: [u8; 0xa0],
    extipsell: Extipsell,
    extipselh: Extipselh,
    extipinsell: Extipinsell,
    extipinselh: Extipinselh,
    extirise: Extirise,
    extifall: Extifall,
    _reserved43: [u8; 0x08],
    if_: If,
    ien: Ien,
    _reserved45: [u8; 0x04],
    em4wuen: Em4wuen,
    em4wupol: Em4wupol,
    _reserved47: [u8; 0x0c],
    dbgroutepen: Dbgroutepen,
    traceroutepen: Traceroutepen,
    femroutepen: Femroutepen,
    _reserved50: [u8; 0x04],
    acmp0_routeen: Acmp0Routeen,
    acmp0_acmpoutroute: Acmp0Acmpoutroute,
    _reserved52: [u8; 0x04],
    acmp1_routeen: Acmp1Routeen,
    acmp1_acmpoutroute: Acmp1Acmpoutroute,
    _reserved54: [u8; 0x04],
    cmu_routeen: CmuRouteen,
    cmu_clkin0route: CmuClkin0route,
    cmu_clkout0route: CmuClkout0route,
    cmu_clkout1route: CmuClkout1route,
    cmu_clkout2route: CmuClkout2route,
    _reserved59: [u8; 0x08],
    dcdc_routeen: DcdcRouteen,
    _reserved60: [u8; 0x0c],
    eusart0_routeen: Eusart0Routeen,
    eusart0_csroute: Eusart0Csroute,
    eusart0_ctsroute: Eusart0Ctsroute,
    eusart0_rtsroute: Eusart0Rtsroute,
    eusart0_rxroute: Eusart0Rxroute,
    eusart0_sclkroute: Eusart0Sclkroute,
    eusart0_txroute: Eusart0Txroute,
    _reserved67: [u8; 0x04],
    eusart1_routeen: Eusart1Routeen,
    eusart1_csroute: Eusart1Csroute,
    eusart1_ctsroute: Eusart1Ctsroute,
    eusart1_rtsroute: Eusart1Rtsroute,
    eusart1_rxroute: Eusart1Rxroute,
    eusart1_sclkroute: Eusart1Sclkroute,
    eusart1_txroute: Eusart1Txroute,
    _reserved74: [u8; 0x04],
    eusart2_routeen: Eusart2Routeen,
    eusart2_csroute: Eusart2Csroute,
    eusart2_ctsroute: Eusart2Ctsroute,
    eusart2_rtsroute: Eusart2Rtsroute,
    eusart2_rxroute: Eusart2Rxroute,
    eusart2_sclkroute: Eusart2Sclkroute,
    eusart2_txroute: Eusart2Txroute,
    _reserved81: [u8; 0x04],
    eusart3_routeen: Eusart3Routeen,
    eusart3_csroute: Eusart3Csroute,
    eusart3_ctsroute: Eusart3Ctsroute,
    eusart3_rtsroute: Eusart3Rtsroute,
    eusart3_rxroute: Eusart3Rxroute,
    eusart3_sclkroute: Eusart3Sclkroute,
    eusart3_txroute: Eusart3Txroute,
    _reserved88: [u8; 0x04],
    eusart4_routeen: Eusart4Routeen,
    eusart4_csroute: Eusart4Csroute,
    eusart4_ctsroute: Eusart4Ctsroute,
    eusart4_rtsroute: Eusart4Rtsroute,
    eusart4_rxroute: Eusart4Rxroute,
    eusart4_sclkroute: Eusart4Sclkroute,
    eusart4_txroute: Eusart4Txroute,
    _reserved95: [u8; 0x04],
    frc_routeen: FrcRouteen,
    frc_dclkroute: FrcDclkroute,
    frc_dframeroute: FrcDframeroute,
    frc_doutroute: FrcDoutroute,
    _reserved99: [u8; 0x04],
    i2c0_routeen: I2c0Routeen,
    i2c0_sclroute: I2c0Sclroute,
    i2c0_sdaroute: I2c0Sdaroute,
    _reserved102: [u8; 0x04],
    i2c1_routeen: I2c1Routeen,
    i2c1_sclroute: I2c1Sclroute,
    i2c1_sdaroute: I2c1Sdaroute,
    _reserved105: [u8; 0x04],
    lesense_routeen: LesenseRouteen,
    lesense_ch0outroute: LesenseCh0outroute,
    lesense_ch1outroute: LesenseCh1outroute,
    lesense_ch2outroute: LesenseCh2outroute,
    lesense_ch3outroute: LesenseCh3outroute,
    lesense_ch4outroute: LesenseCh4outroute,
    lesense_ch5outroute: LesenseCh5outroute,
    lesense_ch6outroute: LesenseCh6outroute,
    lesense_ch7outroute: LesenseCh7outroute,
    lesense_ch8outroute: LesenseCh8outroute,
    lesense_ch9outroute: LesenseCh9outroute,
    lesense_ch10outroute: LesenseCh10outroute,
    lesense_ch11outroute: LesenseCh11outroute,
    lesense_ch12outroute: LesenseCh12outroute,
    lesense_ch13outroute: LesenseCh13outroute,
    lesense_ch14outroute: LesenseCh14outroute,
    lesense_ch15outroute: LesenseCh15outroute,
    _reserved122: [u8; 0x04],
    letimer_routeen: LetimerRouteen,
    letimer_out0route: LetimerOut0route,
    letimer_out1route: LetimerOut1route,
    _reserved125: [u8; 0x04],
    modem_routeen: ModemRouteen,
    modem_ant0route: ModemAnt0route,
    modem_ant1route: ModemAnt1route,
    modem_antrolloverroute: ModemAntrolloverroute,
    modem_antrr0route: ModemAntrr0route,
    modem_antrr1route: ModemAntrr1route,
    modem_antrr2route: ModemAntrr2route,
    modem_antrr3route: ModemAntrr3route,
    modem_antrr4route: ModemAntrr4route,
    modem_antrr5route: ModemAntrr5route,
    modem_antswenroute: ModemAntswenroute,
    modem_antswusroute: ModemAntswusroute,
    modem_anttrigroute: ModemAnttrigroute,
    modem_anttrigstoproute: ModemAnttrigstoproute,
    modem_dclkroute: ModemDclkroute,
    modem_dinroute: ModemDinroute,
    modem_doutroute: ModemDoutroute,
    _reserved142: [u8; 0x08],
    pcnt0_s0inroute: Pcnt0S0inroute,
    pcnt0_s1inroute: Pcnt0S1inroute,
    _reserved144: [u8; 0x04],
    prs0_routeen: Prs0Routeen,
    prs0_asynch0route: Prs0Asynch0route,
    prs0_asynch1route: Prs0Asynch1route,
    prs0_asynch2route: Prs0Asynch2route,
    prs0_asynch3route: Prs0Asynch3route,
    prs0_asynch4route: Prs0Asynch4route,
    prs0_asynch5route: Prs0Asynch5route,
    prs0_asynch6route: Prs0Asynch6route,
    prs0_asynch7route: Prs0Asynch7route,
    prs0_asynch8route: Prs0Asynch8route,
    prs0_asynch9route: Prs0Asynch9route,
    prs0_asynch10route: Prs0Asynch10route,
    prs0_asynch11route: Prs0Asynch11route,
    prs0_synch0route: Prs0Synch0route,
    prs0_synch1route: Prs0Synch1route,
    prs0_synch2route: Prs0Synch2route,
    prs0_synch3route: Prs0Synch3route,
    _reserved161: [u8; 0x64],
    syxo0_bufoutreqinasyncroute: Syxo0Bufoutreqinasyncroute,
    _reserved162: [u8; 0x04],
    timer0_routeen: Timer0Routeen,
    timer0_cc0route: Timer0Cc0route,
    timer0_cc1route: Timer0Cc1route,
    timer0_cc2route: Timer0Cc2route,
    timer0_cdti0route: Timer0Cdti0route,
    timer0_cdti1route: Timer0Cdti1route,
    timer0_cdti2route: Timer0Cdti2route,
    _reserved169: [u8; 0x04],
    timer1_routeen: Timer1Routeen,
    timer1_cc0route: Timer1Cc0route,
    timer1_cc1route: Timer1Cc1route,
    timer1_cc2route: Timer1Cc2route,
    timer1_cdti0route: Timer1Cdti0route,
    timer1_cdti1route: Timer1Cdti1route,
    timer1_cdti2route: Timer1Cdti2route,
    _reserved176: [u8; 0x04],
    timer2_routeen: Timer2Routeen,
    timer2_cc0route: Timer2Cc0route,
    timer2_cc1route: Timer2Cc1route,
    timer2_cc2route: Timer2Cc2route,
    timer2_cdti0route: Timer2Cdti0route,
    timer2_cdti1route: Timer2Cdti1route,
    timer2_cdti2route: Timer2Cdti2route,
    _reserved183: [u8; 0x04],
    timer3_routeen: Timer3Routeen,
    timer3_cc0route: Timer3Cc0route,
    timer3_cc1route: Timer3Cc1route,
    timer3_cc2route: Timer3Cc2route,
    timer3_cdti0route: Timer3Cdti0route,
    timer3_cdti1route: Timer3Cdti1route,
    timer3_cdti2route: Timer3Cdti2route,
    _reserved190: [u8; 0x04],
    timer4_routeen: Timer4Routeen,
    timer4_cc0route: Timer4Cc0route,
    timer4_cc1route: Timer4Cc1route,
    timer4_cc2route: Timer4Cc2route,
    timer4_cdti0route: Timer4Cdti0route,
    timer4_cdti1route: Timer4Cdti1route,
    timer4_cdti2route: Timer4Cdti2route,
    _reserved197: [u8; 0x04],
    timer5_routeen: Timer5Routeen,
    timer5_cc0route: Timer5Cc0route,
    timer5_cc1route: Timer5Cc1route,
    timer5_cc2route: Timer5Cc2route,
    timer5_cdti0route: Timer5Cdti0route,
    timer5_cdti1route: Timer5Cdti1route,
    timer5_cdti2route: Timer5Cdti2route,
    _reserved204: [u8; 0x04],
    timer6_routeen: Timer6Routeen,
    timer6_cc0route: Timer6Cc0route,
    timer6_cc1route: Timer6Cc1route,
    timer6_cc2route: Timer6Cc2route,
    timer6_cdti0route: Timer6Cdti0route,
    timer6_cdti1route: Timer6Cdti1route,
    timer6_cdti2route: Timer6Cdti2route,
    _reserved211: [u8; 0x04],
    timer7_routeen: Timer7Routeen,
    timer7_cc0route: Timer7Cc0route,
    timer7_cc1route: Timer7Cc1route,
    timer7_cc2route: Timer7Cc2route,
    timer7_cdti0route: Timer7Cdti0route,
    timer7_cdti1route: Timer7Cdti1route,
    timer7_cdti2route: Timer7Cdti2route,
    _reserved218: [u8; 0x08],
    usb_usbvbussenseroute: UsbUsbvbussenseroute,
}
impl RegisterBlock {
    #[doc = "0x00 - No Description"]
    #[inline(always)]
    pub const fn ipversion(&self) -> &Ipversion {
        &self.ipversion
    }
    #[doc = "0x30 - Port control"]
    #[inline(always)]
    pub const fn porta_ctrl(&self) -> &PortaCtrl {
        &self.porta_ctrl
    }
    #[doc = "0x34 - mode low"]
    #[inline(always)]
    pub const fn porta_model(&self) -> &PortaModel {
        &self.porta_model
    }
    #[doc = "0x3c - mode high"]
    #[inline(always)]
    pub const fn porta_modeh(&self) -> &PortaModeh {
        &self.porta_modeh
    }
    #[doc = "0x40 - data out"]
    #[inline(always)]
    pub const fn porta_dout(&self) -> &PortaDout {
        &self.porta_dout
    }
    #[doc = "0x44 - data in"]
    #[inline(always)]
    pub const fn porta_din(&self) -> &PortaDin {
        &self.porta_din
    }
    #[doc = "0x60 - Port control"]
    #[inline(always)]
    pub const fn portb_ctrl(&self) -> &PortbCtrl {
        &self.portb_ctrl
    }
    #[doc = "0x64 - mode low"]
    #[inline(always)]
    pub const fn portb_model(&self) -> &PortbModel {
        &self.portb_model
    }
    #[doc = "0x6c - mode high"]
    #[inline(always)]
    pub const fn portb_modeh(&self) -> &PortbModeh {
        &self.portb_modeh
    }
    #[doc = "0x70 - data out"]
    #[inline(always)]
    pub const fn portb_dout(&self) -> &PortbDout {
        &self.portb_dout
    }
    #[doc = "0x74 - data in"]
    #[inline(always)]
    pub const fn portb_din(&self) -> &PortbDin {
        &self.portb_din
    }
    #[doc = "0x90 - Port control"]
    #[inline(always)]
    pub const fn portc_ctrl(&self) -> &PortcCtrl {
        &self.portc_ctrl
    }
    #[doc = "0x94 - mode low"]
    #[inline(always)]
    pub const fn portc_model(&self) -> &PortcModel {
        &self.portc_model
    }
    #[doc = "0x9c - mode high"]
    #[inline(always)]
    pub const fn portc_modeh(&self) -> &PortcModeh {
        &self.portc_modeh
    }
    #[doc = "0xa0 - data out"]
    #[inline(always)]
    pub const fn portc_dout(&self) -> &PortcDout {
        &self.portc_dout
    }
    #[doc = "0xa4 - data in"]
    #[inline(always)]
    pub const fn portc_din(&self) -> &PortcDin {
        &self.portc_din
    }
    #[doc = "0xc0 - Port control"]
    #[inline(always)]
    pub const fn portd_ctrl(&self) -> &PortdCtrl {
        &self.portd_ctrl
    }
    #[doc = "0xc4 - mode low"]
    #[inline(always)]
    pub const fn portd_model(&self) -> &PortdModel {
        &self.portd_model
    }
    #[doc = "0xd0 - data out"]
    #[inline(always)]
    pub const fn portd_dout(&self) -> &PortdDout {
        &self.portd_dout
    }
    #[doc = "0xd4 - data in"]
    #[inline(always)]
    pub const fn portd_din(&self) -> &PortdDin {
        &self.portd_din
    }
    #[doc = "0x200 - No Description"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x210 - No Description"]
    #[inline(always)]
    pub const fn gpiolockstatus(&self) -> &Gpiolockstatus {
        &self.gpiolockstatus
    }
    #[doc = "0x220 - A Bus allocation"]
    #[inline(always)]
    pub const fn abusalloc(&self) -> &Abusalloc {
        &self.abusalloc
    }
    #[doc = "0x224 - B Bus allocation"]
    #[inline(always)]
    pub const fn bbusalloc(&self) -> &Bbusalloc {
        &self.bbusalloc
    }
    #[doc = "0x228 - CD Bus allocation"]
    #[inline(always)]
    pub const fn cdbusalloc(&self) -> &Cdbusalloc {
        &self.cdbusalloc
    }
    #[doc = "0x230 - ABUS AODD0 Switch Register"]
    #[inline(always)]
    pub const fn aodd0switch(&self) -> &Aodd0switch {
        &self.aodd0switch
    }
    #[doc = "0x234 - ABUS AODD1 Switch Register"]
    #[inline(always)]
    pub const fn aodd1switch(&self) -> &Aodd1switch {
        &self.aodd1switch
    }
    #[doc = "0x238 - ABUS AEVEN0 Switch Register"]
    #[inline(always)]
    pub const fn aeven0switch(&self) -> &Aeven0switch {
        &self.aeven0switch
    }
    #[doc = "0x23c - ABUS AEVEN1 Switch Register"]
    #[inline(always)]
    pub const fn aeven1switch(&self) -> &Aeven1switch {
        &self.aeven1switch
    }
    #[doc = "0x240 - ABUS BODD0 Switch Register"]
    #[inline(always)]
    pub const fn bodd0switch(&self) -> &Bodd0switch {
        &self.bodd0switch
    }
    #[doc = "0x244 - ABUS BODD1 Switch Register"]
    #[inline(always)]
    pub const fn bodd1switch(&self) -> &Bodd1switch {
        &self.bodd1switch
    }
    #[doc = "0x248 - ABUS BEVEN0 Switch Register"]
    #[inline(always)]
    pub const fn beven0switch(&self) -> &Beven0switch {
        &self.beven0switch
    }
    #[doc = "0x24c - ABUS BEVEN1 Switch Register"]
    #[inline(always)]
    pub const fn beven1switch(&self) -> &Beven1switch {
        &self.beven1switch
    }
    #[doc = "0x250 - ABUS CDODD0 Switch Register"]
    #[inline(always)]
    pub const fn cdodd0switch(&self) -> &Cdodd0switch {
        &self.cdodd0switch
    }
    #[doc = "0x254 - ABUS CDODD1 Switch Register"]
    #[inline(always)]
    pub const fn cdodd1switch(&self) -> &Cdodd1switch {
        &self.cdodd1switch
    }
    #[doc = "0x258 - ABUS CDEVEN0 Switch Register"]
    #[inline(always)]
    pub const fn cdeven0switch(&self) -> &Cdeven0switch {
        &self.cdeven0switch
    }
    #[doc = "0x25c - ABUS CDEVEN1 Switch Register"]
    #[inline(always)]
    pub const fn cdeven1switch(&self) -> &Cdeven1switch {
        &self.cdeven1switch
    }
    #[doc = "0x300 - External Interrupt Port Select Low"]
    #[inline(always)]
    pub const fn extipsell(&self) -> &Extipsell {
        &self.extipsell
    }
    #[doc = "0x304 - External interrupt Port Select High"]
    #[inline(always)]
    pub const fn extipselh(&self) -> &Extipselh {
        &self.extipselh
    }
    #[doc = "0x308 - External Interrupt Pin Select Low"]
    #[inline(always)]
    pub const fn extipinsell(&self) -> &Extipinsell {
        &self.extipinsell
    }
    #[doc = "0x30c - External Interrupt Pin Select High"]
    #[inline(always)]
    pub const fn extipinselh(&self) -> &Extipinselh {
        &self.extipinselh
    }
    #[doc = "0x310 - External Interrupt Rising Edge Trigger"]
    #[inline(always)]
    pub const fn extirise(&self) -> &Extirise {
        &self.extirise
    }
    #[doc = "0x314 - External Interrupt Falling Edge Trigger"]
    #[inline(always)]
    pub const fn extifall(&self) -> &Extifall {
        &self.extifall
    }
    #[doc = "0x320 - Interrupt Flag"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x324 - Interrupt Enable"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x32c - No Description"]
    #[inline(always)]
    pub const fn em4wuen(&self) -> &Em4wuen {
        &self.em4wuen
    }
    #[doc = "0x330 - No Description"]
    #[inline(always)]
    pub const fn em4wupol(&self) -> &Em4wupol {
        &self.em4wupol
    }
    #[doc = "0x340 - No Description"]
    #[inline(always)]
    pub const fn dbgroutepen(&self) -> &Dbgroutepen {
        &self.dbgroutepen
    }
    #[doc = "0x344 - No Description"]
    #[inline(always)]
    pub const fn traceroutepen(&self) -> &Traceroutepen {
        &self.traceroutepen
    }
    #[doc = "0x348 - No Description"]
    #[inline(always)]
    pub const fn femroutepen(&self) -> &Femroutepen {
        &self.femroutepen
    }
    #[doc = "0x350 - ACMP0 pin enable"]
    #[inline(always)]
    pub const fn acmp0_routeen(&self) -> &Acmp0Routeen {
        &self.acmp0_routeen
    }
    #[doc = "0x354 - ACMPOUT port/pin select"]
    #[inline(always)]
    pub const fn acmp0_acmpoutroute(&self) -> &Acmp0Acmpoutroute {
        &self.acmp0_acmpoutroute
    }
    #[doc = "0x35c - ACMP1 pin enable"]
    #[inline(always)]
    pub const fn acmp1_routeen(&self) -> &Acmp1Routeen {
        &self.acmp1_routeen
    }
    #[doc = "0x360 - ACMPOUT port/pin select"]
    #[inline(always)]
    pub const fn acmp1_acmpoutroute(&self) -> &Acmp1Acmpoutroute {
        &self.acmp1_acmpoutroute
    }
    #[doc = "0x368 - CMU pin enable"]
    #[inline(always)]
    pub const fn cmu_routeen(&self) -> &CmuRouteen {
        &self.cmu_routeen
    }
    #[doc = "0x36c - CLKIN0 port/pin select"]
    #[inline(always)]
    pub const fn cmu_clkin0route(&self) -> &CmuClkin0route {
        &self.cmu_clkin0route
    }
    #[doc = "0x370 - CLKOUT0 port/pin select"]
    #[inline(always)]
    pub const fn cmu_clkout0route(&self) -> &CmuClkout0route {
        &self.cmu_clkout0route
    }
    #[doc = "0x374 - CLKOUT1 port/pin select"]
    #[inline(always)]
    pub const fn cmu_clkout1route(&self) -> &CmuClkout1route {
        &self.cmu_clkout1route
    }
    #[doc = "0x378 - CLKOUT2 port/pin select"]
    #[inline(always)]
    pub const fn cmu_clkout2route(&self) -> &CmuClkout2route {
        &self.cmu_clkout2route
    }
    #[doc = "0x384 - DCDC pin enable"]
    #[inline(always)]
    pub const fn dcdc_routeen(&self) -> &DcdcRouteen {
        &self.dcdc_routeen
    }
    #[doc = "0x394 - EUSART0 pin enable"]
    #[inline(always)]
    pub const fn eusart0_routeen(&self) -> &Eusart0Routeen {
        &self.eusart0_routeen
    }
    #[doc = "0x398 - CS port/pin select"]
    #[inline(always)]
    pub const fn eusart0_csroute(&self) -> &Eusart0Csroute {
        &self.eusart0_csroute
    }
    #[doc = "0x39c - CTS port/pin select"]
    #[inline(always)]
    pub const fn eusart0_ctsroute(&self) -> &Eusart0Ctsroute {
        &self.eusart0_ctsroute
    }
    #[doc = "0x3a0 - RTS port/pin select"]
    #[inline(always)]
    pub const fn eusart0_rtsroute(&self) -> &Eusart0Rtsroute {
        &self.eusart0_rtsroute
    }
    #[doc = "0x3a4 - RX port/pin select"]
    #[inline(always)]
    pub const fn eusart0_rxroute(&self) -> &Eusart0Rxroute {
        &self.eusart0_rxroute
    }
    #[doc = "0x3a8 - SCLK port/pin select"]
    #[inline(always)]
    pub const fn eusart0_sclkroute(&self) -> &Eusart0Sclkroute {
        &self.eusart0_sclkroute
    }
    #[doc = "0x3ac - TX port/pin select"]
    #[inline(always)]
    pub const fn eusart0_txroute(&self) -> &Eusart0Txroute {
        &self.eusart0_txroute
    }
    #[doc = "0x3b4 - EUSART1 pin enable"]
    #[inline(always)]
    pub const fn eusart1_routeen(&self) -> &Eusart1Routeen {
        &self.eusart1_routeen
    }
    #[doc = "0x3b8 - CS port/pin select"]
    #[inline(always)]
    pub const fn eusart1_csroute(&self) -> &Eusart1Csroute {
        &self.eusart1_csroute
    }
    #[doc = "0x3bc - CTS port/pin select"]
    #[inline(always)]
    pub const fn eusart1_ctsroute(&self) -> &Eusart1Ctsroute {
        &self.eusart1_ctsroute
    }
    #[doc = "0x3c0 - RTS port/pin select"]
    #[inline(always)]
    pub const fn eusart1_rtsroute(&self) -> &Eusart1Rtsroute {
        &self.eusart1_rtsroute
    }
    #[doc = "0x3c4 - RX port/pin select"]
    #[inline(always)]
    pub const fn eusart1_rxroute(&self) -> &Eusart1Rxroute {
        &self.eusart1_rxroute
    }
    #[doc = "0x3c8 - SCLK port/pin select"]
    #[inline(always)]
    pub const fn eusart1_sclkroute(&self) -> &Eusart1Sclkroute {
        &self.eusart1_sclkroute
    }
    #[doc = "0x3cc - TX port/pin select"]
    #[inline(always)]
    pub const fn eusart1_txroute(&self) -> &Eusart1Txroute {
        &self.eusart1_txroute
    }
    #[doc = "0x3d4 - EUSART2 pin enable"]
    #[inline(always)]
    pub const fn eusart2_routeen(&self) -> &Eusart2Routeen {
        &self.eusart2_routeen
    }
    #[doc = "0x3d8 - CS port/pin select"]
    #[inline(always)]
    pub const fn eusart2_csroute(&self) -> &Eusart2Csroute {
        &self.eusart2_csroute
    }
    #[doc = "0x3dc - CTS port/pin select"]
    #[inline(always)]
    pub const fn eusart2_ctsroute(&self) -> &Eusart2Ctsroute {
        &self.eusart2_ctsroute
    }
    #[doc = "0x3e0 - RTS port/pin select"]
    #[inline(always)]
    pub const fn eusart2_rtsroute(&self) -> &Eusart2Rtsroute {
        &self.eusart2_rtsroute
    }
    #[doc = "0x3e4 - RX port/pin select"]
    #[inline(always)]
    pub const fn eusart2_rxroute(&self) -> &Eusart2Rxroute {
        &self.eusart2_rxroute
    }
    #[doc = "0x3e8 - SCLK port/pin select"]
    #[inline(always)]
    pub const fn eusart2_sclkroute(&self) -> &Eusart2Sclkroute {
        &self.eusart2_sclkroute
    }
    #[doc = "0x3ec - TX port/pin select"]
    #[inline(always)]
    pub const fn eusart2_txroute(&self) -> &Eusart2Txroute {
        &self.eusart2_txroute
    }
    #[doc = "0x3f4 - EUSART3 pin enable"]
    #[inline(always)]
    pub const fn eusart3_routeen(&self) -> &Eusart3Routeen {
        &self.eusart3_routeen
    }
    #[doc = "0x3f8 - CS port/pin select"]
    #[inline(always)]
    pub const fn eusart3_csroute(&self) -> &Eusart3Csroute {
        &self.eusart3_csroute
    }
    #[doc = "0x3fc - CTS port/pin select"]
    #[inline(always)]
    pub const fn eusart3_ctsroute(&self) -> &Eusart3Ctsroute {
        &self.eusart3_ctsroute
    }
    #[doc = "0x400 - RTS port/pin select"]
    #[inline(always)]
    pub const fn eusart3_rtsroute(&self) -> &Eusart3Rtsroute {
        &self.eusart3_rtsroute
    }
    #[doc = "0x404 - RX port/pin select"]
    #[inline(always)]
    pub const fn eusart3_rxroute(&self) -> &Eusart3Rxroute {
        &self.eusart3_rxroute
    }
    #[doc = "0x408 - SCLK port/pin select"]
    #[inline(always)]
    pub const fn eusart3_sclkroute(&self) -> &Eusart3Sclkroute {
        &self.eusart3_sclkroute
    }
    #[doc = "0x40c - TX port/pin select"]
    #[inline(always)]
    pub const fn eusart3_txroute(&self) -> &Eusart3Txroute {
        &self.eusart3_txroute
    }
    #[doc = "0x414 - EUSART4 pin enable"]
    #[inline(always)]
    pub const fn eusart4_routeen(&self) -> &Eusart4Routeen {
        &self.eusart4_routeen
    }
    #[doc = "0x418 - CS port/pin select"]
    #[inline(always)]
    pub const fn eusart4_csroute(&self) -> &Eusart4Csroute {
        &self.eusart4_csroute
    }
    #[doc = "0x41c - CTS port/pin select"]
    #[inline(always)]
    pub const fn eusart4_ctsroute(&self) -> &Eusart4Ctsroute {
        &self.eusart4_ctsroute
    }
    #[doc = "0x420 - RTS port/pin select"]
    #[inline(always)]
    pub const fn eusart4_rtsroute(&self) -> &Eusart4Rtsroute {
        &self.eusart4_rtsroute
    }
    #[doc = "0x424 - RX port/pin select"]
    #[inline(always)]
    pub const fn eusart4_rxroute(&self) -> &Eusart4Rxroute {
        &self.eusart4_rxroute
    }
    #[doc = "0x428 - SCLK port/pin select"]
    #[inline(always)]
    pub const fn eusart4_sclkroute(&self) -> &Eusart4Sclkroute {
        &self.eusart4_sclkroute
    }
    #[doc = "0x42c - TX port/pin select"]
    #[inline(always)]
    pub const fn eusart4_txroute(&self) -> &Eusart4Txroute {
        &self.eusart4_txroute
    }
    #[doc = "0x434 - FRC pin enable"]
    #[inline(always)]
    pub const fn frc_routeen(&self) -> &FrcRouteen {
        &self.frc_routeen
    }
    #[doc = "0x438 - DCLK port/pin select"]
    #[inline(always)]
    pub const fn frc_dclkroute(&self) -> &FrcDclkroute {
        &self.frc_dclkroute
    }
    #[doc = "0x43c - DFRAME port/pin select"]
    #[inline(always)]
    pub const fn frc_dframeroute(&self) -> &FrcDframeroute {
        &self.frc_dframeroute
    }
    #[doc = "0x440 - DOUT port/pin select"]
    #[inline(always)]
    pub const fn frc_doutroute(&self) -> &FrcDoutroute {
        &self.frc_doutroute
    }
    #[doc = "0x448 - I2C0 pin enable"]
    #[inline(always)]
    pub const fn i2c0_routeen(&self) -> &I2c0Routeen {
        &self.i2c0_routeen
    }
    #[doc = "0x44c - SCL port/pin select"]
    #[inline(always)]
    pub const fn i2c0_sclroute(&self) -> &I2c0Sclroute {
        &self.i2c0_sclroute
    }
    #[doc = "0x450 - SDA port/pin select"]
    #[inline(always)]
    pub const fn i2c0_sdaroute(&self) -> &I2c0Sdaroute {
        &self.i2c0_sdaroute
    }
    #[doc = "0x458 - I2C1 pin enable"]
    #[inline(always)]
    pub const fn i2c1_routeen(&self) -> &I2c1Routeen {
        &self.i2c1_routeen
    }
    #[doc = "0x45c - SCL port/pin select"]
    #[inline(always)]
    pub const fn i2c1_sclroute(&self) -> &I2c1Sclroute {
        &self.i2c1_sclroute
    }
    #[doc = "0x460 - SDA port/pin select"]
    #[inline(always)]
    pub const fn i2c1_sdaroute(&self) -> &I2c1Sdaroute {
        &self.i2c1_sdaroute
    }
    #[doc = "0x468 - LESENSE pin enable"]
    #[inline(always)]
    pub const fn lesense_routeen(&self) -> &LesenseRouteen {
        &self.lesense_routeen
    }
    #[doc = "0x46c - CH0OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch0outroute(&self) -> &LesenseCh0outroute {
        &self.lesense_ch0outroute
    }
    #[doc = "0x470 - CH1OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch1outroute(&self) -> &LesenseCh1outroute {
        &self.lesense_ch1outroute
    }
    #[doc = "0x474 - CH2OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch2outroute(&self) -> &LesenseCh2outroute {
        &self.lesense_ch2outroute
    }
    #[doc = "0x478 - CH3OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch3outroute(&self) -> &LesenseCh3outroute {
        &self.lesense_ch3outroute
    }
    #[doc = "0x47c - CH4OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch4outroute(&self) -> &LesenseCh4outroute {
        &self.lesense_ch4outroute
    }
    #[doc = "0x480 - CH5OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch5outroute(&self) -> &LesenseCh5outroute {
        &self.lesense_ch5outroute
    }
    #[doc = "0x484 - CH6OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch6outroute(&self) -> &LesenseCh6outroute {
        &self.lesense_ch6outroute
    }
    #[doc = "0x488 - CH7OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch7outroute(&self) -> &LesenseCh7outroute {
        &self.lesense_ch7outroute
    }
    #[doc = "0x48c - CH8OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch8outroute(&self) -> &LesenseCh8outroute {
        &self.lesense_ch8outroute
    }
    #[doc = "0x490 - CH9OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch9outroute(&self) -> &LesenseCh9outroute {
        &self.lesense_ch9outroute
    }
    #[doc = "0x494 - CH10OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch10outroute(&self) -> &LesenseCh10outroute {
        &self.lesense_ch10outroute
    }
    #[doc = "0x498 - CH11OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch11outroute(&self) -> &LesenseCh11outroute {
        &self.lesense_ch11outroute
    }
    #[doc = "0x49c - CH12OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch12outroute(&self) -> &LesenseCh12outroute {
        &self.lesense_ch12outroute
    }
    #[doc = "0x4a0 - CH13OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch13outroute(&self) -> &LesenseCh13outroute {
        &self.lesense_ch13outroute
    }
    #[doc = "0x4a4 - CH14OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch14outroute(&self) -> &LesenseCh14outroute {
        &self.lesense_ch14outroute
    }
    #[doc = "0x4a8 - CH15OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch15outroute(&self) -> &LesenseCh15outroute {
        &self.lesense_ch15outroute
    }
    #[doc = "0x4b0 - LETIMER pin enable"]
    #[inline(always)]
    pub const fn letimer_routeen(&self) -> &LetimerRouteen {
        &self.letimer_routeen
    }
    #[doc = "0x4b4 - OUT0 port/pin select"]
    #[inline(always)]
    pub const fn letimer_out0route(&self) -> &LetimerOut0route {
        &self.letimer_out0route
    }
    #[doc = "0x4b8 - OUT1 port/pin select"]
    #[inline(always)]
    pub const fn letimer_out1route(&self) -> &LetimerOut1route {
        &self.letimer_out1route
    }
    #[doc = "0x4c0 - MODEM pin enable"]
    #[inline(always)]
    pub const fn modem_routeen(&self) -> &ModemRouteen {
        &self.modem_routeen
    }
    #[doc = "0x4c4 - ANT0 port/pin select"]
    #[inline(always)]
    pub const fn modem_ant0route(&self) -> &ModemAnt0route {
        &self.modem_ant0route
    }
    #[doc = "0x4c8 - ANT1 port/pin select"]
    #[inline(always)]
    pub const fn modem_ant1route(&self) -> &ModemAnt1route {
        &self.modem_ant1route
    }
    #[doc = "0x4cc - ANTROLLOVER port/pin select"]
    #[inline(always)]
    pub const fn modem_antrolloverroute(&self) -> &ModemAntrolloverroute {
        &self.modem_antrolloverroute
    }
    #[doc = "0x4d0 - ANTRR0 port/pin select"]
    #[inline(always)]
    pub const fn modem_antrr0route(&self) -> &ModemAntrr0route {
        &self.modem_antrr0route
    }
    #[doc = "0x4d4 - ANTRR1 port/pin select"]
    #[inline(always)]
    pub const fn modem_antrr1route(&self) -> &ModemAntrr1route {
        &self.modem_antrr1route
    }
    #[doc = "0x4d8 - ANTRR2 port/pin select"]
    #[inline(always)]
    pub const fn modem_antrr2route(&self) -> &ModemAntrr2route {
        &self.modem_antrr2route
    }
    #[doc = "0x4dc - ANTRR3 port/pin select"]
    #[inline(always)]
    pub const fn modem_antrr3route(&self) -> &ModemAntrr3route {
        &self.modem_antrr3route
    }
    #[doc = "0x4e0 - ANTRR4 port/pin select"]
    #[inline(always)]
    pub const fn modem_antrr4route(&self) -> &ModemAntrr4route {
        &self.modem_antrr4route
    }
    #[doc = "0x4e4 - ANTRR5 port/pin select"]
    #[inline(always)]
    pub const fn modem_antrr5route(&self) -> &ModemAntrr5route {
        &self.modem_antrr5route
    }
    #[doc = "0x4e8 - ANTSWEN port/pin select"]
    #[inline(always)]
    pub const fn modem_antswenroute(&self) -> &ModemAntswenroute {
        &self.modem_antswenroute
    }
    #[doc = "0x4ec - ANTSWUS port/pin select"]
    #[inline(always)]
    pub const fn modem_antswusroute(&self) -> &ModemAntswusroute {
        &self.modem_antswusroute
    }
    #[doc = "0x4f0 - ANTTRIG port/pin select"]
    #[inline(always)]
    pub const fn modem_anttrigroute(&self) -> &ModemAnttrigroute {
        &self.modem_anttrigroute
    }
    #[doc = "0x4f4 - ANTTRIGSTOP port/pin select"]
    #[inline(always)]
    pub const fn modem_anttrigstoproute(&self) -> &ModemAnttrigstoproute {
        &self.modem_anttrigstoproute
    }
    #[doc = "0x4f8 - DCLK port/pin select"]
    #[inline(always)]
    pub const fn modem_dclkroute(&self) -> &ModemDclkroute {
        &self.modem_dclkroute
    }
    #[doc = "0x4fc - DIN port/pin select"]
    #[inline(always)]
    pub const fn modem_dinroute(&self) -> &ModemDinroute {
        &self.modem_dinroute
    }
    #[doc = "0x500 - DOUT port/pin select"]
    #[inline(always)]
    pub const fn modem_doutroute(&self) -> &ModemDoutroute {
        &self.modem_doutroute
    }
    #[doc = "0x50c - S0IN port/pin select"]
    #[inline(always)]
    pub const fn pcnt0_s0inroute(&self) -> &Pcnt0S0inroute {
        &self.pcnt0_s0inroute
    }
    #[doc = "0x510 - S1IN port/pin select"]
    #[inline(always)]
    pub const fn pcnt0_s1inroute(&self) -> &Pcnt0S1inroute {
        &self.pcnt0_s1inroute
    }
    #[doc = "0x518 - PRS0 pin enable"]
    #[inline(always)]
    pub const fn prs0_routeen(&self) -> &Prs0Routeen {
        &self.prs0_routeen
    }
    #[doc = "0x51c - ASYNCH0 port/pin select"]
    #[inline(always)]
    pub const fn prs0_asynch0route(&self) -> &Prs0Asynch0route {
        &self.prs0_asynch0route
    }
    #[doc = "0x520 - ASYNCH1 port/pin select"]
    #[inline(always)]
    pub const fn prs0_asynch1route(&self) -> &Prs0Asynch1route {
        &self.prs0_asynch1route
    }
    #[doc = "0x524 - ASYNCH2 port/pin select"]
    #[inline(always)]
    pub const fn prs0_asynch2route(&self) -> &Prs0Asynch2route {
        &self.prs0_asynch2route
    }
    #[doc = "0x528 - ASYNCH3 port/pin select"]
    #[inline(always)]
    pub const fn prs0_asynch3route(&self) -> &Prs0Asynch3route {
        &self.prs0_asynch3route
    }
    #[doc = "0x52c - ASYNCH4 port/pin select"]
    #[inline(always)]
    pub const fn prs0_asynch4route(&self) -> &Prs0Asynch4route {
        &self.prs0_asynch4route
    }
    #[doc = "0x530 - ASYNCH5 port/pin select"]
    #[inline(always)]
    pub const fn prs0_asynch5route(&self) -> &Prs0Asynch5route {
        &self.prs0_asynch5route
    }
    #[doc = "0x534 - ASYNCH6 port/pin select"]
    #[inline(always)]
    pub const fn prs0_asynch6route(&self) -> &Prs0Asynch6route {
        &self.prs0_asynch6route
    }
    #[doc = "0x538 - ASYNCH7 port/pin select"]
    #[inline(always)]
    pub const fn prs0_asynch7route(&self) -> &Prs0Asynch7route {
        &self.prs0_asynch7route
    }
    #[doc = "0x53c - ASYNCH8 port/pin select"]
    #[inline(always)]
    pub const fn prs0_asynch8route(&self) -> &Prs0Asynch8route {
        &self.prs0_asynch8route
    }
    #[doc = "0x540 - ASYNCH9 port/pin select"]
    #[inline(always)]
    pub const fn prs0_asynch9route(&self) -> &Prs0Asynch9route {
        &self.prs0_asynch9route
    }
    #[doc = "0x544 - ASYNCH10 port/pin select"]
    #[inline(always)]
    pub const fn prs0_asynch10route(&self) -> &Prs0Asynch10route {
        &self.prs0_asynch10route
    }
    #[doc = "0x548 - ASYNCH11 port/pin select"]
    #[inline(always)]
    pub const fn prs0_asynch11route(&self) -> &Prs0Asynch11route {
        &self.prs0_asynch11route
    }
    #[doc = "0x54c - SYNCH0 port/pin select"]
    #[inline(always)]
    pub const fn prs0_synch0route(&self) -> &Prs0Synch0route {
        &self.prs0_synch0route
    }
    #[doc = "0x550 - SYNCH1 port/pin select"]
    #[inline(always)]
    pub const fn prs0_synch1route(&self) -> &Prs0Synch1route {
        &self.prs0_synch1route
    }
    #[doc = "0x554 - SYNCH2 port/pin select"]
    #[inline(always)]
    pub const fn prs0_synch2route(&self) -> &Prs0Synch2route {
        &self.prs0_synch2route
    }
    #[doc = "0x558 - SYNCH3 port/pin select"]
    #[inline(always)]
    pub const fn prs0_synch3route(&self) -> &Prs0Synch3route {
        &self.prs0_synch3route
    }
    #[doc = "0x5c0 - BUFOUTREQINASYNC port/pin select"]
    #[inline(always)]
    pub const fn syxo0_bufoutreqinasyncroute(&self) -> &Syxo0Bufoutreqinasyncroute {
        &self.syxo0_bufoutreqinasyncroute
    }
    #[doc = "0x5c8 - TIMER0 pin enable"]
    #[inline(always)]
    pub const fn timer0_routeen(&self) -> &Timer0Routeen {
        &self.timer0_routeen
    }
    #[doc = "0x5cc - CC0 port/pin select"]
    #[inline(always)]
    pub const fn timer0_cc0route(&self) -> &Timer0Cc0route {
        &self.timer0_cc0route
    }
    #[doc = "0x5d0 - CC1 port/pin select"]
    #[inline(always)]
    pub const fn timer0_cc1route(&self) -> &Timer0Cc1route {
        &self.timer0_cc1route
    }
    #[doc = "0x5d4 - CC2 port/pin select"]
    #[inline(always)]
    pub const fn timer0_cc2route(&self) -> &Timer0Cc2route {
        &self.timer0_cc2route
    }
    #[doc = "0x5d8 - CDTI0 port/pin select"]
    #[inline(always)]
    pub const fn timer0_cdti0route(&self) -> &Timer0Cdti0route {
        &self.timer0_cdti0route
    }
    #[doc = "0x5dc - CDTI1 port/pin select"]
    #[inline(always)]
    pub const fn timer0_cdti1route(&self) -> &Timer0Cdti1route {
        &self.timer0_cdti1route
    }
    #[doc = "0x5e0 - CDTI2 port/pin select"]
    #[inline(always)]
    pub const fn timer0_cdti2route(&self) -> &Timer0Cdti2route {
        &self.timer0_cdti2route
    }
    #[doc = "0x5e8 - TIMER1 pin enable"]
    #[inline(always)]
    pub const fn timer1_routeen(&self) -> &Timer1Routeen {
        &self.timer1_routeen
    }
    #[doc = "0x5ec - CC0 port/pin select"]
    #[inline(always)]
    pub const fn timer1_cc0route(&self) -> &Timer1Cc0route {
        &self.timer1_cc0route
    }
    #[doc = "0x5f0 - CC1 port/pin select"]
    #[inline(always)]
    pub const fn timer1_cc1route(&self) -> &Timer1Cc1route {
        &self.timer1_cc1route
    }
    #[doc = "0x5f4 - CC2 port/pin select"]
    #[inline(always)]
    pub const fn timer1_cc2route(&self) -> &Timer1Cc2route {
        &self.timer1_cc2route
    }
    #[doc = "0x5f8 - CDTI0 port/pin select"]
    #[inline(always)]
    pub const fn timer1_cdti0route(&self) -> &Timer1Cdti0route {
        &self.timer1_cdti0route
    }
    #[doc = "0x5fc - CDTI1 port/pin select"]
    #[inline(always)]
    pub const fn timer1_cdti1route(&self) -> &Timer1Cdti1route {
        &self.timer1_cdti1route
    }
    #[doc = "0x600 - CDTI2 port/pin select"]
    #[inline(always)]
    pub const fn timer1_cdti2route(&self) -> &Timer1Cdti2route {
        &self.timer1_cdti2route
    }
    #[doc = "0x608 - TIMER2 pin enable"]
    #[inline(always)]
    pub const fn timer2_routeen(&self) -> &Timer2Routeen {
        &self.timer2_routeen
    }
    #[doc = "0x60c - CC0 port/pin select"]
    #[inline(always)]
    pub const fn timer2_cc0route(&self) -> &Timer2Cc0route {
        &self.timer2_cc0route
    }
    #[doc = "0x610 - CC1 port/pin select"]
    #[inline(always)]
    pub const fn timer2_cc1route(&self) -> &Timer2Cc1route {
        &self.timer2_cc1route
    }
    #[doc = "0x614 - CC2 port/pin select"]
    #[inline(always)]
    pub const fn timer2_cc2route(&self) -> &Timer2Cc2route {
        &self.timer2_cc2route
    }
    #[doc = "0x618 - CDTI0 port/pin select"]
    #[inline(always)]
    pub const fn timer2_cdti0route(&self) -> &Timer2Cdti0route {
        &self.timer2_cdti0route
    }
    #[doc = "0x61c - CDTI1 port/pin select"]
    #[inline(always)]
    pub const fn timer2_cdti1route(&self) -> &Timer2Cdti1route {
        &self.timer2_cdti1route
    }
    #[doc = "0x620 - CDTI2 port/pin select"]
    #[inline(always)]
    pub const fn timer2_cdti2route(&self) -> &Timer2Cdti2route {
        &self.timer2_cdti2route
    }
    #[doc = "0x628 - TIMER3 pin enable"]
    #[inline(always)]
    pub const fn timer3_routeen(&self) -> &Timer3Routeen {
        &self.timer3_routeen
    }
    #[doc = "0x62c - CC0 port/pin select"]
    #[inline(always)]
    pub const fn timer3_cc0route(&self) -> &Timer3Cc0route {
        &self.timer3_cc0route
    }
    #[doc = "0x630 - CC1 port/pin select"]
    #[inline(always)]
    pub const fn timer3_cc1route(&self) -> &Timer3Cc1route {
        &self.timer3_cc1route
    }
    #[doc = "0x634 - CC2 port/pin select"]
    #[inline(always)]
    pub const fn timer3_cc2route(&self) -> &Timer3Cc2route {
        &self.timer3_cc2route
    }
    #[doc = "0x638 - CDTI0 port/pin select"]
    #[inline(always)]
    pub const fn timer3_cdti0route(&self) -> &Timer3Cdti0route {
        &self.timer3_cdti0route
    }
    #[doc = "0x63c - CDTI1 port/pin select"]
    #[inline(always)]
    pub const fn timer3_cdti1route(&self) -> &Timer3Cdti1route {
        &self.timer3_cdti1route
    }
    #[doc = "0x640 - CDTI2 port/pin select"]
    #[inline(always)]
    pub const fn timer3_cdti2route(&self) -> &Timer3Cdti2route {
        &self.timer3_cdti2route
    }
    #[doc = "0x648 - TIMER4 pin enable"]
    #[inline(always)]
    pub const fn timer4_routeen(&self) -> &Timer4Routeen {
        &self.timer4_routeen
    }
    #[doc = "0x64c - CC0 port/pin select"]
    #[inline(always)]
    pub const fn timer4_cc0route(&self) -> &Timer4Cc0route {
        &self.timer4_cc0route
    }
    #[doc = "0x650 - CC1 port/pin select"]
    #[inline(always)]
    pub const fn timer4_cc1route(&self) -> &Timer4Cc1route {
        &self.timer4_cc1route
    }
    #[doc = "0x654 - CC2 port/pin select"]
    #[inline(always)]
    pub const fn timer4_cc2route(&self) -> &Timer4Cc2route {
        &self.timer4_cc2route
    }
    #[doc = "0x658 - CDTI0 port/pin select"]
    #[inline(always)]
    pub const fn timer4_cdti0route(&self) -> &Timer4Cdti0route {
        &self.timer4_cdti0route
    }
    #[doc = "0x65c - CDTI1 port/pin select"]
    #[inline(always)]
    pub const fn timer4_cdti1route(&self) -> &Timer4Cdti1route {
        &self.timer4_cdti1route
    }
    #[doc = "0x660 - CDTI2 port/pin select"]
    #[inline(always)]
    pub const fn timer4_cdti2route(&self) -> &Timer4Cdti2route {
        &self.timer4_cdti2route
    }
    #[doc = "0x668 - TIMER5 pin enable"]
    #[inline(always)]
    pub const fn timer5_routeen(&self) -> &Timer5Routeen {
        &self.timer5_routeen
    }
    #[doc = "0x66c - CC0 port/pin select"]
    #[inline(always)]
    pub const fn timer5_cc0route(&self) -> &Timer5Cc0route {
        &self.timer5_cc0route
    }
    #[doc = "0x670 - CC1 port/pin select"]
    #[inline(always)]
    pub const fn timer5_cc1route(&self) -> &Timer5Cc1route {
        &self.timer5_cc1route
    }
    #[doc = "0x674 - CC2 port/pin select"]
    #[inline(always)]
    pub const fn timer5_cc2route(&self) -> &Timer5Cc2route {
        &self.timer5_cc2route
    }
    #[doc = "0x678 - CDTI0 port/pin select"]
    #[inline(always)]
    pub const fn timer5_cdti0route(&self) -> &Timer5Cdti0route {
        &self.timer5_cdti0route
    }
    #[doc = "0x67c - CDTI1 port/pin select"]
    #[inline(always)]
    pub const fn timer5_cdti1route(&self) -> &Timer5Cdti1route {
        &self.timer5_cdti1route
    }
    #[doc = "0x680 - CDTI2 port/pin select"]
    #[inline(always)]
    pub const fn timer5_cdti2route(&self) -> &Timer5Cdti2route {
        &self.timer5_cdti2route
    }
    #[doc = "0x688 - TIMER6 pin enable"]
    #[inline(always)]
    pub const fn timer6_routeen(&self) -> &Timer6Routeen {
        &self.timer6_routeen
    }
    #[doc = "0x68c - CC0 port/pin select"]
    #[inline(always)]
    pub const fn timer6_cc0route(&self) -> &Timer6Cc0route {
        &self.timer6_cc0route
    }
    #[doc = "0x690 - CC1 port/pin select"]
    #[inline(always)]
    pub const fn timer6_cc1route(&self) -> &Timer6Cc1route {
        &self.timer6_cc1route
    }
    #[doc = "0x694 - CC2 port/pin select"]
    #[inline(always)]
    pub const fn timer6_cc2route(&self) -> &Timer6Cc2route {
        &self.timer6_cc2route
    }
    #[doc = "0x698 - CDTI0 port/pin select"]
    #[inline(always)]
    pub const fn timer6_cdti0route(&self) -> &Timer6Cdti0route {
        &self.timer6_cdti0route
    }
    #[doc = "0x69c - CDTI1 port/pin select"]
    #[inline(always)]
    pub const fn timer6_cdti1route(&self) -> &Timer6Cdti1route {
        &self.timer6_cdti1route
    }
    #[doc = "0x6a0 - CDTI2 port/pin select"]
    #[inline(always)]
    pub const fn timer6_cdti2route(&self) -> &Timer6Cdti2route {
        &self.timer6_cdti2route
    }
    #[doc = "0x6a8 - TIMER7 pin enable"]
    #[inline(always)]
    pub const fn timer7_routeen(&self) -> &Timer7Routeen {
        &self.timer7_routeen
    }
    #[doc = "0x6ac - CC0 port/pin select"]
    #[inline(always)]
    pub const fn timer7_cc0route(&self) -> &Timer7Cc0route {
        &self.timer7_cc0route
    }
    #[doc = "0x6b0 - CC1 port/pin select"]
    #[inline(always)]
    pub const fn timer7_cc1route(&self) -> &Timer7Cc1route {
        &self.timer7_cc1route
    }
    #[doc = "0x6b4 - CC2 port/pin select"]
    #[inline(always)]
    pub const fn timer7_cc2route(&self) -> &Timer7Cc2route {
        &self.timer7_cc2route
    }
    #[doc = "0x6b8 - CDTI0 port/pin select"]
    #[inline(always)]
    pub const fn timer7_cdti0route(&self) -> &Timer7Cdti0route {
        &self.timer7_cdti0route
    }
    #[doc = "0x6bc - CDTI1 port/pin select"]
    #[inline(always)]
    pub const fn timer7_cdti1route(&self) -> &Timer7Cdti1route {
        &self.timer7_cdti1route
    }
    #[doc = "0x6c0 - CDTI2 port/pin select"]
    #[inline(always)]
    pub const fn timer7_cdti2route(&self) -> &Timer7Cdti2route {
        &self.timer7_cdti2route
    }
    #[doc = "0x6cc - USBVBUSSENSE port/pin select"]
    #[inline(always)]
    pub const fn usb_usbvbussenseroute(&self) -> &UsbUsbvbussenseroute {
        &self.usb_usbvbussenseroute
    }
}
#[doc = "IPVERSION (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ipversion::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipversion`] module"]
#[doc(alias = "IPVERSION")]
pub type Ipversion = crate::Reg<ipversion::IpversionSpec>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "PORTA_CTRL (rw) register accessor: Port control\n\nYou can [`read`](crate::Reg::read) this register and get [`porta_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porta_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@porta_ctrl`] module"]
#[doc(alias = "PORTA_CTRL")]
pub type PortaCtrl = crate::Reg<porta_ctrl::PortaCtrlSpec>;
#[doc = "Port control"]
pub mod porta_ctrl;
#[doc = "PORTA_MODEL (rw) register accessor: mode low\n\nYou can [`read`](crate::Reg::read) this register and get [`porta_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porta_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@porta_model`] module"]
#[doc(alias = "PORTA_MODEL")]
pub type PortaModel = crate::Reg<porta_model::PortaModelSpec>;
#[doc = "mode low"]
pub mod porta_model;
#[doc = "PORTA_MODEH (rw) register accessor: mode high\n\nYou can [`read`](crate::Reg::read) this register and get [`porta_modeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porta_modeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@porta_modeh`] module"]
#[doc(alias = "PORTA_MODEH")]
pub type PortaModeh = crate::Reg<porta_modeh::PortaModehSpec>;
#[doc = "mode high"]
pub mod porta_modeh;
#[doc = "PORTA_DOUT (rw) register accessor: data out\n\nYou can [`read`](crate::Reg::read) this register and get [`porta_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porta_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@porta_dout`] module"]
#[doc(alias = "PORTA_DOUT")]
pub type PortaDout = crate::Reg<porta_dout::PortaDoutSpec>;
#[doc = "data out"]
pub mod porta_dout;
#[doc = "PORTA_DIN (r) register accessor: data in\n\nYou can [`read`](crate::Reg::read) this register and get [`porta_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@porta_din`] module"]
#[doc(alias = "PORTA_DIN")]
pub type PortaDin = crate::Reg<porta_din::PortaDinSpec>;
#[doc = "data in"]
pub mod porta_din;
#[doc = "PORTB_CTRL (rw) register accessor: Port control\n\nYou can [`read`](crate::Reg::read) this register and get [`portb_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portb_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portb_ctrl`] module"]
#[doc(alias = "PORTB_CTRL")]
pub type PortbCtrl = crate::Reg<portb_ctrl::PortbCtrlSpec>;
#[doc = "Port control"]
pub mod portb_ctrl;
#[doc = "PORTB_MODEL (rw) register accessor: mode low\n\nYou can [`read`](crate::Reg::read) this register and get [`portb_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portb_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portb_model`] module"]
#[doc(alias = "PORTB_MODEL")]
pub type PortbModel = crate::Reg<portb_model::PortbModelSpec>;
#[doc = "mode low"]
pub mod portb_model;
#[doc = "PORTB_MODEH (rw) register accessor: mode high\n\nYou can [`read`](crate::Reg::read) this register and get [`portb_modeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portb_modeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portb_modeh`] module"]
#[doc(alias = "PORTB_MODEH")]
pub type PortbModeh = crate::Reg<portb_modeh::PortbModehSpec>;
#[doc = "mode high"]
pub mod portb_modeh;
#[doc = "PORTB_DOUT (rw) register accessor: data out\n\nYou can [`read`](crate::Reg::read) this register and get [`portb_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portb_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portb_dout`] module"]
#[doc(alias = "PORTB_DOUT")]
pub type PortbDout = crate::Reg<portb_dout::PortbDoutSpec>;
#[doc = "data out"]
pub mod portb_dout;
#[doc = "PORTB_DIN (r) register accessor: data in\n\nYou can [`read`](crate::Reg::read) this register and get [`portb_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portb_din`] module"]
#[doc(alias = "PORTB_DIN")]
pub type PortbDin = crate::Reg<portb_din::PortbDinSpec>;
#[doc = "data in"]
pub mod portb_din;
#[doc = "PORTC_CTRL (rw) register accessor: Port control\n\nYou can [`read`](crate::Reg::read) this register and get [`portc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portc_ctrl`] module"]
#[doc(alias = "PORTC_CTRL")]
pub type PortcCtrl = crate::Reg<portc_ctrl::PortcCtrlSpec>;
#[doc = "Port control"]
pub mod portc_ctrl;
#[doc = "PORTC_MODEL (rw) register accessor: mode low\n\nYou can [`read`](crate::Reg::read) this register and get [`portc_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portc_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portc_model`] module"]
#[doc(alias = "PORTC_MODEL")]
pub type PortcModel = crate::Reg<portc_model::PortcModelSpec>;
#[doc = "mode low"]
pub mod portc_model;
#[doc = "PORTC_MODEH (rw) register accessor: mode high\n\nYou can [`read`](crate::Reg::read) this register and get [`portc_modeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portc_modeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portc_modeh`] module"]
#[doc(alias = "PORTC_MODEH")]
pub type PortcModeh = crate::Reg<portc_modeh::PortcModehSpec>;
#[doc = "mode high"]
pub mod portc_modeh;
#[doc = "PORTC_DOUT (rw) register accessor: data out\n\nYou can [`read`](crate::Reg::read) this register and get [`portc_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portc_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portc_dout`] module"]
#[doc(alias = "PORTC_DOUT")]
pub type PortcDout = crate::Reg<portc_dout::PortcDoutSpec>;
#[doc = "data out"]
pub mod portc_dout;
#[doc = "PORTC_DIN (r) register accessor: data in\n\nYou can [`read`](crate::Reg::read) this register and get [`portc_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portc_din`] module"]
#[doc(alias = "PORTC_DIN")]
pub type PortcDin = crate::Reg<portc_din::PortcDinSpec>;
#[doc = "data in"]
pub mod portc_din;
#[doc = "PORTD_CTRL (rw) register accessor: Port control\n\nYou can [`read`](crate::Reg::read) this register and get [`portd_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portd_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portd_ctrl`] module"]
#[doc(alias = "PORTD_CTRL")]
pub type PortdCtrl = crate::Reg<portd_ctrl::PortdCtrlSpec>;
#[doc = "Port control"]
pub mod portd_ctrl;
#[doc = "PORTD_MODEL (rw) register accessor: mode low\n\nYou can [`read`](crate::Reg::read) this register and get [`portd_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portd_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portd_model`] module"]
#[doc(alias = "PORTD_MODEL")]
pub type PortdModel = crate::Reg<portd_model::PortdModelSpec>;
#[doc = "mode low"]
pub mod portd_model;
#[doc = "PORTD_DOUT (rw) register accessor: data out\n\nYou can [`read`](crate::Reg::read) this register and get [`portd_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portd_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portd_dout`] module"]
#[doc(alias = "PORTD_DOUT")]
pub type PortdDout = crate::Reg<portd_dout::PortdDoutSpec>;
#[doc = "data out"]
pub mod portd_dout;
#[doc = "PORTD_DIN (r) register accessor: data in\n\nYou can [`read`](crate::Reg::read) this register and get [`portd_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portd_din`] module"]
#[doc(alias = "PORTD_DIN")]
pub type PortdDin = crate::Reg<portd_din::PortdDinSpec>;
#[doc = "data in"]
pub mod portd_din;
#[doc = "LOCK (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "No Description"]
pub mod lock;
#[doc = "GPIOLOCKSTATUS (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiolockstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiolockstatus`] module"]
#[doc(alias = "GPIOLOCKSTATUS")]
pub type Gpiolockstatus = crate::Reg<gpiolockstatus::GpiolockstatusSpec>;
#[doc = "No Description"]
pub mod gpiolockstatus;
#[doc = "ABUSALLOC (rw) register accessor: A Bus allocation\n\nYou can [`read`](crate::Reg::read) this register and get [`abusalloc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abusalloc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@abusalloc`] module"]
#[doc(alias = "ABUSALLOC")]
pub type Abusalloc = crate::Reg<abusalloc::AbusallocSpec>;
#[doc = "A Bus allocation"]
pub mod abusalloc;
#[doc = "BBUSALLOC (rw) register accessor: B Bus allocation\n\nYou can [`read`](crate::Reg::read) this register and get [`bbusalloc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bbusalloc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bbusalloc`] module"]
#[doc(alias = "BBUSALLOC")]
pub type Bbusalloc = crate::Reg<bbusalloc::BbusallocSpec>;
#[doc = "B Bus allocation"]
pub mod bbusalloc;
#[doc = "CDBUSALLOC (rw) register accessor: CD Bus allocation\n\nYou can [`read`](crate::Reg::read) this register and get [`cdbusalloc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdbusalloc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdbusalloc`] module"]
#[doc(alias = "CDBUSALLOC")]
pub type Cdbusalloc = crate::Reg<cdbusalloc::CdbusallocSpec>;
#[doc = "CD Bus allocation"]
pub mod cdbusalloc;
#[doc = "AODD0SWITCH (rw) register accessor: ABUS AODD0 Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aodd0switch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aodd0switch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aodd0switch`] module"]
#[doc(alias = "AODD0SWITCH")]
pub type Aodd0switch = crate::Reg<aodd0switch::Aodd0switchSpec>;
#[doc = "ABUS AODD0 Switch Register"]
pub mod aodd0switch;
#[doc = "AODD1SWITCH (rw) register accessor: ABUS AODD1 Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aodd1switch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aodd1switch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aodd1switch`] module"]
#[doc(alias = "AODD1SWITCH")]
pub type Aodd1switch = crate::Reg<aodd1switch::Aodd1switchSpec>;
#[doc = "ABUS AODD1 Switch Register"]
pub mod aodd1switch;
#[doc = "AEVEN0SWITCH (rw) register accessor: ABUS AEVEN0 Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aeven0switch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeven0switch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeven0switch`] module"]
#[doc(alias = "AEVEN0SWITCH")]
pub type Aeven0switch = crate::Reg<aeven0switch::Aeven0switchSpec>;
#[doc = "ABUS AEVEN0 Switch Register"]
pub mod aeven0switch;
#[doc = "AEVEN1SWITCH (rw) register accessor: ABUS AEVEN1 Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aeven1switch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeven1switch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeven1switch`] module"]
#[doc(alias = "AEVEN1SWITCH")]
pub type Aeven1switch = crate::Reg<aeven1switch::Aeven1switchSpec>;
#[doc = "ABUS AEVEN1 Switch Register"]
pub mod aeven1switch;
#[doc = "BODD0SWITCH (rw) register accessor: ABUS BODD0 Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bodd0switch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bodd0switch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bodd0switch`] module"]
#[doc(alias = "BODD0SWITCH")]
pub type Bodd0switch = crate::Reg<bodd0switch::Bodd0switchSpec>;
#[doc = "ABUS BODD0 Switch Register"]
pub mod bodd0switch;
#[doc = "BODD1SWITCH (rw) register accessor: ABUS BODD1 Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bodd1switch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bodd1switch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bodd1switch`] module"]
#[doc(alias = "BODD1SWITCH")]
pub type Bodd1switch = crate::Reg<bodd1switch::Bodd1switchSpec>;
#[doc = "ABUS BODD1 Switch Register"]
pub mod bodd1switch;
#[doc = "BEVEN0SWITCH (rw) register accessor: ABUS BEVEN0 Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`beven0switch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`beven0switch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@beven0switch`] module"]
#[doc(alias = "BEVEN0SWITCH")]
pub type Beven0switch = crate::Reg<beven0switch::Beven0switchSpec>;
#[doc = "ABUS BEVEN0 Switch Register"]
pub mod beven0switch;
#[doc = "BEVEN1SWITCH (rw) register accessor: ABUS BEVEN1 Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`beven1switch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`beven1switch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@beven1switch`] module"]
#[doc(alias = "BEVEN1SWITCH")]
pub type Beven1switch = crate::Reg<beven1switch::Beven1switchSpec>;
#[doc = "ABUS BEVEN1 Switch Register"]
pub mod beven1switch;
#[doc = "CDODD0SWITCH (rw) register accessor: ABUS CDODD0 Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdodd0switch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdodd0switch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdodd0switch`] module"]
#[doc(alias = "CDODD0SWITCH")]
pub type Cdodd0switch = crate::Reg<cdodd0switch::Cdodd0switchSpec>;
#[doc = "ABUS CDODD0 Switch Register"]
pub mod cdodd0switch;
#[doc = "CDODD1SWITCH (rw) register accessor: ABUS CDODD1 Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdodd1switch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdodd1switch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdodd1switch`] module"]
#[doc(alias = "CDODD1SWITCH")]
pub type Cdodd1switch = crate::Reg<cdodd1switch::Cdodd1switchSpec>;
#[doc = "ABUS CDODD1 Switch Register"]
pub mod cdodd1switch;
#[doc = "CDEVEN0SWITCH (rw) register accessor: ABUS CDEVEN0 Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdeven0switch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdeven0switch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdeven0switch`] module"]
#[doc(alias = "CDEVEN0SWITCH")]
pub type Cdeven0switch = crate::Reg<cdeven0switch::Cdeven0switchSpec>;
#[doc = "ABUS CDEVEN0 Switch Register"]
pub mod cdeven0switch;
#[doc = "CDEVEN1SWITCH (rw) register accessor: ABUS CDEVEN1 Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdeven1switch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdeven1switch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdeven1switch`] module"]
#[doc(alias = "CDEVEN1SWITCH")]
pub type Cdeven1switch = crate::Reg<cdeven1switch::Cdeven1switchSpec>;
#[doc = "ABUS CDEVEN1 Switch Register"]
pub mod cdeven1switch;
#[doc = "EXTIPSELL (rw) register accessor: External Interrupt Port Select Low\n\nYou can [`read`](crate::Reg::read) this register and get [`extipsell::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipsell::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipsell`] module"]
#[doc(alias = "EXTIPSELL")]
pub type Extipsell = crate::Reg<extipsell::ExtipsellSpec>;
#[doc = "External Interrupt Port Select Low"]
pub mod extipsell;
#[doc = "EXTIPSELH (rw) register accessor: External interrupt Port Select High\n\nYou can [`read`](crate::Reg::read) this register and get [`extipselh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipselh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipselh`] module"]
#[doc(alias = "EXTIPSELH")]
pub type Extipselh = crate::Reg<extipselh::ExtipselhSpec>;
#[doc = "External interrupt Port Select High"]
pub mod extipselh;
#[doc = "EXTIPINSELL (rw) register accessor: External Interrupt Pin Select Low\n\nYou can [`read`](crate::Reg::read) this register and get [`extipinsell::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipinsell::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipinsell`] module"]
#[doc(alias = "EXTIPINSELL")]
pub type Extipinsell = crate::Reg<extipinsell::ExtipinsellSpec>;
#[doc = "External Interrupt Pin Select Low"]
pub mod extipinsell;
#[doc = "EXTIPINSELH (rw) register accessor: External Interrupt Pin Select High\n\nYou can [`read`](crate::Reg::read) this register and get [`extipinselh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipinselh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipinselh`] module"]
#[doc(alias = "EXTIPINSELH")]
pub type Extipinselh = crate::Reg<extipinselh::ExtipinselhSpec>;
#[doc = "External Interrupt Pin Select High"]
pub mod extipinselh;
#[doc = "EXTIRISE (rw) register accessor: External Interrupt Rising Edge Trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`extirise::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extirise::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extirise`] module"]
#[doc(alias = "EXTIRISE")]
pub type Extirise = crate::Reg<extirise::ExtiriseSpec>;
#[doc = "External Interrupt Rising Edge Trigger"]
pub mod extirise;
#[doc = "EXTIFALL (rw) register accessor: External Interrupt Falling Edge Trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`extifall::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extifall::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extifall`] module"]
#[doc(alias = "EXTIFALL")]
pub type Extifall = crate::Reg<extifall::ExtifallSpec>;
#[doc = "External Interrupt Falling Edge Trigger"]
pub mod extifall;
#[doc = "IF (rw) register accessor: Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`] module"]
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IfSpec>;
#[doc = "Interrupt Flag"]
pub mod if_;
#[doc = "IEN (rw) register accessor: Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`] module"]
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IenSpec>;
#[doc = "Interrupt Enable"]
pub mod ien;
#[doc = "EM4WUEN (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`em4wuen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em4wuen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em4wuen`] module"]
#[doc(alias = "EM4WUEN")]
pub type Em4wuen = crate::Reg<em4wuen::Em4wuenSpec>;
#[doc = "No Description"]
pub mod em4wuen;
#[doc = "EM4WUPOL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`em4wupol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em4wupol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em4wupol`] module"]
#[doc(alias = "EM4WUPOL")]
pub type Em4wupol = crate::Reg<em4wupol::Em4wupolSpec>;
#[doc = "No Description"]
pub mod em4wupol;
#[doc = "DBGROUTEPEN (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgroutepen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgroutepen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgroutepen`] module"]
#[doc(alias = "DBGROUTEPEN")]
pub type Dbgroutepen = crate::Reg<dbgroutepen::DbgroutepenSpec>;
#[doc = "No Description"]
pub mod dbgroutepen;
#[doc = "TRACEROUTEPEN (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`traceroutepen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`traceroutepen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@traceroutepen`] module"]
#[doc(alias = "TRACEROUTEPEN")]
pub type Traceroutepen = crate::Reg<traceroutepen::TraceroutepenSpec>;
#[doc = "No Description"]
pub mod traceroutepen;
#[doc = "FEMROUTEPEN (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`femroutepen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`femroutepen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@femroutepen`] module"]
#[doc(alias = "FEMROUTEPEN")]
pub type Femroutepen = crate::Reg<femroutepen::FemroutepenSpec>;
#[doc = "No Description"]
pub mod femroutepen;
#[doc = "ACMP0_ROUTEEN (rw) register accessor: ACMP0 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`acmp0_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acmp0_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acmp0_routeen`] module"]
#[doc(alias = "ACMP0_ROUTEEN")]
pub type Acmp0Routeen = crate::Reg<acmp0_routeen::Acmp0RouteenSpec>;
#[doc = "ACMP0 pin enable"]
pub mod acmp0_routeen;
#[doc = "ACMP0_ACMPOUTROUTE (rw) register accessor: ACMPOUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`acmp0_acmpoutroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acmp0_acmpoutroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acmp0_acmpoutroute`] module"]
#[doc(alias = "ACMP0_ACMPOUTROUTE")]
pub type Acmp0Acmpoutroute = crate::Reg<acmp0_acmpoutroute::Acmp0AcmpoutrouteSpec>;
#[doc = "ACMPOUT port/pin select"]
pub mod acmp0_acmpoutroute;
#[doc = "ACMP1_ROUTEEN (rw) register accessor: ACMP1 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`acmp1_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acmp1_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acmp1_routeen`] module"]
#[doc(alias = "ACMP1_ROUTEEN")]
pub type Acmp1Routeen = crate::Reg<acmp1_routeen::Acmp1RouteenSpec>;
#[doc = "ACMP1 pin enable"]
pub mod acmp1_routeen;
#[doc = "ACMP1_ACMPOUTROUTE (rw) register accessor: ACMPOUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`acmp1_acmpoutroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acmp1_acmpoutroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acmp1_acmpoutroute`] module"]
#[doc(alias = "ACMP1_ACMPOUTROUTE")]
pub type Acmp1Acmpoutroute = crate::Reg<acmp1_acmpoutroute::Acmp1AcmpoutrouteSpec>;
#[doc = "ACMPOUT port/pin select"]
pub mod acmp1_acmpoutroute;
#[doc = "CMU_ROUTEEN (rw) register accessor: CMU pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`cmu_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmu_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmu_routeen`] module"]
#[doc(alias = "CMU_ROUTEEN")]
pub type CmuRouteen = crate::Reg<cmu_routeen::CmuRouteenSpec>;
#[doc = "CMU pin enable"]
pub mod cmu_routeen;
#[doc = "CMU_CLKIN0ROUTE (rw) register accessor: CLKIN0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`cmu_clkin0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmu_clkin0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmu_clkin0route`] module"]
#[doc(alias = "CMU_CLKIN0ROUTE")]
pub type CmuClkin0route = crate::Reg<cmu_clkin0route::CmuClkin0routeSpec>;
#[doc = "CLKIN0 port/pin select"]
pub mod cmu_clkin0route;
#[doc = "CMU_CLKOUT0ROUTE (rw) register accessor: CLKOUT0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`cmu_clkout0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmu_clkout0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmu_clkout0route`] module"]
#[doc(alias = "CMU_CLKOUT0ROUTE")]
pub type CmuClkout0route = crate::Reg<cmu_clkout0route::CmuClkout0routeSpec>;
#[doc = "CLKOUT0 port/pin select"]
pub mod cmu_clkout0route;
#[doc = "CMU_CLKOUT1ROUTE (rw) register accessor: CLKOUT1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`cmu_clkout1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmu_clkout1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmu_clkout1route`] module"]
#[doc(alias = "CMU_CLKOUT1ROUTE")]
pub type CmuClkout1route = crate::Reg<cmu_clkout1route::CmuClkout1routeSpec>;
#[doc = "CLKOUT1 port/pin select"]
pub mod cmu_clkout1route;
#[doc = "CMU_CLKOUT2ROUTE (rw) register accessor: CLKOUT2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`cmu_clkout2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmu_clkout2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmu_clkout2route`] module"]
#[doc(alias = "CMU_CLKOUT2ROUTE")]
pub type CmuClkout2route = crate::Reg<cmu_clkout2route::CmuClkout2routeSpec>;
#[doc = "CLKOUT2 port/pin select"]
pub mod cmu_clkout2route;
#[doc = "DCDC_ROUTEEN (rw) register accessor: DCDC pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_routeen`] module"]
#[doc(alias = "DCDC_ROUTEEN")]
pub type DcdcRouteen = crate::Reg<dcdc_routeen::DcdcRouteenSpec>;
#[doc = "DCDC pin enable"]
pub mod dcdc_routeen;
#[doc = "EUSART0_ROUTEEN (rw) register accessor: EUSART0 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart0_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart0_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart0_routeen`] module"]
#[doc(alias = "EUSART0_ROUTEEN")]
pub type Eusart0Routeen = crate::Reg<eusart0_routeen::Eusart0RouteenSpec>;
#[doc = "EUSART0 pin enable"]
pub mod eusart0_routeen;
#[doc = "EUSART0_CSROUTE (rw) register accessor: CS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart0_csroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart0_csroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart0_csroute`] module"]
#[doc(alias = "EUSART0_CSROUTE")]
pub type Eusart0Csroute = crate::Reg<eusart0_csroute::Eusart0CsrouteSpec>;
#[doc = "CS port/pin select"]
pub mod eusart0_csroute;
#[doc = "EUSART0_CTSROUTE (rw) register accessor: CTS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart0_ctsroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart0_ctsroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart0_ctsroute`] module"]
#[doc(alias = "EUSART0_CTSROUTE")]
pub type Eusart0Ctsroute = crate::Reg<eusart0_ctsroute::Eusart0CtsrouteSpec>;
#[doc = "CTS port/pin select"]
pub mod eusart0_ctsroute;
#[doc = "EUSART0_RTSROUTE (rw) register accessor: RTS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart0_rtsroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart0_rtsroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart0_rtsroute`] module"]
#[doc(alias = "EUSART0_RTSROUTE")]
pub type Eusart0Rtsroute = crate::Reg<eusart0_rtsroute::Eusart0RtsrouteSpec>;
#[doc = "RTS port/pin select"]
pub mod eusart0_rtsroute;
#[doc = "EUSART0_RXROUTE (rw) register accessor: RX port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart0_rxroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart0_rxroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart0_rxroute`] module"]
#[doc(alias = "EUSART0_RXROUTE")]
pub type Eusart0Rxroute = crate::Reg<eusart0_rxroute::Eusart0RxrouteSpec>;
#[doc = "RX port/pin select"]
pub mod eusart0_rxroute;
#[doc = "EUSART0_SCLKROUTE (rw) register accessor: SCLK port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart0_sclkroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart0_sclkroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart0_sclkroute`] module"]
#[doc(alias = "EUSART0_SCLKROUTE")]
pub type Eusart0Sclkroute = crate::Reg<eusart0_sclkroute::Eusart0SclkrouteSpec>;
#[doc = "SCLK port/pin select"]
pub mod eusart0_sclkroute;
#[doc = "EUSART0_TXROUTE (rw) register accessor: TX port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart0_txroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart0_txroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart0_txroute`] module"]
#[doc(alias = "EUSART0_TXROUTE")]
pub type Eusart0Txroute = crate::Reg<eusart0_txroute::Eusart0TxrouteSpec>;
#[doc = "TX port/pin select"]
pub mod eusart0_txroute;
#[doc = "EUSART1_ROUTEEN (rw) register accessor: EUSART1 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart1_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart1_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart1_routeen`] module"]
#[doc(alias = "EUSART1_ROUTEEN")]
pub type Eusart1Routeen = crate::Reg<eusart1_routeen::Eusart1RouteenSpec>;
#[doc = "EUSART1 pin enable"]
pub mod eusart1_routeen;
#[doc = "EUSART1_CSROUTE (rw) register accessor: CS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart1_csroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart1_csroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart1_csroute`] module"]
#[doc(alias = "EUSART1_CSROUTE")]
pub type Eusart1Csroute = crate::Reg<eusart1_csroute::Eusart1CsrouteSpec>;
#[doc = "CS port/pin select"]
pub mod eusart1_csroute;
#[doc = "EUSART1_CTSROUTE (rw) register accessor: CTS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart1_ctsroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart1_ctsroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart1_ctsroute`] module"]
#[doc(alias = "EUSART1_CTSROUTE")]
pub type Eusart1Ctsroute = crate::Reg<eusart1_ctsroute::Eusart1CtsrouteSpec>;
#[doc = "CTS port/pin select"]
pub mod eusart1_ctsroute;
#[doc = "EUSART1_RTSROUTE (rw) register accessor: RTS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart1_rtsroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart1_rtsroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart1_rtsroute`] module"]
#[doc(alias = "EUSART1_RTSROUTE")]
pub type Eusart1Rtsroute = crate::Reg<eusart1_rtsroute::Eusart1RtsrouteSpec>;
#[doc = "RTS port/pin select"]
pub mod eusart1_rtsroute;
#[doc = "EUSART1_RXROUTE (rw) register accessor: RX port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart1_rxroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart1_rxroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart1_rxroute`] module"]
#[doc(alias = "EUSART1_RXROUTE")]
pub type Eusart1Rxroute = crate::Reg<eusart1_rxroute::Eusart1RxrouteSpec>;
#[doc = "RX port/pin select"]
pub mod eusart1_rxroute;
#[doc = "EUSART1_SCLKROUTE (rw) register accessor: SCLK port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart1_sclkroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart1_sclkroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart1_sclkroute`] module"]
#[doc(alias = "EUSART1_SCLKROUTE")]
pub type Eusart1Sclkroute = crate::Reg<eusart1_sclkroute::Eusart1SclkrouteSpec>;
#[doc = "SCLK port/pin select"]
pub mod eusart1_sclkroute;
#[doc = "EUSART1_TXROUTE (rw) register accessor: TX port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart1_txroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart1_txroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart1_txroute`] module"]
#[doc(alias = "EUSART1_TXROUTE")]
pub type Eusart1Txroute = crate::Reg<eusart1_txroute::Eusart1TxrouteSpec>;
#[doc = "TX port/pin select"]
pub mod eusart1_txroute;
#[doc = "EUSART2_ROUTEEN (rw) register accessor: EUSART2 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart2_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart2_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart2_routeen`] module"]
#[doc(alias = "EUSART2_ROUTEEN")]
pub type Eusart2Routeen = crate::Reg<eusart2_routeen::Eusart2RouteenSpec>;
#[doc = "EUSART2 pin enable"]
pub mod eusart2_routeen;
#[doc = "EUSART2_CSROUTE (rw) register accessor: CS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart2_csroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart2_csroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart2_csroute`] module"]
#[doc(alias = "EUSART2_CSROUTE")]
pub type Eusart2Csroute = crate::Reg<eusart2_csroute::Eusart2CsrouteSpec>;
#[doc = "CS port/pin select"]
pub mod eusart2_csroute;
#[doc = "EUSART2_CTSROUTE (rw) register accessor: CTS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart2_ctsroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart2_ctsroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart2_ctsroute`] module"]
#[doc(alias = "EUSART2_CTSROUTE")]
pub type Eusart2Ctsroute = crate::Reg<eusart2_ctsroute::Eusart2CtsrouteSpec>;
#[doc = "CTS port/pin select"]
pub mod eusart2_ctsroute;
#[doc = "EUSART2_RTSROUTE (rw) register accessor: RTS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart2_rtsroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart2_rtsroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart2_rtsroute`] module"]
#[doc(alias = "EUSART2_RTSROUTE")]
pub type Eusart2Rtsroute = crate::Reg<eusart2_rtsroute::Eusart2RtsrouteSpec>;
#[doc = "RTS port/pin select"]
pub mod eusart2_rtsroute;
#[doc = "EUSART2_RXROUTE (rw) register accessor: RX port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart2_rxroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart2_rxroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart2_rxroute`] module"]
#[doc(alias = "EUSART2_RXROUTE")]
pub type Eusart2Rxroute = crate::Reg<eusart2_rxroute::Eusart2RxrouteSpec>;
#[doc = "RX port/pin select"]
pub mod eusart2_rxroute;
#[doc = "EUSART2_SCLKROUTE (rw) register accessor: SCLK port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart2_sclkroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart2_sclkroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart2_sclkroute`] module"]
#[doc(alias = "EUSART2_SCLKROUTE")]
pub type Eusart2Sclkroute = crate::Reg<eusart2_sclkroute::Eusart2SclkrouteSpec>;
#[doc = "SCLK port/pin select"]
pub mod eusart2_sclkroute;
#[doc = "EUSART2_TXROUTE (rw) register accessor: TX port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart2_txroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart2_txroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart2_txroute`] module"]
#[doc(alias = "EUSART2_TXROUTE")]
pub type Eusart2Txroute = crate::Reg<eusart2_txroute::Eusart2TxrouteSpec>;
#[doc = "TX port/pin select"]
pub mod eusart2_txroute;
#[doc = "EUSART3_ROUTEEN (rw) register accessor: EUSART3 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart3_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart3_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart3_routeen`] module"]
#[doc(alias = "EUSART3_ROUTEEN")]
pub type Eusart3Routeen = crate::Reg<eusart3_routeen::Eusart3RouteenSpec>;
#[doc = "EUSART3 pin enable"]
pub mod eusart3_routeen;
#[doc = "EUSART3_CSROUTE (rw) register accessor: CS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart3_csroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart3_csroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart3_csroute`] module"]
#[doc(alias = "EUSART3_CSROUTE")]
pub type Eusart3Csroute = crate::Reg<eusart3_csroute::Eusart3CsrouteSpec>;
#[doc = "CS port/pin select"]
pub mod eusart3_csroute;
#[doc = "EUSART3_CTSROUTE (rw) register accessor: CTS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart3_ctsroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart3_ctsroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart3_ctsroute`] module"]
#[doc(alias = "EUSART3_CTSROUTE")]
pub type Eusart3Ctsroute = crate::Reg<eusart3_ctsroute::Eusart3CtsrouteSpec>;
#[doc = "CTS port/pin select"]
pub mod eusart3_ctsroute;
#[doc = "EUSART3_RTSROUTE (rw) register accessor: RTS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart3_rtsroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart3_rtsroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart3_rtsroute`] module"]
#[doc(alias = "EUSART3_RTSROUTE")]
pub type Eusart3Rtsroute = crate::Reg<eusart3_rtsroute::Eusart3RtsrouteSpec>;
#[doc = "RTS port/pin select"]
pub mod eusart3_rtsroute;
#[doc = "EUSART3_RXROUTE (rw) register accessor: RX port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart3_rxroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart3_rxroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart3_rxroute`] module"]
#[doc(alias = "EUSART3_RXROUTE")]
pub type Eusart3Rxroute = crate::Reg<eusart3_rxroute::Eusart3RxrouteSpec>;
#[doc = "RX port/pin select"]
pub mod eusart3_rxroute;
#[doc = "EUSART3_SCLKROUTE (rw) register accessor: SCLK port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart3_sclkroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart3_sclkroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart3_sclkroute`] module"]
#[doc(alias = "EUSART3_SCLKROUTE")]
pub type Eusart3Sclkroute = crate::Reg<eusart3_sclkroute::Eusart3SclkrouteSpec>;
#[doc = "SCLK port/pin select"]
pub mod eusart3_sclkroute;
#[doc = "EUSART3_TXROUTE (rw) register accessor: TX port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart3_txroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart3_txroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart3_txroute`] module"]
#[doc(alias = "EUSART3_TXROUTE")]
pub type Eusart3Txroute = crate::Reg<eusart3_txroute::Eusart3TxrouteSpec>;
#[doc = "TX port/pin select"]
pub mod eusart3_txroute;
#[doc = "EUSART4_ROUTEEN (rw) register accessor: EUSART4 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart4_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart4_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart4_routeen`] module"]
#[doc(alias = "EUSART4_ROUTEEN")]
pub type Eusart4Routeen = crate::Reg<eusart4_routeen::Eusart4RouteenSpec>;
#[doc = "EUSART4 pin enable"]
pub mod eusart4_routeen;
#[doc = "EUSART4_CSROUTE (rw) register accessor: CS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart4_csroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart4_csroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart4_csroute`] module"]
#[doc(alias = "EUSART4_CSROUTE")]
pub type Eusart4Csroute = crate::Reg<eusart4_csroute::Eusart4CsrouteSpec>;
#[doc = "CS port/pin select"]
pub mod eusart4_csroute;
#[doc = "EUSART4_CTSROUTE (rw) register accessor: CTS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart4_ctsroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart4_ctsroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart4_ctsroute`] module"]
#[doc(alias = "EUSART4_CTSROUTE")]
pub type Eusart4Ctsroute = crate::Reg<eusart4_ctsroute::Eusart4CtsrouteSpec>;
#[doc = "CTS port/pin select"]
pub mod eusart4_ctsroute;
#[doc = "EUSART4_RTSROUTE (rw) register accessor: RTS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart4_rtsroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart4_rtsroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart4_rtsroute`] module"]
#[doc(alias = "EUSART4_RTSROUTE")]
pub type Eusart4Rtsroute = crate::Reg<eusart4_rtsroute::Eusart4RtsrouteSpec>;
#[doc = "RTS port/pin select"]
pub mod eusart4_rtsroute;
#[doc = "EUSART4_RXROUTE (rw) register accessor: RX port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart4_rxroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart4_rxroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart4_rxroute`] module"]
#[doc(alias = "EUSART4_RXROUTE")]
pub type Eusart4Rxroute = crate::Reg<eusart4_rxroute::Eusart4RxrouteSpec>;
#[doc = "RX port/pin select"]
pub mod eusart4_rxroute;
#[doc = "EUSART4_SCLKROUTE (rw) register accessor: SCLK port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart4_sclkroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart4_sclkroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart4_sclkroute`] module"]
#[doc(alias = "EUSART4_SCLKROUTE")]
pub type Eusart4Sclkroute = crate::Reg<eusart4_sclkroute::Eusart4SclkrouteSpec>;
#[doc = "SCLK port/pin select"]
pub mod eusart4_sclkroute;
#[doc = "EUSART4_TXROUTE (rw) register accessor: TX port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart4_txroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart4_txroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart4_txroute`] module"]
#[doc(alias = "EUSART4_TXROUTE")]
pub type Eusart4Txroute = crate::Reg<eusart4_txroute::Eusart4TxrouteSpec>;
#[doc = "TX port/pin select"]
pub mod eusart4_txroute;
#[doc = "FRC_ROUTEEN (rw) register accessor: FRC pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`frc_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frc_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frc_routeen`] module"]
#[doc(alias = "FRC_ROUTEEN")]
pub type FrcRouteen = crate::Reg<frc_routeen::FrcRouteenSpec>;
#[doc = "FRC pin enable"]
pub mod frc_routeen;
#[doc = "FRC_DCLKROUTE (rw) register accessor: DCLK port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`frc_dclkroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frc_dclkroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frc_dclkroute`] module"]
#[doc(alias = "FRC_DCLKROUTE")]
pub type FrcDclkroute = crate::Reg<frc_dclkroute::FrcDclkrouteSpec>;
#[doc = "DCLK port/pin select"]
pub mod frc_dclkroute;
#[doc = "FRC_DFRAMEROUTE (rw) register accessor: DFRAME port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`frc_dframeroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frc_dframeroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frc_dframeroute`] module"]
#[doc(alias = "FRC_DFRAMEROUTE")]
pub type FrcDframeroute = crate::Reg<frc_dframeroute::FrcDframerouteSpec>;
#[doc = "DFRAME port/pin select"]
pub mod frc_dframeroute;
#[doc = "FRC_DOUTROUTE (rw) register accessor: DOUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`frc_doutroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frc_doutroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frc_doutroute`] module"]
#[doc(alias = "FRC_DOUTROUTE")]
pub type FrcDoutroute = crate::Reg<frc_doutroute::FrcDoutrouteSpec>;
#[doc = "DOUT port/pin select"]
pub mod frc_doutroute;
#[doc = "I2C0_ROUTEEN (rw) register accessor: I2C0 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_routeen`] module"]
#[doc(alias = "I2C0_ROUTEEN")]
pub type I2c0Routeen = crate::Reg<i2c0_routeen::I2c0RouteenSpec>;
#[doc = "I2C0 pin enable"]
pub mod i2c0_routeen;
#[doc = "I2C0_SCLROUTE (rw) register accessor: SCL port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_sclroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_sclroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_sclroute`] module"]
#[doc(alias = "I2C0_SCLROUTE")]
pub type I2c0Sclroute = crate::Reg<i2c0_sclroute::I2c0SclrouteSpec>;
#[doc = "SCL port/pin select"]
pub mod i2c0_sclroute;
#[doc = "I2C0_SDAROUTE (rw) register accessor: SDA port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_sdaroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_sdaroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_sdaroute`] module"]
#[doc(alias = "I2C0_SDAROUTE")]
pub type I2c0Sdaroute = crate::Reg<i2c0_sdaroute::I2c0SdarouteSpec>;
#[doc = "SDA port/pin select"]
pub mod i2c0_sdaroute;
#[doc = "I2C1_ROUTEEN (rw) register accessor: I2C1 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_routeen`] module"]
#[doc(alias = "I2C1_ROUTEEN")]
pub type I2c1Routeen = crate::Reg<i2c1_routeen::I2c1RouteenSpec>;
#[doc = "I2C1 pin enable"]
pub mod i2c1_routeen;
#[doc = "I2C1_SCLROUTE (rw) register accessor: SCL port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_sclroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_sclroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_sclroute`] module"]
#[doc(alias = "I2C1_SCLROUTE")]
pub type I2c1Sclroute = crate::Reg<i2c1_sclroute::I2c1SclrouteSpec>;
#[doc = "SCL port/pin select"]
pub mod i2c1_sclroute;
#[doc = "I2C1_SDAROUTE (rw) register accessor: SDA port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_sdaroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_sdaroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_sdaroute`] module"]
#[doc(alias = "I2C1_SDAROUTE")]
pub type I2c1Sdaroute = crate::Reg<i2c1_sdaroute::I2c1SdarouteSpec>;
#[doc = "SDA port/pin select"]
pub mod i2c1_sdaroute;
#[doc = "LESENSE_ROUTEEN (rw) register accessor: LESENSE pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_routeen`] module"]
#[doc(alias = "LESENSE_ROUTEEN")]
pub type LesenseRouteen = crate::Reg<lesense_routeen::LesenseRouteenSpec>;
#[doc = "LESENSE pin enable"]
pub mod lesense_routeen;
#[doc = "LESENSE_CH0OUTROUTE (rw) register accessor: CH0OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch0outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch0outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch0outroute`] module"]
#[doc(alias = "LESENSE_CH0OUTROUTE")]
pub type LesenseCh0outroute = crate::Reg<lesense_ch0outroute::LesenseCh0outrouteSpec>;
#[doc = "CH0OUT port/pin select"]
pub mod lesense_ch0outroute;
#[doc = "LESENSE_CH1OUTROUTE (rw) register accessor: CH1OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch1outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch1outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch1outroute`] module"]
#[doc(alias = "LESENSE_CH1OUTROUTE")]
pub type LesenseCh1outroute = crate::Reg<lesense_ch1outroute::LesenseCh1outrouteSpec>;
#[doc = "CH1OUT port/pin select"]
pub mod lesense_ch1outroute;
#[doc = "LESENSE_CH2OUTROUTE (rw) register accessor: CH2OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch2outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch2outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch2outroute`] module"]
#[doc(alias = "LESENSE_CH2OUTROUTE")]
pub type LesenseCh2outroute = crate::Reg<lesense_ch2outroute::LesenseCh2outrouteSpec>;
#[doc = "CH2OUT port/pin select"]
pub mod lesense_ch2outroute;
#[doc = "LESENSE_CH3OUTROUTE (rw) register accessor: CH3OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch3outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch3outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch3outroute`] module"]
#[doc(alias = "LESENSE_CH3OUTROUTE")]
pub type LesenseCh3outroute = crate::Reg<lesense_ch3outroute::LesenseCh3outrouteSpec>;
#[doc = "CH3OUT port/pin select"]
pub mod lesense_ch3outroute;
#[doc = "LESENSE_CH4OUTROUTE (rw) register accessor: CH4OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch4outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch4outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch4outroute`] module"]
#[doc(alias = "LESENSE_CH4OUTROUTE")]
pub type LesenseCh4outroute = crate::Reg<lesense_ch4outroute::LesenseCh4outrouteSpec>;
#[doc = "CH4OUT port/pin select"]
pub mod lesense_ch4outroute;
#[doc = "LESENSE_CH5OUTROUTE (rw) register accessor: CH5OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch5outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch5outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch5outroute`] module"]
#[doc(alias = "LESENSE_CH5OUTROUTE")]
pub type LesenseCh5outroute = crate::Reg<lesense_ch5outroute::LesenseCh5outrouteSpec>;
#[doc = "CH5OUT port/pin select"]
pub mod lesense_ch5outroute;
#[doc = "LESENSE_CH6OUTROUTE (rw) register accessor: CH6OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch6outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch6outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch6outroute`] module"]
#[doc(alias = "LESENSE_CH6OUTROUTE")]
pub type LesenseCh6outroute = crate::Reg<lesense_ch6outroute::LesenseCh6outrouteSpec>;
#[doc = "CH6OUT port/pin select"]
pub mod lesense_ch6outroute;
#[doc = "LESENSE_CH7OUTROUTE (rw) register accessor: CH7OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch7outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch7outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch7outroute`] module"]
#[doc(alias = "LESENSE_CH7OUTROUTE")]
pub type LesenseCh7outroute = crate::Reg<lesense_ch7outroute::LesenseCh7outrouteSpec>;
#[doc = "CH7OUT port/pin select"]
pub mod lesense_ch7outroute;
#[doc = "LESENSE_CH8OUTROUTE (rw) register accessor: CH8OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch8outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch8outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch8outroute`] module"]
#[doc(alias = "LESENSE_CH8OUTROUTE")]
pub type LesenseCh8outroute = crate::Reg<lesense_ch8outroute::LesenseCh8outrouteSpec>;
#[doc = "CH8OUT port/pin select"]
pub mod lesense_ch8outroute;
#[doc = "LESENSE_CH9OUTROUTE (rw) register accessor: CH9OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch9outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch9outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch9outroute`] module"]
#[doc(alias = "LESENSE_CH9OUTROUTE")]
pub type LesenseCh9outroute = crate::Reg<lesense_ch9outroute::LesenseCh9outrouteSpec>;
#[doc = "CH9OUT port/pin select"]
pub mod lesense_ch9outroute;
#[doc = "LESENSE_CH10OUTROUTE (rw) register accessor: CH10OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch10outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch10outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch10outroute`] module"]
#[doc(alias = "LESENSE_CH10OUTROUTE")]
pub type LesenseCh10outroute = crate::Reg<lesense_ch10outroute::LesenseCh10outrouteSpec>;
#[doc = "CH10OUT port/pin select"]
pub mod lesense_ch10outroute;
#[doc = "LESENSE_CH11OUTROUTE (rw) register accessor: CH11OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch11outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch11outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch11outroute`] module"]
#[doc(alias = "LESENSE_CH11OUTROUTE")]
pub type LesenseCh11outroute = crate::Reg<lesense_ch11outroute::LesenseCh11outrouteSpec>;
#[doc = "CH11OUT port/pin select"]
pub mod lesense_ch11outroute;
#[doc = "LESENSE_CH12OUTROUTE (rw) register accessor: CH12OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch12outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch12outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch12outroute`] module"]
#[doc(alias = "LESENSE_CH12OUTROUTE")]
pub type LesenseCh12outroute = crate::Reg<lesense_ch12outroute::LesenseCh12outrouteSpec>;
#[doc = "CH12OUT port/pin select"]
pub mod lesense_ch12outroute;
#[doc = "LESENSE_CH13OUTROUTE (rw) register accessor: CH13OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch13outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch13outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch13outroute`] module"]
#[doc(alias = "LESENSE_CH13OUTROUTE")]
pub type LesenseCh13outroute = crate::Reg<lesense_ch13outroute::LesenseCh13outrouteSpec>;
#[doc = "CH13OUT port/pin select"]
pub mod lesense_ch13outroute;
#[doc = "LESENSE_CH14OUTROUTE (rw) register accessor: CH14OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch14outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch14outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch14outroute`] module"]
#[doc(alias = "LESENSE_CH14OUTROUTE")]
pub type LesenseCh14outroute = crate::Reg<lesense_ch14outroute::LesenseCh14outrouteSpec>;
#[doc = "CH14OUT port/pin select"]
pub mod lesense_ch14outroute;
#[doc = "LESENSE_CH15OUTROUTE (rw) register accessor: CH15OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch15outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch15outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch15outroute`] module"]
#[doc(alias = "LESENSE_CH15OUTROUTE")]
pub type LesenseCh15outroute = crate::Reg<lesense_ch15outroute::LesenseCh15outrouteSpec>;
#[doc = "CH15OUT port/pin select"]
pub mod lesense_ch15outroute;
#[doc = "LETIMER_ROUTEEN (rw) register accessor: LETIMER pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`letimer_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`letimer_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@letimer_routeen`] module"]
#[doc(alias = "LETIMER_ROUTEEN")]
pub type LetimerRouteen = crate::Reg<letimer_routeen::LetimerRouteenSpec>;
#[doc = "LETIMER pin enable"]
pub mod letimer_routeen;
#[doc = "LETIMER_OUT0ROUTE (rw) register accessor: OUT0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`letimer_out0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`letimer_out0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@letimer_out0route`] module"]
#[doc(alias = "LETIMER_OUT0ROUTE")]
pub type LetimerOut0route = crate::Reg<letimer_out0route::LetimerOut0routeSpec>;
#[doc = "OUT0 port/pin select"]
pub mod letimer_out0route;
#[doc = "LETIMER_OUT1ROUTE (rw) register accessor: OUT1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`letimer_out1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`letimer_out1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@letimer_out1route`] module"]
#[doc(alias = "LETIMER_OUT1ROUTE")]
pub type LetimerOut1route = crate::Reg<letimer_out1route::LetimerOut1routeSpec>;
#[doc = "OUT1 port/pin select"]
pub mod letimer_out1route;
#[doc = "MODEM_ROUTEEN (rw) register accessor: MODEM pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_routeen`] module"]
#[doc(alias = "MODEM_ROUTEEN")]
pub type ModemRouteen = crate::Reg<modem_routeen::ModemRouteenSpec>;
#[doc = "MODEM pin enable"]
pub mod modem_routeen;
#[doc = "MODEM_ANT0ROUTE (rw) register accessor: ANT0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_ant0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_ant0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_ant0route`] module"]
#[doc(alias = "MODEM_ANT0ROUTE")]
pub type ModemAnt0route = crate::Reg<modem_ant0route::ModemAnt0routeSpec>;
#[doc = "ANT0 port/pin select"]
pub mod modem_ant0route;
#[doc = "MODEM_ANT1ROUTE (rw) register accessor: ANT1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_ant1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_ant1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_ant1route`] module"]
#[doc(alias = "MODEM_ANT1ROUTE")]
pub type ModemAnt1route = crate::Reg<modem_ant1route::ModemAnt1routeSpec>;
#[doc = "ANT1 port/pin select"]
pub mod modem_ant1route;
#[doc = "MODEM_ANTROLLOVERROUTE (rw) register accessor: ANTROLLOVER port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_antrolloverroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_antrolloverroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_antrolloverroute`] module"]
#[doc(alias = "MODEM_ANTROLLOVERROUTE")]
pub type ModemAntrolloverroute = crate::Reg<modem_antrolloverroute::ModemAntrolloverrouteSpec>;
#[doc = "ANTROLLOVER port/pin select"]
pub mod modem_antrolloverroute;
#[doc = "MODEM_ANTRR0ROUTE (rw) register accessor: ANTRR0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_antrr0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_antrr0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_antrr0route`] module"]
#[doc(alias = "MODEM_ANTRR0ROUTE")]
pub type ModemAntrr0route = crate::Reg<modem_antrr0route::ModemAntrr0routeSpec>;
#[doc = "ANTRR0 port/pin select"]
pub mod modem_antrr0route;
#[doc = "MODEM_ANTRR1ROUTE (rw) register accessor: ANTRR1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_antrr1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_antrr1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_antrr1route`] module"]
#[doc(alias = "MODEM_ANTRR1ROUTE")]
pub type ModemAntrr1route = crate::Reg<modem_antrr1route::ModemAntrr1routeSpec>;
#[doc = "ANTRR1 port/pin select"]
pub mod modem_antrr1route;
#[doc = "MODEM_ANTRR2ROUTE (rw) register accessor: ANTRR2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_antrr2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_antrr2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_antrr2route`] module"]
#[doc(alias = "MODEM_ANTRR2ROUTE")]
pub type ModemAntrr2route = crate::Reg<modem_antrr2route::ModemAntrr2routeSpec>;
#[doc = "ANTRR2 port/pin select"]
pub mod modem_antrr2route;
#[doc = "MODEM_ANTRR3ROUTE (rw) register accessor: ANTRR3 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_antrr3route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_antrr3route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_antrr3route`] module"]
#[doc(alias = "MODEM_ANTRR3ROUTE")]
pub type ModemAntrr3route = crate::Reg<modem_antrr3route::ModemAntrr3routeSpec>;
#[doc = "ANTRR3 port/pin select"]
pub mod modem_antrr3route;
#[doc = "MODEM_ANTRR4ROUTE (rw) register accessor: ANTRR4 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_antrr4route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_antrr4route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_antrr4route`] module"]
#[doc(alias = "MODEM_ANTRR4ROUTE")]
pub type ModemAntrr4route = crate::Reg<modem_antrr4route::ModemAntrr4routeSpec>;
#[doc = "ANTRR4 port/pin select"]
pub mod modem_antrr4route;
#[doc = "MODEM_ANTRR5ROUTE (rw) register accessor: ANTRR5 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_antrr5route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_antrr5route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_antrr5route`] module"]
#[doc(alias = "MODEM_ANTRR5ROUTE")]
pub type ModemAntrr5route = crate::Reg<modem_antrr5route::ModemAntrr5routeSpec>;
#[doc = "ANTRR5 port/pin select"]
pub mod modem_antrr5route;
#[doc = "MODEM_ANTSWENROUTE (rw) register accessor: ANTSWEN port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_antswenroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_antswenroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_antswenroute`] module"]
#[doc(alias = "MODEM_ANTSWENROUTE")]
pub type ModemAntswenroute = crate::Reg<modem_antswenroute::ModemAntswenrouteSpec>;
#[doc = "ANTSWEN port/pin select"]
pub mod modem_antswenroute;
#[doc = "MODEM_ANTSWUSROUTE (rw) register accessor: ANTSWUS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_antswusroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_antswusroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_antswusroute`] module"]
#[doc(alias = "MODEM_ANTSWUSROUTE")]
pub type ModemAntswusroute = crate::Reg<modem_antswusroute::ModemAntswusrouteSpec>;
#[doc = "ANTSWUS port/pin select"]
pub mod modem_antswusroute;
#[doc = "MODEM_ANTTRIGROUTE (rw) register accessor: ANTTRIG port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_anttrigroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_anttrigroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_anttrigroute`] module"]
#[doc(alias = "MODEM_ANTTRIGROUTE")]
pub type ModemAnttrigroute = crate::Reg<modem_anttrigroute::ModemAnttrigrouteSpec>;
#[doc = "ANTTRIG port/pin select"]
pub mod modem_anttrigroute;
#[doc = "MODEM_ANTTRIGSTOPROUTE (rw) register accessor: ANTTRIGSTOP port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_anttrigstoproute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_anttrigstoproute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_anttrigstoproute`] module"]
#[doc(alias = "MODEM_ANTTRIGSTOPROUTE")]
pub type ModemAnttrigstoproute = crate::Reg<modem_anttrigstoproute::ModemAnttrigstoprouteSpec>;
#[doc = "ANTTRIGSTOP port/pin select"]
pub mod modem_anttrigstoproute;
#[doc = "MODEM_DCLKROUTE (rw) register accessor: DCLK port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_dclkroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_dclkroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_dclkroute`] module"]
#[doc(alias = "MODEM_DCLKROUTE")]
pub type ModemDclkroute = crate::Reg<modem_dclkroute::ModemDclkrouteSpec>;
#[doc = "DCLK port/pin select"]
pub mod modem_dclkroute;
#[doc = "MODEM_DINROUTE (rw) register accessor: DIN port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_dinroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_dinroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_dinroute`] module"]
#[doc(alias = "MODEM_DINROUTE")]
pub type ModemDinroute = crate::Reg<modem_dinroute::ModemDinrouteSpec>;
#[doc = "DIN port/pin select"]
pub mod modem_dinroute;
#[doc = "MODEM_DOUTROUTE (rw) register accessor: DOUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_doutroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_doutroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_doutroute`] module"]
#[doc(alias = "MODEM_DOUTROUTE")]
pub type ModemDoutroute = crate::Reg<modem_doutroute::ModemDoutrouteSpec>;
#[doc = "DOUT port/pin select"]
pub mod modem_doutroute;
#[doc = "PCNT0_S0INROUTE (rw) register accessor: S0IN port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnt0_s0inroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnt0_s0inroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcnt0_s0inroute`] module"]
#[doc(alias = "PCNT0_S0INROUTE")]
pub type Pcnt0S0inroute = crate::Reg<pcnt0_s0inroute::Pcnt0S0inrouteSpec>;
#[doc = "S0IN port/pin select"]
pub mod pcnt0_s0inroute;
#[doc = "PCNT0_S1INROUTE (rw) register accessor: S1IN port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnt0_s1inroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnt0_s1inroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcnt0_s1inroute`] module"]
#[doc(alias = "PCNT0_S1INROUTE")]
pub type Pcnt0S1inroute = crate::Reg<pcnt0_s1inroute::Pcnt0S1inrouteSpec>;
#[doc = "S1IN port/pin select"]
pub mod pcnt0_s1inroute;
#[doc = "PRS0_ROUTEEN (rw) register accessor: PRS0 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_routeen`] module"]
#[doc(alias = "PRS0_ROUTEEN")]
pub type Prs0Routeen = crate::Reg<prs0_routeen::Prs0RouteenSpec>;
#[doc = "PRS0 pin enable"]
pub mod prs0_routeen;
#[doc = "PRS0_ASYNCH0ROUTE (rw) register accessor: ASYNCH0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_asynch0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_asynch0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_asynch0route`] module"]
#[doc(alias = "PRS0_ASYNCH0ROUTE")]
pub type Prs0Asynch0route = crate::Reg<prs0_asynch0route::Prs0Asynch0routeSpec>;
#[doc = "ASYNCH0 port/pin select"]
pub mod prs0_asynch0route;
#[doc = "PRS0_ASYNCH1ROUTE (rw) register accessor: ASYNCH1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_asynch1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_asynch1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_asynch1route`] module"]
#[doc(alias = "PRS0_ASYNCH1ROUTE")]
pub type Prs0Asynch1route = crate::Reg<prs0_asynch1route::Prs0Asynch1routeSpec>;
#[doc = "ASYNCH1 port/pin select"]
pub mod prs0_asynch1route;
#[doc = "PRS0_ASYNCH2ROUTE (rw) register accessor: ASYNCH2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_asynch2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_asynch2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_asynch2route`] module"]
#[doc(alias = "PRS0_ASYNCH2ROUTE")]
pub type Prs0Asynch2route = crate::Reg<prs0_asynch2route::Prs0Asynch2routeSpec>;
#[doc = "ASYNCH2 port/pin select"]
pub mod prs0_asynch2route;
#[doc = "PRS0_ASYNCH3ROUTE (rw) register accessor: ASYNCH3 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_asynch3route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_asynch3route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_asynch3route`] module"]
#[doc(alias = "PRS0_ASYNCH3ROUTE")]
pub type Prs0Asynch3route = crate::Reg<prs0_asynch3route::Prs0Asynch3routeSpec>;
#[doc = "ASYNCH3 port/pin select"]
pub mod prs0_asynch3route;
#[doc = "PRS0_ASYNCH4ROUTE (rw) register accessor: ASYNCH4 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_asynch4route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_asynch4route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_asynch4route`] module"]
#[doc(alias = "PRS0_ASYNCH4ROUTE")]
pub type Prs0Asynch4route = crate::Reg<prs0_asynch4route::Prs0Asynch4routeSpec>;
#[doc = "ASYNCH4 port/pin select"]
pub mod prs0_asynch4route;
#[doc = "PRS0_ASYNCH5ROUTE (rw) register accessor: ASYNCH5 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_asynch5route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_asynch5route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_asynch5route`] module"]
#[doc(alias = "PRS0_ASYNCH5ROUTE")]
pub type Prs0Asynch5route = crate::Reg<prs0_asynch5route::Prs0Asynch5routeSpec>;
#[doc = "ASYNCH5 port/pin select"]
pub mod prs0_asynch5route;
#[doc = "PRS0_ASYNCH6ROUTE (rw) register accessor: ASYNCH6 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_asynch6route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_asynch6route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_asynch6route`] module"]
#[doc(alias = "PRS0_ASYNCH6ROUTE")]
pub type Prs0Asynch6route = crate::Reg<prs0_asynch6route::Prs0Asynch6routeSpec>;
#[doc = "ASYNCH6 port/pin select"]
pub mod prs0_asynch6route;
#[doc = "PRS0_ASYNCH7ROUTE (rw) register accessor: ASYNCH7 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_asynch7route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_asynch7route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_asynch7route`] module"]
#[doc(alias = "PRS0_ASYNCH7ROUTE")]
pub type Prs0Asynch7route = crate::Reg<prs0_asynch7route::Prs0Asynch7routeSpec>;
#[doc = "ASYNCH7 port/pin select"]
pub mod prs0_asynch7route;
#[doc = "PRS0_ASYNCH8ROUTE (rw) register accessor: ASYNCH8 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_asynch8route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_asynch8route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_asynch8route`] module"]
#[doc(alias = "PRS0_ASYNCH8ROUTE")]
pub type Prs0Asynch8route = crate::Reg<prs0_asynch8route::Prs0Asynch8routeSpec>;
#[doc = "ASYNCH8 port/pin select"]
pub mod prs0_asynch8route;
#[doc = "PRS0_ASYNCH9ROUTE (rw) register accessor: ASYNCH9 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_asynch9route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_asynch9route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_asynch9route`] module"]
#[doc(alias = "PRS0_ASYNCH9ROUTE")]
pub type Prs0Asynch9route = crate::Reg<prs0_asynch9route::Prs0Asynch9routeSpec>;
#[doc = "ASYNCH9 port/pin select"]
pub mod prs0_asynch9route;
#[doc = "PRS0_ASYNCH10ROUTE (rw) register accessor: ASYNCH10 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_asynch10route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_asynch10route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_asynch10route`] module"]
#[doc(alias = "PRS0_ASYNCH10ROUTE")]
pub type Prs0Asynch10route = crate::Reg<prs0_asynch10route::Prs0Asynch10routeSpec>;
#[doc = "ASYNCH10 port/pin select"]
pub mod prs0_asynch10route;
#[doc = "PRS0_ASYNCH11ROUTE (rw) register accessor: ASYNCH11 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_asynch11route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_asynch11route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_asynch11route`] module"]
#[doc(alias = "PRS0_ASYNCH11ROUTE")]
pub type Prs0Asynch11route = crate::Reg<prs0_asynch11route::Prs0Asynch11routeSpec>;
#[doc = "ASYNCH11 port/pin select"]
pub mod prs0_asynch11route;
#[doc = "PRS0_SYNCH0ROUTE (rw) register accessor: SYNCH0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_synch0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_synch0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_synch0route`] module"]
#[doc(alias = "PRS0_SYNCH0ROUTE")]
pub type Prs0Synch0route = crate::Reg<prs0_synch0route::Prs0Synch0routeSpec>;
#[doc = "SYNCH0 port/pin select"]
pub mod prs0_synch0route;
#[doc = "PRS0_SYNCH1ROUTE (rw) register accessor: SYNCH1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_synch1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_synch1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_synch1route`] module"]
#[doc(alias = "PRS0_SYNCH1ROUTE")]
pub type Prs0Synch1route = crate::Reg<prs0_synch1route::Prs0Synch1routeSpec>;
#[doc = "SYNCH1 port/pin select"]
pub mod prs0_synch1route;
#[doc = "PRS0_SYNCH2ROUTE (rw) register accessor: SYNCH2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_synch2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_synch2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_synch2route`] module"]
#[doc(alias = "PRS0_SYNCH2ROUTE")]
pub type Prs0Synch2route = crate::Reg<prs0_synch2route::Prs0Synch2routeSpec>;
#[doc = "SYNCH2 port/pin select"]
pub mod prs0_synch2route;
#[doc = "PRS0_SYNCH3ROUTE (rw) register accessor: SYNCH3 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_synch3route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_synch3route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_synch3route`] module"]
#[doc(alias = "PRS0_SYNCH3ROUTE")]
pub type Prs0Synch3route = crate::Reg<prs0_synch3route::Prs0Synch3routeSpec>;
#[doc = "SYNCH3 port/pin select"]
pub mod prs0_synch3route;
#[doc = "SYXO0_BUFOUTREQINASYNCROUTE (rw) register accessor: BUFOUTREQINASYNC port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`syxo0_bufoutreqinasyncroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syxo0_bufoutreqinasyncroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syxo0_bufoutreqinasyncroute`] module"]
#[doc(alias = "SYXO0_BUFOUTREQINASYNCROUTE")]
pub type Syxo0Bufoutreqinasyncroute =
    crate::Reg<syxo0_bufoutreqinasyncroute::Syxo0BufoutreqinasyncrouteSpec>;
#[doc = "BUFOUTREQINASYNC port/pin select"]
pub mod syxo0_bufoutreqinasyncroute;
#[doc = "TIMER0_ROUTEEN (rw) register accessor: TIMER0 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`timer0_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0_routeen`] module"]
#[doc(alias = "TIMER0_ROUTEEN")]
pub type Timer0Routeen = crate::Reg<timer0_routeen::Timer0RouteenSpec>;
#[doc = "TIMER0 pin enable"]
pub mod timer0_routeen;
#[doc = "TIMER0_CC0ROUTE (rw) register accessor: CC0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer0_cc0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0_cc0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0_cc0route`] module"]
#[doc(alias = "TIMER0_CC0ROUTE")]
pub type Timer0Cc0route = crate::Reg<timer0_cc0route::Timer0Cc0routeSpec>;
#[doc = "CC0 port/pin select"]
pub mod timer0_cc0route;
#[doc = "TIMER0_CC1ROUTE (rw) register accessor: CC1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer0_cc1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0_cc1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0_cc1route`] module"]
#[doc(alias = "TIMER0_CC1ROUTE")]
pub type Timer0Cc1route = crate::Reg<timer0_cc1route::Timer0Cc1routeSpec>;
#[doc = "CC1 port/pin select"]
pub mod timer0_cc1route;
#[doc = "TIMER0_CC2ROUTE (rw) register accessor: CC2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer0_cc2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0_cc2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0_cc2route`] module"]
#[doc(alias = "TIMER0_CC2ROUTE")]
pub type Timer0Cc2route = crate::Reg<timer0_cc2route::Timer0Cc2routeSpec>;
#[doc = "CC2 port/pin select"]
pub mod timer0_cc2route;
#[doc = "TIMER0_CDTI0ROUTE (rw) register accessor: CDTI0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer0_cdti0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0_cdti0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0_cdti0route`] module"]
#[doc(alias = "TIMER0_CDTI0ROUTE")]
pub type Timer0Cdti0route = crate::Reg<timer0_cdti0route::Timer0Cdti0routeSpec>;
#[doc = "CDTI0 port/pin select"]
pub mod timer0_cdti0route;
#[doc = "TIMER0_CDTI1ROUTE (rw) register accessor: CDTI1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer0_cdti1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0_cdti1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0_cdti1route`] module"]
#[doc(alias = "TIMER0_CDTI1ROUTE")]
pub type Timer0Cdti1route = crate::Reg<timer0_cdti1route::Timer0Cdti1routeSpec>;
#[doc = "CDTI1 port/pin select"]
pub mod timer0_cdti1route;
#[doc = "TIMER0_CDTI2ROUTE (rw) register accessor: CDTI2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer0_cdti2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0_cdti2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0_cdti2route`] module"]
#[doc(alias = "TIMER0_CDTI2ROUTE")]
pub type Timer0Cdti2route = crate::Reg<timer0_cdti2route::Timer0Cdti2routeSpec>;
#[doc = "CDTI2 port/pin select"]
pub mod timer0_cdti2route;
#[doc = "TIMER1_ROUTEEN (rw) register accessor: TIMER1 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1_routeen`] module"]
#[doc(alias = "TIMER1_ROUTEEN")]
pub type Timer1Routeen = crate::Reg<timer1_routeen::Timer1RouteenSpec>;
#[doc = "TIMER1 pin enable"]
pub mod timer1_routeen;
#[doc = "TIMER1_CC0ROUTE (rw) register accessor: CC0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1_cc0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1_cc0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1_cc0route`] module"]
#[doc(alias = "TIMER1_CC0ROUTE")]
pub type Timer1Cc0route = crate::Reg<timer1_cc0route::Timer1Cc0routeSpec>;
#[doc = "CC0 port/pin select"]
pub mod timer1_cc0route;
#[doc = "TIMER1_CC1ROUTE (rw) register accessor: CC1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1_cc1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1_cc1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1_cc1route`] module"]
#[doc(alias = "TIMER1_CC1ROUTE")]
pub type Timer1Cc1route = crate::Reg<timer1_cc1route::Timer1Cc1routeSpec>;
#[doc = "CC1 port/pin select"]
pub mod timer1_cc1route;
#[doc = "TIMER1_CC2ROUTE (rw) register accessor: CC2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1_cc2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1_cc2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1_cc2route`] module"]
#[doc(alias = "TIMER1_CC2ROUTE")]
pub type Timer1Cc2route = crate::Reg<timer1_cc2route::Timer1Cc2routeSpec>;
#[doc = "CC2 port/pin select"]
pub mod timer1_cc2route;
#[doc = "TIMER1_CDTI0ROUTE (rw) register accessor: CDTI0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1_cdti0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1_cdti0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1_cdti0route`] module"]
#[doc(alias = "TIMER1_CDTI0ROUTE")]
pub type Timer1Cdti0route = crate::Reg<timer1_cdti0route::Timer1Cdti0routeSpec>;
#[doc = "CDTI0 port/pin select"]
pub mod timer1_cdti0route;
#[doc = "TIMER1_CDTI1ROUTE (rw) register accessor: CDTI1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1_cdti1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1_cdti1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1_cdti1route`] module"]
#[doc(alias = "TIMER1_CDTI1ROUTE")]
pub type Timer1Cdti1route = crate::Reg<timer1_cdti1route::Timer1Cdti1routeSpec>;
#[doc = "CDTI1 port/pin select"]
pub mod timer1_cdti1route;
#[doc = "TIMER1_CDTI2ROUTE (rw) register accessor: CDTI2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1_cdti2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1_cdti2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1_cdti2route`] module"]
#[doc(alias = "TIMER1_CDTI2ROUTE")]
pub type Timer1Cdti2route = crate::Reg<timer1_cdti2route::Timer1Cdti2routeSpec>;
#[doc = "CDTI2 port/pin select"]
pub mod timer1_cdti2route;
#[doc = "TIMER2_ROUTEEN (rw) register accessor: TIMER2 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_routeen`] module"]
#[doc(alias = "TIMER2_ROUTEEN")]
pub type Timer2Routeen = crate::Reg<timer2_routeen::Timer2RouteenSpec>;
#[doc = "TIMER2 pin enable"]
pub mod timer2_routeen;
#[doc = "TIMER2_CC0ROUTE (rw) register accessor: CC0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2_cc0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2_cc0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_cc0route`] module"]
#[doc(alias = "TIMER2_CC0ROUTE")]
pub type Timer2Cc0route = crate::Reg<timer2_cc0route::Timer2Cc0routeSpec>;
#[doc = "CC0 port/pin select"]
pub mod timer2_cc0route;
#[doc = "TIMER2_CC1ROUTE (rw) register accessor: CC1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2_cc1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2_cc1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_cc1route`] module"]
#[doc(alias = "TIMER2_CC1ROUTE")]
pub type Timer2Cc1route = crate::Reg<timer2_cc1route::Timer2Cc1routeSpec>;
#[doc = "CC1 port/pin select"]
pub mod timer2_cc1route;
#[doc = "TIMER2_CC2ROUTE (rw) register accessor: CC2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2_cc2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2_cc2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_cc2route`] module"]
#[doc(alias = "TIMER2_CC2ROUTE")]
pub type Timer2Cc2route = crate::Reg<timer2_cc2route::Timer2Cc2routeSpec>;
#[doc = "CC2 port/pin select"]
pub mod timer2_cc2route;
#[doc = "TIMER2_CDTI0ROUTE (rw) register accessor: CDTI0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2_cdti0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2_cdti0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_cdti0route`] module"]
#[doc(alias = "TIMER2_CDTI0ROUTE")]
pub type Timer2Cdti0route = crate::Reg<timer2_cdti0route::Timer2Cdti0routeSpec>;
#[doc = "CDTI0 port/pin select"]
pub mod timer2_cdti0route;
#[doc = "TIMER2_CDTI1ROUTE (rw) register accessor: CDTI1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2_cdti1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2_cdti1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_cdti1route`] module"]
#[doc(alias = "TIMER2_CDTI1ROUTE")]
pub type Timer2Cdti1route = crate::Reg<timer2_cdti1route::Timer2Cdti1routeSpec>;
#[doc = "CDTI1 port/pin select"]
pub mod timer2_cdti1route;
#[doc = "TIMER2_CDTI2ROUTE (rw) register accessor: CDTI2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2_cdti2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2_cdti2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_cdti2route`] module"]
#[doc(alias = "TIMER2_CDTI2ROUTE")]
pub type Timer2Cdti2route = crate::Reg<timer2_cdti2route::Timer2Cdti2routeSpec>;
#[doc = "CDTI2 port/pin select"]
pub mod timer2_cdti2route;
#[doc = "TIMER3_ROUTEEN (rw) register accessor: TIMER3 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`timer3_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer3_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer3_routeen`] module"]
#[doc(alias = "TIMER3_ROUTEEN")]
pub type Timer3Routeen = crate::Reg<timer3_routeen::Timer3RouteenSpec>;
#[doc = "TIMER3 pin enable"]
pub mod timer3_routeen;
#[doc = "TIMER3_CC0ROUTE (rw) register accessor: CC0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer3_cc0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer3_cc0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer3_cc0route`] module"]
#[doc(alias = "TIMER3_CC0ROUTE")]
pub type Timer3Cc0route = crate::Reg<timer3_cc0route::Timer3Cc0routeSpec>;
#[doc = "CC0 port/pin select"]
pub mod timer3_cc0route;
#[doc = "TIMER3_CC1ROUTE (rw) register accessor: CC1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer3_cc1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer3_cc1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer3_cc1route`] module"]
#[doc(alias = "TIMER3_CC1ROUTE")]
pub type Timer3Cc1route = crate::Reg<timer3_cc1route::Timer3Cc1routeSpec>;
#[doc = "CC1 port/pin select"]
pub mod timer3_cc1route;
#[doc = "TIMER3_CC2ROUTE (rw) register accessor: CC2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer3_cc2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer3_cc2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer3_cc2route`] module"]
#[doc(alias = "TIMER3_CC2ROUTE")]
pub type Timer3Cc2route = crate::Reg<timer3_cc2route::Timer3Cc2routeSpec>;
#[doc = "CC2 port/pin select"]
pub mod timer3_cc2route;
#[doc = "TIMER3_CDTI0ROUTE (rw) register accessor: CDTI0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer3_cdti0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer3_cdti0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer3_cdti0route`] module"]
#[doc(alias = "TIMER3_CDTI0ROUTE")]
pub type Timer3Cdti0route = crate::Reg<timer3_cdti0route::Timer3Cdti0routeSpec>;
#[doc = "CDTI0 port/pin select"]
pub mod timer3_cdti0route;
#[doc = "TIMER3_CDTI1ROUTE (rw) register accessor: CDTI1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer3_cdti1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer3_cdti1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer3_cdti1route`] module"]
#[doc(alias = "TIMER3_CDTI1ROUTE")]
pub type Timer3Cdti1route = crate::Reg<timer3_cdti1route::Timer3Cdti1routeSpec>;
#[doc = "CDTI1 port/pin select"]
pub mod timer3_cdti1route;
#[doc = "TIMER3_CDTI2ROUTE (rw) register accessor: CDTI2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer3_cdti2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer3_cdti2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer3_cdti2route`] module"]
#[doc(alias = "TIMER3_CDTI2ROUTE")]
pub type Timer3Cdti2route = crate::Reg<timer3_cdti2route::Timer3Cdti2routeSpec>;
#[doc = "CDTI2 port/pin select"]
pub mod timer3_cdti2route;
#[doc = "TIMER4_ROUTEEN (rw) register accessor: TIMER4 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`timer4_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer4_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer4_routeen`] module"]
#[doc(alias = "TIMER4_ROUTEEN")]
pub type Timer4Routeen = crate::Reg<timer4_routeen::Timer4RouteenSpec>;
#[doc = "TIMER4 pin enable"]
pub mod timer4_routeen;
#[doc = "TIMER4_CC0ROUTE (rw) register accessor: CC0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer4_cc0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer4_cc0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer4_cc0route`] module"]
#[doc(alias = "TIMER4_CC0ROUTE")]
pub type Timer4Cc0route = crate::Reg<timer4_cc0route::Timer4Cc0routeSpec>;
#[doc = "CC0 port/pin select"]
pub mod timer4_cc0route;
#[doc = "TIMER4_CC1ROUTE (rw) register accessor: CC1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer4_cc1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer4_cc1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer4_cc1route`] module"]
#[doc(alias = "TIMER4_CC1ROUTE")]
pub type Timer4Cc1route = crate::Reg<timer4_cc1route::Timer4Cc1routeSpec>;
#[doc = "CC1 port/pin select"]
pub mod timer4_cc1route;
#[doc = "TIMER4_CC2ROUTE (rw) register accessor: CC2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer4_cc2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer4_cc2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer4_cc2route`] module"]
#[doc(alias = "TIMER4_CC2ROUTE")]
pub type Timer4Cc2route = crate::Reg<timer4_cc2route::Timer4Cc2routeSpec>;
#[doc = "CC2 port/pin select"]
pub mod timer4_cc2route;
#[doc = "TIMER4_CDTI0ROUTE (rw) register accessor: CDTI0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer4_cdti0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer4_cdti0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer4_cdti0route`] module"]
#[doc(alias = "TIMER4_CDTI0ROUTE")]
pub type Timer4Cdti0route = crate::Reg<timer4_cdti0route::Timer4Cdti0routeSpec>;
#[doc = "CDTI0 port/pin select"]
pub mod timer4_cdti0route;
#[doc = "TIMER4_CDTI1ROUTE (rw) register accessor: CDTI1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer4_cdti1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer4_cdti1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer4_cdti1route`] module"]
#[doc(alias = "TIMER4_CDTI1ROUTE")]
pub type Timer4Cdti1route = crate::Reg<timer4_cdti1route::Timer4Cdti1routeSpec>;
#[doc = "CDTI1 port/pin select"]
pub mod timer4_cdti1route;
#[doc = "TIMER4_CDTI2ROUTE (rw) register accessor: CDTI2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer4_cdti2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer4_cdti2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer4_cdti2route`] module"]
#[doc(alias = "TIMER4_CDTI2ROUTE")]
pub type Timer4Cdti2route = crate::Reg<timer4_cdti2route::Timer4Cdti2routeSpec>;
#[doc = "CDTI2 port/pin select"]
pub mod timer4_cdti2route;
#[doc = "TIMER5_ROUTEEN (rw) register accessor: TIMER5 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`timer5_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer5_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer5_routeen`] module"]
#[doc(alias = "TIMER5_ROUTEEN")]
pub type Timer5Routeen = crate::Reg<timer5_routeen::Timer5RouteenSpec>;
#[doc = "TIMER5 pin enable"]
pub mod timer5_routeen;
#[doc = "TIMER5_CC0ROUTE (rw) register accessor: CC0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer5_cc0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer5_cc0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer5_cc0route`] module"]
#[doc(alias = "TIMER5_CC0ROUTE")]
pub type Timer5Cc0route = crate::Reg<timer5_cc0route::Timer5Cc0routeSpec>;
#[doc = "CC0 port/pin select"]
pub mod timer5_cc0route;
#[doc = "TIMER5_CC1ROUTE (rw) register accessor: CC1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer5_cc1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer5_cc1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer5_cc1route`] module"]
#[doc(alias = "TIMER5_CC1ROUTE")]
pub type Timer5Cc1route = crate::Reg<timer5_cc1route::Timer5Cc1routeSpec>;
#[doc = "CC1 port/pin select"]
pub mod timer5_cc1route;
#[doc = "TIMER5_CC2ROUTE (rw) register accessor: CC2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer5_cc2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer5_cc2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer5_cc2route`] module"]
#[doc(alias = "TIMER5_CC2ROUTE")]
pub type Timer5Cc2route = crate::Reg<timer5_cc2route::Timer5Cc2routeSpec>;
#[doc = "CC2 port/pin select"]
pub mod timer5_cc2route;
#[doc = "TIMER5_CDTI0ROUTE (rw) register accessor: CDTI0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer5_cdti0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer5_cdti0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer5_cdti0route`] module"]
#[doc(alias = "TIMER5_CDTI0ROUTE")]
pub type Timer5Cdti0route = crate::Reg<timer5_cdti0route::Timer5Cdti0routeSpec>;
#[doc = "CDTI0 port/pin select"]
pub mod timer5_cdti0route;
#[doc = "TIMER5_CDTI1ROUTE (rw) register accessor: CDTI1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer5_cdti1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer5_cdti1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer5_cdti1route`] module"]
#[doc(alias = "TIMER5_CDTI1ROUTE")]
pub type Timer5Cdti1route = crate::Reg<timer5_cdti1route::Timer5Cdti1routeSpec>;
#[doc = "CDTI1 port/pin select"]
pub mod timer5_cdti1route;
#[doc = "TIMER5_CDTI2ROUTE (rw) register accessor: CDTI2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer5_cdti2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer5_cdti2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer5_cdti2route`] module"]
#[doc(alias = "TIMER5_CDTI2ROUTE")]
pub type Timer5Cdti2route = crate::Reg<timer5_cdti2route::Timer5Cdti2routeSpec>;
#[doc = "CDTI2 port/pin select"]
pub mod timer5_cdti2route;
#[doc = "TIMER6_ROUTEEN (rw) register accessor: TIMER6 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`timer6_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer6_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer6_routeen`] module"]
#[doc(alias = "TIMER6_ROUTEEN")]
pub type Timer6Routeen = crate::Reg<timer6_routeen::Timer6RouteenSpec>;
#[doc = "TIMER6 pin enable"]
pub mod timer6_routeen;
#[doc = "TIMER6_CC0ROUTE (rw) register accessor: CC0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer6_cc0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer6_cc0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer6_cc0route`] module"]
#[doc(alias = "TIMER6_CC0ROUTE")]
pub type Timer6Cc0route = crate::Reg<timer6_cc0route::Timer6Cc0routeSpec>;
#[doc = "CC0 port/pin select"]
pub mod timer6_cc0route;
#[doc = "TIMER6_CC1ROUTE (rw) register accessor: CC1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer6_cc1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer6_cc1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer6_cc1route`] module"]
#[doc(alias = "TIMER6_CC1ROUTE")]
pub type Timer6Cc1route = crate::Reg<timer6_cc1route::Timer6Cc1routeSpec>;
#[doc = "CC1 port/pin select"]
pub mod timer6_cc1route;
#[doc = "TIMER6_CC2ROUTE (rw) register accessor: CC2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer6_cc2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer6_cc2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer6_cc2route`] module"]
#[doc(alias = "TIMER6_CC2ROUTE")]
pub type Timer6Cc2route = crate::Reg<timer6_cc2route::Timer6Cc2routeSpec>;
#[doc = "CC2 port/pin select"]
pub mod timer6_cc2route;
#[doc = "TIMER6_CDTI0ROUTE (rw) register accessor: CDTI0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer6_cdti0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer6_cdti0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer6_cdti0route`] module"]
#[doc(alias = "TIMER6_CDTI0ROUTE")]
pub type Timer6Cdti0route = crate::Reg<timer6_cdti0route::Timer6Cdti0routeSpec>;
#[doc = "CDTI0 port/pin select"]
pub mod timer6_cdti0route;
#[doc = "TIMER6_CDTI1ROUTE (rw) register accessor: CDTI1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer6_cdti1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer6_cdti1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer6_cdti1route`] module"]
#[doc(alias = "TIMER6_CDTI1ROUTE")]
pub type Timer6Cdti1route = crate::Reg<timer6_cdti1route::Timer6Cdti1routeSpec>;
#[doc = "CDTI1 port/pin select"]
pub mod timer6_cdti1route;
#[doc = "TIMER6_CDTI2ROUTE (rw) register accessor: CDTI2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer6_cdti2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer6_cdti2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer6_cdti2route`] module"]
#[doc(alias = "TIMER6_CDTI2ROUTE")]
pub type Timer6Cdti2route = crate::Reg<timer6_cdti2route::Timer6Cdti2routeSpec>;
#[doc = "CDTI2 port/pin select"]
pub mod timer6_cdti2route;
#[doc = "TIMER7_ROUTEEN (rw) register accessor: TIMER7 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`timer7_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer7_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer7_routeen`] module"]
#[doc(alias = "TIMER7_ROUTEEN")]
pub type Timer7Routeen = crate::Reg<timer7_routeen::Timer7RouteenSpec>;
#[doc = "TIMER7 pin enable"]
pub mod timer7_routeen;
#[doc = "TIMER7_CC0ROUTE (rw) register accessor: CC0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer7_cc0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer7_cc0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer7_cc0route`] module"]
#[doc(alias = "TIMER7_CC0ROUTE")]
pub type Timer7Cc0route = crate::Reg<timer7_cc0route::Timer7Cc0routeSpec>;
#[doc = "CC0 port/pin select"]
pub mod timer7_cc0route;
#[doc = "TIMER7_CC1ROUTE (rw) register accessor: CC1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer7_cc1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer7_cc1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer7_cc1route`] module"]
#[doc(alias = "TIMER7_CC1ROUTE")]
pub type Timer7Cc1route = crate::Reg<timer7_cc1route::Timer7Cc1routeSpec>;
#[doc = "CC1 port/pin select"]
pub mod timer7_cc1route;
#[doc = "TIMER7_CC2ROUTE (rw) register accessor: CC2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer7_cc2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer7_cc2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer7_cc2route`] module"]
#[doc(alias = "TIMER7_CC2ROUTE")]
pub type Timer7Cc2route = crate::Reg<timer7_cc2route::Timer7Cc2routeSpec>;
#[doc = "CC2 port/pin select"]
pub mod timer7_cc2route;
#[doc = "TIMER7_CDTI0ROUTE (rw) register accessor: CDTI0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer7_cdti0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer7_cdti0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer7_cdti0route`] module"]
#[doc(alias = "TIMER7_CDTI0ROUTE")]
pub type Timer7Cdti0route = crate::Reg<timer7_cdti0route::Timer7Cdti0routeSpec>;
#[doc = "CDTI0 port/pin select"]
pub mod timer7_cdti0route;
#[doc = "TIMER7_CDTI1ROUTE (rw) register accessor: CDTI1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer7_cdti1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer7_cdti1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer7_cdti1route`] module"]
#[doc(alias = "TIMER7_CDTI1ROUTE")]
pub type Timer7Cdti1route = crate::Reg<timer7_cdti1route::Timer7Cdti1routeSpec>;
#[doc = "CDTI1 port/pin select"]
pub mod timer7_cdti1route;
#[doc = "TIMER7_CDTI2ROUTE (rw) register accessor: CDTI2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer7_cdti2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer7_cdti2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer7_cdti2route`] module"]
#[doc(alias = "TIMER7_CDTI2ROUTE")]
pub type Timer7Cdti2route = crate::Reg<timer7_cdti2route::Timer7Cdti2routeSpec>;
#[doc = "CDTI2 port/pin select"]
pub mod timer7_cdti2route;
#[doc = "USB_USBVBUSSENSEROUTE (rw) register accessor: USBVBUSSENSE port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_usbvbussenseroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_usbvbussenseroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_usbvbussenseroute`] module"]
#[doc(alias = "USB_USBVBUSSENSEROUTE")]
pub type UsbUsbvbussenseroute = crate::Reg<usb_usbvbussenseroute::UsbUsbvbussenserouteSpec>;
#[doc = "USBVBUSSENSE port/pin select"]
pub mod usb_usbvbussenseroute;
