#[doc = "Register `FIFORAM273` reader"]
pub type R = crate::R<Fiforam273Spec>;
#[doc = "Register `FIFORAM273` writer"]
pub type W = crate::W<Fiforam273Spec>;
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
    pub fn fiforam(&mut self) -> FiforamW<'_, Fiforam273Spec> {
        FiforamW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fiforam273::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiforam273::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fiforam273Spec;
impl crate::RegisterSpec for Fiforam273Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiforam273::R`](R) reader structure"]
impl crate::Readable for Fiforam273Spec {}
#[doc = "`write(|w| ..)` method takes [`fiforam273::W`](W) writer structure"]
impl crate::Writable for Fiforam273Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFORAM273 to value 0"]
impl crate::Resettable for Fiforam273Spec {}
