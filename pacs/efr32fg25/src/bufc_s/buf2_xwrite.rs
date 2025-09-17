#[doc = "Register `BUF2_XWRITE` writer"]
pub type W = crate::W<Buf2XwriteSpec>;
#[doc = "Field `XORWRITEDATA` writer - Buffer XOR Write Data"]
pub type XorwritedataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Buffer XOR Write Data"]
    #[inline(always)]
    pub fn xorwritedata(&mut self) -> XorwritedataW<'_, Buf2XwriteSpec> {
        XorwritedataW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf2_xwrite::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buf2XwriteSpec;
impl crate::RegisterSpec for Buf2XwriteSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`buf2_xwrite::W`](W) writer structure"]
impl crate::Writable for Buf2XwriteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUF2_XWRITE to value 0"]
impl crate::Resettable for Buf2XwriteSpec {}
