#[doc = "Register `BUF0_READDATA` reader"]
pub type R = crate::R<Buf0ReaddataSpec>;
#[doc = "Field `READDATA` reader - Buffer Read Data"]
pub type ReaddataR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Buffer Read Data"]
    #[inline(always)]
    pub fn readdata(&self) -> ReaddataR {
        ReaddataR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf0_readdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buf0ReaddataSpec;
impl crate::RegisterSpec for Buf0ReaddataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf0_readdata::R`](R) reader structure"]
impl crate::Readable for Buf0ReaddataSpec {}
#[doc = "`reset()` method sets BUF0_READDATA to value 0"]
impl crate::Resettable for Buf0ReaddataSpec {}
