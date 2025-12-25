pub mod error;
pub mod toml;

pub fn load_scene(path: &str) -> Result<scene_types::SceneFile, error::SceneError> {
    use crate::toml::parse_toml_scene_from_str;
    use std::fs;

    let text = fs::read_to_string(&path)?;
    let scene = parse_toml_scene_from_str(&text)?;
    Ok(scene)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::SceneError;
    use std::io::Write;

    #[test]
    fn load_scene_from_file_ok() {
        let mut f = tempfile::NamedTempFile::new().expect("create temp file");
        let toml = r#"
            [camera]
            position = [0, 0, 0]
            target = [0, 0, -1]
            fov = 40.0

            [ambient]
            intensity = 0.4
            color = [255, 255, 255]

            [[lights]]
            type = "point"
            position = [-10, 10, -10]
            intensity = 0.5
            color = [255, 255, 255]

            [[objects]]
            type = "sphere"
            position = [0, 0, -30]
            radius = 5.0
            material = { type = "default" }
            color = [136, 8, 8]
        "#;

        f.write_all(toml.as_bytes()).expect("write toml");
        f.flush().expect("flush toml");

        let path = f.path().to_str().expect("path is valid utf-8");
        let scene = load_scene(path).unwrap();

        assert_eq!(scene.camera.position, [0, 0, 0]);
        assert_eq!(scene.camera.target, [0, 0, -1]);
        assert_eq!(scene.lights.len(), 1);
        assert_eq!(scene.objects.len(), 1);
    }

    #[test]
    fn load_scene_from_file_yelds_error() {
        let err = load_scene("uknown.toml");
        assert!(matches!(err, Err(SceneError::InvalidFile(_))));
    }
}
