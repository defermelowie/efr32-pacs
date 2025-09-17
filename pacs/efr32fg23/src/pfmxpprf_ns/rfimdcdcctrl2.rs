#[doc = "Register `RFIMDCDCCTRL2` reader"]
pub type R = crate::R<Rfimdcdcctrl2Spec>;
#[doc = "Register `RFIMDCDCCTRL2` writer"]
pub type W = crate::W<Rfimdcdcctrl2Spec>;
#[doc = "Field `PPTMAX` reader - Pulse Pairing Time Max"]
pub type PptmaxR = crate::FieldReader<u16>;
#[doc = "Field `PPTMAX` writer - Pulse Pairing Time Max"]
pub type PptmaxW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PPTMIN` reader - Pulse Pairing Time Min"]
pub type PptminR = crate::FieldReader<u16>;
#[doc = "Field `PPTMIN` writer - Pulse Pairing Time Min"]
pub type PptminW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PPND` reader - Pulse Pairing Period"]
pub type PpndR = crate::FieldReader<u16>;
#[doc = "Field `PPND` writer - Pulse Pairing Period"]
pub type PpndW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PPCALEN` reader - Pulse Pairing Calibration Loop Enable"]
pub type PpcalenR = crate::BitReader;
#[doc = "Field `PPCALEN` writer - Pulse Pairing Calibration Loop Enable"]
pub type PpcalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPSYNCONLY` reader - Pulse Pairing Sync Only"]
pub type PpsynconlyR = crate::BitReader;
#[doc = "Field `PPSYNCONLY` writer - Pulse Pairing Sync Only"]
pub type PpsynconlyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Pulse Pairing Time Max"]
    #[inline(always)]
    pub fn pptmax(&self) -> PptmaxR {
        PptmaxR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - Pulse Pairing Time Min"]
    #[inline(always)]
    pub fn pptmin(&self) -> PptminR {
        PptminR::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bits 18:26 - Pulse Pairing Period"]
    #[inline(always)]
    pub fn ppnd(&self) -> PpndR {
        PpndR::new(((self.bits >> 18) & 0x01ff) as u16)
    }
    #[doc = "Bit 27 - Pulse Pairing Calibration Loop Enable"]
    #[inline(always)]
    pub fn ppcalen(&self) -> PpcalenR {
        PpcalenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Pulse Pairing Sync Only"]
    #[inline(always)]
    pub fn ppsynconly(&self) -> PpsynconlyR {
        PpsynconlyR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Pulse Pairing Time Max"]
    #[inline(always)]
    pub fn pptmax(&mut self) -> PptmaxW<'_, Rfimdcdcctrl2Spec> {
        PptmaxW::new(self, 0)
    }
    #[doc = "Bits 9:17 - Pulse Pairing Time Min"]
    #[inline(always)]
    pub fn pptmin(&mut self) -> PptminW<'_, Rfimdcdcctrl2Spec> {
        PptminW::new(self, 9)
    }
    #[doc = "Bits 18:26 - Pulse Pairing Period"]
    #[inline(always)]
    pub fn ppnd(&mut self) -> PpndW<'_, Rfimdcdcctrl2Spec> {
        PpndW::new(self, 18)
    }
    #[doc = "Bit 27 - Pulse Pairing Calibration Loop Enable"]
    #[inline(always)]
    pub fn ppcalen(&mut self) -> PpcalenW<'_, Rfimdcdcctrl2Spec> {
        PpcalenW::new(self, 27)
    }
    #[doc = "Bit 28 - Pulse Pairing Sync Only"]
    #[inline(always)]
    pub fn ppsynconly(&mut self) -> PpsynconlyW<'_, Rfimdcdcctrl2Spec> {
        PpsynconlyW::new(self, 28)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rfimdcdcctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfimdcdcctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rfimdcdcctrl2Spec;
impl crate::RegisterSpec for Rfimdcdcctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfimdcdcctrl2::R`](R) reader structure"]
impl crate::Readable for Rfimdcdcctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`rfimdcdcctrl2::W`](W) writer structure"]
impl crate::Writable for Rfimdcdcctrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RFIMDCDCCTRL2 to value 0x0ad0_b4a0"]
impl crate::Resettable for Rfimdcdcctrl2Spec {
    const RESET_VALUE: u32 = 0x0ad0_b4a0;
}
