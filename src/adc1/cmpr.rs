#[doc = "Register `CMPR` reader"]
pub type R = crate::R<CmprSpec>;
#[doc = "Register `CMPR` writer"]
pub type W = crate::W<CmprSpec>;
#[doc = "Field `CMPLDATA` reader - Compare data low limit"]
pub type CmpldataR = crate::FieldReader<u16>;
#[doc = "Field `CMPLDATA` writer - Compare data low limit"]
pub type CmpldataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `CMPHDATA` reader - Compare data high limit"]
pub type CmphdataR = crate::FieldReader<u16>;
#[doc = "Field `CMPHDATA` writer - Compare data high limit"]
pub type CmphdataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Compare data low limit"]
    #[inline(always)]
    pub fn cmpldata(&self) -> CmpldataR {
        CmpldataR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Compare data high limit"]
    #[inline(always)]
    pub fn cmphdata(&self) -> CmphdataR {
        CmphdataR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Compare data low limit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpldata(&mut self) -> CmpldataW<CmprSpec> {
        CmpldataW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Compare data high limit"]
    #[inline(always)]
    #[must_use]
    pub fn cmphdata(&mut self) -> CmphdataW<CmprSpec> {
        CmphdataW::new(self, 16)
    }
}
#[doc = "Compare register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmprSpec;
impl crate::RegisterSpec for CmprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpr::R`](R) reader structure"]
impl crate::Readable for CmprSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpr::W`](W) writer structure"]
impl crate::Writable for CmprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMPR to value 0"]
impl crate::Resettable for CmprSpec {
    const RESET_VALUE: u32 = 0;
}
