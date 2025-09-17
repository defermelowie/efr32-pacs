#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `TXDSE0MXSEL` reader - TXD SE0 chicken bit"]
pub type Txdse0mxselR = crate::BitReader;
#[doc = "Field `TXDSE0MXSEL` writer - TXD SE0 chicken bit"]
pub type Txdse0mxselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBSSNSHEN` reader - VBUSSENSE going high - enable"]
pub type VbssnshenR = crate::BitReader;
#[doc = "Field `VBSSNSHEN` writer - VBUSSENSE going high - enable"]
pub type VbssnshenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBSSNSLEN` reader - VBUSSENSE going low - enable"]
pub type VbssnslenR = crate::BitReader;
#[doc = "Field `VBSSNSLEN` writer - VBUSSENSE going low - enable"]
pub type VbssnslenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSOVRIDE` reader - VBUS override to GPPLL"]
pub type VbusovrideR = crate::BitReader;
#[doc = "Field `VBUSOVRIDE` writer - VBUS override to GPPLL"]
pub type VbusovrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSSLEW` reader - FS Slew"]
pub type FsslewR = crate::FieldReader;
#[doc = "Field `FSSLEW` writer - FS Slew"]
pub type FsslewW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DISRUPTX` reader - Disable TX pull-up resistor"]
pub type DisruptxR = crate::BitReader;
#[doc = "Field `DISRUPTX` writer - Disable TX pull-up resistor"]
pub type DisruptxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPSVREG` reader - Bypass VREG signal to USB PHY"]
pub type BypsvregR = crate::BitReader;
#[doc = "Field `BYPSVREG` writer - Bypass VREG signal to USB PHY"]
pub type BypsvregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDATACAP` reader - Add 40pf cap to D+/D- in TX mode"]
pub type EndatacapR = crate::BitReader;
#[doc = "Field `ENDATACAP` writer - Add 40pf cap to D+/D- in TX mode"]
pub type EndatacapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TXD SE0 chicken bit"]
    #[inline(always)]
    pub fn txdse0mxsel(&self) -> Txdse0mxselR {
        Txdse0mxselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - VBUSSENSE going high - enable"]
    #[inline(always)]
    pub fn vbssnshen(&self) -> VbssnshenR {
        VbssnshenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - VBUSSENSE going low - enable"]
    #[inline(always)]
    pub fn vbssnslen(&self) -> VbssnslenR {
        VbssnslenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - VBUS override to GPPLL"]
    #[inline(always)]
    pub fn vbusovride(&self) -> VbusovrideR {
        VbusovrideR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 21:23 - FS Slew"]
    #[inline(always)]
    pub fn fsslew(&self) -> FsslewR {
        FsslewR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - Disable TX pull-up resistor"]
    #[inline(always)]
    pub fn disruptx(&self) -> DisruptxR {
        DisruptxR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bypass VREG signal to USB PHY"]
    #[inline(always)]
    pub fn bypsvreg(&self) -> BypsvregR {
        BypsvregR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Add 40pf cap to D+/D- in TX mode"]
    #[inline(always)]
    pub fn endatacap(&self) -> EndatacapR {
        EndatacapR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXD SE0 chicken bit"]
    #[inline(always)]
    pub fn txdse0mxsel(&mut self) -> Txdse0mxselW<'_, CtrlSpec> {
        Txdse0mxselW::new(self, 0)
    }
    #[doc = "Bit 16 - VBUSSENSE going high - enable"]
    #[inline(always)]
    pub fn vbssnshen(&mut self) -> VbssnshenW<'_, CtrlSpec> {
        VbssnshenW::new(self, 16)
    }
    #[doc = "Bit 17 - VBUSSENSE going low - enable"]
    #[inline(always)]
    pub fn vbssnslen(&mut self) -> VbssnslenW<'_, CtrlSpec> {
        VbssnslenW::new(self, 17)
    }
    #[doc = "Bit 18 - VBUS override to GPPLL"]
    #[inline(always)]
    pub fn vbusovride(&mut self) -> VbusovrideW<'_, CtrlSpec> {
        VbusovrideW::new(self, 18)
    }
    #[doc = "Bits 21:23 - FS Slew"]
    #[inline(always)]
    pub fn fsslew(&mut self) -> FsslewW<'_, CtrlSpec> {
        FsslewW::new(self, 21)
    }
    #[doc = "Bit 24 - Disable TX pull-up resistor"]
    #[inline(always)]
    pub fn disruptx(&mut self) -> DisruptxW<'_, CtrlSpec> {
        DisruptxW::new(self, 24)
    }
    #[doc = "Bit 25 - Bypass VREG signal to USB PHY"]
    #[inline(always)]
    pub fn bypsvreg(&mut self) -> BypsvregW<'_, CtrlSpec> {
        BypsvregW::new(self, 25)
    }
    #[doc = "Bit 26 - Add 40pf cap to D+/D- in TX mode"]
    #[inline(always)]
    pub fn endatacap(&mut self) -> EndatacapW<'_, CtrlSpec> {
        EndatacapW::new(self, 26)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0x03e4_0000"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x03e4_0000;
}
