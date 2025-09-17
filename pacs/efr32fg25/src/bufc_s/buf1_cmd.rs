#[doc = "Register `BUF1_CMD` writer"]
pub type W = crate::W<Buf1CmdSpec>;
#[doc = "Field `CLEAR` writer - Buffer Clear"]
pub type ClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREFETCH` writer - Prefetch"]
pub type PrefetchW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Buffer Clear"]
    #[inline(always)]
    pub fn clear(&mut self) -> ClearW<'_, Buf1CmdSpec> {
        ClearW::new(self, 0)
    }
    #[doc = "Bit 1 - Prefetch"]
    #[inline(always)]
    pub fn prefetch(&mut self) -> PrefetchW<'_, Buf1CmdSpec> {
        PrefetchW::new(self, 1)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf1_cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buf1CmdSpec;
impl crate::RegisterSpec for Buf1CmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`buf1_cmd::W`](W) writer structure"]
impl crate::Writable for Buf1CmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUF1_CMD to value 0"]
impl crate::Resettable for Buf1CmdSpec {}
