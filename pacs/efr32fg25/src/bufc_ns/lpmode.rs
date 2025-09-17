#[doc = "Register `LPMODE` reader"]
pub type R = crate::R<LpmodeSpec>;
#[doc = "Register `LPMODE` writer"]
pub type W = crate::W<LpmodeSpec>;
#[doc = "Field `LPEN` reader - Low power mode enable"]
pub type LpenR = crate::FieldReader;
#[doc = "Field `LPEN` writer - Low power mode enable"]
pub type LpenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Low power mode enable"]
    #[inline(always)]
    pub fn lpen(&self) -> LpenR {
        LpenR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low power mode enable"]
    #[inline(always)]
    pub fn lpen(&mut self) -> LpenW<'_, LpmodeSpec> {
        LpenW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`lpmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpmodeSpec;
impl crate::RegisterSpec for LpmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpmode::R`](R) reader structure"]
impl crate::Readable for LpmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`lpmode::W`](W) writer structure"]
impl crate::Writable for LpmodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPMODE to value 0"]
impl crate::Resettable for LpmodeSpec {}
