use crate::sys::*;
use crate::Result;
use std::ffi::CString;
use std::time::Duration;
use typed_builder::TypedBuilder;

pub use crate::sys::Key;

#[derive(Debug, Clone, Copy, TypedBuilder)]
pub struct CompoundKey {
    pub key: Key,

    #[builder(default = false)]
    pub shift: bool,

    #[builder(default = false)]
    pub right_shift: bool,

    #[builder(default = false)]
    pub ctrl: bool,

    #[builder(default = false)]
    pub right_ctrl: bool,

    #[builder(default = false)]
    pub alt: bool,

    #[builder(default = false)]
    pub right_alt: bool,

    #[builder(default = false)]
    pub super_key: bool,

    #[builder(default = false)]
    pub right_super_key: bool,
}

impl CompoundKey {
    pub fn new(key: Key) -> CompoundKeyBuilder<((Key,), (), (), (), (), (), (), (), ())> {
        CompoundKey::builder().key(key)
    }

    pub fn is_pressed(&self) -> bool {
        if !unsafe { is_key_pressed(self.key as i32) } {
            return false;
        }

        if self.shift && !unsafe { is_key_down(Key::LeftShift as i32) } {
            return false;
        }

        if self.right_shift && !unsafe { is_key_down(Key::RightShift as i32) } {
            return false;
        }

        if self.ctrl && !unsafe { is_key_down(Key::LeftControl as i32) } {
            return false;
        }

        if self.right_ctrl && !unsafe { is_key_down(Key::RightControl as i32) } {
            return false;
        }

        if self.alt && !unsafe { is_key_down(Key::LeftAlt as i32) } {
            return false;
        }

        if self.right_alt && !unsafe { is_key_down(Key::RightAlt as i32) } {
            return false;
        }

        if self.super_key && !unsafe { is_key_down(Key::LeftSuper as i32) } {
            return false;
        }

        if self.right_super_key && !unsafe { is_key_down(Key::RightSuper as i32) } {
            return false;
        }

        true
    }
}

impl Key {
    pub fn is_pressed(&self) -> bool {
        unsafe { is_key_pressed(*self as i32) }
    }

    pub fn is_repeated(&self) -> bool {
        unsafe { is_key_pressed_repeat(*self as i32) }
    }

    pub fn is_down(&self) -> bool {
        unsafe { is_key_down(*self as i32) }
    }

    pub fn is_up(&self) -> bool {
        unsafe { is_key_up(*self as i32) }
    }

    pub fn is_released(&self) -> bool {
        unsafe { is_key_released(*self as i32) }
    }

    pub fn get_pressed_keys() -> Vec<Key> {
        let mut key;
        let mut keys: Vec<Key> = Vec::new();

        unsafe {
            loop {
                key = get_key_pressed();
                if key == 0 {
                    break;
                }
                keys.push(std::mem::transmute(key));
            }
        }

        keys
    }

    // TODO: maybe impl an iterator for this and the above?
    pub fn get_pressed_chars() -> Vec<char> {
        let mut key;
        let mut keys: Vec<char> = Vec::new();

        unsafe {
            loop {
                key = get_char_pressed();
                if key == 0 {
                    break;
                }
                keys.push(std::char::from_u32(key as u32).unwrap_or('?'));
            }
        }

        keys
    }
}

pub struct Gamepad(pub u32);

impl Gamepad {
    pub fn is_available(&self) -> bool {
        unsafe { is_gamepad_available(self.0 as i32) }
    }

    pub fn name(&self) -> String {
        unsafe {
            std::ffi::CStr::from_ptr(get_gamepad_name(self.0 as i32))
                .to_string_lossy()
                .into_owned()
        }
    }

    pub fn is_pressed(&self, button: GamepadButton) -> bool {
        unsafe { is_gamepad_button_pressed(self.0 as i32, button as i32) }
    }

    pub fn is_down(&self, button: GamepadButton) -> bool {
        unsafe { is_gamepad_button_down(self.0 as i32, button as i32) }
    }

    pub fn is_released(&self, button: GamepadButton) -> bool {
        unsafe { is_gamepad_button_released(self.0 as i32, button as i32) }
    }

    pub fn is_up(&self, button: GamepadButton) -> bool {
        unsafe { is_gamepad_button_up(self.0 as i32, button as i32) }
    }

    pub fn last_button_pressed(&self) -> Option<GamepadButton> {
        let button = unsafe { get_gamepad_button_pressed() };
        if button == -1 {
            None
        } else {
            Some(unsafe { std::mem::transmute(button) })
        }
    }

    pub fn axis_count(&self) -> i32 {
        unsafe { get_gamepad_axis_count(self.0 as i32) }
    }

    pub fn axis_movement(&self, axis: i32) -> f32 {
        unsafe { get_gamepad_axis_movement(self.0 as i32, axis) }
    }

    pub fn vibrate(&mut self, left_speed: f32, right_speed: f32, duration: Duration) {
        let duration = duration.as_millis() as f32 / 1000.0;
        unsafe { set_gamepad_vibration(self.0 as i32, left_speed, right_speed, duration) }
    }

    pub fn vibratef(&mut self, left_speed: f32, right_speed: f32, duration: f32) {
        unsafe { set_gamepad_vibration(self.0 as i32, left_speed, right_speed, duration) }
    }

    pub fn set_mappings(mappings: impl AsRef<str>) -> Result<()> {
        let mappings = CString::new(mappings.as_ref())?;
        unsafe {
            set_gamepad_mappings(mappings.as_ptr());
        }
        Ok(())
    }
}

impl MouseButton {
    pub fn is_pressed(&self) -> bool {
        unsafe { is_mouse_button_pressed(*self as i32) }
    }

    pub fn is_down(&self) -> bool {
        unsafe { is_mouse_button_down(*self as i32) }
    }

    pub fn is_released(&self) -> bool {
        unsafe { is_mouse_button_released(*self as i32) }
    }

    pub fn is_up(&self) -> bool {
        unsafe { is_mouse_button_up(*self as i32) }
    }
}

impl MouseCursor {
    pub fn set(&mut self) {
        unsafe { set_mouse_cursor(*self as i32) }
    }
}

pub struct Mouse;

impl Mouse {
    pub fn x() -> i32 {
        unsafe { get_mouse_x() }
    }

    pub fn y() -> i32 {
        unsafe { get_mouse_y() }
    }

    pub fn position() -> (i32, i32) {
        let (x, y): (i32, i32) = unsafe { get_mouse_position() }.into();
        (x, y)
    }

    pub fn positionv() -> Vector2 {
        unsafe { get_mouse_position() }
    }

    pub fn delta() -> (i32, i32) {
        let (x, y): (i32, i32) = unsafe { get_mouse_delta() }.into();
        (x, y)
    }

    pub fn deltav() -> Vector2 {
        unsafe { get_mouse_delta() }
    }

    pub fn set_position(x: i32, y: i32) {
        unsafe { set_mouse_position(x, y) }
    }

    pub fn set_positionv(position: impl Into<Vector2>) {
        let position = position.into();
        unsafe { set_mouse_position(position.x as i32, position.y as i32) }
    }

    pub fn set_x(x: i32) {
        unsafe { set_mouse_position(x, Self::y()) }
    }

    pub fn set_y(y: i32) {
        unsafe { set_mouse_position(Self::x(), y) }
    }

    pub fn set_offset(x: i32, y: i32) {
        unsafe { set_mouse_offset(x as i32, y as i32) }
    }

    pub fn set_offsetv(offset: impl Into<Vector2>) {
        let offset = offset.into();
        unsafe { set_mouse_offset(offset.x as i32, offset.y as i32) }
    }

    pub fn set_scale(scale_x: f32, scale_y: f32) {
        unsafe { set_mouse_scale(scale_x, scale_y) }
    }

    pub fn set_scalev(scale: impl Into<Vector2>) {
        let scale = scale.into();
        unsafe { set_mouse_scale(scale.x, scale.y) }
    }

    pub fn wheel() -> f32 {
        unsafe { get_mouse_wheel_move() }
    }

    pub fn wheelv() -> Vector2 {
        unsafe { get_mouse_wheel_move_v() }
    }
}

pub struct Touch;
pub struct TouchPoint(u32);

impl Touch {
    pub fn x() -> i32 {
        unsafe { get_touch_x() }
    }

    pub fn y() -> i32 {
        unsafe { get_touch_y() }
    }

    pub fn touch_points() -> Vec<TouchPoint> {
        let count = unsafe { get_touch_point_count() };
        (0..count).map(move |x| TouchPoint(x as u32)).collect()
    }
}

impl TouchPoint {
    pub fn id(&self) -> u32 {
        self.0
    }

    pub fn position(&self) -> (i32, i32) {
        let (x, y): (i32, i32) = unsafe { get_touch_position(self.0 as i32) }.into();
        (x, y)
    }

    pub fn positionv(&self) -> Vector2 {
        unsafe { get_touch_position(self.0 as i32) }
    }
}

impl Gesture {
    pub fn enable(&self) {
        unsafe { set_gestures_enabled(*self as u32) }
    }

    pub fn is_detected(&self) -> bool {
        unsafe { is_gesture_detected(*self as u32) }
    }

    pub fn current() -> Gesture {
        unsafe { std::mem::transmute(get_gesture_detected()) }
    }

    pub fn duration() -> Duration {
        unsafe { Duration::from_secs_f32(get_gesture_hold_duration()) }
    }

    pub fn durationf() -> f32 {
        unsafe { get_gesture_hold_duration() }
    }

    pub fn drag_vector() -> Vector2 {
        unsafe { get_gesture_drag_vector() }
    }

    pub fn drag_angle() -> f32 {
        unsafe { get_gesture_drag_angle() }
    }

    pub fn pinch_vector() -> Vector2 {
        unsafe { get_gesture_pinch_vector() }
    }

    pub fn pinch_angle() -> f32 {
        unsafe { get_gesture_pinch_angle() }
    }
}
