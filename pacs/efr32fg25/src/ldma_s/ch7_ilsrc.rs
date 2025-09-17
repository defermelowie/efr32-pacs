#[doc = "Register `CH7_ILSRC` reader"]
pub type R = crate::R<Ch7IlsrcSpec>;
#[doc = "Register `CH7_ILSRC` writer"]
pub type W = crate::W<Ch7IlsrcSpec>;
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
    pub fn addr(&mut self) -> AddrW<'_, Ch7IlsrcSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_ilsrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_ilsrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch7IlsrcSpec;
impl crate::RegisterSpec for Ch7IlsrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch7_ilsrc::R`](R) reader structure"]
impl crate::Readable for Ch7IlsrcSpec {}
#[doc = "`write(|w| ..)` method takes [`ch7_ilsrc::W`](W) writer structure"]
impl crate::Writable for Ch7IlsrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH7_ILSRC to value 0"]
impl crate::Resettable for Ch7IlsrcSpec {}
