#[doc = "Register `CH9_INTERACT` reader"]
pub type R = crate::R<Ch9InteractSpec>;
#[doc = "Register `CH9_INTERACT` writer"]
pub type W = crate::W<Ch9InteractSpec>;
#[doc = "Field `THRES` reader - ACMP threshold or DAC data"]
pub type ThresR = crate::FieldReader<u16>;
#[doc = "Field `THRES` writer - ACMP threshold or DAC data"]
pub type ThresW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Set GPIO mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Exmode {
    #[doc = "0: Disabled"]
    Disable = 0,
    #[doc = "1: Push Pull, GPIO is driven high"]
    High = 1,
    #[doc = "2: Push Pull, GPIO is driven low"]
    Low = 2,
    #[doc = "3: DAC output"]
    Dacout = 3,
}
impl From<Exmode> for u8 {
    #[inline(always)]
    fn from(variant: Exmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Exmode {
    type Ux = u8;
}
impl crate::IsEnum for Exmode {}
#[doc = "Field `EXMODE` reader - Set GPIO mode"]
pub type ExmodeR = crate::FieldReader<Exmode>;
impl ExmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exmode {
        match self.bits {
            0 => Exmode::Disable,
            1 => Exmode::High,
            2 => Exmode::Low,
            3 => Exmode::Dacout,
            _ => unreachable!(),
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Exmode::Disable
    }
    #[doc = "Push Pull, GPIO is driven high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Exmode::High
    }
    #[doc = "Push Pull, GPIO is driven low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Exmode::Low
    }
    #[doc = "DAC output"]
    #[inline(always)]
    pub fn is_dacout(&self) -> bool {
        *self == Exmode::Dacout
    }
}
#[doc = "Field `EXMODE` writer - Set GPIO mode"]
pub type ExmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Exmode, crate::Safe>;
impl<'a, REG> ExmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Exmode::Disable)
    }
    #[doc = "Push Pull, GPIO is driven high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Exmode::High)
    }
    #[doc = "Push Pull, GPIO is driven low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Exmode::Low)
    }
    #[doc = "DAC output"]
    #[inline(always)]
    pub fn dacout(self) -> &'a mut crate::W<REG> {
        self.variant(Exmode::Dacout)
    }
}
#[doc = "Field `ALTEX` reader - Use alternative excite pin"]
pub type AltexR = crate::BitReader;
#[doc = "Field `ALTEX` writer - Use alternative excite pin"]
pub type AltexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Select clock used for timing of sample d\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sampleclk {
    #[doc = "0: Prescaled low-frequency LESENSECLK will be used for timing"]
    Lfaclk = 0,
    #[doc = "1: Prescaled high-frequency LESENSEHFCLK will be used for timing"]
    Auxhfrco = 1,
}
impl From<Sampleclk> for bool {
    #[inline(always)]
    fn from(variant: Sampleclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPLECLK` reader - Select clock used for timing of sample d"]
pub type SampleclkR = crate::BitReader<Sampleclk>;
impl SampleclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sampleclk {
        match self.bits {
            false => Sampleclk::Lfaclk,
            true => Sampleclk::Auxhfrco,
        }
    }
    #[doc = "Prescaled low-frequency LESENSECLK will be used for timing"]
    #[inline(always)]
    pub fn is_lfaclk(&self) -> bool {
        *self == Sampleclk::Lfaclk
    }
    #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == Sampleclk::Auxhfrco
    }
}
#[doc = "Field `SAMPLECLK` writer - Select clock used for timing of sample d"]
pub type SampleclkW<'a, REG> = crate::BitWriter<'a, REG, Sampleclk>;
impl<'a, REG> SampleclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Prescaled low-frequency LESENSECLK will be used for timing"]
    #[inline(always)]
    pub fn lfaclk(self) -> &'a mut crate::W<REG> {
        self.variant(Sampleclk::Lfaclk)
    }
    #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Sampleclk::Auxhfrco)
    }
}
#[doc = "Select clock used for excitation timing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exclk {
    #[doc = "0: Prescaled low-frequency LESENSECLK will be used for timing"]
    Lfaclk = 0,
    #[doc = "1: Prescaled high-frequency LESENSEHFCLK will be used for timing"]
    Auxhfrco = 1,
}
impl From<Exclk> for bool {
    #[inline(always)]
    fn from(variant: Exclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXCLK` reader - Select clock used for excitation timing"]
pub type ExclkR = crate::BitReader<Exclk>;
impl ExclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exclk {
        match self.bits {
            false => Exclk::Lfaclk,
            true => Exclk::Auxhfrco,
        }
    }
    #[doc = "Prescaled low-frequency LESENSECLK will be used for timing"]
    #[inline(always)]
    pub fn is_lfaclk(&self) -> bool {
        *self == Exclk::Lfaclk
    }
    #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == Exclk::Auxhfrco
    }
}
#[doc = "Field `EXCLK` writer - Select clock used for excitation timing"]
pub type ExclkW<'a, REG> = crate::BitWriter<'a, REG, Exclk>;
impl<'a, REG> ExclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Prescaled low-frequency LESENSECLK will be used for timing"]
    #[inline(always)]
    pub fn lfaclk(self) -> &'a mut crate::W<REG> {
        self.variant(Exclk::Lfaclk)
    }
    #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Exclk::Auxhfrco)
    }
}
#[doc = "Enable interrupt generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Setif {
    #[doc = "0: No interrupt is generated"]
    None = 0,
    #[doc = "1: Set interrupt flag if the sensor triggers."]
    Level = 1,
    #[doc = "2: Set interrupt flag on positive edge of the sensor state"]
    Posedge = 2,
    #[doc = "3: Set interrupt flag on negative edge of the sensor state"]
    Negedge = 3,
    #[doc = "4: Set interrupt flag on both edges of the sensor state"]
    Bothedges = 4,
}
impl From<Setif> for u8 {
    #[inline(always)]
    fn from(variant: Setif) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Setif {
    type Ux = u8;
}
impl crate::IsEnum for Setif {}
#[doc = "Field `SETIF` reader - Enable interrupt generation"]
pub type SetifR = crate::FieldReader<Setif>;
impl SetifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Setif> {
        match self.bits {
            0 => Some(Setif::None),
            1 => Some(Setif::Level),
            2 => Some(Setif::Posedge),
            3 => Some(Setif::Negedge),
            4 => Some(Setif::Bothedges),
            _ => None,
        }
    }
    #[doc = "No interrupt is generated"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Setif::None
    }
    #[doc = "Set interrupt flag if the sensor triggers."]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Setif::Level
    }
    #[doc = "Set interrupt flag on positive edge of the sensor state"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        *self == Setif::Posedge
    }
    #[doc = "Set interrupt flag on negative edge of the sensor state"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == Setif::Negedge
    }
    #[doc = "Set interrupt flag on both edges of the sensor state"]
    #[inline(always)]
    pub fn is_bothedges(&self) -> bool {
        *self == Setif::Bothedges
    }
}
#[doc = "Field `SETIF` writer - Enable interrupt generation"]
pub type SetifW<'a, REG> = crate::FieldWriter<'a, REG, 3, Setif>;
impl<'a, REG> SetifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No interrupt is generated"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Setif::None)
    }
    #[doc = "Set interrupt flag if the sensor triggers."]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Setif::Level)
    }
    #[doc = "Set interrupt flag on positive edge of the sensor state"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut crate::W<REG> {
        self.variant(Setif::Posedge)
    }
    #[doc = "Set interrupt flag on negative edge of the sensor state"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut crate::W<REG> {
        self.variant(Setif::Negedge)
    }
    #[doc = "Set interrupt flag on both edges of the sensor state"]
    #[inline(always)]
    pub fn bothedges(self) -> &'a mut crate::W<REG> {
        self.variant(Setif::Bothedges)
    }
}
#[doc = "Field `OFFSET` reader - OFFSET for IADC/ACMP interaction"]
pub type OffsetR = crate::FieldReader;
#[doc = "Field `OFFSET` writer - OFFSET for IADC/ACMP interaction"]
pub type OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Sample mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sample {
    #[doc = "0: ACMPCOUNT"]
    Acmpcount = 0,
    #[doc = "1: ACMP"]
    Acmp = 1,
    #[doc = "2: ADC"]
    Adc = 2,
    #[doc = "3: ADCDIFF"]
    Adcdiff = 3,
}
impl From<Sample> for u8 {
    #[inline(always)]
    fn from(variant: Sample) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sample {
    type Ux = u8;
}
impl crate::IsEnum for Sample {}
#[doc = "Field `SAMPLE` reader - Sample mode Selection"]
pub type SampleR = crate::FieldReader<Sample>;
impl SampleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sample {
        match self.bits {
            0 => Sample::Acmpcount,
            1 => Sample::Acmp,
            2 => Sample::Adc,
            3 => Sample::Adcdiff,
            _ => unreachable!(),
        }
    }
    #[doc = "ACMPCOUNT"]
    #[inline(always)]
    pub fn is_acmpcount(&self) -> bool {
        *self == Sample::Acmpcount
    }
    #[doc = "ACMP"]
    #[inline(always)]
    pub fn is_acmp(&self) -> bool {
        *self == Sample::Acmp
    }
    #[doc = "ADC"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == Sample::Adc
    }
    #[doc = "ADCDIFF"]
    #[inline(always)]
    pub fn is_adcdiff(&self) -> bool {
        *self == Sample::Adcdiff
    }
}
#[doc = "Field `SAMPLE` writer - Sample mode Selection"]
pub type SampleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sample, crate::Safe>;
impl<'a, REG> SampleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ACMPCOUNT"]
    #[inline(always)]
    pub fn acmpcount(self) -> &'a mut crate::W<REG> {
        self.variant(Sample::Acmpcount)
    }
    #[doc = "ACMP"]
    #[inline(always)]
    pub fn acmp(self) -> &'a mut crate::W<REG> {
        self.variant(Sample::Acmp)
    }
    #[doc = "ADC"]
    #[inline(always)]
    pub fn adc(self) -> &'a mut crate::W<REG> {
        self.variant(Sample::Adc)
    }
    #[doc = "ADCDIFF"]
    #[inline(always)]
    pub fn adcdiff(self) -> &'a mut crate::W<REG> {
        self.variant(Sample::Adcdiff)
    }
}
impl R {
    #[doc = "Bits 0:11 - ACMP threshold or DAC data"]
    #[inline(always)]
    pub fn thres(&self) -> ThresR {
        ThresR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:17 - Set GPIO mode"]
    #[inline(always)]
    pub fn exmode(&self) -> ExmodeR {
        ExmodeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Use alternative excite pin"]
    #[inline(always)]
    pub fn altex(&self) -> AltexR {
        AltexR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Select clock used for timing of sample d"]
    #[inline(always)]
    pub fn sampleclk(&self) -> SampleclkR {
        SampleclkR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Select clock used for excitation timing"]
    #[inline(always)]
    pub fn exclk(&self) -> ExclkR {
        ExclkR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - Enable interrupt generation"]
    #[inline(always)]
    pub fn setif(&self) -> SetifR {
        SetifR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:27 - OFFSET for IADC/ACMP interaction"]
    #[inline(always)]
    pub fn offset(&self) -> OffsetR {
        OffsetR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Sample mode Selection"]
    #[inline(always)]
    pub fn sample(&self) -> SampleR {
        SampleR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - ACMP threshold or DAC data"]
    #[inline(always)]
    pub fn thres(&mut self) -> ThresW<'_, Ch9InteractSpec> {
        ThresW::new(self, 0)
    }
    #[doc = "Bits 16:17 - Set GPIO mode"]
    #[inline(always)]
    pub fn exmode(&mut self) -> ExmodeW<'_, Ch9InteractSpec> {
        ExmodeW::new(self, 16)
    }
    #[doc = "Bit 18 - Use alternative excite pin"]
    #[inline(always)]
    pub fn altex(&mut self) -> AltexW<'_, Ch9InteractSpec> {
        AltexW::new(self, 18)
    }
    #[doc = "Bit 19 - Select clock used for timing of sample d"]
    #[inline(always)]
    pub fn sampleclk(&mut self) -> SampleclkW<'_, Ch9InteractSpec> {
        SampleclkW::new(self, 19)
    }
    #[doc = "Bit 20 - Select clock used for excitation timing"]
    #[inline(always)]
    pub fn exclk(&mut self) -> ExclkW<'_, Ch9InteractSpec> {
        ExclkW::new(self, 20)
    }
    #[doc = "Bits 21:23 - Enable interrupt generation"]
    #[inline(always)]
    pub fn setif(&mut self) -> SetifW<'_, Ch9InteractSpec> {
        SetifW::new(self, 21)
    }
    #[doc = "Bits 24:27 - OFFSET for IADC/ACMP interaction"]
    #[inline(always)]
    pub fn offset(&mut self) -> OffsetW<'_, Ch9InteractSpec> {
        OffsetW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Sample mode Selection"]
    #[inline(always)]
    pub fn sample(&mut self) -> SampleW<'_, Ch9InteractSpec> {
        SampleW::new(self, 28)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_interact::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_interact::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch9InteractSpec;
impl crate::RegisterSpec for Ch9InteractSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch9_interact::R`](R) reader structure"]
impl crate::Readable for Ch9InteractSpec {}
#[doc = "`write(|w| ..)` method takes [`ch9_interact::W`](W) writer structure"]
impl crate::Writable for Ch9InteractSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH9_INTERACT to value 0"]
impl crate::Resettable for Ch9InteractSpec {}
