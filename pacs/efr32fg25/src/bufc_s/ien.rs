#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `BUF0OF` reader - BUF0OF Interrupt Enable"]
pub type Buf0ofR = crate::BitReader;
#[doc = "Field `BUF0OF` writer - BUF0OF Interrupt Enable"]
pub type Buf0ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF0UF` reader - BUF0UF Interrupt Enable"]
pub type Buf0ufR = crate::BitReader;
#[doc = "Field `BUF0UF` writer - BUF0UF Interrupt Enable"]
pub type Buf0ufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF0THR` reader - BUF0THR Interrupt Enable"]
pub type Buf0thrR = crate::BitReader;
#[doc = "Field `BUF0THR` writer - BUF0THR Interrupt Enable"]
pub type Buf0thrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF0CORR` reader - BUF0CORR Interrupt Enable"]
pub type Buf0corrR = crate::BitReader;
#[doc = "Field `BUF0CORR` writer - BUF0CORR Interrupt Enable"]
pub type Buf0corrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF0NWA` reader - BUF0NWA Interrupt Enable"]
pub type Buf0nwaR = crate::BitReader;
#[doc = "Field `BUF0NWA` writer - BUF0NWA Interrupt Enable"]
pub type Buf0nwaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF1OF` reader - BUF1OF Interrupt Enable"]
pub type Buf1ofR = crate::BitReader;
#[doc = "Field `BUF1OF` writer - BUF1OF Interrupt Enable"]
pub type Buf1ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF1UF` reader - BUF1UF Interrupt Enable"]
pub type Buf1ufR = crate::BitReader;
#[doc = "Field `BUF1UF` writer - BUF1UF Interrupt Enable"]
pub type Buf1ufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF1THR` reader - BUF1THR Interrupt Enable"]
pub type Buf1thrR = crate::BitReader;
#[doc = "Field `BUF1THR` writer - BUF1THR Interrupt Enable"]
pub type Buf1thrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF1CORR` reader - BUF1CORR Interrupt Enable"]
pub type Buf1corrR = crate::BitReader;
#[doc = "Field `BUF1CORR` writer - BUF1CORR Interrupt Enable"]
pub type Buf1corrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF1NWA` reader - BUF1NWA Interrupt Enable"]
pub type Buf1nwaR = crate::BitReader;
#[doc = "Field `BUF1NWA` writer - BUF1NWA Interrupt Enable"]
pub type Buf1nwaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF2OF` reader - BUF2OF Interrupt Enable"]
pub type Buf2ofR = crate::BitReader;
#[doc = "Field `BUF2OF` writer - BUF2OF Interrupt Enable"]
pub type Buf2ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF2UF` reader - BUF2UF Interrupt Enable"]
pub type Buf2ufR = crate::BitReader;
#[doc = "Field `BUF2UF` writer - BUF2UF Interrupt Enable"]
pub type Buf2ufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF2THR` reader - BUF2THR Interrupt Enable"]
pub type Buf2thrR = crate::BitReader;
#[doc = "Field `BUF2THR` writer - BUF2THR Interrupt Enable"]
pub type Buf2thrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF2CORR` reader - BUF2CORR Interrupt Enable"]
pub type Buf2corrR = crate::BitReader;
#[doc = "Field `BUF2CORR` writer - BUF2CORR Interrupt Enable"]
pub type Buf2corrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF2NWA` reader - BUF2NWA Interrupt Enable"]
pub type Buf2nwaR = crate::BitReader;
#[doc = "Field `BUF2NWA` writer - BUF2NWA Interrupt Enable"]
pub type Buf2nwaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF3OF` reader - BUF3OF Interrupt Enable"]
pub type Buf3ofR = crate::BitReader;
#[doc = "Field `BUF3OF` writer - BUF3OF Interrupt Enable"]
pub type Buf3ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF3UF` reader - BUF3UF Interrupt Enable"]
pub type Buf3ufR = crate::BitReader;
#[doc = "Field `BUF3UF` writer - BUF3UF Interrupt Enable"]
pub type Buf3ufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF3THR` reader - BUF3THR Interrupt Enable"]
pub type Buf3thrR = crate::BitReader;
#[doc = "Field `BUF3THR` writer - BUF3THR Interrupt Enable"]
pub type Buf3thrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF3CORR` reader - BUF3CORR Interrupt Enable"]
pub type Buf3corrR = crate::BitReader;
#[doc = "Field `BUF3CORR` writer - BUF3CORR Interrupt Enable"]
pub type Buf3corrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF3NWA` reader - BUF3NWA Interrupt Enable"]
pub type Buf3nwaR = crate::BitReader;
#[doc = "Field `BUF3NWA` writer - BUF3NWA Interrupt Enable"]
pub type Buf3nwaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSERROR` reader - BUSERROR Interrupt Enable"]
pub type BuserrorR = crate::BitReader;
#[doc = "Field `BUSERROR` writer - BUSERROR Interrupt Enable"]
pub type BuserrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BUF0OF Interrupt Enable"]
    #[inline(always)]
    pub fn buf0of(&self) -> Buf0ofR {
        Buf0ofR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BUF0UF Interrupt Enable"]
    #[inline(always)]
    pub fn buf0uf(&self) -> Buf0ufR {
        Buf0ufR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BUF0THR Interrupt Enable"]
    #[inline(always)]
    pub fn buf0thr(&self) -> Buf0thrR {
        Buf0thrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BUF0CORR Interrupt Enable"]
    #[inline(always)]
    pub fn buf0corr(&self) -> Buf0corrR {
        Buf0corrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BUF0NWA Interrupt Enable"]
    #[inline(always)]
    pub fn buf0nwa(&self) -> Buf0nwaR {
        Buf0nwaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - BUF1OF Interrupt Enable"]
    #[inline(always)]
    pub fn buf1of(&self) -> Buf1ofR {
        Buf1ofR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BUF1UF Interrupt Enable"]
    #[inline(always)]
    pub fn buf1uf(&self) -> Buf1ufR {
        Buf1ufR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BUF1THR Interrupt Enable"]
    #[inline(always)]
    pub fn buf1thr(&self) -> Buf1thrR {
        Buf1thrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BUF1CORR Interrupt Enable"]
    #[inline(always)]
    pub fn buf1corr(&self) -> Buf1corrR {
        Buf1corrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BUF1NWA Interrupt Enable"]
    #[inline(always)]
    pub fn buf1nwa(&self) -> Buf1nwaR {
        Buf1nwaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - BUF2OF Interrupt Enable"]
    #[inline(always)]
    pub fn buf2of(&self) -> Buf2ofR {
        Buf2ofR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - BUF2UF Interrupt Enable"]
    #[inline(always)]
    pub fn buf2uf(&self) -> Buf2ufR {
        Buf2ufR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - BUF2THR Interrupt Enable"]
    #[inline(always)]
    pub fn buf2thr(&self) -> Buf2thrR {
        Buf2thrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - BUF2CORR Interrupt Enable"]
    #[inline(always)]
    pub fn buf2corr(&self) -> Buf2corrR {
        Buf2corrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - BUF2NWA Interrupt Enable"]
    #[inline(always)]
    pub fn buf2nwa(&self) -> Buf2nwaR {
        Buf2nwaR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - BUF3OF Interrupt Enable"]
    #[inline(always)]
    pub fn buf3of(&self) -> Buf3ofR {
        Buf3ofR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - BUF3UF Interrupt Enable"]
    #[inline(always)]
    pub fn buf3uf(&self) -> Buf3ufR {
        Buf3ufR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - BUF3THR Interrupt Enable"]
    #[inline(always)]
    pub fn buf3thr(&self) -> Buf3thrR {
        Buf3thrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - BUF3CORR Interrupt Enable"]
    #[inline(always)]
    pub fn buf3corr(&self) -> Buf3corrR {
        Buf3corrR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - BUF3NWA Interrupt Enable"]
    #[inline(always)]
    pub fn buf3nwa(&self) -> Buf3nwaR {
        Buf3nwaR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - BUSERROR Interrupt Enable"]
    #[inline(always)]
    pub fn buserror(&self) -> BuserrorR {
        BuserrorR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BUF0OF Interrupt Enable"]
    #[inline(always)]
    pub fn buf0of(&mut self) -> Buf0ofW<'_, IenSpec> {
        Buf0ofW::new(self, 0)
    }
    #[doc = "Bit 1 - BUF0UF Interrupt Enable"]
    #[inline(always)]
    pub fn buf0uf(&mut self) -> Buf0ufW<'_, IenSpec> {
        Buf0ufW::new(self, 1)
    }
    #[doc = "Bit 2 - BUF0THR Interrupt Enable"]
    #[inline(always)]
    pub fn buf0thr(&mut self) -> Buf0thrW<'_, IenSpec> {
        Buf0thrW::new(self, 2)
    }
    #[doc = "Bit 3 - BUF0CORR Interrupt Enable"]
    #[inline(always)]
    pub fn buf0corr(&mut self) -> Buf0corrW<'_, IenSpec> {
        Buf0corrW::new(self, 3)
    }
    #[doc = "Bit 4 - BUF0NWA Interrupt Enable"]
    #[inline(always)]
    pub fn buf0nwa(&mut self) -> Buf0nwaW<'_, IenSpec> {
        Buf0nwaW::new(self, 4)
    }
    #[doc = "Bit 8 - BUF1OF Interrupt Enable"]
    #[inline(always)]
    pub fn buf1of(&mut self) -> Buf1ofW<'_, IenSpec> {
        Buf1ofW::new(self, 8)
    }
    #[doc = "Bit 9 - BUF1UF Interrupt Enable"]
    #[inline(always)]
    pub fn buf1uf(&mut self) -> Buf1ufW<'_, IenSpec> {
        Buf1ufW::new(self, 9)
    }
    #[doc = "Bit 10 - BUF1THR Interrupt Enable"]
    #[inline(always)]
    pub fn buf1thr(&mut self) -> Buf1thrW<'_, IenSpec> {
        Buf1thrW::new(self, 10)
    }
    #[doc = "Bit 11 - BUF1CORR Interrupt Enable"]
    #[inline(always)]
    pub fn buf1corr(&mut self) -> Buf1corrW<'_, IenSpec> {
        Buf1corrW::new(self, 11)
    }
    #[doc = "Bit 12 - BUF1NWA Interrupt Enable"]
    #[inline(always)]
    pub fn buf1nwa(&mut self) -> Buf1nwaW<'_, IenSpec> {
        Buf1nwaW::new(self, 12)
    }
    #[doc = "Bit 16 - BUF2OF Interrupt Enable"]
    #[inline(always)]
    pub fn buf2of(&mut self) -> Buf2ofW<'_, IenSpec> {
        Buf2ofW::new(self, 16)
    }
    #[doc = "Bit 17 - BUF2UF Interrupt Enable"]
    #[inline(always)]
    pub fn buf2uf(&mut self) -> Buf2ufW<'_, IenSpec> {
        Buf2ufW::new(self, 17)
    }
    #[doc = "Bit 18 - BUF2THR Interrupt Enable"]
    #[inline(always)]
    pub fn buf2thr(&mut self) -> Buf2thrW<'_, IenSpec> {
        Buf2thrW::new(self, 18)
    }
    #[doc = "Bit 19 - BUF2CORR Interrupt Enable"]
    #[inline(always)]
    pub fn buf2corr(&mut self) -> Buf2corrW<'_, IenSpec> {
        Buf2corrW::new(self, 19)
    }
    #[doc = "Bit 20 - BUF2NWA Interrupt Enable"]
    #[inline(always)]
    pub fn buf2nwa(&mut self) -> Buf2nwaW<'_, IenSpec> {
        Buf2nwaW::new(self, 20)
    }
    #[doc = "Bit 24 - BUF3OF Interrupt Enable"]
    #[inline(always)]
    pub fn buf3of(&mut self) -> Buf3ofW<'_, IenSpec> {
        Buf3ofW::new(self, 24)
    }
    #[doc = "Bit 25 - BUF3UF Interrupt Enable"]
    #[inline(always)]
    pub fn buf3uf(&mut self) -> Buf3ufW<'_, IenSpec> {
        Buf3ufW::new(self, 25)
    }
    #[doc = "Bit 26 - BUF3THR Interrupt Enable"]
    #[inline(always)]
    pub fn buf3thr(&mut self) -> Buf3thrW<'_, IenSpec> {
        Buf3thrW::new(self, 26)
    }
    #[doc = "Bit 27 - BUF3CORR Interrupt Enable"]
    #[inline(always)]
    pub fn buf3corr(&mut self) -> Buf3corrW<'_, IenSpec> {
        Buf3corrW::new(self, 27)
    }
    #[doc = "Bit 28 - BUF3NWA Interrupt Enable"]
    #[inline(always)]
    pub fn buf3nwa(&mut self) -> Buf3nwaW<'_, IenSpec> {
        Buf3nwaW::new(self, 28)
    }
    #[doc = "Bit 31 - BUSERROR Interrupt Enable"]
    #[inline(always)]
    pub fn buserror(&mut self) -> BuserrorW<'_, IenSpec> {
        BuserrorW::new(self, 31)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
