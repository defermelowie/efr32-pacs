#[doc = "Register `BODD1SWITCH` reader"]
pub type R = crate::R<Bodd1switchSpec>;
#[doc = "Register `BODD1SWITCH` writer"]
pub type W = crate::W<Bodd1switchSpec>;
#[doc = "Field `BODD1SWITCH` reader - BODD1 Switch Reg"]
pub type Bodd1switchR = crate::FieldReader;
#[doc = "Field `BODD1SWITCH` writer - BODD1 Switch Reg"]
pub type Bodd1switchW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - BODD1 Switch Reg"]
    #[inline(always)]
    pub fn bodd1switch(&self) -> Bodd1switchR {
        Bodd1switchR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - BODD1 Switch Reg"]
    #[inline(always)]
    pub fn bodd1switch(&mut self) -> Bodd1switchW<'_, Bodd1switchSpec> {
        Bodd1switchW::new(self, 0)
    }
}
#[doc = "ABUS BODD1 Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bodd1switch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bodd1switch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bodd1switchSpec;
impl crate::RegisterSpec for Bodd1switchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bodd1switch::R`](R) reader structure"]
impl crate::Readable for Bodd1switchSpec {}
#[doc = "`write(|w| ..)` method takes [`bodd1switch::W`](W) writer structure"]
impl crate::Writable for Bodd1switchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BODD1SWITCH to value 0"]
impl crate::Resettable for Bodd1switchSpec {}
