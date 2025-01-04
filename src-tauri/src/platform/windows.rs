use windows::Win32::Foundation::POINT;
use windows::Win32::UI::WindowsAndMessaging::{
    GetCursorPos, SetCursorPos, SystemParametersInfoW, SPIF_SENDCHANGE,
    SPI_GETACTIVEWINDOWTRACKING, SPI_GETACTIVEWNDTRKTIMEOUT, SPI_SETACTIVEWINDOWTRACKING,
    SPI_SETACTIVEWNDTRKTIMEOUT,
};

use super::FocusControllerTrait;

pub struct FocusController {
    orgin_window_tracking_config: bool,
    orgin_window_tracking_delay_config: u64,
}

impl FocusControllerTrait for FocusController {
    fn initialize(&mut self) {
        self.orgin_window_tracking_config = FocusController::get_window_tracking_config();
        self.orgin_window_tracking_delay_config =
            FocusController::get_window_tracking_delay_config();

        FocusController::set_window_tracking_config(true);
        FocusController::set_window_tracking_delay_config(100);
    }
    fn finalize(&mut self) {
        FocusController::set_window_tracking_config(self.orgin_window_tracking_config);
        FocusController::set_window_tracking_delay_config(self.orgin_window_tracking_delay_config);
    }

    fn focus_on(&mut self) {
        if FocusController::get_window_tracking_config() {
            return;
        }
        FocusController::set_window_tracking_config(true);
    }
    fn focus_off(&mut self) {
        if !FocusController::get_window_tracking_config() {
            return;
        }
        FocusController::set_window_tracking_config(false);
    }
}

impl FocusController {
    pub fn new() -> Self {
        Self {
            orgin_window_tracking_config: false,
            orgin_window_tracking_delay_config: 0,
        }
    }

    fn get_window_tracking_config() -> bool {
        let mut enable = 0;

        let ret = unsafe {
            SystemParametersInfoW(
                SPI_GETACTIVEWINDOWTRACKING,
                0,
                Some(&mut enable as *mut i32 as *mut core::ffi::c_void),
                SPIF_SENDCHANGE,
            )
        };

        match ret {
            Ok(_) => {}
            Err(_) => {
                panic!("Failed to get window tracking");
            }
        }

        enable == 1
    }
    fn set_window_tracking_config(enable: bool) {
        let enable = if enable { 1 } else { 0 };

        let ret = unsafe {
            SystemParametersInfoW(
                SPI_SETACTIVEWINDOWTRACKING,
                0,
                Some(enable as *mut core::ffi::c_void),
                SPIF_SENDCHANGE,
            )
        };
        match ret {
            Ok(_) => {}
            Err(_) => {
                panic!("Failed to set window tracking");
            }
        }
    }

    fn get_window_tracking_delay_config() -> u64 {
        let mut delay = 0;

        let ret = unsafe {
            SystemParametersInfoW(
                SPI_GETACTIVEWNDTRKTIMEOUT,
                0,
                Some(&mut delay as *mut i32 as *mut core::ffi::c_void),
                SPIF_SENDCHANGE,
            )
        };

        match ret {
            Ok(_) => {}
            Err(_) => {
                panic!("Failed to get window tracking delay");
            }
        }

        delay as u64
    }
    fn set_window_tracking_delay_config(delay_msec: u64) {
        let ret = unsafe {
            SystemParametersInfoW(
                SPI_SETACTIVEWNDTRKTIMEOUT,
                0,
                Some(delay_msec as *mut i32 as *mut core::ffi::c_void),
                SPIF_SENDCHANGE,
            )
        };
        match ret {
            Ok(_) => {}
            Err(_) => {
                panic!("Failed to set window tracking delay");
            }
        }
    }

    fn get_cursor_pos() -> POINT {
        let mut pos = POINT { x: 0, y: 0 };

        let ret = unsafe { GetCursorPos(&mut pos) };
        match ret {
            Ok(_) => {}
            Err(_) => {
                panic!("Failed to get cursor position");
            }
        }

        pos
    }
    fn set_cursor_pos(pos: POINT) {
        let ret = unsafe { SetCursorPos(pos.x, pos.y) };
        match ret {
            Ok(_) => {}
            Err(_) => {
                panic!("Failed to set cursor position");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_get_window_traking_config() {
        FocusController::set_window_tracking_config(false);
        std::thread::sleep(std::time::Duration::from_millis(300));
        assert!(!FocusController::get_window_tracking_config());

        FocusController::set_window_tracking_config(true);
        std::thread::sleep(std::time::Duration::from_millis(300));
        assert!(FocusController::get_window_tracking_config());
    }

    #[test]
    fn set_and_get_window_tracking_delay_config() {
        FocusController::set_window_tracking_delay_config(0);
        std::thread::sleep(std::time::Duration::from_millis(300));
        assert_eq!(FocusController::get_window_tracking_delay_config(), 0);

        FocusController::set_window_tracking_delay_config(100);
        std::thread::sleep(std::time::Duration::from_millis(300));
        assert_eq!(FocusController::get_window_tracking_delay_config(), 100);
    }

    #[test]
    fn set_and_get_cursor_pos() {
        let org_pos = FocusController::get_cursor_pos();

        FocusController::set_cursor_pos(POINT { x: 100, y: 100 });
        std::thread::sleep(std::time::Duration::from_millis(300));
        let pos = FocusController::get_cursor_pos();
        assert_eq!(pos, POINT { x: 100, y: 100 });

        FocusController::set_cursor_pos(org_pos);
    }

    #[test]
    fn initalize() {
        let mut fc = FocusController::new();
        fc.initialize();

        std::thread::sleep(std::time::Duration::from_millis(300));
        assert!(FocusController::get_window_tracking_config());
        assert_eq!(FocusController::get_window_tracking_delay_config(), 100);
    }

    #[test]
    fn finalize() {
        let org_wtc = FocusController::get_window_tracking_config();
        let org_wtdc = FocusController::get_window_tracking_delay_config();

        let mut fc = FocusController::new();

        fc.initialize();
        FocusController::set_window_tracking_config(!org_wtc);
        FocusController::set_window_tracking_delay_config(org_wtdc + 100);

        fc.finalize();
        std::thread::sleep(std::time::Duration::from_millis(300));
        assert_eq!(FocusController::get_window_tracking_config(), org_wtc);
        assert_eq!(
            FocusController::get_window_tracking_delay_config(),
            org_wtdc
        );
    }

    #[test]
    fn focus_on() {
        let mut fc = FocusController::new();
        fc.focus_on();
        std::thread::sleep(std::time::Duration::from_millis(300));
        assert!(FocusController::get_window_tracking_config());
    }

    #[test]
    fn tfocus_off() {
        let mut fc = FocusController::new();
        fc.focus_off();
        std::thread::sleep(std::time::Duration::from_millis(300));
        assert!(!FocusController::get_window_tracking_config());
    }
}