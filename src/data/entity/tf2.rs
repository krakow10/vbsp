use crate::bool_from_int;
use crate::{Angles, Color, LightColor, Vector};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
#[serde(tag = "classname")]
pub enum Entity<'a> {
    #[serde(rename = "ambient_generic")]
    #[serde(borrow)]
    AmbientGeneric(AmbientGeneric<'a>),
    #[serde(rename = "item_healthkit_full")]
    AmmoPackFull(AmmoPack),
    #[serde(rename = "item_ammopack_medium")]
    AmmoPackMedium(AmmoPack),
    #[serde(rename = "item_ammopack_small")]
    AmmoPackSmall(AmmoPack),
    #[serde(rename = "func_areaportal")]
    AreaPortal(AreaPortal),
    #[serde(rename = "func_brush")]
    #[serde(borrow)]
    Brush(BrushEntity<'a>),
    #[serde(rename = "func_illusionary")]
    #[serde(borrow)]
    BrushIllusionary(BrushEntity<'a>),
    #[serde(rename = "func_wall")]
    #[serde(borrow)]
    BrushWall(BrushEntity<'a>),
    #[serde(rename = "func_wall_toggle")]
    #[serde(borrow)]
    BrushWallToggle(BrushEntity<'a>),
    #[serde(rename = "func_door")]
    Door(Door<'a>),
    #[serde(rename = "func_dustmotes")]
    DustMotes(DustMotes<'a>),
    #[serde(rename = "env_sprite")]
    #[serde(borrow)]
    EnvSprite(EnvSprite<'a>),
    #[serde(rename = "filter_activator_tfteam")]
    #[serde(borrow)]
    FilterActivatorTeam(FilterActivatorTeam<'a>),
    #[serde(rename = "tf_gamerules")]
    #[serde(borrow)]
    GameRules(GameRules<'a>),
    #[serde(rename = "game_text")]
    #[serde(borrow)]
    GameText(GameText<'a>),
    #[serde(rename = "item_ammopack_full")]
    HealthPackFull(HealthPack),
    #[serde(rename = "item_healthkit_medium")]
    HealthPackMedium(HealthPack),
    #[serde(rename = "item_healthkit_small")]
    HealthPackSmall(HealthPack),
    #[serde(rename = "tf_logic_koth")]
    KothLogic(KothLogic),
    #[serde(rename = "light")]
    Light(Light),
    #[serde(rename = "env_lightglow")]
    LightGlow(LightGlow),
    #[serde(rename = "light_spot")]
    LightSpot(LightSpot),
    #[serde(rename = "logic_auto")]
    LogicAuto(LogicAuto<'a>),
    #[serde(rename = "logic_case")]
    #[serde(borrow)]
    LogicCase(LogicCase<'a>),
    #[serde(rename = "logic_relay")]
    LogicRelay(LogicRelay<'a>),
    #[serde(rename = "info_observer_point")]
    #[serde(borrow)]
    ObserverPoint(ObserverPoint<'a>),
    #[serde(rename = "func_occluder")]
    #[serde(borrow)]
    Occluder(Occluder<'a>),
    #[serde(rename = "info_particle_system")]
    #[serde(borrow)]
    ParticleSystem(ParticleSystem<'a>),
    #[serde(rename = "path_track")]
    PathTrack(PathTrack<'a>),
    #[serde(rename = "prop_dynamic")]
    #[serde(borrow)]
    PropDynamic(PropDynamic<'a>),
    #[serde(rename = "prop_dynamic_override")]
    #[serde(borrow)]
    PropDynamicOverride(PropDynamicOverride<'a>),
    #[serde(rename = "prop_physics_multiplayer")]
    #[serde(borrow)]
    PropPhysics(PropDynamic<'a>),
    #[serde(rename = "func_regenerate")]
    #[serde(borrow)]
    Regenerate(Regenerate<'a>),
    #[serde(rename = "func_respawnroom")]
    RespawnRoom(RespawnRoom<'a>),
    #[serde(rename = "func_respawnroomvisualizer")]
    #[serde(borrow)]
    RespawnVisualizer(RespawnVisualizer<'a>),
    #[serde(rename = "keyframe_rope")]
    #[serde(borrow)]
    RopeKeyFrame(RopeKeyFrame<'a>),
    #[serde(rename = "move_rope")]
    RopeMove(RopeMove<'a>),
    #[serde(rename = "sky_camera")]
    SkyCamera(SkyCamera),
    #[serde(rename = "env_soundscape_proxy")]
    #[serde(borrow)]
    SoundScapeProxy(SoundScapeProxy<'a>),
    #[serde(rename = "info_player_teamspawn")]
    #[serde(borrow)]
    Spawn(Spawn<'a>),
    #[serde(rename = "point_spotlight")]
    SpotLight(SpotLight),
    #[serde(rename = "team_control_point")]
    #[serde(borrow)]
    TeamControlPoint(TeamControlPoint<'a>),
    #[serde(rename = "trigger_multiple")]
    #[serde(borrow)]
    TriggerMultiple(TriggerMultiple<'a>),
    #[serde(rename = "worldspawn")]
    WorldSpawn(WorldSpawn<'a>),
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct AmbientGeneric<'a> {
    #[serde(rename = "cspinup", default)]
    pub c_spin_up: f32,
    #[serde(rename = "fadeinsec", default)]
    pub fade_in_secs: f32,
    #[serde(rename = "fadeoutsec", default)]
    pub fade_out_secs: f32,
    #[serde(rename = "health", default)]
    pub health: u8,
    #[serde(rename = "lfomodpitch", default)]
    pub lfo_mod_pitch: f32,
    #[serde(rename = "lfo_mod_vol", default)]
    pub lfo_mod_vol: f32,
    #[serde(rename = "lfo_rate", default)]
    pub lfo_rate: u32,
    #[serde(rename = "lfo_type", default)]
    pub lfo_type: u32,
    pub message: &'a str,
    pub origin: Vector,
    #[serde(rename = "pitch", default)]
    pub pitch: f32,
    #[serde(rename = "pitch_start", default)]
    pub pitch_start: f32,
    #[serde(rename = "preset", default)]
    pub preset: u32,
    #[serde(rename = "radius", default)]
    pub radius: f32,
    #[serde(rename = "spawn_flags", default)]
    pub spawn_flags: u32,
    #[serde(rename = "spin_down", default)]
    pub spin_down: f32,
    #[serde(rename = "spin_up", default)]
    pub spin_up: f32,
    #[serde(rename = "volstart", default)]
    pub volume_start: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AmmoPack {
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AreaPortal {
    #[serde(rename = "portalnumber")]
    pub number: u8,
    #[serde(rename = "startopen")]
    #[serde(deserialize_with = "bool_from_int")]
    pub start_open: bool,
    #[serde(rename = "portalversion")]
    pub version: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct BrushEntity<'a> {
    #[serde(rename = "rendercolor")]
    pub color: Color,
    pub model: &'a str,
    pub origin: Vector,
    #[serde(rename = "startdisabled", default)]
    #[serde(deserialize_with = "bool_from_int")]
    pub start_disabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Door<'a> {
    #[serde(rename = "forceclosed", default)]
    #[serde(deserialize_with = "bool_from_int")]
    pub force_closed: bool,
    pub model: &'a str,
    #[serde(rename = "movedir")]
    pub move_direction: Vector,
    pub origin: Vector,
    pub speed: f32,
    #[serde(rename = "targetname", default)]
    pub target: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct DustMotes<'a> {
    #[serde(rename = "alpha")]
    pub alpha: u8,
    #[serde(rename = "color")]
    pub color: Color,
    pub model: &'a str,
    #[serde(default)]
    pub origin: Vector,
    #[serde(rename = "sizemax")]
    pub size_max: u32,
    #[serde(rename = "sizemin")]
    pub size_min: u32,
    #[serde(rename = "spawnrate")]
    pub spawn_rate: u32,
    #[serde(rename = "startdisabled", default)]
    #[serde(deserialize_with = "bool_from_int")]
    pub start_disabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSprite<'a> {
    #[serde(rename = "rendercolor")]
    pub color: Color,
    pub model: &'a str,
    pub origin: Vector,
    pub scale: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterActivatorTeam<'a> {
    #[serde(rename = "negated", default)]
    pub negated: Option<&'a str>,
    pub origin: Vector,
    #[serde(rename = "targetname", default)]
    pub target_name: Option<&'a str>,
    #[serde(rename = "teamnum", default)]
    pub team: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GameRules<'a> {
    #[serde(default)]
    #[serde(deserialize_with = "bool_from_int")]
    pub ctf_overtime: bool,
    #[serde(default)]
    pub hud_type: u32,
    pub origin: Vector,
    #[serde(rename = "targetname", default)]
    pub target_name: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GameText<'a> {
    pub channel: u8,
    pub color: Color,
    #[serde(rename = "fadein")]
    pub fade_in: f32,
    #[serde(rename = "fadeout")]
    pub fade_out: f32,
    pub fadeout: f32,
    #[serde(rename = "fxtime")]
    pub fx_time: f32,
    #[serde(rename = "holdtime")]
    pub hold_time: f32,
    pub message: &'a str,
    pub origin: Vector,
    #[serde(rename = "targetname", default)]
    pub target_name: Option<&'a str>,
    pub x: f32,
    pub y: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct HealthPack {
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct KothLogic {
    pub origin: Vector,
    pub timer_length: u32,
    pub unlock_point: u32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Light {
    #[serde(rename = "_light")]
    pub light: LightColor,
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LightGlow {
    #[serde(rename = "rendercolor")]
    pub color: Color,
    #[serde(rename = "horizontalhlowsize")]
    pub horizontal_size: u32,
    #[serde(rename = "maxdist")]
    pub max_distance: u32,
    #[serde(rename = "mindist")]
    pub min_distance: u32,
    pub origin: Vector,
    #[serde(rename = "startdisabled", default)]
    #[serde(deserialize_with = "bool_from_int")]
    pub start_disabled: bool,
    #[serde(rename = "verticalglowsize")]
    pub vertical_size: u32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LightSpot {
    pub angles: Angles,
    #[serde(rename = "_cone")]
    pub cone: u8,
    #[serde(rename = "_light")]
    pub light: LightColor,
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicAuto<'a> {
    #[serde(rename = "onmapspawn", default)]
    pub on_map_spawn: Option<&'a str>,
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct LogicCase<'a> {
    #[serde(rename = "oncase01")]
    pub oncase_01: Option<&'a str>,
    #[serde(rename = "oncase02")]
    pub oncase_02: Option<&'a str>,
    #[serde(rename = "oncase03")]
    pub oncase_03: Option<&'a str>,
    #[serde(rename = "oncase04")]
    pub oncase_04: Option<&'a str>,
    #[serde(rename = "oncase05")]
    pub oncase_05: Option<&'a str>,
    pub origin: Vector,
    #[serde(rename = "targetname")]
    pub target_name: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicRelay<'a> {
    #[serde(rename = "ontrigger", default)]
    pub on_trigger: Option<&'a str>,
    pub origin: Vector,
    #[serde(rename = "targetname", default)]
    pub target_name: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ObserverPoint<'a> {
    pub angles: Angles,
    pub origin: Vector,
    #[serde(rename = "parentname", default)]
    pub parent: Option<&'a str>,
    #[serde(rename = "startdisabled", default)]
    #[serde(deserialize_with = "bool_from_int")]
    pub start_disabled: bool,
    #[serde(rename = "targetname", default)]
    pub target: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Occluder<'a> {
    pub model: &'a str,
    #[serde(rename = "occludernumber", default)]
    pub occluder_number: u32,
    #[serde(rename = "startactive", deserialize_with = "bool_from_int")]
    pub start_active: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ParticleSystem<'a> {
    pub angles: Angles,
    pub effect_name: &'a str,
    pub origin: Vector,
    #[serde(default)]
    #[serde(deserialize_with = "bool_from_int")]
    pub start_active: bool,
    #[serde(rename = "targetname")]
    pub target_name: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PathTrack<'a> {
    pub angles: Angles,
    #[serde(rename = "orientationtype", default)]
    pub orientation_type: u8,
    pub origin: Vector,
    pub radius: f32,
    pub speed: f32,
    #[serde(default)]
    pub target: Option<&'a str>,
    #[serde(rename = "targetname", default)]
    pub target_name: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropDynamic<'a> {
    pub angles: Angles,
    #[serde(rename = "rendercolor")]
    pub color: Color,
    #[serde(rename = "disablereceiveshadows", default)]
    #[serde(deserialize_with = "bool_from_int")]
    pub disable_receive_shadows: bool,
    #[serde(rename = "disableshadows", default)]
    #[serde(deserialize_with = "bool_from_int")]
    pub disable_shadows: bool,
    pub model: &'a str,
    #[serde(rename = "targetname", default)]
    pub name: Option<&'a str>,
    pub origin: Vector,
    #[serde(rename = "parentname", default)]
    pub parent: Option<&'a str>,
    #[serde(rename = "modelscale")]
    pub scale: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropDynamicOverride<'a> {
    pub angles: Angles,
    #[serde(rename = "rendercolor")]
    pub color: Color,
    #[serde(rename = "disablereceiveshadows", default)]
    #[serde(deserialize_with = "bool_from_int")]
    pub disable_receive_shadows: bool,
    #[serde(rename = "disableshadows", default)]
    #[serde(deserialize_with = "bool_from_int")]
    pub disable_shadows: bool,
    pub model: &'a str,
    #[serde(rename = "targetname", default)]
    pub name: Option<&'a str>,
    pub origin: Vector,
    #[serde(rename = "parentname", default)]
    pub parent: Option<&'a str>,
    #[serde(rename = "modelscale")]
    pub scale: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Regenerate<'a> {
    #[serde(rename = "associatedmodel")]
    pub associated_model: &'a str,
    pub model: &'a str,
    #[serde(rename = "teamnum")]
    pub team: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct RespawnRoom<'a> {
    pub model: &'a str,
    #[serde(rename = "startdisabled", default)]
    #[serde(deserialize_with = "bool_from_int")]
    pub start_disabled: bool,
    #[serde(rename = "targetname", default)]
    pub target: Option<&'a str>,
    #[serde(rename = "teamnum")]
    pub team: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct RespawnVisualizer<'a> {
    #[serde(rename = "rendercolor")]
    pub color: Color,
    pub origin: Vector,
    #[serde(rename = "respawnroomname")]
    pub room_name: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub solid_to_enemies: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct RopeKeyFrame<'a> {
    #[serde(rename = "barbed")]
    #[serde(deserialize_with = "bool_from_int")]
    pub barbed: bool,
    #[serde(rename = "breakable")]
    #[serde(deserialize_with = "bool_from_int")]
    pub breakable: bool,
    #[serde(rename = "collide")]
    #[serde(deserialize_with = "bool_from_int")]
    pub collide: bool,
    #[serde(rename = "dangling")]
    #[serde(deserialize_with = "bool_from_int")]
    pub dangling: bool,
    #[serde(rename = "ropematerial")]
    pub material: &'a str,
    #[serde(rename = "movespeed")]
    pub move_speed: f32,
    pub origin: Vector,
    #[serde(rename = "slack")]
    pub slack: f32,
    #[serde(rename = "subdiv")]
    pub sub_div: u8,
    #[serde(rename = "targetname", default)]
    pub target_name: Option<&'a str>,
    #[serde(rename = "texturescale")]
    pub texture_scale: f32,
    #[serde(rename = "width")]
    pub width: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct RopeMove<'a> {
    #[serde(rename = "barbed")]
    #[serde(deserialize_with = "bool_from_int")]
    pub barbed: bool,
    #[serde(rename = "breakable")]
    #[serde(deserialize_with = "bool_from_int")]
    pub breakable: bool,
    #[serde(rename = "dangling")]
    #[serde(deserialize_with = "bool_from_int")]
    pub dangling: bool,
    #[serde(rename = "positioninterpolator")]
    pub interpolator: u8,
    #[serde(rename = "ropematerial")]
    pub material: &'a str,
    #[serde(rename = "movespeed")]
    pub move_speed: f32,
    #[serde(rename = "nextkey")]
    pub next_key: &'a str,
    pub origin: Vector,
    #[serde(rename = "slack")]
    pub slack: f32,
    #[serde(rename = "subdiv")]
    pub sub_div: u8,
    #[serde(rename = "texturescale")]
    pub texture_scale: f32,
    #[serde(rename = "type")]
    pub ty: u8,
    #[serde(rename = "width")]
    pub width: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct SkyCamera {
    pub angles: Angles,
    #[serde(rename = "fogcolor")]
    pub color: Color,
    #[serde(rename = "fogcolor2", default)]
    pub color2: Option<Color>,
    #[serde(rename = "fogdir")]
    pub direction: Vector,
    #[serde(rename = "fogenable")]
    #[serde(deserialize_with = "bool_from_int")]
    pub fog: bool,
    #[serde(rename = "fogend")]
    pub fog_end: f32,
    #[serde(rename = "fogstart")]
    pub fog_start: f32,
    pub origin: Vector,
    pub scale: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub use_angles: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct SoundScapeProxy<'a> {
    #[serde(rename = "mainsoundscapename")]
    pub main_name: &'a str,
    pub origin: Vector,
    pub radius: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Spawn<'a> {
    pub angles: Angles,
    #[serde(rename = "controlpoint", default)]
    pub control_point: Option<&'a str>,
    pub origin: Vector,
    #[serde(rename = "startdisabled", default)]
    #[serde(deserialize_with = "bool_from_int")]
    pub start_disabled: bool,
    #[serde(rename = "targetname", default)]
    pub target: Option<&'a str>,
    #[serde(rename = "teamnum")]
    pub team: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct SpotLight {
    pub angles: Angles,
    #[serde(rename = "rendercolor")]
    pub color: Color,
    #[serde(rename = "spotlightwidth")]
    pub cone: u8,
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TeamControlPoint<'a> {
    pub angles: Angles,
    pub origin: Vector,
    #[serde(default)]
    pub point_default_owner: u8,
    pub point_warn_sound: &'a str,
    #[serde(rename = "startdisabled", default)]
    #[serde(deserialize_with = "bool_from_int")]
    pub start_disabled: bool,
    #[serde(rename = "targetname")]
    pub target_name: &'a str,
    pub team_icon_0: &'a str,
    pub team_icon_2: &'a str,
    pub team_icon_3: &'a str,
    pub team_model_0: &'a str,
    pub team_model_2: &'a str,
    pub team_model_3: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerMultiple<'a> {
    #[serde(rename = "onendtouch", default)]
    pub end_touch: Option<&'a str>,
    #[serde(rename = "onendtouchall", default)]
    pub end_touch_all: Option<&'a str>,
    #[serde(rename = "filtername", default)]
    pub filter: Option<&'a str>,
    pub model: &'a str,
    #[serde(rename = "onnottouching", default)]
    pub not_touching: Option<&'a str>,
    pub origin: Vector,
    #[serde(rename = "startdisabled", default)]
    #[serde(deserialize_with = "bool_from_int")]
    pub start_disabled: bool,
    #[serde(rename = "onstarttouch", default)]
    pub start_touch: Option<&'a str>,
    #[serde(rename = "onstarttouchall", default)]
    pub start_touch_all: Option<&'a str>,
    #[serde(rename = "targetname", default)]
    pub target_name: Option<&'a str>,
    pub wait: Option<u32>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WorldSpawn<'a> {
    #[serde(default)]
    pub comment: Option<&'a str>,
    #[serde(rename = "detailmaterial")]
    pub detail_material: &'a str,
    #[serde(rename = "detailvbsp")]
    pub detail_vbsp: &'a str,
    #[serde(rename = "world_maxs")]
    pub max: Vector,
    #[serde(rename = "world_mins")]
    pub min: Vector,
    #[serde(rename = "skyname")]
    pub skybox: &'a str,
    #[serde(rename = "mapversion")]
    pub version: u32,
}
