#[doc = "Register `DEVINEP1_DTXFSTS` reader"]
pub type R = crate::R<Devinep1DtxfstsSpec>;
#[doc = "Field `INEPTXFSPCAVAIL` reader - IN EP TxFIFO Space Avail"]
pub type IneptxfspcavailR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN EP TxFIFO Space Avail"]
    #[inline(always)]
    pub fn ineptxfspcavail(&self) -> IneptxfspcavailR {
        IneptxfspcavailR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`devinep1_dtxfsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devinep1DtxfstsSpec;
impl crate::RegisterSpec for Devinep1DtxfstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devinep1_dtxfsts::R`](R) reader structure"]
impl crate::Readable for Devinep1DtxfstsSpec {}
#[doc = "`reset()` method sets DEVINEP1_DTXFSTS to value 0x0300"]
impl crate::Resettable for Devinep1DtxfstsSpec {
    const RESET_VALUE: u32 = 0x0300;
}
