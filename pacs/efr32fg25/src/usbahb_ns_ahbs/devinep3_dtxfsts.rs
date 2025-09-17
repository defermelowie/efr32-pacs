#[doc = "Register `DEVINEP3_DTXFSTS` reader"]
pub type R = crate::R<Devinep3DtxfstsSpec>;
#[doc = "Field `INEPTXFSPCAVAIL` reader - IN EP TxFIFO Space Avail"]
pub type IneptxfspcavailR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN EP TxFIFO Space Avail"]
    #[inline(always)]
    pub fn ineptxfspcavail(&self) -> IneptxfspcavailR {
        IneptxfspcavailR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`devinep3_dtxfsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devinep3DtxfstsSpec;
impl crate::RegisterSpec for Devinep3DtxfstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devinep3_dtxfsts::R`](R) reader structure"]
impl crate::Readable for Devinep3DtxfstsSpec {}
#[doc = "`reset()` method sets DEVINEP3_DTXFSTS to value 0x0300"]
impl crate::Resettable for Devinep3DtxfstsSpec {
    const RESET_VALUE: u32 = 0x0300;
}
