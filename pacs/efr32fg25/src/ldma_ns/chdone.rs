#[doc = "Register `CHDONE` reader"]
pub type R = crate::R<ChdoneSpec>;
#[doc = "Register `CHDONE` writer"]
pub type W = crate::W<ChdoneSpec>;
#[doc = "Field `CHDONE0` reader - Channel Linking Done Status flag"]
pub type Chdone0R = crate::BitReader;
#[doc = "Field `CHDONE0` writer - Channel Linking Done Status flag"]
pub type Chdone0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDONE1` reader - Channel Linking Done Status flag"]
pub type Chdone1R = crate::BitReader;
#[doc = "Field `CHDONE1` writer - Channel Linking Done Status flag"]
pub type Chdone1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDONE2` reader - Channel Linking Done Status flag"]
pub type Chdone2R = crate::BitReader;
#[doc = "Field `CHDONE2` writer - Channel Linking Done Status flag"]
pub type Chdone2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDONE3` reader - Channel Linking Done Status flag"]
pub type Chdone3R = crate::BitReader;
#[doc = "Field `CHDONE3` writer - Channel Linking Done Status flag"]
pub type Chdone3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDONE4` reader - Channel Linking Done Status flag"]
pub type Chdone4R = crate::BitReader;
#[doc = "Field `CHDONE4` writer - Channel Linking Done Status flag"]
pub type Chdone4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDONE5` reader - Channel Linking Done Status flag"]
pub type Chdone5R = crate::BitReader;
#[doc = "Field `CHDONE5` writer - Channel Linking Done Status flag"]
pub type Chdone5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDONE6` reader - Channel Linking Done Status flag"]
pub type Chdone6R = crate::BitReader;
#[doc = "Field `CHDONE6` writer - Channel Linking Done Status flag"]
pub type Chdone6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDONE7` reader - Channel Linking Done Status flag"]
pub type Chdone7R = crate::BitReader;
#[doc = "Field `CHDONE7` writer - Channel Linking Done Status flag"]
pub type Chdone7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDONE8` reader - Channel Linking Done Status flag"]
pub type Chdone8R = crate::BitReader;
#[doc = "Field `CHDONE8` writer - Channel Linking Done Status flag"]
pub type Chdone8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDONE9` reader - Channel Linking Done Status flag"]
pub type Chdone9R = crate::BitReader;
#[doc = "Field `CHDONE9` writer - Channel Linking Done Status flag"]
pub type Chdone9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDONE10` reader - Channel Linking Done Status flag"]
pub type Chdone10R = crate::BitReader;
#[doc = "Field `CHDONE10` writer - Channel Linking Done Status flag"]
pub type Chdone10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDONE11` reader - Channel Linking Done Status flag"]
pub type Chdone11R = crate::BitReader;
#[doc = "Field `CHDONE11` writer - Channel Linking Done Status flag"]
pub type Chdone11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDONE12` reader - Channel Linking Done Status flag"]
pub type Chdone12R = crate::BitReader;
#[doc = "Field `CHDONE12` writer - Channel Linking Done Status flag"]
pub type Chdone12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDONE13` reader - Channel Linking Done Status flag"]
pub type Chdone13R = crate::BitReader;
#[doc = "Field `CHDONE13` writer - Channel Linking Done Status flag"]
pub type Chdone13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDONE14` reader - Channel Linking Done Status flag"]
pub type Chdone14R = crate::BitReader;
#[doc = "Field `CHDONE14` writer - Channel Linking Done Status flag"]
pub type Chdone14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDONE15` reader - Channel Linking Done Status flag"]
pub type Chdone15R = crate::BitReader;
#[doc = "Field `CHDONE15` writer - Channel Linking Done Status flag"]
pub type Chdone15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone0(&self) -> Chdone0R {
        Chdone0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone1(&self) -> Chdone1R {
        Chdone1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone2(&self) -> Chdone2R {
        Chdone2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone3(&self) -> Chdone3R {
        Chdone3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone4(&self) -> Chdone4R {
        Chdone4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone5(&self) -> Chdone5R {
        Chdone5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone6(&self) -> Chdone6R {
        Chdone6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone7(&self) -> Chdone7R {
        Chdone7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone8(&self) -> Chdone8R {
        Chdone8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone9(&self) -> Chdone9R {
        Chdone9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone10(&self) -> Chdone10R {
        Chdone10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone11(&self) -> Chdone11R {
        Chdone11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone12(&self) -> Chdone12R {
        Chdone12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone13(&self) -> Chdone13R {
        Chdone13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone14(&self) -> Chdone14R {
        Chdone14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone15(&self) -> Chdone15R {
        Chdone15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone0(&mut self) -> Chdone0W<'_, ChdoneSpec> {
        Chdone0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone1(&mut self) -> Chdone1W<'_, ChdoneSpec> {
        Chdone1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone2(&mut self) -> Chdone2W<'_, ChdoneSpec> {
        Chdone2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone3(&mut self) -> Chdone3W<'_, ChdoneSpec> {
        Chdone3W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone4(&mut self) -> Chdone4W<'_, ChdoneSpec> {
        Chdone4W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone5(&mut self) -> Chdone5W<'_, ChdoneSpec> {
        Chdone5W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone6(&mut self) -> Chdone6W<'_, ChdoneSpec> {
        Chdone6W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone7(&mut self) -> Chdone7W<'_, ChdoneSpec> {
        Chdone7W::new(self, 7)
    }
    #[doc = "Bit 8 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone8(&mut self) -> Chdone8W<'_, ChdoneSpec> {
        Chdone8W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone9(&mut self) -> Chdone9W<'_, ChdoneSpec> {
        Chdone9W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone10(&mut self) -> Chdone10W<'_, ChdoneSpec> {
        Chdone10W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone11(&mut self) -> Chdone11W<'_, ChdoneSpec> {
        Chdone11W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone12(&mut self) -> Chdone12W<'_, ChdoneSpec> {
        Chdone12W::new(self, 12)
    }
    #[doc = "Bit 13 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone13(&mut self) -> Chdone13W<'_, ChdoneSpec> {
        Chdone13W::new(self, 13)
    }
    #[doc = "Bit 14 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone14(&mut self) -> Chdone14W<'_, ChdoneSpec> {
        Chdone14W::new(self, 14)
    }
    #[doc = "Bit 15 - Channel Linking Done Status flag"]
    #[inline(always)]
    pub fn chdone15(&mut self) -> Chdone15W<'_, ChdoneSpec> {
        Chdone15W::new(self, 15)
    }
}
#[doc = "Channel Linking Done Status Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`chdone::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chdone::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChdoneSpec;
impl crate::RegisterSpec for ChdoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chdone::R`](R) reader structure"]
impl crate::Readable for ChdoneSpec {}
#[doc = "`write(|w| ..)` method takes [`chdone::W`](W) writer structure"]
impl crate::Writable for ChdoneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHDONE to value 0"]
impl crate::Resettable for ChdoneSpec {}
