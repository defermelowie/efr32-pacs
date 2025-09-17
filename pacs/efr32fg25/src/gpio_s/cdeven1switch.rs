#[doc = "Register `CDEVEN1SWITCH` reader"]
pub type R = crate::R<Cdeven1switchSpec>;
#[doc = "Register `CDEVEN1SWITCH` writer"]
pub type W = crate::W<Cdeven1switchSpec>;
#[doc = "Field `CEVEN1SWITCH` reader - CEVEN1 switch register"]
pub type Ceven1switchR = crate::FieldReader;
#[doc = "Field `CEVEN1SWITCH` writer - CEVEN1 switch register"]
pub type Ceven1switchW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DEVEN1SWITCH` reader - DEVEN1 switch register"]
pub type Deven1switchR = crate::FieldReader;
#[doc = "Field `DEVEN1SWITCH` writer - DEVEN1 switch register"]
pub type Deven1switchW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:5 - CEVEN1 switch register"]
    #[inline(always)]
    pub fn ceven1switch(&self) -> Ceven1switchR {
        Ceven1switchR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - DEVEN1 switch register"]
    #[inline(always)]
    pub fn deven1switch(&self) -> Deven1switchR {
        Deven1switchR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - CEVEN1 switch register"]
    #[inline(always)]
    pub fn ceven1switch(&mut self) -> Ceven1switchW<'_, Cdeven1switchSpec> {
        Ceven1switchW::new(self, 0)
    }
    #[doc = "Bits 16:19 - DEVEN1 switch register"]
    #[inline(always)]
    pub fn deven1switch(&mut self) -> Deven1switchW<'_, Cdeven1switchSpec> {
        Deven1switchW::new(self, 16)
    }
}
#[doc = "ABUS CDEVEN1 Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdeven1switch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdeven1switch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cdeven1switchSpec;
impl crate::RegisterSpec for Cdeven1switchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdeven1switch::R`](R) reader structure"]
impl crate::Readable for Cdeven1switchSpec {}
#[doc = "`write(|w| ..)` method takes [`cdeven1switch::W`](W) writer structure"]
impl crate::Writable for Cdeven1switchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CDEVEN1SWITCH to value 0"]
impl crate::Resettable for Cdeven1switchSpec {}
