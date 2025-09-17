#[doc = "Register `FIFORAM767` reader"]
pub type R = crate::R<Fiforam767Spec>;
#[doc = "Register `FIFORAM767` writer"]
pub type W = crate::W<Fiforam767Spec>;
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
    pub fn fiforam(&mut self) -> FiforamW<'_, Fiforam767Spec> {
        FiforamW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fiforam767::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiforam767::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fiforam767Spec;
impl crate::RegisterSpec for Fiforam767Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiforam767::R`](R) reader structure"]
impl crate::Readable for Fiforam767Spec {}
#[doc = "`write(|w| ..)` method takes [`fiforam767::W`](W) writer structure"]
impl crate::Writable for Fiforam767Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFORAM767 to value 0"]
impl crate::Resettable for Fiforam767Spec {}
