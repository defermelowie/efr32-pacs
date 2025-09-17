#[doc = "Register `ESAUMRB45` reader"]
pub type R = crate::R<Esaumrb45Spec>;
#[doc = "Register `ESAUMRB45` writer"]
pub type W = crate::W<Esaumrb45Spec>;
#[doc = "Field `ESAUMRB45` reader - Moveable Region Boundary"]
pub type Esaumrb45R = crate::FieldReader<u16>;
#[doc = "Field `ESAUMRB45` writer - Moveable Region Boundary"]
pub type Esaumrb45W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 12:27 - Moveable Region Boundary"]
    #[inline(always)]
    pub fn esaumrb45(&self) -> Esaumrb45R {
        Esaumrb45R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:27 - Moveable Region Boundary"]
    #[inline(always)]
    pub fn esaumrb45(&mut self) -> Esaumrb45W<'_, Esaumrb45Spec> {
        Esaumrb45W::new(self, 12)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`esaumrb45::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esaumrb45::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Esaumrb45Spec;
impl crate::RegisterSpec for Esaumrb45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esaumrb45::R`](R) reader structure"]
impl crate::Readable for Esaumrb45Spec {}
#[doc = "`write(|w| ..)` method takes [`esaumrb45::W`](W) writer structure"]
impl crate::Writable for Esaumrb45Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ESAUMRB45 to value 0x0200_0000"]
impl crate::Resettable for Esaumrb45Spec {
    const RESET_VALUE: u32 = 0x0200_0000;
}
