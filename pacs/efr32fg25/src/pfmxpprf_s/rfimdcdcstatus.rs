#[doc = "Register `RFIMDCDCSTATUS` reader"]
pub type R = crate::R<RfimdcdcstatusSpec>;
#[doc = "Field `DCDCEN` reader - DCDC Enable Status"]
pub type DcdcenR = crate::BitReader;
#[doc = "Field `TXMAXSTATUS` reader - TX MAX Status"]
pub type TxmaxstatusR = crate::BitReader;
#[doc = "Field `RXPPSTATUS` reader - RX PP Status"]
pub type RxppstatusR = crate::BitReader;
#[doc = "Field `WNO1` reader - Cal Loop WNO1 value"]
pub type Wno1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - DCDC Enable Status"]
    #[inline(always)]
    pub fn dcdcen(&self) -> DcdcenR {
        DcdcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX MAX Status"]
    #[inline(always)]
    pub fn txmaxstatus(&self) -> TxmaxstatusR {
        TxmaxstatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX PP Status"]
    #[inline(always)]
    pub fn rxppstatus(&self) -> RxppstatusR {
        RxppstatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:16 - Cal Loop WNO1 value"]
    #[inline(always)]
    pub fn wno1(&self) -> Wno1R {
        Wno1R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rfimdcdcstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfimdcdcstatusSpec;
impl crate::RegisterSpec for RfimdcdcstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfimdcdcstatus::R`](R) reader structure"]
impl crate::Readable for RfimdcdcstatusSpec {}
#[doc = "`reset()` method sets RFIMDCDCSTATUS to value 0"]
impl crate::Resettable for RfimdcdcstatusSpec {}
