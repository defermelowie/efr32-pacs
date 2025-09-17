#[doc = "Register `CH11_ILSRC` reader"]
pub type R = crate::R<Ch11IlsrcSpec>;
#[doc = "Register `CH11_ILSRC` writer"]
pub type W = crate::W<Ch11IlsrcSpec>;
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
    pub fn addr(&mut self) -> AddrW<'_, Ch11IlsrcSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11_ilsrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch11_ilsrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch11IlsrcSpec;
impl crate::RegisterSpec for Ch11IlsrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch11_ilsrc::R`](R) reader structure"]
impl crate::Readable for Ch11IlsrcSpec {}
#[doc = "`write(|w| ..)` method takes [`ch11_ilsrc::W`](W) writer structure"]
impl crate::Writable for Ch11IlsrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH11_ILSRC to value 0"]
impl crate::Resettable for Ch11IlsrcSpec {}
