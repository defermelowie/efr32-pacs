#[doc = "Register `FIFO5D174` reader"]
pub type R = crate::R<Fifo5d174Spec>;
#[doc = "Register `FIFO5D174` writer"]
pub type W = crate::W<Fifo5d174Spec>;
#[doc = "Field `FIFO5D` reader - EP 5 Data"]
pub type Fifo5dR = crate::BitReader;
#[doc = "Field `FIFO5D` writer - EP 5 Data"]
pub type Fifo5dW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EP 5 Data"]
    #[inline(always)]
    pub fn fifo5d(&self) -> Fifo5dR {
        Fifo5dR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EP 5 Data"]
    #[inline(always)]
    pub fn fifo5d(&mut self) -> Fifo5dW<'_, Fifo5d174Spec> {
        Fifo5dW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo5d174::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo5d174::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo5d174Spec;
impl crate::RegisterSpec for Fifo5d174Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo5d174::R`](R) reader structure"]
impl crate::Readable for Fifo5d174Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo5d174::W`](W) writer structure"]
impl crate::Writable for Fifo5d174Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO5D174 to value 0"]
impl crate::Resettable for Fifo5d174Spec {}
