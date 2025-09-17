#[doc = "Register `FIFORAM501` reader"]
pub type R = crate::R<Fiforam501Spec>;
#[doc = "Register `FIFORAM501` writer"]
pub type W = crate::W<Fiforam501Spec>;
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
    pub fn fiforam(&mut self) -> FiforamW<'_, Fiforam501Spec> {
        FiforamW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fiforam501::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiforam501::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fiforam501Spec;
impl crate::RegisterSpec for Fiforam501Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiforam501::R`](R) reader structure"]
impl crate::Readable for Fiforam501Spec {}
#[doc = "`write(|w| ..)` method takes [`fiforam501::W`](W) writer structure"]
impl crate::Writable for Fiforam501Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFORAM501 to value 0"]
impl crate::Resettable for Fiforam501Spec {}
