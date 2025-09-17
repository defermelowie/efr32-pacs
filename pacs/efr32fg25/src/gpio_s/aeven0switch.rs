#[doc = "Register `AEVEN0SWITCH` reader"]
pub type R = crate::R<Aeven0switchSpec>;
#[doc = "Register `AEVEN0SWITCH` writer"]
pub type W = crate::W<Aeven0switchSpec>;
#[doc = "Field `AEVEN0SWITCH` reader - AEVEN0 switch register"]
pub type Aeven0switchR = crate::FieldReader;
#[doc = "Field `AEVEN0SWITCH` writer - AEVEN0 switch register"]
pub type Aeven0switchW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - AEVEN0 switch register"]
    #[inline(always)]
    pub fn aeven0switch(&self) -> Aeven0switchR {
        Aeven0switchR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - AEVEN0 switch register"]
    #[inline(always)]
    pub fn aeven0switch(&mut self) -> Aeven0switchW<'_, Aeven0switchSpec> {
        Aeven0switchW::new(self, 0)
    }
}
#[doc = "ABUS AEVEN0 Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aeven0switch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeven0switch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aeven0switchSpec;
impl crate::RegisterSpec for Aeven0switchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aeven0switch::R`](R) reader structure"]
impl crate::Readable for Aeven0switchSpec {}
#[doc = "`write(|w| ..)` method takes [`aeven0switch::W`](W) writer structure"]
impl crate::Writable for Aeven0switchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AEVEN0SWITCH to value 0"]
impl crate::Resettable for Aeven0switchSpec {}
