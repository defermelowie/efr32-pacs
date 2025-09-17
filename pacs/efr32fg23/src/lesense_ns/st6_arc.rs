#[doc = "Register `ST6_ARC` reader"]
pub type R = crate::R<St6ArcSpec>;
#[doc = "Register `ST6_ARC` writer"]
pub type W = crate::W<St6ArcSpec>;
#[doc = "Field `SCOMP` reader - Sensor compare value"]
pub type ScompR = crate::FieldReader;
#[doc = "Field `SCOMP` writer - Sensor compare value"]
pub type ScompW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SMASK` reader - Sensor mask"]
pub type SmaskR = crate::FieldReader;
#[doc = "Field `SMASK` writer - Sensor mask"]
pub type SmaskW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CURSTATE` reader - Current State"]
pub type CurstateR = crate::FieldReader;
#[doc = "Field `CURSTATE` writer - Current State"]
pub type CurstateW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Configure transition action in normal mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prsact {
    #[doc = "0: No PRS output generated (if PRSCOUNT == 0), or do not count (if PRSCOUNT == 1)."]
    None = 0,
    #[doc = "1: Pulse generated on LESENSE PRS output 0 (if PRSCOUNT == 0)."]
    Prs0 = 1,
    #[doc = "1: Count Up (if PRSCOUNT == 1)."]
    Up = 1,
    #[doc = "2: Pulse generated on LESENSE PRS output 1 (if PRSCOUNT == 0)."]
    Prs1 = 2,
    #[doc = "2: Count Down (if PRSCOUNT == 1)."]
    Down = 2,
    #[doc = "3: Pulse generated on LESENSE PRS output 0 and 1 (if PRSCOUNT == 0)."]
    Prs01 = 3,
    #[doc = "4: Pulse generated on LESENSE PRS output 2. (PRSCOUNT == 0 OR 1)."]
    Prs2 = 4,
    #[doc = "5: Pulse generated on LESENSE PRS output 0 and 2 (if PRSCOUNT == 0)."]
    Prs02 = 5,
    #[doc = "5: Count Up and Pulse generated on LESENSE PRS output 2 (if PRSCOUNT == 1)."]
    Upandprs2 = 5,
    #[doc = "6: Pulse generated on LESENSE PRS output 1 and 2 (if PRSCOUNT == 0)."]
    Prs12 = 6,
    #[doc = "6: Count Down and Pulse generated on LESENSE PRS output 2 (if PRSCOUNT == 1)."]
    Downandprs2 = 6,
    #[doc = "7: Pulse generated on LESENSE PRS output 0, 1 and 2 (if PRSCOUNT == 0)."]
    Prs012 = 7,
}
impl From<Prsact> for u8 {
    #[inline(always)]
    fn from(variant: Prsact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prsact {
    type Ux = u8;
}
impl crate::IsEnum for Prsact {}
#[doc = "Field `PRSACT` reader - Configure transition action in normal mode"]
pub type PrsactR = crate::FieldReader<Prsact>;
impl PrsactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prsact> {
        match self.bits {
            0 => Some(Prsact::None),
            1 => Some(Prsact::Prs0),
            1 => Some(Prsact::Up),
            2 => Some(Prsact::Prs1),
            2 => Some(Prsact::Down),
            3 => Some(Prsact::Prs01),
            4 => Some(Prsact::Prs2),
            5 => Some(Prsact::Prs02),
            5 => Some(Prsact::Upandprs2),
            6 => Some(Prsact::Prs12),
            6 => Some(Prsact::Downandprs2),
            7 => Some(Prsact::Prs012),
            _ => None,
        }
    }
    #[doc = "No PRS output generated (if PRSCOUNT == 0), or do not count (if PRSCOUNT == 1)."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Prsact::None
    }
    #[doc = "Pulse generated on LESENSE PRS output 0 (if PRSCOUNT == 0)."]
    #[inline(always)]
    pub fn is_prs0(&self) -> bool {
        *self == Prsact::Prs0
    }
    #[doc = "Count Up (if PRSCOUNT == 1)."]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Prsact::Up
    }
    #[doc = "Pulse generated on LESENSE PRS output 1 (if PRSCOUNT == 0)."]
    #[inline(always)]
    pub fn is_prs1(&self) -> bool {
        *self == Prsact::Prs1
    }
    #[doc = "Count Down (if PRSCOUNT == 1)."]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Prsact::Down
    }
    #[doc = "Pulse generated on LESENSE PRS output 0 and 1 (if PRSCOUNT == 0)."]
    #[inline(always)]
    pub fn is_prs01(&self) -> bool {
        *self == Prsact::Prs01
    }
    #[doc = "Pulse generated on LESENSE PRS output 2. (PRSCOUNT == 0 OR 1)."]
    #[inline(always)]
    pub fn is_prs2(&self) -> bool {
        *self == Prsact::Prs2
    }
    #[doc = "Pulse generated on LESENSE PRS output 0 and 2 (if PRSCOUNT == 0)."]
    #[inline(always)]
    pub fn is_prs02(&self) -> bool {
        *self == Prsact::Prs02
    }
    #[doc = "Count Up and Pulse generated on LESENSE PRS output 2 (if PRSCOUNT == 1)."]
    #[inline(always)]
    pub fn is_upandprs2(&self) -> bool {
        *self == Prsact::Upandprs2
    }
    #[doc = "Pulse generated on LESENSE PRS output 1 and 2 (if PRSCOUNT == 0)."]
    #[inline(always)]
    pub fn is_prs12(&self) -> bool {
        *self == Prsact::Prs12
    }
    #[doc = "Count Down and Pulse generated on LESENSE PRS output 2 (if PRSCOUNT == 1)."]
    #[inline(always)]
    pub fn is_downandprs2(&self) -> bool {
        *self == Prsact::Downandprs2
    }
    #[doc = "Pulse generated on LESENSE PRS output 0, 1 and 2 (if PRSCOUNT == 0)."]
    #[inline(always)]
    pub fn is_prs012(&self) -> bool {
        *self == Prsact::Prs012
    }
}
#[doc = "Field `PRSACT` writer - Configure transition action in normal mode"]
pub type PrsactW<'a, REG> = crate::FieldWriter<'a, REG, 3, Prsact>;
impl<'a, REG> PrsactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No PRS output generated (if PRSCOUNT == 0), or do not count (if PRSCOUNT == 1)."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Prsact::None)
    }
    #[doc = "Pulse generated on LESENSE PRS output 0 (if PRSCOUNT == 0)."]
    #[inline(always)]
    pub fn prs0(self) -> &'a mut crate::W<REG> {
        self.variant(Prsact::Prs0)
    }
    #[doc = "Count Up (if PRSCOUNT == 1)."]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Prsact::Up)
    }
    #[doc = "Pulse generated on LESENSE PRS output 1 (if PRSCOUNT == 0)."]
    #[inline(always)]
    pub fn prs1(self) -> &'a mut crate::W<REG> {
        self.variant(Prsact::Prs1)
    }
    #[doc = "Count Down (if PRSCOUNT == 1)."]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(Prsact::Down)
    }
    #[doc = "Pulse generated on LESENSE PRS output 0 and 1 (if PRSCOUNT == 0)."]
    #[inline(always)]
    pub fn prs01(self) -> &'a mut crate::W<REG> {
        self.variant(Prsact::Prs01)
    }
    #[doc = "Pulse generated on LESENSE PRS output 2. (PRSCOUNT == 0 OR 1)."]
    #[inline(always)]
    pub fn prs2(self) -> &'a mut crate::W<REG> {
        self.variant(Prsact::Prs2)
    }
    #[doc = "Pulse generated on LESENSE PRS output 0 and 2 (if PRSCOUNT == 0)."]
    #[inline(always)]
    pub fn prs02(self) -> &'a mut crate::W<REG> {
        self.variant(Prsact::Prs02)
    }
    #[doc = "Count Up and Pulse generated on LESENSE PRS output 2 (if PRSCOUNT == 1)."]
    #[inline(always)]
    pub fn upandprs2(self) -> &'a mut crate::W<REG> {
        self.variant(Prsact::Upandprs2)
    }
    #[doc = "Pulse generated on LESENSE PRS output 1 and 2 (if PRSCOUNT == 0)."]
    #[inline(always)]
    pub fn prs12(self) -> &'a mut crate::W<REG> {
        self.variant(Prsact::Prs12)
    }
    #[doc = "Count Down and Pulse generated on LESENSE PRS output 2 (if PRSCOUNT == 1)."]
    #[inline(always)]
    pub fn downandprs2(self) -> &'a mut crate::W<REG> {
        self.variant(Prsact::Downandprs2)
    }
    #[doc = "Pulse generated on LESENSE PRS output 0, 1 and 2 (if PRSCOUNT == 0)."]
    #[inline(always)]
    pub fn prs012(self) -> &'a mut crate::W<REG> {
        self.variant(Prsact::Prs012)
    }
}
#[doc = "Field `NEXTSTATE` reader - Next state index"]
pub type NextstateR = crate::FieldReader;
#[doc = "Field `NEXTSTATE` writer - Next state index"]
pub type NextstateW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SETIF` reader - Set interrupt flag"]
pub type SetifR = crate::BitReader;
#[doc = "Field `SETIF` writer - Set interrupt flag"]
pub type SetifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Sensor compare value"]
    #[inline(always)]
    pub fn scomp(&self) -> ScompR {
        ScompR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Sensor mask"]
    #[inline(always)]
    pub fn smask(&self) -> SmaskR {
        SmaskR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Current State"]
    #[inline(always)]
    pub fn curstate(&self) -> CurstateR {
        CurstateR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - Configure transition action in normal mode"]
    #[inline(always)]
    pub fn prsact(&self) -> PrsactR {
        PrsactR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - Next state index"]
    #[inline(always)]
    pub fn nextstate(&self) -> NextstateR {
        NextstateR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Set interrupt flag"]
    #[inline(always)]
    pub fn setif(&self) -> SetifR {
        SetifR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sensor compare value"]
    #[inline(always)]
    pub fn scomp(&mut self) -> ScompW<'_, St6ArcSpec> {
        ScompW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Sensor mask"]
    #[inline(always)]
    pub fn smask(&mut self) -> SmaskW<'_, St6ArcSpec> {
        SmaskW::new(self, 4)
    }
    #[doc = "Bits 8:12 - Current State"]
    #[inline(always)]
    pub fn curstate(&mut self) -> CurstateW<'_, St6ArcSpec> {
        CurstateW::new(self, 8)
    }
    #[doc = "Bits 13:15 - Configure transition action in normal mode"]
    #[inline(always)]
    pub fn prsact(&mut self) -> PrsactW<'_, St6ArcSpec> {
        PrsactW::new(self, 13)
    }
    #[doc = "Bits 16:20 - Next state index"]
    #[inline(always)]
    pub fn nextstate(&mut self) -> NextstateW<'_, St6ArcSpec> {
        NextstateW::new(self, 16)
    }
    #[doc = "Bit 21 - Set interrupt flag"]
    #[inline(always)]
    pub fn setif(&mut self) -> SetifW<'_, St6ArcSpec> {
        SetifW::new(self, 21)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`st6_arc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st6_arc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St6ArcSpec;
impl crate::RegisterSpec for St6ArcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st6_arc::R`](R) reader structure"]
impl crate::Readable for St6ArcSpec {}
#[doc = "`write(|w| ..)` method takes [`st6_arc::W`](W) writer structure"]
impl crate::Writable for St6ArcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ST6_ARC to value 0"]
impl crate::Resettable for St6ArcSpec {}
