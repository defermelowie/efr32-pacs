#[doc = "Register `DEVOUTEP0_DMAADDR` reader"]
pub type R = crate::R<Devoutep0DmaaddrSpec>;
#[doc = "Register `DEVOUTEP0_DMAADDR` writer"]
pub type W = crate::W<Devoutep0DmaaddrSpec>;
#[doc = "Field `DMAADDR` reader - DMA Address"]
pub type DmaaddrR = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR` writer - DMA Address"]
pub type DmaaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DmaaddrR {
        DmaaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DmaaddrW<'_, Devoutep0DmaaddrSpec> {
        DmaaddrW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`devoutep0_dmaaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devoutep0_dmaaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devoutep0DmaaddrSpec;
impl crate::RegisterSpec for Devoutep0DmaaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devoutep0_dmaaddr::R`](R) reader structure"]
impl crate::Readable for Devoutep0DmaaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`devoutep0_dmaaddr::W`](W) writer structure"]
impl crate::Writable for Devoutep0DmaaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVOUTEP0_DMAADDR to value 0"]
impl crate::Resettable for Devoutep0DmaaddrSpec {}
