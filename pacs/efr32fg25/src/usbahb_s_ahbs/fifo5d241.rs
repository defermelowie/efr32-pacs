#[doc = "Register `FIFO5D241` reader"]
pub type R = crate::R<Fifo5d241Spec>;
#[doc = "Register `FIFO5D241` writer"]
pub type W = crate::W<Fifo5d241Spec>;
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
    pub fn fifo5d(&mut self) -> Fifo5dW<'_, Fifo5d241Spec> {
        Fifo5dW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo5d241::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo5d241::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo5d241Spec;
impl crate::RegisterSpec for Fifo5d241Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo5d241::R`](R) reader structure"]
impl crate::Readable for Fifo5d241Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo5d241::W`](W) writer structure"]
impl crate::Writable for Fifo5d241Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO5D241 to value 0"]
impl crate::Resettable for Fifo5d241Spec {}
