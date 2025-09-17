#[doc = "Register `CHIPREVHW` reader"]
pub type R = crate::R<ChiprevhwSpec>;
#[doc = "Register `CHIPREVHW` writer"]
pub type W = crate::W<ChiprevhwSpec>;
#[doc = "Field `PARTNUMBER` reader - Hardwired Chip Part Number value"]
pub type PartnumberR = crate::FieldReader<u16>;
#[doc = "Field `PARTNUMBER` writer - Hardwired Chip Part Number value"]
pub type PartnumberW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `MINOR` reader - Hardwired Chip Revision Minor value"]
pub type MinorR = crate::FieldReader;
#[doc = "Field `MINOR` writer - Hardwired Chip Revision Minor value"]
pub type MinorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MAJOR` reader - Hardwired Chip Revision Major value"]
pub type MajorR = crate::FieldReader;
#[doc = "Field `MAJOR` writer - Hardwired Chip Revision Major value"]
pub type MajorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:11 - Hardwired Chip Part Number value"]
    #[inline(always)]
    pub fn partnumber(&self) -> PartnumberR {
        PartnumberR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - Hardwired Chip Revision Minor value"]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Hardwired Chip Revision Major value"]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Hardwired Chip Part Number value"]
    #[inline(always)]
    pub fn partnumber(&mut self) -> PartnumberW<'_, ChiprevhwSpec> {
        PartnumberW::new(self, 0)
    }
    #[doc = "Bits 12:15 - Hardwired Chip Revision Minor value"]
    #[inline(always)]
    pub fn minor(&mut self) -> MinorW<'_, ChiprevhwSpec> {
        MinorW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Hardwired Chip Revision Major value"]
    #[inline(always)]
    pub fn major(&mut self) -> MajorW<'_, ChiprevhwSpec> {
        MajorW::new(self, 16)
    }
}
#[doc = "Read to get the hard-wired chip revision.\n\nYou can [`read`](crate::Reg::read) this register and get [`chiprevhw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chiprevhw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChiprevhwSpec;
impl crate::RegisterSpec for ChiprevhwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chiprevhw::R`](R) reader structure"]
impl crate::Readable for ChiprevhwSpec {}
#[doc = "`write(|w| ..)` method takes [`chiprevhw::W`](W) writer structure"]
impl crate::Writable for ChiprevhwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHIPREVHW to value 0x0001_0010"]
impl crate::Resettable for ChiprevhwSpec {
    const RESET_VALUE: u32 = 0x0001_0010;
}
