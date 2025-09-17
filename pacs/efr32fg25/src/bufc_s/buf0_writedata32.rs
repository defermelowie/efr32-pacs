#[doc = "Register `BUF0_WRITEDATA32` writer"]
pub type W = crate::W<Buf0Writedata32Spec>;
#[doc = "Field `WRITEDATA32` writer - Buffer Write Data"]
pub type Writedata32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Buffer Write Data"]
    #[inline(always)]
    pub fn writedata32(&mut self) -> Writedata32W<'_, Buf0Writedata32Spec> {
        Writedata32W::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf0_writedata32::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buf0Writedata32Spec;
impl crate::RegisterSpec for Buf0Writedata32Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`buf0_writedata32::W`](W) writer structure"]
impl crate::Writable for Buf0Writedata32Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUF0_WRITEDATA32 to value 0"]
impl crate::Resettable for Buf0Writedata32Spec {}
