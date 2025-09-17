#[doc = "Register `GRXSTSP` reader"]
pub type R = crate::R<GrxstspSpec>;
#[doc = "Field `EPNUM` reader - Endpoint Number"]
pub type EpnumR = crate::FieldReader;
#[doc = "Field `BCNT` reader - Byte Count"]
pub type BcntR = crate::FieldReader<u16>;
#[doc = "Field `DPID` reader - Data PID"]
pub type DpidR = crate::FieldReader;
#[doc = "Field `PKTSTS` reader - Packet Status"]
pub type PktstsR = crate::FieldReader;
#[doc = "Field `FN` reader - Frame Number"]
pub type FnR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Endpoint Number"]
    #[inline(always)]
    pub fn epnum(&self) -> EpnumR {
        EpnumR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14 - Byte Count"]
    #[inline(always)]
    pub fn bcnt(&self) -> BcntR {
        BcntR::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DpidR {
        DpidR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:20 - Packet Status"]
    #[inline(always)]
    pub fn pktsts(&self) -> PktstsR {
        PktstsR::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:24 - Frame Number"]
    #[inline(always)]
    pub fn fn_(&self) -> FnR {
        FnR::new(((self.bits >> 21) & 0x0f) as u8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`grxstsp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrxstspSpec;
impl crate::RegisterSpec for GrxstspSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grxstsp::R`](R) reader structure"]
impl crate::Readable for GrxstspSpec {}
#[doc = "`reset()` method sets GRXSTSP to value 0"]
impl crate::Resettable for GrxstspSpec {}
