#[doc = "Register `CMR_B` writer"]
pub type W = crate::W<CmrBSpec>;
#[doc = "Field `TR` writer - Transmission request"]
pub type TrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AT` writer - Abort transmission"]
pub type AtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRB` writer - Release receive buffer"]
pub type RrbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDO` writer - Clear data overrun"]
pub type CdoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERB` writer - Empty Rxbuffer"]
pub type ErbW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmission request"]
    #[inline(always)]
    #[must_use]
    pub fn tr(&mut self) -> TrW<CmrBSpec> {
        TrW::new(self, 0)
    }
    #[doc = "Bit 1 - Abort transmission"]
    #[inline(always)]
    #[must_use]
    pub fn at(&mut self) -> AtW<CmrBSpec> {
        AtW::new(self, 1)
    }
    #[doc = "Bit 2 - Release receive buffer"]
    #[inline(always)]
    #[must_use]
    pub fn rrb(&mut self) -> RrbW<CmrBSpec> {
        RrbW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear data overrun"]
    #[inline(always)]
    #[must_use]
    pub fn cdo(&mut self) -> CdoW<CmrBSpec> {
        CdoW::new(self, 3)
    }
    #[doc = "Bit 5 - Empty Rxbuffer"]
    #[inline(always)]
    #[must_use]
    pub fn erb(&mut self) -> ErbW<CmrBSpec> {
        ErbW::new(self, 5)
    }
}
#[doc = "Basic Command register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmr_b::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmrBSpec;
impl crate::RegisterSpec for CmrBSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmr_b::W`](W) writer structure"]
impl crate::Writable for CmrBSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMR_B to value 0xff"]
impl crate::Resettable for CmrBSpec {
    const RESET_VALUE: u32 = 0xff;
}
