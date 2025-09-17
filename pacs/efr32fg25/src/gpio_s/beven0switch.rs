#[doc = "Register `BEVEN0SWITCH` reader"]
pub type R = crate::R<Beven0switchSpec>;
#[doc = "Register `BEVEN0SWITCH` writer"]
pub type W = crate::W<Beven0switchSpec>;
#[doc = "Field `BEVEN0SWITCH` reader - BEVEN0 switch register"]
pub type Beven0switchR = crate::FieldReader;
#[doc = "Field `BEVEN0SWITCH` writer - BEVEN0 switch register"]
pub type Beven0switchW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - BEVEN0 switch register"]
    #[inline(always)]
    pub fn beven0switch(&self) -> Beven0switchR {
        Beven0switchR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - BEVEN0 switch register"]
    #[inline(always)]
    pub fn beven0switch(&mut self) -> Beven0switchW<'_, Beven0switchSpec> {
        Beven0switchW::new(self, 0)
    }
}
#[doc = "ABUS BEVEN0 Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`beven0switch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`beven0switch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Beven0switchSpec;
impl crate::RegisterSpec for Beven0switchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`beven0switch::R`](R) reader structure"]
impl crate::Readable for Beven0switchSpec {}
#[doc = "`write(|w| ..)` method takes [`beven0switch::W`](W) writer structure"]
impl crate::Writable for Beven0switchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BEVEN0SWITCH to value 0"]
impl crate::Resettable for Beven0switchSpec {}
