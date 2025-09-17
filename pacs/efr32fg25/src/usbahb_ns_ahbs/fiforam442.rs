#[doc = "Register `FIFORAM442` reader"]
pub type R = crate::R<Fiforam442Spec>;
#[doc = "Register `FIFORAM442` writer"]
pub type W = crate::W<Fiforam442Spec>;
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
    pub fn fiforam(&mut self) -> FiforamW<'_, Fiforam442Spec> {
        FiforamW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fiforam442::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiforam442::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fiforam442Spec;
impl crate::RegisterSpec for Fiforam442Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiforam442::R`](R) reader structure"]
impl crate::Readable for Fiforam442Spec {}
#[doc = "`write(|w| ..)` method takes [`fiforam442::W`](W) writer structure"]
impl crate::Writable for Fiforam442Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFORAM442 to value 0"]
impl crate::Resettable for Fiforam442Spec {}
