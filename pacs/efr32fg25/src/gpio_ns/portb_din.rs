#[doc = "Register `PORTB_DIN` reader"]
pub type R = crate::R<PortbDinSpec>;
#[doc = "Field `DIN` reader - Data input"]
pub type DinR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - Data input"]
    #[inline(always)]
    pub fn din(&self) -> DinR {
        DinR::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "data in\n\nYou can [`read`](crate::Reg::read) this register and get [`portb_din::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PortbDinSpec;
impl crate::RegisterSpec for PortbDinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`portb_din::R`](R) reader structure"]
impl crate::Readable for PortbDinSpec {}
#[doc = "`reset()` method sets PORTB_DIN to value 0"]
impl crate::Resettable for PortbDinSpec {}
