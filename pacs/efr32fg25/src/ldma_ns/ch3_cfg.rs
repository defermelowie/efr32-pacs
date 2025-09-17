#[doc = "Register `CH3_CFG` reader"]
pub type R = crate::R<Ch3CfgSpec>;
#[doc = "Register `CH3_CFG` writer"]
pub type W = crate::W<Ch3CfgSpec>;
#[doc = "Arbitration Slot Number Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Arbslots {
    #[doc = "0: One arbitration slot selected"]
    One = 0,
    #[doc = "1: Two arbitration slots selected"]
    Two = 1,
    #[doc = "2: Four arbitration slots selected"]
    Four = 2,
    #[doc = "3: Eight arbitration slots selected"]
    Eight = 3,
}
impl From<Arbslots> for u8 {
    #[inline(always)]
    fn from(variant: Arbslots) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Arbslots {
    type Ux = u8;
}
impl crate::IsEnum for Arbslots {}
#[doc = "Field `ARBSLOTS` reader - Arbitration Slot Number Select"]
pub type ArbslotsR = crate::FieldReader<Arbslots>;
impl ArbslotsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arbslots {
        match self.bits {
            0 => Arbslots::One,
            1 => Arbslots::Two,
            2 => Arbslots::Four,
            3 => Arbslots::Eight,
            _ => unreachable!(),
        }
    }
    #[doc = "One arbitration slot selected"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Arbslots::One
    }
    #[doc = "Two arbitration slots selected"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Arbslots::Two
    }
    #[doc = "Four arbitration slots selected"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == Arbslots::Four
    }
    #[doc = "Eight arbitration slots selected"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == Arbslots::Eight
    }
}
#[doc = "Field `ARBSLOTS` writer - Arbitration Slot Number Select"]
pub type ArbslotsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Arbslots, crate::Safe>;
impl<'a, REG> ArbslotsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One arbitration slot selected"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Arbslots::One)
    }
    #[doc = "Two arbitration slots selected"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Arbslots::Two)
    }
    #[doc = "Four arbitration slots selected"]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(Arbslots::Four)
    }
    #[doc = "Eight arbitration slots selected"]
    #[inline(always)]
    pub fn eight(self) -> &'a mut crate::W<REG> {
        self.variant(Arbslots::Eight)
    }
}
#[doc = "Source Address Increment Sign\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srcincsign {
    #[doc = "0: Increment source address"]
    Positive = 0,
    #[doc = "1: Decrement source address"]
    Negative = 1,
}
impl From<Srcincsign> for bool {
    #[inline(always)]
    fn from(variant: Srcincsign) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRCINCSIGN` reader - Source Address Increment Sign"]
pub type SrcincsignR = crate::BitReader<Srcincsign>;
impl SrcincsignR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srcincsign {
        match self.bits {
            false => Srcincsign::Positive,
            true => Srcincsign::Negative,
        }
    }
    #[doc = "Increment source address"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == Srcincsign::Positive
    }
    #[doc = "Decrement source address"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == Srcincsign::Negative
    }
}
#[doc = "Field `SRCINCSIGN` writer - Source Address Increment Sign"]
pub type SrcincsignW<'a, REG> = crate::BitWriter<'a, REG, Srcincsign>;
impl<'a, REG> SrcincsignW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Increment source address"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut crate::W<REG> {
        self.variant(Srcincsign::Positive)
    }
    #[doc = "Decrement source address"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut crate::W<REG> {
        self.variant(Srcincsign::Negative)
    }
}
#[doc = "Destination Address Increment Sign\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dstincsign {
    #[doc = "0: Increment destination address"]
    Positive = 0,
    #[doc = "1: Decrement destination address"]
    Negative = 1,
}
impl From<Dstincsign> for bool {
    #[inline(always)]
    fn from(variant: Dstincsign) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSTINCSIGN` reader - Destination Address Increment Sign"]
pub type DstincsignR = crate::BitReader<Dstincsign>;
impl DstincsignR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dstincsign {
        match self.bits {
            false => Dstincsign::Positive,
            true => Dstincsign::Negative,
        }
    }
    #[doc = "Increment destination address"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == Dstincsign::Positive
    }
    #[doc = "Decrement destination address"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == Dstincsign::Negative
    }
}
#[doc = "Field `DSTINCSIGN` writer - Destination Address Increment Sign"]
pub type DstincsignW<'a, REG> = crate::BitWriter<'a, REG, Dstincsign>;
impl<'a, REG> DstincsignW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Increment destination address"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut crate::W<REG> {
        self.variant(Dstincsign::Positive)
    }
    #[doc = "Decrement destination address"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut crate::W<REG> {
        self.variant(Dstincsign::Negative)
    }
}
#[doc = "Structure Fetch Bus Port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Structbusport {
    #[doc = "0: AHBM0"]
    Ahbm0 = 0,
    #[doc = "1: AHBM1"]
    Ahbm1 = 1,
}
impl From<Structbusport> for bool {
    #[inline(always)]
    fn from(variant: Structbusport) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRUCTBUSPORT` reader - Structure Fetch Bus Port"]
pub type StructbusportR = crate::BitReader<Structbusport>;
impl StructbusportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Structbusport {
        match self.bits {
            false => Structbusport::Ahbm0,
            true => Structbusport::Ahbm1,
        }
    }
    #[doc = "AHBM0"]
    #[inline(always)]
    pub fn is_ahbm0(&self) -> bool {
        *self == Structbusport::Ahbm0
    }
    #[doc = "AHBM1"]
    #[inline(always)]
    pub fn is_ahbm1(&self) -> bool {
        *self == Structbusport::Ahbm1
    }
}
#[doc = "Field `STRUCTBUSPORT` writer - Structure Fetch Bus Port"]
pub type StructbusportW<'a, REG> = crate::BitWriter<'a, REG, Structbusport>;
impl<'a, REG> StructbusportW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AHBM0"]
    #[inline(always)]
    pub fn ahbm0(self) -> &'a mut crate::W<REG> {
        self.variant(Structbusport::Ahbm0)
    }
    #[doc = "AHBM1"]
    #[inline(always)]
    pub fn ahbm1(self) -> &'a mut crate::W<REG> {
        self.variant(Structbusport::Ahbm1)
    }
}
#[doc = "Source Bus Port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srcbusport {
    #[doc = "0: AHBM0"]
    Ahbm0 = 0,
    #[doc = "1: AHBM1"]
    Ahbm1 = 1,
}
impl From<Srcbusport> for bool {
    #[inline(always)]
    fn from(variant: Srcbusport) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRCBUSPORT` reader - Source Bus Port"]
pub type SrcbusportR = crate::BitReader<Srcbusport>;
impl SrcbusportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srcbusport {
        match self.bits {
            false => Srcbusport::Ahbm0,
            true => Srcbusport::Ahbm1,
        }
    }
    #[doc = "AHBM0"]
    #[inline(always)]
    pub fn is_ahbm0(&self) -> bool {
        *self == Srcbusport::Ahbm0
    }
    #[doc = "AHBM1"]
    #[inline(always)]
    pub fn is_ahbm1(&self) -> bool {
        *self == Srcbusport::Ahbm1
    }
}
#[doc = "Field `SRCBUSPORT` writer - Source Bus Port"]
pub type SrcbusportW<'a, REG> = crate::BitWriter<'a, REG, Srcbusport>;
impl<'a, REG> SrcbusportW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AHBM0"]
    #[inline(always)]
    pub fn ahbm0(self) -> &'a mut crate::W<REG> {
        self.variant(Srcbusport::Ahbm0)
    }
    #[doc = "AHBM1"]
    #[inline(always)]
    pub fn ahbm1(self) -> &'a mut crate::W<REG> {
        self.variant(Srcbusport::Ahbm1)
    }
}
#[doc = "Destination Bus Port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dstbusport {
    #[doc = "0: AHBM0"]
    Ahbm0 = 0,
    #[doc = "1: AHBM1"]
    Ahbm1 = 1,
}
impl From<Dstbusport> for bool {
    #[inline(always)]
    fn from(variant: Dstbusport) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSTBUSPORT` reader - Destination Bus Port"]
pub type DstbusportR = crate::BitReader<Dstbusport>;
impl DstbusportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dstbusport {
        match self.bits {
            false => Dstbusport::Ahbm0,
            true => Dstbusport::Ahbm1,
        }
    }
    #[doc = "AHBM0"]
    #[inline(always)]
    pub fn is_ahbm0(&self) -> bool {
        *self == Dstbusport::Ahbm0
    }
    #[doc = "AHBM1"]
    #[inline(always)]
    pub fn is_ahbm1(&self) -> bool {
        *self == Dstbusport::Ahbm1
    }
}
#[doc = "Field `DSTBUSPORT` writer - Destination Bus Port"]
pub type DstbusportW<'a, REG> = crate::BitWriter<'a, REG, Dstbusport>;
impl<'a, REG> DstbusportW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AHBM0"]
    #[inline(always)]
    pub fn ahbm0(self) -> &'a mut crate::W<REG> {
        self.variant(Dstbusport::Ahbm0)
    }
    #[doc = "AHBM1"]
    #[inline(always)]
    pub fn ahbm1(self) -> &'a mut crate::W<REG> {
        self.variant(Dstbusport::Ahbm1)
    }
}
impl R {
    #[doc = "Bits 16:17 - Arbitration Slot Number Select"]
    #[inline(always)]
    pub fn arbslots(&self) -> ArbslotsR {
        ArbslotsR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - Source Address Increment Sign"]
    #[inline(always)]
    pub fn srcincsign(&self) -> SrcincsignR {
        SrcincsignR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Destination Address Increment Sign"]
    #[inline(always)]
    pub fn dstincsign(&self) -> DstincsignR {
        DstincsignR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Structure Fetch Bus Port"]
    #[inline(always)]
    pub fn structbusport(&self) -> StructbusportR {
        StructbusportR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Source Bus Port"]
    #[inline(always)]
    pub fn srcbusport(&self) -> SrcbusportR {
        SrcbusportR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Destination Bus Port"]
    #[inline(always)]
    pub fn dstbusport(&self) -> DstbusportR {
        DstbusportR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - Arbitration Slot Number Select"]
    #[inline(always)]
    pub fn arbslots(&mut self) -> ArbslotsW<'_, Ch3CfgSpec> {
        ArbslotsW::new(self, 16)
    }
    #[doc = "Bit 20 - Source Address Increment Sign"]
    #[inline(always)]
    pub fn srcincsign(&mut self) -> SrcincsignW<'_, Ch3CfgSpec> {
        SrcincsignW::new(self, 20)
    }
    #[doc = "Bit 21 - Destination Address Increment Sign"]
    #[inline(always)]
    pub fn dstincsign(&mut self) -> DstincsignW<'_, Ch3CfgSpec> {
        DstincsignW::new(self, 21)
    }
    #[doc = "Bit 22 - Structure Fetch Bus Port"]
    #[inline(always)]
    pub fn structbusport(&mut self) -> StructbusportW<'_, Ch3CfgSpec> {
        StructbusportW::new(self, 22)
    }
    #[doc = "Bit 23 - Source Bus Port"]
    #[inline(always)]
    pub fn srcbusport(&mut self) -> SrcbusportW<'_, Ch3CfgSpec> {
        SrcbusportW::new(self, 23)
    }
    #[doc = "Bit 24 - Destination Bus Port"]
    #[inline(always)]
    pub fn dstbusport(&mut self) -> DstbusportW<'_, Ch3CfgSpec> {
        DstbusportW::new(self, 24)
    }
}
#[doc = "Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3CfgSpec;
impl crate::RegisterSpec for Ch3CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3_cfg::R`](R) reader structure"]
impl crate::Readable for Ch3CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ch3_cfg::W`](W) writer structure"]
impl crate::Writable for Ch3CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH3_CFG to value 0"]
impl crate::Resettable for Ch3CfgSpec {}
