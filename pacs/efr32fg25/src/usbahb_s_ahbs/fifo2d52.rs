#[doc = "Register `FIFO2D52` reader"]
pub type R = crate::R<Fifo2d52Spec>;
#[doc = "Register `FIFO2D52` writer"]
pub type W = crate::W<Fifo2d52Spec>;
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
    pub fn fifo2d(&mut self) -> Fifo2dW<'_, Fifo2d52Spec> {
        Fifo2dW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo2d52::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo2d52::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo2d52Spec;
impl crate::RegisterSpec for Fifo2d52Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo2d52::R`](R) reader structure"]
impl crate::Readable for Fifo2d52Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo2d52::W`](W) writer structure"]
impl crate::Writable for Fifo2d52Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO2D52 to value 0"]
impl crate::Resettable for Fifo2d52Spec {}
