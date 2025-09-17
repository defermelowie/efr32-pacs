#[doc = "Register `RESFIFO` reader"]
pub type R = crate::R<ResfifoSpec>;
#[doc = "Field `BUFDATASRC` reader - Result data and source"]
pub type BufdatasrcR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - Result data and source"]
    #[inline(always)]
    pub fn bufdatasrc(&self) -> BufdatasrcR {
        BufdatasrcR::new(self.bits & 0x000f_ffff)
    }
}
#[doc = "Result Fifo\n\nYou can [`read`](crate::Reg::read) this register and get [`resfifo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResfifoSpec;
impl crate::RegisterSpec for ResfifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resfifo::R`](R) reader structure"]
impl crate::Readable for ResfifoSpec {}
#[doc = "`reset()` method sets RESFIFO to value 0"]
impl crate::Resettable for ResfifoSpec {}
