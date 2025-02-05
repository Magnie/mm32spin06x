#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `ADEN` reader - ADC enable"]
pub type AdenR = crate::BitReader;
#[doc = "Field `ADEN` writer - ADC enable"]
pub type AdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADWEN` reader - ADC window comparison enable"]
pub type AdwenR = crate::BitReader;
#[doc = "Field `ADWEN` writer - ADC window comparison enable"]
pub type AdwenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEN` reader - Temperature sensor enable"]
pub type TsenR = crate::BitReader;
#[doc = "Field `TSEN` writer - Temperature sensor enable"]
pub type TsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSEN` reader - Reference voltage enable"]
pub type VsenR = crate::BitReader;
#[doc = "Field `VSEN` writer - Reference voltage enable"]
pub type VsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPREH` reader - ADC high prescaler coefficient"]
pub type AdcprehR = crate::FieldReader;
#[doc = "Field `ADCPREH` writer - ADC high prescaler coefficient"]
pub type AdcprehW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RSLTCTL` reader - Resolution"]
pub type RsltctlR = crate::FieldReader;
#[doc = "Field `RSLTCTL` writer - Resolution"]
pub type RsltctlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SAMCTL` reader - Channel x Sample time selection"]
pub type SamctlR = crate::FieldReader;
#[doc = "Field `SAMCTL` writer - Channel x Sample time selection"]
pub type SamctlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADCPREL` reader - ADC low prescaler coefficient"]
pub type AdcprelR = crate::BitReader;
#[doc = "Field `ADCPREL` writer - ADC low prescaler coefficient"]
pub type AdcprelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC enable"]
    #[inline(always)]
    pub fn aden(&self) -> AdenR {
        AdenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC window comparison enable"]
    #[inline(always)]
    pub fn adwen(&self) -> AdwenR {
        AdwenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Temperature sensor enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TsenR {
        TsenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reference voltage enable"]
    #[inline(always)]
    pub fn vsen(&self) -> VsenR {
        VsenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - ADC high prescaler coefficient"]
    #[inline(always)]
    pub fn adcpreh(&self) -> AdcprehR {
        AdcprehR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - Resolution"]
    #[inline(always)]
    pub fn rsltctl(&self) -> RsltctlR {
        RsltctlR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:13 - Channel x Sample time selection"]
    #[inline(always)]
    pub fn samctl(&self) -> SamctlR {
        SamctlR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - ADC low prescaler coefficient"]
    #[inline(always)]
    pub fn adcprel(&self) -> AdcprelR {
        AdcprelR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC enable"]
    #[inline(always)]
    #[must_use]
    pub fn aden(&mut self) -> AdenW<CfgrSpec> {
        AdenW::new(self, 0)
    }
    #[doc = "Bit 1 - ADC window comparison enable"]
    #[inline(always)]
    #[must_use]
    pub fn adwen(&mut self) -> AdwenW<CfgrSpec> {
        AdwenW::new(self, 1)
    }
    #[doc = "Bit 2 - Temperature sensor enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TsenW<CfgrSpec> {
        TsenW::new(self, 2)
    }
    #[doc = "Bit 3 - Reference voltage enable"]
    #[inline(always)]
    #[must_use]
    pub fn vsen(&mut self) -> VsenW<CfgrSpec> {
        VsenW::new(self, 3)
    }
    #[doc = "Bits 4:6 - ADC high prescaler coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn adcpreh(&mut self) -> AdcprehW<CfgrSpec> {
        AdcprehW::new(self, 4)
    }
    #[doc = "Bits 7:9 - Resolution"]
    #[inline(always)]
    #[must_use]
    pub fn rsltctl(&mut self) -> RsltctlW<CfgrSpec> {
        RsltctlW::new(self, 7)
    }
    #[doc = "Bits 10:13 - Channel x Sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn samctl(&mut self) -> SamctlW<CfgrSpec> {
        SamctlW::new(self, 10)
    }
    #[doc = "Bit 14 - ADC low prescaler coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn adcprel(&mut self) -> AdcprelW<CfgrSpec> {
        AdcprelW::new(self, 14)
    }
}
#[doc = "Configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CfgrSpec {
    const RESET_VALUE: u32 = 0;
}
