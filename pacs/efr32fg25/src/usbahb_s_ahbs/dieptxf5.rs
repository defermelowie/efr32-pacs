#[doc = "Register `DIEPTXF5` reader"]
pub type R = crate::R<Dieptxf5Spec>;
#[doc = "Register `DIEPTXF5` writer"]
pub type W = crate::W<Dieptxf5Spec>;
#[doc = "Field `INEPNTXFSTADDR` reader - IN EP FIFOn xmit RAM Start Addr"]
pub type InepntxfstaddrR = crate::FieldReader<u16>;
#[doc = "Field `INEPNTXFSTADDR` writer - IN EP FIFOn xmit RAM Start Addr"]
pub type InepntxfstaddrW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `INEPNTXFDEP` reader - IN Endpoint TxFIFO Depth"]
pub type InepntxfdepR = crate::FieldReader<u16>;
#[doc = "Field `INEPNTXFDEP` writer - IN Endpoint TxFIFO Depth"]
pub type InepntxfdepW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:12 - IN EP FIFOn xmit RAM Start Addr"]
    #[inline(always)]
    pub fn inepntxfstaddr(&self) -> InepntxfstaddrR {
        InepntxfstaddrR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:25 - IN Endpoint TxFIFO Depth"]
    #[inline(always)]
    pub fn inepntxfdep(&self) -> InepntxfdepR {
        InepntxfdepR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - IN EP FIFOn xmit RAM Start Addr"]
    #[inline(always)]
    pub fn inepntxfstaddr(&mut self) -> InepntxfstaddrW<'_, Dieptxf5Spec> {
        InepntxfstaddrW::new(self, 0)
    }
    #[doc = "Bits 16:25 - IN Endpoint TxFIFO Depth"]
    #[inline(always)]
    pub fn inepntxfdep(&mut self) -> InepntxfdepW<'_, Dieptxf5Spec> {
        InepntxfdepW::new(self, 16)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dieptxf5Spec;
impl crate::RegisterSpec for Dieptxf5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptxf5::R`](R) reader structure"]
impl crate::Readable for Dieptxf5Spec {}
#[doc = "`write(|w| ..)` method takes [`dieptxf5::W`](W) writer structure"]
impl crate::Writable for Dieptxf5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPTXF5 to value 0x0300_1200"]
impl crate::Resettable for Dieptxf5Spec {
    const RESET_VALUE: u32 = 0x0300_1200;
}
