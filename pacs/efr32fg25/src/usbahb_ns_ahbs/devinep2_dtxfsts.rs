#[doc = "Register `DEVINEP2_DTXFSTS` reader"]
pub type R = crate::R<Devinep2DtxfstsSpec>;
#[doc = "Field `INEPTXFSPCAVAIL` reader - IN EP TxFIFO Space Avail"]
pub type IneptxfspcavailR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN EP TxFIFO Space Avail"]
    #[inline(always)]
    pub fn ineptxfspcavail(&self) -> IneptxfspcavailR {
        IneptxfspcavailR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`devinep2_dtxfsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devinep2DtxfstsSpec;
impl crate::RegisterSpec for Devinep2DtxfstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devinep2_dtxfsts::R`](R) reader structure"]
impl crate::Readable for Devinep2DtxfstsSpec {}
#[doc = "`reset()` method sets DEVINEP2_DTXFSTS to value 0x0300"]
impl crate::Resettable for Devinep2DtxfstsSpec {
    const RESET_VALUE: u32 = 0x0300;
}
