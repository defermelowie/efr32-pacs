#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `CHNLSTART0` writer - Start channel 0 tamper detection"]
pub type Chnlstart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNLSTOP0` writer - Stop channel 0 tamper detection"]
pub type Chnlstop0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNLLOAD0` writer - Start channel 0 tamper detection"]
pub type Chnlload0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNLSTART1` writer - Start channel 1 tamper detection"]
pub type Chnlstart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNLSTOP1` writer - Stop channel 1 tamper detection"]
pub type Chnlstop1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNLLOAD1` writer - Start channel 1 tamper detection"]
pub type Chnlload1W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Start channel 0 tamper detection"]
    #[inline(always)]
    pub fn chnlstart0(&mut self) -> Chnlstart0W<'_, CmdSpec> {
        Chnlstart0W::new(self, 0)
    }
    #[doc = "Bit 1 - Stop channel 0 tamper detection"]
    #[inline(always)]
    pub fn chnlstop0(&mut self) -> Chnlstop0W<'_, CmdSpec> {
        Chnlstop0W::new(self, 1)
    }
    #[doc = "Bit 2 - Start channel 0 tamper detection"]
    #[inline(always)]
    pub fn chnlload0(&mut self) -> Chnlload0W<'_, CmdSpec> {
        Chnlload0W::new(self, 2)
    }
    #[doc = "Bit 3 - Start channel 1 tamper detection"]
    #[inline(always)]
    pub fn chnlstart1(&mut self) -> Chnlstart1W<'_, CmdSpec> {
        Chnlstart1W::new(self, 3)
    }
    #[doc = "Bit 4 - Stop channel 1 tamper detection"]
    #[inline(always)]
    pub fn chnlstop1(&mut self) -> Chnlstop1W<'_, CmdSpec> {
        Chnlstop1W::new(self, 4)
    }
    #[doc = "Bit 5 - Start channel 1 tamper detection"]
    #[inline(always)]
    pub fn chnlload1(&mut self) -> Chnlload1W<'_, CmdSpec> {
        Chnlload1W::new(self, 5)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CmdSpec {}
