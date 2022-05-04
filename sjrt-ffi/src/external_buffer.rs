use sjrt::IBuffer;

pub struct ExternalBuffer {
    get_width_callback: unsafe extern "C" fn() -> i32,
    get_height_callback: unsafe extern "C" fn() -> i32,
    set_color_callback: unsafe extern "C" fn(x: i32, y: i32, red: u8, green: u8, blue: u8),
}

impl ExternalBuffer {
    pub fn new(
        get_width_callback: unsafe extern "C" fn() -> i32,
        get_height_callback: unsafe extern "C" fn() -> i32,
        set_color_callback: unsafe extern "C" fn(x: i32, y: i32, red: u8, green: u8, blue: u8),
    ) -> Self {
        Self {
            get_width_callback,
            get_height_callback,
            set_color_callback,
        }
    }
}

impl IBuffer for ExternalBuffer {
    fn get_width(&self) -> i32 {
        unsafe { (self.get_width_callback)() }
    }

    fn get_height(&self) -> i32 {
        unsafe { (self.get_height_callback)() }
    }

    fn set_color(&mut self, x: i32, y: i32, red: u8, green: u8, blue: u8) {
        unsafe { (self.set_color_callback)(x, y, red, green, blue) }
    }
}
