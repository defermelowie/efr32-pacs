#[doc = "Register `CDEVEN0SWITCH` reader"]
pub type R = crate::R<Cdeven0switchSpec>;
#[doc = "Register `CDEVEN0SWITCH` writer"]
pub type W = crate::W<Cdeven0switchSpec>;
#[doc = "Field `CEVEN0SWITCH` reader - CEVEN0 switch register"]
pub type Ceven0switchR = crate::FieldReader;
#[doc = "Field `CEVEN0SWITCH` writer - CEVEN0 switch register"]
pub type Ceven0switchW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DEVEN0SWITCH` reader - DEVEN0 switch register"]
pub type Deven0switchR = crate::FieldReader;
#[doc = "Field `DEVEN0SWITCH` writer - DEVEN0 switch register"]
pub type Deven0switchW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:5 - CEVEN0 switch register"]
    #[inline(always)]
    pub fn ceven0switch(&self) -> Ceven0switchR {
        Ceven0switchR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - DEVEN0 switch register"]
    #[inline(always)]
    pub fn deven0switch(&self) -> Deven0switchR {
        Deven0switchR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - CEVEN0 switch register"]
    #[inline(always)]
    pub fn ceven0switch(&mut self) -> Ceven0switchW<'_, Cdeven0switchSpec> {
        Ceven0switchW::new(self, 0)
    }
    #[doc = "Bits 16:19 - DEVEN0 switch register"]
    #[inline(always)]
    pub fn deven0switch(&mut self) -> Deven0switchW<'_, Cdeven0switchSpec> {
        Deven0switchW::new(self, 16)
    }
}
#[doc = "ABUS CDEVEN0 Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdeven0switch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdeven0switch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cdeven0switchSpec;
impl crate::RegisterSpec for Cdeven0switchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdeven0switch::R`](R) reader structure"]
impl crate::Readable for Cdeven0switchSpec {}
#[doc = "`write(|w| ..)` method takes [`cdeven0switch::W`](W) writer structure"]
impl crate::Writable for Cdeven0switchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CDEVEN0SWITCH to value 0"]
impl crate::Resettable for Cdeven0switchSpec {}
