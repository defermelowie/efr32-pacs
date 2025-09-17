#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `DONE0` reader - Done Interrupt Enable"]
pub type Done0R = crate::BitReader;
#[doc = "Field `DONE0` writer - Done Interrupt Enable"]
pub type Done0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE1` reader - Done Interrupt Enable"]
pub type Done1R = crate::BitReader;
#[doc = "Field `DONE1` writer - Done Interrupt Enable"]
pub type Done1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE2` reader - Done Interrupt Enable"]
pub type Done2R = crate::BitReader;
#[doc = "Field `DONE2` writer - Done Interrupt Enable"]
pub type Done2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE3` reader - Done Interrupt Enable"]
pub type Done3R = crate::BitReader;
#[doc = "Field `DONE3` writer - Done Interrupt Enable"]
pub type Done3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE4` reader - Done Interrupt Enable"]
pub type Done4R = crate::BitReader;
#[doc = "Field `DONE4` writer - Done Interrupt Enable"]
pub type Done4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE5` reader - Done Interrupt Enable"]
pub type Done5R = crate::BitReader;
#[doc = "Field `DONE5` writer - Done Interrupt Enable"]
pub type Done5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE6` reader - Done Interrupt Enable"]
pub type Done6R = crate::BitReader;
#[doc = "Field `DONE6` writer - Done Interrupt Enable"]
pub type Done6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE7` reader - Done Interrupt Enable"]
pub type Done7R = crate::BitReader;
#[doc = "Field `DONE7` writer - Done Interrupt Enable"]
pub type Done7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE8` reader - Done Interrupt Enable"]
pub type Done8R = crate::BitReader;
#[doc = "Field `DONE8` writer - Done Interrupt Enable"]
pub type Done8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE9` reader - Done Interrupt Enable"]
pub type Done9R = crate::BitReader;
#[doc = "Field `DONE9` writer - Done Interrupt Enable"]
pub type Done9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE10` reader - Done Interrupt Enable"]
pub type Done10R = crate::BitReader;
#[doc = "Field `DONE10` writer - Done Interrupt Enable"]
pub type Done10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE11` reader - Done Interrupt Enable"]
pub type Done11R = crate::BitReader;
#[doc = "Field `DONE11` writer - Done Interrupt Enable"]
pub type Done11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE12` reader - Done Interrupt Enable"]
pub type Done12R = crate::BitReader;
#[doc = "Field `DONE12` writer - Done Interrupt Enable"]
pub type Done12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE13` reader - Done Interrupt Enable"]
pub type Done13R = crate::BitReader;
#[doc = "Field `DONE13` writer - Done Interrupt Enable"]
pub type Done13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE14` reader - Done Interrupt Enable"]
pub type Done14R = crate::BitReader;
#[doc = "Field `DONE14` writer - Done Interrupt Enable"]
pub type Done14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE15` reader - Done Interrupt Enable"]
pub type Done15R = crate::BitReader;
#[doc = "Field `DONE15` writer - Done Interrupt Enable"]
pub type Done15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR` reader - Error Interrupt Enable"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Error Interrupt Enable"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done0(&self) -> Done0R {
        Done0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done1(&self) -> Done1R {
        Done1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done2(&self) -> Done2R {
        Done2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done3(&self) -> Done3R {
        Done3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done4(&self) -> Done4R {
        Done4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done5(&self) -> Done5R {
        Done5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done6(&self) -> Done6R {
        Done6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done7(&self) -> Done7R {
        Done7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done8(&self) -> Done8R {
        Done8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done9(&self) -> Done9R {
        Done9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done10(&self) -> Done10R {
        Done10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done11(&self) -> Done11R {
        Done11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done12(&self) -> Done12R {
        Done12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done13(&self) -> Done13R {
        Done13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done14(&self) -> Done14R {
        Done14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done15(&self) -> Done15R {
        Done15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done0(&mut self) -> Done0W<'_, IenSpec> {
        Done0W::new(self, 0)
    }
    #[doc = "Bit 1 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done1(&mut self) -> Done1W<'_, IenSpec> {
        Done1W::new(self, 1)
    }
    #[doc = "Bit 2 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done2(&mut self) -> Done2W<'_, IenSpec> {
        Done2W::new(self, 2)
    }
    #[doc = "Bit 3 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done3(&mut self) -> Done3W<'_, IenSpec> {
        Done3W::new(self, 3)
    }
    #[doc = "Bit 4 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done4(&mut self) -> Done4W<'_, IenSpec> {
        Done4W::new(self, 4)
    }
    #[doc = "Bit 5 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done5(&mut self) -> Done5W<'_, IenSpec> {
        Done5W::new(self, 5)
    }
    #[doc = "Bit 6 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done6(&mut self) -> Done6W<'_, IenSpec> {
        Done6W::new(self, 6)
    }
    #[doc = "Bit 7 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done7(&mut self) -> Done7W<'_, IenSpec> {
        Done7W::new(self, 7)
    }
    #[doc = "Bit 8 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done8(&mut self) -> Done8W<'_, IenSpec> {
        Done8W::new(self, 8)
    }
    #[doc = "Bit 9 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done9(&mut self) -> Done9W<'_, IenSpec> {
        Done9W::new(self, 9)
    }
    #[doc = "Bit 10 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done10(&mut self) -> Done10W<'_, IenSpec> {
        Done10W::new(self, 10)
    }
    #[doc = "Bit 11 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done11(&mut self) -> Done11W<'_, IenSpec> {
        Done11W::new(self, 11)
    }
    #[doc = "Bit 12 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done12(&mut self) -> Done12W<'_, IenSpec> {
        Done12W::new(self, 12)
    }
    #[doc = "Bit 13 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done13(&mut self) -> Done13W<'_, IenSpec> {
        Done13W::new(self, 13)
    }
    #[doc = "Bit 14 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done14(&mut self) -> Done14W<'_, IenSpec> {
        Done14W::new(self, 14)
    }
    #[doc = "Bit 15 - Done Interrupt Enable"]
    #[inline(always)]
    pub fn done15(&mut self) -> Done15W<'_, IenSpec> {
        Done15W::new(self, 15)
    }
    #[doc = "Bit 31 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, IenSpec> {
        ErrorW::new(self, 31)
    }
}
#[doc = "Done Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
