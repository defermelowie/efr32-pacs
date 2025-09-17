#[doc = "Register `CHIPREV` reader"]
pub type R = crate::R<ChiprevSpec>;
#[doc = "Register `CHIPREV` writer"]
pub type W = crate::W<ChiprevSpec>;
#[doc = "Field `PARTNUMBER` reader - Chip Part Number value"]
pub type PartnumberR = crate::FieldReader<u16>;
#[doc = "Field `PARTNUMBER` writer - Chip Part Number value"]
pub type PartnumberW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `MINOR` reader - Chip Revision Minor value"]
pub type MinorR = crate::FieldReader;
#[doc = "Field `MINOR` writer - Chip Revision Minor value"]
pub type MinorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MAJOR` reader - Chip Revision Major value"]
pub type MajorR = crate::FieldReader;
#[doc = "Field `MAJOR` writer - Chip Revision Major value"]
pub type MajorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:11 - Chip Part Number value"]
    #[inline(always)]
    pub fn partnumber(&self) -> PartnumberR {
        PartnumberR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - Chip Revision Minor value"]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Chip Revision Major value"]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Chip Part Number value"]
    #[inline(always)]
    pub fn partnumber(&mut self) -> PartnumberW<'_, ChiprevSpec> {
        PartnumberW::new(self, 0)
    }
    #[doc = "Bits 12:15 - Chip Revision Minor value"]
    #[inline(always)]
    pub fn minor(&mut self) -> MinorW<'_, ChiprevSpec> {
        MinorW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Chip Revision Major value"]
    #[inline(always)]
    pub fn major(&mut self) -> MajorW<'_, ChiprevSpec> {
        MajorW::new(self, 16)
    }
}
#[doc = "Read to get the chip revision programmed by feature configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`chiprev::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chiprev::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChiprevSpec;
impl crate::RegisterSpec for ChiprevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chiprev::R`](R) reader structure"]
impl crate::Readable for ChiprevSpec {}
#[doc = "`write(|w| ..)` method takes [`chiprev::W`](W) writer structure"]
impl crate::Writable for ChiprevSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHIPREV to value 0"]
impl crate::Resettable for ChiprevSpec {}
