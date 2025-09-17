#[doc = "Register `AODD1SWITCH` reader"]
pub type R = crate::R<Aodd1switchSpec>;
#[doc = "Register `AODD1SWITCH` writer"]
pub type W = crate::W<Aodd1switchSpec>;
#[doc = "Field `AODD1SWITCH` reader - AODD1 switch register"]
pub type Aodd1switchR = crate::FieldReader;
#[doc = "Field `AODD1SWITCH` writer - AODD1 switch register"]
pub type Aodd1switchW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - AODD1 switch register"]
    #[inline(always)]
    pub fn aodd1switch(&self) -> Aodd1switchR {
        Aodd1switchR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - AODD1 switch register"]
    #[inline(always)]
    pub fn aodd1switch(&mut self) -> Aodd1switchW<'_, Aodd1switchSpec> {
        Aodd1switchW::new(self, 0)
    }
}
#[doc = "ABUS AODD1 Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aodd1switch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aodd1switch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aodd1switchSpec;
impl crate::RegisterSpec for Aodd1switchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aodd1switch::R`](R) reader structure"]
impl crate::Readable for Aodd1switchSpec {}
#[doc = "`write(|w| ..)` method takes [`aodd1switch::W`](W) writer structure"]
impl crate::Writable for Aodd1switchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AODD1SWITCH to value 0"]
impl crate::Resettable for Aodd1switchSpec {}
