#[doc = "Register `FIFORAM227` reader"]
pub type R = crate::R<Fiforam227Spec>;
#[doc = "Register `FIFORAM227` writer"]
pub type W = crate::W<Fiforam227Spec>;
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
    pub fn fiforam(&mut self) -> FiforamW<'_, Fiforam227Spec> {
        FiforamW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fiforam227::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiforam227::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fiforam227Spec;
impl crate::RegisterSpec for Fiforam227Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiforam227::R`](R) reader structure"]
impl crate::Readable for Fiforam227Spec {}
#[doc = "`write(|w| ..)` method takes [`fiforam227::W`](W) writer structure"]
impl crate::Writable for Fiforam227Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFORAM227 to value 0"]
impl crate::Resettable for Fiforam227Spec {}
