#[doc = "Register `DECSTATE` reader"]
pub type R = crate::R<DecstateSpec>;
#[doc = "Field `DECSTATE` reader - Shows the current decoder state"]
pub type DecstateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Shows the current decoder state"]
    #[inline(always)]
    pub fn decstate(&self) -> DecstateR {
        DecstateR::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Current decoder state\n\nYou can [`read`](crate::Reg::read) this register and get [`decstate::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DecstateSpec;
impl crate::RegisterSpec for DecstateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`decstate::R`](R) reader structure"]
impl crate::Readable for DecstateSpec {}
#[doc = "`reset()` method sets DECSTATE to value 0"]
impl crate::Resettable for DecstateSpec {}
