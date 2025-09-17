#[doc = "Register `BUF3_WRITEDATA32` writer"]
pub type W = crate::W<Buf3Writedata32Spec>;
#[doc = "Field `WRITEDATA32` writer - Buffer Write Data"]
pub type Writedata32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Buffer Write Data"]
    #[inline(always)]
    pub fn writedata32(&mut self) -> Writedata32W<'_, Buf3Writedata32Spec> {
        Writedata32W::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf3_writedata32::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buf3Writedata32Spec;
impl crate::RegisterSpec for Buf3Writedata32Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`buf3_writedata32::W`](W) writer structure"]
impl crate::Writable for Buf3Writedata32Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUF3_WRITEDATA32 to value 0"]
impl crate::Resettable for Buf3Writedata32Spec {}
