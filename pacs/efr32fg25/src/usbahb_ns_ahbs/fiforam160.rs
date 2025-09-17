#[doc = "Register `FIFORAM160` reader"]
pub type R = crate::R<Fiforam160Spec>;
#[doc = "Register `FIFORAM160` writer"]
pub type W = crate::W<Fiforam160Spec>;
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
    pub fn fiforam(&mut self) -> FiforamW<'_, Fiforam160Spec> {
        FiforamW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fiforam160::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiforam160::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fiforam160Spec;
impl crate::RegisterSpec for Fiforam160Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiforam160::R`](R) reader structure"]
impl crate::Readable for Fiforam160Spec {}
#[doc = "`write(|w| ..)` method takes [`fiforam160::W`](W) writer structure"]
impl crate::Writable for Fiforam160Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFORAM160 to value 0"]
impl crate::Resettable for Fiforam160Spec {}
