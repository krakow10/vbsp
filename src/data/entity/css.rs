
use serde::Deserialize;

use crate::{Angles, Color, LightColor, Vector};

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
#[serde(tag = "classname")]
pub enum Entity<'a> {
	#[serde(rename="env_beam")]
	EnvBeam(EnvBeam),
	#[serde(rename="env_detail_controller")]
	EnvDetailController(EnvDetailController),
	#[serde(rename="env_embers")]
	EnvEmbers(EnvEmbers),
	#[serde(rename="env_entity_maker")]
	EnvEntityMaker(EnvEntityMaker),
	#[serde(rename="env_fade")]
	EnvFade(EnvFade),
	#[serde(rename="env_fire")]
	EnvFire(EnvFire),
	#[serde(rename="env_fire_trail")]
	EnvFireTrail(EnvFireTrail),
	#[serde(rename="env_fog_controller")]
	EnvFogController(EnvFogController),
	#[serde(rename="env_hudhint")]
	EnvHudhint(EnvHudhint),
	#[serde(rename="env_laser")]
	EnvLaser(EnvLaser),
	#[serde(rename="env_lightglow")]
	EnvLightglow(EnvLightglow),
	#[serde(rename="env_shake")]
	EnvShake(EnvShake),
	#[serde(rename="env_shooter")]
	EnvShooter(EnvShooter),
	#[serde(rename="env_smokestack")]
	EnvSmokestack(EnvSmokestack),
	#[serde(rename="env_soundscape")]
	EnvSoundscape(EnvSoundscape),
	#[serde(rename="env_soundscape_triggerable")]
	EnvSoundscapeTriggerable(EnvSoundscapeTriggerable),
	#[serde(rename="env_spark")]
	EnvSpark(EnvSpark),
	#[serde(rename="env_sprite")]
	EnvSprite(EnvSprite),
	#[serde(rename="env_spritetrail")]
	EnvSpritetrail(EnvSpritetrail),
	#[serde(rename="env_steam")]
	EnvSteam(EnvSteam),
	#[serde(rename="env_sun")]
	EnvSun(EnvSun),
	#[serde(rename="env_tonemap_controller")]
	EnvTonemapController(EnvTonemapController),
	#[serde(rename="env_wind")]
	EnvWind(EnvWind),
	#[serde(rename="filter_activator_class")]
	FilterActivatorClass(FilterActivatorClass),
	#[serde(rename="filter_activator_name")]
	FilterActivatorName(FilterActivatorName),
	#[serde(rename="filter_damage_type")]
	FilterDamageType(FilterDamageType),
	#[serde(rename="filter_multi")]
	FilterMulti(FilterMulti),
	#[serde(rename="func_areaportalwindow")]
	FuncAreaportalwindow(FuncAreaportalwindow),
	#[serde(rename="func_breakable")]
	FuncBreakable(FuncBreakable),
	#[serde(rename="func_button")]
	FuncButton(FuncButton),
	#[serde(rename="func_buyzone")]
	FuncBuyzone(FuncBuyzone),
	#[serde(rename="func_conveyor")]
	FuncConveyor(FuncConveyor),
	#[serde(rename="func_door_rotating")]
	FuncDoorRotating(FuncDoorRotating),
	#[serde(rename="func_dustcloud")]
	FuncDustcloud(FuncDustcloud),
	#[serde(rename="func_movelinear")]
	FuncMovelinear(FuncMovelinear),
	#[serde(rename="func_physbox")]
	FuncPhysbox(FuncPhysbox),
	#[serde(rename="func_physbox_multiplayer")]
	FuncPhysboxMultiplayer(FuncPhysboxMultiplayer),
	#[serde(rename="func_precipitation")]
	FuncPrecipitation(FuncPrecipitation),
	#[serde(rename="func_rot_button")]
	FuncRotButton(FuncRotButton),
	#[serde(rename="func_rotating")]
	FuncRotating(FuncRotating),
	#[serde(rename="func_smokevolume")]
	FuncSmokevolume(FuncSmokevolume),
	#[serde(rename="func_tracktrain")]
	FuncTracktrain(FuncTracktrain),
	#[serde(rename="func_wall")]
	FuncWall(FuncWall),
	#[serde(rename="func_wall_toggle")]
	FuncWallToggle(FuncWallToggle),
	#[serde(rename="func_water_analog")]
	FuncWaterAnalog(FuncWaterAnalog),
	#[serde(rename="game_player_equip")]
	GamePlayerEquip(GamePlayerEquip),
	#[serde(rename="game_ui")]
	GameUi(GameUi),
	#[serde(rename="game_weapon_manager")]
	GameWeaponManager(GameWeaponManager),
	#[serde(rename="info_ladder")]
	InfoLadder(InfoLadder),
	#[serde(rename="info_player_start")]
	InfoPlayerStart(InfoPlayerStart),
	#[serde(rename = "info_player_counterterrorist")]
    CounterTerroristSpawn(CounterTerroristSpawn),
	#[serde(rename="info_player_terrorist")]
	InfoPlayerTerrorist(InfoPlayerTerrorist),
	#[serde(rename="info_target")]
	InfoTarget(InfoTarget),
	#[serde(rename="info_teleport_destination")]
	InfoTeleportDestination(InfoTeleportDestination),
	#[serde(rename="infodecal")]
	Infodecal(Infodecal),
	#[serde(rename="keyframe_rope")]
	KeyframeRope(KeyframeRope),
	#[serde(rename="light")]
	Light(Light),
	#[serde(rename="light_environment")]
	LightEnvironment(LightEnvironment),
	#[serde(rename="logic_auto")]
	LogicAuto(LogicAuto),
	#[serde(rename="logic_relay")]
	LogicRelay(LogicRelay),
	#[serde(rename="logic_timer")]
	LogicTimer(LogicTimer),
	#[serde(rename="math_counter")]
	MathCounter(MathCounter),
	#[serde(rename="move_rope")]
	MoveRope(MoveRope),
	#[serde(rename="path_track")]
	PathTrack(PathTrack),
	#[serde(rename="phys_ballsocket")]
	PhysBallsocket(PhysBallsocket),
	#[serde(rename="player_speedmod")]
	PlayerSpeedmod(PlayerSpeedmod),
	#[serde(rename="player_weaponstrip")]
	PlayerWeaponstrip(PlayerWeaponstrip),
	#[serde(rename="point_clientcommand")]
	PointClientcommand(PointClientcommand),
	#[serde(rename="point_servercommand")]
	PointServercommand(PointServercommand),
	#[serde(rename="point_spotlight")]
	PointSpotlight(PointSpotlight),
	#[serde(rename="point_template")]
	PointTemplate(PointTemplate),
	#[serde(rename="point_viewcontrol")]
	PointViewcontrol(PointViewcontrol),
	#[serde(rename="prop_dynamic")]
	PropDynamic(PropDynamic),
	#[serde(rename="prop_physics")]
	PropPhysics(PropPhysics),
	#[serde(rename="prop_physics_multiplayer")]
	PropPhysicsMultiplayer(PropPhysicsMultiplayer),
	#[serde(rename="prop_physics_override")]
	PropPhysicsOverride(PropPhysicsOverride),
	#[serde(rename="prop_ragdoll")]
	PropRagdoll(PropRagdoll),
	#[serde(rename="shadow_control")]
	ShadowControl(ShadowControl),
	#[serde(rename="sky_camera")]
	SkyCamera(SkyCamera),
	#[serde(rename="trigger_gravity")]
	TriggerGravity(TriggerGravity),
	#[serde(rename="trigger_hurt")]
	TriggerHurt(TriggerHurt),
	#[serde(rename="trigger_look")]
	TriggerLook(TriggerLook),
	#[serde(rename="trigger_multiple")]
	TriggerMultiple(TriggerMultiple),
	#[serde(rename="trigger_once")]
	TriggerOnce(TriggerOnce),
	#[serde(rename="trigger_push")]
	TriggerPush(TriggerPush),
	#[serde(rename="trigger_soundscape")]
	TriggerSoundscape(TriggerSoundscape),
	#[serde(rename="trigger_teleport")]
	TriggerTeleport(TriggerTeleport),
	#[serde(rename="water_lod_control")]
	WaterLodControl(WaterLodControl),
	#[serde(rename="weapon_ak47")]
	WeaponAk47(WeaponAk47),
	#[serde(rename="weapon_awp")]
	WeaponAwp(WeaponAwp),
	#[serde(rename="weapon_deagle")]
	WeaponDeagle(WeaponDeagle),
	#[serde(rename="weapon_elite")]
	WeaponElite(WeaponElite),
	#[serde(rename="weapon_famas")]
	WeaponFamas(WeaponFamas),
	#[serde(rename="weapon_fiveseven")]
	WeaponFiveseven(WeaponFiveseven),
	#[serde(rename="weapon_flashbang")]
	WeaponFlashbang(WeaponFlashbang),
	#[serde(rename="weapon_g3sg1")]
	WeaponG3Sg1(WeaponG3Sg1),
	#[serde(rename="weapon_glock")]
	WeaponGlock(WeaponGlock),
	#[serde(rename="weapon_hegrenade")]
	WeaponHegrenade(WeaponHegrenade),
	#[serde(rename="weapon_knife")]
	WeaponKnife(WeaponKnife),
	#[serde(rename="weapon_m249")]
	WeaponM249(WeaponM249),
	#[serde(rename="weapon_m3")]
	WeaponM3(WeaponM3),
	#[serde(rename="weapon_m4a1")]
	WeaponM4A1(WeaponM4A1),
	#[serde(rename="weapon_mac10")]
	WeaponMac10(WeaponMac10),
	#[serde(rename="weapon_p90")]
	WeaponP90(WeaponP90),
	#[serde(rename="weapon_scout")]
	WeaponScout(WeaponScout),
	#[serde(rename="weapon_sg550")]
	WeaponSg550(WeaponSg550),
	#[serde(rename="weapon_smokegrenade")]
	WeaponSmokegrenade(WeaponSmokegrenade),
	#[serde(rename="weapon_tmp")]
	WeaponTmp(WeaponTmp),
	#[serde(rename="weapon_ump45")]
	WeaponUmp45(WeaponUmp45),
	#[serde(rename="weapon_usp")]
	WeaponUsp(WeaponUsp),
	#[serde(rename="weapon_xm1014")]
	WeaponXm1014(WeaponXm1014),
	#[serde(rename="worldspawn")]
	Worldspawn(Worldspawn),
}


#[derive(Debug, Clone, Deserialize)]
pub struct CounterTerroristSpawn <'a>{
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
pub struct InfoPlayerTerrorist<'a>{
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
    pub disableshadows: "0",
    pub renderamt: "255",
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
    pub _ambienthdr: "-1 -1 -1 1"
}

#[derive(Debug, Clone, Deserialize)]
pub struct GamePlayerEquip<'a>{
	pub origin:Vector,
    pub weapon_knife: "1",
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerHurt<'a>{
    pub damagecap: "1000",
    pub model: &'a str,
    pub spawnflags: u32,
    pub damagetype: "0",
    pub origin:Vector,
    pub damage: "1000",

#[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub damagemodel: &'a str"
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncButton<'a>{
    pub movedir: Vector,
    pub onpressed: "kill04,disable,,0.5,-1",
    pub spawnflags: u32,
    pub unlocked_sentence: "0",
    pub speed: f32,
    pub wait: "3",
    pub sounds: "0",
    pub origin:Vector,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    pub lip: "0",
    pub model: &'a str,
    pub locked_sentence: "0",
    pub unlocked_sound: "0",
    pub renderamt: "255",
    pub rendermode: u32,
    pub locked_sound: "0",
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub health: "0"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponM4A1<'a>{
    pub spawnflags: u32,
    pub angles:Vector,
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WaterLodControl<'a>{
    pub cheapwaterenddistance: "2000",
    pub cheapwaterstartdistance: "1000",
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncWall<'a>{
    pub renderamt: "100",
    pub rendercolor: Color,
    pub rendermode: u32,
    pub disableshadows: "0",
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    pub model: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvTonemapController<'a>{
	pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSun<'a>{
    pub rendercolor: Color,
    pub target: &'a str,
    pub hdrcolorscale: f32,
    pub material: "sprites/light_glow02_add_noz",
    pub size: "12",
    pub origin:Vector,
    pub angles:Vector,
    pub overlaymaterial: "sprites/light_glow02_add_noz",
    pub overlaycolor: "0 0 0",
    pub overlaysize: "-1"
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
    pub spawnflags: u32"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSpritetrail<'a>{
    pub endwidth: "1.0",
    pub lifetime: "1",
    pub parentname: &'a str,
    pub origin:Vector,
    pub rendercolor: Color,
    pub renderamt: "255",
    pub startwidth: "8.0",
    pub spritename: "sprites/bluelaser1.vmt",
    pub rendermode: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponDeagle<'a>{
    pub origin:Vector,
    pub angles:Vector,
    pub ammo: u32
}

#[derive(Debug, Clone, Deserialize)]
pub struct ShadowControl<'a>{
	pub origin:Vector,
	pub angles:Vector,
    pub distance: "75",
    pub color: "128 128 128",
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
    pub volume: "0",
    pub model: &'a str,
    pub solidbsp: "0",
    pub dmg: "0",
    pub disableshadows: "1",
    pub renderamt: "255",
    pub parentname: &'a str,
    pub fanfriction: "0",
    pub origin:Vector,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    //pub hammerid: "7383",
    pub maxspeed: "50",
    pub angles:Vector,
    pub rendermode: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncBreakable<'a>{
    pub gibdir: "0 0 0",
    pub rendercolor: Color,
    pub performancemode: "0",
    pub explodemagnitude: "0",
    pub pressuredelay: "0",
    pub origin:Vector,
    pub explodedamage: "0",
    pub material: "10",
    pub model: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub explosion: "0",
    pub propdata: "0",
    pub spawnobject: "0",
    pub exploderadius: "0",
    pub health: "170",
    pub rendermode: u32,
    pub nodamageforces: "0",
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    //pub hammerid: "8344",
    pub renderamt: "255",
    pub physdamagescale: "1.0",
    pub spawnflags: u32,
    pub disableshadows: "1",
    pub minhealthdmg: "0"
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
    pub onmapspawn: "d_red_3,open,,0,-1"
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncWallToggle<'a>{
    pub rendercolor: Color,
    pub renderamt: "255",
    //pub hammerid: "20341",
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    pub model: &'a str,
    pub disableshadows: "1",
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
    pub spawnflags: u32"
}

#[derive(Debug, Clone, Deserialize)]
pub struct PointViewcontrol<'a>{
    pub wait: "10",
    pub angles:Vector,
    pub spawnflags: u32,
    pub acceleration: "500",
    pub target: &'a str,
    pub deceleration: "500",
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
    pub template01: "scout1",
    pub spawnflags: u32,
    pub targetname: &'a str,
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncDoorRotating<'a>{
    pub unlocked_sentence: "0",
    pub renderamt: "255",
    pub rendermode: u32,
    pub spawnflags: u32,
    pub forceclosed: "0",
    pub locked_sentence: "0",
    pub dmg: "0",
    pub spawnpos: "0",
    pub ignoredebris: "0",
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub solidbsp: "0",
    pub rendercolor: Color,
    pub model: &'a str,
    pub targetname: &'a str,
    pub health: "0",
    pub origin:Vector,
    pub lip: "0",
    pub speed: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    pub loopmovesound: "0",
    pub angles:Vector,
    pub wait: "3",
    pub distance: "135",
    pub disableshadows: "0"
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerPush<'a>{
    pub alternateticksfix: "0",
    pub speed: f32,
    pub model: &'a str,
    pub origin:Vector,
    pub spawnflags: u32,
    pub pushdir: "0 90 0",

#[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncPhysbox<'a>{
    pub preferredcarryangles: "0 0 0",
    pub spawnobject: "0",
    pub disableshadows: "0",
    pub spawnflags: u32,
    pub explodedamage: "0",
    pub performancemode: "0",
    pub gibdir: "0 0 0",
    pub propdata: "0",
    pub damagetoenablemotion: "0",
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub massscale: "0",
    pub nodamageforces: "0",
    pub origin:Vector,
    pub parentname: &'a str,
    pub notsolid: "1",
    pub damagetype: "0",
    pub exploderadius: "0",
    pub renderamt: "255",
    pub rendercolor: Color,
    pub explosion: "0",
    pub material: "7",
    pub rendermode: u32,
    pub forcetoenablemotion: "0",
    pub explodemagnitude: "0",
    pub health: "0",
    pub model: &'a str,
    pub pressuredelay: "0"
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
    pub negated: "1",
    pub targetname: &'a str,
    pub damagetype: "32"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvFogController<'a>{
    pub fogend: "4000",
    pub fogcolor: "0 0 0",
    pub origin:Vector,
    pub fogcolor2: "255 255 255",
    pub fogenable: "1",
    pub farz: "16384",
    pub spawnflags: u32,
    pub use_angles: "1",
    pub fogstart: "1000",
    pub mindxlevel: "0",
    pub fogdir: "0 0 -1",
    pub maxdxlevel: "0",
    pub fogblend: "0",
    pub foglerptime: "0",
    pub angles:Vector,
    pub fogmaxdensity: "1",
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
    pub disableshadows: "0",
    pub speed: f32,
    pub renderamt: "255",
    pub model: &'a str,
    pub movedir: Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FilterActivatorName<'a>{
	pub origin:Vector,
    pub negated: "allow entities that match criteria",
    pub targetname: &'a str,
    pub filtername: "activator"
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerMultiple<'a>{
    pub model: &'a str,

#[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub wait: "0.01",
    pub spawnflags: u32,
    pub ontrigger: "!activator,addoutput,targetname default,0.13,-1",
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FilterMulti<'a>{
    pub targetname: &'a str,
    pub filter02: "filt_2",
    pub origin:Vector,
    pub negated: "0",
    pub filter01: "filt_1",
    //pub hammerid: "49342",
    pub filtertype: "1"
}

#[derive(Debug, Clone, Deserialize)]
pub struct PropPhysicsOverride<'a>{
	pub origin:Vector,
    pub spawnflags: u32,
    pub skin: "0",
    pub nodamageforces: "0",
    pub performancemode: "0",
    pub minhealthdmg: "0",
    pub fadescale: "1",
    pub exploderadius: "0",
    pub explodedamage: "0",
    pub damagetype: "0",
    pub angles:Vector,
    //pub hammerid: "89676",
    pub inertiascale: "1.0",
    pub forcetoenablemotion: "0",
    pub model: &'a str,
    pub massscale: "0",
    pub maxdxlevel: "0",
    pub fademindist: "-1",
    pub fademaxdist: "0",
    pub damagetoenablemotion: "0",
    pub parentname: &'a str,
    pub shadowcastdist: "0",
    pub health: "0",
    pub disableshadows: "0",
    pub pressuredelay: "0",
    pub mindxlevel: "0",
    pub physdamagescale: "0.1"
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
    pub weaponname: "weapon_mp5navy",
    pub maxpieces: "0",
    pub origin:Vector,
    pub ammomod: "1",
    //pub hammerid: "90603"
}

#[derive(Debug, Clone, Deserialize)]
pub struct FilterActivatorClass<'a>{
	pub origin:Vector,
    pub targetname: &'a str,
    pub negated: "allow entities that match criteria",
    pub filterclass: "hegrenade_projectile",
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
pub struct EnvFire<'a>{
    pub spawnflags: u32,

#[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub ignitionpoint: "32",
    pub health: "1",
    pub origin:Vector,
    pub damagescale: "1.0",
    //pub hammerid: "101536",
    pub firesize: "64",
    pub firetype: "0",
    pub fireattack: "4"
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerWeaponstrip<'a>{
    pub targetname: &'a str,
    //pub hammerid: "107702",
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncBuyzone<'a>{
    pub teamnum: "2",
    pub model: &'a str,
    //pub hammerid: "113521"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSoundscape<'a>{

#[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    //pub hammerid: "133638",
    pub origin:Vector,
    pub radius: "848",
    pub soundscape: "lego1"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSprite<'a>{
    pub glowproxysize: "2.0",
    pub rendermode: u32,
    pub mindxlevel: "0",
    pub framerate: "10.0",
    //pub hammerid: "134683",
    pub maxdxlevel: "0",
    pub hdrcolorscale: f32,
    pub origin:Vector,
    pub renderamt: "255",
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
    pub max: "1",
    pub origin:Vector,
    pub onhitmax: "!self,disable,,0,1",
    //pub hammerid: "134898",
    pub startvalue: "0",
    pub min: "0"
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
    pub entitytemplate: "noobtemplate",
    pub angles:Vector,
    //pub hammerid: "176738",
    pub spawnflags: u32,
    pub postspawndirection: "0 0 0",
    pub origin:Vector,
    pub targetname: &'a str,
    pub postspawninheritangles: "0",
    pub postspawndirectionvariance: "0.15",
    pub postspawnspeed: "0"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSoundscapeTriggerable<'a>{
    pub soundscape: "lego2",
    pub radius: "128",
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
    pub message: "press e to restart the map"
}

#[derive(Debug, Clone, Deserialize)]
pub struct PropDynamic<'a>{
    pub solid: "0",
    pub mindxlevel: "0",
    pub rendercolor: Color,
    pub disableshadows: "0",
    //pub hammerid: "190009",
    pub randomanimation: "0",
    pub explodedamage: "0",
    pub angles:Vector,
    pub pressuredelay: "0",
    pub disablebonefollowers: "1",
    pub rendermode: u32,
    pub exploderadius: "0",
    pub fadescale: "1",
    pub spawnflags: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub renderamt: "255",
    pub performancemode: "0",
    pub maxanimtime: "10",
    pub skin: "0",
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    pub model: &'a str,
    pub fademindist: "-1",
    pub setbodygroup: "0",
    pub fademaxdist: "0",
    pub origin:Vector,
    pub minanimtime: "5",
    pub maxdxlevel: "0",

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
    pub soundscape: "nuke.abomb",
    pub model: &'a str,
    pub origin:Vector,
    pub spawnflags: u32,
    //pub hammerid: "7740"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSpark<'a>{
    pub spawnflags: u32,
    pub targetname: &'a str,
    pub maxdelay: "0",
    pub magnitude: "1",
    pub angles:Vector,
    //pub hammerid: "34153",
    pub origin:Vector,
    pub traillength: "1"
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogicTimer<'a>{
    pub spawnflags: u32,
    pub userandomtime: "1",
    pub lowerrandombound: "1

#[serde(deserialize_with = "bool_from_int")]2",
    pub startdisabled: bool,
    //pub hammerid: "34188",
    pub ontimer: "spark1,sparkonce,,0,-1",
    pub upperrandombound: "20",
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PropPhysicsMultiplayer<'a>{
    pub fadescale: "1",
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub spawnflags: u32,
    pub fademaxdist: "1200",
    pub physicsmode: "0",
    pub inertiascale: "1.0",
    pub massscale: "0",
    pub shadowcastdist: "0",
    pub explodedamage: "0",
    pub rendercolor: Color,
    pub disableshadows: "1",
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    pub angles:Vector,
    pub performancemode: "0",
    pub mindxlevel: "0",
    pub renderamt: "255",
    //pub hammerid: "35774",
    pub skin: "0",
    pub rendermode: u32,
    pub minhealthdmg: "0",
    pub maxdxlevel: "0",
    pub forcetoenablemotion: "0",
    pub physdamagescale: "0.1",
    pub exploderadius: "0",
    pub damagetype: "0",
    pub model: &'a str,
    pub origin:Vector,
    pub pressuredelay: "0",
    pub nodamageforces: "0",
    pub fademindist: "1000",
    pub damagetoenablemotion: "0",
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
    pub fademindist: "200",
    pub fademaxdist: "350",
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
    pub pressuredelay: "0",
    pub explodedamage: "0",
    pub damagetoenablemotion: "0",
    pub inertiascale: "1.0",
    pub spawnflags: u32,
    pub performancemode: "0",
    pub forcetoenablemotion: "0",
    pub fademindist: "2000",
    pub minhealthdmg: "0",
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    pub disableshadows: "0",
    pub exploderadius: "0",
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub mindxlevel: "0",
    pub massscale: "0",
    pub physdamagescale: "0.1",
    pub shadowcastdist: "0",
    pub origin:Vector,
    //pub hammerid: "487103",
    pub damagetype: "0",
    pub fadescale: "1",
    pub rendermode: u32,
    pub renderamt: "255",
    pub skin: "0",
    pub model: &'a str,
    pub nodamageforces: "1",
    pub angles:Vector,
    pub fademaxdist: "2100",
    pub rendercolor: Color,
    pub maxdxlevel: "0"
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
    pub renderamt: "255",
    pub spawnflags: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub startposition: bool,
    pub movedistance: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncPrecipitation<'a>{
	//pub hammerid: "288365",
    pub renderamt: "100",
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
    pub glowproxysize: "0",
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
    pub renderamt: "160",
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
    pub lifetime: "8",
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
    pub color: "128 128 128",
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
    pub renderamt: "255",
    pub targetname: &'a str,
    pub spawnflags: u32,
    pub origin:Vector,
    pub rendercolor: Color,
    //pub hammerid: "393222"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponTmp<'a>{
    pub maxdxlevel: "0",
    pub angles:Vector,
    pub fadescale: "1.0",
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub shadowcastdist: "0",
    //pub hammerid: "402780",
    pub fademindist: "-1.0",
    pub mindxlevel: "0",
    pub rendermode: u32,
    pub renderamt: "255",
    pub rendercolor: Color,
    pub fademaxdist: "0.0",
    pub origin:Vector,
    pub spawnflags: u32,
    pub nodamageforces: "0",
    pub ammo: u32
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponXm1014<'a>{
	#[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub rendercolor: Color,
    //pub hammerid: "402832",
    pub spawnflags: u32,
    pub mindxlevel: "0",
    pub ammo: u32,
    pub nodamageforces: "0",
    pub renderamt: "255",
    pub origin:Vector,
    pub fademaxdist: "0.0",
    pub rendermode: u32,
    pub maxdxlevel: "0",
    pub fadescale: "1.0",
    pub shadowcastdist: "0",
    pub fademindist: "-1.0",
    pub angles:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponMac10<'a>{
    pub spawnflags: u32,
    pub rendermode: u32,
    pub fademindist: "-1.0",
    pub ammo: u32,
    pub angles:Vector,
    pub shadowcastdist: "0",
    pub rendercolor: Color,
    pub fademaxdist: "0.0",
    //pub hammerid: "403143",
    pub origin:Vector,
    pub renderamt: "255",
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub nodamageforces: "0",
    pub mindxlevel: "0",
    pub fadescale: "1.0",
    pub maxdxlevel: "0"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponUmp45<'a>{
    pub rendermode: u32,
    pub spawnflags: u32,
    pub shadowcastdist: "0",
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub fademaxdist: "0.0",
    //pub hammerid: "403301",
    pub ammo: u32,
    pub mindxlevel: "0",
    pub renderamt: "255",
    pub origin:Vector,
    pub nodamageforces: "0",
    pub fademindist: "-1.0",
    pub angles:Vector,
    pub maxdxlevel: "0",
    pub fadescale: "1.0",
    pub rendercolor: Color,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponFamas<'a>{
	//pub hammerid: "403360",
    pub shadowcastdist: "0",
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub rendermode: u32,
    pub nodamageforces: "0",
    pub origin:Vector,
    pub fademaxdist: "0.0",
    pub maxdxlevel: "0",
    pub renderamt: "255",
    pub mindxlevel: "0",
    pub angles:Vector,
    pub fadescale: "1.0",
    pub spawnflags: u32,
    pub rendercolor: Color,
    pub fademindist: "-1.0",
    pub ammo: u32
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponG3Sg1<'a>{
	pub angles:Vector,
    pub nodamageforces: "0",
    pub rendermode: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub fademaxdist: "0.0",
    pub spawnflags: u32,
    pub fadescale: "1.0",
    pub ammo: u32,
    //pub hammerid: "403518",
    pub renderamt: "255",
    pub fademindist: "-1.0",
    pub origin:Vector,
    pub mindxlevel: "0",
    pub rendercolor: Color,
    pub shadowcastdist: "0",
    pub maxdxlevel: "0",
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponSg550<'a>{
    pub renderamt: "255",
    //pub hammerid: "403621",
    pub origin:Vector,
    pub maxdxlevel: "0",
    pub fadescale: "1.0",
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub rendermode: u32,
    pub spawnflags: u32,
    pub nodamageforces: "0",
    pub fademaxdist: "0.0",
    pub rendercolor: Color,
    pub mindxlevel: "0",
    pub ammo: u32,
    pub angles:Vector,
    pub fademindist: "-1.0",
    pub shadowcastdist: "0"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponFlashbang<'a>{
	pub origin:Vector,
    pub fademindist: "-1.0",
    pub spawnflags: u32,
    //pub hammerid: "416160",
    pub renderamt: "255",
    pub fadescale: "1.0",
    pub fademaxdist: "0.0",
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
    pub mindxlevel: "0",
    pub ropematerial: &'a str,
    pub texturescale: "1",
    pub collide: "0",
    //pub hammerid: "129625",
    pub angles:Vector,
    pub breakable: "0",
    pub type: "0",
    pub maxdxlevel: "0",
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
    pub renderamt: "255",
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
    pub texturescroll: "35",
    pub dissolvetype: "none",
    pub lasertarget: &'a str,
    pub width: "4",
    pub rendercolor: Color,
    pub renderamt: "100",
    pub texture: &'a str,
    pub damage: "0",
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
    pub orientationtype: "1",
    pub angles:Vector,
    pub spawnflags: u32,
    pub target: &'a str,
    //pub hammerid: "30225",
    pub targetname: &'a str,
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncTracktrain<'a>{
    pub velocitytype: "1",
    pub rendermode: u32,
    pub orientationtype: "1",
    pub movesoundmaxpitch: "200",
    pub movesoundmintime: "0",
    pub height: "4",
    pub target: &'a str,
    pub speed: f32,
    pub dmg: "0",
    pub renderamt: "255",
    pub targetname: &'a str,
    pub movesound: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    pub movesoundminpitch: "60",
    //pub hammerid: "30270",
    pub rendercolor: Color,
    pub volume: "10",
    pub wheels: "50",
    pub bank: "0",
    pub model: &'a str,
    pub spawnflags: u32,
    pub movesoundmaxtime: "0",
    pub disableshadows: "1",
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
    pub renderamt: "255",
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
    pub radius: "375",
    //pub hammerid: "105159"
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncRotButton<'a>{
    pub distance: "90",
    pub sounds: "25",
    pub angles:Vector,
    pub origin:Vector,
    pub onpressed: "water_movelin01,close,,30,-1",
    pub wait: "3",

#[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub targetname: &'a str,
    pub speed: f32,
    //pub hammerid: "239116",
    pub health: "0",
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
    pub fogdir: "1 0 0",
    pub scale: "16",
    pub fogstart: "500.0",
    pub fogend: "2000.0",
    pub fogcolor2: "255 255 255",
    //pub hammerid: "361080",
    pub angles:Vector,
    pub fogcolor: "255 255 255",
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GameUi<'a>{
    pub fieldofview: "-1.0",
    pub origin:Vector,
    pub pressedforward: "push_forward,enable,,0,-1",
    pub playeroff: "button_lvl1,addoutput,renderamt 200,0,-1",
    pub unpressedforward: "push_forward,disable,,0,-1",
    pub spawnflags: u32,
    //pub hammerid: "435027",
    pub unpressedmoveright: "push_right,disable,,0,-1",
    pub unpressedmoveleft: "push_left,disable,,0,-1",
    pub unpressedback: "push_backward,disable,,0,-1",
    pub pressedmoveright: "push_right,enable,,0,-1",
    pub pressedback: "push_backward,enable,,0,-1",
    pub pressedmoveleft: "push_left,enable,,0,-1",
    pub targetname: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncPhysboxMultiplayer<'a>{
    pub targetname: &'a str,
    pub health: "0",
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    pub pressuredelay: "0",
    pub damagetype: "0",
    pub material: "10",
    pub preferredcarryangles: "0 0 0",
    pub origin:Vector,
    pub exploderadius: "0",
    pub propdata: "0",
    pub model: &'a str,
    pub rendermode: u32,
    pub notsolid: "0",
    pub explodemagnitude: "0",
    pub explosion: "0",
    pub gibdir: "0 0 0",
    pub renderamt: "255",
    //pub hammerid: "435046",
    pub disableshadows: "1",
    pub performancemode: "0",
    pub forcetoenablemotion: "0",
    pub _minlight: "2",
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub spawnflags: u32,
    pub damagetoenablemotion: "0",
    pub spawnobject: "0",
    pub nodamageforces: "0",
    pub explodedamage: "0",
    pub massscale: "0.01"
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
    pub fieldofview: "0.9",
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
    pub skin: "0",
    pub renderamt: "255",
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
    pub fadescale: "1",
    pub modelscale: f32,
    pub origin:Vector,
    pub model: &'a str,
    pub fademindist: "-1",
    pub skin: "0",
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
    pub texturescroll: "35",
    pub hdrcolorscale: f32,
    //pub hammerid: "774042",
    pub spawnflags: u32,
    pub life: f32,
    pub decalname: &'a str,
    pub renderamt: "200",
    pub boltwidth: "6",
    pub noiseamplitude: "12",
    pub striketime: "1",
    pub texture: &'a str,
    pub lightningstart: "start_tesla1",
    pub radius: "256",
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
