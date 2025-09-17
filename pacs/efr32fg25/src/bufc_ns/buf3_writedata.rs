#[doc = "Register `BUF3_WRITEDATA` writer"]
pub type W = crate::W<Buf3WritedataSpec>;
#[doc = "Field `WRITEDATA` writer - Buffer Write Data"]
pub type WritedataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Buffer Write Data"]
    #[inline(always)]
    pub fn writedata(&mut self) -> WritedataW<'_, Buf3WritedataSpec> {
        WritedataW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf3_writedata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buf3WritedataSpec;
impl crate::RegisterSpec for Buf3WritedataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`buf3_writedata::W`](W) writer structure"]
impl crate::Writable for Buf3WritedataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUF3_WRITEDATA to value 0"]
impl crate::Resettable for Buf3WritedataSpec {}
