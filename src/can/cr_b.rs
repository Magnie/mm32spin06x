#[doc = "Register `CR_B` reader"]
pub type R = crate::R<CrBSpec>;
#[doc = "Register `CR_B` writer"]
pub type W = crate::W<CrBSpec>;
#[doc = "Field `RR` reader - Reset request"]
pub type RrR = crate::BitReader;
#[doc = "Field `RR` writer - Reset request"]
pub type RrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIE` reader - Receive interrupt enable"]
pub type RieR = crate::BitReader;
#[doc = "Field `RIE` writer - Receive interrupt enable"]
pub type RieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE` reader - Transmit interrupt enable"]
pub type TieR = crate::BitReader;
#[doc = "Field `TIE` writer - Transmit interrupt enable"]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIE` reader - Error interrupt enable"]
pub type EieR = crate::BitReader;
#[doc = "Field `EIE` writer - Error interrupt enable"]
pub type EieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIE` reader - Overflow interrupt enable"]
pub type OieR = crate::BitReader;
#[doc = "Field `OIE` writer - Overflow interrupt enable"]
pub type OieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reset request"]
    #[inline(always)]
    pub fn rr(&self) -> RrR {
        RrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EieR {
        EieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn oie(&self) -> OieR {
        OieR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset request"]
    #[inline(always)]
    #[must_use]
    pub fn rr(&mut self) -> RrW<CrBSpec> {
        RrW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RieW<CrBSpec> {
        RieW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TieW<CrBSpec> {
        TieW::new(self, 2)
    }
    #[doc = "Bit 3 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eie(&mut self) -> EieW<CrBSpec> {
        EieW::new(self, 3)
    }
    #[doc = "Bit 4 - Overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn oie(&mut self) -> OieW<CrBSpec> {
        OieW::new(self, 4)
    }
}
#[doc = "Basic control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrBSpec;
impl crate::RegisterSpec for CrBSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_b::R`](R) reader structure"]
impl crate::Readable for CrBSpec {}
#[doc = "`write(|w| ..)` method takes [`cr_b::W`](W) writer structure"]
impl crate::Writable for CrBSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR_B to value 0x21"]
impl crate::Resettable for CrBSpec {
    const RESET_VALUE: u32 = 0x21;
}
