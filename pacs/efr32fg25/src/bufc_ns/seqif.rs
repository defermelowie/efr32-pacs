#[doc = "Register `SEQIF` reader"]
pub type R = crate::R<SeqifSpec>;
#[doc = "Register `SEQIF` writer"]
pub type W = crate::W<SeqifSpec>;
#[doc = "Field `BUF0OF` reader - Buffer 0 Overflow"]
pub type Buf0ofR = crate::BitReader;
#[doc = "Field `BUF0OF` writer - Buffer 0 Overflow"]
pub type Buf0ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF0UF` reader - Buffer 0 Underflow"]
pub type Buf0ufR = crate::BitReader;
#[doc = "Field `BUF0UF` writer - Buffer 0 Underflow"]
pub type Buf0ufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF0THR` reader - Buffer 0 Threshold Event"]
pub type Buf0thrR = crate::BitReader;
#[doc = "Field `BUF0THR` writer - Buffer 0 Threshold Event"]
pub type Buf0thrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF0CORR` reader - Buffer 0 Corrupt"]
pub type Buf0corrR = crate::BitReader;
#[doc = "Field `BUF0CORR` writer - Buffer 0 Corrupt"]
pub type Buf0corrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF0NWA` reader - Buffer 0 Not Word-Aligned"]
pub type Buf0nwaR = crate::BitReader;
#[doc = "Field `BUF0NWA` writer - Buffer 0 Not Word-Aligned"]
pub type Buf0nwaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF1OF` reader - Buffer 1 Overflow"]
pub type Buf1ofR = crate::BitReader;
#[doc = "Field `BUF1OF` writer - Buffer 1 Overflow"]
pub type Buf1ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF1UF` reader - Buffer 1 Underflow"]
pub type Buf1ufR = crate::BitReader;
#[doc = "Field `BUF1UF` writer - Buffer 1 Underflow"]
pub type Buf1ufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF1THR` reader - Buffer 1 Threshold Event"]
pub type Buf1thrR = crate::BitReader;
#[doc = "Field `BUF1THR` writer - Buffer 1 Threshold Event"]
pub type Buf1thrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF1CORR` reader - Buffer 1 Corrupt"]
pub type Buf1corrR = crate::BitReader;
#[doc = "Field `BUF1CORR` writer - Buffer 1 Corrupt"]
pub type Buf1corrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF1NWA` reader - Buffer 1 Not Word-Aligned"]
pub type Buf1nwaR = crate::BitReader;
#[doc = "Field `BUF1NWA` writer - Buffer 1 Not Word-Aligned"]
pub type Buf1nwaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF2OF` reader - Buffer 2 Overflow"]
pub type Buf2ofR = crate::BitReader;
#[doc = "Field `BUF2OF` writer - Buffer 2 Overflow"]
pub type Buf2ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF2UF` reader - Buffer 2 Underflow"]
pub type Buf2ufR = crate::BitReader;
#[doc = "Field `BUF2UF` writer - Buffer 2 Underflow"]
pub type Buf2ufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF2THR` reader - Buffer 2 Threshold Event"]
pub type Buf2thrR = crate::BitReader;
#[doc = "Field `BUF2THR` writer - Buffer 2 Threshold Event"]
pub type Buf2thrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF2CORR` reader - Buffer 2 Corrupt"]
pub type Buf2corrR = crate::BitReader;
#[doc = "Field `BUF2CORR` writer - Buffer 2 Corrupt"]
pub type Buf2corrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF2NWA` reader - Buffer 2 Not Word-Aligned"]
pub type Buf2nwaR = crate::BitReader;
#[doc = "Field `BUF2NWA` writer - Buffer 2 Not Word-Aligned"]
pub type Buf2nwaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF3OF` reader - Buffer 3 Overflow"]
pub type Buf3ofR = crate::BitReader;
#[doc = "Field `BUF3OF` writer - Buffer 3 Overflow"]
pub type Buf3ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF3UF` reader - Buffer 3 Underflow"]
pub type Buf3ufR = crate::BitReader;
#[doc = "Field `BUF3UF` writer - Buffer 3 Underflow"]
pub type Buf3ufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF3THR` reader - Buffer 3 Threshold Event"]
pub type Buf3thrR = crate::BitReader;
#[doc = "Field `BUF3THR` writer - Buffer 3 Threshold Event"]
pub type Buf3thrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF3CORR` reader - Buffer 3 Corrupt"]
pub type Buf3corrR = crate::BitReader;
#[doc = "Field `BUF3CORR` writer - Buffer 3 Corrupt"]
pub type Buf3corrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF3NWA` reader - Buffer 3 Not Word-Aligned"]
pub type Buf3nwaR = crate::BitReader;
#[doc = "Field `BUF3NWA` writer - Buffer 3 Not Word-Aligned"]
pub type Buf3nwaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSERROR` reader - Bus Error"]
pub type BuserrorR = crate::BitReader;
#[doc = "Field `BUSERROR` writer - Bus Error"]
pub type BuserrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Buffer 0 Overflow"]
    #[inline(always)]
    pub fn buf0of(&self) -> Buf0ofR {
        Buf0ofR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Buffer 0 Underflow"]
    #[inline(always)]
    pub fn buf0uf(&self) -> Buf0ufR {
        Buf0ufR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Buffer 0 Threshold Event"]
    #[inline(always)]
    pub fn buf0thr(&self) -> Buf0thrR {
        Buf0thrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Buffer 0 Corrupt"]
    #[inline(always)]
    pub fn buf0corr(&self) -> Buf0corrR {
        Buf0corrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer 0 Not Word-Aligned"]
    #[inline(always)]
    pub fn buf0nwa(&self) -> Buf0nwaR {
        Buf0nwaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Buffer 1 Overflow"]
    #[inline(always)]
    pub fn buf1of(&self) -> Buf1ofR {
        Buf1ofR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Buffer 1 Underflow"]
    #[inline(always)]
    pub fn buf1uf(&self) -> Buf1ufR {
        Buf1ufR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Buffer 1 Threshold Event"]
    #[inline(always)]
    pub fn buf1thr(&self) -> Buf1thrR {
        Buf1thrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Buffer 1 Corrupt"]
    #[inline(always)]
    pub fn buf1corr(&self) -> Buf1corrR {
        Buf1corrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Buffer 1 Not Word-Aligned"]
    #[inline(always)]
    pub fn buf1nwa(&self) -> Buf1nwaR {
        Buf1nwaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Buffer 2 Overflow"]
    #[inline(always)]
    pub fn buf2of(&self) -> Buf2ofR {
        Buf2ofR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Buffer 2 Underflow"]
    #[inline(always)]
    pub fn buf2uf(&self) -> Buf2ufR {
        Buf2ufR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Buffer 2 Threshold Event"]
    #[inline(always)]
    pub fn buf2thr(&self) -> Buf2thrR {
        Buf2thrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Buffer 2 Corrupt"]
    #[inline(always)]
    pub fn buf2corr(&self) -> Buf2corrR {
        Buf2corrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Buffer 2 Not Word-Aligned"]
    #[inline(always)]
    pub fn buf2nwa(&self) -> Buf2nwaR {
        Buf2nwaR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Buffer 3 Overflow"]
    #[inline(always)]
    pub fn buf3of(&self) -> Buf3ofR {
        Buf3ofR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Buffer 3 Underflow"]
    #[inline(always)]
    pub fn buf3uf(&self) -> Buf3ufR {
        Buf3ufR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Buffer 3 Threshold Event"]
    #[inline(always)]
    pub fn buf3thr(&self) -> Buf3thrR {
        Buf3thrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Buffer 3 Corrupt"]
    #[inline(always)]
    pub fn buf3corr(&self) -> Buf3corrR {
        Buf3corrR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Buffer 3 Not Word-Aligned"]
    #[inline(always)]
    pub fn buf3nwa(&self) -> Buf3nwaR {
        Buf3nwaR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Bus Error"]
    #[inline(always)]
    pub fn buserror(&self) -> BuserrorR {
        BuserrorR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buffer 0 Overflow"]
    #[inline(always)]
    pub fn buf0of(&mut self) -> Buf0ofW<'_, SeqifSpec> {
        Buf0ofW::new(self, 0)
    }
    #[doc = "Bit 1 - Buffer 0 Underflow"]
    #[inline(always)]
    pub fn buf0uf(&mut self) -> Buf0ufW<'_, SeqifSpec> {
        Buf0ufW::new(self, 1)
    }
    #[doc = "Bit 2 - Buffer 0 Threshold Event"]
    #[inline(always)]
    pub fn buf0thr(&mut self) -> Buf0thrW<'_, SeqifSpec> {
        Buf0thrW::new(self, 2)
    }
    #[doc = "Bit 3 - Buffer 0 Corrupt"]
    #[inline(always)]
    pub fn buf0corr(&mut self) -> Buf0corrW<'_, SeqifSpec> {
        Buf0corrW::new(self, 3)
    }
    #[doc = "Bit 4 - Buffer 0 Not Word-Aligned"]
    #[inline(always)]
    pub fn buf0nwa(&mut self) -> Buf0nwaW<'_, SeqifSpec> {
        Buf0nwaW::new(self, 4)
    }
    #[doc = "Bit 8 - Buffer 1 Overflow"]
    #[inline(always)]
    pub fn buf1of(&mut self) -> Buf1ofW<'_, SeqifSpec> {
        Buf1ofW::new(self, 8)
    }
    #[doc = "Bit 9 - Buffer 1 Underflow"]
    #[inline(always)]
    pub fn buf1uf(&mut self) -> Buf1ufW<'_, SeqifSpec> {
        Buf1ufW::new(self, 9)
    }
    #[doc = "Bit 10 - Buffer 1 Threshold Event"]
    #[inline(always)]
    pub fn buf1thr(&mut self) -> Buf1thrW<'_, SeqifSpec> {
        Buf1thrW::new(self, 10)
    }
    #[doc = "Bit 11 - Buffer 1 Corrupt"]
    #[inline(always)]
    pub fn buf1corr(&mut self) -> Buf1corrW<'_, SeqifSpec> {
        Buf1corrW::new(self, 11)
    }
    #[doc = "Bit 12 - Buffer 1 Not Word-Aligned"]
    #[inline(always)]
    pub fn buf1nwa(&mut self) -> Buf1nwaW<'_, SeqifSpec> {
        Buf1nwaW::new(self, 12)
    }
    #[doc = "Bit 16 - Buffer 2 Overflow"]
    #[inline(always)]
    pub fn buf2of(&mut self) -> Buf2ofW<'_, SeqifSpec> {
        Buf2ofW::new(self, 16)
    }
    #[doc = "Bit 17 - Buffer 2 Underflow"]
    #[inline(always)]
    pub fn buf2uf(&mut self) -> Buf2ufW<'_, SeqifSpec> {
        Buf2ufW::new(self, 17)
    }
    #[doc = "Bit 18 - Buffer 2 Threshold Event"]
    #[inline(always)]
    pub fn buf2thr(&mut self) -> Buf2thrW<'_, SeqifSpec> {
        Buf2thrW::new(self, 18)
    }
    #[doc = "Bit 19 - Buffer 2 Corrupt"]
    #[inline(always)]
    pub fn buf2corr(&mut self) -> Buf2corrW<'_, SeqifSpec> {
        Buf2corrW::new(self, 19)
    }
    #[doc = "Bit 20 - Buffer 2 Not Word-Aligned"]
    #[inline(always)]
    pub fn buf2nwa(&mut self) -> Buf2nwaW<'_, SeqifSpec> {
        Buf2nwaW::new(self, 20)
    }
    #[doc = "Bit 24 - Buffer 3 Overflow"]
    #[inline(always)]
    pub fn buf3of(&mut self) -> Buf3ofW<'_, SeqifSpec> {
        Buf3ofW::new(self, 24)
    }
    #[doc = "Bit 25 - Buffer 3 Underflow"]
    #[inline(always)]
    pub fn buf3uf(&mut self) -> Buf3ufW<'_, SeqifSpec> {
        Buf3ufW::new(self, 25)
    }
    #[doc = "Bit 26 - Buffer 3 Threshold Event"]
    #[inline(always)]
    pub fn buf3thr(&mut self) -> Buf3thrW<'_, SeqifSpec> {
        Buf3thrW::new(self, 26)
    }
    #[doc = "Bit 27 - Buffer 3 Corrupt"]
    #[inline(always)]
    pub fn buf3corr(&mut self) -> Buf3corrW<'_, SeqifSpec> {
        Buf3corrW::new(self, 27)
    }
    #[doc = "Bit 28 - Buffer 3 Not Word-Aligned"]
    #[inline(always)]
    pub fn buf3nwa(&mut self) -> Buf3nwaW<'_, SeqifSpec> {
        Buf3nwaW::new(self, 28)
    }
    #[doc = "Bit 31 - Bus Error"]
    #[inline(always)]
    pub fn buserror(&mut self) -> BuserrorW<'_, SeqifSpec> {
        BuserrorW::new(self, 31)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`seqif::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqif::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeqifSpec;
impl crate::RegisterSpec for SeqifSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seqif::R`](R) reader structure"]
impl crate::Readable for SeqifSpec {}
#[doc = "`write(|w| ..)` method takes [`seqif::W`](W) writer structure"]
impl crate::Writable for SeqifSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEQIF to value 0"]
impl crate::Resettable for SeqifSpec {}
