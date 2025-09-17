#[doc = "Register `BEVEN1SWITCH` reader"]
pub type R = crate::R<Beven1switchSpec>;
#[doc = "Register `BEVEN1SWITCH` writer"]
pub type W = crate::W<Beven1switchSpec>;
#[doc = "Field `BEVEN1SWITCH` reader - BEVEN1 switch register"]
pub type Beven1switchR = crate::FieldReader;
#[doc = "Field `BEVEN1SWITCH` writer - BEVEN1 switch register"]
pub type Beven1switchW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - BEVEN1 switch register"]
    #[inline(always)]
    pub fn beven1switch(&self) -> Beven1switchR {
        Beven1switchR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - BEVEN1 switch register"]
    #[inline(always)]
    pub fn beven1switch(&mut self) -> Beven1switchW<'_, Beven1switchSpec> {
        Beven1switchW::new(self, 0)
    }
}
#[doc = "ABUS BEVEN1 Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`beven1switch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`beven1switch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Beven1switchSpec;
impl crate::RegisterSpec for Beven1switchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`beven1switch::R`](R) reader structure"]
impl crate::Readable for Beven1switchSpec {}
#[doc = "`write(|w| ..)` method takes [`beven1switch::W`](W) writer structure"]
impl crate::Writable for Beven1switchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BEVEN1SWITCH to value 0"]
impl crate::Resettable for Beven1switchSpec {}
