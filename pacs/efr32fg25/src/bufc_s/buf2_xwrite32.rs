#[doc = "Register `BUF2_XWRITE32` writer"]
pub type W = crate::W<Buf2Xwrite32Spec>;
#[doc = "Field `XORWRITEDATA32` writer - Buffer XOR Write Data"]
pub type Xorwritedata32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Buffer XOR Write Data"]
    #[inline(always)]
    pub fn xorwritedata32(&mut self) -> Xorwritedata32W<'_, Buf2Xwrite32Spec> {
        Xorwritedata32W::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf2_xwrite32::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buf2Xwrite32Spec;
impl crate::RegisterSpec for Buf2Xwrite32Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`buf2_xwrite32::W`](W) writer structure"]
impl crate::Writable for Buf2Xwrite32Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUF2_XWRITE32 to value 0"]
impl crate::Resettable for Buf2Xwrite32Spec {}
