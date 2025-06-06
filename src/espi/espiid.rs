#[doc = "Register `ESPIID` reader"]
pub type R = crate::R<EspiidSpec>;
#[doc = "Register `ESPIID` writer"]
pub type W = crate::W<EspiidSpec>;
#[doc = "Field `ID` reader - Host Version ID"]
pub type IdR = crate::FieldReader;
#[doc = "Field `ID` writer - Host Version ID"]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Host Version ID"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESPIID").field("id", &self.id()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Host Version ID"]
    #[inline(always)]
    pub fn id(&mut self) -> IdW<EspiidSpec> {
        IdW::new(self, 0)
    }
}
#[doc = "eSPI Identification Register (ESPIID)\n\nYou can [`read`](crate::Reg::read) this register and get [`espiid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`espiid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EspiidSpec;
impl crate::RegisterSpec for EspiidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`espiid::R`](R) reader structure"]
impl crate::Readable for EspiidSpec {}
#[doc = "`write(|w| ..)` method takes [`espiid::W`](W) writer structure"]
impl crate::Writable for EspiidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESPIID to value 0"]
impl crate::Resettable for EspiidSpec {
    const RESET_VALUE: u32 = 0;
}
