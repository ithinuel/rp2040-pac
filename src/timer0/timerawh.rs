#[doc = "Register `TIMERAWH` reader"]
pub type R = crate::R<TIMERAWH_SPEC>;
#[doc = "Field `TIMERAWH` reader - "]
pub type TIMERAWH_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timerawh(&self) -> TIMERAWH_R {
        TIMERAWH_R::new(self.bits)
    }
}
#[doc = "Raw read from bits 63:32 of time (no side effects)  

You can [`read`](crate::Reg::read) this register and get [`timerawh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMERAWH_SPEC;
impl crate::RegisterSpec for TIMERAWH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timerawh::R`](R) reader structure"]
impl crate::Readable for TIMERAWH_SPEC {}
#[doc = "`reset()` method sets TIMERAWH to value 0"]
impl crate::Resettable for TIMERAWH_SPEC {}
