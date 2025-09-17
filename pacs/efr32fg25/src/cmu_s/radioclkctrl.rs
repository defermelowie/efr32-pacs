#[doc = "Register `RADIOCLKCTRL` reader"]
pub type R = crate::R<RadioclkctrlSpec>;
#[doc = "Register `RADIOCLKCTRL` writer"]
pub type W = crate::W<RadioclkctrlSpec>;
#[doc = "Field `EN` reader - Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Clock Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "1: FSRCO is clocking RADIOCLK"]
    Fsrco = 1,
    #[doc = "2: RFFPLL0RADIO is clocking RADIOCLK"]
    Rffpll0radio = 2,
    #[doc = "3: HFXO is clocking RADIOCLK"]
    Hfxo = 3,
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
            1 => Some(Clksel::Fsrco),
            2 => Some(Clksel::Rffpll0radio),
            3 => Some(Clksel::Hfxo),
            _ => None,
        }
    }
    #[doc = "FSRCO is clocking RADIOCLK"]
    #[inline(always)]
    pub fn is_fsrco(&self) -> bool {
        *self == Clksel::Fsrco
    }
    #[doc = "RFFPLL0RADIO is clocking RADIOCLK"]
    #[inline(always)]
    pub fn is_rffpll0radio(&self) -> bool {
        *self == Clksel::Rffpll0radio
    }
    #[doc = "HFXO is clocking RADIOCLK"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Clksel::Hfxo
    }
}
#[doc = "Field `CLKSEL` writer - Clock Select"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Clksel>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FSRCO is clocking RADIOCLK"]
    #[inline(always)]
    pub fn fsrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Fsrco)
    }
    #[doc = "RFFPLL0RADIO is clocking RADIOCLK"]
    #[inline(always)]
    pub fn rffpll0radio(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Rffpll0radio)
    }
    #[doc = "HFXO is clocking RADIOCLK"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Hfxo)
    }
}
#[doc = "Field `DBGCLK` reader - Enable Clock for Debugger"]
pub type DbgclkR = crate::BitReader;
#[doc = "Field `DBGCLK` writer - Enable Clock for Debugger"]
pub type DbgclkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - Clock Select"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 31 - Enable Clock for Debugger"]
    #[inline(always)]
    pub fn dbgclk(&self) -> DbgclkR {
        DbgclkR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, RadioclkctrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Clock Select"]
    #[inline(always)]
    pub fn clksel(&mut self) -> ClkselW<'_, RadioclkctrlSpec> {
        ClkselW::new(self, 8)
    }
    #[doc = "Bit 31 - Enable Clock for Debugger"]
    #[inline(always)]
    pub fn dbgclk(&mut self) -> DbgclkW<'_, RadioclkctrlSpec> {
        DbgclkW::new(self, 31)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`radioclkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`radioclkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RadioclkctrlSpec;
impl crate::RegisterSpec for RadioclkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`radioclkctrl::R`](R) reader structure"]
impl crate::Readable for RadioclkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`radioclkctrl::W`](W) writer structure"]
impl crate::Writable for RadioclkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RADIOCLKCTRL to value 0x0100"]
impl crate::Resettable for RadioclkctrlSpec {
    const RESET_VALUE: u32 = 0x0100;
}
