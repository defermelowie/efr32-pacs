#[doc = "Register `BUF3_READDATA32` reader"]
pub type R = crate::R<Buf3Readdata32Spec>;
#[doc = "Field `READDATA32` reader - Buffer Read Data"]
pub type Readdata32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Read Data"]
    #[inline(always)]
    pub fn readdata32(&self) -> Readdata32R {
        Readdata32R::new(self.bits)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf3_readdata32::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buf3Readdata32Spec;
impl crate::RegisterSpec for Buf3Readdata32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf3_readdata32::R`](R) reader structure"]
impl crate::Readable for Buf3Readdata32Spec {}
#[doc = "`reset()` method sets BUF3_READDATA32 to value 0"]
impl crate::Resettable for Buf3Readdata32Spec {}
