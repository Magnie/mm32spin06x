#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `DBG_SLEEP` writer - Debug Sleep mode"]
pub type DbgSleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_STOP` writer - Debug Stop mode"]
pub type DbgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_STANDBY` writer - Debug Standby mode"]
pub type DbgStandbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_STOP_FOR_LDO` writer - Debug Stop Ldo"]
pub type DbgStopForLdoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_IWDG_STOP` writer - Debug independent watchdog stopped when core is stopped"]
pub type DbgIwdgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_WWDG_STOP` writer - Debug window watchdog when core is halted"]
pub type DbgWwdgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIMx_STOP` writer - TIMx counter stopped when core is halted (x = 3,2,1)"]
pub type DbgTimxStopW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DBG_CAN_STOP` writer - CAN stopped when core is halted"]
pub type DbgCanStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM1x_STOP` writer - TIMx counter stopped when core is halted (x = 16,17,14)"]
pub type DbgTim1xStopW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl W {
    #[doc = "Bit 0 - Debug Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_sleep(&mut self) -> DbgSleepW<CrSpec> {
        DbgSleepW::new(self, 0)
    }
    #[doc = "Bit 1 - Debug Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_stop(&mut self) -> DbgStopW<CrSpec> {
        DbgStopW::new(self, 1)
    }
    #[doc = "Bit 2 - Debug Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_standby(&mut self) -> DbgStandbyW<CrSpec> {
        DbgStandbyW::new(self, 2)
    }
    #[doc = "Bit 3 - Debug Stop Ldo"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_stop_for_ldo(&mut self) -> DbgStopForLdoW<CrSpec> {
        DbgStopForLdoW::new(self, 3)
    }
    #[doc = "Bit 8 - Debug independent watchdog stopped when core is stopped"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_iwdg_stop(&mut self) -> DbgIwdgStopW<CrSpec> {
        DbgIwdgStopW::new(self, 8)
    }
    #[doc = "Bit 9 - Debug window watchdog when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_wwdg_stop(&mut self) -> DbgWwdgStopW<CrSpec> {
        DbgWwdgStopW::new(self, 9)
    }
    #[doc = "Bits 10:12 - TIMx counter stopped when core is halted (x = 3,2,1)"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_timx_stop(&mut self) -> DbgTimxStopW<CrSpec> {
        DbgTimxStopW::new(self, 10)
    }
    #[doc = "Bit 14 - CAN stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_can_stop(&mut self) -> DbgCanStopW<CrSpec> {
        DbgCanStopW::new(self, 14)
    }
    #[doc = "Bits 16:18 - TIMx counter stopped when core is halted (x = 16,17,14)"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim1x_stop(&mut self) -> DbgTim1xStopW<CrSpec> {
        DbgTim1xStopW::new(self, 16)
    }
}
#[doc = "Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}
