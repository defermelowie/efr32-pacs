#[doc = "Register `GHWCFG1` reader"]
pub type R = crate::R<Ghwcfg1Spec>;
#[doc = "Field `EPDIR` reader - User HW Config1 Register"]
pub type EpdirR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - User HW Config1 Register"]
    #[inline(always)]
    pub fn epdir(&self) -> EpdirR {
        EpdirR::new(self.bits)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ghwcfg1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ghwcfg1Spec;
impl crate::RegisterSpec for Ghwcfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghwcfg1::R`](R) reader structure"]
impl crate::Readable for Ghwcfg1Spec {}
#[doc = "`reset()` method sets GHWCFG1 to value 0"]
impl crate::Resettable for Ghwcfg1Spec {}
