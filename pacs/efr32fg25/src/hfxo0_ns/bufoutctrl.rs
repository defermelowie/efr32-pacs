#[doc = "Register `BUFOUTCTRL` reader"]
pub type R = crate::R<BufoutctrlSpec>;
#[doc = "Register `BUFOUTCTRL` writer"]
pub type W = crate::W<BufoutctrlSpec>;
#[doc = "Field `XOUTBIASANA` reader - Driver Bias Current"]
pub type XoutbiasanaR = crate::FieldReader;
#[doc = "Field `XOUTBIASANA` writer - Driver Bias Current"]
pub type XoutbiasanaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `XOUTCFANA` reader - Buffer Gain"]
pub type XoutcfanaR = crate::FieldReader;
#[doc = "Field `XOUTCFANA` writer - Buffer Gain"]
pub type XoutcfanaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `XOUTGMANA` reader - No Description"]
pub type XoutgmanaR = crate::FieldReader;
#[doc = "Field `XOUTGMANA` writer - No Description"]
pub type XoutgmanaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PEAKDETTHRESANA` reader - Peak Detector Threshold for XOUT"]
pub type PeakdetthresanaR = crate::FieldReader;
#[doc = "Field `PEAKDETTHRESANA` writer - Peak Detector Threshold for XOUT"]
pub type PeakdetthresanaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Tuning Cap Change Timeout\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Timeoutctune {
    #[doc = "0: The tuning cap change timeout is set to 2 us minimum. The maximum can be +40%."]
    T2us = 0,
    #[doc = "1: The tuning cap change timeout is set to 5 us minimum. The maximum can be +40%."]
    T5us = 1,
    #[doc = "2: The tuning cap change timeout is set to 10 us minimum. The maximum can be +40%."]
    T10us = 2,
    #[doc = "3: The tuning cap change timeout is set to 16 us minimum. The maximum can be +40%."]
    T16us = 3,
    #[doc = "4: The tuning cap change timeout is set to 21 us minimum. The maximum can be +40%."]
    T21us = 4,
    #[doc = "5: The tuning cap change timeout is set to 26 us minimum. The maximum can be +40%."]
    T26us = 5,
    #[doc = "6: The tuning cap change timeout is set to 31 us minimum. The maximum can be +40%."]
    T31us = 6,
    #[doc = "7: The tuning cap change timeout is set to 42 us minimum. The maximum can be +40%."]
    T42us = 7,
    #[doc = "8: The tuning cap change timeout is set to 52 us minimum. The maximum can be +40%."]
    T52us = 8,
    #[doc = "9: The tuning cap change timeout is set to 63 us minimum. The maximum can be +40%."]
    T63us = 9,
    #[doc = "10: The tuning cap change timeout is set to 83 us minimum. The maximum can be +40%."]
    T83us = 10,
    #[doc = "11: The tuning cap change timeout is set to 104 us minimum. The maximum can be +40%."]
    T104us = 11,
    #[doc = "12: The tuning cap change timeout is set to 208 us minimum. The maximum can be +40%."]
    T208us = 12,
    #[doc = "13: The tuning cap change timeout is set to 313 us minimum. The maximum can be +40%."]
    T313us = 13,
    #[doc = "14: The tuning cap change timeout is set to 521 us minimum. The maximum can be +40%."]
    T521us = 14,
    #[doc = "15: The tuning cap change timeout is set to 938 us minimum. The maximum can be +40%."]
    T938us = 15,
}
impl From<Timeoutctune> for u8 {
    #[inline(always)]
    fn from(variant: Timeoutctune) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Timeoutctune {
    type Ux = u8;
}
impl crate::IsEnum for Timeoutctune {}
#[doc = "Field `TIMEOUTCTUNE` reader - Tuning Cap Change Timeout"]
pub type TimeoutctuneR = crate::FieldReader<Timeoutctune>;
impl TimeoutctuneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timeoutctune {
        match self.bits {
            0 => Timeoutctune::T2us,
            1 => Timeoutctune::T5us,
            2 => Timeoutctune::T10us,
            3 => Timeoutctune::T16us,
            4 => Timeoutctune::T21us,
            5 => Timeoutctune::T26us,
            6 => Timeoutctune::T31us,
            7 => Timeoutctune::T42us,
            8 => Timeoutctune::T52us,
            9 => Timeoutctune::T63us,
            10 => Timeoutctune::T83us,
            11 => Timeoutctune::T104us,
            12 => Timeoutctune::T208us,
            13 => Timeoutctune::T313us,
            14 => Timeoutctune::T521us,
            15 => Timeoutctune::T938us,
            _ => unreachable!(),
        }
    }
    #[doc = "The tuning cap change timeout is set to 2 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t2us(&self) -> bool {
        *self == Timeoutctune::T2us
    }
    #[doc = "The tuning cap change timeout is set to 5 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t5us(&self) -> bool {
        *self == Timeoutctune::T5us
    }
    #[doc = "The tuning cap change timeout is set to 10 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t10us(&self) -> bool {
        *self == Timeoutctune::T10us
    }
    #[doc = "The tuning cap change timeout is set to 16 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t16us(&self) -> bool {
        *self == Timeoutctune::T16us
    }
    #[doc = "The tuning cap change timeout is set to 21 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t21us(&self) -> bool {
        *self == Timeoutctune::T21us
    }
    #[doc = "The tuning cap change timeout is set to 26 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t26us(&self) -> bool {
        *self == Timeoutctune::T26us
    }
    #[doc = "The tuning cap change timeout is set to 31 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t31us(&self) -> bool {
        *self == Timeoutctune::T31us
    }
    #[doc = "The tuning cap change timeout is set to 42 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t42us(&self) -> bool {
        *self == Timeoutctune::T42us
    }
    #[doc = "The tuning cap change timeout is set to 52 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t52us(&self) -> bool {
        *self == Timeoutctune::T52us
    }
    #[doc = "The tuning cap change timeout is set to 63 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t63us(&self) -> bool {
        *self == Timeoutctune::T63us
    }
    #[doc = "The tuning cap change timeout is set to 83 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t83us(&self) -> bool {
        *self == Timeoutctune::T83us
    }
    #[doc = "The tuning cap change timeout is set to 104 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t104us(&self) -> bool {
        *self == Timeoutctune::T104us
    }
    #[doc = "The tuning cap change timeout is set to 208 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t208us(&self) -> bool {
        *self == Timeoutctune::T208us
    }
    #[doc = "The tuning cap change timeout is set to 313 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t313us(&self) -> bool {
        *self == Timeoutctune::T313us
    }
    #[doc = "The tuning cap change timeout is set to 521 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t521us(&self) -> bool {
        *self == Timeoutctune::T521us
    }
    #[doc = "The tuning cap change timeout is set to 938 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t938us(&self) -> bool {
        *self == Timeoutctune::T938us
    }
}
#[doc = "Field `TIMEOUTCTUNE` writer - Tuning Cap Change Timeout"]
pub type TimeoutctuneW<'a, REG> = crate::FieldWriter<'a, REG, 4, Timeoutctune, crate::Safe>;
impl<'a, REG> TimeoutctuneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The tuning cap change timeout is set to 2 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t2us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutctune::T2us)
    }
    #[doc = "The tuning cap change timeout is set to 5 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t5us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutctune::T5us)
    }
    #[doc = "The tuning cap change timeout is set to 10 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t10us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutctune::T10us)
    }
    #[doc = "The tuning cap change timeout is set to 16 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t16us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutctune::T16us)
    }
    #[doc = "The tuning cap change timeout is set to 21 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t21us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutctune::T21us)
    }
    #[doc = "The tuning cap change timeout is set to 26 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t26us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutctune::T26us)
    }
    #[doc = "The tuning cap change timeout is set to 31 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t31us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutctune::T31us)
    }
    #[doc = "The tuning cap change timeout is set to 42 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t42us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutctune::T42us)
    }
    #[doc = "The tuning cap change timeout is set to 52 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t52us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutctune::T52us)
    }
    #[doc = "The tuning cap change timeout is set to 63 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t63us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutctune::T63us)
    }
    #[doc = "The tuning cap change timeout is set to 83 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t83us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutctune::T83us)
    }
    #[doc = "The tuning cap change timeout is set to 104 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t104us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutctune::T104us)
    }
    #[doc = "The tuning cap change timeout is set to 208 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t208us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutctune::T208us)
    }
    #[doc = "The tuning cap change timeout is set to 313 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t313us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutctune::T313us)
    }
    #[doc = "The tuning cap change timeout is set to 521 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t521us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutctune::T521us)
    }
    #[doc = "The tuning cap change timeout is set to 938 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t938us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutctune::T938us)
    }
}
#[doc = "Oscillator Startup Timeout\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Timeoutstartup {
    #[doc = "0: The oscillator startup timeout is set to 42 us minimum. The maximum can be +40%."]
    T42us = 0,
    #[doc = "1: The oscillator startup timeout is set to 83 us minimum. The maximum can be +40%."]
    T83us = 1,
    #[doc = "2: The oscillator startup timeout is set to 108 us minimum. The maximum can be +40%."]
    T108us = 2,
    #[doc = "3: The oscillator startup timeout is set to 133 us minimum. The maximum can be +40%."]
    T133us = 3,
    #[doc = "4: The oscillator startup timeout is set to 158 us minimum. The maximum can be +40%."]
    T158us = 4,
    #[doc = "5: The oscillator startup timeout is set to 183 us minimum. The maximum can be +40%."]
    T183us = 5,
    #[doc = "6: The oscillator startup timeout is set to 208 us minimum. The maximum can be +40%."]
    T208us = 6,
    #[doc = "7: The oscillator startup timeout is set to 233 us minimum. The maximum can be +40%."]
    T233us = 7,
    #[doc = "8: The oscillator startup timeout is set to 258 us minimum. The maximum can be +40%."]
    T258us = 8,
    #[doc = "9: The oscillator startup timeout is set to 283 us minimum. The maximum can be +40%."]
    T283us = 9,
    #[doc = "10: The oscillator startup timeout is set to 333 us minimum. The maximum can be +40%."]
    T333us = 10,
    #[doc = "11: The oscillator startup timeout is set to 375 us minimum. The maximum can be +40%."]
    T375us = 11,
    #[doc = "12: The oscillator startup timeout is set to 417 us minimum. The maximum can be +40%."]
    T417us = 12,
    #[doc = "13: The oscillator startup timeout is set to 458 us minimum. The maximum can be +40%."]
    T458us = 13,
    #[doc = "14: The oscillator startup timeout is set to 500 us minimum. The maximum can be +40%."]
    T500us = 14,
    #[doc = "15: The oscillator startup timeout is set to 667 us minimum. The maximum can be +40%."]
    T667us = 15,
}
impl From<Timeoutstartup> for u8 {
    #[inline(always)]
    fn from(variant: Timeoutstartup) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Timeoutstartup {
    type Ux = u8;
}
impl crate::IsEnum for Timeoutstartup {}
#[doc = "Field `TIMEOUTSTARTUP` reader - Oscillator Startup Timeout"]
pub type TimeoutstartupR = crate::FieldReader<Timeoutstartup>;
impl TimeoutstartupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timeoutstartup {
        match self.bits {
            0 => Timeoutstartup::T42us,
            1 => Timeoutstartup::T83us,
            2 => Timeoutstartup::T108us,
            3 => Timeoutstartup::T133us,
            4 => Timeoutstartup::T158us,
            5 => Timeoutstartup::T183us,
            6 => Timeoutstartup::T208us,
            7 => Timeoutstartup::T233us,
            8 => Timeoutstartup::T258us,
            9 => Timeoutstartup::T283us,
            10 => Timeoutstartup::T333us,
            11 => Timeoutstartup::T375us,
            12 => Timeoutstartup::T417us,
            13 => Timeoutstartup::T458us,
            14 => Timeoutstartup::T500us,
            15 => Timeoutstartup::T667us,
            _ => unreachable!(),
        }
    }
    #[doc = "The oscillator startup timeout is set to 42 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t42us(&self) -> bool {
        *self == Timeoutstartup::T42us
    }
    #[doc = "The oscillator startup timeout is set to 83 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t83us(&self) -> bool {
        *self == Timeoutstartup::T83us
    }
    #[doc = "The oscillator startup timeout is set to 108 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t108us(&self) -> bool {
        *self == Timeoutstartup::T108us
    }
    #[doc = "The oscillator startup timeout is set to 133 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t133us(&self) -> bool {
        *self == Timeoutstartup::T133us
    }
    #[doc = "The oscillator startup timeout is set to 158 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t158us(&self) -> bool {
        *self == Timeoutstartup::T158us
    }
    #[doc = "The oscillator startup timeout is set to 183 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t183us(&self) -> bool {
        *self == Timeoutstartup::T183us
    }
    #[doc = "The oscillator startup timeout is set to 208 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t208us(&self) -> bool {
        *self == Timeoutstartup::T208us
    }
    #[doc = "The oscillator startup timeout is set to 233 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t233us(&self) -> bool {
        *self == Timeoutstartup::T233us
    }
    #[doc = "The oscillator startup timeout is set to 258 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t258us(&self) -> bool {
        *self == Timeoutstartup::T258us
    }
    #[doc = "The oscillator startup timeout is set to 283 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t283us(&self) -> bool {
        *self == Timeoutstartup::T283us
    }
    #[doc = "The oscillator startup timeout is set to 333 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t333us(&self) -> bool {
        *self == Timeoutstartup::T333us
    }
    #[doc = "The oscillator startup timeout is set to 375 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t375us(&self) -> bool {
        *self == Timeoutstartup::T375us
    }
    #[doc = "The oscillator startup timeout is set to 417 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t417us(&self) -> bool {
        *self == Timeoutstartup::T417us
    }
    #[doc = "The oscillator startup timeout is set to 458 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t458us(&self) -> bool {
        *self == Timeoutstartup::T458us
    }
    #[doc = "The oscillator startup timeout is set to 500 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t500us(&self) -> bool {
        *self == Timeoutstartup::T500us
    }
    #[doc = "The oscillator startup timeout is set to 667 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t667us(&self) -> bool {
        *self == Timeoutstartup::T667us
    }
}
#[doc = "Field `TIMEOUTSTARTUP` writer - Oscillator Startup Timeout"]
pub type TimeoutstartupW<'a, REG> = crate::FieldWriter<'a, REG, 4, Timeoutstartup, crate::Safe>;
impl<'a, REG> TimeoutstartupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The oscillator startup timeout is set to 42 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t42us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutstartup::T42us)
    }
    #[doc = "The oscillator startup timeout is set to 83 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t83us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutstartup::T83us)
    }
    #[doc = "The oscillator startup timeout is set to 108 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t108us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutstartup::T108us)
    }
    #[doc = "The oscillator startup timeout is set to 133 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t133us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutstartup::T133us)
    }
    #[doc = "The oscillator startup timeout is set to 158 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t158us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutstartup::T158us)
    }
    #[doc = "The oscillator startup timeout is set to 183 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t183us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutstartup::T183us)
    }
    #[doc = "The oscillator startup timeout is set to 208 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t208us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutstartup::T208us)
    }
    #[doc = "The oscillator startup timeout is set to 233 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t233us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutstartup::T233us)
    }
    #[doc = "The oscillator startup timeout is set to 258 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t258us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutstartup::T258us)
    }
    #[doc = "The oscillator startup timeout is set to 283 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t283us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutstartup::T283us)
    }
    #[doc = "The oscillator startup timeout is set to 333 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t333us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutstartup::T333us)
    }
    #[doc = "The oscillator startup timeout is set to 375 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t375us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutstartup::T375us)
    }
    #[doc = "The oscillator startup timeout is set to 417 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t417us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutstartup::T417us)
    }
    #[doc = "The oscillator startup timeout is set to 458 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t458us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutstartup::T458us)
    }
    #[doc = "The oscillator startup timeout is set to 500 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t500us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutstartup::T500us)
    }
    #[doc = "The oscillator startup timeout is set to 667 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t667us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutstartup::T667us)
    }
}
#[doc = "Field `MINIMUMSTARTUPDELAY` reader - Minimum Startup Delay"]
pub type MinimumstartupdelayR = crate::BitReader;
#[doc = "Field `MINIMUMSTARTUPDELAY` writer - Minimum Startup Delay"]
pub type MinimumstartupdelayW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Driver Bias Current"]
    #[inline(always)]
    pub fn xoutbiasana(&self) -> XoutbiasanaR {
        XoutbiasanaR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Buffer Gain"]
    #[inline(always)]
    pub fn xoutcfana(&self) -> XoutcfanaR {
        XoutcfanaR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - No Description"]
    #[inline(always)]
    pub fn xoutgmana(&self) -> XoutgmanaR {
        XoutgmanaR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Peak Detector Threshold for XOUT"]
    #[inline(always)]
    pub fn peakdetthresana(&self) -> PeakdetthresanaR {
        PeakdetthresanaR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Tuning Cap Change Timeout"]
    #[inline(always)]
    pub fn timeoutctune(&self) -> TimeoutctuneR {
        TimeoutctuneR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Oscillator Startup Timeout"]
    #[inline(always)]
    pub fn timeoutstartup(&self) -> TimeoutstartupR {
        TimeoutstartupR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Minimum Startup Delay"]
    #[inline(always)]
    pub fn minimumstartupdelay(&self) -> MinimumstartupdelayR {
        MinimumstartupdelayR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Driver Bias Current"]
    #[inline(always)]
    pub fn xoutbiasana(&mut self) -> XoutbiasanaW<'_, BufoutctrlSpec> {
        XoutbiasanaW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Buffer Gain"]
    #[inline(always)]
    pub fn xoutcfana(&mut self) -> XoutcfanaW<'_, BufoutctrlSpec> {
        XoutcfanaW::new(self, 4)
    }
    #[doc = "Bits 8:11 - No Description"]
    #[inline(always)]
    pub fn xoutgmana(&mut self) -> XoutgmanaW<'_, BufoutctrlSpec> {
        XoutgmanaW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Peak Detector Threshold for XOUT"]
    #[inline(always)]
    pub fn peakdetthresana(&mut self) -> PeakdetthresanaW<'_, BufoutctrlSpec> {
        PeakdetthresanaW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Tuning Cap Change Timeout"]
    #[inline(always)]
    pub fn timeoutctune(&mut self) -> TimeoutctuneW<'_, BufoutctrlSpec> {
        TimeoutctuneW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Oscillator Startup Timeout"]
    #[inline(always)]
    pub fn timeoutstartup(&mut self) -> TimeoutstartupW<'_, BufoutctrlSpec> {
        TimeoutstartupW::new(self, 20)
    }
    #[doc = "Bit 31 - Minimum Startup Delay"]
    #[inline(always)]
    pub fn minimumstartupdelay(&mut self) -> MinimumstartupdelayW<'_, BufoutctrlSpec> {
        MinimumstartupdelayW::new(self, 31)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`bufoutctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bufoutctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BufoutctrlSpec;
impl crate::RegisterSpec for BufoutctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bufoutctrl::R`](R) reader structure"]
impl crate::Readable for BufoutctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`bufoutctrl::W`](W) writer structure"]
impl crate::Writable for BufoutctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUFOUTCTRL to value 0x0064_3c15"]
impl crate::Resettable for BufoutctrlSpec {
    const RESET_VALUE: u32 = 0x0064_3c15;
}
