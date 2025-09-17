#[doc = "Register `CFGNSRPURATD0` reader"]
pub type R = crate::R<Cfgnsrpuratd0Spec>;
#[doc = "Register `CFGNSRPURATD0` writer"]
pub type W = crate::W<Cfgnsrpuratd0Spec>;
#[doc = "Field `RATDCFGNSTCALIB` reader - CFGNSTCALIB Protection Bit"]
pub type RatdcfgnstcalibR = crate::BitReader;
#[doc = "Field `RATDCFGNSTCALIB` writer - CFGNSTCALIB Protection Bit"]
pub type RatdcfgnstcalibW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RATDCFGNSSYSTIC` reader - CFGNSSYSTIC Protection Bit"]
pub type RatdcfgnssysticR = crate::BitReader;
#[doc = "Field `RATDCFGNSSYSTIC` writer - CFGNSSYSTIC Protection Bit"]
pub type RatdcfgnssysticW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - CFGNSTCALIB Protection Bit"]
    #[inline(always)]
    pub fn ratdcfgnstcalib(&self) -> RatdcfgnstcalibR {
        RatdcfgnstcalibR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CFGNSSYSTIC Protection Bit"]
    #[inline(always)]
    pub fn ratdcfgnssystic(&self) -> RatdcfgnssysticR {
        RatdcfgnssysticR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - CFGNSTCALIB Protection Bit"]
    #[inline(always)]
    pub fn ratdcfgnstcalib(&mut self) -> RatdcfgnstcalibW<'_, Cfgnsrpuratd0Spec> {
        RatdcfgnstcalibW::new(self, 7)
    }
    #[doc = "Bit 8 - CFGNSSYSTIC Protection Bit"]
    #[inline(always)]
    pub fn ratdcfgnssystic(&mut self) -> RatdcfgnssysticW<'_, Cfgnsrpuratd0Spec> {
        RatdcfgnssysticW::new(self, 8)
    }
}
#[doc = "Protected register address = (RPURATD register index X 32 + RPURATD bit index) X 4.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgnsrpuratd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgnsrpuratd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgnsrpuratd0Spec;
impl crate::RegisterSpec for Cfgnsrpuratd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgnsrpuratd0::R`](R) reader structure"]
impl crate::Readable for Cfgnsrpuratd0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgnsrpuratd0::W`](W) writer structure"]
impl crate::Writable for Cfgnsrpuratd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGNSRPURATD0 to value 0"]
impl crate::Resettable for Cfgnsrpuratd0Spec {}
