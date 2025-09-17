#[doc = "Register `GHWCFG2` reader"]
pub type R = crate::R<Ghwcfg2Spec>;
#[doc = "Field `OTGMODE` reader - New BitField"]
pub type OtgmodeR = crate::FieldReader;
#[doc = "Field `OTGARCH` reader - Architecture"]
pub type OtgarchR = crate::FieldReader;
#[doc = "Field `SINGPNT` reader - Point-to-Point"]
pub type SingpntR = crate::BitReader;
#[doc = "Field `HSPHYTYPE` reader - New BitField"]
pub type HsphytypeR = crate::FieldReader;
#[doc = "Field `FSPHYTYPE` reader - Full-Speed PHY Interface Type"]
pub type FsphytypeR = crate::FieldReader;
#[doc = "Field `NUMDEVEPS` reader - Number of Device Endpoints"]
pub type NumdevepsR = crate::FieldReader;
#[doc = "Field `NUMHSTCHNL` reader - Number of Host Channels"]
pub type NumhstchnlR = crate::FieldReader;
#[doc = "Field `PERIOSUPPORT` reader - Per OUT Chan Supported in Host Mode"]
pub type PeriosupportR = crate::BitReader;
#[doc = "Field `DYNFIFOSIZING` reader - Dynamic FIFO Sizing Enabled"]
pub type DynfifosizingR = crate::BitReader;
#[doc = "Field `MULTIPROCINTRPT` reader - Multi Processor Interrupt Enabled"]
pub type MultiprocintrptR = crate::BitReader;
#[doc = "Field `NPTXQDEPTH` reader - Non-periodic Request Queue Depth"]
pub type NptxqdepthR = crate::FieldReader;
#[doc = "Field `PTXQDEPTH` reader - Host Mode Periodic Req Queue Dept"]
pub type PtxqdepthR = crate::FieldReader;
#[doc = "Field `TKNQDEPTH` reader - Device Mode IN Token Seq Queue Depth"]
pub type TknqdepthR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - New BitField"]
    #[inline(always)]
    pub fn otgmode(&self) -> OtgmodeR {
        OtgmodeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Architecture"]
    #[inline(always)]
    pub fn otgarch(&self) -> OtgarchR {
        OtgarchR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Point-to-Point"]
    #[inline(always)]
    pub fn singpnt(&self) -> SingpntR {
        SingpntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - New BitField"]
    #[inline(always)]
    pub fn hsphytype(&self) -> HsphytypeR {
        HsphytypeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Full-Speed PHY Interface Type"]
    #[inline(always)]
    pub fn fsphytype(&self) -> FsphytypeR {
        FsphytypeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:13 - Number of Device Endpoints"]
    #[inline(always)]
    pub fn numdeveps(&self) -> NumdevepsR {
        NumdevepsR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:17 - Number of Host Channels"]
    #[inline(always)]
    pub fn numhstchnl(&self) -> NumhstchnlR {
        NumhstchnlR::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - Per OUT Chan Supported in Host Mode"]
    #[inline(always)]
    pub fn periosupport(&self) -> PeriosupportR {
        PeriosupportR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Dynamic FIFO Sizing Enabled"]
    #[inline(always)]
    pub fn dynfifosizing(&self) -> DynfifosizingR {
        DynfifosizingR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Multi Processor Interrupt Enabled"]
    #[inline(always)]
    pub fn multiprocintrpt(&self) -> MultiprocintrptR {
        MultiprocintrptR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Non-periodic Request Queue Depth"]
    #[inline(always)]
    pub fn nptxqdepth(&self) -> NptxqdepthR {
        NptxqdepthR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Host Mode Periodic Req Queue Dept"]
    #[inline(always)]
    pub fn ptxqdepth(&self) -> PtxqdepthR {
        PtxqdepthR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:30 - Device Mode IN Token Seq Queue Depth"]
    #[inline(always)]
    pub fn tknqdepth(&self) -> TknqdepthR {
        TknqdepthR::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ghwcfg2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ghwcfg2Spec;
impl crate::RegisterSpec for Ghwcfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghwcfg2::R`](R) reader structure"]
impl crate::Readable for Ghwcfg2Spec {}
#[doc = "`reset()` method sets GHWCFG2 to value 0xc014"]
impl crate::Resettable for Ghwcfg2Spec {
    const RESET_VALUE: u32 = 0xc014;
}
