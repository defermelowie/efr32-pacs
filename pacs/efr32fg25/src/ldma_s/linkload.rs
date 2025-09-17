#[doc = "Register `LINKLOAD` writer"]
pub type W = crate::W<LinkloadSpec>;
#[doc = "Field `LINKLOAD` writer - DMA Link Loads"]
pub type LinkloadW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - DMA Link Loads"]
    #[inline(always)]
    pub fn linkload(&mut self) -> LinkloadW<'_, LinkloadSpec> {
        LinkloadW::new(self, 0)
    }
}
#[doc = "Channel Link Load Register (Writes will only take effect when EN=1)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linkload::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LinkloadSpec;
impl crate::RegisterSpec for LinkloadSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`linkload::W`](W) writer structure"]
impl crate::Writable for LinkloadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LINKLOAD to value 0"]
impl crate::Resettable for LinkloadSpec {}
