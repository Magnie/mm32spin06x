#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `EXTI0` reader - EXTI 0 configuration"]
pub type Exti0R = crate::FieldReader;
#[doc = "Field `EXTI0` writer - EXTI 0 configuration"]
pub type Exti0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI1` reader - EXTI 1 configuration"]
pub type Exti1R = crate::FieldReader;
#[doc = "Field `EXTI1` writer - EXTI 1 configuration"]
pub type Exti1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI2` reader - EXTI 2 configuration"]
pub type Exti2R = crate::FieldReader;
#[doc = "Field `EXTI2` writer - EXTI 2 configuration"]
pub type Exti2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI3` reader - EXTI 3 configuration"]
pub type Exti3R = crate::FieldReader;
#[doc = "Field `EXTI3` writer - EXTI 3 configuration"]
pub type Exti3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - EXTI 0 configuration"]
    #[inline(always)]
    pub fn exti0(&self) -> Exti0R {
        Exti0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 1 configuration"]
    #[inline(always)]
    pub fn exti1(&self) -> Exti1R {
        Exti1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 2 configuration"]
    #[inline(always)]
    pub fn exti2(&self) -> Exti2R {
        Exti2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 3 configuration"]
    #[inline(always)]
    pub fn exti3(&self) -> Exti3R {
        Exti3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 0 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn exti0(&mut self) -> Exti0W<Cr1Spec> {
        Exti0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXTI 1 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn exti1(&mut self) -> Exti1W<Cr1Spec> {
        Exti1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXTI 2 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn exti2(&mut self) -> Exti2W<Cr1Spec> {
        Exti2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXTI 3 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn exti3(&mut self) -> Exti3W<Cr1Spec> {
        Exti3W::new(self, 12)
    }
}
#[doc = "External interrupt configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {
    const RESET_VALUE: u32 = 0;
}
