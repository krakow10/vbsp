use std::str::FromStr;

use serde::{Deserialize, Deserializer};

use crate::{Angles, Color, LightColor, Vector};

use super::FromStrProp;

fn bool_from_int<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    let int = u8::deserialize(deserializer)?;
    Ok(int > 0)
}

#[derive(Debug, Clone, Deserialize)]
pub enum Negated {
    Yes,
    No,
    MatchingCriteria,
}
pub struct NegatedParseErr;
impl FromStr for Negated {
    type Err = NegatedParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Negated::Yes),
            "0" => Ok(Negated::No),
            "allow entities that match criteria" => Ok(Negated::MatchingCriteria),
            _ => Err(NegatedParseErr),
        }
    }
}
impl FromStrProp for Negated {}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
#[serde(tag = "classname")]
pub enum Entity<'a> {
    #[serde(rename = "env_beam")]
    #[serde(borrow)]
    EnvBeam(EnvBeam<'a>),
    #[serde(rename = "env_detail_controller")]
    EnvDetailController(EnvDetailController),
    #[serde(rename = "env_embers")]
    #[serde(borrow)]
    EnvEmbers(EnvEmbers<'a>),
    #[serde(rename = "env_entity_maker")]
    #[serde(borrow)]
    EnvEntityMaker(EnvEntityMaker<'a>),
    #[serde(rename = "env_fade")]
    #[serde(borrow)]
    EnvFade(EnvFade<'a>),
    #[serde(rename = "env_fire")]
    EnvFire(EnvFire),
    #[serde(rename = "env_fire_trail")]
    #[serde(borrow)]
    EnvFireTrail(EnvFireTrail<'a>),
    #[serde(rename = "env_fog_controller")]
    EnvFogController(EnvFogController),
    #[serde(rename = "env_hudhint")]
    #[serde(borrow)]
    EnvHudhint(EnvHudhint<'a>),
    #[serde(rename = "env_laser")]
    #[serde(borrow)]
    EnvLaser(EnvLaser<'a>),
    #[serde(rename = "env_lightglow")]
    EnvLightglow(EnvLightglow),
    #[serde(rename = "env_shake")]
    #[serde(borrow)]
    EnvShake(EnvShake<'a>),
    #[serde(rename = "env_shooter")]
    #[serde(borrow)]
    EnvShooter(EnvShooter<'a>),
    #[serde(rename = "env_smokestack")]
    #[serde(borrow)]
    EnvSmokestack(EnvSmokestack<'a>),
    #[serde(rename = "env_soundscape")]
    #[serde(borrow)]
    EnvSoundscape(EnvSoundscape<'a>),
    #[serde(rename = "env_soundscape_triggerable")]
    #[serde(borrow)]
    EnvSoundscapeTriggerable(EnvSoundscapeTriggerable<'a>),
    #[serde(rename = "env_spark")]
    #[serde(borrow)]
    EnvSpark(EnvSpark<'a>),
    #[serde(rename = "env_sprite")]
    #[serde(borrow)]
    EnvSprite(EnvSprite<'a>),
    #[serde(rename = "env_spritetrail")]
    #[serde(borrow)]
    EnvSpritetrail(EnvSpritetrail<'a>),
    #[serde(rename = "env_steam")]
    #[serde(borrow)]
    EnvSteam(EnvSteam<'a>),
    #[serde(rename = "env_sun")]
    #[serde(borrow)]
    EnvSun(EnvSun<'a>),
    #[serde(rename = "env_tonemap_controller")]
    EnvTonemapController(EnvTonemapController),
    #[serde(rename = "env_wind")]
    EnvWind(EnvWind),
    #[serde(rename = "filter_activator_class")]
    #[serde(borrow)]
    FilterActivatorClass(FilterActivatorClass<'a>),
    #[serde(rename = "filter_activator_name")]
    #[serde(borrow)]
    FilterActivatorName(FilterActivatorName<'a>),
    #[serde(rename = "filter_damage_type")]
    #[serde(borrow)]
    FilterDamageType(FilterDamageType<'a>),
    #[serde(rename = "filter_multi")]
    #[serde(borrow)]
    FilterMulti(FilterMulti<'a>),
    #[serde(rename = "func_areaportalwindow")]
    #[serde(borrow)]
    FuncAreaportalwindow(FuncAreaportalwindow<'a>),
    #[serde(rename = "func_breakable")]
    #[serde(borrow)]
    FuncBreakable(FuncBreakable<'a>),
    #[serde(rename = "func_button")]
    #[serde(borrow)]
    FuncButton(FuncButton<'a>),
    #[serde(rename = "func_buyzone")]
    #[serde(borrow)]
    FuncBuyzone(FuncBuyzone<'a>),
    #[serde(rename = "func_conveyor")]
    #[serde(borrow)]
    FuncConveyor(FuncConveyor<'a>),
    #[serde(rename = "func_door_rotating")]
    #[serde(borrow)]
    FuncDoorRotating(FuncDoorRotating<'a>),
    #[serde(rename = "func_dustcloud")]
    #[serde(borrow)]
    FuncDustcloud(FuncDustcloud<'a>),
    #[serde(rename = "func_movelinear")]
    #[serde(borrow)]
    FuncMovelinear(FuncMovelinear<'a>),
    #[serde(rename = "func_physbox")]
    #[serde(borrow)]
    FuncPhysbox(FuncPhysbox<'a>),
    #[serde(rename = "func_physbox_multiplayer")]
    #[serde(borrow)]
    FuncPhysboxMultiplayer(FuncPhysboxMultiplayer<'a>),
    #[serde(rename = "func_precipitation")]
    #[serde(borrow)]
    FuncPrecipitation(FuncPrecipitation<'a>),
    #[serde(rename = "func_rot_button")]
    #[serde(borrow)]
    FuncRotButton(FuncRotButton<'a>),
    #[serde(rename = "func_rotating")]
    #[serde(borrow)]
    FuncRotating(FuncRotating<'a>),
    #[serde(rename = "func_smokevolume")]
    #[serde(borrow)]
    FuncSmokevolume(FuncSmokevolume<'a>),
    #[serde(rename = "func_tracktrain")]
    #[serde(borrow)]
    FuncTracktrain(FuncTracktrain<'a>),
    #[serde(rename = "func_wall")]
    #[serde(borrow)]
    FuncWall(FuncWall<'a>),
    #[serde(rename = "func_wall_toggle")]
    #[serde(borrow)]
    FuncWallToggle(FuncWallToggle<'a>),
    #[serde(rename = "func_water_analog")]
    #[serde(borrow)]
    FuncWaterAnalog(FuncWaterAnalog<'a>),
    #[serde(rename = "game_player_equip")]
    GamePlayerEquip(GamePlayerEquip),
    #[serde(rename = "game_ui")]
    #[serde(borrow)]
    GameUi(GameUi<'a>),
    #[serde(rename = "game_weapon_manager")]
    #[serde(borrow)]
    GameWeaponManager(GameWeaponManager<'a>),
    #[serde(rename = "info_ladder")]
    InfoLadder(InfoLadder),
    #[serde(rename = "info_player_start")]
    InfoPlayerStart(InfoPlayerStart),
    #[serde(rename = "info_player_counterterrorist")]
    CounterTerroristSpawn(CounterTerroristSpawn),
    #[serde(rename = "info_player_terrorist")]
    InfoPlayerTerrorist(InfoPlayerTerrorist),
    #[serde(rename = "info_target")]
    #[serde(borrow)]
    InfoTarget(InfoTarget<'a>),
    #[serde(rename = "info_teleport_destination")]
    #[serde(borrow)]
    InfoTeleportDestination(InfoTeleportDestination<'a>),
    #[serde(rename = "infodecal")]
    #[serde(borrow)]
    Infodecal(Infodecal<'a>),
    #[serde(rename = "keyframe_rope")]
    #[serde(borrow)]
    KeyframeRope(KeyframeRope<'a>),
    #[serde(rename = "light")]
    Light(Light),
    #[serde(rename = "light_environment")]
    #[serde(borrow)]
    LightEnvironment(LightEnvironment<'a>),
    #[serde(rename = "logic_auto")]
    #[serde(borrow)]
    LogicAuto(LogicAuto<'a>),
    #[serde(rename = "logic_relay")]
    #[serde(borrow)]
    LogicRelay(LogicRelay<'a>),
    #[serde(rename = "logic_timer")]
    #[serde(borrow)]
    LogicTimer(LogicTimer<'a>),
    #[serde(rename = "math_counter")]
    #[serde(borrow)]
    MathCounter(MathCounter<'a>),
    #[serde(rename = "move_rope")]
    #[serde(borrow)]
    MoveRope(MoveRope<'a>),
    #[serde(rename = "path_track")]
    #[serde(borrow)]
    PathTrack(PathTrack<'a>),
    #[serde(rename = "phys_ballsocket")]
    #[serde(borrow)]
    PhysBallsocket(PhysBallsocket<'a>),
    #[serde(rename = "player_speedmod")]
    #[serde(borrow)]
    PlayerSpeedmod(PlayerSpeedmod<'a>),
    #[serde(rename = "player_weaponstrip")]
    #[serde(borrow)]
    PlayerWeaponstrip(PlayerWeaponstrip<'a>),
    #[serde(rename = "point_clientcommand")]
    #[serde(borrow)]
    PointClientcommand(PointClientcommand<'a>),
    #[serde(rename = "point_servercommand")]
    #[serde(borrow)]
    PointServercommand(PointServercommand<'a>),
    #[serde(rename = "point_spotlight")]
    #[serde(borrow)]
    PointSpotlight(PointSpotlight<'a>),
    #[serde(rename = "point_template")]
    #[serde(borrow)]
    PointTemplate(PointTemplate<'a>),
    #[serde(rename = "point_viewcontrol")]
    #[serde(borrow)]
    PointViewcontrol(PointViewcontrol<'a>),
    #[serde(rename = "prop_dynamic")]
    #[serde(borrow)]
    PropDynamic(PropDynamic<'a>),
    #[serde(rename = "prop_physics")]
    #[serde(borrow)]
    PropPhysics(PropPhysics<'a>),
    #[serde(rename = "prop_physics_multiplayer")]
    #[serde(borrow)]
    PropPhysicsMultiplayer(PropPhysicsMultiplayer<'a>),
    #[serde(rename = "prop_physics_override")]
    #[serde(borrow)]
    PropPhysicsOverride(PropPhysicsOverride<'a>),
    #[serde(rename = "prop_ragdoll")]
    #[serde(borrow)]
    PropRagdoll(PropRagdoll<'a>),
    #[serde(rename = "shadow_control")]
    ShadowControl(ShadowControl),
    #[serde(rename = "sky_camera")]
    SkyCamera(SkyCamera),
    #[serde(rename = "trigger_gravity")]
    #[serde(borrow)]
    TriggerGravity(TriggerGravity<'a>),
    #[serde(rename = "trigger_hurt")]
    #[serde(borrow)]
    TriggerHurt(TriggerHurt<'a>),
    #[serde(rename = "trigger_look")]
    #[serde(borrow)]
    TriggerLook(TriggerLook<'a>),
    #[serde(rename = "trigger_multiple")]
    #[serde(borrow)]
    TriggerMultiple(TriggerMultiple<'a>),
    #[serde(rename = "trigger_once")]
    #[serde(borrow)]
    TriggerOnce(TriggerOnce<'a>),
    #[serde(rename = "trigger_push")]
    #[serde(borrow)]
    TriggerPush(TriggerPush<'a>),
    #[serde(rename = "trigger_soundscape")]
    #[serde(borrow)]
    TriggerSoundscape(TriggerSoundscape<'a>),
    #[serde(rename = "trigger_teleport")]
    #[serde(borrow)]
    TriggerTeleport(TriggerTeleport<'a>),
    #[serde(rename = "water_lod_control")]
    WaterLodControl(WaterLodControl),
    #[serde(rename = "weapon_ak47")]
    #[serde(borrow)]
    WeaponAk47(WeaponAk47<'a>),
    #[serde(rename = "weapon_awp")]
    #[serde(borrow)]
    WeaponAwp(WeaponAwp<'a>),
    #[serde(rename = "weapon_deagle")]
    WeaponDeagle(WeaponDeagle),
    #[serde(rename = "weapon_elite")]
    #[serde(borrow)]
    WeaponElite(WeaponElite<'a>),
    #[serde(rename = "weapon_famas")]
    WeaponFamas(WeaponFamas),
    #[serde(rename = "weapon_fiveseven")]
    #[serde(borrow)]
    WeaponFiveseven(WeaponFiveseven<'a>),
    #[serde(rename = "weapon_flashbang")]
    #[serde(borrow)]
    WeaponFlashbang(WeaponFlashbang<'a>),
    #[serde(rename = "weapon_g3sg1")]
    WeaponG3Sg1(WeaponG3Sg1),
    #[serde(rename = "weapon_glock")]
    #[serde(borrow)]
    WeaponGlock(WeaponGlock<'a>),
    #[serde(rename = "weapon_hegrenade")]
    #[serde(borrow)]
    WeaponHegrenade(WeaponHegrenade<'a>),
    #[serde(rename = "weapon_knife")]
    #[serde(borrow)]
    WeaponKnife(WeaponKnife<'a>),
    #[serde(rename = "weapon_m249")]
    #[serde(borrow)]
    WeaponM249(WeaponM249<'a>),
    #[serde(rename = "weapon_m3")]
    #[serde(borrow)]
    WeaponM3(WeaponM3<'a>),
    #[serde(rename = "weapon_m4a1")]
    WeaponM4A1(WeaponM4A1),
    #[serde(rename = "weapon_mac10")]
    WeaponMac10(WeaponMac10),
    #[serde(rename = "weapon_p90")]
    #[serde(borrow)]
    WeaponP90(WeaponP90<'a>),
    #[serde(rename = "weapon_scout")]
    #[serde(borrow)]
    WeaponScout(WeaponScout<'a>),
    #[serde(rename = "weapon_sg550")]
    WeaponSg550(WeaponSg550),
    #[serde(rename = "weapon_smokegrenade")]
    #[serde(borrow)]
    WeaponSmokegrenade(WeaponSmokegrenade<'a>),
    #[serde(rename = "weapon_tmp")]
    WeaponTmp(WeaponTmp),
    #[serde(rename = "weapon_ump45")]
    WeaponUmp45(WeaponUmp45),
    #[serde(rename = "weapon_usp")]
    #[serde(borrow)]
    WeaponUsp(WeaponUsp<'a>),
    #[serde(rename = "weapon_xm1014")]
    WeaponXm1014(WeaponXm1014),
    #[serde(rename = "worldspawn")]
    #[serde(borrow)]
    Worldspawn(Worldspawn<'a>),
}

#[derive(Debug, Clone, Deserialize)]
pub struct CounterTerroristSpawn {
    pub origin: Vector,
    pub angles: Angles,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Worldspawn<'a> {
    pub maxpropscreenwidth: i32, //"-1",
    pub skyname: &'a str,
    pub detailmaterial: &'a str,
    pub world_mins: Vector,
    pub world_maxs: Vector,
    pub detailvbsp: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InfoPlayerTerrorist {
    pub origin: Vector,
    pub angles: Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerTeleport<'a> {
    pub target: &'a str,

    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub origin: Vector,
    pub spawnflags: u32,
    pub model: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InfoTeleportDestination<'a> {
    pub origin: Vector,
    pub targetname: &'a str,
    pub angles: Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncWaterAnalog<'a> {
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub model: &'a str,
    pub waveheight: f32,
    pub _minlight: f32,
    pub movedir: Vector,
    pub movedistance: f32,
    pub speed: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    pub renderamt: u8,
    pub origin: Vector,
    pub rendermode: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LightEnvironment<'a> {
    pub origin: Vector,
    pub _light: LightColor,
    pub angles: Vector,
    pub _ambient: LightColor,
    pub _lighthdr: LightColor,
    pub pitch: f32,
    pub _ambienthdr: &'a str, //"-1 -1 -1 1"
}

#[derive(Debug, Clone, Deserialize)]
pub struct GamePlayerEquip {
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub weapon_knife: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerHurt<'a> {
    pub damagecap: i32,
    pub model: &'a str,
    pub spawnflags: u32,
    pub damagetype: u32,
    pub origin: Vector,
    pub damage: i32,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub damagemodel: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncButton<'a> {
    pub movedir: Vector,
    pub onpressed: &'a str,
    pub spawnflags: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub unlocked_sentence: bool,
    pub speed: f32,
    pub wait: f32,
    pub sounds: u32,
    pub origin: Vector,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub lip: bool,
    pub model: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub locked_sentence: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub unlocked_sound: bool,
    pub renderamt: u8,
    pub rendermode: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub locked_sound: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub health: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponM4A1 {
    pub spawnflags: u32,
    pub angles: Vector,
    pub origin: Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WaterLodControl {
    pub cheapwaterenddistance: f32,
    pub cheapwaterstartdistance: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncWall<'a> {
    pub renderamt: u8,
    pub rendercolor: Color,
    pub rendermode: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    pub model: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvTonemapController {
    pub origin: Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSun<'a> {
    pub rendercolor: Color,
    pub target: &'a str,
    pub hdrcolorscale: f32,
    pub material: &'a str,
    pub size: f32,
    pub origin: Vector,
    pub angles: Vector,
    pub overlaymaterial: &'a str,
    pub overlaycolor: Color,
    pub overlaysize: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InfoTarget<'a> {
    pub spawnflags: u32,
    pub origin: Vector,
    pub targetname: &'a str,
    pub angles: Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponAwp<'a> {
    pub origin: Vector,
    pub angles: Vector,
    pub targetname: &'a str,
    pub spawnflags: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSpritetrail<'a> {
    pub endwidth: f32,
    pub lifetime: f32,
    pub parentname: &'a str,
    pub origin: Vector,
    pub rendercolor: Color,
    pub renderamt: u8,
    pub startwidth: f32,
    pub spritename: &'a str,
    pub rendermode: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponDeagle {
    pub origin: Vector,
    pub angles: Vector,
    pub ammo: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ShadowControl {
    pub origin: Vector,
    pub angles: Vector,
    pub distance: f32,
    pub color: Color,
    //pub hammerid: "2971"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponElite<'a> {
    //pub hammerid: "7248",
    pub targetname: &'a str,
    pub ammo: u32,
    pub spawnflags: u32,
    pub angles: Vector,
    pub origin: Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncRotating<'a> {
    pub spawnflags: u32,
    pub volume: u8,
    pub model: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub solidbsp: bool,
    pub dmg: i32,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    pub renderamt: u8,
    pub parentname: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub fanfriction: bool,
    pub origin: Vector,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    //pub hammerid: "7383",
    pub maxspeed: f32,
    pub angles: Vector,
    pub rendermode: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncBreakable<'a> {
    pub gibdir: Vector,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub performancemode: bool,
    pub explodemagnitude: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub pressuredelay: bool,
    pub origin: Vector,
    pub explodedamage: u8,
    pub material: u32,
    pub model: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub explosion: bool,
    pub propdata: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnobject: bool,
    pub exploderadius: f32,
    pub health: u8,
    pub rendermode: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    //pub hammerid: "8344",
    pub renderamt: u8,
    pub physdamagescale: f32,
    pub spawnflags: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    pub minhealthdmg: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponP90<'a> {
    pub ammo: u32,
    pub origin: Vector,
    pub spawnflags: u32,
    pub targetname: &'a str,
    //pub hammerid: "15611",
    pub angles: Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponGlock<'a> {
    pub targetname: &'a str,
    //pub hammerid: "19772",
    pub origin: Vector,
    pub angles: Vector,
    pub spawnflags: u32,
    pub ammo: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogicAuto<'a> {
    //pub hammerid: "20253",
    pub origin: Vector,
    pub spawnflags: u32,
    pub onmapspawn: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncWallToggle<'a> {
    pub rendercolor: Color,
    pub renderamt: u8,
    //pub hammerid: "20341",
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    pub model: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    pub spawnflags: u32,
    pub rendermode: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub targetname: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponM249<'a> {
    pub angles: Vector,
    pub targetname: &'a str,
    pub origin: Vector,
    //pub hammerid: "24744",
    pub spawnflags: u32,
    pub ammo: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponHegrenade<'a> {
    pub origin: Vector,
    //pub hammerid: "28302",
    pub targetname: &'a str,
    pub angles: Vector,
    pub spawnflags: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PointViewcontrol<'a> {
    pub wait: f32,
    pub angles: Vector,
    pub spawnflags: u32,
    pub acceleration: f32,
    pub target: &'a str,
    pub deceleration: f32,
    pub origin: Vector,
    //pub hammerid: "36815"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponScout<'a> {
    pub spawnflags: u32,
    pub targetname: &'a str,
    pub origin: Vector,
    pub angles: Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PointTemplate<'a> {
    pub template01: &'a str,
    pub spawnflags: u32,
    pub targetname: &'a str,
    pub origin: Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncDoorRotating<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    pub unlocked_sentence: bool,
    pub renderamt: u8,
    pub rendermode: u32,
    pub spawnflags: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub forceclosed: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub locked_sentence: bool,
    pub dmg: i32,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnpos: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub ignoredebris: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub solidbsp: bool,
    pub rendercolor: Color,
    pub model: &'a str,
    pub targetname: &'a str,
    pub health: u8,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub lip: bool,
    pub speed: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub loopmovesound: bool,
    pub angles: Vector,
    pub wait: f32,
    pub distance: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerPush<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    pub alternateticksfix: bool,
    pub speed: f32,
    pub model: &'a str,
    pub origin: Vector,
    pub spawnflags: u32,
    pub pushdir: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncPhysbox<'a> {
    pub preferredcarryangles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnobject: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    pub spawnflags: u32,
    pub explodedamage: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub performancemode: bool,
    pub gibdir: Vector,
    pub propdata: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub damagetoenablemotion: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub massscale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub parentname: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub notsolid: bool,
    pub damagetype: u32,
    pub exploderadius: f32,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub explosion: bool,
    pub material: u32,
    pub rendermode: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub forcetoenablemotion: bool,
    pub explodemagnitude: f32,
    pub health: u8,
    pub model: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub pressuredelay: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponSmokegrenade<'a> {
    pub origin: Vector,
    pub angles: Vector,
    pub spawnflags: u32,
    pub targetname: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FilterDamageType<'a> {
    pub origin: Vector,
    pub negated: Negated,
    pub targetname: &'a str,
    pub damagetype: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvFogController {
    pub fogend: f32,
    pub fogcolor: Color,
    pub origin: Vector,
    pub fogcolor2: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub fogenable: bool,
    pub farz: f32,
    pub spawnflags: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub use_angles: bool,
    pub fogstart: f32,
    pub mindxlevel: u8,
    pub fogdir: Vector,
    pub maxdxlevel: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub fogblend: bool,
    pub foglerptime: f32,
    pub angles: Vector,
    pub fogmaxdensity: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncConveyor<'a> {
    pub spawnflags: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    pub rendercolor: Color,
    pub rendermode: u32,
    pub angles: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    pub speed: f32,
    pub renderamt: u8,
    pub model: &'a str,
    pub movedir: Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FilterActivatorName<'a> {
    pub origin: Vector,
    pub negated: Negated,
    pub targetname: &'a str,
    pub filtername: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerMultiple<'a> {
    pub model: &'a str,

    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub wait: f32,
    pub spawnflags: u32,
    pub ontrigger: &'a str,
    pub origin: Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FilterMulti<'a> {
    pub targetname: &'a str,
    pub filter02: &'a str,
    pub origin: Vector,
    pub negated: Negated,
    pub filter01: &'a str,
    //pub hammerid: "49342",
    #[serde(deserialize_with = "bool_from_int")]
    pub filtertype: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PropPhysicsOverride<'a> {
    pub origin: Vector,
    pub spawnflags: u32,
    pub skin: u16,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub performancemode: bool,
    pub minhealthdmg: u8,
    pub fadescale: f32,
    pub exploderadius: f32,
    pub explodedamage: u8,
    pub damagetype: u32,
    pub angles: Vector,
    //pub hammerid: "89676",
    pub inertiascale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub forcetoenablemotion: bool,
    pub model: &'a str,
    pub massscale: f32,
    pub maxdxlevel: u8,
    pub fademindist: f32,
    pub fademaxdist: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub damagetoenablemotion: bool,
    pub parentname: &'a str,
    pub shadowcastdist: f32,
    pub health: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub pressuredelay: bool,
    pub mindxlevel: u8,
    pub physdamagescale: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PointServercommand<'a> {
    pub origin: Vector,
    pub targetname: &'a str,
    //pub hammerid: "90371"
}

#[derive(Debug, Clone, Deserialize)]
pub struct PointClientcommand<'a> {
    pub targetname: &'a str,
    //pub hammerid: "90377",
    pub origin: Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GameWeaponManager<'a> {
    pub weaponname: &'a str,
    pub maxpieces: u8,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub ammomod: bool,
    //pub hammerid: "90603"
}

#[derive(Debug, Clone, Deserialize)]
pub struct FilterActivatorClass<'a> {
    pub origin: Vector,
    pub targetname: &'a str,
    pub negated: Negated,
    pub filterclass: &'a str,
    //pub hammerid: "91384",
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerSpeedmod<'a> {
    //pub hammerid: "91426",
    pub origin: Vector,
    pub spawnflags: u32,
    pub targetname: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvFire {
    pub spawnflags: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub ignitionpoint: f32,
    pub health: u8,
    pub origin: Vector,
    pub damagescale: f32,
    //pub hammerid: "101536",
    pub firesize: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub firetype: bool,
    pub fireattack: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerWeaponstrip<'a> {
    pub targetname: &'a str,
    //pub hammerid: "107702",
    pub origin: Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncBuyzone<'a> {
    pub teamnum: u8,
    pub model: &'a str,
    //pub hammerid: "113521"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSoundscape<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    //pub hammerid: "133638",
    pub origin: Vector,
    pub radius: f32,
    pub soundscape: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSprite<'a> {
    pub glowproxysize: f32,
    pub rendermode: u32,
    pub mindxlevel: u8,
    pub framerate: f32,
    //pub hammerid: "134683",
    pub maxdxlevel: u8,
    pub hdrcolorscale: f32,
    pub origin: Vector,
    pub renderamt: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    pub model: &'a str,
    pub spawnflags: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub rendercolor: Color,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MathCounter<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub targetname: &'a str,
    pub max: i32,
    pub origin: Vector,
    pub onhitmax: &'a str,
    //pub hammerid: "134898",
    pub startvalue: i32,
    pub min: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponKnife<'a> {
    pub angles: Vector,
    //pub hammerid: "176620",
    pub spawnflags: u32,
    pub targetname: &'a str,
    pub origin: Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvEntityMaker<'a> {
    pub entitytemplate: &'a str,
    pub angles: Vector,
    //pub hammerid: "176738",
    pub spawnflags: u32,
    pub postspawndirection: Vector,
    pub origin: Vector,
    pub targetname: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub postspawninheritangles: bool,
    pub postspawndirectionvariance: f32,
    pub postspawnspeed: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSoundscapeTriggerable<'a> {
    pub soundscape: &'a str,
    pub radius: f32,
    //pub hammerid: "179655",
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub targetname: &'a str,
    pub origin: Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvHudhint<'a> {
    pub origin: Vector,
    pub spawnflags: u32,
    //pub hammerid: "183578",
    pub targetname: &'a str,
    pub message: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PropDynamic<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    pub solid: bool,
    pub mindxlevel: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    //pub hammerid: "190009",
    #[serde(deserialize_with = "bool_from_int")]
    pub randomanimation: bool,
    pub explodedamage: u8,
    pub angles: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub pressuredelay: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablebonefollowers: bool,
    pub rendermode: u32,
    pub exploderadius: f32,
    pub fadescale: f32,
    pub spawnflags: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub renderamt: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub performancemode: bool,
    pub maxanimtime: f32,
    pub skin: u16,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    pub model: &'a str,
    pub fademindist: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub setbodygroup: bool,
    pub fademaxdist: f32,
    pub origin: Vector,
    pub minanimtime: f32,
    pub maxdxlevel: u8,

    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Infodecal<'a> {
    pub texture: &'a str,
    //pub hammerid: "3739",
    pub angles: Vector,
    pub origin: Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerSoundscape<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub soundscape: &'a str,
    pub model: &'a str,
    pub origin: Vector,
    pub spawnflags: u32,
    //pub hammerid: "7740"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSpark<'a> {
    pub spawnflags: u32,
    pub targetname: &'a str,
    pub maxdelay: f32,
    pub magnitude: f32,
    pub angles: Vector,
    //pub hammerid: "34153",
    pub origin: Vector,
    pub traillength: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogicTimer<'a> {
    pub spawnflags: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub userandomtime: bool,
    pub lowerrandombound: i32,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    //pub hammerid: "34188",
    pub ontimer: &'a str,
    pub upperrandombound: i32,
    pub origin: Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PropPhysicsMultiplayer<'a> {
    pub fadescale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub spawnflags: u32,
    pub fademaxdist: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub physicsmode: bool,
    pub inertiascale: f32,
    pub massscale: f32,
    pub shadowcastdist: f32,
    pub explodedamage: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    pub angles: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub performancemode: bool,
    pub mindxlevel: u8,
    pub renderamt: u8,
    //pub hammerid: "35774",
    pub skin: u16,
    pub rendermode: u32,
    pub minhealthdmg: u8,
    pub maxdxlevel: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub forcetoenablemotion: bool,
    pub physdamagescale: f32,
    pub exploderadius: f32,
    pub damagetype: u32,
    pub model: &'a str,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub pressuredelay: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    pub fademindist: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub damagetoenablemotion: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvWind {
    pub origin: Vector,
    pub maxgustdelay: f32,
    pub mingustdelay: f32,
    pub maxgust: f32,
    pub minwind: f32,
    pub gustduration: f32,
    pub mingust: f32,
    pub angles: Vector,
    //pub hammerid: "38005",
    pub maxwind: f32,
    pub gustdirchange: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvDetailController {
    //pub hammerid: "207110",
    pub origin: Vector,
    pub fademindist: f32,
    pub fademaxdist: f32,
    pub angles: Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InfoPlayerStart {
    pub angles: Vector,
    pub origin: Vector,
    //pub hammerid: "259724",
    pub spawnflags: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PropPhysics<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    pub pressuredelay: bool,
    pub explodedamage: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub damagetoenablemotion: bool,
    pub inertiascale: f32,
    pub spawnflags: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub performancemode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub forcetoenablemotion: bool,
    pub fademindist: f32,
    pub minhealthdmg: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    pub exploderadius: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub mindxlevel: u8,
    pub massscale: f32,
    pub physdamagescale: f32,
    pub shadowcastdist: f32,
    pub origin: Vector,
    //pub hammerid: "487103",
    pub damagetype: u32,
    pub fadescale: f32,
    pub rendermode: u32,
    pub renderamt: u8,
    pub skin: u16,
    pub model: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    pub angles: Vector,
    pub fademaxdist: f32,
    pub rendercolor: Color,
    pub maxdxlevel: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponAk47<'a> {
    pub angles: Vector,
    pub targetname: &'a str,
    pub ammo: u32,
    pub origin: Vector,
    pub spawnflags: u32,
    //pub hammerid: "155595"
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncMovelinear<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub origin: Vector,
    pub rendermode: u32,
    pub parentname: &'a str,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub blockdamage: bool,
    pub model: &'a str,
    pub movedir: Vector,
    pub speed: f32,
    //pub hammerid: "155597",
    pub renderamt: u8,
    pub spawnflags: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub startposition: bool,
    pub movedistance: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncPrecipitation<'a> {
    //pub hammerid: "288365",
    pub renderamt: u8,
    pub preciptype: u8,
    pub rendercolor: Color,
    pub model: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvLightglow {
    pub hdrcolorscale: f32,
    pub origin: Vector,
    pub angles: Vector,
    pub horizontalglowsize: f32,
    pub verticalglowsize: f32,
    pub maxdist: f32,
    //pub hammerid: "156600",
    pub outermaxdist: f32,
    pub spawnflags: u32,
    pub glowproxysize: f32,
    pub mindist: f32,
    pub rendercolor: Color,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSmokestack<'a> {
    pub rendercolor: Color,
    pub roll: f32,
    pub windangle: f32,
    pub twist: f32,
    pub startsize: f32,
    pub renderamt: u8,
    pub basespread: f32,
    pub angles: Vector,
    pub initialstate: u8,
    pub jetlength: f32,
    pub endsize: f32,
    pub windspeed: f32,
    pub smokematerial: &'a str,
    pub origin: Vector,
    pub rate: f32,
    pub spreadspeed: f32,
    //pub hammerid: "129270",
    pub speed: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncSmokevolume<'a> {
    pub color2: Color,
    pub particledrawwidth: f32,
    //pub hammerid: "31779",
    pub particlespacingdistance: f32,
    pub color1: Color,
    pub spawnflags: u32,
    pub rotationspeed: f32,
    pub movementspeed: f32,
    pub material: &'a str,
    pub densityrampspeed: f32,
    pub density: f32,
    pub model: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvEmbers<'a> {
    pub angles: Vector,
    pub spawnflags: u32,
    //pub hammerid: "363584",
    pub model: &'a str,
    pub speed: f32,
    pub lifetime: f32,
    pub rendercolor: Color,
    pub density: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncDustcloud<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    pub frozen: bool,
    pub sizemax: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    //pub hammerid: "370009",
    pub lifetimemin: f32,
    pub lifetimemax: f32,
    pub color: Color,
    pub distmax: f32,
    pub alpha: u8,
    pub model: &'a str,
    pub speedmax: f32,
    pub sizemin: f32,
    pub spawnrate: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerOnce<'a> {
    pub origin: Vector,
    pub model: &'a str,
    //pub hammerid: "387965",
    pub angles: Vector,

    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub ontrigger: &'a str,
    pub spawnflags: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PointSpotlight<'a> {
    pub hdrcolorscale: f32,
    pub angles: Vector,
    pub spotlightlength: f32,
    pub spotlightwidth: f32,
    pub renderamt: u8,
    pub targetname: &'a str,
    pub spawnflags: u32,
    pub origin: Vector,
    pub rendercolor: Color,
    //pub hammerid: "393222"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponTmp {
    pub maxdxlevel: u8,
    pub angles: Vector,
    pub fadescale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub shadowcastdist: f32,
    //pub hammerid: "402780",
    pub fademindist: f32,
    pub mindxlevel: u8,
    pub rendermode: u32,
    pub renderamt: u8,
    pub rendercolor: Color,
    pub fademaxdist: f32,
    pub origin: Vector,
    pub spawnflags: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    pub ammo: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponXm1014 {
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub rendercolor: Color,
    //pub hammerid: "402832",
    pub spawnflags: u32,
    pub mindxlevel: u8,
    pub ammo: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    pub renderamt: u8,
    pub origin: Vector,
    pub fademaxdist: f32,
    pub rendermode: u32,
    pub maxdxlevel: u8,
    pub fadescale: f32,
    pub shadowcastdist: f32,
    pub fademindist: f32,
    pub angles: Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponMac10 {
    pub spawnflags: u32,
    pub rendermode: u32,
    pub fademindist: f32,
    pub ammo: u32,
    pub angles: Vector,
    pub shadowcastdist: f32,
    pub rendercolor: Color,
    pub fademaxdist: f32,
    //pub hammerid: "403143",
    pub origin: Vector,
    pub renderamt: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    pub mindxlevel: u8,
    pub fadescale: f32,
    pub maxdxlevel: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponUmp45 {
    pub rendermode: u32,
    pub spawnflags: u32,
    pub shadowcastdist: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub fademaxdist: f32,
    //pub hammerid: "403301",
    pub ammo: u32,
    pub mindxlevel: u8,
    pub renderamt: u8,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    pub fademindist: f32,
    pub angles: Vector,
    pub maxdxlevel: u8,
    pub fadescale: f32,
    pub rendercolor: Color,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponFamas {
    //pub hammerid: "403360",
    pub shadowcastdist: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub rendermode: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub fademaxdist: f32,
    pub maxdxlevel: u8,
    pub renderamt: u8,
    pub mindxlevel: u8,
    pub angles: Vector,
    pub fadescale: f32,
    pub spawnflags: u32,
    pub rendercolor: Color,
    pub fademindist: f32,
    pub ammo: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponG3Sg1 {
    pub angles: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    pub rendermode: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub fademaxdist: f32,
    pub spawnflags: u32,
    pub fadescale: f32,
    pub ammo: u32,
    //pub hammerid: "403518",
    pub renderamt: u8,
    pub fademindist: f32,
    pub origin: Vector,
    pub mindxlevel: u8,
    pub rendercolor: Color,
    pub shadowcastdist: f32,
    pub maxdxlevel: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponSg550 {
    pub renderamt: u8,
    //pub hammerid: "403621",
    pub origin: Vector,
    pub maxdxlevel: u8,
    pub fadescale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub rendermode: u32,
    pub spawnflags: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    pub fademaxdist: f32,
    pub rendercolor: Color,
    pub mindxlevel: u8,
    pub ammo: u32,
    pub angles: Vector,
    pub fademindist: f32,
    pub shadowcastdist: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponFlashbang<'a> {
    pub origin: Vector,
    pub fademindist: f32,
    pub spawnflags: u32,
    //pub hammerid: "416160",
    pub renderamt: u8,
    pub fadescale: f32,
    pub fademaxdist: f32,
    pub angles: Vector,
    pub targetname: &'a str,
    pub rendercolor: Color,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvFireTrail<'a> {
    pub targetname: &'a str,
    //pub hammerid: "425179",
    pub origin: Vector,
    pub parentname: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InfoLadder {
    //"hammerid": "2854",
    #[serde(rename = "maxs.y")]
    pub maxs_y: f32,
    #[serde(rename = "mins.z")]
    pub mins_z: f32,
    #[serde(rename = "maxs.x")]
    pub maxs_x: f32,
    #[serde(rename = "mins.y")]
    pub mins_y: f32,
    #[serde(rename = "maxs.z")]
    pub maxs_z: f32,
    #[serde(rename = "mins.x")]
    pub mins_x: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponM3<'a> {
    pub ammo: u32,
    //pub hammerid: "176561",
    pub angles: Vector,
    pub spawnflags: u32,
    pub targetname: &'a str,
    pub origin: Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponFiveseven<'a> {
    pub origin: Vector,
    pub ammo: u32,
    //pub hammerid: "177007",
    pub targetname: &'a str,
    pub spawnflags: u32,
    pub angles: Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MoveRope<'a> {
    pub spawnflags: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub barbed: bool,
    pub slack: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub nowind: bool,
    pub origin: Vector,
    pub subdiv: u8,
    pub movespeed: f32,
    pub width: f32,
    pub positioninterpolator: u8,
    pub mindxlevel: u8,
    pub ropematerial: &'a str,
    pub texturescale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub collide: bool,
    //pub hammerid: "129625",
    pub angles: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub breakable: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub r#type: bool,
    pub maxdxlevel: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub dangling: bool,
    pub targetname: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvFade<'a> {
    pub spawnflags: u32,
    pub rendercolor: Color,
    pub origin: Vector,
    //pub hammerid: "262982",
    pub duration: f32,
    pub holdtime: f32,
    pub renderamt: u8,
    pub targetname: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncAreaportalwindow<'a> {
    pub translucencylimit: f32,
    pub fadestartdist: f32,
    pub fadedist: f32,
    pub portalnumber: u16,
    pub target: &'a str,
    pub portalversion: u8,
    //pub hammerid: "270454"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvLaser<'a> {
    pub texturescroll: f32,
    pub dissolvetype: &'a str,
    pub lasertarget: &'a str,
    pub width: f32,
    pub rendercolor: Color,
    pub renderamt: u8,
    pub texture: &'a str,
    pub damage: i32,
    //pub hammerid: "731916",
    pub origin: Vector,
    pub spawnflags: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogicRelay<'a> {
    pub origin: Vector,
    //pub hammerid: "805602",
    pub ontrigger: &'a str,
    pub targetname: &'a str,
    pub spawnflags: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PathTrack<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    pub orientationtype: bool,
    pub angles: Vector,
    pub spawnflags: u32,
    pub target: &'a str,
    //pub hammerid: "30225",
    pub targetname: &'a str,
    pub origin: Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncTracktrain<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    pub velocitytype: bool,
    pub rendermode: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub orientationtype: bool,
    pub movesoundmaxpitch: u8,
    pub movesoundmintime: f32,
    pub height: f32,
    pub target: &'a str,
    pub speed: f32,
    pub dmg: i32,
    pub renderamt: u8,
    pub targetname: &'a str,
    pub movesound: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    pub movesoundminpitch: u8,
    //pub hammerid: "30270",
    pub rendercolor: Color,
    pub volume: u8,
    pub wheels: u8,
    pub bank: i16,
    pub model: &'a str,
    pub spawnflags: u32,
    pub movesoundmaxtime: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub startspeed: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSteam<'a> {
    pub rate: f32,
    pub spawnflags: u32,
    pub speed: f32,
    pub startsize: f32,
    pub renderamt: u8,
    pub endsize: f32,
    pub targetname: &'a str,
    pub rollspeed: f32,
    //pub hammerid: "65082",
    pub angles: Vector,
    pub origin: Vector,
    pub rendercolor: Color,
    pub spreadspeed: f32,
    pub jetlength: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvShake<'a> {
    pub amplitude: f32,
    pub targetname: &'a str,
    pub origin: Vector,
    pub duration: f32,
    pub spawnflags: u32,
    pub frequency: f32,
    pub radius: f32,
    //pub hammerid: "105159"
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncRotButton<'a> {
    pub distance: f32,
    pub sounds: u32,
    pub angles: Vector,
    pub origin: Vector,
    pub onpressed: &'a str,
    pub wait: f32,

    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub targetname: &'a str,
    pub speed: f32,
    //pub hammerid: "239116",
    pub health: u8,
    pub spawnflags: u32,
    pub model: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerGravity<'a> {
    //pub hammerid: "259593",
    pub origin: Vector,
    pub spawnflags: u32,
    pub model: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub gravity: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SkyCamera {
    pub fogdir: Vector,
    pub scale: f32,
    pub fogstart: f32,
    pub fogend: f32,
    pub fogcolor2: Color,
    //pub hammerid: "361080",
    pub angles: Vector,
    pub fogcolor: Color,
    pub origin: Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GameUi<'a> {
    pub fieldofview: f32,
    pub origin: Vector,
    pub pressedforward: &'a str,
    pub playeroff: &'a str,
    pub unpressedforward: &'a str,
    pub spawnflags: u32,
    //pub hammerid: "435027",
    pub unpressedmoveright: &'a str,
    pub unpressedmoveleft: &'a str,
    pub unpressedback: &'a str,
    pub pressedmoveright: &'a str,
    pub pressedback: &'a str,
    pub pressedmoveleft: &'a str,
    pub targetname: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncPhysboxMultiplayer<'a> {
    pub targetname: &'a str,
    pub health: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub pressuredelay: bool,
    pub damagetype: u32,
    pub material: u32,
    pub preferredcarryangles: Angles,
    pub origin: Vector,
    pub exploderadius: f32,
    pub propdata: u32,
    pub model: &'a str,
    pub rendermode: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub notsolid: bool,
    pub explodemagnitude: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub explosion: bool,
    pub gibdir: Vector,
    pub renderamt: u8,
    //pub hammerid: "435046",
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub performancemode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub forcetoenablemotion: bool,
    pub _minlight: f32,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub spawnflags: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub damagetoenablemotion: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnobject: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    pub explodedamage: u8,
    pub massscale: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerLook<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub looktime: f32,
    //pub hammerid: "491215",
    pub target: &'a str,
    pub spawnflags: u32,
    pub model: &'a str,
    pub timeout: f32,
    pub fieldofview: f32,
    pub ontrigger: &'a str,
    pub origin: Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KeyframeRope<'a> {
    pub origin: Vector,
    pub slack: f32,
    pub texturescale: f32,
    pub ropematerial: &'a str,
    pub movespeed: f32,
    pub angles: Vector,
    //pub hammerid: "543370",
    pub width: f32,
    pub targetname: &'a str,
    pub subdiv: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvShooter<'a> {
    pub m_flvariance: f32,
    pub angles: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub rendermode: u32,
    pub m_flvelocity: f32,
    pub origin: Vector,
    pub parentname: &'a str,
    pub shootmodel: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub nogibshadows: bool,
    pub spawnflags: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub massoverride: bool,
    pub skin: u16,
    pub renderamt: u8,
    pub gibangles: Vector,
    pub delay: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub simulation: bool,
    pub targetname: &'a str,
    pub m_flgiblife: f32,
    pub m_igibs: u32,
    pub gibgravityscale: f32,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    pub shootsounds: i8,
    //pub hammerid: "564233"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponUsp<'a> {
    //pub hammerid: "597821",
    pub origin: Vector,
    pub spawnflags: u32,
    pub targetname: &'a str,
    pub angles: Vector,
    pub ammo: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PropRagdoll<'a> {
    pub fadescale: f32,
    pub modelscale: f32,
    pub origin: Vector,
    pub model: &'a str,
    pub fademindist: f32,
    pub skin: u16,
    pub spawnflags: u32,
    pub angles: Vector,
    //pub hammerid: "739080",
    pub targetname: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PhysBallsocket<'a> {
    pub targetname: &'a str,
    pub origin: Vector,
    pub attach1: &'a str,
    //pub hammerid: "739103"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvBeam<'a> {
    pub origin: Vector,
    pub lightningend: &'a str,
    pub texturescroll: f32,
    pub hdrcolorscale: f32,
    //pub hammerid: "774042",
    pub spawnflags: u32,
    pub life: f32,
    pub decalname: &'a str,
    pub renderamt: u8,
    pub boltwidth: f32,
    pub noiseamplitude: f32,
    pub striketime: f32,
    pub texture: &'a str,
    pub lightningstart: &'a str,
    pub radius: f32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Light {
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub _lightscalehdr: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub _quadratic_attn: bool,
    pub _lighthdr: LightColor,
    pub _light: Color,
    //pub hammerid: "823486"
}
