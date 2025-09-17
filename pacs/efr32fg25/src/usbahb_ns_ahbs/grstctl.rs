#[doc = "Register `GRSTCTL` reader"]
pub type R = crate::R<GrstctlSpec>;
#[doc = "Register `GRSTCTL` writer"]
pub type W = crate::W<GrstctlSpec>;
#[doc = "Field `CSFTRST` reader - Core Soft Reset"]
pub type CsftrstR = crate::BitReader;
#[doc = "Field `CSFTRST` writer - Core Soft Reset"]
pub type CsftrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIUFSSFTRST` reader - PIU FS Dedicated Controller Sft rst"]
pub type PiufssftrstR = crate::BitReader;
#[doc = "Field `PIUFSSFTRST` writer - PIU FS Dedicated Controller Sft rst"]
pub type PiufssftrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFFLSH` reader - RxFIFO Flush"]
pub type RxfflshR = crate::BitReader;
#[doc = "Field `RXFFLSH` writer - RxFIFO Flush"]
pub type RxfflshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFFLSH` reader - TxFIFO Flush"]
pub type TxfflshR = crate::BitReader;
#[doc = "Field `TXFFLSH` writer - TxFIFO Flush"]
pub type TxfflshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFNUM` reader - TxFIFO Number"]
pub type TxfnumR = crate::FieldReader;
#[doc = "Field `TXFNUM` writer - TxFIFO Number"]
pub type TxfnumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DMAREQ` reader - DMA Request Signal"]
pub type DmareqR = crate::BitReader;
#[doc = "Field `AHBIDLE` reader - AHB Master Idle"]
pub type AhbidleR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Core Soft Reset"]
    #[inline(always)]
    pub fn csftrst(&self) -> CsftrstR {
        CsftrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PIU FS Dedicated Controller Sft rst"]
    #[inline(always)]
    pub fn piufssftrst(&self) -> PiufssftrstR {
        PiufssftrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - RxFIFO Flush"]
    #[inline(always)]
    pub fn rxfflsh(&self) -> RxfflshR {
        RxfflshR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TxFIFO Flush"]
    #[inline(always)]
    pub fn txfflsh(&self) -> TxfflshR {
        TxfflshR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - TxFIFO Number"]
    #[inline(always)]
    pub fn txfnum(&self) -> TxfnumR {
        TxfnumR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - DMA Request Signal"]
    #[inline(always)]
    pub fn dmareq(&self) -> DmareqR {
        DmareqR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AHB Master Idle"]
    #[inline(always)]
    pub fn ahbidle(&self) -> AhbidleR {
        AhbidleR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core Soft Reset"]
    #[inline(always)]
    pub fn csftrst(&mut self) -> CsftrstW<'_, GrstctlSpec> {
        CsftrstW::new(self, 0)
    }
    #[doc = "Bit 1 - PIU FS Dedicated Controller Sft rst"]
    #[inline(always)]
    pub fn piufssftrst(&mut self) -> PiufssftrstW<'_, GrstctlSpec> {
        PiufssftrstW::new(self, 1)
    }
    #[doc = "Bit 4 - RxFIFO Flush"]
    #[inline(always)]
    pub fn rxfflsh(&mut self) -> RxfflshW<'_, GrstctlSpec> {
        RxfflshW::new(self, 4)
    }
    #[doc = "Bit 5 - TxFIFO Flush"]
    #[inline(always)]
    pub fn txfflsh(&mut self) -> TxfflshW<'_, GrstctlSpec> {
        TxfflshW::new(self, 5)
    }
    #[doc = "Bits 6:10 - TxFIFO Number"]
    #[inline(always)]
    pub fn txfnum(&mut self) -> TxfnumW<'_, GrstctlSpec> {
        TxfnumW::new(self, 6)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`grstctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grstctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrstctlSpec;
impl crate::RegisterSpec for GrstctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grstctl::R`](R) reader structure"]
impl crate::Readable for GrstctlSpec {}
#[doc = "`write(|w| ..)` method takes [`grstctl::W`](W) writer structure"]
impl crate::Writable for GrstctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRSTCTL to value 0"]
impl crate::Resettable for GrstctlSpec {}
