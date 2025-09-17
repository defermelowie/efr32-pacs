#[doc = "Register `DEVINEP2_DMAADDR` reader"]
pub type R = crate::R<Devinep2DmaaddrSpec>;
#[doc = "Register `DEVINEP2_DMAADDR` writer"]
pub type W = crate::W<Devinep2DmaaddrSpec>;
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
    pub fn dmaaddr(&mut self) -> DmaaddrW<'_, Devinep2DmaaddrSpec> {
        DmaaddrW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`devinep2_dmaaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devinep2_dmaaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devinep2DmaaddrSpec;
impl crate::RegisterSpec for Devinep2DmaaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devinep2_dmaaddr::R`](R) reader structure"]
impl crate::Readable for Devinep2DmaaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`devinep2_dmaaddr::W`](W) writer structure"]
impl crate::Writable for Devinep2DmaaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVINEP2_DMAADDR to value 0"]
impl crate::Resettable for Devinep2DmaaddrSpec {}
