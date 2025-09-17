#[doc = "Register `REQCLEAR` writer"]
pub type W = crate::W<ReqclearSpec>;
#[doc = "Field `REQCLEAR` writer - DMA Request Clear"]
pub type ReqclearW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - DMA Request Clear"]
    #[inline(always)]
    pub fn reqclear(&mut self) -> ReqclearW<'_, ReqclearSpec> {
        ReqclearW::new(self, 0)
    }
}
#[doc = "Channel Request Clear Register (Writes will only take effect when EN=1)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reqclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReqclearSpec;
impl crate::RegisterSpec for ReqclearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`reqclear::W`](W) writer structure"]
impl crate::Writable for ReqclearSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REQCLEAR to value 0"]
impl crate::Resettable for ReqclearSpec {}
