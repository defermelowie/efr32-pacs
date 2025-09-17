#[doc = "Register `CH7_EVALCFG` reader"]
pub type R = crate::R<Ch7EvalcfgSpec>;
#[doc = "Register `CH7_EVALCFG` writer"]
pub type W = crate::W<Ch7EvalcfgSpec>;
#[doc = "Field `DECODE` reader - Send result to decoder"]
pub type DecodeR = crate::BitReader;
#[doc = "Field `DECODE` writer - Send result to decoder"]
pub type DecodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Select mode for threshold comparison\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comp {
    #[doc = "0: Comparison evaluates to 1 if sensor data is less than CTRTHRESHOLD, or if the ACMP output is 0"]
    Less = 0,
    #[doc = "1: Comparison evaluates to 1 if sensor data is greater than, or equal to CTRTHRESHOLD, or if the ACMP output is 1"]
    Ge = 1,
}
impl From<Comp> for bool {
    #[inline(always)]
    fn from(variant: Comp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP` reader - Select mode for threshold comparison"]
pub type CompR = crate::BitReader<Comp>;
impl CompR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Comp {
        match self.bits {
            false => Comp::Less,
            true => Comp::Ge,
        }
    }
    #[doc = "Comparison evaluates to 1 if sensor data is less than CTRTHRESHOLD, or if the ACMP output is 0"]
    #[inline(always)]
    pub fn is_less(&self) -> bool {
        *self == Comp::Less
    }
    #[doc = "Comparison evaluates to 1 if sensor data is greater than, or equal to CTRTHRESHOLD, or if the ACMP output is 1"]
    #[inline(always)]
    pub fn is_ge(&self) -> bool {
        *self == Comp::Ge
    }
}
#[doc = "Field `COMP` writer - Select mode for threshold comparison"]
pub type CompW<'a, REG> = crate::BitWriter<'a, REG, Comp>;
impl<'a, REG> CompW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparison evaluates to 1 if sensor data is less than CTRTHRESHOLD, or if the ACMP output is 0"]
    #[inline(always)]
    pub fn less(self) -> &'a mut crate::W<REG> {
        self.variant(Comp::Less)
    }
    #[doc = "Comparison evaluates to 1 if sensor data is greater than, or equal to CTRTHRESHOLD, or if the ACMP output is 1"]
    #[inline(always)]
    pub fn ge(self) -> &'a mut crate::W<REG> {
        self.variant(Comp::Ge)
    }
}
#[doc = "Enable storing of sensor sample in resul\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Strsample {
    #[doc = "0: Nothing will be stored in the result buffer."]
    Disable = 0,
    #[doc = "1: The sensor sample data will be stored in the result buffer."]
    Data = 1,
    #[doc = "2: The data source, i.e. the channel, will be stored alongside the sensor sample data."]
    Datasrc = 2,
}
impl From<Strsample> for u8 {
    #[inline(always)]
    fn from(variant: Strsample) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Strsample {
    type Ux = u8;
}
impl crate::IsEnum for Strsample {}
#[doc = "Field `STRSAMPLE` reader - Enable storing of sensor sample in resul"]
pub type StrsampleR = crate::FieldReader<Strsample>;
impl StrsampleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Strsample> {
        match self.bits {
            0 => Some(Strsample::Disable),
            1 => Some(Strsample::Data),
            2 => Some(Strsample::Datasrc),
            _ => None,
        }
    }
    #[doc = "Nothing will be stored in the result buffer."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Strsample::Disable
    }
    #[doc = "The sensor sample data will be stored in the result buffer."]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == Strsample::Data
    }
    #[doc = "The data source, i.e. the channel, will be stored alongside the sensor sample data."]
    #[inline(always)]
    pub fn is_datasrc(&self) -> bool {
        *self == Strsample::Datasrc
    }
}
#[doc = "Field `STRSAMPLE` writer - Enable storing of sensor sample in resul"]
pub type StrsampleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Strsample>;
impl<'a, REG> StrsampleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Nothing will be stored in the result buffer."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Strsample::Disable)
    }
    #[doc = "The sensor sample data will be stored in the result buffer."]
    #[inline(always)]
    pub fn data(self) -> &'a mut crate::W<REG> {
        self.variant(Strsample::Data)
    }
    #[doc = "The data source, i.e. the channel, will be stored alongside the sensor sample data."]
    #[inline(always)]
    pub fn datasrc(self) -> &'a mut crate::W<REG> {
        self.variant(Strsample::Datasrc)
    }
}
#[doc = "Field `SCANRESINV` reader - Enable inversion of result"]
pub type ScanresinvR = crate::BitReader;
#[doc = "Field `SCANRESINV` writer - Enable inversion of result"]
pub type ScanresinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Configure evaluation mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Threshold comparison is used to evaluate sensor result"]
    Thres = 0,
    #[doc = "1: Sliding window is used to evaluate sensor result"]
    Slidingwin = 1,
    #[doc = "2: Step detection is used to evaluate sensor result"]
    Stepdet = 2,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Configure evaluation mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::Thres),
            1 => Some(Mode::Slidingwin),
            2 => Some(Mode::Stepdet),
            _ => None,
        }
    }
    #[doc = "Threshold comparison is used to evaluate sensor result"]
    #[inline(always)]
    pub fn is_thres(&self) -> bool {
        *self == Mode::Thres
    }
    #[doc = "Sliding window is used to evaluate sensor result"]
    #[inline(always)]
    pub fn is_slidingwin(&self) -> bool {
        *self == Mode::Slidingwin
    }
    #[doc = "Step detection is used to evaluate sensor result"]
    #[inline(always)]
    pub fn is_stepdet(&self) -> bool {
        *self == Mode::Stepdet
    }
}
#[doc = "Field `MODE` writer - Configure evaluation mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Threshold comparison is used to evaluate sensor result"]
    #[inline(always)]
    pub fn thres(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Thres)
    }
    #[doc = "Sliding window is used to evaluate sensor result"]
    #[inline(always)]
    pub fn slidingwin(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Slidingwin)
    }
    #[doc = "Step detection is used to evaluate sensor result"]
    #[inline(always)]
    pub fn stepdet(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Stepdet)
    }
}
impl R {
    #[doc = "Bit 2 - Send result to decoder"]
    #[inline(always)]
    pub fn decode(&self) -> DecodeR {
        DecodeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Select mode for threshold comparison"]
    #[inline(always)]
    pub fn comp(&self) -> CompR {
        CompR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Enable storing of sensor sample in resul"]
    #[inline(always)]
    pub fn strsample(&self) -> StrsampleR {
        StrsampleR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Enable inversion of result"]
    #[inline(always)]
    pub fn scanresinv(&self) -> ScanresinvR {
        ScanresinvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Configure evaluation mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Send result to decoder"]
    #[inline(always)]
    pub fn decode(&mut self) -> DecodeW<'_, Ch7EvalcfgSpec> {
        DecodeW::new(self, 2)
    }
    #[doc = "Bit 3 - Select mode for threshold comparison"]
    #[inline(always)]
    pub fn comp(&mut self) -> CompW<'_, Ch7EvalcfgSpec> {
        CompW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Enable storing of sensor sample in resul"]
    #[inline(always)]
    pub fn strsample(&mut self) -> StrsampleW<'_, Ch7EvalcfgSpec> {
        StrsampleW::new(self, 4)
    }
    #[doc = "Bit 6 - Enable inversion of result"]
    #[inline(always)]
    pub fn scanresinv(&mut self) -> ScanresinvW<'_, Ch7EvalcfgSpec> {
        ScanresinvW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Configure evaluation mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, Ch7EvalcfgSpec> {
        ModeW::new(self, 8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_evalcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_evalcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch7EvalcfgSpec;
impl crate::RegisterSpec for Ch7EvalcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch7_evalcfg::R`](R) reader structure"]
impl crate::Readable for Ch7EvalcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ch7_evalcfg::W`](W) writer structure"]
impl crate::Writable for Ch7EvalcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH7_EVALCFG to value 0"]
impl crate::Resettable for Ch7EvalcfgSpec {}
