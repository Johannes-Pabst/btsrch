use eframe::egui::{Color32, CornerRadius, Frame, Margin, Shadow, Stroke};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct UIConfig {
    pub textbox_frame: FrameConfig,
    pub outer_frame: FrameConfig,
    pub non_selected_result_frame: FrameConfig,
    pub selected_result_frame: FrameConfig,
}
impl Default for UIConfig {
    fn default() -> Self {
        Self {
            textbox_frame: FrameConfig {
                inner_margin: MarginConfig {
                    left: 0,
                    right: 0,
                    top: 0,
                    bottom: 0,
                },
                fill: RgbaConfig {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0,
                },
                stroke: StrokeConfig {
                    width: 0.0,
                    color: RgbaConfig {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0,
                    },
                },
                corner_radius: CornerRadiusConfig::One(0),
                outer_margin: MarginConfig {
                    bottom: 0,
                    left: 0,
                    right: 0,
                    top: 0,
                },
                shadow: ShadowConfig {
                    blur: 0,
                    color: RgbaConfig {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0,
                    },
                    offset_x: 0,
                    offset_y: 0,
                    spread: 0,
                },
            },
            outer_frame: FrameConfig {
                inner_margin: MarginConfig {
                    left: 0,
                    right: 0,
                    top: 0,
                    bottom: 0,
                },
                fill: RgbaConfig {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0,
                },
                stroke: StrokeConfig {
                    width: 0.0,
                    color: RgbaConfig {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0,
                    },
                },
                corner_radius: CornerRadiusConfig::One(0),
                outer_margin: MarginConfig {
                    bottom: 0,
                    left: 0,
                    right: 0,
                    top: 0,
                },
                shadow: ShadowConfig {
                    blur: 0,
                    color: RgbaConfig {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0,
                    },
                    offset_x: 0,
                    offset_y: 0,
                    spread: 0,
                },
            },
            non_selected_result_frame: FrameConfig {
                inner_margin: MarginConfig {
                    left: 0,
                    right: 0,
                    top: 0,
                    bottom: 0,
                },
                fill: RgbaConfig {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0,
                },
                stroke: StrokeConfig {
                    width: 0.0,
                    color: RgbaConfig {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0,
                    },
                },
                corner_radius: CornerRadiusConfig::One(0),
                outer_margin: MarginConfig {
                    bottom: 0,
                    left: 0,
                    right: 0,
                    top: 0,
                },
                shadow: ShadowConfig {
                    blur: 0,
                    color: RgbaConfig {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0,
                    },
                    offset_x: 0,
                    offset_y: 0,
                    spread: 0,
                },
            },
            selected_result_frame: FrameConfig {
                inner_margin: MarginConfig {
                    left: 0,
                    right: 0,
                    top: 0,
                    bottom: 0,
                },
                fill: RgbaConfig {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0,
                },
                stroke: StrokeConfig {
                    width: 0.0,
                    color: RgbaConfig {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0,
                    },
                },
                corner_radius: CornerRadiusConfig::One(0),
                outer_margin: MarginConfig {
                    bottom: 0,
                    left: 0,
                    right: 0,
                    top: 0,
                },
                shadow: ShadowConfig {
                    blur: 0,
                    color: RgbaConfig {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0,
                    },
                    offset_x: 0,
                    offset_y: 0,
                    spread: 0,
                },
            },
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RgbaConfig {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
impl Default for RgbaConfig {
    fn default() -> Self {
        Self {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ShadowConfig {
    pub offset_x: i8,
    pub offset_y: i8,
    pub blur: u8,
    pub spread: u8,
    pub color: RgbaConfig,
}
impl Default for ShadowConfig {
    fn default() -> Self {
        Self {
            offset_x: 0,
            offset_y: 0,
            blur: 0,
            spread: 0,
            color: Default::default(),
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct FrameConfig {
    pub inner_margin: MarginConfig,
    pub fill: RgbaConfig,
    pub stroke: StrokeConfig,
    pub corner_radius: CornerRadiusConfig,
    pub outer_margin: MarginConfig,
    pub shadow: ShadowConfig,
}
impl Default for FrameConfig {
    fn default() -> Self {
        Self {
            inner_margin: Default::default(),
            fill: Default::default(),
            stroke: Default::default(),
            corner_radius: Default::default(),
            outer_margin: Default::default(),
            shadow: Default::default(),
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct MarginConfig {
    pub left: i8,
    pub right: i8,
    pub top: i8,
    pub bottom: i8,
}
impl Default for MarginConfig {
    fn default() -> Self {
        Self {
            left: Default::default(),
            right: Default::default(),
            top: Default::default(),
            bottom: Default::default(),
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct StrokeConfig {
    pub width: f32,
    pub color: RgbaConfig,
}
impl Default for StrokeConfig {
    fn default() -> Self {
        Self {
            width: Default::default(),
            color: Default::default(),
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CornerRadiusConfig {
    One(u8),
    All(CornerRadiusDetailedConfig),
}
impl Default for CornerRadiusConfig {
    fn default() -> Self {
        Self::One(0)
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CornerRadiusDetailedConfig {
    pub nw: u8,
    pub ne: u8,
    pub sw: u8,
    pub se: u8,
}
impl Default for CornerRadiusDetailedConfig {
    fn default() -> Self {
        Self {
            nw: Default::default(),
            ne: Default::default(),
            sw: Default::default(),
            se: Default::default(),
        }
    }
}
impl From<FrameConfig> for Frame {
    fn from(value: FrameConfig) -> Self {
        Frame {
            inner_margin: value.inner_margin.into(),
            fill: value.fill.into(),
            stroke: value.stroke.into(),
            corner_radius: value.corner_radius.into(),
            outer_margin: value.outer_margin.into(),
            shadow: value.shadow.into(),
        }
    }
}
impl From<MarginConfig> for Margin {
    fn from(value: MarginConfig) -> Self {
        Margin {
            left: value.left,
            right: value.right,
            top: value.top,
            bottom: value.bottom,
        }
    }
}
impl From<RgbaConfig> for Color32 {
    fn from(value: RgbaConfig) -> Self {
        Color32::from_rgba_unmultiplied(value.r, value.g, value.b, value.a)
    }
}
impl From<StrokeConfig> for Stroke {
    fn from(value: StrokeConfig) -> Self {
        Stroke {
            width: value.width,
            color: value.color.into(),
        }
    }
}
impl From<CornerRadiusConfig> for CornerRadius {
    fn from(value: CornerRadiusConfig) -> Self {
        match value {
            CornerRadiusConfig::All(c) => CornerRadius {
                nw: c.nw,
                ne: c.ne,
                sw: c.sw,
                se: c.se,
            },
            CornerRadiusConfig::One(x) => CornerRadius::same(x),
        }
    }
}
impl From<ShadowConfig> for Shadow {
    fn from(value: ShadowConfig) -> Self {
        Shadow {
            offset: [value.offset_x, value.offset_y],
            blur: value.blur,
            spread: value.spread,
            color: value.color.into(),
        }
    }
}