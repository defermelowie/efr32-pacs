#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `VBUS` reader - VBUS_SENSE interrupt flag"]
pub type VbusR = crate::BitReader;
#[doc = "Field `VBUS` writer - VBUS_SENSE interrupt flag"]
pub type VbusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DWCOTG` reader - DWC_OTG interrupt flag"]
pub type DwcotgR = crate::BitReader;
#[doc = "Field `DWCOTG` writer - DWC_OTG interrupt flag"]
pub type DwcotgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - VBUS_SENSE interrupt flag"]
    #[inline(always)]
    pub fn vbus(&self) -> VbusR {
        VbusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DWC_OTG interrupt flag"]
    #[inline(always)]
    pub fn dwcotg(&self) -> DwcotgR {
        DwcotgR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBUS_SENSE interrupt flag"]
    #[inline(always)]
    pub fn vbus(&mut self) -> VbusW<'_, IfSpec> {
        VbusW::new(self, 0)
    }
    #[doc = "Bit 1 - DWC_OTG interrupt flag"]
    #[inline(always)]
    pub fn dwcotg(&mut self) -> DwcotgW<'_, IfSpec> {
        DwcotgW::new(self, 1)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`write(|w| ..)` method takes [`if_::W`](W) writer structure"]
impl crate::Writable for IfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IfSpec {}
