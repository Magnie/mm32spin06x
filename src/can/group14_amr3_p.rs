#[doc = "Register `GROUP14_AMR3_P` reader"]
pub type R = crate::R<Group14Amr3PSpec>;
#[doc = "Register `GROUP14_AMR3_P` writer"]
pub type W = crate::W<Group14Amr3PSpec>;
#[doc = "Field `AM` reader - Acceptance mask"]
pub type AmR = crate::FieldReader;
#[doc = "Field `AM` writer - Acceptance mask"]
pub type AmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Acceptance mask"]
    #[inline(always)]
    pub fn am(&self) -> AmR {
        AmR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Acceptance mask"]
    #[inline(always)]
    #[must_use]
    pub fn am(&mut self) -> AmW<Group14Amr3PSpec> {
        AmW::new(self, 0)
    }
}
#[doc = "Peli Acceptance Mask register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`group14_amr3_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`group14_amr3_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Group14Amr3PSpec;
impl crate::RegisterSpec for Group14Amr3PSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`group14_amr3_p::R`](R) reader structure"]
impl crate::Readable for Group14Amr3PSpec {}
#[doc = "`write(|w| ..)` method takes [`group14_amr3_p::W`](W) writer structure"]
impl crate::Writable for Group14Amr3PSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GROUP14_AMR3_P to value 0"]
impl crate::Resettable for Group14Amr3PSpec {
    const RESET_VALUE: u32 = 0;
}
