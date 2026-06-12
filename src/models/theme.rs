use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppTheme {
    AeroBlue,
    RubyPink,
    ForestGreen,
    SilverMetallic,
}

impl AppTheme {
    pub fn name(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "Azul Aero",
            AppTheme::RubyPink => "Rosa Choque",
            AppTheme::ForestGreen => "Verde Natureza",
            AppTheme::SilverMetallic => "Prata Metálico",
        }
    }

    pub fn bg_gradient(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "from-[#c2ddf4] via-[#ffffff] to-[#eff8fa]",
            AppTheme::RubyPink => "from-[#fcd5ce] via-[#ffffff] to-[#fde2e4]",
            AppTheme::ForestGreen => "from-[#c8e6c9] via-[#ffffff] to-[#e8f5e9]",
            AppTheme::SilverMetallic => "from-[#e5e5ea] via-[#ffffff] to-[#f5f5f7]",
        }
    }

    pub fn glass_bg(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "bg-[#ffffff]/50 border-[#8ab8e6]/60 shadow-[#4b7cb6]/20",
            AppTheme::RubyPink => "bg-[#ffffff]/55 border-[#f5b3b5]/60 shadow-[#e07a7f]/20",
            AppTheme::ForestGreen => "bg-[#ffffff]/50 border-[#a2cfab]/60 shadow-[#5b9666]/20",
            AppTheme::SilverMetallic => "bg-[#ffffff]/60 border-[#c7c7cc]/60 shadow-[#8e8e93]/20",
        }
    }

    pub fn accent_color(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "text-[#1d528f] bg-[#1d528f]",
            AppTheme::RubyPink => "text-[#a81c43] bg-[#a81c43]",
            AppTheme::ForestGreen => "text-[#2e6930] bg-[#2e6930]",
            AppTheme::SilverMetallic => "text-[#3a3a3c] bg-[#3a3a3c]",
        }
    }

    pub fn glass_border_color(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "border-[#5c98d6]",
            AppTheme::RubyPink => "border-[#ea888e]",
            AppTheme::ForestGreen => "border-[#85c290]",
            AppTheme::SilverMetallic => "border-[#b0b0b8]",
        }
    }

    pub fn titlebar_gradient(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "from-[#e3effa] via-[#edf5fc] to-[#f4f9fd]",
            AppTheme::RubyPink => "from-[#fae6e8] via-[#fcf0f1] to-[#fdf7f7]",
            AppTheme::ForestGreen => "from-[#e7f4e9] via-[#f1f9f2] to-[#f7fcf8]",
            AppTheme::SilverMetallic => "from-[#f2f2f4] via-[#f7f7f9] to-[#fafafb]",
        }
    }

    pub fn titlebar_border(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "border-[#a6b9cd]/45",
            AppTheme::RubyPink => "border-[#dfacb0]/45",
            AppTheme::ForestGreen => "border-[#accbad]/45",
            AppTheme::SilverMetallic => "border-[#bcbcbc]/45",
        }
    }

    pub fn titlebar_text(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "text-[#2d517a]",
            AppTheme::RubyPink => "text-[#5a2024]",
            AppTheme::ForestGreen => "text-[#1d3d20]",
            AppTheme::SilverMetallic => "text-[#333333]",
        }
    }

    pub fn bg_chat(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "linear-gradient(180deg, #c2ddf4 0%, #ffffff 15%, #ffffff 89%, #eff8fa 100%)",
            AppTheme::RubyPink => "linear-gradient(180deg, rgba(254, 230, 232, 0.95) 0%, rgba(253, 203, 196, 0.9) 100%)",
            AppTheme::ForestGreen => "linear-gradient(180deg, rgba(232, 245, 233, 0.95) 0%, rgba(175, 224, 177, 0.9) 100%)",
            AppTheme::SilverMetallic => "linear-gradient(180deg, rgba(245, 245, 247, 0.95) 0%, rgba(209, 209, 214, 0.9) 100%)",
        }
    }

    pub fn modal_gradient(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "from-[#e6f1fc] to-[#c8def5]",
            AppTheme::RubyPink => "from-[#fae6e8] to-[#f5b3b5]",
            AppTheme::ForestGreen => "from-[#e7f4e9] to-[#a2cfab]",
            AppTheme::SilverMetallic => "from-[#f2f2f4] to-[#d1d1d6]",
        }
    }

    pub fn modal_border(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "border-[#7ba9d4]",
            AppTheme::RubyPink => "border-[#ea888e]",
            AppTheme::ForestGreen => "border-[#85c290]",
            AppTheme::SilverMetallic => "border-[#b0b0b8]",
        }
    }

    pub fn btn_primary(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "bg-gradient-to-b from-[#8fc1e9] via-[#5c98d6] to-[#4585c5] hover:from-[#9bd0fa] hover:via-[#70abeb] hover:to-[#579adf] text-white border-[#4074a8]",
            AppTheme::RubyPink => "bg-gradient-to-b from-[#f5b3b5] via-[#ea888e] to-[#a81c43] hover:from-[#ffc4c6] hover:via-[#f59e9f] hover:to-[#c82255] text-white border-[#a81c43]",
            AppTheme::ForestGreen => "bg-gradient-to-b from-[#a2cfab] via-[#85c290] to-[#2e6930] hover:from-[#b5dec0] hover:via-[#9cd0ab] hover:to-[#387e3a] text-white border-[#2e6930]",
            AppTheme::SilverMetallic => "bg-gradient-to-b from-[#d1d1d6] via-[#aeaea2] to-[#3a3a3c] hover:from-[#e5e5ea] hover:via-[#c7c7cc] hover:to-[#48484a] text-white border-[#3a3a3c]",
        }
    }

    pub fn tooltip_bg(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "from-sky-50 to-sky-100/95 border-[#a6b9cd]",
            AppTheme::RubyPink => "from-[#fff3f3] to-[#ffd7db]/95 border-[#ea888e]",
            AppTheme::ForestGreen => "from-[#f4fbf5] to-[#dcf5e1]/95 border-[#85c290]",
            AppTheme::SilverMetallic => "from-[#fafafb] to-[#e5e5ea]/95 border-[#b0b0b8]",
        }
    }

    pub fn toast_gradient(&self) -> &'static str {
        match self {
            AppTheme::AeroBlue => "linear-gradient(135deg, rgba(240, 248, 255, 0.95) 0%, rgba(215, 235, 252, 0.95) 100%)",
            AppTheme::RubyPink => "linear-gradient(135deg, rgba(255, 243, 243, 0.95) 0%, rgba(255, 215, 219, 0.95) 100%)",
            AppTheme::ForestGreen => "linear-gradient(135deg, rgba(244, 251, 245, 0.95) 0%, rgba(220, 245, 225, 0.95) 100%)",
            AppTheme::SilverMetallic => "linear-gradient(135deg, rgba(250, 250, 251, 0.95) 0%, rgba(229, 229, 234, 0.95) 100%)",
        }
    }
}
