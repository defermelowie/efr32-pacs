#[doc = "Register `FIFORAM597` reader"]
pub type R = crate::R<Fiforam597Spec>;
#[doc = "Register `FIFORAM597` writer"]
pub type W = crate::W<Fiforam597Spec>;
#[doc = "Field `FIFORAM` reader - FIFORAM direct access"]
pub type FiforamR = crate::FieldReader<u32>;
#[doc = "Field `FIFORAM` writer - FIFORAM direct access"]
pub type FiforamW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - FIFORAM direct access"]
    #[inline(always)]
    pub fn fiforam(&self) -> FiforamR {
        FiforamR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - FIFORAM direct access"]
    #[inline(always)]
    pub fn fiforam(&mut self) -> FiforamW<'_, Fiforam597Spec> {
        FiforamW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fiforam597::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiforam597::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fiforam597Spec;
impl crate::RegisterSpec for Fiforam597Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiforam597::R`](R) reader structure"]
impl crate::Readable for Fiforam597Spec {}
#[doc = "`write(|w| ..)` method takes [`fiforam597::W`](W) writer structure"]
impl crate::Writable for Fiforam597Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFORAM597 to value 0"]
impl crate::Resettable for Fiforam597Spec {}
