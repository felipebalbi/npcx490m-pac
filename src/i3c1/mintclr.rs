#[doc = "Register `MINTCLR` reader"]
pub type R = crate::R<MintclrSpec>;
#[doc = "Register `MINTCLR` writer"]
pub type W = crate::W<MintclrSpec>;
#[doc = "Field `TGTSTART` reader - Target START Detected"]
pub type TgtstartR = crate::BitReader;
#[doc = "Field `TGTSTART` writer - Target START Detected"]
pub type TgtstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCTRLDONE` reader - MCTRL Done"]
pub type MctrldoneR = crate::BitReader;
#[doc = "Field `MCTRLDONE` writer - MCTRL Done"]
pub type MctrldoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPLETE` reader - Message Completed"]
pub type CompleteR = crate::BitReader;
#[doc = "Field `COMPLETE` writer - Message Completed"]
pub type CompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPEND` reader - Receive Buffer Pending"]
pub type RxpendR = crate::BitReader;
#[doc = "Field `RXPEND` writer - Receive Buffer Pending"]
pub type RxpendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXNOTFULL` reader - Transmit Buffer Not Full"]
pub type TxnotfullR = crate::BitReader;
#[doc = "Field `TXNOTFULL` writer - Transmit Buffer Not Full"]
pub type TxnotfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBIWON` reader - IBI Arbitration Won"]
pub type IbiwonR = crate::BitReader;
#[doc = "Field `IBIWON` writer - IBI Arbitration Won"]
pub type IbiwonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRWARN` reader - Error/Warning"]
pub type ErrwarnR = crate::BitReader;
#[doc = "Field `ERRWARN` writer - Error/Warning"]
pub type ErrwarnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOWCNTLR` reader - I3CI Now Bus Controller"]
pub type NowcntlrR = crate::BitReader;
#[doc = "Field `NOWCNTLR` writer - I3CI Now Bus Controller"]
pub type NowcntlrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - Target START Detected"]
    #[inline(always)]
    pub fn tgtstart(&self) -> TgtstartR {
        TgtstartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MCTRL Done"]
    #[inline(always)]
    pub fn mctrldone(&self) -> MctrldoneR {
        MctrldoneR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Message Completed"]
    #[inline(always)]
    pub fn complete(&self) -> CompleteR {
        CompleteR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receive Buffer Pending"]
    #[inline(always)]
    pub fn rxpend(&self) -> RxpendR {
        RxpendR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Buffer Not Full"]
    #[inline(always)]
    pub fn txnotfull(&self) -> TxnotfullR {
        TxnotfullR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - IBI Arbitration Won"]
    #[inline(always)]
    pub fn ibiwon(&self) -> IbiwonR {
        IbiwonR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Error/Warning"]
    #[inline(always)]
    pub fn errwarn(&self) -> ErrwarnR {
        ErrwarnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - I3CI Now Bus Controller"]
    #[inline(always)]
    pub fn nowcntlr(&self) -> NowcntlrR {
        NowcntlrR::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MINTCLR")
            .field("tgtstart", &self.tgtstart())
            .field("mctrldone", &self.mctrldone())
            .field("complete", &self.complete())
            .field("rxpend", &self.rxpend())
            .field("txnotfull", &self.txnotfull())
            .field("ibiwon", &self.ibiwon())
            .field("errwarn", &self.errwarn())
            .field("nowcntlr", &self.nowcntlr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - Target START Detected"]
    #[inline(always)]
    pub fn tgtstart(&mut self) -> TgtstartW<MintclrSpec> {
        TgtstartW::new(self, 8)
    }
    #[doc = "Bit 9 - MCTRL Done"]
    #[inline(always)]
    pub fn mctrldone(&mut self) -> MctrldoneW<MintclrSpec> {
        MctrldoneW::new(self, 9)
    }
    #[doc = "Bit 10 - Message Completed"]
    #[inline(always)]
    pub fn complete(&mut self) -> CompleteW<MintclrSpec> {
        CompleteW::new(self, 10)
    }
    #[doc = "Bit 11 - Receive Buffer Pending"]
    #[inline(always)]
    pub fn rxpend(&mut self) -> RxpendW<MintclrSpec> {
        RxpendW::new(self, 11)
    }
    #[doc = "Bit 12 - Transmit Buffer Not Full"]
    #[inline(always)]
    pub fn txnotfull(&mut self) -> TxnotfullW<MintclrSpec> {
        TxnotfullW::new(self, 12)
    }
    #[doc = "Bit 13 - IBI Arbitration Won"]
    #[inline(always)]
    pub fn ibiwon(&mut self) -> IbiwonW<MintclrSpec> {
        IbiwonW::new(self, 13)
    }
    #[doc = "Bit 15 - Error/Warning"]
    #[inline(always)]
    pub fn errwarn(&mut self) -> ErrwarnW<MintclrSpec> {
        ErrwarnW::new(self, 15)
    }
    #[doc = "Bit 19 - I3CI Now Bus Controller"]
    #[inline(always)]
    pub fn nowcntlr(&mut self) -> NowcntlrW<MintclrSpec> {
        NowcntlrW::new(self, 19)
    }
}
#[doc = "Controller Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mintclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mintclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MintclrSpec;
impl crate::RegisterSpec for MintclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mintclr::R`](R) reader structure"]
impl crate::Readable for MintclrSpec {}
#[doc = "`write(|w| ..)` method takes [`mintclr::W`](W) writer structure"]
impl crate::Writable for MintclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MINTCLR to value 0"]
impl crate::Resettable for MintclrSpec {
    const RESET_VALUE: u32 = 0;
}
