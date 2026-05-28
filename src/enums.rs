/// Power and Eco states
pub enum PowerState {
    On,
    Off,
    EcoOn,
    EcoProOn,
    EcoOff,
}

/// Physical navigation keys on the remote or projector body
pub enum Key {
    Menu,
    Up,
    Down,
    Left,
    Right,
    Enter,
    Back,
    Empowering, // The Acer 'e' key
}

/// All available input sources across the Acer lineup
pub enum Source {
    NextSource, // Cycles inputs
    Vga1,
    Vga2,
    Dvi,
    VgaViaDvi,    // Analog RGB for DVI-A
    ComponentVga, // YPbPr for D-Sub
    ComponentDvi, // YPbPr for DVI
    SVideo,
    Composite,
    Component,
    Hdmi1,
    Hdmi2,
    Hdmi3WarpBlend,
    DisplayPort,
    HdbaseT,
    Bnc,
    LanWifi,
    Media,
    UsbDisplay,
    WirelessMirroring,
}

/// Image proportions
pub enum AspectRatio {
    SixteenNine,
    FourThree,
    LetterBox,
    OneOne,
}

/// 3D capabilities and formats
pub enum ThreeDMode {
    On,
    Off,
    TwoDToThreeDOn,
    TwoDToThreeDOff,
    FormatAuto,
    FramePacking,
    SideBySideHalf,
    SideBySideFull,
    TopAndBottom,
    LRInvert,
    FrameSequential,
    FieldSequential,
}

/// Direct actions and OSD menu toggles
pub enum Action {
    Mute,
    Freeze,
    Hide,
    ReSync,
    Volume,
    Brightness,
    Contrast,
    ColorTemp,
    ColorRgb,
    Saturation,
    Hue,
    Sharpness,
    KeystoneMenu,
    KeystoneUp,
    KeystoneDown,
    KeystoneLeft,
    KeystoneRight,
    ZoomIn,
    ZoomOut,
    Language,
}

/// Information you can ask the projector to return
pub enum Query {
    ModelName,
    NativeResolution,
    CompanyName,
    Information,
    EcoModeState,
    CurrentSource,
    Lamp1State,
    Lamp1Hours,
    Lamp2State,
    Lamp2Hours,
}
