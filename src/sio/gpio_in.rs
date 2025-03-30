#[doc = "Register `GPIO_IN` reader"]
pub type R = crate::R<GPIO_IN_SPEC>;
#[doc = "Field `GPIO_IN` reader - Input value for GPIO0...29"]
pub type GPIO_IN_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29 - Input value for GPIO0...29"]
    #[inline(always)]
    pub fn gpio_in(&self) -> GPIO_IN_R {
        GPIO_IN_R::new(self.bits & 0x3fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_IN")
            .field("gpio_in", &self.gpio_in())
            .finish()
    }
}
#[doc = "Input value for GPIO pins  

You can [`read`](crate::Reg::read) this register and get [`gpio_in::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_IN_SPEC;
impl crate::RegisterSpec for GPIO_IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_in::R`](R) reader structure"]
impl crate::Readable for GPIO_IN_SPEC {}
#[doc = "`reset()` method sets GPIO_IN to value 0"]
impl crate::Resettable for GPIO_IN_SPEC {}
