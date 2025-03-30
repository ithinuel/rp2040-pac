#[doc = "Register `TIMEHW` writer"]
pub type W = crate::W<TIMEHW_SPEC>;
#[doc = "Field `TIMEHW` writer - "]
pub type TIMEHW_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<TIMEHW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timehw(&mut self) -> TIMEHW_W<TIMEHW_SPEC> {
        TIMEHW_W::new(self, 0)
    }
}
#[doc = "Write to bits 63:32 of time  
 always write timelw before timehw  

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timehw::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMEHW_SPEC;
impl crate::RegisterSpec for TIMEHW_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`timehw::W`](W) writer structure"]
impl crate::Writable for TIMEHW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMEHW to value 0"]
impl crate::Resettable for TIMEHW_SPEC {}
