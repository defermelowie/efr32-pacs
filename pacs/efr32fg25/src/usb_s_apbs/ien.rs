#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `VBUS` reader - VBUS_SENSE interrupt enable"]
pub type VbusR = crate::BitReader;
#[doc = "Field `VBUS` writer - VBUS_SENSE interrupt enable"]
pub type VbusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DWCOTG` reader - DWC_OTG interrupt enable"]
pub type DwcotgR = crate::BitReader;
#[doc = "Field `DWCOTG` writer - DWC_OTG interrupt enable"]
pub type DwcotgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - VBUS_SENSE interrupt enable"]
    #[inline(always)]
    pub fn vbus(&self) -> VbusR {
        VbusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DWC_OTG interrupt enable"]
    #[inline(always)]
    pub fn dwcotg(&self) -> DwcotgR {
        DwcotgR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBUS_SENSE interrupt enable"]
    #[inline(always)]
    pub fn vbus(&mut self) -> VbusW<'_, IenSpec> {
        VbusW::new(self, 0)
    }
    #[doc = "Bit 1 - DWC_OTG interrupt enable"]
    #[inline(always)]
    pub fn dwcotg(&mut self) -> DwcotgW<'_, IenSpec> {
        DwcotgW::new(self, 1)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
