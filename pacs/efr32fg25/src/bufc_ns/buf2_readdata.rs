#[doc = "Register `BUF2_READDATA` reader"]
pub type R = crate::R<Buf2ReaddataSpec>;
#[doc = "Field `READDATA` reader - Buffer Read Data"]
pub type ReaddataR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Buffer Read Data"]
    #[inline(always)]
    pub fn readdata(&self) -> ReaddataR {
        ReaddataR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf2_readdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buf2ReaddataSpec;
impl crate::RegisterSpec for Buf2ReaddataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf2_readdata::R`](R) reader structure"]
impl crate::Readable for Buf2ReaddataSpec {}
#[doc = "`reset()` method sets BUF2_READDATA to value 0"]
impl crate::Resettable for Buf2ReaddataSpec {}
