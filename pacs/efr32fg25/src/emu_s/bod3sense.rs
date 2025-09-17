#[doc = "Register `BOD3SENSE` reader"]
pub type R = crate::R<Bod3senseSpec>;
#[doc = "Register `BOD3SENSE` writer"]
pub type W = crate::W<Bod3senseSpec>;
#[doc = "Field `AVDDBODEN` reader - AVDD BOD enable"]
pub type AvddbodenR = crate::BitReader;
#[doc = "Field `AVDDBODEN` writer - AVDD BOD enable"]
pub type AvddbodenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOVDD0BODEN` reader - IOVDD0 BOD enable"]
pub type Iovdd0bodenR = crate::BitReader;
#[doc = "Field `IOVDD0BODEN` writer - IOVDD0 BOD enable"]
pub type Iovdd0bodenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOVDD1BODEN` reader - IOVDD1 BOD enable"]
pub type Iovdd1bodenR = crate::BitReader;
#[doc = "Field `IOVDD1BODEN` writer - IOVDD1 BOD enable"]
pub type Iovdd1bodenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOVDD2BODEN` reader - IOVDD2 BOD enable"]
pub type Iovdd2bodenR = crate::BitReader;
#[doc = "Field `IOVDD2BODEN` writer - IOVDD2 BOD enable"]
pub type Iovdd2bodenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AVDD BOD enable"]
    #[inline(always)]
    pub fn avddboden(&self) -> AvddbodenR {
        AvddbodenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IOVDD0 BOD enable"]
    #[inline(always)]
    pub fn iovdd0boden(&self) -> Iovdd0bodenR {
        Iovdd0bodenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IOVDD1 BOD enable"]
    #[inline(always)]
    pub fn iovdd1boden(&self) -> Iovdd1bodenR {
        Iovdd1bodenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IOVDD2 BOD enable"]
    #[inline(always)]
    pub fn iovdd2boden(&self) -> Iovdd2bodenR {
        Iovdd2bodenR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AVDD BOD enable"]
    #[inline(always)]
    pub fn avddboden(&mut self) -> AvddbodenW<'_, Bod3senseSpec> {
        AvddbodenW::new(self, 0)
    }
    #[doc = "Bit 1 - IOVDD0 BOD enable"]
    #[inline(always)]
    pub fn iovdd0boden(&mut self) -> Iovdd0bodenW<'_, Bod3senseSpec> {
        Iovdd0bodenW::new(self, 1)
    }
    #[doc = "Bit 2 - IOVDD1 BOD enable"]
    #[inline(always)]
    pub fn iovdd1boden(&mut self) -> Iovdd1bodenW<'_, Bod3senseSpec> {
        Iovdd1bodenW::new(self, 2)
    }
    #[doc = "Bit 3 - IOVDD2 BOD enable"]
    #[inline(always)]
    pub fn iovdd2boden(&mut self) -> Iovdd2bodenW<'_, Bod3senseSpec> {
        Iovdd2bodenW::new(self, 3)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`bod3sense::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bod3sense::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bod3senseSpec;
impl crate::RegisterSpec for Bod3senseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bod3sense::R`](R) reader structure"]
impl crate::Readable for Bod3senseSpec {}
#[doc = "`write(|w| ..)` method takes [`bod3sense::W`](W) writer structure"]
impl crate::Writable for Bod3senseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BOD3SENSE to value 0"]
impl crate::Resettable for Bod3senseSpec {}
