#[doc = "Register `GAHBCFG` reader"]
pub type R = crate::R<GahbcfgSpec>;
#[doc = "Register `GAHBCFG` writer"]
pub type W = crate::W<GahbcfgSpec>;
#[doc = "Field `GLBLINTRMSK` reader - Global Interrupt Mask"]
pub type GlblintrmskR = crate::BitReader;
#[doc = "Field `GLBLINTRMSK` writer - Global Interrupt Mask"]
pub type GlblintrmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBSTLEN` reader - Burst Length/Type"]
pub type HbstlenR = crate::FieldReader;
#[doc = "Field `HBSTLEN` writer - Burst Length/Type"]
pub type HbstlenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NPTXFEMPLVL` reader - Non-Periodic TxFIFO Empty Level"]
pub type NptxfemplvlR = crate::BitReader;
#[doc = "Field `NPTXFEMPLVL` writer - Non-Periodic TxFIFO Empty Level"]
pub type NptxfemplvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REMMEMSUPP` reader - Remote Memory Support"]
pub type RemmemsuppR = crate::BitReader;
#[doc = "Field `REMMEMSUPP` writer - Remote Memory Support"]
pub type RemmemsuppW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTIALLDMAWRIT` reader - Notify All Dma Write Trans"]
pub type NotialldmawritR = crate::BitReader;
#[doc = "Field `NOTIALLDMAWRIT` writer - Notify All Dma Write Trans"]
pub type NotialldmawritW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBSINGLE` reader - AHB Single Support"]
pub type AhbsingleR = crate::BitReader;
#[doc = "Field `AHBSINGLE` writer - AHB Single Support"]
pub type AhbsingleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVDESCEND` reader - Inverse Descriptor Endianness"]
pub type InvdescendR = crate::BitReader;
#[doc = "Field `INVDESCEND` writer - Inverse Descriptor Endianness"]
pub type InvdescendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Global Interrupt Mask"]
    #[inline(always)]
    pub fn glblintrmsk(&self) -> GlblintrmskR {
        GlblintrmskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Burst Length/Type"]
    #[inline(always)]
    pub fn hbstlen(&self) -> HbstlenR {
        HbstlenR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Non-Periodic TxFIFO Empty Level"]
    #[inline(always)]
    pub fn nptxfemplvl(&self) -> NptxfemplvlR {
        NptxfemplvlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 21 - Remote Memory Support"]
    #[inline(always)]
    pub fn remmemsupp(&self) -> RemmemsuppR {
        RemmemsuppR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Notify All Dma Write Trans"]
    #[inline(always)]
    pub fn notialldmawrit(&self) -> NotialldmawritR {
        NotialldmawritR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - AHB Single Support"]
    #[inline(always)]
    pub fn ahbsingle(&self) -> AhbsingleR {
        AhbsingleR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Inverse Descriptor Endianness"]
    #[inline(always)]
    pub fn invdescend(&self) -> InvdescendR {
        InvdescendR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global Interrupt Mask"]
    #[inline(always)]
    pub fn glblintrmsk(&mut self) -> GlblintrmskW<'_, GahbcfgSpec> {
        GlblintrmskW::new(self, 0)
    }
    #[doc = "Bits 1:4 - Burst Length/Type"]
    #[inline(always)]
    pub fn hbstlen(&mut self) -> HbstlenW<'_, GahbcfgSpec> {
        HbstlenW::new(self, 1)
    }
    #[doc = "Bit 5 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, GahbcfgSpec> {
        DmaenW::new(self, 5)
    }
    #[doc = "Bit 7 - Non-Periodic TxFIFO Empty Level"]
    #[inline(always)]
    pub fn nptxfemplvl(&mut self) -> NptxfemplvlW<'_, GahbcfgSpec> {
        NptxfemplvlW::new(self, 7)
    }
    #[doc = "Bit 21 - Remote Memory Support"]
    #[inline(always)]
    pub fn remmemsupp(&mut self) -> RemmemsuppW<'_, GahbcfgSpec> {
        RemmemsuppW::new(self, 21)
    }
    #[doc = "Bit 22 - Notify All Dma Write Trans"]
    #[inline(always)]
    pub fn notialldmawrit(&mut self) -> NotialldmawritW<'_, GahbcfgSpec> {
        NotialldmawritW::new(self, 22)
    }
    #[doc = "Bit 23 - AHB Single Support"]
    #[inline(always)]
    pub fn ahbsingle(&mut self) -> AhbsingleW<'_, GahbcfgSpec> {
        AhbsingleW::new(self, 23)
    }
    #[doc = "Bit 24 - Inverse Descriptor Endianness"]
    #[inline(always)]
    pub fn invdescend(&mut self) -> InvdescendW<'_, GahbcfgSpec> {
        InvdescendW::new(self, 24)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`gahbcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gahbcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GahbcfgSpec;
impl crate::RegisterSpec for GahbcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gahbcfg::R`](R) reader structure"]
impl crate::Readable for GahbcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`gahbcfg::W`](W) writer structure"]
impl crate::Writable for GahbcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GAHBCFG to value 0"]
impl crate::Resettable for GahbcfgSpec {}
