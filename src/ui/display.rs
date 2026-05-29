use std::fs;

/// Check if any HDMI display is connected via DRM sysfs
pub fn is_hdmi_connected() -> bool {
  let patterns = [
    "/sys/class/drm/card0-HDMI-A-1/status",
    "/sys/class/drm/card1-HDMI-A-1/status",
    "/sys/class/drm/card0-HDMI-1/status",
  ];

  for path in &patterns {
    if let Ok(status) = fs::read_to_string(path) {
      if status.trim() == "connected" {
        return true;
      }
    }
  }

  false
}

/// Get first available resolution from DRM sysfs
pub fn get_display_resolution() -> (u32, u32) {
  let paths = [
    "/sys/class/drm/card0-HDMI-A-1/modes",
    "/sys/class/drm/card1-HDMI-A-1/modes",
  ];

  for path in &paths {
    if let Ok(modes) = fs::read_to_string(path) {
      if let Some(first) = modes.lines().next() {
        if let Some(res) = parse_resolution(first) {
          return res;
        }
      }
    }
  }

  (1920, 1080) // sensible default
}

fn parse_resolution(mode: &str) -> Option<(u32, u32)> {
  let parts: Vec<&str> = mode.split('x').collect();
  if parts.len() == 2 {
    let w = parts[0].parse().ok()?;
    let h = parts[1].trim_end_matches('i').parse().ok()?;
    return Some((w, h));
  }
  None
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_resolution() {
    assert_eq!(
      parse_resolution("1920x1080"),
      Some((1920, 1080))
    );
    assert_eq!(
      parse_resolution("1920x1080i"),
      Some((1920, 1080))
    );
    assert_eq!(
      parse_resolution("1280x720"),
      Some((1280, 720))
    );
    assert_eq!(parse_resolution("invalid"), None);
  }
}
