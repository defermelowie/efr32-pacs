#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `CH0` reader - Channel"]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH0` writer - Channel"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` reader - Channel"]
pub type Ch1R = crate::BitReader;
#[doc = "Field `CH1` writer - Channel"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` reader - Channel"]
pub type Ch2R = crate::BitReader;
#[doc = "Field `CH2` writer - Channel"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` reader - Channel"]
pub type Ch3R = crate::BitReader;
#[doc = "Field `CH3` writer - Channel"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4` reader - Channel"]
pub type Ch4R = crate::BitReader;
#[doc = "Field `CH4` writer - Channel"]
pub type Ch4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5` reader - Channel"]
pub type Ch5R = crate::BitReader;
#[doc = "Field `CH5` writer - Channel"]
pub type Ch5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6` reader - Channel"]
pub type Ch6R = crate::BitReader;
#[doc = "Field `CH6` writer - Channel"]
pub type Ch6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7` reader - Channel"]
pub type Ch7R = crate::BitReader;
#[doc = "Field `CH7` writer - Channel"]
pub type Ch7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8` reader - Channel"]
pub type Ch8R = crate::BitReader;
#[doc = "Field `CH8` writer - Channel"]
pub type Ch8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9` reader - Channel"]
pub type Ch9R = crate::BitReader;
#[doc = "Field `CH9` writer - Channel"]
pub type Ch9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10` reader - Channel"]
pub type Ch10R = crate::BitReader;
#[doc = "Field `CH10` writer - Channel"]
pub type Ch10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11` reader - Channel"]
pub type Ch11R = crate::BitReader;
#[doc = "Field `CH11` writer - Channel"]
pub type Ch11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH12` reader - Channel"]
pub type Ch12R = crate::BitReader;
#[doc = "Field `CH12` writer - Channel"]
pub type Ch12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH13` reader - Channel"]
pub type Ch13R = crate::BitReader;
#[doc = "Field `CH13` writer - Channel"]
pub type Ch13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH14` reader - Channel"]
pub type Ch14R = crate::BitReader;
#[doc = "Field `CH14` writer - Channel"]
pub type Ch14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH15` reader - Channel"]
pub type Ch15R = crate::BitReader;
#[doc = "Field `CH15` writer - Channel"]
pub type Ch15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANDONE` reader - Scan Complete"]
pub type ScandoneR = crate::BitReader;
#[doc = "Field `SCANDONE` writer - Scan Complete"]
pub type ScandoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEC` reader - Decoder"]
pub type DecR = crate::BitReader;
#[doc = "Field `DEC` writer - Decoder"]
pub type DecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESWL` reader - Result Watermark Level"]
pub type ReswlR = crate::BitReader;
#[doc = "Field `RESWL` writer - Result Watermark Level"]
pub type ReswlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESOF` reader - Result Overflow"]
pub type ResofR = crate::BitReader;
#[doc = "Field `RESOF` writer - Result Overflow"]
pub type ResofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTOF` reader - Counter Overflow"]
pub type CntofR = crate::BitReader;
#[doc = "Field `CNTOF` writer - Counter Overflow"]
pub type CntofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUF` reader - Result Underflow"]
pub type ResufR = crate::BitReader;
#[doc = "Field `RESUF` writer - Result Underflow"]
pub type ResufW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel"]
    #[inline(always)]
    pub fn ch4(&self) -> Ch4R {
        Ch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel"]
    #[inline(always)]
    pub fn ch5(&self) -> Ch5R {
        Ch5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel"]
    #[inline(always)]
    pub fn ch6(&self) -> Ch6R {
        Ch6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel"]
    #[inline(always)]
    pub fn ch7(&self) -> Ch7R {
        Ch7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel"]
    #[inline(always)]
    pub fn ch8(&self) -> Ch8R {
        Ch8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel"]
    #[inline(always)]
    pub fn ch9(&self) -> Ch9R {
        Ch9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel"]
    #[inline(always)]
    pub fn ch10(&self) -> Ch10R {
        Ch10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel"]
    #[inline(always)]
    pub fn ch11(&self) -> Ch11R {
        Ch11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel"]
    #[inline(always)]
    pub fn ch12(&self) -> Ch12R {
        Ch12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel"]
    #[inline(always)]
    pub fn ch13(&self) -> Ch13R {
        Ch13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel"]
    #[inline(always)]
    pub fn ch14(&self) -> Ch14R {
        Ch14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel"]
    #[inline(always)]
    pub fn ch15(&self) -> Ch15R {
        Ch15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Scan Complete"]
    #[inline(always)]
    pub fn scandone(&self) -> ScandoneR {
        ScandoneR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Decoder"]
    #[inline(always)]
    pub fn dec(&self) -> DecR {
        DecR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Result Watermark Level"]
    #[inline(always)]
    pub fn reswl(&self) -> ReswlR {
        ReswlR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Result Overflow"]
    #[inline(always)]
    pub fn resof(&self) -> ResofR {
        ResofR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Counter Overflow"]
    #[inline(always)]
    pub fn cntof(&self) -> CntofR {
        CntofR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Result Underflow"]
    #[inline(always)]
    pub fn resuf(&self) -> ResufR {
        ResufR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel"]
    #[inline(always)]
    pub fn ch0(&mut self) -> Ch0W<'_, IenSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel"]
    #[inline(always)]
    pub fn ch1(&mut self) -> Ch1W<'_, IenSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel"]
    #[inline(always)]
    pub fn ch2(&mut self) -> Ch2W<'_, IenSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel"]
    #[inline(always)]
    pub fn ch3(&mut self) -> Ch3W<'_, IenSpec> {
        Ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel"]
    #[inline(always)]
    pub fn ch4(&mut self) -> Ch4W<'_, IenSpec> {
        Ch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel"]
    #[inline(always)]
    pub fn ch5(&mut self) -> Ch5W<'_, IenSpec> {
        Ch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel"]
    #[inline(always)]
    pub fn ch6(&mut self) -> Ch6W<'_, IenSpec> {
        Ch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel"]
    #[inline(always)]
    pub fn ch7(&mut self) -> Ch7W<'_, IenSpec> {
        Ch7W::new(self, 7)
    }
    #[doc = "Bit 8 - Channel"]
    #[inline(always)]
    pub fn ch8(&mut self) -> Ch8W<'_, IenSpec> {
        Ch8W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel"]
    #[inline(always)]
    pub fn ch9(&mut self) -> Ch9W<'_, IenSpec> {
        Ch9W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel"]
    #[inline(always)]
    pub fn ch10(&mut self) -> Ch10W<'_, IenSpec> {
        Ch10W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel"]
    #[inline(always)]
    pub fn ch11(&mut self) -> Ch11W<'_, IenSpec> {
        Ch11W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel"]
    #[inline(always)]
    pub fn ch12(&mut self) -> Ch12W<'_, IenSpec> {
        Ch12W::new(self, 12)
    }
    #[doc = "Bit 13 - Channel"]
    #[inline(always)]
    pub fn ch13(&mut self) -> Ch13W<'_, IenSpec> {
        Ch13W::new(self, 13)
    }
    #[doc = "Bit 14 - Channel"]
    #[inline(always)]
    pub fn ch14(&mut self) -> Ch14W<'_, IenSpec> {
        Ch14W::new(self, 14)
    }
    #[doc = "Bit 15 - Channel"]
    #[inline(always)]
    pub fn ch15(&mut self) -> Ch15W<'_, IenSpec> {
        Ch15W::new(self, 15)
    }
    #[doc = "Bit 16 - Scan Complete"]
    #[inline(always)]
    pub fn scandone(&mut self) -> ScandoneW<'_, IenSpec> {
        ScandoneW::new(self, 16)
    }
    #[doc = "Bit 17 - Decoder"]
    #[inline(always)]
    pub fn dec(&mut self) -> DecW<'_, IenSpec> {
        DecW::new(self, 17)
    }
    #[doc = "Bit 18 - Result Watermark Level"]
    #[inline(always)]
    pub fn reswl(&mut self) -> ReswlW<'_, IenSpec> {
        ReswlW::new(self, 18)
    }
    #[doc = "Bit 19 - Result Overflow"]
    #[inline(always)]
    pub fn resof(&mut self) -> ResofW<'_, IenSpec> {
        ResofW::new(self, 19)
    }
    #[doc = "Bit 20 - Counter Overflow"]
    #[inline(always)]
    pub fn cntof(&mut self) -> CntofW<'_, IenSpec> {
        CntofW::new(self, 20)
    }
    #[doc = "Bit 21 - Result Underflow"]
    #[inline(always)]
    pub fn resuf(&mut self) -> ResufW<'_, IenSpec> {
        ResufW::new(self, 21)
    }
}
#[doc = "Interrupt Enables\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
