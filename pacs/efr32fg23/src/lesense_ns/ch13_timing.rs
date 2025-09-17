#[doc = "Register `CH13_TIMING` reader"]
pub type R = crate::R<Ch13TimingSpec>;
#[doc = "Register `CH13_TIMING` writer"]
pub type W = crate::W<Ch13TimingSpec>;
#[doc = "Field `EXTIME` reader - Set excitation time"]
pub type ExtimeR = crate::FieldReader;
#[doc = "Field `EXTIME` writer - Set excitation time"]
pub type ExtimeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SAMPLEDLY` reader - Set sample delay"]
pub type SampledlyR = crate::FieldReader;
#[doc = "Field `SAMPLEDLY` writer - Set sample delay"]
pub type SampledlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEASUREDLY` reader - Set measure delay"]
pub type MeasuredlyR = crate::FieldReader<u16>;
#[doc = "Field `MEASUREDLY` writer - Set measure delay"]
pub type MeasuredlyW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:5 - Set excitation time"]
    #[inline(always)]
    pub fn extime(&self) -> ExtimeR {
        ExtimeR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:13 - Set sample delay"]
    #[inline(always)]
    pub fn sampledly(&self) -> SampledlyR {
        SampledlyR::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 14:23 - Set measure delay"]
    #[inline(always)]
    pub fn measuredly(&self) -> MeasuredlyR {
        MeasuredlyR::new(((self.bits >> 14) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - Set excitation time"]
    #[inline(always)]
    pub fn extime(&mut self) -> ExtimeW<'_, Ch13TimingSpec> {
        ExtimeW::new(self, 0)
    }
    #[doc = "Bits 6:13 - Set sample delay"]
    #[inline(always)]
    pub fn sampledly(&mut self) -> SampledlyW<'_, Ch13TimingSpec> {
        SampledlyW::new(self, 6)
    }
    #[doc = "Bits 14:23 - Set measure delay"]
    #[inline(always)]
    pub fn measuredly(&mut self) -> MeasuredlyW<'_, Ch13TimingSpec> {
        MeasuredlyW::new(self, 14)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_timing::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_timing::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch13TimingSpec;
impl crate::RegisterSpec for Ch13TimingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch13_timing::R`](R) reader structure"]
impl crate::Readable for Ch13TimingSpec {}
#[doc = "`write(|w| ..)` method takes [`ch13_timing::W`](W) writer structure"]
impl crate::Writable for Ch13TimingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH13_TIMING to value 0"]
impl crate::Resettable for Ch13TimingSpec {}
