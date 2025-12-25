use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SceneFile {
    pub camera: CameraDef,
    pub ambient: AmbientDef,
    pub lights: Vec<LightDef>,
    pub objects: Vec<ObjectDef>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CameraDef {
    pub position: [i32; 3],
    pub target: [i32; 3],
    pub fov: f32,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AmbientDef {
    pub intensity: f32,
    pub color: [u8; 3],
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LightDef {
    #[serde(rename = "type")]
    pub kind: LightKindDef,
    pub position: [i32; 3],
    pub intensity: f32,
    pub color: [u8; 3],
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub enum LightKindDef {
    Point,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ObjectDef {
    pub position: [i32; 3],
    pub material: MaterialDef,
    pub color: [u8; 3],
    #[serde(flatten)]
    pub shape: ShapeDef,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase", deny_unknown_fields)]
pub enum ShapeDef {
    Sphere { radius: f32 },
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase", deny_unknown_fields)]
pub enum MaterialDef {
    Default(MaterialEmptyDef),
    Custom(MaterialCustomDef),
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MaterialCustomDef {
    #[serde(rename = "ambient-coefficient")]
    pub ambient_coeff: f32,
    #[serde(rename = "diffuse-coefficient")]
    pub diffuse_coeff: f32,
    #[serde(rename = "specular-coefficient")]
    pub specular_coeff: f32,
    pub shininess: f32,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MaterialEmptyDef {}
