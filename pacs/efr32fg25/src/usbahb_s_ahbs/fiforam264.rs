#[doc = "Register `FIFORAM264` reader"]
pub type R = crate::R<Fiforam264Spec>;
#[doc = "Register `FIFORAM264` writer"]
pub type W = crate::W<Fiforam264Spec>;
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
    pub fn fiforam(&mut self) -> FiforamW<'_, Fiforam264Spec> {
        FiforamW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fiforam264::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiforam264::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fiforam264Spec;
impl crate::RegisterSpec for Fiforam264Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiforam264::R`](R) reader structure"]
impl crate::Readable for Fiforam264Spec {}
#[doc = "`write(|w| ..)` method takes [`fiforam264::W`](W) writer structure"]
impl crate::Writable for Fiforam264Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFORAM264 to value 0"]
impl crate::Resettable for Fiforam264Spec {}
