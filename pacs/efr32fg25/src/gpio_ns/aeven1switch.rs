#[doc = "Register `AEVEN1SWITCH` reader"]
pub type R = crate::R<Aeven1switchSpec>;
#[doc = "Register `AEVEN1SWITCH` writer"]
pub type W = crate::W<Aeven1switchSpec>;
#[doc = "Field `AEVEN1SWITCH` reader - AEVEN1 switch register"]
pub type Aeven1switchR = crate::FieldReader;
#[doc = "Field `AEVEN1SWITCH` writer - AEVEN1 switch register"]
pub type Aeven1switchW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - AEVEN1 switch register"]
    #[inline(always)]
    pub fn aeven1switch(&self) -> Aeven1switchR {
        Aeven1switchR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - AEVEN1 switch register"]
    #[inline(always)]
    pub fn aeven1switch(&mut self) -> Aeven1switchW<'_, Aeven1switchSpec> {
        Aeven1switchW::new(self, 0)
    }
}
#[doc = "ABUS AEVEN1 Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aeven1switch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeven1switch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aeven1switchSpec;
impl crate::RegisterSpec for Aeven1switchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aeven1switch::R`](R) reader structure"]
impl crate::Readable for Aeven1switchSpec {}
#[doc = "`write(|w| ..)` method takes [`aeven1switch::W`](W) writer structure"]
impl crate::Writable for Aeven1switchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AEVEN1SWITCH to value 0"]
impl crate::Resettable for Aeven1switchSpec {}
