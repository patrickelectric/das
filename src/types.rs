/// Define possible firmwares used by the vehicle
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Firmware {
    ArduPilot,
    PX4,
    None,
    Other(String),
}

/// Define vehicle type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    Antenna,
    Blimp,
    Copter,
    Plane,
    Rover,
    Sub,
    Other(),
}

/// Define other structs used in the Vehicle struct
#[derive(Debug, Clone, PartialEq)]
pub struct Attitude {
    roll: f64,
    pitch: f64,
    yaw: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Altitude {
    /// Mean Sea Level, in meters
    msl: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Battery {
    /// List of cell voltage in volts
    cells: Vec<f64>,
    /// In volts
    voltage: f64,
}

/// Coordinates in degrees
#[derive(Debug, Clone, PartialEq)]
pub struct Coordinates {
    latitude: f64,
    longitude: f64,
    altitude: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PowerSupply {
    /// in Volts
    voltage: Option<f64>,
    /// in Amps
    current: Option<f64>,
    /// Percentage available
    remaining: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    value: f64,
    name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Velocity {
    /// Ground X Speed in m/s (Latitude, positive north)
    x: f64,
    /// Ground Y Speed in m/s (Longitude, positive east)
    y: f64,
    /// Ground Z Speed in m/s (Altitude, positive down)
    z: f64,
    /// Combined X-Y Speed in m/s (positive north-east)
    ground: f64,
    /// Combined X-Y-Z Speed in m/s (positive north-east-down)
    overall: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StatusText {
    severity: AlertLevel,
    text: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FixTypeGPS {
    NoGPS,
    NoFix,
    Fix2D,
    Fix3D,
    DGPS,
    RTKFloat,
    RTKFixed,
    Static,
    PPP,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StatusGPS {
    visible_satellites: u32,
    fix_type: FixTypeGPS,
    hdop: f64,
    vdop: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlertLevel {
    Emergency,
    Alert,
    Critical,
    Error,
    Warning,
    Notice,
    Informational,
    Debug,
}
