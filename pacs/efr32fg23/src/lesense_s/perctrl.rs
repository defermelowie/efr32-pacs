#[doc = "Register `PERCTRL` reader"]
pub type R = crate::R<PerctrlSpec>;
#[doc = "Register `PERCTRL` writer"]
pub type W = crate::W<PerctrlSpec>;
#[doc = "DAC CH0 data selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dacch0data {
    #[doc = "0: DAC data is defined by CH0DATA in the DAC interface."]
    Dacdata = 0,
    #[doc = "1: DAC data is defined by THRES in CHx_INTERACT."]
    Thres = 1,
}
impl From<Dacch0data> for bool {
    #[inline(always)]
    fn from(variant: Dacch0data) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACCH0DATA` reader - DAC CH0 data selection."]
pub type Dacch0dataR = crate::BitReader<Dacch0data>;
impl Dacch0dataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dacch0data {
        match self.bits {
            false => Dacch0data::Dacdata,
            true => Dacch0data::Thres,
        }
    }
    #[doc = "DAC data is defined by CH0DATA in the DAC interface."]
    #[inline(always)]
    pub fn is_dacdata(&self) -> bool {
        *self == Dacch0data::Dacdata
    }
    #[doc = "DAC data is defined by THRES in CHx_INTERACT."]
    #[inline(always)]
    pub fn is_thres(&self) -> bool {
        *self == Dacch0data::Thres
    }
}
#[doc = "Field `DACCH0DATA` writer - DAC CH0 data selection."]
pub type Dacch0dataW<'a, REG> = crate::BitWriter<'a, REG, Dacch0data>;
impl<'a, REG> Dacch0dataW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC data is defined by CH0DATA in the DAC interface."]
    #[inline(always)]
    pub fn dacdata(self) -> &'a mut crate::W<REG> {
        self.variant(Dacch0data::Dacdata)
    }
    #[doc = "DAC data is defined by THRES in CHx_INTERACT."]
    #[inline(always)]
    pub fn thres(self) -> &'a mut crate::W<REG> {
        self.variant(Dacch0data::Thres)
    }
}
#[doc = "DAC startup configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dacstartup {
    #[doc = "0: DAC is started a full LESENSECLK before sensor interaction starts."]
    Fullcycle = 0,
    #[doc = "1: DAC is started half a LESENSECLK cycle before sensor interaction starts."]
    Halfcycle = 1,
}
impl From<Dacstartup> for bool {
    #[inline(always)]
    fn from(variant: Dacstartup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACSTARTUP` reader - DAC startup configuration"]
pub type DacstartupR = crate::BitReader<Dacstartup>;
impl DacstartupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dacstartup {
        match self.bits {
            false => Dacstartup::Fullcycle,
            true => Dacstartup::Halfcycle,
        }
    }
    #[doc = "DAC is started a full LESENSECLK before sensor interaction starts."]
    #[inline(always)]
    pub fn is_fullcycle(&self) -> bool {
        *self == Dacstartup::Fullcycle
    }
    #[doc = "DAC is started half a LESENSECLK cycle before sensor interaction starts."]
    #[inline(always)]
    pub fn is_halfcycle(&self) -> bool {
        *self == Dacstartup::Halfcycle
    }
}
#[doc = "Field `DACSTARTUP` writer - DAC startup configuration"]
pub type DacstartupW<'a, REG> = crate::BitWriter<'a, REG, Dacstartup>;
impl<'a, REG> DacstartupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC is started a full LESENSECLK before sensor interaction starts."]
    #[inline(always)]
    pub fn fullcycle(self) -> &'a mut crate::W<REG> {
        self.variant(Dacstartup::Fullcycle)
    }
    #[doc = "DAC is started half a LESENSECLK cycle before sensor interaction starts."]
    #[inline(always)]
    pub fn halfcycle(self) -> &'a mut crate::W<REG> {
        self.variant(Dacstartup::Halfcycle)
    }
}
#[doc = "DAC conversion trigger configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dacconvtrig {
    #[doc = "0: DAC is enabled before every LESENSE channle measurement."]
    Channelstart = 0,
    #[doc = "1: DAC is only enabled once per scan."]
    Scanstart = 1,
}
impl From<Dacconvtrig> for bool {
    #[inline(always)]
    fn from(variant: Dacconvtrig) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACCONVTRIG` reader - DAC conversion trigger configuration"]
pub type DacconvtrigR = crate::BitReader<Dacconvtrig>;
impl DacconvtrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dacconvtrig {
        match self.bits {
            false => Dacconvtrig::Channelstart,
            true => Dacconvtrig::Scanstart,
        }
    }
    #[doc = "DAC is enabled before every LESENSE channle measurement."]
    #[inline(always)]
    pub fn is_channelstart(&self) -> bool {
        *self == Dacconvtrig::Channelstart
    }
    #[doc = "DAC is only enabled once per scan."]
    #[inline(always)]
    pub fn is_scanstart(&self) -> bool {
        *self == Dacconvtrig::Scanstart
    }
}
#[doc = "Field `DACCONVTRIG` writer - DAC conversion trigger configuration"]
pub type DacconvtrigW<'a, REG> = crate::BitWriter<'a, REG, Dacconvtrig>;
impl<'a, REG> DacconvtrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC is enabled before every LESENSE channle measurement."]
    #[inline(always)]
    pub fn channelstart(self) -> &'a mut crate::W<REG> {
        self.variant(Dacconvtrig::Channelstart)
    }
    #[doc = "DAC is only enabled once per scan."]
    #[inline(always)]
    pub fn scanstart(self) -> &'a mut crate::W<REG> {
        self.variant(Dacconvtrig::Scanstart)
    }
}
#[doc = "ACMP0 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmp0mode {
    #[doc = "0: LESENSE controls POSSEL of ACMP0"]
    Mux = 0,
    #[doc = "1: LESENSE controls POSSEL and reference divider of ACMP0"]
    Muxthres = 1,
}
impl From<Acmp0mode> for bool {
    #[inline(always)]
    fn from(variant: Acmp0mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP0MODE` reader - ACMP0 mode"]
pub type Acmp0modeR = crate::BitReader<Acmp0mode>;
impl Acmp0modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acmp0mode {
        match self.bits {
            false => Acmp0mode::Mux,
            true => Acmp0mode::Muxthres,
        }
    }
    #[doc = "LESENSE controls POSSEL of ACMP0"]
    #[inline(always)]
    pub fn is_mux(&self) -> bool {
        *self == Acmp0mode::Mux
    }
    #[doc = "LESENSE controls POSSEL and reference divider of ACMP0"]
    #[inline(always)]
    pub fn is_muxthres(&self) -> bool {
        *self == Acmp0mode::Muxthres
    }
}
#[doc = "Field `ACMP0MODE` writer - ACMP0 mode"]
pub type Acmp0modeW<'a, REG> = crate::BitWriter<'a, REG, Acmp0mode>;
impl<'a, REG> Acmp0modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LESENSE controls POSSEL of ACMP0"]
    #[inline(always)]
    pub fn mux(self) -> &'a mut crate::W<REG> {
        self.variant(Acmp0mode::Mux)
    }
    #[doc = "LESENSE controls POSSEL and reference divider of ACMP0"]
    #[inline(always)]
    pub fn muxthres(self) -> &'a mut crate::W<REG> {
        self.variant(Acmp0mode::Muxthres)
    }
}
#[doc = "ACMP1 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmp1mode {
    #[doc = "0: LESENSE controls the POSSEL of ACMP1"]
    Mux = 0,
    #[doc = "1: LESENSE POSSEL and reference divider of ACMP1"]
    Muxthres = 1,
}
impl From<Acmp1mode> for bool {
    #[inline(always)]
    fn from(variant: Acmp1mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP1MODE` reader - ACMP1 mode"]
pub type Acmp1modeR = crate::BitReader<Acmp1mode>;
impl Acmp1modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acmp1mode {
        match self.bits {
            false => Acmp1mode::Mux,
            true => Acmp1mode::Muxthres,
        }
    }
    #[doc = "LESENSE controls the POSSEL of ACMP1"]
    #[inline(always)]
    pub fn is_mux(&self) -> bool {
        *self == Acmp1mode::Mux
    }
    #[doc = "LESENSE POSSEL and reference divider of ACMP1"]
    #[inline(always)]
    pub fn is_muxthres(&self) -> bool {
        *self == Acmp1mode::Muxthres
    }
}
#[doc = "Field `ACMP1MODE` writer - ACMP1 mode"]
pub type Acmp1modeW<'a, REG> = crate::BitWriter<'a, REG, Acmp1mode>;
impl<'a, REG> Acmp1modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LESENSE controls the POSSEL of ACMP1"]
    #[inline(always)]
    pub fn mux(self) -> &'a mut crate::W<REG> {
        self.variant(Acmp1mode::Mux)
    }
    #[doc = "LESENSE POSSEL and reference divider of ACMP1"]
    #[inline(always)]
    pub fn muxthres(self) -> &'a mut crate::W<REG> {
        self.variant(Acmp1mode::Muxthres)
    }
}
#[doc = "Field `ACMP0INV` reader - Invert analog comparator 0 output"]
pub type Acmp0invR = crate::BitReader;
#[doc = "Field `ACMP0INV` writer - Invert analog comparator 0 output"]
pub type Acmp0invW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP1INV` reader - Invert analog comparator 1 output"]
pub type Acmp1invR = crate::BitReader;
#[doc = "Field `ACMP1INV` writer - Invert analog comparator 1 output"]
pub type Acmp1invW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - DAC CH0 data selection."]
    #[inline(always)]
    pub fn dacch0data(&self) -> Dacch0dataR {
        Dacch0dataR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - DAC startup configuration"]
    #[inline(always)]
    pub fn dacstartup(&self) -> DacstartupR {
        DacstartupR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - DAC conversion trigger configuration"]
    #[inline(always)]
    pub fn dacconvtrig(&self) -> DacconvtrigR {
        DacconvtrigR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 20 - ACMP0 mode"]
    #[inline(always)]
    pub fn acmp0mode(&self) -> Acmp0modeR {
        Acmp0modeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - ACMP1 mode"]
    #[inline(always)]
    pub fn acmp1mode(&self) -> Acmp1modeR {
        Acmp1modeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Invert analog comparator 0 output"]
    #[inline(always)]
    pub fn acmp0inv(&self) -> Acmp0invR {
        Acmp0invR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Invert analog comparator 1 output"]
    #[inline(always)]
    pub fn acmp1inv(&self) -> Acmp1invR {
        Acmp1invR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - DAC CH0 data selection."]
    #[inline(always)]
    pub fn dacch0data(&mut self) -> Dacch0dataW<'_, PerctrlSpec> {
        Dacch0dataW::new(self, 2)
    }
    #[doc = "Bit 6 - DAC startup configuration"]
    #[inline(always)]
    pub fn dacstartup(&mut self) -> DacstartupW<'_, PerctrlSpec> {
        DacstartupW::new(self, 6)
    }
    #[doc = "Bit 8 - DAC conversion trigger configuration"]
    #[inline(always)]
    pub fn dacconvtrig(&mut self) -> DacconvtrigW<'_, PerctrlSpec> {
        DacconvtrigW::new(self, 8)
    }
    #[doc = "Bit 20 - ACMP0 mode"]
    #[inline(always)]
    pub fn acmp0mode(&mut self) -> Acmp0modeW<'_, PerctrlSpec> {
        Acmp0modeW::new(self, 20)
    }
    #[doc = "Bit 22 - ACMP1 mode"]
    #[inline(always)]
    pub fn acmp1mode(&mut self) -> Acmp1modeW<'_, PerctrlSpec> {
        Acmp1modeW::new(self, 22)
    }
    #[doc = "Bit 24 - Invert analog comparator 0 output"]
    #[inline(always)]
    pub fn acmp0inv(&mut self) -> Acmp0invW<'_, PerctrlSpec> {
        Acmp0invW::new(self, 24)
    }
    #[doc = "Bit 25 - Invert analog comparator 1 output"]
    #[inline(always)]
    pub fn acmp1inv(&mut self) -> Acmp1invW<'_, PerctrlSpec> {
        Acmp1invW::new(self, 25)
    }
}
#[doc = "Peripheral Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`perctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerctrlSpec;
impl crate::RegisterSpec for PerctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perctrl::R`](R) reader structure"]
impl crate::Readable for PerctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`perctrl::W`](W) writer structure"]
impl crate::Writable for PerctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERCTRL to value 0"]
impl crate::Resettable for PerctrlSpec {}
