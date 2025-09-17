#[doc = "Register `FIFO7D673` reader"]
pub type R = crate::R<Fifo7d673Spec>;
#[doc = "Register `FIFO7D673` writer"]
pub type W = crate::W<Fifo7d673Spec>;
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
    pub fn fifo7d(&mut self) -> Fifo7dW<'_, Fifo7d673Spec> {
        Fifo7dW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo7d673::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo7d673::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo7d673Spec;
impl crate::RegisterSpec for Fifo7d673Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo7d673::R`](R) reader structure"]
impl crate::Readable for Fifo7d673Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo7d673::W`](W) writer structure"]
impl crate::Writable for Fifo7d673Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO7D673 to value 0"]
impl crate::Resettable for Fifo7d673Spec {}
