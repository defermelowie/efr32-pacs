#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Configure scan mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Scanmode {
    #[doc = "0: A new scan is started each time the period counter overflows"]
    Periodic = 0,
    #[doc = "1: A single scan is performed when START in CMD is set"]
    Oneshot = 1,
    #[doc = "2: Pulse on PRS channel"]
    Prs = 2,
}
impl From<Scanmode> for u8 {
    #[inline(always)]
    fn from(variant: Scanmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Scanmode {
    type Ux = u8;
}
impl crate::IsEnum for Scanmode {}
#[doc = "Field `SCANMODE` reader - Configure scan mode"]
pub type ScanmodeR = crate::FieldReader<Scanmode>;
impl ScanmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Scanmode> {
        match self.bits {
            0 => Some(Scanmode::Periodic),
            1 => Some(Scanmode::Oneshot),
            2 => Some(Scanmode::Prs),
            _ => None,
        }
    }
    #[doc = "A new scan is started each time the period counter overflows"]
    #[inline(always)]
    pub fn is_periodic(&self) -> bool {
        *self == Scanmode::Periodic
    }
    #[doc = "A single scan is performed when START in CMD is set"]
    #[inline(always)]
    pub fn is_oneshot(&self) -> bool {
        *self == Scanmode::Oneshot
    }
    #[doc = "Pulse on PRS channel"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == Scanmode::Prs
    }
}
#[doc = "Field `SCANMODE` writer - Configure scan mode"]
pub type ScanmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Scanmode>;
impl<'a, REG> ScanmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A new scan is started each time the period counter overflows"]
    #[inline(always)]
    pub fn periodic(self) -> &'a mut crate::W<REG> {
        self.variant(Scanmode::Periodic)
    }
    #[doc = "A single scan is performed when START in CMD is set"]
    #[inline(always)]
    pub fn oneshot(self) -> &'a mut crate::W<REG> {
        self.variant(Scanmode::Oneshot)
    }
    #[doc = "Pulse on PRS channel"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut crate::W<REG> {
        self.variant(Scanmode::Prs)
    }
}
#[doc = "Select scan configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Scanconf {
    #[doc = "0: The channel configuration register registers used are directly mapped to the channel number."]
    Dirmap = 0,
    #[doc = "1: The channel configuration registers used are CH\\[X+8\\]_CONF for channels 0-7 and CH\\[X-8\\]_CONF for channels 8-15."]
    Invmap = 1,
    #[doc = "2: The channel configuration registers used toggle between CH\\[X\\]_CONF and CH\\[X+8\\]_CONF when channel x triggers"]
    Toggle = 2,
    #[doc = "3: The decoder state defines the CONF registers to be used."]
    Decdef = 3,
}
impl From<Scanconf> for u8 {
    #[inline(always)]
    fn from(variant: Scanconf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Scanconf {
    type Ux = u8;
}
impl crate::IsEnum for Scanconf {}
#[doc = "Field `SCANCONF` reader - Select scan configuration"]
pub type ScanconfR = crate::FieldReader<Scanconf>;
impl ScanconfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scanconf {
        match self.bits {
            0 => Scanconf::Dirmap,
            1 => Scanconf::Invmap,
            2 => Scanconf::Toggle,
            3 => Scanconf::Decdef,
            _ => unreachable!(),
        }
    }
    #[doc = "The channel configuration register registers used are directly mapped to the channel number."]
    #[inline(always)]
    pub fn is_dirmap(&self) -> bool {
        *self == Scanconf::Dirmap
    }
    #[doc = "The channel configuration registers used are CH\\[X+8\\]_CONF for channels 0-7 and CH\\[X-8\\]_CONF for channels 8-15."]
    #[inline(always)]
    pub fn is_invmap(&self) -> bool {
        *self == Scanconf::Invmap
    }
    #[doc = "The channel configuration registers used toggle between CH\\[X\\]_CONF and CH\\[X+8\\]_CONF when channel x triggers"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Scanconf::Toggle
    }
    #[doc = "The decoder state defines the CONF registers to be used."]
    #[inline(always)]
    pub fn is_decdef(&self) -> bool {
        *self == Scanconf::Decdef
    }
}
#[doc = "Field `SCANCONF` writer - Select scan configuration"]
pub type ScanconfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Scanconf, crate::Safe>;
impl<'a, REG> ScanconfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The channel configuration register registers used are directly mapped to the channel number."]
    #[inline(always)]
    pub fn dirmap(self) -> &'a mut crate::W<REG> {
        self.variant(Scanconf::Dirmap)
    }
    #[doc = "The channel configuration registers used are CH\\[X+8\\]_CONF for channels 0-7 and CH\\[X-8\\]_CONF for channels 8-15."]
    #[inline(always)]
    pub fn invmap(self) -> &'a mut crate::W<REG> {
        self.variant(Scanconf::Invmap)
    }
    #[doc = "The channel configuration registers used toggle between CH\\[X\\]_CONF and CH\\[X+8\\]_CONF when channel x triggers"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Scanconf::Toggle)
    }
    #[doc = "The decoder state defines the CONF registers to be used."]
    #[inline(always)]
    pub fn decdef(self) -> &'a mut crate::W<REG> {
        self.variant(Scanconf::Decdef)
    }
}
#[doc = "Field `DUALSAMPLE` reader - Enable dual sample mode"]
pub type DualsampleR = crate::BitReader;
#[doc = "Field `DUALSAMPLE` writer - Enable dual sample mode"]
pub type DualsampleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRSCANRES` reader - Enable storing of SCANRES"]
pub type StrscanresR = crate::BitReader;
#[doc = "Field `STRSCANRES` writer - Enable storing of SCANRES"]
pub type StrscanresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DMA wake-up from EM2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmawu {
    #[doc = "0: No DMA wake-up from EM2"]
    Disable = 0,
    #[doc = "1: DMA wake-up from EM2 when FIFO count is greater or equal to RESFIDL"]
    Enable = 1,
}
impl From<Dmawu> for bool {
    #[inline(always)]
    fn from(variant: Dmawu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAWU` reader - DMA wake-up from EM2"]
pub type DmawuR = crate::BitReader<Dmawu>;
impl DmawuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmawu {
        match self.bits {
            false => Dmawu::Disable,
            true => Dmawu::Enable,
        }
    }
    #[doc = "No DMA wake-up from EM2"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dmawu::Disable
    }
    #[doc = "DMA wake-up from EM2 when FIFO count is greater or equal to RESFIDL"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dmawu::Enable
    }
}
#[doc = "Field `DMAWU` writer - DMA wake-up from EM2"]
pub type DmawuW<'a, REG> = crate::BitWriter<'a, REG, Dmawu>;
impl<'a, REG> DmawuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No DMA wake-up from EM2"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmawu::Disable)
    }
    #[doc = "DMA wake-up from EM2 when FIFO count is greater or equal to RESFIDL"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmawu::Enable)
    }
}
#[doc = "Field `RESFIDL` reader - Result FIFO level"]
pub type ResfidlR = crate::FieldReader;
#[doc = "Field `RESFIDL` writer - Result FIFO level"]
pub type ResfidlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Debug Mode Run Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Debugrun {
    #[doc = "0: LESENSE can not start new scans in debug mode"]
    X0 = 0,
    #[doc = "1: LESENSE can start new scans in debug mode"]
    X1 = 1,
}
impl From<Debugrun> for bool {
    #[inline(always)]
    fn from(variant: Debugrun) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DebugrunR = crate::BitReader<Debugrun>;
impl DebugrunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Debugrun {
        match self.bits {
            false => Debugrun::X0,
            true => Debugrun::X1,
        }
    }
    #[doc = "LESENSE can not start new scans in debug mode"]
    #[inline(always)]
    pub fn is_x0(&self) -> bool {
        *self == Debugrun::X0
    }
    #[doc = "LESENSE can start new scans in debug mode"]
    #[inline(always)]
    pub fn is_x1(&self) -> bool {
        *self == Debugrun::X1
    }
}
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DebugrunW<'a, REG> = crate::BitWriter<'a, REG, Debugrun>;
impl<'a, REG> DebugrunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LESENSE can not start new scans in debug mode"]
    #[inline(always)]
    pub fn x0(self) -> &'a mut crate::W<REG> {
        self.variant(Debugrun::X0)
    }
    #[doc = "LESENSE can start new scans in debug mode"]
    #[inline(always)]
    pub fn x1(self) -> &'a mut crate::W<REG> {
        self.variant(Debugrun::X1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Configure scan mode"]
    #[inline(always)]
    pub fn scanmode(&self) -> ScanmodeR {
        ScanmodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Select scan configuration"]
    #[inline(always)]
    pub fn scanconf(&self) -> ScanconfR {
        ScanconfR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - Enable dual sample mode"]
    #[inline(always)]
    pub fn dualsample(&self) -> DualsampleR {
        DualsampleR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable storing of SCANRES"]
    #[inline(always)]
    pub fn strscanres(&self) -> StrscanresR {
        StrscanresR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA wake-up from EM2"]
    #[inline(always)]
    pub fn dmawu(&self) -> DmawuR {
        DmawuR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Result FIFO level"]
    #[inline(always)]
    pub fn resfidl(&self) -> ResfidlR {
        ResfidlR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DebugrunR {
        DebugrunR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configure scan mode"]
    #[inline(always)]
    pub fn scanmode(&mut self) -> ScanmodeW<'_, CfgSpec> {
        ScanmodeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Select scan configuration"]
    #[inline(always)]
    pub fn scanconf(&mut self) -> ScanconfW<'_, CfgSpec> {
        ScanconfW::new(self, 2)
    }
    #[doc = "Bit 5 - Enable dual sample mode"]
    #[inline(always)]
    pub fn dualsample(&mut self) -> DualsampleW<'_, CfgSpec> {
        DualsampleW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable storing of SCANRES"]
    #[inline(always)]
    pub fn strscanres(&mut self) -> StrscanresW<'_, CfgSpec> {
        StrscanresW::new(self, 6)
    }
    #[doc = "Bit 7 - DMA wake-up from EM2"]
    #[inline(always)]
    pub fn dmawu(&mut self) -> DmawuW<'_, CfgSpec> {
        DmawuW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Result FIFO level"]
    #[inline(always)]
    pub fn resfidl(&mut self) -> ResfidlW<'_, CfgSpec> {
        ResfidlW::new(self, 8)
    }
    #[doc = "Bit 17 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&mut self) -> DebugrunW<'_, CfgSpec> {
        DebugrunW::new(self, 17)
    }
}
#[doc = "Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
