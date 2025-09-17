#[doc = "Register `FIFORAM485` reader"]
pub type R = crate::R<Fiforam485Spec>;
#[doc = "Register `FIFORAM485` writer"]
pub type W = crate::W<Fiforam485Spec>;
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
    pub fn fiforam(&mut self) -> FiforamW<'_, Fiforam485Spec> {
        FiforamW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fiforam485::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiforam485::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fiforam485Spec;
impl crate::RegisterSpec for Fiforam485Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiforam485::R`](R) reader structure"]
impl crate::Readable for Fiforam485Spec {}
#[doc = "`write(|w| ..)` method takes [`fiforam485::W`](W) writer structure"]
impl crate::Writable for Fiforam485Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFORAM485 to value 0"]
impl crate::Resettable for Fiforam485Spec {}
