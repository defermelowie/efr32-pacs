#[doc = "Register `CH13_REQSEL` reader"]
pub type R = crate::R<Ch13ReqselSpec>;
#[doc = "Register `CH13_REQSEL` writer"]
pub type W = crate::W<Ch13ReqselSpec>;
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
    pub fn sigsel(&mut self) -> SigselW<'_, Ch13ReqselSpec> {
        SigselW::new(self, 0)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&mut self) -> SourceselW<'_, Ch13ReqselSpec> {
        SourceselW::new(self, 16)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch13_reqsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch13_reqsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch13ReqselSpec;
impl crate::RegisterSpec for Ch13ReqselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch13_reqsel::R`](R) reader structure"]
impl crate::Readable for Ch13ReqselSpec {}
#[doc = "`write(|w| ..)` method takes [`ch13_reqsel::W`](W) writer structure"]
impl crate::Writable for Ch13ReqselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH13_REQSEL to value 0"]
impl crate::Resettable for Ch13ReqselSpec {}
