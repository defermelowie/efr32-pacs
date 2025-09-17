#[doc = "Register `DEVINEP7_DMAADDR` reader"]
pub type R = crate::R<Devinep7DmaaddrSpec>;
#[doc = "Register `DEVINEP7_DMAADDR` writer"]
pub type W = crate::W<Devinep7DmaaddrSpec>;
#[doc = "Field `DMAADDR` reader - DMA ADDR"]
pub type DmaaddrR = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR` writer - DMA ADDR"]
pub type DmaaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA ADDR"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DmaaddrR {
        DmaaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA ADDR"]
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DmaaddrW<'_, Devinep7DmaaddrSpec> {
        DmaaddrW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`devinep7_dmaaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devinep7_dmaaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devinep7DmaaddrSpec;
impl crate::RegisterSpec for Devinep7DmaaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devinep7_dmaaddr::R`](R) reader structure"]
impl crate::Readable for Devinep7DmaaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`devinep7_dmaaddr::W`](W) writer structure"]
impl crate::Writable for Devinep7DmaaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVINEP7_DMAADDR to value 0"]
impl crate::Resettable for Devinep7DmaaddrSpec {}
