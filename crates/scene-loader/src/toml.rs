use crate::error::SceneError;
use scene_types::SceneFile;

pub fn parse_toml_scene_from_str(toml_str: &str) -> Result<SceneFile, SceneError> {
    let scene: SceneFile = toml::from_str(toml_str)?;
    Ok(scene)
}

#[cfg(test)]
mod tests {
    use scene_types::{
        AmbientDef, CameraDef, LightDef, LightKindDef, MaterialCustomDef, MaterialDef, ObjectDef,
        ShapeDef,
    };

    use super::*;

    #[test]
    fn parse_toml_scene_from_str_ok() {
        let input = r#"
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
            material = {
                    type = "custom",
                    ambient-coefficient = 0.1,
                    diffuse-coefficient = 0.9,
                    specular-coefficient = 0.9,
                    shininess = 200.0
                }
            color = [136, 8, 8]
        "#;

        let scene: SceneFile = parse_toml_scene_from_str(input).unwrap();

        let mut scene_expected: SceneFile = SceneFile {
            camera: CameraDef {
                position: [0, 0, 0],
                target: [0, 0, -1],
                fov: 40.0,
            },
            ambient: AmbientDef {
                intensity: 0.4,
                color: [255, 255, 255],
            },
            lights: vec![LightDef {
                kind: LightKindDef::Point,
                position: [-10, 10, -10],
                intensity: 0.5,
                color: [255, 255, 255],
            }],
            objects: vec![ObjectDef {
                position: [0, 0, -30],
                material: MaterialDef::Custom(MaterialCustomDef {
                    ambient_coeff: 0.1,
                    diffuse_coeff: 0.9,
                    specular_coeff: 0.9,
                    shininess: 200.0,
                }),
                color: [136, 8, 8],
                shape: ShapeDef::Sphere { radius: 5.0 },
            }],
        };

        assert_eq!(scene, scene_expected);

        let input = r#"
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

        let scene: SceneFile = parse_toml_scene_from_str(input).unwrap();
        scene_expected.objects[0].material = MaterialDef::Default {};

        assert_eq!(scene, scene_expected);
    }

    #[test]
    fn parse_toml_scene_from_str_yelds_error() {
        let input = r#"
            [camera]
            UKNOWN = [0, 0, 0]
            target = [0, 0, -1]
            fov = 40.0

            [ambient]
            intensity = 0.4
            color = [255, 255, 255]

            [[lights]]
            type = { type = "point" }
            position = [-10, 10, -10]
            intensity = 0.5
            color = [255, 255, 255]

            [[objects]]
            type = "sphere"
            position = [0, 0, -30]
            radius = 5.0
            material-type = "default"
            color = [136, 8, 8]
        "#;

        let err = parse_toml_scene_from_str(input);
        assert!(matches!(err, Err(SceneError::InvalidScene(_))));

        let input = r#"
            [camera]
            position = [0, 0, 0]
            target = [0, 0, -1]
            fov = 40.0

            [ambient]
            intensity = 0.4
            color = [255, 255, 255]

            [[lights]]
            type = { type = "point" }
            position = [-10, 10, -10]
            intensity = 0.5
            color = [255, 255, 255]

            [[objects]]
            type = "sphere"
            position = [0, 0, -30]
            radius = 5.0
            material-type = "UKNOWN"
            color = [136, 8, 8]
        "#;

        let err = parse_toml_scene_from_str(input);
        assert!(matches!(err, Err(SceneError::InvalidScene(_))));
    }
}
