#[doc = "Register `GUSBCFG` reader"]
pub type R = crate::R<GusbcfgSpec>;
#[doc = "Register `GUSBCFG` writer"]
pub type W = crate::W<GusbcfgSpec>;
#[doc = "Field `TOUTCAL` reader - HS/FS Timeout cal"]
pub type ToutcalR = crate::FieldReader;
#[doc = "Field `TOUTCAL` writer - HS/FS Timeout cal"]
pub type ToutcalW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHYIF` reader - PHY Interface"]
pub type PhyifR = crate::BitReader;
#[doc = "Field `PHYIF` writer - PHY Interface"]
pub type PhyifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSINTF` reader - Full-Speed Serial Interface Sel"]
pub type FsintfR = crate::BitReader;
#[doc = "Field `FSINTF` writer - Full-Speed Serial Interface Sel"]
pub type FsintfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYSEL` reader - \"2.0 HS sel or 1.1 FS sel"]
pub type PhyselR = crate::BitReader;
#[doc = "Field `PHYSEL` writer - \"2.0 HS sel or 1.1 FS sel"]
pub type PhyselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBTRDTIM` reader - USB turnaround time"]
pub type UsbtrdtimR = crate::FieldReader;
#[doc = "Field `USBTRDTIM` writer - USB turnaround time"]
pub type UsbtrdtimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TERMSELDLPULSE` reader - TermSel DLine Pulsing Selection"]
pub type TermseldlpulseR = crate::BitReader;
#[doc = "Field `TERMSELDLPULSE` writer - TermSel DLine Pulsing Selection"]
pub type TermseldlpulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICUSBCAP` reader - IC_USB-Capable"]
pub type IcusbcapR = crate::BitReader;
#[doc = "Field `TXENDDELAY` reader - Tx End Delay"]
pub type TxenddelayR = crate::BitReader;
#[doc = "Field `TXENDDELAY` writer - Tx End Delay"]
pub type TxenddelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORRUPTTXPKT` writer - corrupt tx packet"]
pub type CorrupttxpktW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - HS/FS Timeout cal"]
    #[inline(always)]
    pub fn toutcal(&self) -> ToutcalR {
        ToutcalR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - PHY Interface"]
    #[inline(always)]
    pub fn phyif(&self) -> PhyifR {
        PhyifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Full-Speed Serial Interface Sel"]
    #[inline(always)]
    pub fn fsintf(&self) -> FsintfR {
        FsintfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - \"2.0 HS sel or 1.1 FS sel"]
    #[inline(always)]
    pub fn physel(&self) -> PhyselR {
        PhyselR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 10:13 - USB turnaround time"]
    #[inline(always)]
    pub fn usbtrdtim(&self) -> UsbtrdtimR {
        UsbtrdtimR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - TermSel DLine Pulsing Selection"]
    #[inline(always)]
    pub fn termseldlpulse(&self) -> TermseldlpulseR {
        TermseldlpulseR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - IC_USB-Capable"]
    #[inline(always)]
    pub fn icusbcap(&self) -> IcusbcapR {
        IcusbcapR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Tx End Delay"]
    #[inline(always)]
    pub fn txenddelay(&self) -> TxenddelayR {
        TxenddelayR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - HS/FS Timeout cal"]
    #[inline(always)]
    pub fn toutcal(&mut self) -> ToutcalW<'_, GusbcfgSpec> {
        ToutcalW::new(self, 0)
    }
    #[doc = "Bit 3 - PHY Interface"]
    #[inline(always)]
    pub fn phyif(&mut self) -> PhyifW<'_, GusbcfgSpec> {
        PhyifW::new(self, 3)
    }
    #[doc = "Bit 5 - Full-Speed Serial Interface Sel"]
    #[inline(always)]
    pub fn fsintf(&mut self) -> FsintfW<'_, GusbcfgSpec> {
        FsintfW::new(self, 5)
    }
    #[doc = "Bit 6 - \"2.0 HS sel or 1.1 FS sel"]
    #[inline(always)]
    pub fn physel(&mut self) -> PhyselW<'_, GusbcfgSpec> {
        PhyselW::new(self, 6)
    }
    #[doc = "Bits 10:13 - USB turnaround time"]
    #[inline(always)]
    pub fn usbtrdtim(&mut self) -> UsbtrdtimW<'_, GusbcfgSpec> {
        UsbtrdtimW::new(self, 10)
    }
    #[doc = "Bit 22 - TermSel DLine Pulsing Selection"]
    #[inline(always)]
    pub fn termseldlpulse(&mut self) -> TermseldlpulseW<'_, GusbcfgSpec> {
        TermseldlpulseW::new(self, 22)
    }
    #[doc = "Bit 28 - Tx End Delay"]
    #[inline(always)]
    pub fn txenddelay(&mut self) -> TxenddelayW<'_, GusbcfgSpec> {
        TxenddelayW::new(self, 28)
    }
    #[doc = "Bit 31 - corrupt tx packet"]
    #[inline(always)]
    pub fn corrupttxpkt(&mut self) -> CorrupttxpktW<'_, GusbcfgSpec> {
        CorrupttxpktW::new(self, 31)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`gusbcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gusbcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GusbcfgSpec;
impl crate::RegisterSpec for GusbcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gusbcfg::R`](R) reader structure"]
impl crate::Readable for GusbcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`gusbcfg::W`](W) writer structure"]
impl crate::Writable for GusbcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GUSBCFG to value 0x1440"]
impl crate::Resettable for GusbcfgSpec {
    const RESET_VALUE: u32 = 0x1440;
}
