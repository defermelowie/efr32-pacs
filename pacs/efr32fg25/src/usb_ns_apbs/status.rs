#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `VBUSVALID` reader - VBus valid"]
pub type VbusvalidR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - VBus valid"]
    #[inline(always)]
    pub fn vbusvalid(&self) -> VbusvalidR {
        VbusvalidR::new((self.bits & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
