
use std::str::FromStr;

use serde::{Deserialize, Deserializer};

use crate::{Angles, Color, LightColor, Vector};

use super::FromStrProp;

fn bool_from_int<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    let int = u8::deserialize(deserializer)?;
    Ok(int > 0)
}

#[derive(Debug, Clone, Deserialize)]
pub enum Negated{
	Yes,
	No,
	MatchingCriteria,
}
pub struct NegatedParseErr;
impl FromStr for Negated{
	type Err=NegatedParseErr;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s{
			"1"=>Ok(Negated::Yes),
			"0"=>Ok(Negated::No),
			"allow entities that match criteria"=>Ok(Negated::MatchingCriteria),
			_=>Err(NegatedParseErr),
		}
	}
}
impl FromStrProp for Negated{}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
#[serde(tag = "classname")]
pub enum Entity<'a> {
	#[serde(rename="env_beam")]
	EnvBeam(EnvBeam<'a>),
	#[serde(rename="env_detail_controller")]
	EnvDetailController(EnvDetailController<'a>),
	#[serde(rename="env_embers")]
	EnvEmbers(EnvEmbers<'a>),
	#[serde(rename="env_entity_maker")]
	EnvEntityMaker(EnvEntityMaker<'a>),
	#[serde(rename="env_fade")]
	EnvFade(EnvFade<'a>),
	#[serde(rename="env_fire")]
	EnvFire(EnvFire),
	#[serde(rename="env_fire_trail")]
	EnvFireTrail(EnvFireTrail<'a>),
	#[serde(rename="env_fog_controller")]
	EnvFogController(EnvFogController),
	#[serde(rename="env_hudhint")]
	EnvHudhint(EnvHudhint<'a>),
	#[serde(rename="env_laser")]
	EnvLaser(EnvLaser<'a>),
	#[serde(rename="env_lightglow")]
	EnvLightglow(EnvLightglow<'a>),
	#[serde(rename="env_shake")]
	EnvShake(EnvShake<'a>),
	#[serde(rename="env_shooter")]
	EnvShooter(EnvShooter<'a>),
	#[serde(rename="env_smokestack")]
	EnvSmokestack(EnvSmokestack<'a>),
	#[serde(rename="env_soundscape")]
	EnvSoundscape(EnvSoundscape<'a>),
	#[serde(rename="env_soundscape_triggerable")]
	EnvSoundscapeTriggerable(EnvSoundscapeTriggerable<'a>),
	#[serde(rename="env_spark")]
	EnvSpark(EnvSpark<'a>),
	#[serde(rename="env_sprite")]
	EnvSprite(EnvSprite<'a>),
	#[serde(rename="env_spritetrail")]
	EnvSpritetrail(EnvSpritetrail<'a>),
	#[serde(rename="env_steam")]
	EnvSteam(EnvSteam<'a>),
	#[serde(rename="env_sun")]
	EnvSun(EnvSun<'a>),
	#[serde(rename="env_tonemap_controller")]
	EnvTonemapController(EnvTonemapController),
	#[serde(rename="env_wind")]
	EnvWind(EnvWind<'a>),
	#[serde(rename="filter_activator_class")]
	FilterActivatorClass(FilterActivatorClass<'a>),
	#[serde(rename="filter_activator_name")]
	FilterActivatorName(FilterActivatorName<'a>),
	#[serde(rename="filter_damage_type")]
	FilterDamageType(FilterDamageType<'a>),
	#[serde(rename="filter_multi")]
	FilterMulti(FilterMulti<'a>),
	#[serde(rename="func_areaportalwindow")]
	FuncAreaportalwindow(FuncAreaportalwindow<'a>),
	#[serde(rename="func_breakable")]
	FuncBreakable(FuncBreakable<'a>),
	#[serde(rename="func_button")]
	FuncButton(FuncButton<'a>),
	#[serde(rename="func_buyzone")]
	FuncBuyzone(FuncBuyzone<'a>),
	#[serde(rename="func_conveyor")]
	FuncConveyor(FuncConveyor<'a>),
	#[serde(rename="func_door_rotating")]
	FuncDoorRotating(FuncDoorRotating<'a>),
	#[serde(rename="func_dustcloud")]
	FuncDustcloud(FuncDustcloud<'a>),
	#[serde(rename="func_movelinear")]
	FuncMovelinear(FuncMovelinear<'a>),
	#[serde(rename="func_physbox")]
	FuncPhysbox(FuncPhysbox<'a>),
	#[serde(rename="func_physbox_multiplayer")]
	FuncPhysboxMultiplayer(FuncPhysboxMultiplayer<'a>),
	#[serde(rename="func_precipitation")]
	FuncPrecipitation(FuncPrecipitation<'a>),
	#[serde(rename="func_rot_button")]
	FuncRotButton(FuncRotButton<'a>),
	#[serde(rename="func_rotating")]
	FuncRotating(FuncRotating<'a>),
	#[serde(rename="func_smokevolume")]
	FuncSmokevolume(FuncSmokevolume<'a>),
	#[serde(rename="func_tracktrain")]
	FuncTracktrain(FuncTracktrain<'a>),
	#[serde(rename="func_wall")]
	FuncWall(FuncWall<'a>),
	#[serde(rename="func_wall_toggle")]
	FuncWallToggle(FuncWallToggle<'a>),
	#[serde(rename="func_water_analog")]
	FuncWaterAnalog(FuncWaterAnalog<'a>),
	#[serde(rename="game_player_equip")]
	GamePlayerEquip(GamePlayerEquip),
	#[serde(rename="game_ui")]
	GameUi(GameUi<'a>),
	#[serde(rename="game_weapon_manager")]
	GameWeaponManager(GameWeaponManager<'a>),
	#[serde(rename="info_ladder")]
	InfoLadder(InfoLadder<'a>),
	#[serde(rename="info_player_start")]
	InfoPlayerStart(InfoPlayerStart<'a>),
	#[serde(rename = "info_player_counterterrorist")]
    CounterTerroristSpawn(CounterTerroristSpawn),
	#[serde(rename="info_player_terrorist")]
	InfoPlayerTerrorist(InfoPlayerTerrorist),
	#[serde(rename="info_target")]
	InfoTarget(InfoTarget<'a>),
	#[serde(rename="info_teleport_destination")]
	InfoTeleportDestination(InfoTeleportDestination<'a>),
	#[serde(rename="infodecal")]
	Infodecal(Infodecal<'a>),
	#[serde(rename="keyframe_rope")]
	KeyframeRope(KeyframeRope<'a>),
	#[serde(rename="light")]
	Light(Light<'a>),
	#[serde(rename="light_environment")]
	LightEnvironment(LightEnvironment<'a>),
	#[serde(rename="logic_auto")]
	LogicAuto(LogicAuto<'a>),
	#[serde(rename="logic_relay")]
	LogicRelay(LogicRelay<'a>),
	#[serde(rename="logic_timer")]
	LogicTimer(LogicTimer<'a>),
	#[serde(rename="math_counter")]
	MathCounter(MathCounter<'a>),
	#[serde(rename="move_rope")]
	MoveRope(MoveRope<'a>),
	#[serde(rename="path_track")]
	PathTrack(PathTrack<'a>),
	#[serde(rename="phys_ballsocket")]
	PhysBallsocket(PhysBallsocket<'a>),
	#[serde(rename="player_speedmod")]
	PlayerSpeedmod(PlayerSpeedmod<'a>),
	#[serde(rename="player_weaponstrip")]
	PlayerWeaponstrip(PlayerWeaponstrip<'a>),
	#[serde(rename="point_clientcommand")]
	PointClientcommand(PointClientcommand<'a>),
	#[serde(rename="point_servercommand")]
	PointServercommand(PointServercommand<'a>),
	#[serde(rename="point_spotlight")]
	PointSpotlight(PointSpotlight<'a>),
	#[serde(rename="point_template")]
	PointTemplate(PointTemplate<'a>),
	#[serde(rename="point_viewcontrol")]
	PointViewcontrol(PointViewcontrol<'a>),
	#[serde(rename="prop_dynamic")]
	PropDynamic(PropDynamic<'a>),
	#[serde(rename="prop_physics")]
	PropPhysics(PropPhysics<'a>),
	#[serde(rename="prop_physics_multiplayer")]
	PropPhysicsMultiplayer(PropPhysicsMultiplayer<'a>),
	#[serde(rename="prop_physics_override")]
	PropPhysicsOverride(PropPhysicsOverride<'a>),
	#[serde(rename="prop_ragdoll")]
	PropRagdoll(PropRagdoll<'a>),
	#[serde(rename="shadow_control")]
	ShadowControl(ShadowControl),
	#[serde(rename="sky_camera")]
	SkyCamera(SkyCamera<'a>),
	#[serde(rename="trigger_gravity")]
	TriggerGravity(TriggerGravity<'a>),
	#[serde(rename="trigger_hurt")]
	TriggerHurt(TriggerHurt<'a>),
	#[serde(rename="trigger_look")]
	TriggerLook(TriggerLook<'a>),
	#[serde(rename="trigger_multiple")]
	TriggerMultiple(TriggerMultiple<'a>),
	#[serde(rename="trigger_once")]
	TriggerOnce(TriggerOnce<'a>),
	#[serde(rename="trigger_push")]
	TriggerPush(TriggerPush<'a>),
	#[serde(rename="trigger_soundscape")]
	TriggerSoundscape(TriggerSoundscape<'a>),
	#[serde(rename="trigger_teleport")]
	TriggerTeleport(TriggerTeleport<'a>),
	#[serde(rename="water_lod_control")]
	WaterLodControl(WaterLodControl),
	#[serde(rename="weapon_ak47")]
	WeaponAk47(WeaponAk47<'a>),
	#[serde(rename="weapon_awp")]
	WeaponAwp(WeaponAwp<'a>),
	#[serde(rename="weapon_deagle")]
	WeaponDeagle(WeaponDeagle),
	#[serde(rename="weapon_elite")]
	WeaponElite(WeaponElite<'a>),
	#[serde(rename="weapon_famas")]
	WeaponFamas(WeaponFamas<'a>),
	#[serde(rename="weapon_fiveseven")]
	WeaponFiveseven(WeaponFiveseven<'a>),
	#[serde(rename="weapon_flashbang")]
	WeaponFlashbang(WeaponFlashbang<'a>),
	#[serde(rename="weapon_g3sg1")]
	WeaponG3Sg1(WeaponG3Sg1<'a>),
	#[serde(rename="weapon_glock")]
	WeaponGlock(WeaponGlock<'a>),
	#[serde(rename="weapon_hegrenade")]
	WeaponHegrenade(WeaponHegrenade<'a>),
	#[serde(rename="weapon_knife")]
	WeaponKnife(WeaponKnife<'a>),
	#[serde(rename="weapon_m249")]
	WeaponM249(WeaponM249<'a>),
	#[serde(rename="weapon_m3")]
	WeaponM3(WeaponM3<'a>),
	#[serde(rename="weapon_m4a1")]
	WeaponM4A1(WeaponM4A1),
	#[serde(rename="weapon_mac10")]
	WeaponMac10(WeaponMac10<'a>),
	#[serde(rename="weapon_p90")]
	WeaponP90(WeaponP90<'a>),
	#[serde(rename="weapon_scout")]
	WeaponScout(WeaponScout<'a>),
	#[serde(rename="weapon_sg550")]
	WeaponSg550(WeaponSg550<'a>),
	#[serde(rename="weapon_smokegrenade")]
	WeaponSmokegrenade(WeaponSmokegrenade<'a>),
	#[serde(rename="weapon_tmp")]
	WeaponTmp(WeaponTmp<'a>),
	#[serde(rename="weapon_ump45")]
	WeaponUmp45(WeaponUmp45<'a>),
	#[serde(rename="weapon_usp")]
	WeaponUsp(WeaponUsp<'a>),
	#[serde(rename="weapon_xm1014")]
	WeaponXm1014(WeaponXm1014<'a>),
	#[serde(rename="worldspawn")]
	Worldspawn(Worldspawn<'a>),
}


#[derive(Debug, Clone, Deserialize)]
pub struct CounterTerroristSpawn{
	pub origin: Vector,
    pub angles: Angles,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Worldspawn<'a>{
    pub maxpropscreenwidth: i32,//"-1",
    pub skyname: &'a str,
    pub detailmaterial: &'a str,
    pub world_mins: Vector,
    pub world_maxs: Vector,
    pub detailvbsp: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InfoPlayerTerrorist{
    pub origin:Vector,
    pub angles:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerTeleport<'a>{
    pub target: &'a str,

#[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub origin:Vector,
    pub spawnflags: u32,
    pub model: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InfoTeleportDestination<'a>{
	pub origin:Vector,
    pub targetname: &'a str,
    pub angles:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncWaterAnalog<'a>{
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
    pub origin:Vector,
    pub rendermode: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LightEnvironment<'a>{
	pub origin:Vector,
    pub _light: LightColor,
    pub angles:Vector,
    pub _ambient: LightColor,
    pub _lighthdr: LightColor,
    pub pitch: i32,
    pub _ambienthdr: &'a str,//"-1 -1 -1 1"
}

#[derive(Debug, Clone, Deserialize)]
pub struct GamePlayerEquip{
	pub origin:Vector,
	#[serde(deserialize_with = "bool_from_int")]
    pub weapon_knife: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerHurt<'a>{
    pub damagecap: i32,
    pub model: &'a str,
    pub spawnflags: u32,
    pub damagetype: u32,
    pub origin:Vector,
    pub damage: i32,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub damagemodel: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncButton<'a>{
    pub movedir: Vector,
    pub onpressed: &'a str,
    pub spawnflags: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub unlocked_sentence: bool,
    pub speed: f32,
    pub wait: f32,
    pub sounds: u32,
    pub origin:Vector,
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
pub struct WeaponM4A1{
    pub spawnflags: u32,
    pub angles:Vector,
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WaterLodControl{
    pub cheapwaterenddistance: f32,
    pub cheapwaterstartdistance: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncWall<'a>{
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
pub struct EnvTonemapController{
	pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSun<'a>{
    pub rendercolor: Color,
    pub target: &'a str,
    pub hdrcolorscale: f32,
    pub material: &'a str,
    pub size: f32,
    pub origin:Vector,
    pub angles:Vector,
    pub overlaymaterial: &'a str,
    pub overlaycolor: Color,
    pub overlaysize: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InfoTarget<'a>{
    pub spawnflags: u32,
    pub origin:Vector,
    pub targetname: &'a str,
    pub angles:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponAwp<'a>{
	pub origin:Vector,
    pub angles:Vector,
    pub targetname: &'a str,
    pub spawnflags: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSpritetrail<'a>{
    pub endwidth: f32,
    pub lifetime: f32,
    pub parentname: &'a str,
    pub origin:Vector,
    pub rendercolor: Color,
    pub renderamt: u8,
    pub startwidth: f32,
    pub spritename: &'a str,
    pub rendermode: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponDeagle{
    pub origin:Vector,
    pub angles:Vector,
    pub ammo: u32
}

#[derive(Debug, Clone, Deserialize)]
pub struct ShadowControl{
	pub origin:Vector,
	pub angles:Vector,
    pub distance: f32,
    pub color: Color,
    //pub hammerid: "2971"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponElite<'a>{
    //pub hammerid: "7248",
    pub targetname: &'a str,
    pub ammo: u32,
    pub spawnflags: u32,
    pub angles:Vector,
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncRotating<'a>{
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
    pub origin:Vector,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    //pub hammerid: "7383",
    pub maxspeed: f32,
    pub angles:Vector,
    pub rendermode: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncBreakable<'a>{
    pub gibdir: Vector,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub performancemode: bool,
    pub explodemagnitude: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub pressuredelay: bool,
    pub origin:Vector,
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
pub struct WeaponP90<'a>{
    pub ammo: u32,
    pub origin:Vector,
    pub spawnflags: u32,
    pub targetname: &'a str,
    //pub hammerid: "15611",
    pub angles:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponGlock<'a>{
    pub targetname: &'a str,
    //pub hammerid: "19772",
    pub origin:Vector,
    pub angles:Vector,
    pub spawnflags: u32,
    pub ammo: u32
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogicAuto<'a>{
	//pub hammerid: "20253",
    pub origin:Vector,
    pub spawnflags: u32,
    pub onmapspawn: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncWallToggle<'a>{
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
pub struct WeaponM249<'a>{
	pub angles:Vector,
    pub targetname: &'a str,
    pub origin:Vector,
    //pub hammerid: "24744",
    pub spawnflags: u32,
    pub ammo: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponHegrenade<'a>{
	pub origin:Vector,
	//pub hammerid: "28302",
    pub targetname: &'a str,
    pub angles:Vector,
    pub spawnflags: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PointViewcontrol<'a>{
    pub wait: f32,
    pub angles:Vector,
    pub spawnflags: u32,
    pub acceleration: f32,
    pub target: &'a str,
    pub deceleration: f32,
    pub origin:Vector,
    //pub hammerid: "36815"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponScout<'a>{
    pub spawnflags: u32,
    pub targetname: &'a str,
    pub origin:Vector,
    pub angles:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PointTemplate<'a>{
    pub template01: &'a str,
    pub spawnflags: u32,
    pub targetname: &'a str,
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncDoorRotating<'a>{
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
    pub origin:Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub lip: bool,
    pub speed: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub loopmovesound: bool,
    pub angles:Vector,
    pub wait: f32,
    pub distance: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerPush<'a>{
	#[serde(deserialize_with = "bool_from_int")]
    pub alternateticksfix: bool,
    pub speed: f32,
    pub model: &'a str,
    pub origin:Vector,
    pub spawnflags: u32,
    pub pushdir: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncPhysbox<'a>{
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
    pub origin:Vector,
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
pub struct WeaponSmokegrenade<'a>{
    pub origin:Vector,
    pub angles:Vector,
    pub spawnflags: u32,
    pub targetname: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FilterDamageType<'a>{
	pub origin:Vector,
    pub negated: Negated,
    pub targetname: &'a str,
    pub damagetype: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvFogController{
    pub fogend: f32,
    pub fogcolor: Color,
    pub origin:Vector,
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
    pub angles:Vector,
    pub fogmaxdensity: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncConveyor<'a>{
    pub spawnflags: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    pub rendercolor: Color,
    pub rendermode: u32,
    pub angles:Vector,
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
pub struct FilterActivatorName<'a>{
	pub origin:Vector,
    pub negated: Negated,
    pub targetname: &'a str,
    pub filtername: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerMultiple<'a>{
    pub model: &'a str,

#[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub wait: f32,
    pub spawnflags: u32,
    pub ontrigger: &'a str,
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FilterMulti<'a>{
    pub targetname: &'a str,
    pub filter02: &'a str,
    pub origin:Vector,
    pub negated: Negated,
    pub filter01: &'a str,
    //pub hammerid: "49342",
    #[serde(deserialize_with = "bool_from_int")]
    pub filtertype: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PropPhysicsOverride<'a>{
	pub origin:Vector,
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
    pub angles:Vector,
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
pub struct PointServercommand<'a>{
    pub origin:Vector,
    pub targetname: &'a str,
    //pub hammerid: "90371"
}

#[derive(Debug, Clone, Deserialize)]
pub struct PointClientcommand<'a>{
    pub targetname: &'a str,
    //pub hammerid: "90377",
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GameWeaponManager<'a>{
    pub weaponname: &'a str,
    pub maxpieces: u8,
    pub origin:Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub ammomod: bool,
    //pub hammerid: "90603"
}

#[derive(Debug, Clone, Deserialize)]
pub struct FilterActivatorClass<'a>{
	pub origin:Vector,
    pub targetname: &'a str,
    pub negated: Negated,
    pub filterclass: &'a str,
    //pub hammerid: "91384",
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerSpeedmod<'a>{
	//pub hammerid: "91426",
    pub origin:Vector,
    pub spawnflags: u32,
    pub targetname: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvFire{
    pub spawnflags: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub ignitionpoint: f32,
    pub health: u8,
    pub origin:Vector,
    pub damagescale: f32,
    //pub hammerid: "101536",
    pub firesize: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub firetype: bool,
    pub fireattack: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerWeaponstrip<'a>{
    pub targetname: &'a str,
    //pub hammerid: "107702",
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncBuyzone<'a>{
    pub teamnum: u8,
    pub model: &'a str,
    //pub hammerid: "113521"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSoundscape<'a>{

#[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    //pub hammerid: "133638",
    pub origin:Vector,
    pub radius: f32,
    pub soundscape: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSprite<'a>{
    pub glowproxysize: f32,
    pub rendermode: u32,
    pub mindxlevel: u8,
    pub framerate: f32,
    //pub hammerid: "134683",
    pub maxdxlevel: u8,
    pub hdrcolorscale: f32,
    pub origin:Vector,
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
pub struct MathCounter<'a>{
	#[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub targetname: &'a str,
    pub max: i32,
    pub origin:Vector,
    pub onhitmax: &'a str,
    //pub hammerid: "134898",
    pub startvalue: i32,
    pub min: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponKnife<'a>{
    pub angles:Vector,
    //pub hammerid: "176620",
    pub spawnflags: u32,
    pub targetname: &'a str,
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvEntityMaker<'a>{
    pub entitytemplate: &'a str,
    pub angles:Vector,
    //pub hammerid: "176738",
    pub spawnflags: u32,
    pub postspawndirection: Vector,
    pub origin:Vector,
    pub targetname: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub postspawninheritangles: bool,
    pub postspawndirectionvariance: f32,
    pub postspawnspeed: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSoundscapeTriggerable<'a>{
    pub soundscape: &'a str,
    pub radius: f32,
    //pub hammerid: "179655",
	#[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub targetname: &'a str,
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvHudhint<'a>{
	pub origin:Vector,
    pub spawnflags: u32,
    //pub hammerid: "183578",
    pub targetname: &'a str,
    pub message: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PropDynamic<'a>{
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
    pub angles:Vector,
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
    pub origin:Vector,
    pub minanimtime: f32,
    pub maxdxlevel: u8,

#[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Infodecal<'a>{
    pub texture: &'a str,
    //pub hammerid: "3739",
    pub angles:Vector,
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerSoundscape<'a>{

#[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub soundscape: &'a str,
    pub model: &'a str,
    pub origin:Vector,
    pub spawnflags: u32,
    //pub hammerid: "7740"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSpark<'a>{
    pub spawnflags: u32,
    pub targetname: &'a str,
    pub maxdelay: f32,
    pub magnitude: f32,
    pub angles:Vector,
    //pub hammerid: "34153",
    pub origin:Vector,
    pub traillength: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogicTimer<'a>{
    pub spawnflags: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub userandomtime: bool,
    pub lowerrandombound: i32,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    //pub hammerid: "34188",
    pub ontimer: &'a str,
    pub upperrandombound: i32,
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PropPhysicsMultiplayer<'a>{
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
    pub angles:Vector,
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
    pub origin:Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub pressuredelay: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    pub fademindist: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub damagetoenablemotion: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvWind<'a>{
	pub origin:Vector,
    pub maxgustdelay: "20",
    pub mingustdelay: "10",
    pub maxgust: "250",
    pub minwind: "20",
    pub gustduration: "5",
    pub mingust: "100",
    pub angles:Vector,
    //pub hammerid: "38005",
    pub maxwind: "50",
    pub gustdirchange: "20",
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvDetailController<'a>{
    //pub hammerid: "207110",
    pub origin:Vector,
    pub fademindist: f32,
    pub fademaxdist: f32,
    pub angles:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InfoPlayerStart<'a>{
	pub angles:Vector,
    pub origin:Vector,
    //pub hammerid: "259724",
    pub spawnflags: u32"
}

#[derive(Debug, Clone, Deserialize)]
pub struct PropPhysics<'a>{
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
    pub origin:Vector,
    //pub hammerid: "487103",
    pub damagetype: u32,
    pub fadescale: f32,
    pub rendermode: u32,
    pub renderamt: u8,
    pub skin: u16,
    pub model: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    pub angles:Vector,
    pub fademaxdist: f32,
    pub rendercolor: Color,
    pub maxdxlevel: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponAk47<'a>{
    pub angles:Vector,
    pub targetname: &'a str,
    pub ammo: u32,
    pub origin:Vector,
    pub spawnflags: u32,
    //pub hammerid: "155595"
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncMovelinear<'a>{
	#[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub origin:Vector,
    pub rendermode: u32,
    pub parentname: &'a str,
    pub rendercolor: Color,
    pub blockdamage: "0",
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
pub struct FuncPrecipitation<'a>{
	//pub hammerid: "288365",
    pub renderamt: u8,
    pub preciptype: "3",
    pub rendercolor: Color,
    pub model: &'a str"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvLightglow<'a>{
    pub hdrcolorscale: f32,
    pub origin:Vector,
    pub angles:Vector,
    pub horizontalglowsize: "8",
    pub verticalglowsize: "8",
    pub maxdist: "256",
    //pub hammerid: "156600",
    pub outermaxdist: "0",
    pub spawnflags: u32,
    pub glowproxysize: f32,
    pub mindist: "64",
    pub rendercolor: Color,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSmokestack<'a>{
    pub rendercolor: Color,
    pub roll: "15.0",
    pub windangle: "0",
    pub twist: "2",
    pub startsize: "20",
    pub renderamt: u8,
    pub basespread: "125",
    pub angles:Vector,
    pub initialstate: "1",
    pub jetlength: "200",
    pub endsize: "30",
    pub windspeed: "4",
    pub smokematerial: "particle/smokestack.vmt",
    pub origin:Vector,
    pub rate: "20",
    pub spreadspeed: "15",
    //pub hammerid: "129270",
    pub speed: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncSmokevolume<'a>{
    pub color2: "192 192 192",
    pub particledrawwidth: "192",
    //pub hammerid: "31779",
    pub particlespacingdistance: "80",
    pub color1: "192 192 192",
    pub spawnflags: u32,
    pub rotationspeed: "10",
    pub movementspeed: "10",
    pub material: "particle/particle_noisesphere",
    pub densityrampspeed: "1",
    pub density: "0.5",
    pub model: &'a str"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvEmbers<'a>{
	pub angles:Vector,
    pub spawnflags: u32,
    //pub hammerid: "363584",
    pub model: &'a str,
    pub speed: f32,
    pub lifetime: f32,
    pub rendercolor: Color,
    pub density: "32"
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncDustcloud<'a>{
    pub frozen: "0",
    pub sizemax: "150",

#[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    //pub hammerid: "370009",
    pub lifetimemin: "3",
    pub lifetimemax: "5",
    pub color: Color,
    pub distmax: "1024",
    pub alpha: "96",
    pub model: &'a str,
    pub speedmax: "6",
    pub sizemin: "50",
    pub spawnrate: "500"
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerOnce<'a>{
	pub origin:Vector,
    pub model: &'a str,
    //pub hammerid: "387965",
    pub angles:Vector,

#[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub ontrigger: "servercommand,command,say level 5 unlocked,0,-1",
    pub spawnflags: u32"
}

#[derive(Debug, Clone, Deserialize)]
pub struct PointSpotlight<'a>{
    pub hdrcolorscale: f32,
    pub angles:Vector,
    pub spotlightlength: "200.0",
    pub spotlightwidth: "20.0",
    pub renderamt: u8,
    pub targetname: &'a str,
    pub spawnflags: u32,
    pub origin:Vector,
    pub rendercolor: Color,
    //pub hammerid: "393222"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponTmp<'a>{
    pub maxdxlevel: u8,
    pub angles:Vector,
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
    pub origin:Vector,
    pub spawnflags: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    pub ammo: u32
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponXm1014<'a>{
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
    pub origin:Vector,
    pub fademaxdist: f32,
    pub rendermode: u32,
    pub maxdxlevel: u8,
    pub fadescale: f32,
    pub shadowcastdist: f32,
    pub fademindist: f32,
    pub angles:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponMac10<'a>{
    pub spawnflags: u32,
    pub rendermode: u32,
    pub fademindist: f32,
    pub ammo: u32,
    pub angles:Vector,
    pub shadowcastdist: f32,
    pub rendercolor: Color,
    pub fademaxdist: f32,
    //pub hammerid: "403143",
    pub origin:Vector,
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
pub struct WeaponUmp45<'a>{
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
    pub origin:Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    pub fademindist: f32,
    pub angles:Vector,
    pub maxdxlevel: u8,
    pub fadescale: f32,
    pub rendercolor: Color,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponFamas<'a>{
	//pub hammerid: "403360",
    pub shadowcastdist: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub rendermode: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    pub origin:Vector,
    pub fademaxdist: f32,
    pub maxdxlevel: u8,
    pub renderamt: u8,
    pub mindxlevel: u8,
    pub angles:Vector,
    pub fadescale: f32,
    pub spawnflags: u32,
    pub rendercolor: Color,
    pub fademindist: f32,
    pub ammo: u32
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponG3Sg1<'a>{
	pub angles:Vector,
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
    pub origin:Vector,
    pub mindxlevel: u8,
    pub rendercolor: Color,
    pub shadowcastdist: f32,
    pub maxdxlevel: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponSg550<'a>{
    pub renderamt: u8,
    //pub hammerid: "403621",
    pub origin:Vector,
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
    pub angles:Vector,
    pub fademindist: f32,
    pub shadowcastdist: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponFlashbang<'a>{
	pub origin:Vector,
    pub fademindist: f32,
    pub spawnflags: u32,
    //pub hammerid: "416160",
    pub renderamt: u8,
    pub fadescale: f32,
    pub fademaxdist: f32,
    pub angles:Vector,
    pub targetname: &'a str,
    pub rendercolor: Color,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvFireTrail<'a>{
    pub targetname: &'a str,
    //pub hammerid: "425179",
    pub origin:Vector,
    pub parentname: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InfoLadder<'a>{
    "maxspub y: "3528.00",
    "minspub z: "112.00",
    //pub hammerid: "2854",
    "maxspub x: "-9792.00",
    "minspub y: "3520.00",
    "maxspub z: "640.00",
    "minspub x: "-9856.00"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponM3<'a>{
	pub ammo: u32,
//pub hammerid: "176561",
    pub angles:Vector,
    pub spawnflags: u32,
    pub targetname: &'a str,
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponFiveseven<'a>{
	pub origin:Vector,
    pub ammo: u32,
    //pub hammerid: "177007",
    pub targetname: &'a str,
    pub spawnflags: u32,
    pub angles:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MoveRope<'a>{
	pub spawnflags: u32,
	pub barbed: "0",
	pub slack: "105",
	pub nowind: "1",
    pub origin:Vector,
    pub subdiv: "4",
    pub movespeed: "64",
    pub width: "1",
    pub positioninterpolator: "2",
    pub mindxlevel: u8,
    pub ropematerial: &'a str,
    pub texturescale: "1",
    pub collide: "0",
    //pub hammerid: "129625",
    pub angles:Vector,
    pub breakable: "0",
    pub type: "0",
    pub maxdxlevel: u8,
    pub dangling: "0",
    pub targetname: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvFade<'a>{
	pub spawnflags: u32,
	pub rendercolor: Color,
    pub origin:Vector,
    //pub hammerid: "262982",
    pub duration: "3",
    pub holdtime: "0.0",
    pub renderamt: u8,
    pub targetname: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncAreaportalwindow<'a>{
	pub translucencylimit: "0.0",
	pub fadestartdist: "1950",
	pub fadedist: "2000",
	pub portalnumber: "1",
	pub target: &'a str,
	pub portalversion: "1",
	//pub hammerid: "270454"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvLaser<'a>{
    pub texturescroll: f32,
    pub dissolvetype: "none",
    pub lasertarget: &'a str,
    pub width: "4",
    pub rendercolor: Color,
    pub renderamt: u8,
    pub texture: &'a str,
    pub damage: i32,
    //pub hammerid: "731916",
    pub origin:Vector,
    pub spawnflags: u32"
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogicRelay<'a>{
	pub origin:Vector,
    //pub hammerid: "805602",
    pub ontrigger: "snek_btn_1,kill,,0,-1",
    pub targetname: &'a str,
    pub spawnflags: u32"
}

#[derive(Debug, Clone, Deserialize)]
pub struct PathTrack<'a>{
	#[serde(deserialize_with = "bool_from_int")]
    pub orientationtype: bool,
    pub angles:Vector,
    pub spawnflags: u32,
    pub target: &'a str,
    //pub hammerid: "30225",
    pub targetname: &'a str,
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncTracktrain<'a>{
	#[serde(deserialize_with = "bool_from_int")]
    pub velocitytype: bool,
    pub rendermode: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub orientationtype: bool,
    pub movesoundmaxpitch: u8,
    pub movesoundmintime: "0",
    pub height: "4",
    pub target: &'a str,
    pub speed: f32,
    pub dmg: i32,
    pub renderamt: u8,
    pub targetname: &'a str,
    pub movesound: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    pub movesoundminpitch: "60",
    //pub hammerid: "30270",
    pub rendercolor: Color,
    pub volume: u8,
    pub wheels: "50",
    pub bank: "0",
    pub model: &'a str,
    pub spawnflags: u32,
    pub movesoundmaxtime: "0",
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    pub origin:Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub startspeed: "100"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSteam<'a>{
    pub rate: "26",
    pub spawnflags: u32,
    pub speed: f32,
    pub startsize: "10",
    pub renderamt: u8,
    pub endsize: "25",
    pub targetname: &'a str,
    pub rollspeed: "8",
    //pub hammerid: "65082",
    pub angles:Vector,
    pub origin:Vector,
    pub rendercolor: Color,
    pub spreadspeed: "15",
    pub jetlength: "80"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvShake<'a>{
    pub amplitude: "6",
    pub targetname: &'a str,
    pub origin:Vector,
    pub duration: "4",
    pub spawnflags: u32,
    pub frequency: "2.5",
    pub radius: f32,
    //pub hammerid: "105159"
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncRotButton<'a>{
    pub distance: f32,
    pub sounds: u32,
    pub angles:Vector,
    pub origin:Vector,
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
pub struct TriggerGravity<'a>{
	//pub hammerid: "259593",
    pub origin:Vector,
    pub spawnflags: u32,
    pub model: &'a str,

#[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub gravity: ".05",
}

#[derive(Debug, Clone, Deserialize)]
pub struct SkyCamera<'a>{
    pub fogdir: Vector,
    pub scale: "16",
    pub fogstart: f32,
    pub fogend: f32,
    pub fogcolor2: Color,
    //pub hammerid: "361080",
    pub angles:Vector,
    pub fogcolor: Color,
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GameUi<'a>{
    pub fieldofview: f32,
    pub origin:Vector,
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
pub struct FuncPhysboxMultiplayer<'a>{
    pub targetname: &'a str,
    pub health: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub pressuredelay: bool,
    pub damagetype: u32,
    pub material: u32,
    pub preferredcarryangles: Angles,
    pub origin:Vector,
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
pub struct TriggerLook<'a>{
	#[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub looktime: "5",
    //pub hammerid: "491215",
    pub target: &'a str,
    pub spawnflags: u32,
    pub model: &'a str,
    pub timeout: "0",
    pub fieldofview: f32,
    pub ontrigger: "door_pic,open,,0,-1",
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KeyframeRope<'a>{
	pub origin:Vector,
    pub slack: "25",
    pub texturescale: "1",
    pub ropematerial: &'a str,
    pub movespeed: "64",
    pub angles:Vector,
    //pub hammerid: "543370",
    pub width: "2",
    pub targetname: &'a str,
    pub subdiv: "2"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvShooter<'a>{
    pub m_flvariance: "0.3",
    pub angles:Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub rendermode: u32,
    pub m_flvelocity: "772",
    pub origin:Vector,
    pub parentname: &'a str,
    pub shootmodel: &'a str,
    pub nogibshadows: "0",
    pub spawnflags: u32,
    pub massoverride: "0",
    pub skin: u16,
    pub renderamt: u8,
    pub gibangles: "0 0 0",
    pub delay: "3",
    pub simulation: "0",
    pub targetname: &'a str,
    pub m_flgiblife: "1.5",
    pub m_igibs: "99999",
    pub gibgravityscale: "1",
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    pub shootsounds: "-1",
    //pub hammerid: "564233"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponUsp<'a>{
	//pub hammerid: "597821",
    pub origin:Vector,
    pub spawnflags: u32,
    pub targetname: &'a str,
    pub angles:Vector,
    pub ammo: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PropRagdoll<'a>{
    pub fadescale: f32,
    pub modelscale: f32,
    pub origin:Vector,
    pub model: &'a str,
    pub fademindist: f32,
    pub skin: u16,
    pub spawnflags: u32,
    pub angles:Vector,
    //pub hammerid: "739080",
    pub targetname: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PhysBallsocket<'a>{
    pub targetname: &'a str,
    pub origin:Vector,
    pub attach1: &'a str,
    //pub hammerid: "739103"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvBeam<'a>{
	pub origin:Vector,
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
    pub lightningstart: "start_tesla1",
    pub radius: f32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Light<'a>{
	pub origin:Vector,
    pub _lightscalehdr: "1",
    pub _quadratic_attn: "1",
    pub _lighthdr: LightColor,
    pub _light: Color,
    //pub hammerid: "823486"
}
