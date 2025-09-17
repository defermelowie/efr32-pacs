#[doc = "Register `RESCOUNT` reader"]
pub type R = crate::R<RescountSpec>;
#[doc = "Field `COUNT` reader - Result Fifo Count"]
pub type CountR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Result Fifo Count"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Result FIFO Count\n\nYou can [`read`](crate::Reg::read) this register and get [`rescount::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RescountSpec;
impl crate::RegisterSpec for RescountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rescount::R`](R) reader structure"]
impl crate::Readable for RescountSpec {}
#[doc = "`reset()` method sets RESCOUNT to value 0"]
impl crate::Resettable for RescountSpec {}
