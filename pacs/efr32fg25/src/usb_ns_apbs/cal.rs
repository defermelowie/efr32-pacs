#[doc = "Register `CAL` reader"]
pub type R = crate::R<CalSpec>;
#[doc = "Register `CAL` writer"]
pub type W = crate::W<CalSpec>;
#[doc = "Field `FSSLEWT` reader - FS slew"]
pub type FsslewtR = crate::FieldReader;
#[doc = "Field `FSSLEWT` writer - FS slew"]
pub type FsslewtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 24:26 - FS slew"]
    #[inline(always)]
    pub fn fsslewt(&self) -> FsslewtR {
        FsslewtR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 24:26 - FS slew"]
    #[inline(always)]
    pub fn fsslewt(&mut self) -> FsslewtW<'_, CalSpec> {
        FsslewtW::new(self, 24)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalSpec;
impl crate::RegisterSpec for CalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal::R`](R) reader structure"]
impl crate::Readable for CalSpec {}
#[doc = "`write(|w| ..)` method takes [`cal::W`](W) writer structure"]
impl crate::Writable for CalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAL to value 0x0707_8000"]
impl crate::Resettable for CalSpec {
    const RESET_VALUE: u32 = 0x0707_8000;
}
