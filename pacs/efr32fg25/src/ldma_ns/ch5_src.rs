#[doc = "Register `CH5_SRC` reader"]
pub type R = crate::R<Ch5SrcSpec>;
#[doc = "Register `CH5_SRC` writer"]
pub type W = crate::W<Ch5SrcSpec>;
#[doc = "Field `ADDR` reader - Source Data Address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Source Data Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source Data Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Data Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, Ch5SrcSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch5SrcSpec;
impl crate::RegisterSpec for Ch5SrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch5_src::R`](R) reader structure"]
impl crate::Readable for Ch5SrcSpec {}
#[doc = "`write(|w| ..)` method takes [`ch5_src::W`](W) writer structure"]
impl crate::Writable for Ch5SrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH5_SRC to value 0"]
impl crate::Resettable for Ch5SrcSpec {}
