#[doc = "Register `FIFO7D108` reader"]
pub type R = crate::R<Fifo7d108Spec>;
#[doc = "Register `FIFO7D108` writer"]
pub type W = crate::W<Fifo7d108Spec>;
#[doc = "Field `FIFO7D` reader - EP 7 Data"]
pub type Fifo7dR = crate::FieldReader<u32>;
#[doc = "Field `FIFO7D` writer - EP 7 Data"]
pub type Fifo7dW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - EP 7 Data"]
    #[inline(always)]
    pub fn fifo7d(&self) -> Fifo7dR {
        Fifo7dR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - EP 7 Data"]
    #[inline(always)]
    pub fn fifo7d(&mut self) -> Fifo7dW<'_, Fifo7d108Spec> {
        Fifo7dW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo7d108::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo7d108::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo7d108Spec;
impl crate::RegisterSpec for Fifo7d108Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo7d108::R`](R) reader structure"]
impl crate::Readable for Fifo7d108Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo7d108::W`](W) writer structure"]
impl crate::Writable for Fifo7d108Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO7D108 to value 0"]
impl crate::Resettable for Fifo7d108Spec {}
