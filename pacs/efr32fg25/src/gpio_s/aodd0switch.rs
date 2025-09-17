#[doc = "Register `AODD0SWITCH` reader"]
pub type R = crate::R<Aodd0switchSpec>;
#[doc = "Register `AODD0SWITCH` writer"]
pub type W = crate::W<Aodd0switchSpec>;
#[doc = "Field `AODD0SWITCH` reader - AODD0 switch register"]
pub type Aodd0switchR = crate::FieldReader;
#[doc = "Field `AODD0SWITCH` writer - AODD0 switch register"]
pub type Aodd0switchW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - AODD0 switch register"]
    #[inline(always)]
    pub fn aodd0switch(&self) -> Aodd0switchR {
        Aodd0switchR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - AODD0 switch register"]
    #[inline(always)]
    pub fn aodd0switch(&mut self) -> Aodd0switchW<'_, Aodd0switchSpec> {
        Aodd0switchW::new(self, 0)
    }
}
#[doc = "ABUS AODD0 Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aodd0switch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aodd0switch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aodd0switchSpec;
impl crate::RegisterSpec for Aodd0switchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aodd0switch::R`](R) reader structure"]
impl crate::Readable for Aodd0switchSpec {}
#[doc = "`write(|w| ..)` method takes [`aodd0switch::W`](W) writer structure"]
impl crate::Writable for Aodd0switchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AODD0SWITCH to value 0"]
impl crate::Resettable for Aodd0switchSpec {}
