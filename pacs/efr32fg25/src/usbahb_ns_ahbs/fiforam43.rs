#[doc = "Register `FIFORAM43` reader"]
pub type R = crate::R<Fiforam43Spec>;
#[doc = "Register `FIFORAM43` writer"]
pub type W = crate::W<Fiforam43Spec>;
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
    pub fn fiforam(&mut self) -> FiforamW<'_, Fiforam43Spec> {
        FiforamW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fiforam43::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiforam43::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fiforam43Spec;
impl crate::RegisterSpec for Fiforam43Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiforam43::R`](R) reader structure"]
impl crate::Readable for Fiforam43Spec {}
#[doc = "`write(|w| ..)` method takes [`fiforam43::W`](W) writer structure"]
impl crate::Writable for Fiforam43Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFORAM43 to value 0"]
impl crate::Resettable for Fiforam43Spec {}
