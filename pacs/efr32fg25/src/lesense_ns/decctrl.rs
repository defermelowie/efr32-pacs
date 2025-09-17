#[doc = "Register `DECCTRL` reader"]
pub type R = crate::R<DecctrlSpec>;
#[doc = "Register `DECCTRL` writer"]
pub type W = crate::W<DecctrlSpec>;
#[doc = "Field `DECDIS` reader - Disable the decoder"]
pub type DecdisR = crate::BitReader;
#[doc = "Field `DECDIS` writer - Disable the decoder"]
pub type DecdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTMAP` reader - Enable decoder to channel interrupt map"]
pub type IntmapR = crate::BitReader;
#[doc = "Field `INTMAP` writer - Enable decoder to channel interrupt map"]
pub type IntmapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYSTPRS0` reader - Enable decoder hysteresis on PRS0 output"]
pub type Hystprs0R = crate::BitReader;
#[doc = "Field `HYSTPRS0` writer - Enable decoder hysteresis on PRS0 output"]
pub type Hystprs0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYSTPRS1` reader - Enable decoder hysteresis on PRS1 output"]
pub type Hystprs1R = crate::BitReader;
#[doc = "Field `HYSTPRS1` writer - Enable decoder hysteresis on PRS1 output"]
pub type Hystprs1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYSTPRS2` reader - Enable decoder hysteresis on PRS2 output"]
pub type Hystprs2R = crate::BitReader;
#[doc = "Field `HYSTPRS2` writer - Enable decoder hysteresis on PRS2 output"]
pub type Hystprs2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYSTIRQ` reader - Enable decoder hysteresis on interrupt r"]
pub type HystirqR = crate::BitReader;
#[doc = "Field `HYSTIRQ` writer - Enable decoder hysteresis on interrupt r"]
pub type HystirqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSCNT` reader - Enable count mode on decoder PRS channel"]
pub type PrscntR = crate::BitReader;
#[doc = "Field `PRSCNT` writer - Enable count mode on decoder PRS channel"]
pub type PrscntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Disable the decoder"]
    #[inline(always)]
    pub fn decdis(&self) -> DecdisR {
        DecdisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Enable decoder to channel interrupt map"]
    #[inline(always)]
    pub fn intmap(&self) -> IntmapR {
        IntmapR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable decoder hysteresis on PRS0 output"]
    #[inline(always)]
    pub fn hystprs0(&self) -> Hystprs0R {
        Hystprs0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable decoder hysteresis on PRS1 output"]
    #[inline(always)]
    pub fn hystprs1(&self) -> Hystprs1R {
        Hystprs1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable decoder hysteresis on PRS2 output"]
    #[inline(always)]
    pub fn hystprs2(&self) -> Hystprs2R {
        Hystprs2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable decoder hysteresis on interrupt r"]
    #[inline(always)]
    pub fn hystirq(&self) -> HystirqR {
        HystirqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable count mode on decoder PRS channel"]
    #[inline(always)]
    pub fn prscnt(&self) -> PrscntR {
        PrscntR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable the decoder"]
    #[inline(always)]
    pub fn decdis(&mut self) -> DecdisW<'_, DecctrlSpec> {
        DecdisW::new(self, 0)
    }
    #[doc = "Bit 2 - Enable decoder to channel interrupt map"]
    #[inline(always)]
    pub fn intmap(&mut self) -> IntmapW<'_, DecctrlSpec> {
        IntmapW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable decoder hysteresis on PRS0 output"]
    #[inline(always)]
    pub fn hystprs0(&mut self) -> Hystprs0W<'_, DecctrlSpec> {
        Hystprs0W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable decoder hysteresis on PRS1 output"]
    #[inline(always)]
    pub fn hystprs1(&mut self) -> Hystprs1W<'_, DecctrlSpec> {
        Hystprs1W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable decoder hysteresis on PRS2 output"]
    #[inline(always)]
    pub fn hystprs2(&mut self) -> Hystprs2W<'_, DecctrlSpec> {
        Hystprs2W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable decoder hysteresis on interrupt r"]
    #[inline(always)]
    pub fn hystirq(&mut self) -> HystirqW<'_, DecctrlSpec> {
        HystirqW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable count mode on decoder PRS channel"]
    #[inline(always)]
    pub fn prscnt(&mut self) -> PrscntW<'_, DecctrlSpec> {
        PrscntW::new(self, 7)
    }
}
#[doc = "Decoder control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`decctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`decctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DecctrlSpec;
impl crate::RegisterSpec for DecctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`decctrl::R`](R) reader structure"]
impl crate::Readable for DecctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`decctrl::W`](W) writer structure"]
impl crate::Writable for DecctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DECCTRL to value 0"]
impl crate::Resettable for DecctrlSpec {}
