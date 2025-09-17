#[doc = "Register `FIFORAM483` reader"]
pub type R = crate::R<Fiforam483Spec>;
#[doc = "Register `FIFORAM483` writer"]
pub type W = crate::W<Fiforam483Spec>;
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
    pub fn fiforam(&mut self) -> FiforamW<'_, Fiforam483Spec> {
        FiforamW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fiforam483::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiforam483::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fiforam483Spec;
impl crate::RegisterSpec for Fiforam483Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiforam483::R`](R) reader structure"]
impl crate::Readable for Fiforam483Spec {}
#[doc = "`write(|w| ..)` method takes [`fiforam483::W`](W) writer structure"]
impl crate::Writable for Fiforam483Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFORAM483 to value 0"]
impl crate::Resettable for Fiforam483Spec {}
