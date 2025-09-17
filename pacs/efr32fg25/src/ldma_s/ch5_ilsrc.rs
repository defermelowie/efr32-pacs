#[doc = "Register `CH5_ILSRC` reader"]
pub type R = crate::R<Ch5IlsrcSpec>;
#[doc = "Register `CH5_ILSRC` writer"]
pub type W = crate::W<Ch5IlsrcSpec>;
#[doc = "Field `ADDR` reader - Interleave Source Address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Interleave Source Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interleave Source Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interleave Source Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, Ch5IlsrcSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_ilsrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_ilsrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch5IlsrcSpec;
impl crate::RegisterSpec for Ch5IlsrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch5_ilsrc::R`](R) reader structure"]
impl crate::Readable for Ch5IlsrcSpec {}
#[doc = "`write(|w| ..)` method takes [`ch5_ilsrc::W`](W) writer structure"]
impl crate::Writable for Ch5IlsrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH5_ILSRC to value 0"]
impl crate::Resettable for Ch5IlsrcSpec {}
