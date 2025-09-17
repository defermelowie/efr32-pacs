#[doc = "Register `DMEM0PORTMAPSEL` reader"]
pub type R = crate::R<Dmem0portmapselSpec>;
#[doc = "Register `DMEM0PORTMAPSEL` writer"]
pub type W = crate::W<Dmem0portmapselSpec>;
#[doc = "Field `LDMAPORTSEL` reader - LDMA portmap selection"]
pub type LdmaportselR = crate::BitReader;
#[doc = "Field `LDMAPORTSEL` writer - LDMA portmap selection"]
pub type LdmaportselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRWAESPORTSEL` reader - SRWAES portmap selection"]
pub type SrwaesportselR = crate::BitReader;
#[doc = "Field `SRWAESPORTSEL` writer - SRWAES portmap selection"]
pub type SrwaesportselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBSRWPORTSEL` reader - AHBSRW portmap selection"]
pub type AhbsrwportselR = crate::BitReader;
#[doc = "Field `AHBSRWPORTSEL` writer - AHBSRW portmap selection"]
pub type AhbsrwportselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRWECA0PORTSEL` reader - SRWECA0 portmap selection"]
pub type Srweca0portselR = crate::BitReader;
#[doc = "Field `SRWECA0PORTSEL` writer - SRWECA0 portmap selection"]
pub type Srweca0portselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRWECA1PORTSEL` reader - SRWECA1 portmap selection"]
pub type Srweca1portselR = crate::BitReader;
#[doc = "Field `SRWECA1PORTSEL` writer - SRWECA1 portmap selection"]
pub type Srweca1portselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDMA1PORTSEL` reader - LDMA port1 portmap selection"]
pub type Ldma1portselR = crate::BitReader;
#[doc = "Field `LDMA1PORTSEL` writer - LDMA port1 portmap selection"]
pub type Ldma1portselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRWLDMAPORTSEL` reader - SRWLDMA port1 portmap selection"]
pub type SrwldmaportselR = crate::BitReader;
#[doc = "Field `SRWLDMAPORTSEL` writer - SRWLDMA port1 portmap selection"]
pub type SrwldmaportselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBPORTSEL` reader - USB portmap selection"]
pub type UsbportselR = crate::BitReader;
#[doc = "Field `USBPORTSEL` writer - USB portmap selection"]
pub type UsbportselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFCPORTSEL` reader - BUFC portmap selection"]
pub type BufcportselR = crate::BitReader;
#[doc = "Field `BUFCPORTSEL` writer - BUFC portmap selection"]
pub type BufcportselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LDMA portmap selection"]
    #[inline(always)]
    pub fn ldmaportsel(&self) -> LdmaportselR {
        LdmaportselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRWAES portmap selection"]
    #[inline(always)]
    pub fn srwaesportsel(&self) -> SrwaesportselR {
        SrwaesportselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHBSRW portmap selection"]
    #[inline(always)]
    pub fn ahbsrwportsel(&self) -> AhbsrwportselR {
        AhbsrwportselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SRWECA0 portmap selection"]
    #[inline(always)]
    pub fn srweca0portsel(&self) -> Srweca0portselR {
        Srweca0portselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SRWECA1 portmap selection"]
    #[inline(always)]
    pub fn srweca1portsel(&self) -> Srweca1portselR {
        Srweca1portselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LDMA port1 portmap selection"]
    #[inline(always)]
    pub fn ldma1portsel(&self) -> Ldma1portselR {
        Ldma1portselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SRWLDMA port1 portmap selection"]
    #[inline(always)]
    pub fn srwldmaportsel(&self) -> SrwldmaportselR {
        SrwldmaportselR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB portmap selection"]
    #[inline(always)]
    pub fn usbportsel(&self) -> UsbportselR {
        UsbportselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BUFC portmap selection"]
    #[inline(always)]
    pub fn bufcportsel(&self) -> BufcportselR {
        BufcportselR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LDMA portmap selection"]
    #[inline(always)]
    pub fn ldmaportsel(&mut self) -> LdmaportselW<'_, Dmem0portmapselSpec> {
        LdmaportselW::new(self, 0)
    }
    #[doc = "Bit 1 - SRWAES portmap selection"]
    #[inline(always)]
    pub fn srwaesportsel(&mut self) -> SrwaesportselW<'_, Dmem0portmapselSpec> {
        SrwaesportselW::new(self, 1)
    }
    #[doc = "Bit 2 - AHBSRW portmap selection"]
    #[inline(always)]
    pub fn ahbsrwportsel(&mut self) -> AhbsrwportselW<'_, Dmem0portmapselSpec> {
        AhbsrwportselW::new(self, 2)
    }
    #[doc = "Bit 3 - SRWECA0 portmap selection"]
    #[inline(always)]
    pub fn srweca0portsel(&mut self) -> Srweca0portselW<'_, Dmem0portmapselSpec> {
        Srweca0portselW::new(self, 3)
    }
    #[doc = "Bit 4 - SRWECA1 portmap selection"]
    #[inline(always)]
    pub fn srweca1portsel(&mut self) -> Srweca1portselW<'_, Dmem0portmapselSpec> {
        Srweca1portselW::new(self, 4)
    }
    #[doc = "Bit 5 - LDMA port1 portmap selection"]
    #[inline(always)]
    pub fn ldma1portsel(&mut self) -> Ldma1portselW<'_, Dmem0portmapselSpec> {
        Ldma1portselW::new(self, 5)
    }
    #[doc = "Bit 6 - SRWLDMA port1 portmap selection"]
    #[inline(always)]
    pub fn srwldmaportsel(&mut self) -> SrwldmaportselW<'_, Dmem0portmapselSpec> {
        SrwldmaportselW::new(self, 6)
    }
    #[doc = "Bit 7 - USB portmap selection"]
    #[inline(always)]
    pub fn usbportsel(&mut self) -> UsbportselW<'_, Dmem0portmapselSpec> {
        UsbportselW::new(self, 7)
    }
    #[doc = "Bit 8 - BUFC portmap selection"]
    #[inline(always)]
    pub fn bufcportsel(&mut self) -> BufcportselW<'_, Dmem0portmapselSpec> {
        BufcportselW::new(self, 8)
    }
}
#[doc = "Configure DMEM0 port remap selection.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmem0portmapsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmem0portmapsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmem0portmapselSpec;
impl crate::RegisterSpec for Dmem0portmapselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmem0portmapsel::R`](R) reader structure"]
impl crate::Readable for Dmem0portmapselSpec {}
#[doc = "`write(|w| ..)` method takes [`dmem0portmapsel::W`](W) writer structure"]
impl crate::Writable for Dmem0portmapselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMEM0PORTMAPSEL to value 0x93"]
impl crate::Resettable for Dmem0portmapselSpec {
    const RESET_VALUE: u32 = 0x93;
}
