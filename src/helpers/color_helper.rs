use ratatui::style::Color;

pub const EXT_DEFAULT_CLR: Color = Color::White;
const EXT_VISUAL_STUDIO_CLR: Color = Color::Rgb(134, 97, 197);
const EXT_TXT_CLR: Color = Color::LightGreen;
const EXT_JSON_CLR: Color = Color::Rgb(233, 131, 14);
const EXT_EXE_CLR: Color = Color::LightRed;

pub fn get_ext_color(ext: String) -> Color {
    match ext.as_str() {
        "sln" => EXT_VISUAL_STUDIO_CLR,
        "txt" => EXT_TXT_CLR,
        "json" => EXT_JSON_CLR,
        "exe" => EXT_EXE_CLR,
        _ => EXT_DEFAULT_CLR,
    }
}
