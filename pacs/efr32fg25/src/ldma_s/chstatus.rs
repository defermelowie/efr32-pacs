#[doc = "Register `CHSTATUS` reader"]
pub type R = crate::R<ChstatusSpec>;
#[doc = "Field `CHSTATUS` reader - DMA Channel Status"]
pub type ChstatusR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - DMA Channel Status"]
    #[inline(always)]
    pub fn chstatus(&self) -> ChstatusR {
        ChstatusR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChstatusSpec;
impl crate::RegisterSpec for ChstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chstatus::R`](R) reader structure"]
impl crate::Readable for ChstatusSpec {}
#[doc = "`reset()` method sets CHSTATUS to value 0"]
impl crate::Resettable for ChstatusSpec {}
