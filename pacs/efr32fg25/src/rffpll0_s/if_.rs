#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `RFFPLLRADIORDY` reader - Radio Output Ready Interrupt"]
pub type RffpllradiordyR = crate::BitReader;
#[doc = "Field `RFFPLLRADIORDY` writer - Radio Output Ready Interrupt"]
pub type RffpllradiordyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFFPLLSYSRDY` reader - Digital System Output Ready Interrupt"]
pub type RffpllsysrdyR = crate::BitReader;
#[doc = "Field `RFFPLLSYSRDY` writer - Digital System Output Ready Interrupt"]
pub type RffpllsysrdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Radio Output Ready Interrupt"]
    #[inline(always)]
    pub fn rffpllradiordy(&self) -> RffpllradiordyR {
        RffpllradiordyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Digital System Output Ready Interrupt"]
    #[inline(always)]
    pub fn rffpllsysrdy(&self) -> RffpllsysrdyR {
        RffpllsysrdyR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Radio Output Ready Interrupt"]
    #[inline(always)]
    pub fn rffpllradiordy(&mut self) -> RffpllradiordyW<'_, IfSpec> {
        RffpllradiordyW::new(self, 0)
    }
    #[doc = "Bit 1 - Digital System Output Ready Interrupt"]
    #[inline(always)]
    pub fn rffpllsysrdy(&mut self) -> RffpllsysrdyW<'_, IfSpec> {
        RffpllsysrdyW::new(self, 1)
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
