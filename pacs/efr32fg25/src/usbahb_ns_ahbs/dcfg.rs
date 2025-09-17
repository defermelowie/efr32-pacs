#[doc = "Register `DCFG` reader"]
pub type R = crate::R<DcfgSpec>;
#[doc = "Register `DCFG` writer"]
pub type W = crate::W<DcfgSpec>;
#[doc = "Field `DEVSPD` reader - Device Speed"]
pub type DevspdR = crate::FieldReader;
#[doc = "Field `DEVSPD` writer - Device Speed"]
pub type DevspdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NZSTSOUTHSHK` reader - NZ-Length Status OUT Handshake"]
pub type NzstsouthshkR = crate::BitReader;
#[doc = "Field `NZSTSOUTHSHK` writer - NZ-Length Status OUT Handshake"]
pub type NzstsouthshkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENA32KHZSUSP` reader - Enable 32 KHz Suspend mode"]
pub type Ena32khzsuspR = crate::BitReader;
#[doc = "Field `ENA32KHZSUSP` writer - Enable 32 KHz Suspend mode"]
pub type Ena32khzsuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVADDR` reader - Device Address"]
pub type DevaddrR = crate::FieldReader;
#[doc = "Field `DEVADDR` writer - Device Address"]
pub type DevaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PERFRINT` reader - Periodic Frame Interval"]
pub type PerfrintR = crate::FieldReader;
#[doc = "Field `PERFRINT` writer - Periodic Frame Interval"]
pub type PerfrintW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENDEVOUTNAK` reader - Enable Device out of NAK"]
pub type EndevoutnakR = crate::BitReader;
#[doc = "Field `ENDEVOUTNAK` writer - Enable Device out of NAK"]
pub type EndevoutnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XCVRDLY` reader - No Description"]
pub type XcvrdlyR = crate::BitReader;
#[doc = "Field `XCVRDLY` writer - No Description"]
pub type XcvrdlyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRATICINTMSK` reader - No Description"]
pub type ErraticintmskR = crate::BitReader;
#[doc = "Field `ERRATICINTMSK` writer - No Description"]
pub type ErraticintmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERSCHINTVL` reader - Periodic Scheduling Interval"]
pub type PerschintvlR = crate::FieldReader;
#[doc = "Field `PERSCHINTVL` writer - Periodic Scheduling Interval"]
pub type PerschintvlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESVALID` reader - Resume VAlidation Period"]
pub type ResvalidR = crate::FieldReader;
#[doc = "Field `RESVALID` writer - Resume VAlidation Period"]
pub type ResvalidW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline(always)]
    pub fn devspd(&self) -> DevspdR {
        DevspdR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - NZ-Length Status OUT Handshake"]
    #[inline(always)]
    pub fn nzstsouthshk(&self) -> NzstsouthshkR {
        NzstsouthshkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable 32 KHz Suspend mode"]
    #[inline(always)]
    pub fn ena32khzsusp(&self) -> Ena32khzsuspR {
        Ena32khzsuspR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline(always)]
    pub fn devaddr(&self) -> DevaddrR {
        DevaddrR::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline(always)]
    pub fn perfrint(&self) -> PerfrintR {
        PerfrintR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Enable Device out of NAK"]
    #[inline(always)]
    pub fn endevoutnak(&self) -> EndevoutnakR {
        EndevoutnakR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - No Description"]
    #[inline(always)]
    pub fn xcvrdly(&self) -> XcvrdlyR {
        XcvrdlyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - No Description"]
    #[inline(always)]
    pub fn erraticintmsk(&self) -> ErraticintmskR {
        ErraticintmskR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Periodic Scheduling Interval"]
    #[inline(always)]
    pub fn perschintvl(&self) -> PerschintvlR {
        PerschintvlR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:31 - Resume VAlidation Period"]
    #[inline(always)]
    pub fn resvalid(&self) -> ResvalidR {
        ResvalidR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline(always)]
    pub fn devspd(&mut self) -> DevspdW<'_, DcfgSpec> {
        DevspdW::new(self, 0)
    }
    #[doc = "Bit 2 - NZ-Length Status OUT Handshake"]
    #[inline(always)]
    pub fn nzstsouthshk(&mut self) -> NzstsouthshkW<'_, DcfgSpec> {
        NzstsouthshkW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable 32 KHz Suspend mode"]
    #[inline(always)]
    pub fn ena32khzsusp(&mut self) -> Ena32khzsuspW<'_, DcfgSpec> {
        Ena32khzsuspW::new(self, 3)
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline(always)]
    pub fn devaddr(&mut self) -> DevaddrW<'_, DcfgSpec> {
        DevaddrW::new(self, 4)
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline(always)]
    pub fn perfrint(&mut self) -> PerfrintW<'_, DcfgSpec> {
        PerfrintW::new(self, 11)
    }
    #[doc = "Bit 13 - Enable Device out of NAK"]
    #[inline(always)]
    pub fn endevoutnak(&mut self) -> EndevoutnakW<'_, DcfgSpec> {
        EndevoutnakW::new(self, 13)
    }
    #[doc = "Bit 14 - No Description"]
    #[inline(always)]
    pub fn xcvrdly(&mut self) -> XcvrdlyW<'_, DcfgSpec> {
        XcvrdlyW::new(self, 14)
    }
    #[doc = "Bit 15 - No Description"]
    #[inline(always)]
    pub fn erraticintmsk(&mut self) -> ErraticintmskW<'_, DcfgSpec> {
        ErraticintmskW::new(self, 15)
    }
    #[doc = "Bits 24:25 - Periodic Scheduling Interval"]
    #[inline(always)]
    pub fn perschintvl(&mut self) -> PerschintvlW<'_, DcfgSpec> {
        PerschintvlW::new(self, 24)
    }
    #[doc = "Bits 26:31 - Resume VAlidation Period"]
    #[inline(always)]
    pub fn resvalid(&mut self) -> ResvalidW<'_, DcfgSpec> {
        ResvalidW::new(self, 26)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcfgSpec;
impl crate::RegisterSpec for DcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcfg::R`](R) reader structure"]
impl crate::Readable for DcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dcfg::W`](W) writer structure"]
impl crate::Writable for DcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCFG to value 0x0800_0000"]
impl crate::Resettable for DcfgSpec {
    const RESET_VALUE: u32 = 0x0800_0000;
}
