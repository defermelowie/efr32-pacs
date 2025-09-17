#[doc = "Register `CALTOP` reader"]
pub type R = crate::R<CaltopSpec>;
#[doc = "Register `CALTOP` writer"]
pub type W = crate::W<CaltopSpec>;
#[doc = "Field `CALTOP` reader - Calibration Counter Top Value"]
pub type CaltopR = crate::FieldReader<u32>;
#[doc = "Field `CALTOP` writer - Calibration Counter Top Value"]
pub type CaltopW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Calibration Counter Top Value"]
    #[inline(always)]
    pub fn caltop(&self) -> CaltopR {
        CaltopR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Calibration Counter Top Value"]
    #[inline(always)]
    pub fn caltop(&mut self) -> CaltopW<'_, CaltopSpec> {
        CaltopW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`caltop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caltop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CaltopSpec;
impl crate::RegisterSpec for CaltopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`caltop::R`](R) reader structure"]
impl crate::Readable for CaltopSpec {}
#[doc = "`write(|w| ..)` method takes [`caltop::W`](W) writer structure"]
impl crate::Writable for CaltopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CALTOP to value 0"]
impl crate::Resettable for CaltopSpec {}
