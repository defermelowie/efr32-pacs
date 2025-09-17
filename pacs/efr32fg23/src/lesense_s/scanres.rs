#[doc = "Register `SCANRES` reader"]
pub type R = crate::R<ScanresSpec>;
#[doc = "Field `SCANRES` reader - Scan results"]
pub type ScanresR = crate::FieldReader<u16>;
#[doc = "Field `STEPDIR` reader - Direction of previous step detection"]
pub type StepdirR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Scan results"]
    #[inline(always)]
    pub fn scanres(&self) -> ScanresR {
        ScanresR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Direction of previous step detection"]
    #[inline(always)]
    pub fn stepdir(&self) -> StepdirR {
        StepdirR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Scan result register\n\nYou can [`read`](crate::Reg::read) this register and get [`scanres::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScanresSpec;
impl crate::RegisterSpec for ScanresSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scanres::R`](R) reader structure"]
impl crate::Readable for ScanresSpec {}
#[doc = "`reset()` method sets SCANRES to value 0"]
impl crate::Resettable for ScanresSpec {}
