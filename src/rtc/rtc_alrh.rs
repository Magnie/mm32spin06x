#[doc = "Register `RTC_ALRH` reader"]
pub type R = crate::R<RtcAlrhSpec>;
#[doc = "Register `RTC_ALRH` writer"]
pub type W = crate::W<RtcAlrhSpec>;
#[doc = "Field `ALR` reader - RTC alarm high"]
pub type AlrR = crate::FieldReader<u16>;
#[doc = "Field `ALR` writer - RTC alarm high"]
pub type AlrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC alarm high"]
    #[inline(always)]
    pub fn alr(&self) -> AlrR {
        AlrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC alarm high"]
    #[inline(always)]
    #[must_use]
    pub fn alr(&mut self) -> AlrW<RtcAlrhSpec> {
        AlrW::new(self, 0)
    }
}
#[doc = "RTC alarm high register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_alrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_alrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcAlrhSpec;
impl crate::RegisterSpec for RtcAlrhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtc_alrh::R`](R) reader structure"]
impl crate::Readable for RtcAlrhSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_alrh::W`](W) writer structure"]
impl crate::Writable for RtcAlrhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RTC_ALRH to value 0xffff"]
impl crate::Resettable for RtcAlrhSpec {
    const RESET_VALUE: u16 = 0xffff;
}
