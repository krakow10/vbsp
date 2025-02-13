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
pub struct EnvBeam<'a> {
    pub boltwidth: u8,
    pub decalname: &'a str,
    pub hdrcolorscale: f32,
    pub life: f32,
    pub lightningend: &'a str,
    pub lightningstart: &'a str,
    pub noiseamplitude: u8,
    pub origin: Vector,
    pub radius: u16,
    pub renderamt: u8,
    pub rendercolor: Color,
    pub spawnflags: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub striketime: bool,
    pub targetname: &'a str,
    pub texture: &'a str,
    pub texturescroll: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvDetailController {
    pub angles: Angles,
    pub fademaxdist: u16,
    pub fademindist: u8,
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvEmbers<'a> {
    pub angles: Angles,
    pub density: u8,
    pub lifetime: u8,
    pub model: &'a str,
    pub rendercolor: Color,
    pub spawnflags: u32,
    pub speed: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvEntityMaker<'a> {
    pub angles: Angles,
    pub entitytemplate: &'a str,
    pub onentityspawned: &'a str,
    pub origin: Vector,
    pub postspawndirection: Color,
    pub postspawndirectionvariance: f32,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub postspawninheritangles: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub postspawnspeed: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvFade<'a> {
    pub duration: f32,
    pub holdtime: f32,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    pub spawnflags: u8,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvFire {
    pub damagescale: f32,
    pub fireattack: u8,
    pub firesize: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub firetype: bool,
    pub health: u8,
    pub ignitionpoint: u8,
    pub origin: Vector,
    pub spawnflags: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvFireTrail<'a> {
    pub origin: Vector,
    pub parentname: &'a str,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvFogController<'a> {
    pub angles: Angles,
    pub farz: i32,
    #[serde(deserialize_with = "bool_from_int")]
    pub fogblend: bool,
    pub fogcolor: Color,
    pub fogcolor2: Color,
    pub fogdir: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub fogenable: bool,
    pub fogend: f32,
    #[serde(default)]
    pub foglerptime: Option<f32>,
    pub fogmaxdensity: f32,
    pub fogstart: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub mindxlevel: bool,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    pub use_angles: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvHudhint<'a> {
    pub message: &'a str,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvLaser<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    pub damage: bool,
    pub dissolvetype: &'a str,
    pub lasertarget: &'a str,
    #[serde(default)]
    pub noiseamplitude: Option<u8>,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub texture: &'a str,
    pub texturescroll: u8,
    pub width: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvLightglow {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    pub glowproxysize: bool,
    pub hdrcolorscale: f32,
    pub horizontalglowsize: u8,
    pub maxdist: u16,
    pub mindist: u8,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub outermaxdist: bool,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub verticalglowsize: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvShake<'a> {
    pub amplitude: u8,
    pub duration: u8,
    pub frequency: f32,
    pub origin: Vector,
    pub radius: u16,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvShooter<'a> {
    pub angles: Angles,
    pub delay: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    pub gibangles: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub gibgravityscale: bool,
    pub m_flgiblife: f32,
    pub m_flvariance: f32,
    pub m_flvelocity: u16,
    pub m_igibs: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub massoverride: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub nogibshadows: bool,
    pub origin: Vector,
    pub parentname: &'a str,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub rendermode: bool,
    pub shootmodel: &'a str,
    pub shootsounds: i32,
    #[serde(deserialize_with = "bool_from_int")]
    pub simulation: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub skin: bool,
    pub spawnflags: u8,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSmokestack<'a> {
    pub angles: Angles,
    pub basespread: u8,
    pub endsize: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub initialstate: bool,
    pub jetlength: u8,
    pub origin: Vector,
    pub rate: u8,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(default)]
    pub roll: Option<f32>,
    pub smokematerial: &'a str,
    pub speed: u8,
    pub spreadspeed: u8,
    pub startsize: u8,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub twist: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub windangle: bool,
    #[serde(default)]
    pub windspeed: Option<u8>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSoundscape<'a> {
    pub origin: Vector,
    #[serde(default)]
    pub position1: Option<&'a str>,
    #[serde(default)]
    pub position2: Option<&'a str>,
    pub radius: f32,
    pub soundscape: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSoundscapeTriggerable<'a> {
    pub origin: Vector,
    pub radius: u16,
    pub soundscape: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSpark<'a> {
    pub angles: Angles,
    pub magnitude: u8,
    pub maxdelay: u8,
    pub origin: Vector,
    pub spawnflags: u8,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub traillength: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSprite<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    pub framerate: f32,
    pub glowproxysize: f32,
    pub hdrcolorscale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub mindxlevel: bool,
    pub model: &'a str,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub renderfx: bool,
    pub rendermode: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSpritetrail<'a> {
    pub endwidth: f32,
    pub lifetime: f32,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub mindxlevel: bool,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub renderamt: u8,
    pub rendercolor: Color,
    pub rendermode: u8,
    pub spritename: &'a str,
    pub startwidth: f32,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSteam<'a> {
    pub angles: Angles,
    pub endsize: u8,
    pub jetlength: u16,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub r#type: bool,
    pub rate: u8,
    pub renderamt: u8,
    pub rendercolor: Color,
    pub rollspeed: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub speed: u8,
    pub spreadspeed: u8,
    pub startsize: u8,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSun<'a> {
    pub angles: Angles,
    pub hdrcolorscale: f32,
    pub material: &'a str,
    pub origin: Vector,
    pub overlaycolor: Color,
    pub overlaymaterial: &'a str,
    pub overlaysize: i32,
    #[serde(default)]
    pub pitch: Option<i32>,
    pub rendercolor: Color,
    pub size: u8,
    #[serde(default)]
    pub target: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub use_angles: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvTonemapController<'a> {
    pub origin: Vector,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvWind {
    pub angles: Angles,
    pub gustdirchange: u8,
    pub gustduration: u8,
    pub maxgust: u8,
    pub maxgustdelay: u8,
    pub maxwind: u8,
    pub mingust: u8,
    pub mingustdelay: u8,
    pub minwind: u8,
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterActivatorClass<'a> {
    pub filterclass: &'a str,
    pub negated: &'a str,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterActivatorName<'a> {
    pub filtername: &'a str,
    pub negated: &'a str,
    #[serde(default)]
    pub onfail: Option<&'a str>,
    #[serde(default)]
    pub onpass: Option<&'a str>,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterDamageType<'a> {
    pub damagetype: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub negated: bool,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterMulti<'a> {
    pub filter01: &'a str,
    pub filter02: &'a str,
    #[serde(default)]
    pub filter03: Option<&'a str>,
    #[serde(default)]
    pub filter04: Option<&'a str>,
    #[serde(default)]
    pub filter05: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    pub filtertype: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub negated: bool,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncAreaportalwindow<'a> {
    pub fadedist: u16,
    pub fadestartdist: u16,
    pub portalnumber: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub portalversion: bool,
    pub target: &'a str,
    pub translucencylimit: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncBreakable<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub explodedamage: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub explodemagnitude: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub exploderadius: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub explosion: bool,
    pub gibdir: Color,
    pub health: u16,
    pub material: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub minhealthdmg: bool,
    pub model: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub nodamageforces: bool,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub performancemode: bool,
    pub physdamagescale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub pressuredelay: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub propdata: bool,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub rendermode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnobject: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncButton<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub health: bool,
    pub lip: f32,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub locked_sentence: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub locked_sound: bool,
    pub model: &'a str,
    pub movedir: Color,
    #[serde(default)]
    pub ondamaged: Option<&'a str>,
    #[serde(default)]
    pub onin: Option<&'a str>,
    #[serde(default)]
    pub onout: Option<&'a str>,
    pub onpressed: &'a str,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub rendermode: u8,
    pub sounds: u8,
    pub spawnflags: u16,
    pub speed: f32,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub unlocked_sentence: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub unlocked_sound: bool,
    pub wait: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncBuyzone<'a> {
    pub model: &'a str,
    pub teamnum: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncConveyor<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    pub model: &'a str,
    pub movedir: Color,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub rendermode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub speed: u16,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncDoorRotating<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    pub distance: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub dmg: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub forceclosed: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub health: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub ignoredebris: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub lip: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub locked_sentence: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub loopmovesound: bool,
    pub model: &'a str,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub rendermode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub solidbsp: bool,
    pub spawnflags: u16,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnpos: bool,
    pub speed: u8,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    pub unlocked_sentence: bool,
    pub wait: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncDustcloud<'a> {
    pub alpha: u8,
    pub color: Color,
    pub distmax: u16,
    #[serde(deserialize_with = "bool_from_int")]
    pub frozen: bool,
    pub lifetimemax: u8,
    pub lifetimemin: u8,
    pub model: &'a str,
    pub sizemax: u8,
    pub sizemin: u8,
    pub spawnrate: u16,
    pub speedmax: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncMovelinear<'a> {
    pub blockdamage: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    pub model: &'a str,
    pub movedir: Vector,
    pub movedistance: u16,
    #[serde(default)]
    pub onfullyclosed: Option<&'a str>,
    #[serde(default)]
    pub onfullyopen: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub rendermode: u8,
    pub spawnflags: u8,
    pub speed: u8,
    pub startposition: f32,
    #[serde(default)]
    pub startsound: Option<&'a str>,
    #[serde(default)]
    pub stopsound: Option<&'a str>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncPhysbox<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    pub damagetoenablemotion: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub damagetype: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub explodedamage: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub explodemagnitude: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub exploderadius: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub explosion: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub forcetoenablemotion: bool,
    pub gibdir: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub health: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub massscale: bool,
    pub material: u8,
    pub model: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub notsolid: bool,
    pub origin: Vector,
    pub parentname: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub performancemode: bool,
    pub preferredcarryangles: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub pressuredelay: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub propdata: bool,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub rendermode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnobject: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncPhysboxMultiplayer<'a> {
    pub _minlight: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub damagetoenablemotion: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub damagetype: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub explodedamage: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub explodemagnitude: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub exploderadius: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub explosion: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub forcetoenablemotion: bool,
    pub gibdir: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub health: bool,
    pub massscale: f32,
    pub material: u8,
    pub model: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub notsolid: bool,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub performancemode: bool,
    pub preferredcarryangles: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub pressuredelay: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub propdata: bool,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub rendermode: bool,
    pub spawnflags: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnobject: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncPrecipitation<'a> {
    pub model: &'a str,
    pub preciptype: u8,
    pub renderamt: u8,
    pub rendercolor: Color,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncRotButton<'a> {
    pub angles: Angles,
    pub distance: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub health: bool,
    pub model: &'a str,
    pub onpressed: &'a str,
    pub origin: Vector,
    pub sounds: u8,
    pub spawnflags: u16,
    pub speed: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub targetname: &'a str,
    pub wait: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncRotating<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    pub dmg: f32,
    pub fanfriction: f32,
    pub maxspeed: f32,
    pub model: &'a str,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub rendermode: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub solidbsp: bool,
    pub spawnflags: u16,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub volume: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncSmokevolume<'a> {
    pub color1: Color,
    pub color2: Color,
    pub density: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub densityrampspeed: bool,
    pub material: &'a str,
    pub model: &'a str,
    pub movementspeed: u8,
    pub particledrawwidth: u8,
    pub particlespacingdistance: u8,
    pub rotationspeed: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncTracktrain<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    pub bank: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub dmg: bool,
    pub height: u8,
    pub model: &'a str,
    pub movesound: &'a str,
    pub movesoundmaxpitch: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub movesoundmaxtime: bool,
    pub movesoundminpitch: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub movesoundmintime: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub orientationtype: bool,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub rendermode: bool,
    pub spawnflags: u16,
    pub speed: u16,
    pub startspeed: u16,
    pub target: &'a str,
    pub targetname: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub velocitytype: bool,
    pub volume: u8,
    pub wheels: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncWall<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    pub model: &'a str,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub rendermode: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncWallToggle<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub _minlight: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    pub model: &'a str,
    pub renderamt: u8,
    pub rendercolor: Color,
    pub renderfx: u8,
    pub rendermode: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncWaterAnalog<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    pub model: &'a str,
    pub movedir: Color,
    pub movedistance: u8,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub rendermode: bool,
    pub speed: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startposition: bool,
    pub waveheight: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GamePlayerEquip<'a> {
    #[serde(default)]
    pub item_assaultsuit: Option<u8>,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub weapon_awp: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub weapon_glock: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub weapon_knife: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub weapon_p90: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub weapon_usp: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GameUi<'a> {
    pub fieldofview: f32,
    pub origin: Vector,
    pub playeroff: &'a str,
    pub pressedback: &'a str,
    pub pressedforward: &'a str,
    pub pressedmoveleft: &'a str,
    pub pressedmoveright: &'a str,
    pub spawnflags: u16,
    pub targetname: &'a str,
    pub unpressedback: &'a str,
    pub unpressedforward: &'a str,
    pub unpressedmoveleft: &'a str,
    pub unpressedmoveright: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GameWeaponManager<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    pub ammomod: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub maxpieces: bool,
    pub origin: Vector,
    pub weaponname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoLadder {
    #[serde(rename = "maxs.x")]
    pub maxs_x: f32,
    #[serde(rename = "maxs.y")]
    pub maxs_y: f32,
    #[serde(rename = "maxs.z")]
    pub maxs_z: f32,
    #[serde(rename = "mins.x")]
    pub mins_x: f32,
    #[serde(rename = "mins.y")]
    pub mins_y: f32,
    #[serde(rename = "mins.z")]
    pub mins_z: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoPlayerCounterterrorist {
    pub angles: Angles,
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoPlayerStart {
    pub angles: Angles,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoPlayerTerrorist {
    pub angles: Angles,
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoTarget<'a> {
    pub angles: Angles,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoTeleportDestination<'a> {
    pub angles: Angles,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Infodecal<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub origin: Vector,
    pub texture: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct KeyframeRope<'a> {
    pub angles: Angles,
    pub movespeed: u8,
    #[serde(default)]
    pub nextkey: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub ropematerial: &'a str,
    pub slack: u8,
    pub subdiv: u8,
    pub targetname: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub texturescale: bool,
    pub width: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Light {
    pub _light: Color,
    pub _lighthdr: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub _lightscalehdr: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub _quadratic_attn: bool,
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LightEnvironment {
    pub _ambient: Color,
    pub _ambienthdr: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub _ambientscalehdr: bool,
    pub _light: Color,
    pub _lighthdr: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub _lightscalehdr: bool,
    pub angles: Angles,
    pub origin: Vector,
    #[serde(default)]
    pub pitch: Option<i32>,
    #[serde(default)]
    pub sunspreadangle: Option<u8>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicAuto<'a> {
    pub onmapspawn: &'a str,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicRelay<'a> {
    pub ontrigger: &'a str,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicTimer<'a> {
    #[serde(default)]
    pub lowerrandombound: Option<u8>,
    pub ontimer: &'a str,
    pub origin: Vector,
    #[serde(default)]
    pub refiretime: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub upperrandombound: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub userandomtime: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MathCounter<'a> {
    pub max: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub min: bool,
    pub onhitmax: &'a str,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startvalue: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MoveRope<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub barbed: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub breakable: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub collide: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub dangling: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub mindxlevel: bool,
    pub movespeed: u8,
    #[serde(default)]
    pub nextkey: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub nowind: bool,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub positioninterpolator: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub r#type: bool,
    pub ropematerial: &'a str,
    pub slack: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    pub subdiv: u8,
    pub targetname: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub texturescale: bool,
    pub width: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PathTrack<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    pub orientationtype: bool,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    pub target: &'a str,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysBallsocket<'a> {
    pub attach1: &'a str,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PlayerSpeedmod<'a> {
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PlayerWeaponstrip<'a> {
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointClientcommand<'a> {
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointServercommand<'a> {
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointSpotlight<'a> {
    pub angles: Angles,
    pub hdrcolorscale: f32,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    pub spawnflags: u8,
    pub spotlightlength: f32,
    pub spotlightwidth: f32,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointTemplate<'a> {
    #[serde(default)]
    pub onentityspawned: Option<&'a str>,
    pub origin: Vector,
    pub spawnflags: u8,
    pub targetname: &'a str,
    pub template01: &'a str,
    #[serde(default)]
    pub template02: Option<&'a str>,
    #[serde(default)]
    pub template03: Option<&'a str>,
    #[serde(default)]
    pub template04: Option<&'a str>,
    #[serde(default)]
    pub template05: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointViewcontrol<'a> {
    pub acceleration: u16,
    pub angles: Angles,
    pub deceleration: u16,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub interpolatepositiontoplayer: bool,
    pub origin: Vector,
    pub spawnflags: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub speed: bool,
    #[serde(default)]
    pub target: Option<&'a str>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub wait: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropDynamic<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablebonefollowers: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub explodedamage: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub exploderadius: bool,
    pub fademaxdist: u16,
    pub fademindist: i32,
    #[serde(deserialize_with = "bool_from_int")]
    pub fadescale: bool,
    pub maxanimtime: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub maxdxlevel: bool,
    pub minanimtime: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub mindxlevel: bool,
    pub model: &'a str,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    pub performancemode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub pressuredelay: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub randomanimation: bool,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub rendermode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub setbodygroup: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub skin: bool,
    pub solid: u8,
    pub spawnflags: u16,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropPhysics<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    pub damagetoenablemotion: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub damagetype: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub explodedamage: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub exploderadius: bool,
    pub fademaxdist: u16,
    pub fademindist: u16,
    #[serde(deserialize_with = "bool_from_int")]
    pub fadescale: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub forcetoenablemotion: bool,
    pub inertiascale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub massscale: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub mindxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub minhealthdmg: bool,
    pub model: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub performancemode: bool,
    pub physdamagescale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub pressuredelay: bool,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub rendermode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub shadowcastdist: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub skin: bool,
    pub spawnflags: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropPhysicsMultiplayer<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    pub damagetoenablemotion: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub damagetype: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub explodedamage: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub exploderadius: bool,
    pub fademaxdist: u16,
    pub fademindist: i32,
    #[serde(deserialize_with = "bool_from_int")]
    pub fadescale: bool,
    pub forcetoenablemotion: u16,
    pub inertiascale: f32,
    pub massscale: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub mindxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub minhealthdmg: bool,
    pub model: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub performancemode: bool,
    pub physdamagescale: f32,
    #[serde(default)]
    pub physicsmode: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    pub pressuredelay: bool,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub rendermode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub shadowcastdist: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub skin: bool,
    pub spawnflags: u16,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropPhysicsOverride<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub body: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub damagetoenablemotion: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub damagetype: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub explodedamage: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub exploderadius: bool,
    pub fademaxdist: u16,
    pub fademindist: i32,
    #[serde(deserialize_with = "bool_from_int")]
    pub fadescale: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub forcetoenablemotion: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub health: bool,
    pub inertiascale: f32,
    pub massscale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub mindxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub minhealthdmg: bool,
    pub model: &'a str,
    #[serde(default)]
    pub modelscale: Option<f32>,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    pub performancemode: bool,
    pub physdamagescale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub pressuredelay: bool,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub rendermode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub shadowcastdist: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub skin: bool,
    pub spawnflags: u16,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropRagdoll<'a> {
    pub angles: Angles,
    pub fademindist: i32,
    #[serde(deserialize_with = "bool_from_int")]
    pub fadescale: bool,
    pub model: &'a str,
    pub modelscale: f32,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub skin: bool,
    pub spawnflags: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ShadowControl {
    pub angles: Angles,
    pub color: Color,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableallshadows: bool,
    pub distance: u8,
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct SkyCamera {
    pub angles: Angles,
    pub fogcolor: Color,
    pub fogcolor2: Color,
    pub fogdir: Color,
    pub fogend: f32,
    pub fogstart: f32,
    pub origin: Vector,
    pub scale: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerGravity<'a> {
    pub gravity: f32,
    pub model: &'a str,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerHurt<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub damage: f32,
    pub damagecap: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub damagemodel: bool,
    pub damagetype: u8,
    pub model: &'a str,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerLook<'a> {
    pub fieldofview: f32,
    pub looktime: u8,
    pub model: &'a str,
    pub ontrigger: &'a str,
    pub origin: Vector,
    pub spawnflags: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub target: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub timeout: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerMultiple<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub filtername: Option<&'a str>,
    pub model: &'a str,
    #[serde(default)]
    pub onendtouch: Option<&'a str>,
    #[serde(default)]
    pub onendtouchall: Option<&'a str>,
    #[serde(default)]
    pub onstarttouch: Option<&'a str>,
    #[serde(default)]
    pub onstarttouchall: Option<&'a str>,
    pub ontrigger: &'a str,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub spawnflags: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    #[serde(default)]
    pub target: Option<&'a str>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub wait: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerOnce<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub model: &'a str,
    pub ontrigger: &'a str,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerPush<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub alternateticksfix: bool,
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub filtername: Option<&'a str>,
    pub model: &'a str,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub pushdir: Vector,
    pub spawnflags: u8,
    pub speed: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerSoundscape<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub model: &'a str,
    pub origin: Vector,
    pub soundscape: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerTeleport<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub filtername: Option<&'a str>,
    #[serde(default)]
    pub model: Option<&'a str>,
    #[serde(default)]
    pub onendtouch: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub spawnflags: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub target: &'a str,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WaterLodControl {
    pub cheapwaterenddistance: f32,
    pub cheapwaterstartdistance: f32,
    #[serde(default)]
    pub origin: Option<Vector>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponAk47<'a> {
    pub ammo: u16,
    pub angles: Angles,
    #[serde(default)]
    pub onplayerpickup: Option<&'a str>,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponAwp<'a> {
    #[serde(default)]
    pub ammo: Option<u8>,
    pub angles: Angles,
    #[serde(default)]
    pub fademaxdist: Option<f32>,
    #[serde(default)]
    pub fademindist: Option<f32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    pub origin: Vector,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponDeagle {
    pub ammo: u8,
    pub angles: Angles,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponElite<'a> {
    pub ammo: u8,
    pub angles: Angles,
    #[serde(default)]
    pub fademaxdist: Option<f32>,
    #[serde(default)]
    pub fademindist: Option<f32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    pub origin: Vector,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponFamas {
    pub ammo: u8,
    pub angles: Angles,
    pub fademaxdist: f32,
    pub fademindist: f32,
    pub fadescale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub mindxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub rendermode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub shadowcastdist: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponFiveseven<'a> {
    pub ammo: u16,
    pub angles: Angles,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponFlashbang<'a> {
    pub angles: Angles,
    pub fademaxdist: f32,
    pub fademindist: f32,
    pub fadescale: f32,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponG3sg1 {
    pub ammo: u8,
    pub angles: Angles,
    pub fademaxdist: f32,
    pub fademindist: f32,
    pub fadescale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub mindxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub rendermode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub shadowcastdist: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponGlock<'a> {
    pub ammo: u16,
    pub angles: Angles,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponHegrenade<'a> {
    pub angles: Angles,
    #[serde(default)]
    pub fademaxdist: Option<f32>,
    #[serde(default)]
    pub fademindist: Option<f32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub mindxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub nodamageforces: bool,
    pub origin: Vector,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub rendermode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub shadowcastdist: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponKnife<'a> {
    pub angles: Angles,
    pub onplayerpickup: &'a str,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponM249<'a> {
    pub ammo: u8,
    pub angles: Angles,
    #[serde(default)]
    pub fademaxdist: Option<f32>,
    #[serde(default)]
    pub fademindist: Option<f32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    pub origin: Vector,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponM3<'a> {
    pub ammo: u16,
    pub angles: Angles,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponM4a1<'a> {
    #[serde(default)]
    pub ammo: Option<u16>,
    pub angles: Angles,
    #[serde(default)]
    pub fademaxdist: Option<f32>,
    #[serde(default)]
    pub fademindist: Option<f32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub mindxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub nodamageforces: bool,
    pub origin: Vector,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub rendermode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub shadowcastdist: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponMac10 {
    pub ammo: u8,
    pub angles: Angles,
    pub fademaxdist: f32,
    pub fademindist: f32,
    pub fadescale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub mindxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub rendermode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub shadowcastdist: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponP90<'a> {
    pub ammo: u16,
    pub angles: Angles,
    #[serde(default)]
    pub fademaxdist: Option<f32>,
    #[serde(default)]
    pub fademindist: Option<f32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub mindxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub nodamageforces: bool,
    pub origin: Vector,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub rendermode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub shadowcastdist: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponScout<'a> {
    #[serde(default)]
    pub ammo: Option<u8>,
    pub angles: Angles,
    #[serde(default)]
    pub fademaxdist: Option<f32>,
    #[serde(default)]
    pub fademindist: Option<f32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    #[serde(default)]
    pub onplayerpickup: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponSg550 {
    pub ammo: u8,
    pub angles: Angles,
    pub fademaxdist: f32,
    pub fademindist: f32,
    pub fadescale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub mindxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub rendermode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub shadowcastdist: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponSmokegrenade<'a> {
    pub angles: Angles,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponTmp {
    pub ammo: u8,
    pub angles: Angles,
    pub fademaxdist: f32,
    pub fademindist: f32,
    pub fadescale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub mindxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub rendermode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub shadowcastdist: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponUmp45 {
    pub ammo: u8,
    pub angles: Angles,
    pub fademaxdist: f32,
    pub fademindist: f32,
    pub fadescale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub mindxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub rendermode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub shadowcastdist: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponUsp<'a> {
    pub ammo: u8,
    pub angles: Angles,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponXm1014<'a> {
    pub ammo: u16,
    pub angles: Angles,
    #[serde(default)]
    pub fademaxdist: Option<f32>,
    #[serde(default)]
    pub fademindist: Option<f32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub mindxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub nodamageforces: bool,
    pub origin: Vector,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub rendermode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub shadowcastdist: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Worldspawn<'a> {
    pub detailmaterial: &'a str,
    pub detailvbsp: &'a str,
    pub maxpropscreenwidth: i32,
    pub skyname: &'a str,
    pub world_maxs: Vector,
    pub world_mins: Vector,
}
