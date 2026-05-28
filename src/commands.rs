use crate::enums::*;

/// The master command sent to the projector.
pub enum Command {
    Power(PowerState),
    Press(Key),
    SetSource(Source),
    SetAspectRatio(AspectRatio),
    Set3D(ThreeDMode),
    Toggle(Action),
    Ask(Query),
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ascii = match self {
            // Power Routing
            Command::Power(state) => match state {
                PowerState::On => "* 0 IR 001",
                PowerState::Off => "* 0 IR 002",
                PowerState::EcoOn => "* 0 IR 051",
                PowerState::EcoProOn => "* 0 IR 078",
                PowerState::EcoOff => "* 0 IR 055",
            },

            // Navigation Routing
            Command::Press(key) => match key {
                Key::Menu => "* 0 IR 008",
                Key::Up => "* 0 IR 009",
                Key::Down => "* 0 IR 010",
                Key::Right => "* 0 IR 011",
                Key::Left => "* 0 IR 012",
                Key::Enter => "* 0 IR 013",
                Key::Back => "* 0 IR 053",
                Key::Empowering => "* 0 IR 047",
            },

            // Source Routing
            Command::SetSource(source) => match source {
                Source::NextSource => "* 0 IR 031",
                Source::Vga1 => "* 0 IR 015",
                Source::Vga2 => "* 0 IR 077",
                Source::Dvi => "* 0 IR 016",
                Source::VgaViaDvi => "* 0 IR 028",
                Source::ComponentVga => "* 0 IR 017",
                Source::ComponentDvi => "* 0 IR 029",
                Source::SVideo => "* 0 IR 018",
                Source::Composite => "* 0 IR 019",
                Source::Component => "* 0 IR 020",
                Source::Hdmi1 => "* 0 IR 050",
                Source::Hdmi2 => "* 0 IR 068",
                Source::Hdmi3WarpBlend => "* 0 IR 069",
                Source::DisplayPort => "* 0 IR 074",
                Source::HdbaseT => "* 0 IR 070",
                Source::Bnc => "* 0 IR 071",
                Source::LanWifi => "* 0 IR 072",
                Source::Media => "* 0 IR 075",
                Source::UsbDisplay => "* 0 IR 076",
                Source::WirelessMirroring => "* 0 IR 079",
            },

            // Aspect Ratio Routing
            Command::SetAspectRatio(ratio) => match ratio {
                AspectRatio::SixteenNine => "* 0 IR 021",
                AspectRatio::FourThree => "* 0 IR 022",
                AspectRatio::LetterBox => "* 0 IR 040",
                AspectRatio::OneOne => "* 0 IR 041",
            },

            // 3D Routing
            Command::Set3D(mode) => match mode {
                ThreeDMode::On => "* 0 IR 056",
                ThreeDMode::Off => "* 0 IR 057",
                ThreeDMode::TwoDToThreeDOn => "* 0 IR 058",
                ThreeDMode::TwoDToThreeDOff => "* 0 IR 059",
                ThreeDMode::FormatAuto => "* 0 IR 060",
                ThreeDMode::FramePacking => "* 0 IR 061",
                ThreeDMode::SideBySideHalf => "* 0 IR 062",
                ThreeDMode::SideBySideFull => "* 0 IR 063",
                ThreeDMode::TopAndBottom => "* 0 IR 064",
                ThreeDMode::LRInvert => "* 0 IR 065",
                ThreeDMode::FrameSequential => "* 0 IR 066",
                ThreeDMode::FieldSequential => "* 0 IR 067",
            },

            // Direct Actions & Menus
            Command::Toggle(action) => match action {
                Action::Mute => "* 0 IR 006",
                Action::Freeze => "* 0 IR 007",
                Action::Hide => "* 0 IR 030",
                Action::ReSync => "* 0 IR 014",
                Action::Volume => "* 0 IR 023",
                Action::Brightness => "* 0 IR 025",
                Action::Contrast => "* 0 IR 026",
                Action::ColorTemp => "* 0 IR 027",
                Action::ColorRgb => "* 0 IR 048",
                Action::Saturation => "* 0 IR 032",
                Action::Hue => "* 0 IR 033",
                Action::Sharpness => "* 0 IR 034",
                Action::KeystoneMenu => "* 0 IR 004",
                Action::KeystoneUp => "* 0 IR 042",
                Action::KeystoneDown => "* 0 IR 043",
                Action::KeystoneLeft => "* 0 IR 044",
                Action::KeystoneRight => "* 0 IR 045",
                Action::ZoomIn => "* 0 IR 046",
                Action::ZoomOut => "* 0 IR 054",
                Action::Language => "* 0 IR 049",
            },

            // Queries (Asking for data)
            Command::Ask(query) => match query {
                Query::ModelName => "* 0 IR 035",
                Query::NativeResolution => "* 0 IR 036",
                Query::CompanyName => "* 0 IR 037",
                Query::Information => "* 0 IR 073",
                Query::EcoModeState => "* 0 IR 052",
                Query::CurrentSource => "* 0 Src ?",
                Query::Lamp1State => "* 0 Lamp ?",
                Query::Lamp1Hours => "* 0 Lamp",
                Query::Lamp2State => "* 0 Lamp2 ?",
                Query::Lamp2Hours => "* 0 Lamp2",
            },
        };

        // Appends the carriage return to whatever variant was matched
        write!(f, "{}\r", ascii)
    }
}
