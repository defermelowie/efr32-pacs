#[doc = "Register `CHDIS` writer"]
pub type W = crate::W<ChdisSpec>;
#[doc = "Field `CHDIS` writer - DMA Channel disable"]
pub type ChdisW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - DMA Channel disable"]
    #[inline(always)]
    pub fn chdis(&mut self) -> ChdisW<'_, ChdisSpec> {
        ChdisW::new(self, 0)
    }
}
#[doc = "Channel Disable Register (Writes will only take effect when EN=1)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chdis::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChdisSpec;
impl crate::RegisterSpec for ChdisSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chdis::W`](W) writer structure"]
impl crate::Writable for ChdisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHDIS to value 0"]
impl crate::Resettable for ChdisSpec {}
