#[doc = "Register `RADIORAMRETNCTRL` reader"]
pub type R = crate::R<RadioramretnctrlSpec>;
#[doc = "Register `RADIORAMRETNCTRL` writer"]
pub type W = crate::W<RadioramretnctrlSpec>;
#[doc = "SEQRAM Retention Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Seqramretnctrl {
    #[doc = "0: SEQRAM not powered down"]
    Allon = 0,
    #[doc = "1: Power down SEQRAM block 0"]
    Blk0 = 1,
    #[doc = "2: Power down SEQRAM block 1"]
    Blk1 = 2,
    #[doc = "4: Power down SEQRAM block 2"]
    Blk2 = 4,
    #[doc = "6: Power down SEQRAM block 1 to 2"]
    Blk1to2 = 6,
    #[doc = "7: Power down all SEQRAM blocks"]
    Alloff = 7,
}
impl From<Seqramretnctrl> for u8 {
    #[inline(always)]
    fn from(variant: Seqramretnctrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Seqramretnctrl {
    type Ux = u8;
}
impl crate::IsEnum for Seqramretnctrl {}
#[doc = "Field `SEQRAMRETNCTRL` reader - SEQRAM Retention Control"]
pub type SeqramretnctrlR = crate::FieldReader<Seqramretnctrl>;
impl SeqramretnctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Seqramretnctrl> {
        match self.bits {
            0 => Some(Seqramretnctrl::Allon),
            1 => Some(Seqramretnctrl::Blk0),
            2 => Some(Seqramretnctrl::Blk1),
            4 => Some(Seqramretnctrl::Blk2),
            6 => Some(Seqramretnctrl::Blk1to2),
            7 => Some(Seqramretnctrl::Alloff),
            _ => None,
        }
    }
    #[doc = "SEQRAM not powered down"]
    #[inline(always)]
    pub fn is_allon(&self) -> bool {
        *self == Seqramretnctrl::Allon
    }
    #[doc = "Power down SEQRAM block 0"]
    #[inline(always)]
    pub fn is_blk0(&self) -> bool {
        *self == Seqramretnctrl::Blk0
    }
    #[doc = "Power down SEQRAM block 1"]
    #[inline(always)]
    pub fn is_blk1(&self) -> bool {
        *self == Seqramretnctrl::Blk1
    }
    #[doc = "Power down SEQRAM block 2"]
    #[inline(always)]
    pub fn is_blk2(&self) -> bool {
        *self == Seqramretnctrl::Blk2
    }
    #[doc = "Power down SEQRAM block 1 to 2"]
    #[inline(always)]
    pub fn is_blk1to2(&self) -> bool {
        *self == Seqramretnctrl::Blk1to2
    }
    #[doc = "Power down all SEQRAM blocks"]
    #[inline(always)]
    pub fn is_alloff(&self) -> bool {
        *self == Seqramretnctrl::Alloff
    }
}
#[doc = "Field `SEQRAMRETNCTRL` writer - SEQRAM Retention Control"]
pub type SeqramretnctrlW<'a, REG> = crate::FieldWriter<'a, REG, 3, Seqramretnctrl>;
impl<'a, REG> SeqramretnctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SEQRAM not powered down"]
    #[inline(always)]
    pub fn allon(self) -> &'a mut crate::W<REG> {
        self.variant(Seqramretnctrl::Allon)
    }
    #[doc = "Power down SEQRAM block 0"]
    #[inline(always)]
    pub fn blk0(self) -> &'a mut crate::W<REG> {
        self.variant(Seqramretnctrl::Blk0)
    }
    #[doc = "Power down SEQRAM block 1"]
    #[inline(always)]
    pub fn blk1(self) -> &'a mut crate::W<REG> {
        self.variant(Seqramretnctrl::Blk1)
    }
    #[doc = "Power down SEQRAM block 2"]
    #[inline(always)]
    pub fn blk2(self) -> &'a mut crate::W<REG> {
        self.variant(Seqramretnctrl::Blk2)
    }
    #[doc = "Power down SEQRAM block 1 to 2"]
    #[inline(always)]
    pub fn blk1to2(self) -> &'a mut crate::W<REG> {
        self.variant(Seqramretnctrl::Blk1to2)
    }
    #[doc = "Power down all SEQRAM blocks"]
    #[inline(always)]
    pub fn alloff(self) -> &'a mut crate::W<REG> {
        self.variant(Seqramretnctrl::Alloff)
    }
}
#[doc = "FRCRAM Retention Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frcramretnctrl {
    #[doc = "0: FRCRAM not powered down"]
    Allon = 0,
    #[doc = "1: Power down FRCRAM"]
    Alloff = 1,
}
impl From<Frcramretnctrl> for bool {
    #[inline(always)]
    fn from(variant: Frcramretnctrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRCRAMRETNCTRL` reader - FRCRAM Retention Control"]
pub type FrcramretnctrlR = crate::BitReader<Frcramretnctrl>;
impl FrcramretnctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frcramretnctrl {
        match self.bits {
            false => Frcramretnctrl::Allon,
            true => Frcramretnctrl::Alloff,
        }
    }
    #[doc = "FRCRAM not powered down"]
    #[inline(always)]
    pub fn is_allon(&self) -> bool {
        *self == Frcramretnctrl::Allon
    }
    #[doc = "Power down FRCRAM"]
    #[inline(always)]
    pub fn is_alloff(&self) -> bool {
        *self == Frcramretnctrl::Alloff
    }
}
#[doc = "Field `FRCRAMRETNCTRL` writer - FRCRAM Retention Control"]
pub type FrcramretnctrlW<'a, REG> = crate::BitWriter<'a, REG, Frcramretnctrl>;
impl<'a, REG> FrcramretnctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FRCRAM not powered down"]
    #[inline(always)]
    pub fn allon(self) -> &'a mut crate::W<REG> {
        self.variant(Frcramretnctrl::Allon)
    }
    #[doc = "Power down FRCRAM"]
    #[inline(always)]
    pub fn alloff(self) -> &'a mut crate::W<REG> {
        self.variant(Frcramretnctrl::Alloff)
    }
}
#[doc = "ITCRAM Retention Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Itcramretnctrl {
    #[doc = "0: None of the RAM blocks powered down"]
    Allon = 0,
    #[doc = "512: Power down RAM block 9"]
    Blk9 = 512,
    #[doc = "768: Power down RAM block 8 and above"]
    Blk8to9 = 768,
    #[doc = "896: Power down RAM block 7 and above"]
    Blk7to9 = 896,
    #[doc = "960: Power down RAM block 6 and above"]
    Blk6to9 = 960,
    #[doc = "992: Power down RAM block 5 and above"]
    Blk5to9 = 992,
    #[doc = "1008: Power down RAM block 4 and above"]
    Blk4to9 = 1008,
    #[doc = "1016: Power down RAM block 3 and above"]
    Blk3to9 = 1016,
    #[doc = "1020: Power down RAM block 2 and above"]
    Blk2to9 = 1020,
    #[doc = "1022: Power down RAM block 1 and above"]
    Blk1to9 = 1022,
    #[doc = "1023: All of the RAM blocks powered down"]
    Alloff = 1023,
}
impl From<Itcramretnctrl> for u16 {
    #[inline(always)]
    fn from(variant: Itcramretnctrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Itcramretnctrl {
    type Ux = u16;
}
impl crate::IsEnum for Itcramretnctrl {}
#[doc = "Field `ITCRAMRETNCTRL` reader - ITCRAM Retention Control"]
pub type ItcramretnctrlR = crate::FieldReader<Itcramretnctrl>;
impl ItcramretnctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Itcramretnctrl> {
        match self.bits {
            0 => Some(Itcramretnctrl::Allon),
            512 => Some(Itcramretnctrl::Blk9),
            768 => Some(Itcramretnctrl::Blk8to9),
            896 => Some(Itcramretnctrl::Blk7to9),
            960 => Some(Itcramretnctrl::Blk6to9),
            992 => Some(Itcramretnctrl::Blk5to9),
            1008 => Some(Itcramretnctrl::Blk4to9),
            1016 => Some(Itcramretnctrl::Blk3to9),
            1020 => Some(Itcramretnctrl::Blk2to9),
            1022 => Some(Itcramretnctrl::Blk1to9),
            1023 => Some(Itcramretnctrl::Alloff),
            _ => None,
        }
    }
    #[doc = "None of the RAM blocks powered down"]
    #[inline(always)]
    pub fn is_allon(&self) -> bool {
        *self == Itcramretnctrl::Allon
    }
    #[doc = "Power down RAM block 9"]
    #[inline(always)]
    pub fn is_blk9(&self) -> bool {
        *self == Itcramretnctrl::Blk9
    }
    #[doc = "Power down RAM block 8 and above"]
    #[inline(always)]
    pub fn is_blk8to9(&self) -> bool {
        *self == Itcramretnctrl::Blk8to9
    }
    #[doc = "Power down RAM block 7 and above"]
    #[inline(always)]
    pub fn is_blk7to9(&self) -> bool {
        *self == Itcramretnctrl::Blk7to9
    }
    #[doc = "Power down RAM block 6 and above"]
    #[inline(always)]
    pub fn is_blk6to9(&self) -> bool {
        *self == Itcramretnctrl::Blk6to9
    }
    #[doc = "Power down RAM block 5 and above"]
    #[inline(always)]
    pub fn is_blk5to9(&self) -> bool {
        *self == Itcramretnctrl::Blk5to9
    }
    #[doc = "Power down RAM block 4 and above"]
    #[inline(always)]
    pub fn is_blk4to9(&self) -> bool {
        *self == Itcramretnctrl::Blk4to9
    }
    #[doc = "Power down RAM block 3 and above"]
    #[inline(always)]
    pub fn is_blk3to9(&self) -> bool {
        *self == Itcramretnctrl::Blk3to9
    }
    #[doc = "Power down RAM block 2 and above"]
    #[inline(always)]
    pub fn is_blk2to9(&self) -> bool {
        *self == Itcramretnctrl::Blk2to9
    }
    #[doc = "Power down RAM block 1 and above"]
    #[inline(always)]
    pub fn is_blk1to9(&self) -> bool {
        *self == Itcramretnctrl::Blk1to9
    }
    #[doc = "All of the RAM blocks powered down"]
    #[inline(always)]
    pub fn is_alloff(&self) -> bool {
        *self == Itcramretnctrl::Alloff
    }
}
#[doc = "Field `ITCRAMRETNCTRL` writer - ITCRAM Retention Control"]
pub type ItcramretnctrlW<'a, REG> = crate::FieldWriter<'a, REG, 10, Itcramretnctrl>;
impl<'a, REG> ItcramretnctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "None of the RAM blocks powered down"]
    #[inline(always)]
    pub fn allon(self) -> &'a mut crate::W<REG> {
        self.variant(Itcramretnctrl::Allon)
    }
    #[doc = "Power down RAM block 9"]
    #[inline(always)]
    pub fn blk9(self) -> &'a mut crate::W<REG> {
        self.variant(Itcramretnctrl::Blk9)
    }
    #[doc = "Power down RAM block 8 and above"]
    #[inline(always)]
    pub fn blk8to9(self) -> &'a mut crate::W<REG> {
        self.variant(Itcramretnctrl::Blk8to9)
    }
    #[doc = "Power down RAM block 7 and above"]
    #[inline(always)]
    pub fn blk7to9(self) -> &'a mut crate::W<REG> {
        self.variant(Itcramretnctrl::Blk7to9)
    }
    #[doc = "Power down RAM block 6 and above"]
    #[inline(always)]
    pub fn blk6to9(self) -> &'a mut crate::W<REG> {
        self.variant(Itcramretnctrl::Blk6to9)
    }
    #[doc = "Power down RAM block 5 and above"]
    #[inline(always)]
    pub fn blk5to9(self) -> &'a mut crate::W<REG> {
        self.variant(Itcramretnctrl::Blk5to9)
    }
    #[doc = "Power down RAM block 4 and above"]
    #[inline(always)]
    pub fn blk4to9(self) -> &'a mut crate::W<REG> {
        self.variant(Itcramretnctrl::Blk4to9)
    }
    #[doc = "Power down RAM block 3 and above"]
    #[inline(always)]
    pub fn blk3to9(self) -> &'a mut crate::W<REG> {
        self.variant(Itcramretnctrl::Blk3to9)
    }
    #[doc = "Power down RAM block 2 and above"]
    #[inline(always)]
    pub fn blk2to9(self) -> &'a mut crate::W<REG> {
        self.variant(Itcramretnctrl::Blk2to9)
    }
    #[doc = "Power down RAM block 1 and above"]
    #[inline(always)]
    pub fn blk1to9(self) -> &'a mut crate::W<REG> {
        self.variant(Itcramretnctrl::Blk1to9)
    }
    #[doc = "All of the RAM blocks powered down"]
    #[inline(always)]
    pub fn alloff(self) -> &'a mut crate::W<REG> {
        self.variant(Itcramretnctrl::Alloff)
    }
}
#[doc = "DTCRAM Retention Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dtcramretnctrl {
    #[doc = "0: None of the RAMs powered down"]
    Allon = 0,
    #[doc = "2: BLK1"]
    Blk1 = 2,
    #[doc = "3: ALLOFF"]
    Alloff = 3,
}
impl From<Dtcramretnctrl> for u8 {
    #[inline(always)]
    fn from(variant: Dtcramretnctrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dtcramretnctrl {
    type Ux = u8;
}
impl crate::IsEnum for Dtcramretnctrl {}
#[doc = "Field `DTCRAMRETNCTRL` reader - DTCRAM Retention Control"]
pub type DtcramretnctrlR = crate::FieldReader<Dtcramretnctrl>;
impl DtcramretnctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dtcramretnctrl> {
        match self.bits {
            0 => Some(Dtcramretnctrl::Allon),
            2 => Some(Dtcramretnctrl::Blk1),
            3 => Some(Dtcramretnctrl::Alloff),
            _ => None,
        }
    }
    #[doc = "None of the RAMs powered down"]
    #[inline(always)]
    pub fn is_allon(&self) -> bool {
        *self == Dtcramretnctrl::Allon
    }
    #[doc = "BLK1"]
    #[inline(always)]
    pub fn is_blk1(&self) -> bool {
        *self == Dtcramretnctrl::Blk1
    }
    #[doc = "ALLOFF"]
    #[inline(always)]
    pub fn is_alloff(&self) -> bool {
        *self == Dtcramretnctrl::Alloff
    }
}
#[doc = "Field `DTCRAMRETNCTRL` writer - DTCRAM Retention Control"]
pub type DtcramretnctrlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dtcramretnctrl>;
impl<'a, REG> DtcramretnctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None of the RAMs powered down"]
    #[inline(always)]
    pub fn allon(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcramretnctrl::Allon)
    }
    #[doc = "BLK1"]
    #[inline(always)]
    pub fn blk1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcramretnctrl::Blk1)
    }
    #[doc = "ALLOFF"]
    #[inline(always)]
    pub fn alloff(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcramretnctrl::Alloff)
    }
}
#[doc = "AHBRAM Retention Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ahbramretnctrl {
    #[doc = "0: no banks powered down"]
    Allon = 0,
    #[doc = "8: power down blk3"]
    Blk3 = 8,
    #[doc = "12: power down blk 2 to 3"]
    Blk2to3 = 12,
    #[doc = "14: power down blk 1 to 3"]
    Blk1to3 = 14,
    #[doc = "15: power down all blocks"]
    Alloff = 15,
}
impl From<Ahbramretnctrl> for u8 {
    #[inline(always)]
    fn from(variant: Ahbramretnctrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ahbramretnctrl {
    type Ux = u8;
}
impl crate::IsEnum for Ahbramretnctrl {}
#[doc = "Field `AHBRAMRETNCTRL` reader - AHBRAM Retention Control"]
pub type AhbramretnctrlR = crate::FieldReader<Ahbramretnctrl>;
impl AhbramretnctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ahbramretnctrl> {
        match self.bits {
            0 => Some(Ahbramretnctrl::Allon),
            8 => Some(Ahbramretnctrl::Blk3),
            12 => Some(Ahbramretnctrl::Blk2to3),
            14 => Some(Ahbramretnctrl::Blk1to3),
            15 => Some(Ahbramretnctrl::Alloff),
            _ => None,
        }
    }
    #[doc = "no banks powered down"]
    #[inline(always)]
    pub fn is_allon(&self) -> bool {
        *self == Ahbramretnctrl::Allon
    }
    #[doc = "power down blk3"]
    #[inline(always)]
    pub fn is_blk3(&self) -> bool {
        *self == Ahbramretnctrl::Blk3
    }
    #[doc = "power down blk 2 to 3"]
    #[inline(always)]
    pub fn is_blk2to3(&self) -> bool {
        *self == Ahbramretnctrl::Blk2to3
    }
    #[doc = "power down blk 1 to 3"]
    #[inline(always)]
    pub fn is_blk1to3(&self) -> bool {
        *self == Ahbramretnctrl::Blk1to3
    }
    #[doc = "power down all blocks"]
    #[inline(always)]
    pub fn is_alloff(&self) -> bool {
        *self == Ahbramretnctrl::Alloff
    }
}
#[doc = "Field `AHBRAMRETNCTRL` writer - AHBRAM Retention Control"]
pub type AhbramretnctrlW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ahbramretnctrl>;
impl<'a, REG> AhbramretnctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no banks powered down"]
    #[inline(always)]
    pub fn allon(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbramretnctrl::Allon)
    }
    #[doc = "power down blk3"]
    #[inline(always)]
    pub fn blk3(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbramretnctrl::Blk3)
    }
    #[doc = "power down blk 2 to 3"]
    #[inline(always)]
    pub fn blk2to3(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbramretnctrl::Blk2to3)
    }
    #[doc = "power down blk 1 to 3"]
    #[inline(always)]
    pub fn blk1to3(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbramretnctrl::Blk1to3)
    }
    #[doc = "power down all blocks"]
    #[inline(always)]
    pub fn alloff(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbramretnctrl::Alloff)
    }
}
impl R {
    #[doc = "Bits 0:2 - SEQRAM Retention Control"]
    #[inline(always)]
    pub fn seqramretnctrl(&self) -> SeqramretnctrlR {
        SeqramretnctrlR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - FRCRAM Retention Control"]
    #[inline(always)]
    pub fn frcramretnctrl(&self) -> FrcramretnctrlR {
        FrcramretnctrlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:21 - ITCRAM Retention Control"]
    #[inline(always)]
    pub fn itcramretnctrl(&self) -> ItcramretnctrlR {
        ItcramretnctrlR::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:25 - DTCRAM Retention Control"]
    #[inline(always)]
    pub fn dtcramretnctrl(&self) -> DtcramretnctrlR {
        DtcramretnctrlR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:31 - AHBRAM Retention Control"]
    #[inline(always)]
    pub fn ahbramretnctrl(&self) -> AhbramretnctrlR {
        AhbramretnctrlR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SEQRAM Retention Control"]
    #[inline(always)]
    pub fn seqramretnctrl(&mut self) -> SeqramretnctrlW<'_, RadioramretnctrlSpec> {
        SeqramretnctrlW::new(self, 0)
    }
    #[doc = "Bit 8 - FRCRAM Retention Control"]
    #[inline(always)]
    pub fn frcramretnctrl(&mut self) -> FrcramretnctrlW<'_, RadioramretnctrlSpec> {
        FrcramretnctrlW::new(self, 8)
    }
    #[doc = "Bits 12:21 - ITCRAM Retention Control"]
    #[inline(always)]
    pub fn itcramretnctrl(&mut self) -> ItcramretnctrlW<'_, RadioramretnctrlSpec> {
        ItcramretnctrlW::new(self, 12)
    }
    #[doc = "Bits 24:25 - DTCRAM Retention Control"]
    #[inline(always)]
    pub fn dtcramretnctrl(&mut self) -> DtcramretnctrlW<'_, RadioramretnctrlSpec> {
        DtcramretnctrlW::new(self, 24)
    }
    #[doc = "Bits 28:31 - AHBRAM Retention Control"]
    #[inline(always)]
    pub fn ahbramretnctrl(&mut self) -> AhbramretnctrlW<'_, RadioramretnctrlSpec> {
        AhbramretnctrlW::new(self, 28)
    }
}
#[doc = "Configure SEQRAM Retention controls.\n\nYou can [`read`](crate::Reg::read) this register and get [`radioramretnctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`radioramretnctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RadioramretnctrlSpec;
impl crate::RegisterSpec for RadioramretnctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`radioramretnctrl::R`](R) reader structure"]
impl crate::Readable for RadioramretnctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`radioramretnctrl::W`](W) writer structure"]
impl crate::Writable for RadioramretnctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RADIORAMRETNCTRL to value 0"]
impl crate::Resettable for RadioramretnctrlSpec {}
