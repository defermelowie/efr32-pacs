#[doc = "Register `CDODD1SWITCH` reader"]
pub type R = crate::R<Cdodd1switchSpec>;
#[doc = "Register `CDODD1SWITCH` writer"]
pub type W = crate::W<Cdodd1switchSpec>;
#[doc = "Field `CODD1SWITCH` reader - CODD1 switch register"]
pub type Codd1switchR = crate::FieldReader;
#[doc = "Field `CODD1SWITCH` writer - CODD1 switch register"]
pub type Codd1switchW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DODD1SWITCH` reader - DODD1 switch register"]
pub type Dodd1switchR = crate::FieldReader;
#[doc = "Field `DODD1SWITCH` writer - DODD1 switch register"]
pub type Dodd1switchW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - CODD1 switch register"]
    #[inline(always)]
    pub fn codd1switch(&self) -> Codd1switchR {
        Codd1switchR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - DODD1 switch register"]
    #[inline(always)]
    pub fn dodd1switch(&self) -> Dodd1switchR {
        Dodd1switchR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - CODD1 switch register"]
    #[inline(always)]
    pub fn codd1switch(&mut self) -> Codd1switchW<'_, Cdodd1switchSpec> {
        Codd1switchW::new(self, 0)
    }
    #[doc = "Bits 16:19 - DODD1 switch register"]
    #[inline(always)]
    pub fn dodd1switch(&mut self) -> Dodd1switchW<'_, Cdodd1switchSpec> {
        Dodd1switchW::new(self, 16)
    }
}
#[doc = "ABUS CDODD1 Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdodd1switch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdodd1switch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cdodd1switchSpec;
impl crate::RegisterSpec for Cdodd1switchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdodd1switch::R`](R) reader structure"]
impl crate::Readable for Cdodd1switchSpec {}
#[doc = "`write(|w| ..)` method takes [`cdodd1switch::W`](W) writer structure"]
impl crate::Writable for Cdodd1switchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CDODD1SWITCH to value 0"]
impl crate::Resettable for Cdodd1switchSpec {}
