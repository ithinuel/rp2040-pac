#[doc = "Register `INTS` reader"]
pub type R = crate::R<INTS_SPEC>;
#[doc = "Field `ALARM_0` reader - "]
pub type ALARM_0_R = crate::BitReader;
#[doc = "Field `ALARM_1` reader - "]
pub type ALARM_1_R = crate::BitReader;
#[doc = "Field `ALARM_2` reader - "]
pub type ALARM_2_R = crate::BitReader;
#[doc = "Field `ALARM_3` reader - "]
pub type ALARM_3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn alarm_0(&self) -> ALARM_0_R {
        ALARM_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn alarm_1(&self) -> ALARM_1_R {
        ALARM_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn alarm_2(&self) -> ALARM_2_R {
        ALARM_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn alarm_3(&self) -> ALARM_3_R {
        ALARM_3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTS")
            .field("alarm_3", &self.alarm_3())
            .field("alarm_2", &self.alarm_2())
            .field("alarm_1", &self.alarm_1())
            .field("alarm_0", &self.alarm_0())
            .finish()
    }
}
#[doc = "Interrupt status after masking & forcing  

You can [`read`](crate::Reg::read) this register and get [`ints::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTS_SPEC;
impl crate::RegisterSpec for INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ints::R`](R) reader structure"]
impl crate::Readable for INTS_SPEC {}
#[doc = "`reset()` method sets INTS to value 0"]
impl crate::Resettable for INTS_SPEC {}
