#[doc = "Register `GOTGCTL` reader"]
pub type R = crate::R<GotgctlSpec>;
#[doc = "Register `GOTGCTL` writer"]
pub type W = crate::W<GotgctlSpec>;
#[doc = "Field `CURMOD` reader - Current Mode of Operation"]
pub type CurmodR = crate::BitReader;
impl R {
    #[doc = "Bit 21 - Current Mode of Operation"]
    #[inline(always)]
    pub fn curmod(&self) -> CurmodR {
        CurmodR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`gotgctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GotgctlSpec;
impl crate::RegisterSpec for GotgctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gotgctl::R`](R) reader structure"]
impl crate::Readable for GotgctlSpec {}
#[doc = "`write(|w| ..)` method takes [`gotgctl::W`](W) writer structure"]
impl crate::Writable for GotgctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GOTGCTL to value 0x0001_0000"]
impl crate::Resettable for GotgctlSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
