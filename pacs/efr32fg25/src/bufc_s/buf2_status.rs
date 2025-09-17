#[doc = "Register `BUF2_STATUS` reader"]
pub type R = crate::R<Buf2StatusSpec>;
#[doc = "Field `BYTES` reader - Number of Bytes in the Buffer"]
pub type BytesR = crate::FieldReader<u16>;
#[doc = "Field `THRESHOLDFLAG` reader - Buffer Threshold Flag"]
pub type ThresholdflagR = crate::BitReader;
impl R {
    #[doc = "Bits 0:12 - Number of Bytes in the Buffer"]
    #[inline(always)]
    pub fn bytes(&self) -> BytesR {
        BytesR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 20 - Buffer Threshold Flag"]
    #[inline(always)]
    pub fn thresholdflag(&self) -> ThresholdflagR {
        ThresholdflagR::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf2_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buf2StatusSpec;
impl crate::RegisterSpec for Buf2StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf2_status::R`](R) reader structure"]
impl crate::Readable for Buf2StatusSpec {}
#[doc = "`reset()` method sets BUF2_STATUS to value 0"]
impl crate::Resettable for Buf2StatusSpec {}
