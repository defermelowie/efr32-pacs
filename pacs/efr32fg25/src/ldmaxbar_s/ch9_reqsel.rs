#[doc = "Register `CH9_REQSEL` reader"]
pub type R = crate::R<Ch9ReqselSpec>;
#[doc = "Register `CH9_REQSEL` writer"]
pub type W = crate::W<Ch9ReqselSpec>;
#[doc = "Field `SIGSEL` reader - Signal Select"]
pub type SigselR = crate::FieldReader;
#[doc = "Field `SIGSEL` writer - Signal Select"]
pub type SigselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SOURCESEL` reader - Source Select"]
pub type SourceselR = crate::FieldReader;
#[doc = "Field `SOURCESEL` writer - Source Select"]
pub type SourceselW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:3 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SigselR {
        SigselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&self) -> SourceselR {
        SourceselR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&mut self) -> SigselW<'_, Ch9ReqselSpec> {
        SigselW::new(self, 0)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&mut self) -> SourceselW<'_, Ch9ReqselSpec> {
        SourceselW::new(self, 16)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9_reqsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch9_reqsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch9ReqselSpec;
impl crate::RegisterSpec for Ch9ReqselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch9_reqsel::R`](R) reader structure"]
impl crate::Readable for Ch9ReqselSpec {}
#[doc = "`write(|w| ..)` method takes [`ch9_reqsel::W`](W) writer structure"]
impl crate::Writable for Ch9ReqselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH9_REQSEL to value 0"]
impl crate::Resettable for Ch9ReqselSpec {}
