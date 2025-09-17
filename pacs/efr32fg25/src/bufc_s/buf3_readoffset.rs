#[doc = "Register `BUF3_READOFFSET` reader"]
pub type R = crate::R<Buf3ReadoffsetSpec>;
#[doc = "Register `BUF3_READOFFSET` writer"]
pub type W = crate::W<Buf3ReadoffsetSpec>;
#[doc = "Field `READOFFSET` reader - Read Offset"]
pub type ReadoffsetR = crate::FieldReader<u16>;
#[doc = "Field `READOFFSET` writer - Read Offset"]
pub type ReadoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Read Offset"]
    #[inline(always)]
    pub fn readoffset(&self) -> ReadoffsetR {
        ReadoffsetR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Read Offset"]
    #[inline(always)]
    pub fn readoffset(&mut self) -> ReadoffsetW<'_, Buf3ReadoffsetSpec> {
        ReadoffsetW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`buf3_readoffset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf3_readoffset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buf3ReadoffsetSpec;
impl crate::RegisterSpec for Buf3ReadoffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf3_readoffset::R`](R) reader structure"]
impl crate::Readable for Buf3ReadoffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`buf3_readoffset::W`](W) writer structure"]
impl crate::Writable for Buf3ReadoffsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUF3_READOFFSET to value 0"]
impl crate::Resettable for Buf3ReadoffsetSpec {}
