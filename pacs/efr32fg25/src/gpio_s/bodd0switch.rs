#[doc = "Register `BODD0SWITCH` reader"]
pub type R = crate::R<Bodd0switchSpec>;
#[doc = "Register `BODD0SWITCH` writer"]
pub type W = crate::W<Bodd0switchSpec>;
#[doc = "Field `BODD0SWITCH` reader - BODD0 Switch Reg"]
pub type Bodd0switchR = crate::FieldReader;
#[doc = "Field `BODD0SWITCH` writer - BODD0 Switch Reg"]
pub type Bodd0switchW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - BODD0 Switch Reg"]
    #[inline(always)]
    pub fn bodd0switch(&self) -> Bodd0switchR {
        Bodd0switchR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - BODD0 Switch Reg"]
    #[inline(always)]
    pub fn bodd0switch(&mut self) -> Bodd0switchW<'_, Bodd0switchSpec> {
        Bodd0switchW::new(self, 0)
    }
}
#[doc = "ABUS BODD0 Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bodd0switch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bodd0switch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bodd0switchSpec;
impl crate::RegisterSpec for Bodd0switchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bodd0switch::R`](R) reader structure"]
impl crate::Readable for Bodd0switchSpec {}
#[doc = "`write(|w| ..)` method takes [`bodd0switch::W`](W) writer structure"]
impl crate::Writable for Bodd0switchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BODD0SWITCH to value 0"]
impl crate::Resettable for Bodd0switchSpec {}
