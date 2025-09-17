#[doc = "Register `BUF3_ADDR` reader"]
pub type R = crate::R<Buf3AddrSpec>;
#[doc = "Register `BUF3_ADDR` writer"]
pub type W = crate::W<Buf3AddrSpec>;
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
    pub fn addr(&mut self) -> AddrW<'_, Buf3AddrSpec> {
        AddrW::new(self, 2)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf3_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf3_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buf3AddrSpec;
impl crate::RegisterSpec for Buf3AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf3_addr::R`](R) reader structure"]
impl crate::Readable for Buf3AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`buf3_addr::W`](W) writer structure"]
impl crate::Writable for Buf3AddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUF3_ADDR to value 0x2000_0000"]
impl crate::Resettable for Buf3AddrSpec {
    const RESET_VALUE: u32 = 0x2000_0000;
}
