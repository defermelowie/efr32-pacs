#[doc = "Register `FIFO8D678` reader"]
pub type R = crate::R<Fifo8d678Spec>;
#[doc = "Register `FIFO8D678` writer"]
pub type W = crate::W<Fifo8d678Spec>;
#[doc = "Field `FIFO8D` reader - EP 8 Data"]
pub type Fifo8dR = crate::FieldReader<u32>;
#[doc = "Field `FIFO8D` writer - EP 8 Data"]
pub type Fifo8dW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - EP 8 Data"]
    #[inline(always)]
    pub fn fifo8d(&self) -> Fifo8dR {
        Fifo8dR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - EP 8 Data"]
    #[inline(always)]
    pub fn fifo8d(&mut self) -> Fifo8dW<'_, Fifo8d678Spec> {
        Fifo8dW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo8d678::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo8d678::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo8d678Spec;
impl crate::RegisterSpec for Fifo8d678Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo8d678::R`](R) reader structure"]
impl crate::Readable for Fifo8d678Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo8d678::W`](W) writer structure"]
impl crate::Writable for Fifo8d678Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO8D678 to value 0"]
impl crate::Resettable for Fifo8d678Spec {}
