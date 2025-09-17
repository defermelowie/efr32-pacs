#[doc = "Register `FIFORAM569` reader"]
pub type R = crate::R<Fiforam569Spec>;
#[doc = "Register `FIFORAM569` writer"]
pub type W = crate::W<Fiforam569Spec>;
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
    pub fn fiforam(&mut self) -> FiforamW<'_, Fiforam569Spec> {
        FiforamW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fiforam569::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiforam569::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fiforam569Spec;
impl crate::RegisterSpec for Fiforam569Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiforam569::R`](R) reader structure"]
impl crate::Readable for Fiforam569Spec {}
#[doc = "`write(|w| ..)` method takes [`fiforam569::W`](W) writer structure"]
impl crate::Writable for Fiforam569Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFORAM569 to value 0"]
impl crate::Resettable for Fiforam569Spec {}
