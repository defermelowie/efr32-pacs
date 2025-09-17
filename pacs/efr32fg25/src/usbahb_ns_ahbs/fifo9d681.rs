#[doc = "Register `FIFO9D681` reader"]
pub type R = crate::R<Fifo9d681Spec>;
#[doc = "Register `FIFO9D681` writer"]
pub type W = crate::W<Fifo9d681Spec>;
#[doc = "Field `FIFO9D` reader - EP 9 Data"]
pub type Fifo9dR = crate::FieldReader<u32>;
#[doc = "Field `FIFO9D` writer - EP 9 Data"]
pub type Fifo9dW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - EP 9 Data"]
    #[inline(always)]
    pub fn fifo9d(&self) -> Fifo9dR {
        Fifo9dR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - EP 9 Data"]
    #[inline(always)]
    pub fn fifo9d(&mut self) -> Fifo9dW<'_, Fifo9d681Spec> {
        Fifo9dW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo9d681::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo9d681::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo9d681Spec;
impl crate::RegisterSpec for Fifo9d681Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo9d681::R`](R) reader structure"]
impl crate::Readable for Fifo9d681Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo9d681::W`](W) writer structure"]
impl crate::Writable for Fifo9d681Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO9D681 to value 0"]
impl crate::Resettable for Fifo9d681Spec {}
