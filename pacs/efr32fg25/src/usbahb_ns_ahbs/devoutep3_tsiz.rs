#[doc = "Register `DEVOUTEP3_TSIZ` reader"]
pub type R = crate::R<Devoutep3TsizSpec>;
#[doc = "Register `DEVOUTEP3_TSIZ` writer"]
pub type W = crate::W<Devoutep3TsizSpec>;
#[doc = "Field `XFERSIZE` reader - Transfer Size"]
pub type XfersizeR = crate::FieldReader<u32>;
#[doc = "Field `XFERSIZE` writer - Transfer Size"]
pub type XfersizeW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PKTCNT` reader - Packet Count"]
pub type PktcntR = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - Packet Count"]
pub type PktcntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RXDPIDSUPCNT` reader - rx Data PID/SETUP pkt cnt"]
pub type RxdpidsupcntR = crate::FieldReader;
#[doc = "Field `RXDPIDSUPCNT` writer - rx Data PID/SETUP pkt cnt"]
pub type RxdpidsupcntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&self) -> XfersizeR {
        XfersizeR::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PktcntR {
        PktcntR::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - rx Data PID/SETUP pkt cnt"]
    #[inline(always)]
    pub fn rxdpidsupcnt(&self) -> RxdpidsupcntR {
        RxdpidsupcntR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&mut self) -> XfersizeW<'_, Devoutep3TsizSpec> {
        XfersizeW::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PktcntW<'_, Devoutep3TsizSpec> {
        PktcntW::new(self, 19)
    }
    #[doc = "Bits 29:30 - rx Data PID/SETUP pkt cnt"]
    #[inline(always)]
    pub fn rxdpidsupcnt(&mut self) -> RxdpidsupcntW<'_, Devoutep3TsizSpec> {
        RxdpidsupcntW::new(self, 29)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`devoutep3_tsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devoutep3_tsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devoutep3TsizSpec;
impl crate::RegisterSpec for Devoutep3TsizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devoutep3_tsiz::R`](R) reader structure"]
impl crate::Readable for Devoutep3TsizSpec {}
#[doc = "`write(|w| ..)` method takes [`devoutep3_tsiz::W`](W) writer structure"]
impl crate::Writable for Devoutep3TsizSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVOUTEP3_TSIZ to value 0"]
impl crate::Resettable for Devoutep3TsizSpec {}
