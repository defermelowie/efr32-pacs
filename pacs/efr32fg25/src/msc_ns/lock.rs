#[doc = "Register `LOCK` writer"]
pub type W = crate::W<LockSpec>;
#[doc = "Configuration Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Lockkey {
    #[doc = "0: LOCK"]
    Lock = 0,
    #[doc = "7025: UNLOCK"]
    Unlock = 7025,
}
impl From<Lockkey> for u16 {
    #[inline(always)]
    fn from(variant: Lockkey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lockkey {
    type Ux = u16;
}
impl crate::IsEnum for Lockkey {}
#[doc = "Field `LOCKKEY` writer - Configuration Lock"]
pub type LockkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, Lockkey>;
impl<'a, REG> LockkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "LOCK"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(Lockkey::Lock)
    }
    #[doc = "UNLOCK"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Lockkey::Unlock)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configuration Lock"]
    #[inline(always)]
    pub fn lockkey(&mut self) -> LockkeyW<'_, LockSpec> {
        LockkeyW::new(self, 0)
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
