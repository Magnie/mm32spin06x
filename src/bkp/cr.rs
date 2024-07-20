#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `TPE` reader - TAMPER pin enable"]
pub type TpeR = crate::BitReader;
#[doc = "Field `TPE` writer - TAMPER pin enable"]
pub type TpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPAL` reader - TAMPER pin active level"]
pub type TpalR = crate::BitReader;
#[doc = "Field `TPAL` writer - TAMPER pin active level"]
pub type TpalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TAMPER pin enable"]
    #[inline(always)]
    pub fn tpe(&self) -> TpeR {
        TpeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMPER pin active level"]
    #[inline(always)]
    pub fn tpal(&self) -> TpalR {
        TpalR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TAMPER pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpe(&mut self) -> TpeW<CrSpec> {
        TpeW::new(self, 0)
    }
    #[doc = "Bit 1 - TAMPER pin active level"]
    #[inline(always)]
    #[must_use]
    pub fn tpal(&mut self) -> TpalW<CrSpec> {
        TpalW::new(self, 1)
    }
}
#[doc = "Backup control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u16 = 0;
}
