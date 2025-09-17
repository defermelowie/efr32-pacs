#[doc = "Register `DTHRCTL` reader"]
pub type R = crate::R<DthrctlSpec>;
#[doc = "Register `DTHRCTL` writer"]
pub type W = crate::W<DthrctlSpec>;
#[doc = "Field `NONISOTHREN` reader - Non-ISO IN EP Threshold Enable"]
pub type NonisothrenR = crate::BitReader;
#[doc = "Field `NONISOTHREN` writer - Non-ISO IN EP Threshold Enable"]
pub type NonisothrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOTHREN` reader - ISO IN EP Threshold Enable"]
pub type IsothrenR = crate::BitReader;
#[doc = "Field `ISOTHREN` writer - ISO IN EP Threshold Enable"]
pub type IsothrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTHRLEN` reader - Transmit threshold length"]
pub type TxthrlenR = crate::FieldReader<u16>;
#[doc = "Field `TXTHRLEN` writer - Transmit threshold length"]
pub type TxthrlenW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `AHBTHRRATIO` reader - AHB Threshold Ratio"]
pub type AhbthrratioR = crate::FieldReader;
#[doc = "Field `AHBTHRRATIO` writer - AHB Threshold Ratio"]
pub type AhbthrratioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXTHREN` reader - Receive Threshold Enable"]
pub type RxthrenR = crate::BitReader;
#[doc = "Field `RXTHREN` writer - Receive Threshold Enable"]
pub type RxthrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTHRLEN` reader - Receive Threshold Length"]
pub type RxthrlenR = crate::FieldReader<u16>;
#[doc = "Field `RXTHRLEN` writer - Receive Threshold Length"]
pub type RxthrlenW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `ARBPRKEN` reader - Arbiter Parking Enable"]
pub type ArbprkenR = crate::BitReader;
#[doc = "Field `ARBPRKEN` writer - Arbiter Parking Enable"]
pub type ArbprkenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Non-ISO IN EP Threshold Enable"]
    #[inline(always)]
    pub fn nonisothren(&self) -> NonisothrenR {
        NonisothrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ISO IN EP Threshold Enable"]
    #[inline(always)]
    pub fn isothren(&self) -> IsothrenR {
        IsothrenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:10 - Transmit threshold length"]
    #[inline(always)]
    pub fn txthrlen(&self) -> TxthrlenR {
        TxthrlenR::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bits 11:12 - AHB Threshold Ratio"]
    #[inline(always)]
    pub fn ahbthrratio(&self) -> AhbthrratioR {
        AhbthrratioR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 16 - Receive Threshold Enable"]
    #[inline(always)]
    pub fn rxthren(&self) -> RxthrenR {
        RxthrenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:25 - Receive Threshold Length"]
    #[inline(always)]
    pub fn rxthrlen(&self) -> RxthrlenR {
        RxthrlenR::new(((self.bits >> 17) & 0x01ff) as u16)
    }
    #[doc = "Bit 27 - Arbiter Parking Enable"]
    #[inline(always)]
    pub fn arbprken(&self) -> ArbprkenR {
        ArbprkenR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non-ISO IN EP Threshold Enable"]
    #[inline(always)]
    pub fn nonisothren(&mut self) -> NonisothrenW<'_, DthrctlSpec> {
        NonisothrenW::new(self, 0)
    }
    #[doc = "Bit 1 - ISO IN EP Threshold Enable"]
    #[inline(always)]
    pub fn isothren(&mut self) -> IsothrenW<'_, DthrctlSpec> {
        IsothrenW::new(self, 1)
    }
    #[doc = "Bits 2:10 - Transmit threshold length"]
    #[inline(always)]
    pub fn txthrlen(&mut self) -> TxthrlenW<'_, DthrctlSpec> {
        TxthrlenW::new(self, 2)
    }
    #[doc = "Bits 11:12 - AHB Threshold Ratio"]
    #[inline(always)]
    pub fn ahbthrratio(&mut self) -> AhbthrratioW<'_, DthrctlSpec> {
        AhbthrratioW::new(self, 11)
    }
    #[doc = "Bit 16 - Receive Threshold Enable"]
    #[inline(always)]
    pub fn rxthren(&mut self) -> RxthrenW<'_, DthrctlSpec> {
        RxthrenW::new(self, 16)
    }
    #[doc = "Bits 17:25 - Receive Threshold Length"]
    #[inline(always)]
    pub fn rxthrlen(&mut self) -> RxthrlenW<'_, DthrctlSpec> {
        RxthrlenW::new(self, 17)
    }
    #[doc = "Bit 27 - Arbiter Parking Enable"]
    #[inline(always)]
    pub fn arbprken(&mut self) -> ArbprkenW<'_, DthrctlSpec> {
        ArbprkenW::new(self, 27)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dthrctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dthrctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DthrctlSpec;
impl crate::RegisterSpec for DthrctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dthrctl::R`](R) reader structure"]
impl crate::Readable for DthrctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dthrctl::W`](W) writer structure"]
impl crate::Writable for DthrctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTHRCTL to value 0x0810_0020"]
impl crate::Resettable for DthrctlSpec {
    const RESET_VALUE: u32 = 0x0810_0020;
}
