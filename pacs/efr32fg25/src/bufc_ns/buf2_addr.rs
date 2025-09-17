#[doc = "Register `BUF2_ADDR` reader"]
pub type R = crate::R<Buf2AddrSpec>;
#[doc = "Register `BUF2_ADDR` writer"]
pub type W = crate::W<Buf2AddrSpec>;
#[doc = "Field `ADDR` reader - Buffer Address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Buffer Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Buffer Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Buffer Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, Buf2AddrSpec> {
        AddrW::new(self, 2)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf2_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf2_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buf2AddrSpec;
impl crate::RegisterSpec for Buf2AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf2_addr::R`](R) reader structure"]
impl crate::Readable for Buf2AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`buf2_addr::W`](W) writer structure"]
impl crate::Writable for Buf2AddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUF2_ADDR to value 0x2000_0000"]
impl crate::Resettable for Buf2AddrSpec {
    const RESET_VALUE: u32 = 0x2000_0000;
}
