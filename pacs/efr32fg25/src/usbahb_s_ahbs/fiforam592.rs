#[doc = "Register `FIFORAM592` reader"]
pub type R = crate::R<Fiforam592Spec>;
#[doc = "Register `FIFORAM592` writer"]
pub type W = crate::W<Fiforam592Spec>;
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
    pub fn fiforam(&mut self) -> FiforamW<'_, Fiforam592Spec> {
        FiforamW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fiforam592::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiforam592::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fiforam592Spec;
impl crate::RegisterSpec for Fiforam592Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiforam592::R`](R) reader structure"]
impl crate::Readable for Fiforam592Spec {}
#[doc = "`write(|w| ..)` method takes [`fiforam592::W`](W) writer structure"]
impl crate::Writable for Fiforam592Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFORAM592 to value 0"]
impl crate::Resettable for Fiforam592Spec {}
