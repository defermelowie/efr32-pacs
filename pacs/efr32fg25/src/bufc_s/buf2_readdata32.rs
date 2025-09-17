#[doc = "Register `BUF2_READDATA32` reader"]
pub type R = crate::R<Buf2Readdata32Spec>;
#[doc = "Field `READDATA32` reader - Buffer Read Data"]
pub type Readdata32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Read Data"]
    #[inline(always)]
    pub fn readdata32(&self) -> Readdata32R {
        Readdata32R::new(self.bits)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf2_readdata32::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buf2Readdata32Spec;
impl crate::RegisterSpec for Buf2Readdata32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf2_readdata32::R`](R) reader structure"]
impl crate::Readable for Buf2Readdata32Spec {}
#[doc = "`reset()` method sets BUF2_READDATA32 to value 0"]
impl crate::Resettable for Buf2Readdata32Spec {}
