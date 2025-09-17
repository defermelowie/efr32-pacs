#[doc = "Register `GHWCFG4` reader"]
pub type R = crate::R<Ghwcfg4Spec>;
#[doc = "Field `NUMDEVPERIOEPS` reader - Dev Mode Periodic IN EPs"]
pub type NumdevperioepsR = crate::FieldReader;
#[doc = "Field `PARTIALPWRDN` reader - New BitField"]
pub type PartialpwrdnR = crate::BitReader;
#[doc = "Field `AHBFREQ` reader - AHB Frequency"]
pub type AhbfreqR = crate::BitReader;
#[doc = "Field `HIBERNATION` reader - New BitField"]
pub type HibernationR = crate::BitReader;
#[doc = "Field `EXTENDEDHIBERNATION` reader - New BitField"]
pub type ExtendedhibernationR = crate::BitReader;
#[doc = "Field `PHYDATAWIDTH` reader - New BitField"]
pub type PhydatawidthR = crate::FieldReader;
#[doc = "Field `NUMCTLEPS` reader - New BitField"]
pub type NumctlepsR = crate::FieldReader;
#[doc = "Field `IDDGFLTR` reader - New BitField"]
pub type IddgfltrR = crate::BitReader;
#[doc = "Field `VBUSVALIDFLTR` reader - New BitField"]
pub type VbusvalidfltrR = crate::BitReader;
#[doc = "Field `AVALIDFLTR` reader - New BitField"]
pub type AvalidfltrR = crate::BitReader;
#[doc = "Field `BVALIDFLTR` reader - New BitField"]
pub type BvalidfltrR = crate::BitReader;
#[doc = "Field `SESSENDFLTR` reader - New BitField"]
pub type SessendfltrR = crate::BitReader;
#[doc = "Field `DEDFIFOMODE` reader - Dedicated xmit FIFO For device IN EPs"]
pub type DedfifomodeR = crate::BitReader;
#[doc = "Field `INEPS` reader - Num Device Mode IN EPs inc Ctrl EPs"]
pub type InepsR = crate::FieldReader;
#[doc = "Field `DESCDMAENABLED` reader - New BitField"]
pub type DescdmaenabledR = crate::BitReader;
#[doc = "Field `DESCDMA` reader - New BitField"]
pub type DescdmaR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Dev Mode Periodic IN EPs"]
    #[inline(always)]
    pub fn numdevperioeps(&self) -> NumdevperioepsR {
        NumdevperioepsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - New BitField"]
    #[inline(always)]
    pub fn partialpwrdn(&self) -> PartialpwrdnR {
        PartialpwrdnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AHB Frequency"]
    #[inline(always)]
    pub fn ahbfreq(&self) -> AhbfreqR {
        AhbfreqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - New BitField"]
    #[inline(always)]
    pub fn hibernation(&self) -> HibernationR {
        HibernationR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - New BitField"]
    #[inline(always)]
    pub fn extendedhibernation(&self) -> ExtendedhibernationR {
        ExtendedhibernationR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 14:15 - New BitField"]
    #[inline(always)]
    pub fn phydatawidth(&self) -> PhydatawidthR {
        PhydatawidthR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - New BitField"]
    #[inline(always)]
    pub fn numctleps(&self) -> NumctlepsR {
        NumctlepsR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - New BitField"]
    #[inline(always)]
    pub fn iddgfltr(&self) -> IddgfltrR {
        IddgfltrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - New BitField"]
    #[inline(always)]
    pub fn vbusvalidfltr(&self) -> VbusvalidfltrR {
        VbusvalidfltrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - New BitField"]
    #[inline(always)]
    pub fn avalidfltr(&self) -> AvalidfltrR {
        AvalidfltrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - New BitField"]
    #[inline(always)]
    pub fn bvalidfltr(&self) -> BvalidfltrR {
        BvalidfltrR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - New BitField"]
    #[inline(always)]
    pub fn sessendfltr(&self) -> SessendfltrR {
        SessendfltrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Dedicated xmit FIFO For device IN EPs"]
    #[inline(always)]
    pub fn dedfifomode(&self) -> DedfifomodeR {
        DedfifomodeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:29 - Num Device Mode IN EPs inc Ctrl EPs"]
    #[inline(always)]
    pub fn ineps(&self) -> InepsR {
        InepsR::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - New BitField"]
    #[inline(always)]
    pub fn descdmaenabled(&self) -> DescdmaenabledR {
        DescdmaenabledR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - New BitField"]
    #[inline(always)]
    pub fn descdma(&self) -> DescdmaR {
        DescdmaR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ghwcfg4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ghwcfg4Spec;
impl crate::RegisterSpec for Ghwcfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghwcfg4::R`](R) reader structure"]
impl crate::Readable for Ghwcfg4Spec {}
#[doc = "`reset()` method sets GHWCFG4 to value 0x2600_8030"]
impl crate::Resettable for Ghwcfg4Spec {
    const RESET_VALUE: u32 = 0x2600_8030;
}
