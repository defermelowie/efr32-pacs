#[doc = "Register `ROOTDATA0` reader"]
pub type R = crate::R<Rootdata0Spec>;
#[doc = "Register `ROOTDATA0` writer"]
pub type W = crate::W<Rootdata0Spec>;
#[doc = "Field `DATA` reader - Data"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Rootdata0Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Generic data space for user to pass to root, e.g., address of struct in mem\n\nYou can [`read`](crate::Reg::read) this register and get [`rootdata0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rootdata0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rootdata0Spec;
impl crate::RegisterSpec for Rootdata0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rootdata0::R`](R) reader structure"]
impl crate::Readable for Rootdata0Spec {}
#[doc = "`write(|w| ..)` method takes [`rootdata0::W`](W) writer structure"]
impl crate::Writable for Rootdata0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROOTDATA0 to value 0"]
impl crate::Resettable for Rootdata0Spec {}
