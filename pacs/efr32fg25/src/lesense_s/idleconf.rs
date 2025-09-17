#[doc = "Register `IDLECONF` reader"]
pub type R = crate::R<IdleconfSpec>;
#[doc = "Register `IDLECONF` writer"]
pub type W = crate::W<IdleconfSpec>;
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chidle0 {
    #[doc = "0: Channel output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: Channel output is high in idle phase"]
    High = 1,
    #[doc = "2: Channel output is low in idle phase"]
    Low = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    Dac = 3,
}
impl From<Chidle0> for u8 {
    #[inline(always)]
    fn from(variant: Chidle0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chidle0 {
    type Ux = u8;
}
impl crate::IsEnum for Chidle0 {}
#[doc = "Field `CHIDLE0` reader - Channel IDLE configuration"]
pub type Chidle0R = crate::FieldReader<Chidle0>;
impl Chidle0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chidle0 {
        match self.bits {
            0 => Chidle0::Disable,
            1 => Chidle0::High,
            2 => Chidle0::Low,
            3 => Chidle0::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chidle0::Disable
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Chidle0::High
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Chidle0::Low
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Chidle0::Dac
    }
}
#[doc = "Field `CHIDLE0` writer - Channel IDLE configuration"]
pub type Chidle0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Chidle0, crate::Safe>;
impl<'a, REG> Chidle0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle0::Disable)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle0::High)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle0::Low)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle0::Dac)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chidle1 {
    #[doc = "0: Channel output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: Channel output is high in idle phase"]
    High = 1,
    #[doc = "2: Channel output is low in idle phase"]
    Low = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    Dac = 3,
}
impl From<Chidle1> for u8 {
    #[inline(always)]
    fn from(variant: Chidle1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chidle1 {
    type Ux = u8;
}
impl crate::IsEnum for Chidle1 {}
#[doc = "Field `CHIDLE1` reader - Channel IDLE configuration"]
pub type Chidle1R = crate::FieldReader<Chidle1>;
impl Chidle1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chidle1 {
        match self.bits {
            0 => Chidle1::Disable,
            1 => Chidle1::High,
            2 => Chidle1::Low,
            3 => Chidle1::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chidle1::Disable
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Chidle1::High
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Chidle1::Low
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Chidle1::Dac
    }
}
#[doc = "Field `CHIDLE1` writer - Channel IDLE configuration"]
pub type Chidle1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Chidle1, crate::Safe>;
impl<'a, REG> Chidle1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle1::Disable)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle1::High)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle1::Low)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle1::Dac)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chidle2 {
    #[doc = "0: Channel output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: Channel output is high in idle phase"]
    High = 1,
    #[doc = "2: Channel output is low in idle phase"]
    Low = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    Dac = 3,
}
impl From<Chidle2> for u8 {
    #[inline(always)]
    fn from(variant: Chidle2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chidle2 {
    type Ux = u8;
}
impl crate::IsEnum for Chidle2 {}
#[doc = "Field `CHIDLE2` reader - Channel IDLE configuration"]
pub type Chidle2R = crate::FieldReader<Chidle2>;
impl Chidle2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chidle2 {
        match self.bits {
            0 => Chidle2::Disable,
            1 => Chidle2::High,
            2 => Chidle2::Low,
            3 => Chidle2::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chidle2::Disable
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Chidle2::High
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Chidle2::Low
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Chidle2::Dac
    }
}
#[doc = "Field `CHIDLE2` writer - Channel IDLE configuration"]
pub type Chidle2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Chidle2, crate::Safe>;
impl<'a, REG> Chidle2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle2::Disable)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle2::High)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle2::Low)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle2::Dac)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chidle3 {
    #[doc = "0: Channel output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: Channel output is high in idle phase"]
    High = 1,
    #[doc = "2: Channel output is low in idle phase"]
    Low = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    Dac = 3,
}
impl From<Chidle3> for u8 {
    #[inline(always)]
    fn from(variant: Chidle3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chidle3 {
    type Ux = u8;
}
impl crate::IsEnum for Chidle3 {}
#[doc = "Field `CHIDLE3` reader - Channel IDLE configuration"]
pub type Chidle3R = crate::FieldReader<Chidle3>;
impl Chidle3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chidle3 {
        match self.bits {
            0 => Chidle3::Disable,
            1 => Chidle3::High,
            2 => Chidle3::Low,
            3 => Chidle3::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chidle3::Disable
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Chidle3::High
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Chidle3::Low
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Chidle3::Dac
    }
}
#[doc = "Field `CHIDLE3` writer - Channel IDLE configuration"]
pub type Chidle3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Chidle3, crate::Safe>;
impl<'a, REG> Chidle3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle3::Disable)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle3::High)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle3::Low)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle3::Dac)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chidle4 {
    #[doc = "0: Channel output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: Channel output is high in idle phase"]
    High = 1,
    #[doc = "2: Channel output is low in idle phase"]
    Low = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    Dac = 3,
}
impl From<Chidle4> for u8 {
    #[inline(always)]
    fn from(variant: Chidle4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chidle4 {
    type Ux = u8;
}
impl crate::IsEnum for Chidle4 {}
#[doc = "Field `CHIDLE4` reader - Channel IDLE configuration"]
pub type Chidle4R = crate::FieldReader<Chidle4>;
impl Chidle4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chidle4 {
        match self.bits {
            0 => Chidle4::Disable,
            1 => Chidle4::High,
            2 => Chidle4::Low,
            3 => Chidle4::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chidle4::Disable
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Chidle4::High
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Chidle4::Low
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Chidle4::Dac
    }
}
#[doc = "Field `CHIDLE4` writer - Channel IDLE configuration"]
pub type Chidle4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Chidle4, crate::Safe>;
impl<'a, REG> Chidle4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle4::Disable)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle4::High)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle4::Low)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle4::Dac)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chidle5 {
    #[doc = "0: Channel output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: Channel output is high in idle phase"]
    High = 1,
    #[doc = "2: Channel output is low in idle phase"]
    Low = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    Dac = 3,
}
impl From<Chidle5> for u8 {
    #[inline(always)]
    fn from(variant: Chidle5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chidle5 {
    type Ux = u8;
}
impl crate::IsEnum for Chidle5 {}
#[doc = "Field `CHIDLE5` reader - Channel IDLE configuration"]
pub type Chidle5R = crate::FieldReader<Chidle5>;
impl Chidle5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chidle5 {
        match self.bits {
            0 => Chidle5::Disable,
            1 => Chidle5::High,
            2 => Chidle5::Low,
            3 => Chidle5::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chidle5::Disable
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Chidle5::High
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Chidle5::Low
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Chidle5::Dac
    }
}
#[doc = "Field `CHIDLE5` writer - Channel IDLE configuration"]
pub type Chidle5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Chidle5, crate::Safe>;
impl<'a, REG> Chidle5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle5::Disable)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle5::High)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle5::Low)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle5::Dac)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chidle6 {
    #[doc = "0: Channel output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: Channel output is high in idle phase"]
    High = 1,
    #[doc = "2: Channel output is low in idle phase"]
    Low = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    Dac = 3,
}
impl From<Chidle6> for u8 {
    #[inline(always)]
    fn from(variant: Chidle6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chidle6 {
    type Ux = u8;
}
impl crate::IsEnum for Chidle6 {}
#[doc = "Field `CHIDLE6` reader - Channel IDLE configuration"]
pub type Chidle6R = crate::FieldReader<Chidle6>;
impl Chidle6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chidle6 {
        match self.bits {
            0 => Chidle6::Disable,
            1 => Chidle6::High,
            2 => Chidle6::Low,
            3 => Chidle6::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chidle6::Disable
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Chidle6::High
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Chidle6::Low
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Chidle6::Dac
    }
}
#[doc = "Field `CHIDLE6` writer - Channel IDLE configuration"]
pub type Chidle6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Chidle6, crate::Safe>;
impl<'a, REG> Chidle6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle6::Disable)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle6::High)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle6::Low)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle6::Dac)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chidle7 {
    #[doc = "0: Channel output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: Channel output is high in idle phase"]
    High = 1,
    #[doc = "2: Channel output is low in idle phase"]
    Low = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    Dac = 3,
}
impl From<Chidle7> for u8 {
    #[inline(always)]
    fn from(variant: Chidle7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chidle7 {
    type Ux = u8;
}
impl crate::IsEnum for Chidle7 {}
#[doc = "Field `CHIDLE7` reader - Channel IDLE configuration"]
pub type Chidle7R = crate::FieldReader<Chidle7>;
impl Chidle7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chidle7 {
        match self.bits {
            0 => Chidle7::Disable,
            1 => Chidle7::High,
            2 => Chidle7::Low,
            3 => Chidle7::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chidle7::Disable
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Chidle7::High
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Chidle7::Low
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Chidle7::Dac
    }
}
#[doc = "Field `CHIDLE7` writer - Channel IDLE configuration"]
pub type Chidle7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Chidle7, crate::Safe>;
impl<'a, REG> Chidle7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle7::Disable)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle7::High)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle7::Low)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle7::Dac)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chidle8 {
    #[doc = "0: Channel output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: Channel output is high in idle phase"]
    High = 1,
    #[doc = "2: Channel output is low in idle phase"]
    Low = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    Dac = 3,
}
impl From<Chidle8> for u8 {
    #[inline(always)]
    fn from(variant: Chidle8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chidle8 {
    type Ux = u8;
}
impl crate::IsEnum for Chidle8 {}
#[doc = "Field `CHIDLE8` reader - Channel IDLE configuration"]
pub type Chidle8R = crate::FieldReader<Chidle8>;
impl Chidle8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chidle8 {
        match self.bits {
            0 => Chidle8::Disable,
            1 => Chidle8::High,
            2 => Chidle8::Low,
            3 => Chidle8::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chidle8::Disable
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Chidle8::High
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Chidle8::Low
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Chidle8::Dac
    }
}
#[doc = "Field `CHIDLE8` writer - Channel IDLE configuration"]
pub type Chidle8W<'a, REG> = crate::FieldWriter<'a, REG, 2, Chidle8, crate::Safe>;
impl<'a, REG> Chidle8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle8::Disable)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle8::High)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle8::Low)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle8::Dac)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chidle9 {
    #[doc = "0: Channel output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: Channel output is high in idle phase"]
    High = 1,
    #[doc = "2: Channel output is low in idle phase"]
    Low = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    Dac = 3,
}
impl From<Chidle9> for u8 {
    #[inline(always)]
    fn from(variant: Chidle9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chidle9 {
    type Ux = u8;
}
impl crate::IsEnum for Chidle9 {}
#[doc = "Field `CHIDLE9` reader - Channel IDLE configuration"]
pub type Chidle9R = crate::FieldReader<Chidle9>;
impl Chidle9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chidle9 {
        match self.bits {
            0 => Chidle9::Disable,
            1 => Chidle9::High,
            2 => Chidle9::Low,
            3 => Chidle9::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chidle9::Disable
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Chidle9::High
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Chidle9::Low
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Chidle9::Dac
    }
}
#[doc = "Field `CHIDLE9` writer - Channel IDLE configuration"]
pub type Chidle9W<'a, REG> = crate::FieldWriter<'a, REG, 2, Chidle9, crate::Safe>;
impl<'a, REG> Chidle9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle9::Disable)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle9::High)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle9::Low)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle9::Dac)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chidle10 {
    #[doc = "0: Channel output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: Channel output is high in idle phase"]
    High = 1,
    #[doc = "2: Channel output is low in idle phase"]
    Low = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    Dac = 3,
}
impl From<Chidle10> for u8 {
    #[inline(always)]
    fn from(variant: Chidle10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chidle10 {
    type Ux = u8;
}
impl crate::IsEnum for Chidle10 {}
#[doc = "Field `CHIDLE10` reader - Channel IDLE configuration"]
pub type Chidle10R = crate::FieldReader<Chidle10>;
impl Chidle10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chidle10 {
        match self.bits {
            0 => Chidle10::Disable,
            1 => Chidle10::High,
            2 => Chidle10::Low,
            3 => Chidle10::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chidle10::Disable
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Chidle10::High
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Chidle10::Low
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Chidle10::Dac
    }
}
#[doc = "Field `CHIDLE10` writer - Channel IDLE configuration"]
pub type Chidle10W<'a, REG> = crate::FieldWriter<'a, REG, 2, Chidle10, crate::Safe>;
impl<'a, REG> Chidle10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle10::Disable)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle10::High)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle10::Low)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle10::Dac)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chidle11 {
    #[doc = "0: Channel output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: Channel output is high in idle phase"]
    High = 1,
    #[doc = "2: Channel output is low in idle phase"]
    Low = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    Dac = 3,
}
impl From<Chidle11> for u8 {
    #[inline(always)]
    fn from(variant: Chidle11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chidle11 {
    type Ux = u8;
}
impl crate::IsEnum for Chidle11 {}
#[doc = "Field `CHIDLE11` reader - Channel IDLE configuration"]
pub type Chidle11R = crate::FieldReader<Chidle11>;
impl Chidle11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chidle11 {
        match self.bits {
            0 => Chidle11::Disable,
            1 => Chidle11::High,
            2 => Chidle11::Low,
            3 => Chidle11::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chidle11::Disable
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Chidle11::High
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Chidle11::Low
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Chidle11::Dac
    }
}
#[doc = "Field `CHIDLE11` writer - Channel IDLE configuration"]
pub type Chidle11W<'a, REG> = crate::FieldWriter<'a, REG, 2, Chidle11, crate::Safe>;
impl<'a, REG> Chidle11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle11::Disable)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle11::High)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle11::Low)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle11::Dac)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chidle12 {
    #[doc = "0: Channel output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: Channel output is high in idle phase"]
    High = 1,
    #[doc = "2: Channel output is low in idle phase"]
    Low = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    Dac = 3,
}
impl From<Chidle12> for u8 {
    #[inline(always)]
    fn from(variant: Chidle12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chidle12 {
    type Ux = u8;
}
impl crate::IsEnum for Chidle12 {}
#[doc = "Field `CHIDLE12` reader - Channel IDLE configuration"]
pub type Chidle12R = crate::FieldReader<Chidle12>;
impl Chidle12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chidle12 {
        match self.bits {
            0 => Chidle12::Disable,
            1 => Chidle12::High,
            2 => Chidle12::Low,
            3 => Chidle12::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chidle12::Disable
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Chidle12::High
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Chidle12::Low
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Chidle12::Dac
    }
}
#[doc = "Field `CHIDLE12` writer - Channel IDLE configuration"]
pub type Chidle12W<'a, REG> = crate::FieldWriter<'a, REG, 2, Chidle12, crate::Safe>;
impl<'a, REG> Chidle12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle12::Disable)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle12::High)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle12::Low)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle12::Dac)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chidle13 {
    #[doc = "0: Channel output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: Channel output is high in idle phase"]
    High = 1,
    #[doc = "2: Channel output is low in idle phase"]
    Low = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    Dac = 3,
}
impl From<Chidle13> for u8 {
    #[inline(always)]
    fn from(variant: Chidle13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chidle13 {
    type Ux = u8;
}
impl crate::IsEnum for Chidle13 {}
#[doc = "Field `CHIDLE13` reader - Channel IDLE configuration"]
pub type Chidle13R = crate::FieldReader<Chidle13>;
impl Chidle13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chidle13 {
        match self.bits {
            0 => Chidle13::Disable,
            1 => Chidle13::High,
            2 => Chidle13::Low,
            3 => Chidle13::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chidle13::Disable
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Chidle13::High
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Chidle13::Low
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Chidle13::Dac
    }
}
#[doc = "Field `CHIDLE13` writer - Channel IDLE configuration"]
pub type Chidle13W<'a, REG> = crate::FieldWriter<'a, REG, 2, Chidle13, crate::Safe>;
impl<'a, REG> Chidle13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle13::Disable)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle13::High)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle13::Low)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle13::Dac)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chidle14 {
    #[doc = "0: Channel output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: Channel output is high in idle phase"]
    High = 1,
    #[doc = "2: Channel output is low in idle phase"]
    Low = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    Dac = 3,
}
impl From<Chidle14> for u8 {
    #[inline(always)]
    fn from(variant: Chidle14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chidle14 {
    type Ux = u8;
}
impl crate::IsEnum for Chidle14 {}
#[doc = "Field `CHIDLE14` reader - Channel IDLE configuration"]
pub type Chidle14R = crate::FieldReader<Chidle14>;
impl Chidle14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chidle14 {
        match self.bits {
            0 => Chidle14::Disable,
            1 => Chidle14::High,
            2 => Chidle14::Low,
            3 => Chidle14::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chidle14::Disable
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Chidle14::High
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Chidle14::Low
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Chidle14::Dac
    }
}
#[doc = "Field `CHIDLE14` writer - Channel IDLE configuration"]
pub type Chidle14W<'a, REG> = crate::FieldWriter<'a, REG, 2, Chidle14, crate::Safe>;
impl<'a, REG> Chidle14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle14::Disable)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle14::High)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle14::Low)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle14::Dac)
    }
}
#[doc = "Channel IDLE configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chidle15 {
    #[doc = "0: Channel output is disabled in idle phase"]
    Disable = 0,
    #[doc = "1: Channel output is high in idle phase"]
    High = 1,
    #[doc = "2: Channel output is low in idle phase"]
    Low = 2,
    #[doc = "3: Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    Dac = 3,
}
impl From<Chidle15> for u8 {
    #[inline(always)]
    fn from(variant: Chidle15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chidle15 {
    type Ux = u8;
}
impl crate::IsEnum for Chidle15 {}
#[doc = "Field `CHIDLE15` reader - Channel IDLE configuration"]
pub type Chidle15R = crate::FieldReader<Chidle15>;
impl Chidle15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chidle15 {
        match self.bits {
            0 => Chidle15::Disable,
            1 => Chidle15::High,
            2 => Chidle15::Low,
            3 => Chidle15::Dac,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chidle15::Disable
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Chidle15::High
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Chidle15::Low
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Chidle15::Dac
    }
}
#[doc = "Field `CHIDLE15` writer - Channel IDLE configuration"]
pub type Chidle15W<'a, REG> = crate::FieldWriter<'a, REG, 2, Chidle15, crate::Safe>;
impl<'a, REG> Chidle15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle15::Disable)
    }
    #[doc = "Channel output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle15::High)
    }
    #[doc = "Channel output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle15::Low)
    }
    #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Chidle15::Dac)
    }
}
impl R {
    #[doc = "Bits 0:1 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle0(&self) -> Chidle0R {
        Chidle0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle1(&self) -> Chidle1R {
        Chidle1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle2(&self) -> Chidle2R {
        Chidle2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle3(&self) -> Chidle3R {
        Chidle3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle4(&self) -> Chidle4R {
        Chidle4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle5(&self) -> Chidle5R {
        Chidle5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle6(&self) -> Chidle6R {
        Chidle6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle7(&self) -> Chidle7R {
        Chidle7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle8(&self) -> Chidle8R {
        Chidle8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle9(&self) -> Chidle9R {
        Chidle9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle10(&self) -> Chidle10R {
        Chidle10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle11(&self) -> Chidle11R {
        Chidle11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle12(&self) -> Chidle12R {
        Chidle12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle13(&self) -> Chidle13R {
        Chidle13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle14(&self) -> Chidle14R {
        Chidle14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle15(&self) -> Chidle15R {
        Chidle15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle0(&mut self) -> Chidle0W<'_, IdleconfSpec> {
        Chidle0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle1(&mut self) -> Chidle1W<'_, IdleconfSpec> {
        Chidle1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle2(&mut self) -> Chidle2W<'_, IdleconfSpec> {
        Chidle2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle3(&mut self) -> Chidle3W<'_, IdleconfSpec> {
        Chidle3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle4(&mut self) -> Chidle4W<'_, IdleconfSpec> {
        Chidle4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle5(&mut self) -> Chidle5W<'_, IdleconfSpec> {
        Chidle5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle6(&mut self) -> Chidle6W<'_, IdleconfSpec> {
        Chidle6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle7(&mut self) -> Chidle7W<'_, IdleconfSpec> {
        Chidle7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle8(&mut self) -> Chidle8W<'_, IdleconfSpec> {
        Chidle8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle9(&mut self) -> Chidle9W<'_, IdleconfSpec> {
        Chidle9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle10(&mut self) -> Chidle10W<'_, IdleconfSpec> {
        Chidle10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle11(&mut self) -> Chidle11W<'_, IdleconfSpec> {
        Chidle11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle12(&mut self) -> Chidle12W<'_, IdleconfSpec> {
        Chidle12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle13(&mut self) -> Chidle13W<'_, IdleconfSpec> {
        Chidle13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle14(&mut self) -> Chidle14W<'_, IdleconfSpec> {
        Chidle14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Channel IDLE configuration"]
    #[inline(always)]
    pub fn chidle15(&mut self) -> Chidle15W<'_, IdleconfSpec> {
        Chidle15W::new(self, 30)
    }
}
#[doc = "GPIO Idle phase configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`idleconf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idleconf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdleconfSpec;
impl crate::RegisterSpec for IdleconfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idleconf::R`](R) reader structure"]
impl crate::Readable for IdleconfSpec {}
#[doc = "`write(|w| ..)` method takes [`idleconf::W`](W) writer structure"]
impl crate::Writable for IdleconfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDLECONF to value 0"]
impl crate::Resettable for IdleconfSpec {}
