#[doc = "Register `CCR3` reader"]
pub type R = crate::R<Ccr3Spec>;
#[doc = "Register `CCR3` writer"]
pub type W = crate::W<Ccr3Spec>;
#[doc = "Field `CCR3` reader - Capture/Compare 3 value"]
pub type Ccr3R = crate::FieldReader<u32>;
#[doc = "Field `CCR3` writer - Capture/Compare 3 value"]
pub type Ccr3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Capture/Compare 3 value"]
    #[inline(always)]
    pub fn ccr3(&self) -> Ccr3R {
        Ccr3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Capture/Compare 3 value"]
    #[inline(always)]
    #[must_use]
    pub fn ccr3(&mut self) -> Ccr3W<Ccr3Spec> {
        Ccr3W::new(self, 0)
    }
}
#[doc = "capture/compare register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr3Spec;
impl crate::RegisterSpec for Ccr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr3::R`](R) reader structure"]
impl crate::Readable for Ccr3Spec {}
#[doc = "`write(|w| ..)` method takes [`ccr3::W`](W) writer structure"]
impl crate::Writable for Ccr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR3 to value 0"]
impl crate::Resettable for Ccr3Spec {
    const RESET_VALUE: u32 = 0;
}
