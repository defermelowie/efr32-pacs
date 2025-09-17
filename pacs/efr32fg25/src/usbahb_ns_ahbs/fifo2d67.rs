#[doc = "Register `FIFO2D67` reader"]
pub type R = crate::R<Fifo2d67Spec>;
#[doc = "Register `FIFO2D67` writer"]
pub type W = crate::W<Fifo2d67Spec>;
#[doc = "Field `FIFO2D` reader - EP2 data"]
pub type Fifo2dR = crate::FieldReader<u32>;
#[doc = "Field `FIFO2D` writer - EP2 data"]
pub type Fifo2dW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - EP2 data"]
    #[inline(always)]
    pub fn fifo2d(&self) -> Fifo2dR {
        Fifo2dR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - EP2 data"]
    #[inline(always)]
    pub fn fifo2d(&mut self) -> Fifo2dW<'_, Fifo2d67Spec> {
        Fifo2dW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo2d67::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo2d67::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo2d67Spec;
impl crate::RegisterSpec for Fifo2d67Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo2d67::R`](R) reader structure"]
impl crate::Readable for Fifo2d67Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo2d67::W`](W) writer structure"]
impl crate::Writable for Fifo2d67Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO2D67 to value 0"]
impl crate::Resettable for Fifo2d67Spec {}
