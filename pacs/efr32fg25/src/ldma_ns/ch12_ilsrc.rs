#[doc = "Register `CH12_ILSRC` reader"]
pub type R = crate::R<Ch12IlsrcSpec>;
#[doc = "Register `CH12_ILSRC` writer"]
pub type W = crate::W<Ch12IlsrcSpec>;
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
    pub fn addr(&mut self) -> AddrW<'_, Ch12IlsrcSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch12_ilsrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch12_ilsrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch12IlsrcSpec;
impl crate::RegisterSpec for Ch12IlsrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch12_ilsrc::R`](R) reader structure"]
impl crate::Readable for Ch12IlsrcSpec {}
#[doc = "`write(|w| ..)` method takes [`ch12_ilsrc::W`](W) writer structure"]
impl crate::Writable for Ch12IlsrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH12_ILSRC to value 0"]
impl crate::Resettable for Ch12IlsrcSpec {}
