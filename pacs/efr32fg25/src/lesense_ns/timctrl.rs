#[doc = "Register `TIMCTRL` reader"]
pub type R = crate::R<TimctrlSpec>;
#[doc = "Register `TIMCTRL` writer"]
pub type W = crate::W<TimctrlSpec>;
#[doc = "Prescaling factor for high frequency tim\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Auxpresc {
    #[doc = "0: High frequency timer is clocked at LESENSEHFCLK/1"]
    Div1 = 0,
    #[doc = "1: High frequency timer is clocked at LESENSEHFCLK/2"]
    Div2 = 1,
    #[doc = "2: High frequency timer is clocked at LESENSEHFCLK/4"]
    Div4 = 2,
    #[doc = "3: High frequency timer is clocked at LESENSEHFCLK/8"]
    Div8 = 3,
}
impl From<Auxpresc> for u8 {
    #[inline(always)]
    fn from(variant: Auxpresc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Auxpresc {
    type Ux = u8;
}
impl crate::IsEnum for Auxpresc {}
#[doc = "Field `AUXPRESC` reader - Prescaling factor for high frequency tim"]
pub type AuxprescR = crate::FieldReader<Auxpresc>;
impl AuxprescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Auxpresc {
        match self.bits {
            0 => Auxpresc::Div1,
            1 => Auxpresc::Div2,
            2 => Auxpresc::Div4,
            3 => Auxpresc::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "High frequency timer is clocked at LESENSEHFCLK/1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Auxpresc::Div1
    }
    #[doc = "High frequency timer is clocked at LESENSEHFCLK/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Auxpresc::Div2
    }
    #[doc = "High frequency timer is clocked at LESENSEHFCLK/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Auxpresc::Div4
    }
    #[doc = "High frequency timer is clocked at LESENSEHFCLK/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Auxpresc::Div8
    }
}
#[doc = "Field `AUXPRESC` writer - Prescaling factor for high frequency tim"]
pub type AuxprescW<'a, REG> = crate::FieldWriter<'a, REG, 2, Auxpresc, crate::Safe>;
impl<'a, REG> AuxprescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High frequency timer is clocked at LESENSEHFCLK/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Auxpresc::Div1)
    }
    #[doc = "High frequency timer is clocked at LESENSEHFCLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Auxpresc::Div2)
    }
    #[doc = "High frequency timer is clocked at LESENSEHFCLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Auxpresc::Div4)
    }
    #[doc = "High frequency timer is clocked at LESENSEHFCLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Auxpresc::Div8)
    }
}
#[doc = "Prescaling factor for low frequency time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lfpresc {
    #[doc = "0: Low frequency timer is clocked with LESENSECLK/1"]
    Div1 = 0,
    #[doc = "1: Low frequency timer is clocked with LESENSECLK/2"]
    Div2 = 1,
    #[doc = "2: Low frequency timer is clocked with LESENSECLK/4"]
    Div4 = 2,
    #[doc = "3: Low frequency timer is clocked with LESENSECLK/8"]
    Div8 = 3,
    #[doc = "4: Low frequency timer is clocked with LESENSECLK/16"]
    Div16 = 4,
    #[doc = "5: Low frequency timer is clocked with LESENSECLK/32"]
    Div32 = 5,
    #[doc = "6: Low frequency timer is clocked with LESENSECLK/64"]
    Div64 = 6,
    #[doc = "7: Low frequency timer is clocked with LESENSECLK/128"]
    Div128 = 7,
}
impl From<Lfpresc> for u8 {
    #[inline(always)]
    fn from(variant: Lfpresc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lfpresc {
    type Ux = u8;
}
impl crate::IsEnum for Lfpresc {}
#[doc = "Field `LFPRESC` reader - Prescaling factor for low frequency time"]
pub type LfprescR = crate::FieldReader<Lfpresc>;
impl LfprescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfpresc {
        match self.bits {
            0 => Lfpresc::Div1,
            1 => Lfpresc::Div2,
            2 => Lfpresc::Div4,
            3 => Lfpresc::Div8,
            4 => Lfpresc::Div16,
            5 => Lfpresc::Div32,
            6 => Lfpresc::Div64,
            7 => Lfpresc::Div128,
            _ => unreachable!(),
        }
    }
    #[doc = "Low frequency timer is clocked with LESENSECLK/1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Lfpresc::Div1
    }
    #[doc = "Low frequency timer is clocked with LESENSECLK/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Lfpresc::Div2
    }
    #[doc = "Low frequency timer is clocked with LESENSECLK/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Lfpresc::Div4
    }
    #[doc = "Low frequency timer is clocked with LESENSECLK/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Lfpresc::Div8
    }
    #[doc = "Low frequency timer is clocked with LESENSECLK/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Lfpresc::Div16
    }
    #[doc = "Low frequency timer is clocked with LESENSECLK/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Lfpresc::Div32
    }
    #[doc = "Low frequency timer is clocked with LESENSECLK/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Lfpresc::Div64
    }
    #[doc = "Low frequency timer is clocked with LESENSECLK/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Lfpresc::Div128
    }
}
#[doc = "Field `LFPRESC` writer - Prescaling factor for low frequency time"]
pub type LfprescW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lfpresc, crate::Safe>;
impl<'a, REG> LfprescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low frequency timer is clocked with LESENSECLK/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Lfpresc::Div1)
    }
    #[doc = "Low frequency timer is clocked with LESENSECLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Lfpresc::Div2)
    }
    #[doc = "Low frequency timer is clocked with LESENSECLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Lfpresc::Div4)
    }
    #[doc = "Low frequency timer is clocked with LESENSECLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Lfpresc::Div8)
    }
    #[doc = "Low frequency timer is clocked with LESENSECLK/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Lfpresc::Div16)
    }
    #[doc = "Low frequency timer is clocked with LESENSECLK/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Lfpresc::Div32)
    }
    #[doc = "Low frequency timer is clocked with LESENSECLK/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Lfpresc::Div64)
    }
    #[doc = "Low frequency timer is clocked with LESENSECLK/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Lfpresc::Div128)
    }
}
#[doc = "Period counter prescaling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pcpresc {
    #[doc = "0: The period counter clock frequency is LESENSECLK/1"]
    Div1 = 0,
    #[doc = "1: The period counter clock frequency is LESENSECLK/2"]
    Div2 = 1,
    #[doc = "2: The period counter clock frequency is LESENSECLK/4"]
    Div4 = 2,
    #[doc = "3: The period counter clock frequency is LESENSECLK/8"]
    Div8 = 3,
    #[doc = "4: The period counter clock frequency is LESENSECLK/16"]
    Div16 = 4,
    #[doc = "5: The period counter clock frequency is LESENSECLK/32"]
    Div32 = 5,
    #[doc = "6: The period counter clock frequency is LESENSECLK/64"]
    Div64 = 6,
    #[doc = "7: The period counter clock frequency is LESENSECLK/128"]
    Div128 = 7,
}
impl From<Pcpresc> for u8 {
    #[inline(always)]
    fn from(variant: Pcpresc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pcpresc {
    type Ux = u8;
}
impl crate::IsEnum for Pcpresc {}
#[doc = "Field `PCPRESC` reader - Period counter prescaling"]
pub type PcprescR = crate::FieldReader<Pcpresc>;
impl PcprescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcpresc {
        match self.bits {
            0 => Pcpresc::Div1,
            1 => Pcpresc::Div2,
            2 => Pcpresc::Div4,
            3 => Pcpresc::Div8,
            4 => Pcpresc::Div16,
            5 => Pcpresc::Div32,
            6 => Pcpresc::Div64,
            7 => Pcpresc::Div128,
            _ => unreachable!(),
        }
    }
    #[doc = "The period counter clock frequency is LESENSECLK/1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Pcpresc::Div1
    }
    #[doc = "The period counter clock frequency is LESENSECLK/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Pcpresc::Div2
    }
    #[doc = "The period counter clock frequency is LESENSECLK/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Pcpresc::Div4
    }
    #[doc = "The period counter clock frequency is LESENSECLK/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Pcpresc::Div8
    }
    #[doc = "The period counter clock frequency is LESENSECLK/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Pcpresc::Div16
    }
    #[doc = "The period counter clock frequency is LESENSECLK/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Pcpresc::Div32
    }
    #[doc = "The period counter clock frequency is LESENSECLK/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Pcpresc::Div64
    }
    #[doc = "The period counter clock frequency is LESENSECLK/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Pcpresc::Div128
    }
}
#[doc = "Field `PCPRESC` writer - Period counter prescaling"]
pub type PcprescW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pcpresc, crate::Safe>;
impl<'a, REG> PcprescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The period counter clock frequency is LESENSECLK/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Pcpresc::Div1)
    }
    #[doc = "The period counter clock frequency is LESENSECLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Pcpresc::Div2)
    }
    #[doc = "The period counter clock frequency is LESENSECLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Pcpresc::Div4)
    }
    #[doc = "The period counter clock frequency is LESENSECLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Pcpresc::Div8)
    }
    #[doc = "The period counter clock frequency is LESENSECLK/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Pcpresc::Div16)
    }
    #[doc = "The period counter clock frequency is LESENSECLK/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Pcpresc::Div32)
    }
    #[doc = "The period counter clock frequency is LESENSECLK/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Pcpresc::Div64)
    }
    #[doc = "The period counter clock frequency is LESENSECLK/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Pcpresc::Div128)
    }
}
#[doc = "Field `PCTOP` reader - Period counter top value"]
pub type PctopR = crate::FieldReader;
#[doc = "Field `PCTOP` writer - Period counter top value"]
pub type PctopW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `STARTDLY` reader - Start delay configuration"]
pub type StartdlyR = crate::FieldReader;
#[doc = "Field `STARTDLY` writer - Start delay configuration"]
pub type StartdlyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "AUX startup config\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Auxstartup {
    #[doc = "0: Request oscillator .5 LESENSECLK cycle before sensing starts"]
    Predemand = 0,
    #[doc = "1: Request oscillator at sensing time"]
    Ondemand = 1,
}
impl From<Auxstartup> for bool {
    #[inline(always)]
    fn from(variant: Auxstartup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUXSTARTUP` reader - AUX startup config"]
pub type AuxstartupR = crate::BitReader<Auxstartup>;
impl AuxstartupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Auxstartup {
        match self.bits {
            false => Auxstartup::Predemand,
            true => Auxstartup::Ondemand,
        }
    }
    #[doc = "Request oscillator .5 LESENSECLK cycle before sensing starts"]
    #[inline(always)]
    pub fn is_predemand(&self) -> bool {
        *self == Auxstartup::Predemand
    }
    #[doc = "Request oscillator at sensing time"]
    #[inline(always)]
    pub fn is_ondemand(&self) -> bool {
        *self == Auxstartup::Ondemand
    }
}
#[doc = "Field `AUXSTARTUP` writer - AUX startup config"]
pub type AuxstartupW<'a, REG> = crate::BitWriter<'a, REG, Auxstartup>;
impl<'a, REG> AuxstartupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Request oscillator .5 LESENSECLK cycle before sensing starts"]
    #[inline(always)]
    pub fn predemand(self) -> &'a mut crate::W<REG> {
        self.variant(Auxstartup::Predemand)
    }
    #[doc = "Request oscillator at sensing time"]
    #[inline(always)]
    pub fn ondemand(self) -> &'a mut crate::W<REG> {
        self.variant(Auxstartup::Ondemand)
    }
}
impl R {
    #[doc = "Bits 0:1 - Prescaling factor for high frequency tim"]
    #[inline(always)]
    pub fn auxpresc(&self) -> AuxprescR {
        AuxprescR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Prescaling factor for low frequency time"]
    #[inline(always)]
    pub fn lfpresc(&self) -> LfprescR {
        LfprescR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Period counter prescaling"]
    #[inline(always)]
    pub fn pcpresc(&self) -> PcprescR {
        PcprescR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:19 - Period counter top value"]
    #[inline(always)]
    pub fn pctop(&self) -> PctopR {
        PctopR::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 22:23 - Start delay configuration"]
    #[inline(always)]
    pub fn startdly(&self) -> StartdlyR {
        StartdlyR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 28 - AUX startup config"]
    #[inline(always)]
    pub fn auxstartup(&self) -> AuxstartupR {
        AuxstartupR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Prescaling factor for high frequency tim"]
    #[inline(always)]
    pub fn auxpresc(&mut self) -> AuxprescW<'_, TimctrlSpec> {
        AuxprescW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Prescaling factor for low frequency time"]
    #[inline(always)]
    pub fn lfpresc(&mut self) -> LfprescW<'_, TimctrlSpec> {
        LfprescW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Period counter prescaling"]
    #[inline(always)]
    pub fn pcpresc(&mut self) -> PcprescW<'_, TimctrlSpec> {
        PcprescW::new(self, 8)
    }
    #[doc = "Bits 12:19 - Period counter top value"]
    #[inline(always)]
    pub fn pctop(&mut self) -> PctopW<'_, TimctrlSpec> {
        PctopW::new(self, 12)
    }
    #[doc = "Bits 22:23 - Start delay configuration"]
    #[inline(always)]
    pub fn startdly(&mut self) -> StartdlyW<'_, TimctrlSpec> {
        StartdlyW::new(self, 22)
    }
    #[doc = "Bit 28 - AUX startup config"]
    #[inline(always)]
    pub fn auxstartup(&mut self) -> AuxstartupW<'_, TimctrlSpec> {
        AuxstartupW::new(self, 28)
    }
}
#[doc = "Timing Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimctrlSpec;
impl crate::RegisterSpec for TimctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timctrl::R`](R) reader structure"]
impl crate::Readable for TimctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`timctrl::W`](W) writer structure"]
impl crate::Writable for TimctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMCTRL to value 0"]
impl crate::Resettable for TimctrlSpec {}
