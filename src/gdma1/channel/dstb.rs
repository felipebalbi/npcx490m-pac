#[doc = "Register `DSTB` reader"]
pub type R = crate::R<DstbSpec>;
#[doc = "Register `DSTB` writer"]
pub type W = crate::W<DstbSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Channel 0/1 Destination Base Address Register (GDMAn_DSTB0, GDMAn_DSTB1)\n\nYou can [`read`](crate::Reg::read) this register and get [`dstb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstbSpec;
impl crate::RegisterSpec for DstbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dstb::R`](R) reader structure"]
impl crate::Readable for DstbSpec {}
#[doc = "`write(|w| ..)` method takes [`dstb::W`](W) writer structure"]
impl crate::Writable for DstbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSTB to value 0"]
impl crate::Resettable for DstbSpec {
    const RESET_VALUE: u32 = 0;
}
