#[doc = "Register `FIFO4D634` reader"]
pub type R = crate::R<Fifo4d634Spec>;
#[doc = "Register `FIFO4D634` writer"]
pub type W = crate::W<Fifo4d634Spec>;
#[doc = "Field `FIFO4D` reader - EP 4 Data"]
pub type Fifo4dR = crate::FieldReader<u32>;
#[doc = "Field `FIFO4D` writer - EP 4 Data"]
pub type Fifo4dW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - EP 4 Data"]
    #[inline(always)]
    pub fn fifo4d(&self) -> Fifo4dR {
        Fifo4dR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - EP 4 Data"]
    #[inline(always)]
    pub fn fifo4d(&mut self) -> Fifo4dW<'_, Fifo4d634Spec> {
        Fifo4dW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo4d634::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo4d634::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo4d634Spec;
impl crate::RegisterSpec for Fifo4d634Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo4d634::R`](R) reader structure"]
impl crate::Readable for Fifo4d634Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo4d634::W`](W) writer structure"]
impl crate::Writable for Fifo4d634Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO4D634 to value 0"]
impl crate::Resettable for Fifo4d634Spec {}
