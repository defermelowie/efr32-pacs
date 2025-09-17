#[doc = "Register `DAPCLKCTRL` reader"]
pub type R = crate::R<DapclkctrlSpec>;
#[doc = "Register `DAPCLKCTRL` writer"]
pub type W = crate::W<DapclkctrlSpec>;
#[doc = "Clock Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "0: DAP is not clocked"]
    Disabled = 0,
    #[doc = "1: FSRCO is clocking DAP"]
    Fsrco = 1,
    #[doc = "2: HFRCODPLL is clocking DAP"]
    Hfrcodpll = 2,
}
impl From<Clksel> for u8 {
    #[inline(always)]
    fn from(variant: Clksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clksel {
    type Ux = u8;
}
impl crate::IsEnum for Clksel {}
#[doc = "Field `CLKSEL` reader - Clock Select"]
pub type ClkselR = crate::FieldReader<Clksel>;
impl ClkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clksel> {
        match self.bits {
            0 => Some(Clksel::Disabled),
            1 => Some(Clksel::Fsrco),
            2 => Some(Clksel::Hfrcodpll),
            _ => None,
        }
    }
    #[doc = "DAP is not clocked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Clksel::Disabled
    }
    #[doc = "FSRCO is clocking DAP"]
    #[inline(always)]
    pub fn is_fsrco(&self) -> bool {
        *self == Clksel::Fsrco
    }
    #[doc = "HFRCODPLL is clocking DAP"]
    #[inline(always)]
    pub fn is_hfrcodpll(&self) -> bool {
        *self == Clksel::Hfrcodpll
    }
}
#[doc = "Field `CLKSEL` writer - Clock Select"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clksel>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DAP is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Disabled)
    }
    #[doc = "FSRCO is clocking DAP"]
    #[inline(always)]
    pub fn fsrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Fsrco)
    }
    #[doc = "HFRCODPLL is clocking DAP"]
    #[inline(always)]
    pub fn hfrcodpll(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Hfrcodpll)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock Select"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Select"]
    #[inline(always)]
    pub fn clksel(&mut self) -> ClkselW<'_, DapclkctrlSpec> {
        ClkselW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dapclkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dapclkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DapclkctrlSpec;
impl crate::RegisterSpec for DapclkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dapclkctrl::R`](R) reader structure"]
impl crate::Readable for DapclkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dapclkctrl::W`](W) writer structure"]
impl crate::Writable for DapclkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAPCLKCTRL to value 0x01"]
impl crate::Resettable for DapclkctrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
