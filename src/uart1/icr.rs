#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `TXICLR` writer - Transmit buffer empty interrupt clear bit"]
pub type TxiclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXICLR` writer - Receive interrupt clear bit"]
pub type RxiclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC_CLR` writer - Transmit complete interrupt clear bit"]
pub type TxcClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOERRCLR` writer - Receive overflow error interrupt clear bit"]
pub type RxoerrclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPERRCLR` writer - Parity error interrupt clear bit"]
pub type RxperrclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFERRCLR` writer - Frame error interrupt clear bit"]
pub type RxferrclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBRKCLR` writer - Receive frame break interrupt clear bit"]
pub type RxbrkclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBRK_CLR` writer - Transmit Break Frame Interrupt clear Bit"]
pub type TxbrkClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXB8_CLR` writer - Receive Bit 8 Interrupt clear Bit"]
pub type Rxb8ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIDLCLR` writer - Receive Bit 8 Interrupt clear Bit"]
pub type RxidlclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABREND_CLR` writer - Auto baud rate end interrupt clear bit"]
pub type AbrendClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABRERR_CLR` writer - Auto baud rate error interrupt clear bit"]
pub type AbrerrClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmit buffer empty interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn txiclr(&mut self) -> TxiclrW<IcrSpec> {
        TxiclrW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxiclr(&mut self) -> RxiclrW<IcrSpec> {
        RxiclrW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit complete interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn txc_clr(&mut self) -> TxcClrW<IcrSpec> {
        TxcClrW::new(self, 2)
    }
    #[doc = "Bit 3 - Receive overflow error interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxoerrclr(&mut self) -> RxoerrclrW<IcrSpec> {
        RxoerrclrW::new(self, 3)
    }
    #[doc = "Bit 4 - Parity error interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxperrclr(&mut self) -> RxperrclrW<IcrSpec> {
        RxperrclrW::new(self, 4)
    }
    #[doc = "Bit 5 - Frame error interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxferrclr(&mut self) -> RxferrclrW<IcrSpec> {
        RxferrclrW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive frame break interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxbrkclr(&mut self) -> RxbrkclrW<IcrSpec> {
        RxbrkclrW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit Break Frame Interrupt clear Bit"]
    #[inline(always)]
    #[must_use]
    pub fn txbrk_clr(&mut self) -> TxbrkClrW<IcrSpec> {
        TxbrkClrW::new(self, 7)
    }
    #[doc = "Bit 8 - Receive Bit 8 Interrupt clear Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxb8_clr(&mut self) -> Rxb8ClrW<IcrSpec> {
        Rxb8ClrW::new(self, 8)
    }
    #[doc = "Bit 9 - Receive Bit 8 Interrupt clear Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxidlclr(&mut self) -> RxidlclrW<IcrSpec> {
        RxidlclrW::new(self, 9)
    }
    #[doc = "Bit 10 - Auto baud rate end interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn abrend_clr(&mut self) -> AbrendClrW<IcrSpec> {
        AbrendClrW::new(self, 10)
    }
    #[doc = "Bit 11 - Auto baud rate error interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn abrerr_clr(&mut self) -> AbrerrClrW<IcrSpec> {
        AbrerrClrW::new(self, 11)
    }
}
#[doc = "Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {
    const RESET_VALUE: u32 = 0;
}
