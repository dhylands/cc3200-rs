// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate cc3200_sys;

use self::cc3200_sys::{board_init, GPIO_IF_LedConfigure, GPIO_IF_LedOn, GPIO_IF_LedOff, MAP_UtilsDelay};

#[allow(non_camel_case_types)]
pub enum LedName {
    NO_LED_IND = 0,
    MCU_SENDING_DATA_IND,
    MCU_ASSOCIATED_IND,       // Device associated to an AP
    MCU_IP_ALLOC_IND,         // Device acquired an IP
    MCU_SERVER_INIT_IND,      // Device connected to remote server
    MCU_CLIENT_CONNECTED_IND, // Any client connects to device
    MCU_ON_IND,               // Device SLHost invoked successfully
    MCU_EXECUTE_SUCCESS_IND,  // Task executed sucessfully
    MCU_EXECUTE_FAIL_IND,     // Task execution failed
    MCU_RED_LED_GPIO,	      // GP09 for LED RED as per LP 3.0
    MCU_ORANGE_LED_GPIO,      // GP10 for LED ORANGE as per LP 3.0
    MCU_GREEN_LED_GPIO,       // GP11 for LED GREEN as per LP 3.0
    MCU_ALL_LED_IND
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum LedEnum {
    NO_LED = 0x0,
    LED1   = 0x1, // RED LED D7/GP9/Pin64
    LED2   = 0x2, // ORANGE LED D6/GP10/Pin1
    LED3   = 0x4, // GREEN LED D5/GP11/Pin2
}

pub struct Board { }

impl Board {
    pub fn init() {
        unsafe {
            board_init();
        }
    }

    pub fn led_configure(leds: &[LedEnum]) {
        let mut val = LedEnum::NO_LED as u8;
        for led in leds {
            val |= *led as u8;
        }
        unsafe { GPIO_IF_LedConfigure(val); }
    }

    pub fn led_off(led: LedName) {
        unsafe {
            GPIO_IF_LedOff(led as i8);
        }
    }

    pub fn led_on(led: LedName) {
        unsafe {
            GPIO_IF_LedOn(led as i8);
        }
    }
}

pub struct Utils { }

impl Utils {
    pub fn delay(loops: u32) {
        unsafe {
            MAP_UtilsDelay(loops);
        }
    }
}