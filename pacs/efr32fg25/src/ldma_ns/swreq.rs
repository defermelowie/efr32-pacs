#[doc = "Register `SWREQ` writer"]
pub type W = crate::W<SwreqSpec>;
#[doc = "Field `SWREQ` writer - Software Transfer Requests"]
pub type SwreqW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Software Transfer Requests"]
    #[inline(always)]
    pub fn swreq(&mut self) -> SwreqW<'_, SwreqSpec> {
        SwreqW::new(self, 0)
    }
}
#[doc = "Channel Software Transfer Request (Writes will only take effect when EN=1)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreq::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwreqSpec;
impl crate::RegisterSpec for SwreqSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swreq::W`](W) writer structure"]
impl crate::Writable for SwreqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWREQ to value 0"]
impl crate::Resettable for SwreqSpec {}
