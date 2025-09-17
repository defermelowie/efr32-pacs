#[doc = "Register `CH6_EVALTHRES` reader"]
pub type R = crate::R<Ch6EvalthresSpec>;
#[doc = "Register `CH6_EVALTHRES` writer"]
pub type W = crate::W<Ch6EvalthresSpec>;
#[doc = "Field `EVALTHRES` reader - Threshold"]
pub type EvalthresR = crate::FieldReader<u16>;
#[doc = "Field `EVALTHRES` writer - Threshold"]
pub type EvalthresW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Threshold"]
    #[inline(always)]
    pub fn evalthres(&self) -> EvalthresR {
        EvalthresR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Threshold"]
    #[inline(always)]
    pub fn evalthres(&mut self) -> EvalthresW<'_, Ch6EvalthresSpec> {
        EvalthresW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_evalthres::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_evalthres::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch6EvalthresSpec;
impl crate::RegisterSpec for Ch6EvalthresSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch6_evalthres::R`](R) reader structure"]
impl crate::Readable for Ch6EvalthresSpec {}
#[doc = "`write(|w| ..)` method takes [`ch6_evalthres::W`](W) writer structure"]
impl crate::Writable for Ch6EvalthresSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH6_EVALTHRES to value 0"]
impl crate::Resettable for Ch6EvalthresSpec {}
