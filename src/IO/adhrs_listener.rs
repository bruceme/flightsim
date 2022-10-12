pub struct RawAttitude {
    pub flags: u32,
    pub roll:f32,
    pub pitch: f32,
    pub heading:f32,
    pub accelleration:f32,
    pub indicate_static_pressure:f32,
    pub raw_pitot_pressure:f32,
    pub temperature:f32,
}

pub trait AdhrsListener {
    fn get_raw_attitude(&self) -> &RawAttitude;
}