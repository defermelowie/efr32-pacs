#[doc = "Register `FIFO1D924` reader"]
pub type R = crate::R<Fifo1d924Spec>;
#[doc = "Register `FIFO1D924` writer"]
pub type W = crate::W<Fifo1d924Spec>;
#[doc = "Field `FIFO1D` reader - EP1 Data"]
pub type Fifo1dR = crate::FieldReader<u32>;
#[doc = "Field `FIFO1D` writer - EP1 Data"]
pub type Fifo1dW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - EP1 Data"]
    #[inline(always)]
    pub fn fifo1d(&self) -> Fifo1dR {
        Fifo1dR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - EP1 Data"]
    #[inline(always)]
    pub fn fifo1d(&mut self) -> Fifo1dW<'_, Fifo1d924Spec> {
        Fifo1dW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo1d924::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo1d924::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo1d924Spec;
impl crate::RegisterSpec for Fifo1d924Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo1d924::R`](R) reader structure"]
impl crate::Readable for Fifo1d924Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo1d924::W`](W) writer structure"]
impl crate::Writable for Fifo1d924Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO1D924 to value 0"]
impl crate::Resettable for Fifo1d924Spec {}
