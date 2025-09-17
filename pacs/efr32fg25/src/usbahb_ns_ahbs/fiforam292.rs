#[doc = "Register `FIFORAM292` reader"]
pub type R = crate::R<Fiforam292Spec>;
#[doc = "Register `FIFORAM292` writer"]
pub type W = crate::W<Fiforam292Spec>;
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
    pub fn fiforam(&mut self) -> FiforamW<'_, Fiforam292Spec> {
        FiforamW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fiforam292::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiforam292::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fiforam292Spec;
impl crate::RegisterSpec for Fiforam292Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiforam292::R`](R) reader structure"]
impl crate::Readable for Fiforam292Spec {}
#[doc = "`write(|w| ..)` method takes [`fiforam292::W`](W) writer structure"]
impl crate::Writable for Fiforam292Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFORAM292 to value 0"]
impl crate::Resettable for Fiforam292Spec {}
