#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `AHB0ERR1B` reader - AHB0 1-bit ECC Error Interrupt Enable"]
pub type Ahb0err1bR = crate::BitReader;
#[doc = "Field `AHB0ERR1B` writer - AHB0 1-bit ECC Error Interrupt Enable"]
pub type Ahb0err1bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB1ERR1B` reader - AHB1 1-bit ECC Error Interrupt Enable"]
pub type Ahb1err1bR = crate::BitReader;
#[doc = "Field `AHB1ERR1B` writer - AHB1 1-bit ECC Error Interrupt Enable"]
pub type Ahb1err1bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB0ERR2B` reader - AHB0 2-bit ECC Error Interrupt Enable"]
pub type Ahb0err2bR = crate::BitReader;
#[doc = "Field `AHB0ERR2B` writer - AHB0 2-bit ECC Error Interrupt Enable"]
pub type Ahb0err2bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB1ERR2B` reader - AHB1 2-bit ECC Error Interrupt Enable"]
pub type Ahb1err2bR = crate::BitReader;
#[doc = "Field `AHB1ERR2B` writer - AHB1 2-bit ECC Error Interrupt Enable"]
pub type Ahb1err2bW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AHB0 1-bit ECC Error Interrupt Enable"]
    #[inline(always)]
    pub fn ahb0err1b(&self) -> Ahb0err1bR {
        Ahb0err1bR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AHB1 1-bit ECC Error Interrupt Enable"]
    #[inline(always)]
    pub fn ahb1err1b(&self) -> Ahb1err1bR {
        Ahb1err1bR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB0 2-bit ECC Error Interrupt Enable"]
    #[inline(always)]
    pub fn ahb0err2b(&self) -> Ahb0err2bR {
        Ahb0err2bR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AHB1 2-bit ECC Error Interrupt Enable"]
    #[inline(always)]
    pub fn ahb1err2b(&self) -> Ahb1err2bR {
        Ahb1err2bR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AHB0 1-bit ECC Error Interrupt Enable"]
    #[inline(always)]
    pub fn ahb0err1b(&mut self) -> Ahb0err1bW<'_, IenSpec> {
        Ahb0err1bW::new(self, 0)
    }
    #[doc = "Bit 1 - AHB1 1-bit ECC Error Interrupt Enable"]
    #[inline(always)]
    pub fn ahb1err1b(&mut self) -> Ahb1err1bW<'_, IenSpec> {
        Ahb1err1bW::new(self, 1)
    }
    #[doc = "Bit 4 - AHB0 2-bit ECC Error Interrupt Enable"]
    #[inline(always)]
    pub fn ahb0err2b(&mut self) -> Ahb0err2bW<'_, IenSpec> {
        Ahb0err2bW::new(self, 4)
    }
    #[doc = "Bit 5 - AHB1 2-bit ECC Error Interrupt Enable"]
    #[inline(always)]
    pub fn ahb1err2b(&mut self) -> Ahb1err2bW<'_, IenSpec> {
        Ahb1err2bW::new(self, 5)
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
