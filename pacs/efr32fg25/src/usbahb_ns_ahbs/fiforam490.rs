#[doc = "Register `FIFORAM490` reader"]
pub type R = crate::R<Fiforam490Spec>;
#[doc = "Register `FIFORAM490` writer"]
pub type W = crate::W<Fiforam490Spec>;
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
    pub fn fiforam(&mut self) -> FiforamW<'_, Fiforam490Spec> {
        FiforamW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fiforam490::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiforam490::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fiforam490Spec;
impl crate::RegisterSpec for Fiforam490Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiforam490::R`](R) reader structure"]
impl crate::Readable for Fiforam490Spec {}
#[doc = "`write(|w| ..)` method takes [`fiforam490::W`](W) writer structure"]
impl crate::Writable for Fiforam490Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFORAM490 to value 0"]
impl crate::Resettable for Fiforam490Spec {}
