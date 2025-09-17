#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "enable delay for comparison\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chnlcmpdlyen0 {
    #[doc = "0: Disables 1 clock delay to the TX value used for comparison."]
    X0 = 0,
    #[doc = "1: Enables 1 clock delay to the TX value used for comparison."]
    X1 = 1,
}
impl From<Chnlcmpdlyen0> for bool {
    #[inline(always)]
    fn from(variant: Chnlcmpdlyen0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHNLCMPDLYEN0` reader - enable delay for comparison"]
pub type Chnlcmpdlyen0R = crate::BitReader<Chnlcmpdlyen0>;
impl Chnlcmpdlyen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chnlcmpdlyen0 {
        match self.bits {
            false => Chnlcmpdlyen0::X0,
            true => Chnlcmpdlyen0::X1,
        }
    }
    #[doc = "Disables 1 clock delay to the TX value used for comparison."]
    #[inline(always)]
    pub fn is_x0(&self) -> bool {
        *self == Chnlcmpdlyen0::X0
    }
    #[doc = "Enables 1 clock delay to the TX value used for comparison."]
    #[inline(always)]
    pub fn is_x1(&self) -> bool {
        *self == Chnlcmpdlyen0::X1
    }
}
#[doc = "Field `CHNLCMPDLYEN0` writer - enable delay for comparison"]
pub type Chnlcmpdlyen0W<'a, REG> = crate::BitWriter<'a, REG, Chnlcmpdlyen0>;
impl<'a, REG> Chnlcmpdlyen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables 1 clock delay to the TX value used for comparison."]
    #[inline(always)]
    pub fn x0(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlcmpdlyen0::X0)
    }
    #[doc = "Enables 1 clock delay to the TX value used for comparison."]
    #[inline(always)]
    pub fn x1(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlcmpdlyen0::X1)
    }
}
#[doc = "enable detect filtering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chnltampdetfilten0 {
    #[doc = "0: Disables channel 0 Tamper detect filtering feature"]
    Disable = 0,
    #[doc = "1: Enables channel 0 Tamper detect filtering feature"]
    Enable = 1,
}
impl From<Chnltampdetfilten0> for bool {
    #[inline(always)]
    fn from(variant: Chnltampdetfilten0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHNLTAMPDETFILTEN0` reader - enable detect filtering"]
pub type Chnltampdetfilten0R = crate::BitReader<Chnltampdetfilten0>;
impl Chnltampdetfilten0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chnltampdetfilten0 {
        match self.bits {
            false => Chnltampdetfilten0::Disable,
            true => Chnltampdetfilten0::Enable,
        }
    }
    #[doc = "Disables channel 0 Tamper detect filtering feature"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chnltampdetfilten0::Disable
    }
    #[doc = "Enables channel 0 Tamper detect filtering feature"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chnltampdetfilten0::Enable
    }
}
#[doc = "Field `CHNLTAMPDETFILTEN0` writer - enable detect filtering"]
pub type Chnltampdetfilten0W<'a, REG> = crate::BitWriter<'a, REG, Chnltampdetfilten0>;
impl<'a, REG> Chnltampdetfilten0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables channel 0 Tamper detect filtering feature"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chnltampdetfilten0::Disable)
    }
    #[doc = "Enables channel 0 Tamper detect filtering feature"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chnltampdetfilten0::Enable)
    }
}
#[doc = "enable driving pad\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chnlpaden0 {
    #[doc = "0: Disables channel 0 Tamper detect filtering feature"]
    Disable = 0,
    #[doc = "1: Enables channel 0 Tamper detect filtering feature"]
    Enable = 1,
}
impl From<Chnlpaden0> for bool {
    #[inline(always)]
    fn from(variant: Chnlpaden0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHNLPADEN0` reader - enable driving pad"]
pub type Chnlpaden0R = crate::BitReader<Chnlpaden0>;
impl Chnlpaden0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chnlpaden0 {
        match self.bits {
            false => Chnlpaden0::Disable,
            true => Chnlpaden0::Enable,
        }
    }
    #[doc = "Disables channel 0 Tamper detect filtering feature"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chnlpaden0::Disable
    }
    #[doc = "Enables channel 0 Tamper detect filtering feature"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chnlpaden0::Enable
    }
}
#[doc = "Field `CHNLPADEN0` writer - enable driving pad"]
pub type Chnlpaden0W<'a, REG> = crate::BitWriter<'a, REG, Chnlpaden0>;
impl<'a, REG> Chnlpaden0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables channel 0 Tamper detect filtering feature"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlpaden0::Disable)
    }
    #[doc = "Enables channel 0 Tamper detect filtering feature"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlpaden0::Enable)
    }
}
#[doc = "enable delay for comparison\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chnlcmpdlyen1 {
    #[doc = "0: Disables 1 clock delay to the TX value used for comparison."]
    Disable = 0,
    #[doc = "1: Enables 1 clock delay to the TX value used for comparison."]
    Enable = 1,
}
impl From<Chnlcmpdlyen1> for bool {
    #[inline(always)]
    fn from(variant: Chnlcmpdlyen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHNLCMPDLYEN1` reader - enable delay for comparison"]
pub type Chnlcmpdlyen1R = crate::BitReader<Chnlcmpdlyen1>;
impl Chnlcmpdlyen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chnlcmpdlyen1 {
        match self.bits {
            false => Chnlcmpdlyen1::Disable,
            true => Chnlcmpdlyen1::Enable,
        }
    }
    #[doc = "Disables 1 clock delay to the TX value used for comparison."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chnlcmpdlyen1::Disable
    }
    #[doc = "Enables 1 clock delay to the TX value used for comparison."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chnlcmpdlyen1::Enable
    }
}
#[doc = "Field `CHNLCMPDLYEN1` writer - enable delay for comparison"]
pub type Chnlcmpdlyen1W<'a, REG> = crate::BitWriter<'a, REG, Chnlcmpdlyen1>;
impl<'a, REG> Chnlcmpdlyen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables 1 clock delay to the TX value used for comparison."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlcmpdlyen1::Disable)
    }
    #[doc = "Enables 1 clock delay to the TX value used for comparison."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlcmpdlyen1::Enable)
    }
}
#[doc = "enable detect filtering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chnltampdetfilten1 {
    #[doc = "0: Disables channel 1 Tamper detect filtering feature"]
    Disable = 0,
    #[doc = "1: Enables channel 1 Tamper detect filtering feature"]
    Enable = 1,
}
impl From<Chnltampdetfilten1> for bool {
    #[inline(always)]
    fn from(variant: Chnltampdetfilten1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHNLTAMPDETFILTEN1` reader - enable detect filtering"]
pub type Chnltampdetfilten1R = crate::BitReader<Chnltampdetfilten1>;
impl Chnltampdetfilten1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chnltampdetfilten1 {
        match self.bits {
            false => Chnltampdetfilten1::Disable,
            true => Chnltampdetfilten1::Enable,
        }
    }
    #[doc = "Disables channel 1 Tamper detect filtering feature"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chnltampdetfilten1::Disable
    }
    #[doc = "Enables channel 1 Tamper detect filtering feature"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chnltampdetfilten1::Enable
    }
}
#[doc = "Field `CHNLTAMPDETFILTEN1` writer - enable detect filtering"]
pub type Chnltampdetfilten1W<'a, REG> = crate::BitWriter<'a, REG, Chnltampdetfilten1>;
impl<'a, REG> Chnltampdetfilten1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables channel 1 Tamper detect filtering feature"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chnltampdetfilten1::Disable)
    }
    #[doc = "Enables channel 1 Tamper detect filtering feature"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chnltampdetfilten1::Enable)
    }
}
#[doc = "enable driving pad\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chnlpaden1 {
    #[doc = "0: Disables channel 0 Tamper detect filtering feature"]
    Disable = 0,
    #[doc = "1: Enables channel 0 Tamper detect filtering feature"]
    Enable = 1,
}
impl From<Chnlpaden1> for bool {
    #[inline(always)]
    fn from(variant: Chnlpaden1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHNLPADEN1` reader - enable driving pad"]
pub type Chnlpaden1R = crate::BitReader<Chnlpaden1>;
impl Chnlpaden1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chnlpaden1 {
        match self.bits {
            false => Chnlpaden1::Disable,
            true => Chnlpaden1::Enable,
        }
    }
    #[doc = "Disables channel 0 Tamper detect filtering feature"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chnlpaden1::Disable
    }
    #[doc = "Enables channel 0 Tamper detect filtering feature"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chnlpaden1::Enable
    }
}
#[doc = "Field `CHNLPADEN1` writer - enable driving pad"]
pub type Chnlpaden1W<'a, REG> = crate::BitWriter<'a, REG, Chnlpaden1>;
impl<'a, REG> Chnlpaden1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables channel 0 Tamper detect filtering feature"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlpaden1::Disable)
    }
    #[doc = "Enables channel 0 Tamper detect filtering feature"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chnlpaden1::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - enable delay for comparison"]
    #[inline(always)]
    pub fn chnlcmpdlyen0(&self) -> Chnlcmpdlyen0R {
        Chnlcmpdlyen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable detect filtering"]
    #[inline(always)]
    pub fn chnltampdetfilten0(&self) -> Chnltampdetfilten0R {
        Chnltampdetfilten0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable driving pad"]
    #[inline(always)]
    pub fn chnlpaden0(&self) -> Chnlpaden0R {
        Chnlpaden0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable delay for comparison"]
    #[inline(always)]
    pub fn chnlcmpdlyen1(&self) -> Chnlcmpdlyen1R {
        Chnlcmpdlyen1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable detect filtering"]
    #[inline(always)]
    pub fn chnltampdetfilten1(&self) -> Chnltampdetfilten1R {
        Chnltampdetfilten1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - enable driving pad"]
    #[inline(always)]
    pub fn chnlpaden1(&self) -> Chnlpaden1R {
        Chnlpaden1R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable delay for comparison"]
    #[inline(always)]
    pub fn chnlcmpdlyen0(&mut self) -> Chnlcmpdlyen0W<'_, CfgSpec> {
        Chnlcmpdlyen0W::new(self, 0)
    }
    #[doc = "Bit 1 - enable detect filtering"]
    #[inline(always)]
    pub fn chnltampdetfilten0(&mut self) -> Chnltampdetfilten0W<'_, CfgSpec> {
        Chnltampdetfilten0W::new(self, 1)
    }
    #[doc = "Bit 2 - enable driving pad"]
    #[inline(always)]
    pub fn chnlpaden0(&mut self) -> Chnlpaden0W<'_, CfgSpec> {
        Chnlpaden0W::new(self, 2)
    }
    #[doc = "Bit 3 - enable delay for comparison"]
    #[inline(always)]
    pub fn chnlcmpdlyen1(&mut self) -> Chnlcmpdlyen1W<'_, CfgSpec> {
        Chnlcmpdlyen1W::new(self, 3)
    }
    #[doc = "Bit 4 - enable detect filtering"]
    #[inline(always)]
    pub fn chnltampdetfilten1(&mut self) -> Chnltampdetfilten1W<'_, CfgSpec> {
        Chnltampdetfilten1W::new(self, 4)
    }
    #[doc = "Bit 5 - enable driving pad"]
    #[inline(always)]
    pub fn chnlpaden1(&mut self) -> Chnlpaden1W<'_, CfgSpec> {
        Chnlpaden1W::new(self, 5)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {}
