#[doc = "Register `FIFORAM222` reader"]
pub type R = crate::R<Fiforam222Spec>;
#[doc = "Register `FIFORAM222` writer"]
pub type W = crate::W<Fiforam222Spec>;
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
    pub fn fiforam(&mut self) -> FiforamW<'_, Fiforam222Spec> {
        FiforamW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fiforam222::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiforam222::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fiforam222Spec;
impl crate::RegisterSpec for Fiforam222Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiforam222::R`](R) reader structure"]
impl crate::Readable for Fiforam222Spec {}
#[doc = "`write(|w| ..)` method takes [`fiforam222::W`](W) writer structure"]
impl crate::Writable for Fiforam222Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFORAM222 to value 0"]
impl crate::Resettable for Fiforam222Spec {}
