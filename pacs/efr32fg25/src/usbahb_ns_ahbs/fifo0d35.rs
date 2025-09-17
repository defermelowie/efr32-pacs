#[doc = "Register `FIFO0D35` reader"]
pub type R = crate::R<Fifo0d35Spec>;
#[doc = "Register `FIFO0D35` writer"]
pub type W = crate::W<Fifo0d35Spec>;
#[doc = "Field `FIFO0D` reader - EP0 data"]
pub type Fifo0dR = crate::FieldReader<u32>;
#[doc = "Field `FIFO0D` writer - EP0 data"]
pub type Fifo0dW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - EP0 data"]
    #[inline(always)]
    pub fn fifo0d(&self) -> Fifo0dR {
        Fifo0dR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - EP0 data"]
    #[inline(always)]
    pub fn fifo0d(&mut self) -> Fifo0dW<'_, Fifo0d35Spec> {
        Fifo0dW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo0d35::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo0d35::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo0d35Spec;
impl crate::RegisterSpec for Fifo0d35Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo0d35::R`](R) reader structure"]
impl crate::Readable for Fifo0d35Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo0d35::W`](W) writer structure"]
impl crate::Writable for Fifo0d35Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO0D35 to value 0"]
impl crate::Resettable for Fifo0d35Spec {}
