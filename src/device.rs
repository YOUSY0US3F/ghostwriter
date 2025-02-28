use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeviceModel {
    Remarkable2,
    RemarkablePaperPro,
    Unknown,
}

impl DeviceModel {
    pub fn detect() -> Self {
        if Path::new("/etc/hwrevision").exists() {
            if let Ok(hwrev) = std::fs::read_to_string("/etc/hwrevision") {
                if hwrev.contains("ferrari 1.0") {
                    return DeviceModel::RemarkablePaperPro;
                }
                if hwrev.contains("reMarkable2 1.0") {
                    return DeviceModel::Remarkable2;
                }
            }
        }

        // Nothing matched :shrug:
        DeviceModel::Unknown
    }

    pub fn name(&self) -> &str {
        match self {
            DeviceModel::Remarkable2 => "Remarkable2",
            DeviceModel::RemarkablePaperPro => "RemarkablePaperPro",
            DeviceModel::Unknown => "Unknown",
        }
    }

}
