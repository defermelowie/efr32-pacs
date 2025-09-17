#[doc = "Register `USB0CLKCTRL` reader"]
pub type R = crate::R<Usb0clkctrlSpec>;
#[doc = "Register `USB0CLKCTRL` writer"]
pub type W = crate::W<Usb0clkctrlSpec>;
#[doc = "Clock Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "1: USBPLL0 is clocking USB0CLK"]
    Usbpll0 = 1,
    #[doc = "2: LFXO is clocking USB0CLK"]
    Lfxo = 2,
    #[doc = "3: LFRCO is clocking USB0CLK"]
    Lfrco = 3,
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
            1 => Some(Clksel::Usbpll0),
            2 => Some(Clksel::Lfxo),
            3 => Some(Clksel::Lfrco),
            _ => None,
        }
    }
    #[doc = "USBPLL0 is clocking USB0CLK"]
    #[inline(always)]
    pub fn is_usbpll0(&self) -> bool {
        *self == Clksel::Usbpll0
    }
    #[doc = "LFXO is clocking USB0CLK"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Clksel::Lfxo
    }
    #[doc = "LFRCO is clocking USB0CLK"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Clksel::Lfrco
    }
}
#[doc = "Field `CLKSEL` writer - Clock Select"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clksel>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USBPLL0 is clocking USB0CLK"]
    #[inline(always)]
    pub fn usbpll0(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Usbpll0)
    }
    #[doc = "LFXO is clocking USB0CLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Lfxo)
    }
    #[doc = "LFRCO is clocking USB0CLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Lfrco)
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
    pub fn clksel(&mut self) -> ClkselW<'_, Usb0clkctrlSpec> {
        ClkselW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`usb0clkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb0clkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb0clkctrlSpec;
impl crate::RegisterSpec for Usb0clkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb0clkctrl::R`](R) reader structure"]
impl crate::Readable for Usb0clkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`usb0clkctrl::W`](W) writer structure"]
impl crate::Writable for Usb0clkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USB0CLKCTRL to value 0x01"]
impl crate::Resettable for Usb0clkctrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
