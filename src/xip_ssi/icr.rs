#[doc = "Register `ICR` reader"]
pub type R = crate::R<ICR_SPEC>;
#[doc = "Field `ICR` reader - Clear-on-read all active interrupts"]
pub type ICR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Clear-on-read all active interrupts"]
    #[inline(always)]
    pub fn icr(&self) -> ICR_R {
        ICR_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR").field("icr", &self.icr()).finish()
    }
}
#[doc = "Interrupt clear  

You can [`read`](crate::Reg::read) this register and get [`icr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for ICR_SPEC {}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {}
