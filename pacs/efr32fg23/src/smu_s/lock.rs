#[doc = "Register `LOCK` writer"]
pub type W = crate::W<LockSpec>;
#[doc = "No Description\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Smulockkey {
    #[doc = "11325013: Unlocks Registers"]
    Unlock = 11325013,
}
impl From<Smulockkey> for u32 {
    #[inline(always)]
    fn from(variant: Smulockkey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Smulockkey {
    type Ux = u32;
}
impl crate::IsEnum for Smulockkey {}
#[doc = "Field `SMULOCKKEY` writer - No Description"]
pub type SmulockkeyW<'a, REG> = crate::FieldWriter<'a, REG, 24, Smulockkey>;
impl<'a, REG> SmulockkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Unlocks Registers"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Smulockkey::Unlock)
    }
}
impl W {
    #[doc = "Bits 0:23 - No Description"]
    #[inline(always)]
    pub fn smulockkey(&mut self) -> SmulockkeyW<'_, LockSpec> {
        SmulockkeyW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockSpec;
impl crate::RegisterSpec for LockSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lock::W`](W) writer structure"]
impl crate::Writable for LockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LockSpec {}
