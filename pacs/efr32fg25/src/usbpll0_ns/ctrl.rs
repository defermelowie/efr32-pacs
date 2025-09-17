#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `FORCEEN` reader - Force Enable"]
pub type ForceenR = crate::BitReader;
#[doc = "Field `FORCEEN` writer - Force Enable"]
pub type ForceenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISONDEMAND` reader - Disable on Demand"]
pub type DisondemandR = crate::BitReader;
#[doc = "Field `DISONDEMAND` writer - Disable on Demand"]
pub type DisondemandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHUNTREGLPEN` reader - Shunt Regulator LP Enable"]
pub type ShuntreglpenR = crate::BitReader;
#[doc = "Field `SHUNTREGLPEN` writer - Shunt Regulator LP Enable"]
pub type ShuntreglpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVR` reader - Divider R"]
pub type DivrR = crate::FieldReader;
#[doc = "Field `DIVR` writer - Divider R"]
pub type DivrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DIVX` reader - Divider X"]
pub type DivxR = crate::FieldReader;
#[doc = "Field `DIVX` writer - Divider X"]
pub type DivxW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DIVN` reader - Divider N"]
pub type DivnR = crate::FieldReader;
#[doc = "Field `DIVN` writer - Divider N"]
pub type DivnW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Force Enable"]
    #[inline(always)]
    pub fn forceen(&self) -> ForceenR {
        ForceenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable on Demand"]
    #[inline(always)]
    pub fn disondemand(&self) -> DisondemandR {
        DisondemandR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Shunt Regulator LP Enable"]
    #[inline(always)]
    pub fn shuntreglpen(&self) -> ShuntreglpenR {
        ShuntreglpenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Divider R"]
    #[inline(always)]
    pub fn divr(&self) -> DivrR {
        DivrR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:17 - Divider X"]
    #[inline(always)]
    pub fn divx(&self) -> DivxR {
        DivxR::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 18:24 - Divider N"]
    #[inline(always)]
    pub fn divn(&self) -> DivnR {
        DivnR::new(((self.bits >> 18) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Force Enable"]
    #[inline(always)]
    pub fn forceen(&mut self) -> ForceenW<'_, CtrlSpec> {
        ForceenW::new(self, 0)
    }
    #[doc = "Bit 1 - Disable on Demand"]
    #[inline(always)]
    pub fn disondemand(&mut self) -> DisondemandW<'_, CtrlSpec> {
        DisondemandW::new(self, 1)
    }
    #[doc = "Bit 4 - Shunt Regulator LP Enable"]
    #[inline(always)]
    pub fn shuntreglpen(&mut self) -> ShuntreglpenW<'_, CtrlSpec> {
        ShuntreglpenW::new(self, 4)
    }
    #[doc = "Bits 8:12 - Divider R"]
    #[inline(always)]
    pub fn divr(&mut self) -> DivrW<'_, CtrlSpec> {
        DivrW::new(self, 8)
    }
    #[doc = "Bits 13:17 - Divider X"]
    #[inline(always)]
    pub fn divx(&mut self) -> DivxW<'_, CtrlSpec> {
        DivxW::new(self, 13)
    }
    #[doc = "Bits 18:24 - Divider N"]
    #[inline(always)]
    pub fn divn(&mut self) -> DivnW<'_, CtrlSpec> {
        DivnW::new(self, 18)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0x0036_0100"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x0036_0100;
}
