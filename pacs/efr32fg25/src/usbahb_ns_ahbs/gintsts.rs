#[doc = "Register `GINTSTS` reader"]
pub type R = crate::R<GintstsSpec>;
#[doc = "Register `GINTSTS` writer"]
pub type W = crate::W<GintstsSpec>;
#[doc = "Field `CURMOD` reader - Current Mode of Operation"]
pub type CurmodR = crate::BitReader;
#[doc = "Field `MODEMIS` reader - Mode Mismatch Interrupt"]
pub type ModemisR = crate::BitReader;
#[doc = "Field `MODEMIS` writer - Mode Mismatch Interrupt"]
pub type ModemisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOF` reader - tart of (micro)Frame"]
pub type SofR = crate::BitReader;
#[doc = "Field `SOF` writer - tart of (micro)Frame"]
pub type SofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFLVL` reader - RxFIFO Non-Empty"]
pub type RxflvlR = crate::BitReader;
#[doc = "Field `NPTxFEmp` reader - Non periodic TXFIFO Empty"]
pub type NptxFempR = crate::BitReader;
#[doc = "Field `GINNAKEFF` reader - Global IN Non-periodic NAK Effective"]
pub type GinnakeffR = crate::BitReader;
#[doc = "Field `GOUTNAKEFF` reader - Global OUT NAK Effective"]
pub type GoutnakeffR = crate::BitReader;
#[doc = "Field `ERLYSUSP` reader - Early Suspend"]
pub type ErlysuspR = crate::BitReader;
#[doc = "Field `ERLYSUSP` writer - Early Suspend"]
pub type ErlysuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSUSP` reader - USB Suspend"]
pub type UsbsuspR = crate::BitReader;
#[doc = "Field `USBSUSP` writer - USB Suspend"]
pub type UsbsuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRST` reader - USB Reset"]
pub type UsbrstR = crate::BitReader;
#[doc = "Field `USBRST` writer - USB Reset"]
pub type UsbrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENUMDONE` reader - Enumeration Done"]
pub type EnumdoneR = crate::BitReader;
#[doc = "Field `ENUMDONE` writer - Enumeration Done"]
pub type EnumdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOOUTDROP` reader - Iso OUT Packet Dropped IRQ"]
pub type IsooutdropR = crate::BitReader;
#[doc = "Field `ISOOUTDROP` writer - Iso OUT Packet Dropped IRQ"]
pub type IsooutdropW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPF` reader - End of Periodic Frame Interrupt"]
pub type EopfR = crate::BitReader;
#[doc = "Field `EOPF` writer - End of Periodic Frame Interrupt"]
pub type EopfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPMIS` reader - Endpoint Mismatch Interrupt"]
pub type EpmisR = crate::BitReader;
#[doc = "Field `EPMIS` writer - Endpoint Mismatch Interrupt"]
pub type EpmisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEPINT` reader - IN Endpoints Interrupt"]
pub type IepintR = crate::BitReader;
#[doc = "Field `OEPINT` reader - OUT Endpoints Interrupt"]
pub type OepintR = crate::BitReader;
#[doc = "Field `INCOMPISOIN` reader - Incomplete Isochronous IN Transfer"]
pub type IncompisoinR = crate::BitReader;
#[doc = "Field `INCOMPLP` reader - Incomplete Periodic Transfer"]
pub type IncomplpR = crate::BitReader;
#[doc = "Field `FETSUSP` reader - Data Fetch Suspended"]
pub type FetsuspR = crate::BitReader;
#[doc = "Field `FETSUSP` writer - Data Fetch Suspended"]
pub type FetsuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETDET` reader - Reset detected Interrupt"]
pub type ResetdetR = crate::BitReader;
#[doc = "Field `RESETDET` writer - Reset detected Interrupt"]
pub type ResetdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONIDSTSCHNG` reader - Connector ID Status Change"]
pub type ConidstschngR = crate::BitReader;
#[doc = "Field `CONIDSTSCHNG` writer - Connector ID Status Change"]
pub type ConidstschngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPINT` reader - Resume/Remote Wakeup Detected Irq"]
pub type WkupintR = crate::BitReader;
#[doc = "Field `WKUPINT` writer - Resume/Remote Wakeup Detected Irq"]
pub type WkupintW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Current Mode of Operation"]
    #[inline(always)]
    pub fn curmod(&self) -> CurmodR {
        CurmodR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mode Mismatch Interrupt"]
    #[inline(always)]
    pub fn modemis(&self) -> ModemisR {
        ModemisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - tart of (micro)Frame"]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RxFIFO Non-Empty"]
    #[inline(always)]
    pub fn rxflvl(&self) -> RxflvlR {
        RxflvlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Non periodic TXFIFO Empty"]
    #[inline(always)]
    pub fn nptx_femp(&self) -> NptxFempR {
        NptxFempR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Global IN Non-periodic NAK Effective"]
    #[inline(always)]
    pub fn ginnakeff(&self) -> GinnakeffR {
        GinnakeffR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK Effective"]
    #[inline(always)]
    pub fn goutnakeff(&self) -> GoutnakeffR {
        GoutnakeffR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Suspend"]
    #[inline(always)]
    pub fn erlysusp(&self) -> ErlysuspR {
        ErlysuspR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB Suspend"]
    #[inline(always)]
    pub fn usbsusp(&self) -> UsbsuspR {
        UsbsuspR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB Reset"]
    #[inline(always)]
    pub fn usbrst(&self) -> UsbrstR {
        UsbrstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enumeration Done"]
    #[inline(always)]
    pub fn enumdone(&self) -> EnumdoneR {
        EnumdoneR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Iso OUT Packet Dropped IRQ"]
    #[inline(always)]
    pub fn isooutdrop(&self) -> IsooutdropR {
        IsooutdropR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt"]
    #[inline(always)]
    pub fn eopf(&self) -> EopfR {
        EopfR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Endpoint Mismatch Interrupt"]
    #[inline(always)]
    pub fn epmis(&self) -> EpmisR {
        EpmisR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt"]
    #[inline(always)]
    pub fn iepint(&self) -> IepintR {
        IepintR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt"]
    #[inline(always)]
    pub fn oepint(&self) -> OepintR {
        OepintR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer"]
    #[inline(always)]
    pub fn incompisoin(&self) -> IncompisoinR {
        IncompisoinR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer"]
    #[inline(always)]
    pub fn incomplp(&self) -> IncomplpR {
        IncomplpR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data Fetch Suspended"]
    #[inline(always)]
    pub fn fetsusp(&self) -> FetsuspR {
        FetsuspR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reset detected Interrupt"]
    #[inline(always)]
    pub fn resetdet(&self) -> ResetdetR {
        ResetdetR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - Connector ID Status Change"]
    #[inline(always)]
    pub fn conidstschng(&self) -> ConidstschngR {
        ConidstschngR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Irq"]
    #[inline(always)]
    pub fn wkupint(&self) -> WkupintR {
        WkupintR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode Mismatch Interrupt"]
    #[inline(always)]
    pub fn modemis(&mut self) -> ModemisW<'_, GintstsSpec> {
        ModemisW::new(self, 1)
    }
    #[doc = "Bit 3 - tart of (micro)Frame"]
    #[inline(always)]
    pub fn sof(&mut self) -> SofW<'_, GintstsSpec> {
        SofW::new(self, 3)
    }
    #[doc = "Bit 10 - Early Suspend"]
    #[inline(always)]
    pub fn erlysusp(&mut self) -> ErlysuspW<'_, GintstsSpec> {
        ErlysuspW::new(self, 10)
    }
    #[doc = "Bit 11 - USB Suspend"]
    #[inline(always)]
    pub fn usbsusp(&mut self) -> UsbsuspW<'_, GintstsSpec> {
        UsbsuspW::new(self, 11)
    }
    #[doc = "Bit 12 - USB Reset"]
    #[inline(always)]
    pub fn usbrst(&mut self) -> UsbrstW<'_, GintstsSpec> {
        UsbrstW::new(self, 12)
    }
    #[doc = "Bit 13 - Enumeration Done"]
    #[inline(always)]
    pub fn enumdone(&mut self) -> EnumdoneW<'_, GintstsSpec> {
        EnumdoneW::new(self, 13)
    }
    #[doc = "Bit 14 - Iso OUT Packet Dropped IRQ"]
    #[inline(always)]
    pub fn isooutdrop(&mut self) -> IsooutdropW<'_, GintstsSpec> {
        IsooutdropW::new(self, 14)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt"]
    #[inline(always)]
    pub fn eopf(&mut self) -> EopfW<'_, GintstsSpec> {
        EopfW::new(self, 15)
    }
    #[doc = "Bit 17 - Endpoint Mismatch Interrupt"]
    #[inline(always)]
    pub fn epmis(&mut self) -> EpmisW<'_, GintstsSpec> {
        EpmisW::new(self, 17)
    }
    #[doc = "Bit 22 - Data Fetch Suspended"]
    #[inline(always)]
    pub fn fetsusp(&mut self) -> FetsuspW<'_, GintstsSpec> {
        FetsuspW::new(self, 22)
    }
    #[doc = "Bit 23 - Reset detected Interrupt"]
    #[inline(always)]
    pub fn resetdet(&mut self) -> ResetdetW<'_, GintstsSpec> {
        ResetdetW::new(self, 23)
    }
    #[doc = "Bit 28 - Connector ID Status Change"]
    #[inline(always)]
    pub fn conidstschng(&mut self) -> ConidstschngW<'_, GintstsSpec> {
        ConidstschngW::new(self, 28)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Irq"]
    #[inline(always)]
    pub fn wkupint(&mut self) -> WkupintW<'_, GintstsSpec> {
        WkupintW::new(self, 31)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`gintsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GintstsSpec;
impl crate::RegisterSpec for GintstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintsts::R`](R) reader structure"]
impl crate::Readable for GintstsSpec {}
#[doc = "`write(|w| ..)` method takes [`gintsts::W`](W) writer structure"]
impl crate::Writable for GintstsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GINTSTS to value 0x1000_0020"]
impl crate::Resettable for GintstsSpec {
    const RESET_VALUE: u32 = 0x1000_0020;
}
