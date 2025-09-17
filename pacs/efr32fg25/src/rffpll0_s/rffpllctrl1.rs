#[doc = "Register `RFFPLLCTRL1` reader"]
pub type R = crate::R<Rffpllctrl1Spec>;
#[doc = "Register `RFFPLLCTRL1` writer"]
pub type W = crate::W<Rffpllctrl1Spec>;
#[doc = "Field `DIVN` reader - Feedback Divider Ratio"]
pub type DivnR = crate::FieldReader;
#[doc = "Field `DIVN` writer - Feedback Divider Ratio"]
pub type DivnW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DIVX` reader - Clock Divider Ratio for Radio"]
pub type DivxR = crate::FieldReader;
#[doc = "Field `DIVX` writer - Clock Divider Ratio for Radio"]
pub type DivxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIVY` reader - Clock Divider Ratio for digital"]
pub type DivyR = crate::FieldReader;
#[doc = "Field `DIVY` writer - Clock Divider Ratio for digital"]
pub type DivyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:6 - Feedback Divider Ratio"]
    #[inline(always)]
    pub fn divn(&self) -> DivnR {
        DivnR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 12:15 - Clock Divider Ratio for Radio"]
    #[inline(always)]
    pub fn divx(&self) -> DivxR {
        DivxR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Clock Divider Ratio for digital"]
    #[inline(always)]
    pub fn divy(&self) -> DivyR {
        DivyR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Feedback Divider Ratio"]
    #[inline(always)]
    pub fn divn(&mut self) -> DivnW<'_, Rffpllctrl1Spec> {
        DivnW::new(self, 0)
    }
    #[doc = "Bits 12:15 - Clock Divider Ratio for Radio"]
    #[inline(always)]
    pub fn divx(&mut self) -> DivxW<'_, Rffpllctrl1Spec> {
        DivxW::new(self, 12)
    }
    #[doc = "Bits 16:20 - Clock Divider Ratio for digital"]
    #[inline(always)]
    pub fn divy(&mut self) -> DivyW<'_, Rffpllctrl1Spec> {
        DivyW::new(self, 16)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rffpllctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rffpllctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rffpllctrl1Spec;
impl crate::RegisterSpec for Rffpllctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rffpllctrl1::R`](R) reader structure"]
impl crate::Readable for Rffpllctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`rffpllctrl1::W`](W) writer structure"]
impl crate::Writable for Rffpllctrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RFFPLLCTRL1 to value 0x1711_6057"]
impl crate::Resettable for Rffpllctrl1Spec {
    const RESET_VALUE: u32 = 0x1711_6057;
}
