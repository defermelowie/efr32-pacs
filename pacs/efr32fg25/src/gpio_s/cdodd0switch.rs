#[doc = "Register `CDODD0SWITCH` reader"]
pub type R = crate::R<Cdodd0switchSpec>;
#[doc = "Register `CDODD0SWITCH` writer"]
pub type W = crate::W<Cdodd0switchSpec>;
#[doc = "Field `CODD0SWITCH` reader - CODD0 switch register"]
pub type Codd0switchR = crate::FieldReader;
#[doc = "Field `CODD0SWITCH` writer - CODD0 switch register"]
pub type Codd0switchW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DODD0SWITCH` reader - DODD0 switch register"]
pub type Dodd0switchR = crate::FieldReader;
#[doc = "Field `DODD0SWITCH` writer - DODD0 switch register"]
pub type Dodd0switchW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - CODD0 switch register"]
    #[inline(always)]
    pub fn codd0switch(&self) -> Codd0switchR {
        Codd0switchR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - DODD0 switch register"]
    #[inline(always)]
    pub fn dodd0switch(&self) -> Dodd0switchR {
        Dodd0switchR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - CODD0 switch register"]
    #[inline(always)]
    pub fn codd0switch(&mut self) -> Codd0switchW<'_, Cdodd0switchSpec> {
        Codd0switchW::new(self, 0)
    }
    #[doc = "Bits 16:19 - DODD0 switch register"]
    #[inline(always)]
    pub fn dodd0switch(&mut self) -> Dodd0switchW<'_, Cdodd0switchSpec> {
        Dodd0switchW::new(self, 16)
    }
}
#[doc = "ABUS CDODD0 Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdodd0switch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdodd0switch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cdodd0switchSpec;
impl crate::RegisterSpec for Cdodd0switchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdodd0switch::R`](R) reader structure"]
impl crate::Readable for Cdodd0switchSpec {}
#[doc = "`write(|w| ..)` method takes [`cdodd0switch::W`](W) writer structure"]
impl crate::Writable for Cdodd0switchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CDODD0SWITCH to value 0"]
impl crate::Resettable for Cdodd0switchSpec {}
