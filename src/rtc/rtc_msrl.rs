#[doc = "Register `RTC_MSRL` reader"]
pub type R = crate::R<RtcMsrlSpec>;
#[doc = "Register `RTC_MSRL` writer"]
pub type W = crate::W<RtcMsrlSpec>;
#[doc = "Field `MSR` reader - RTC msec low"]
pub type MsrR = crate::FieldReader<u16>;
#[doc = "Field `MSR` writer - RTC msec low"]
pub type MsrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC msec low"]
    #[inline(always)]
    pub fn msr(&self) -> MsrR {
        MsrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC msec low"]
    #[inline(always)]
    #[must_use]
    pub fn msr(&mut self) -> MsrW<RtcMsrlSpec> {
        MsrW::new(self, 0)
    }
}
#[doc = "RTC millisecond alarm low register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_msrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_msrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcMsrlSpec;
impl crate::RegisterSpec for RtcMsrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtc_msrl::R`](R) reader structure"]
impl crate::Readable for RtcMsrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_msrl::W`](W) writer structure"]
impl crate::Writable for RtcMsrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RTC_MSRL to value 0"]
impl crate::Resettable for RtcMsrlSpec {
    const RESET_VALUE: u16 = 0;
}
