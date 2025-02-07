
use serde::Deserialize;

use crate::{Angles, Color, LightColor, Vector};

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
#[serde(tag = "classname")]
pub enum Entity {
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
pub struct CounterTerroristSpawn {
	pub origin: Vector,
    pub angles: Angles,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Worldspawn{
    "maxpropscreenwidth": "-1",
    "skyname": "italy",
    "detailmaterial": "detail/detailsprites",
    "world_mins": "-2737 -1681 -244",
    "world_maxs": "432 944 360",
    "detailvbsp": "detail.vbsp"
}

#[derive(Debug, Clone, Deserialize)]
pub struct InfoPlayerTerrorist{
    pub origin:Vector,
    pub angles:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerTeleport{
    "target": "level1",
    "startdisabled": "0",
    pub origin:Vector,
    "spawnflags": "1",
    "model": "*2"
}

#[derive(Debug, Clone, Deserialize)]
pub struct InfoTeleportDestination{
	pub origin:Vector,
    "targetname": "level1",
    pub angles:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncWaterAnalog{
    "rendercolor": "255 255 255",
    "renderfx": "0",
    "model": "*6",
    "waveheight": "3.0",
    "_minlight": "0.0",
    "movedir": "0 0 0",
    "movedistance": "100",
    "speed": "100",
    "disablereceiveshadows": "0",
    "disableshadows": "0",
    "renderamt": "255",
    pub origin:Vector,
    "rendermode": "0"
}

#[derive(Debug, Clone, Deserialize)]
pub struct LightEnvironment{
	pub origin:Vector,
    "_light": "255 255 198 300",
    pub angles:Vector,
    "_ambient": "255 255 255 20",
    "_lighthdr": "-1 -1 -1 1",
    "pitch": "-90",
    "_ambienthdr": "-1 -1 -1 1"
}

#[derive(Debug, Clone, Deserialize)]
pub struct GamePlayerEquip{
	pub origin:Vector,
    "weapon_knife": "1",
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerHurt{
    "damagecap": "1000",
    "model": "*76",
    "spawnflags": "1",
    "damagetype": "0",
    pub origin:Vector,
    "damage": "1000",
    "startdisabled": "0",
    "damagemodel": "0"
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncButton{
    "movedir": "0 0 0",
    "onpressed": "kill04,disable,,0.5,-1",
    "spawnflags": "1025",
    "unlocked_sentence": "0",
    "speed": "5",
    "wait": "3",
    "sounds": "0",
    pub origin:Vector,
    "rendercolor": "255 255 255",
    "disablereceiveshadows": "0",
    "lip": "0",
    "model": "*81",
    "locked_sentence": "0",
    "unlocked_sound": "0",
    "renderamt": "255",
    "rendermode": "0",
    "locked_sound": "0",
    "renderfx": "0",
    "health": "0"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponM4A1{
    "spawnflags": "1",
    pub angles:Vector,
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WaterLodControl{
    "cheapwaterenddistance": "2000",
    "cheapwaterstartdistance": "1000",
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncWall{
    "renderamt": "100",
    "rendercolor": "255 255 255",
    "rendermode": "2",
    "disableshadows": "0",
    "disablereceiveshadows": "0",
    "model": "*1",
    "renderfx": "0"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvTonemapController{
	pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSun{
    "rendercolor": "237 232 216",
    "target": "sun",
    "hdrcolorscale": "1.0",
    "material": "sprites/light_glow02_add_noz",
    "size": "12",
    pub origin:Vector,
    pub angles:Vector,
    "overlaymaterial": "sprites/light_glow02_add_noz",
    "overlaycolor": "0 0 0",
    "overlaysize": "-1"
}

#[derive(Debug, Clone, Deserialize)]
pub struct InfoTarget{
    "spawnflags": "0",
    pub origin:Vector,
    "targetname": "sun",
    pub angles:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponAwp{
	pub origin:Vector,
    pub angles:Vector,
    "targetname": "secretw1",
    "spawnflags": "1"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSpritetrail{
    "endwidth": "1.0",
    "lifetime": "1",
    "parentname": "secretw1",
    pub origin:Vector,
    "rendercolor": "0 13 168",
    "renderamt": "255",
    "startwidth": "8.0",
    "spritename": "sprites/bluelaser1.vmt",
    "rendermode": "5"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponDeagle{
    pub origin:Vector,
    pub angles:Vector,
    "ammo": "31"
}

#[derive(Debug, Clone, Deserialize)]
pub struct ShadowControl{
	pub origin:Vector,
	pub angles:Vector,
    "distance": "75",
    "color": "128 128 128",
    //"hammerid": "2971"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponElite{
    //"hammerid": "7248",
    "targetname": "gunmapby",
    "ammo": "31",
    "spawnflags": "1",
    pub angles:Vector,
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncRotating{
    "spawnflags": "67",
    "volume": "0",
    "model": "*43",
    "solidbsp": "0",
    "dmg": "0",
    "disableshadows": "1",
    "renderamt": "255",
    "parentname": "gunmapby",
    "fanfriction": "0",
    pub origin:Vector,
    "rendercolor": "255 255 255",
    "disablereceiveshadows": "1",
    "renderfx": "0",
    //"hammerid": "7383",
    "maxspeed": "50",
    pub angles:Vector,
    "rendermode": "0"
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncBreakable{
    "gibdir": "0 0 0",
    "rendercolor": "255 255 255",
    "performancemode": "0",
    "explodemagnitude": "0",
    "pressuredelay": "0",
    pub origin:Vector,
    "explodedamage": "0",
    "material": "10",
    "model": "*44",
    "renderfx": "0",
    "explosion": "0",
    "propdata": "0",
    "spawnobject": "0",
    "exploderadius": "0",
    "health": "170",
    "rendermode": "0",
    "nodamageforces": "0",
    "disablereceiveshadows": "1",
    //"hammerid": "8344",
    "renderamt": "255",
    "physdamagescale": "1.0",
    "spawnflags": "0",
    "disableshadows": "1",
    "minhealthdmg": "0"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponP90{
    "ammo": "31",
    pub origin:Vector,
    "spawnflags": "1",
    "targetname": "p90",
    //"hammerid": "15611",
    pub angles:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponGlock{
    "targetname": "glock",
    //"hammerid": "19772",
    pub origin:Vector,
    pub angles:Vector,
    "spawnflags": "1",
    "ammo": "31"
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogicAuto{
	//"hammerid": "20253",
    pub origin:Vector,
    "spawnflags": "0",
    "onmapspawn": "d_red_3,open,,0,-1"
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncWallToggle{
    "rendercolor": "255 255 255",
    "renderamt": "255",
    //"hammerid": "20341",
    "disablereceiveshadows": "1",
    "model": "*210",
    "disableshadows": "1",
    "spawnflags": "0",
    "rendermode": "0",
    "renderfx": "0",
    "targetname": "wall_to_glock"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponM249{
	pub angles:Vector,
    "targetname": "m249",
    pub origin:Vector,
    //"hammerid": "24744",
    "spawnflags": "1",
    "ammo": "31",
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponHegrenade{
	pub origin:Vector,
	//"hammerid": "28302",
    "targetname": "nade",
    pub angles:Vector,
    "spawnflags": "1"
}

#[derive(Debug, Clone, Deserialize)]
pub struct PointViewcontrol{
    "wait": "10",
    pub angles:Vector,
    "spawnflags": "1",
    "acceleration": "500",
    "target": "cam1",
    "deceleration": "500",
    pub origin:Vector,
    //"hammerid": "36815"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponScout{
    "spawnflags": "0",
    "targetname": "scout1",
    pub origin:Vector,
    pub angles:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PointTemplate{
    "template01": "scout1",
    "spawnflags": "2",
    "targetname": "template1",
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncDoorRotating{
    "unlocked_sentence": "0",
    "renderamt": "255",
    "rendermode": "0",
    "spawnflags": "4114",
    "forceclosed": "0",
    "locked_sentence": "0",
    "dmg": "0",
    "spawnpos": "0",
    "ignoredebris": "0",
    "renderfx": "0",
    "solidbsp": "0",
    "rendercolor": "255 255 255",
    "model": "*453",
    "targetname": "door_finish",
    "health": "0",
    pub origin:Vector,
    "lip": "0",
    "speed": "60",
    "disablereceiveshadows": "0",
    "loopmovesound": "0",
    pub angles:Vector,
    "wait": "3",
    "distance": "135",
    "disableshadows": "0"
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerPush{
    "alternateticksfix": "0",
    "speed": "1000",
    "model": "*456",
    pub origin:Vector,
    "spawnflags": "1",
    "pushdir": "0 90 0",
    "startdisabled": "0",
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncPhysbox{
    "preferredcarryangles": "0 0 0",
    "spawnobject": "0",
    "disableshadows": "0",
    "spawnflags": "0",
    "explodedamage": "0",
    "performancemode": "0",
    "gibdir": "0 0 0",
    "propdata": "0",
    "damagetoenablemotion": "0",
    "disablereceiveshadows": "0",
    "renderfx": "0",
    "massscale": "0",
    "nodamageforces": "0",
    pub origin:Vector,
    "parentname": "wdk2",
    "notsolid": "1",
    "damagetype": "0",
    "exploderadius": "0",
    "renderamt": "255",
    "rendercolor": "255 255 255",
    "explosion": "0",
    "material": "7",
    "rendermode": "0",
    "forcetoenablemotion": "0",
    "explodemagnitude": "0",
    "health": "0",
    "model": "*655",
    "pressuredelay": "0"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponSmokegrenade{
    pub origin:Vector,
    pub angles:Vector,
    "spawnflags": "1",
    "targetname": "smoke1"
}

#[derive(Debug, Clone, Deserialize)]
pub struct FilterDamageType{
	pub origin:Vector,
    "negated": "1",
    "targetname": "falldmg",
    "damagetype": "32"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvFogController{
    "fogend": "4000",
    "fogcolor": "0 0 0",
    pub origin:Vector,
    "fogcolor2": "255 255 255",
    "fogenable": "1",
    "farz": "16384",
    "spawnflags": "0",
    "use_angles": "1",
    "fogstart": "1000",
    "mindxlevel": "0",
    "fogdir": "0 0 -1",
    "maxdxlevel": "0",
    "fogblend": "0",
    "foglerptime": "0",
    pub angles:Vector,
    "fogmaxdensity": "1",
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncConveyor{
    "spawnflags": "0",
    "disablereceiveshadows": "0",
    "rendercolor": "255 255 255",
    "rendermode": "0",
    pub angles:Vector,
    "renderfx": "0",
    "disableshadows": "0",
    "speed": "300",
    "renderamt": "255",
    "model": "*60",
    "movedir": "0 45 0"
}

#[derive(Debug, Clone, Deserialize)]
pub struct FilterActivatorName{
	pub origin:Vector,
    "negated": "allow entities that match criteria",
    "targetname": "filter_activator",
    "filtername": "activator"
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerMultiple{
    "model": "*86",
    "startdisabled": "0",
    "wait": "0.01",
    "spawnflags": "1",
    "ontrigger": "!activator,addoutput,targetname default,0.13,-1",
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FilterMulti{
    "targetname": "1multi",
    "filter02": "filt_2",
    pub origin:Vector,
    "negated": "0",
    "filter01": "filt_1",
    //"hammerid": "49342",
    "filtertype": "1"
}

#[derive(Debug, Clone, Deserialize)]
pub struct PropPhysicsOverride{
	pub origin:Vector,
    "spawnflags": "524",
    "skin": "0",
    "nodamageforces": "0",
    "performancemode": "0",
    "minhealthdmg": "0",
    "fadescale": "1",
    "exploderadius": "0",
    "explodedamage": "0",
    "damagetype": "0",
    pub angles:Vector,
    //"hammerid": "89676",
    "inertiascale": "1.0",
    "forcetoenablemotion": "0",
    "model": "models/weapons/w_snip_scout.mdl",
    "massscale": "0",
    "maxdxlevel": "0",
    "fademindist": "-1",
    "fademaxdist": "0",
    "damagetoenablemotion": "0",
    "parentname": "scoutrotator",
    "shadowcastdist": "0",
    "health": "0",
    "disableshadows": "0",
    "pressuredelay": "0",
    "mindxlevel": "0",
    "physdamagescale": "0.1"
}

#[derive(Debug, Clone, Deserialize)]
pub struct PointServercommand{
    pub origin:Vector,
    "targetname": "servcommand",
    //"hammerid": "90371"
}

#[derive(Debug, Clone, Deserialize)]
pub struct PointClientcommand{
    "targetname": "clientcommand",
    //"hammerid": "90377",
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GameWeaponManager{
    "weaponname": "weapon_mp5navy",
    "maxpieces": "0",
    pub origin:Vector,
    "ammomod": "1",
    //"hammerid": "90603"
}

#[derive(Debug, Clone, Deserialize)]
pub struct FilterActivatorClass{
	pub origin:Vector,
    "targetname": "hegrenade",
    "negated": "allow entities that match criteria",
    "filterclass": "hegrenade_projectile",
    //"hammerid": "91384",
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerSpeedmod{
	//"hammerid": "91426",
    pub origin:Vector,
    "spawnflags": "0",
    "targetname": "speed"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvFire{
    "spawnflags": "31",
    "startdisabled": "0",
    "ignitionpoint": "32",
    "health": "1",
    pub origin:Vector,
    "damagescale": "1.0",
    //"hammerid": "101536",
    "firesize": "64",
    "firetype": "0",
    "fireattack": "4"
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerWeaponstrip{
    "targetname": "strip",
    //"hammerid": "107702",
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncBuyzone{
    "teamnum": "2",
    "model": "*542",
    //"hammerid": "113521"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSoundscape{
    "startdisabled": "0",
    //"hammerid": "133638",
    pub origin:Vector,
    "radius": "848",
    "soundscape": "lego1"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSprite{
    "glowproxysize": "2.0",
    "rendermode": "0",
    "mindxlevel": "0",
    "framerate": "10.0",
    //"hammerid": "134683",
    "maxdxlevel": "0",
    "hdrcolorscale": "1.0",
    pub origin:Vector,
    "renderamt": "255",
    "disablereceiveshadows": "0",
    "model": "lego/longjumps.vmt",
    "spawnflags": "1",
    "renderfx": "0",
    "rendercolor": "255 255 255",
}

#[derive(Debug, Clone, Deserialize)]
pub struct MathCounter{
    "startdisabled": "0",
    "targetname": "counter",
    "max": "1",
    pub origin:Vector,
    "onhitmax": "!self,disable,,0,1",
    //"hammerid": "134898",
    "startvalue": "0",
    "min": "0"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponKnife{
    pub angles:Vector,
    //"hammerid": "176620",
    "spawnflags": "1",
    "targetname": "noobknife",
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvEntityMaker{
    "entitytemplate": "noobtemplate",
    pub angles:Vector,
    //"hammerid": "176738",
    "spawnflags": "0",
    "postspawndirection": "0 0 0",
    pub origin:Vector,
    "targetname": "noobmaker",
    "postspawninheritangles": "0",
    "postspawndirectionvariance": "0.15",
    "postspawnspeed": "0"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSoundscapeTriggerable{
    "soundscape": "lego2",
    "radius": "128",
    //"hammerid": "179655",
    "startdisabled": "0",
    "targetname": "first",
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvHudhint{
	pub origin:Vector,
    "spawnflags": "0",
    //"hammerid": "183578",
    "targetname": "quickrestart",
    "message": "press e to restart the map"
}

#[derive(Debug, Clone, Deserialize)]
pub struct PropDynamic{
    "solid": "0",
    "mindxlevel": "0",
    "rendercolor": "255 255 255",
    "disableshadows": "0",
    //"hammerid": "190009",
    "randomanimation": "0",
    "explodedamage": "0",
    pub angles:Vector,
    "pressuredelay": "0",
    "disablebonefollowers": "1",
    "rendermode": "0",
    "exploderadius": "0",
    "fadescale": "1",
    "spawnflags": "0",
    "renderfx": "0",
    "renderamt": "255",
    "performancemode": "0",
    "maxanimtime": "10",
    "skin": "0",
    "disablereceiveshadows": "0",
    "model": "models/player/slow/banana_joe/slow.mdl",
    "fademindist": "-1",
    "setbodygroup": "0",
    "fademaxdist": "0",
    pub origin:Vector,
    "minanimtime": "5",
    "maxdxlevel": "0",
    "startdisabled": "0"
}

#[derive(Debug, Clone, Deserialize)]
pub struct Infodecal{
    "texture": "decals/decalgraffiti044a",
    //"hammerid": "3739",
    pub angles:Vector,
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerSoundscape{
    "startdisabled": "0",
    "soundscape": "nuke.abomb",
    "model": "*407",
    pub origin:Vector,
    "spawnflags": "1",
    //"hammerid": "7740"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSpark{
    "spawnflags": "0",
    "targetname": "spark1",
    "maxdelay": "0",
    "magnitude": "1",
    pub angles:Vector,
    //"hammerid": "34153",
    pub origin:Vector,
    "traillength": "1"
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogicTimer{
    "spawnflags": "0",
    "userandomtime": "1",
    "lowerrandombound": "12",
    "startdisabled": "0",
    //"hammerid": "34188",
    "ontimer": "spark1,sparkonce,,0,-1",
    "upperrandombound": "20",
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PropPhysicsMultiplayer{
    "fadescale": "1",
    "renderfx": "0",
    "spawnflags": "256",
    "fademaxdist": "1200",
    "physicsmode": "0",
    "inertiascale": "1.0",
    "massscale": "0",
    "shadowcastdist": "0",
    "explodedamage": "0",
    "rendercolor": "255 255 255",
    "disableshadows": "1",
    "disablereceiveshadows": "1",
    pub angles:Vector,
    "performancemode": "0",
    "mindxlevel": "0",
    "renderamt": "255",
    //"hammerid": "35774",
    "skin": "0",
    "rendermode": "0",
    "minhealthdmg": "0",
    "maxdxlevel": "0",
    "forcetoenablemotion": "0",
    "physdamagescale": "0.1",
    "exploderadius": "0",
    "damagetype": "0",
    "model": "models/props_c17/oildrum001.mdl",
    pub origin:Vector,
    "pressuredelay": "0",
    "nodamageforces": "0",
    "fademindist": "1000",
    "damagetoenablemotion": "0",
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvWind{
	pub origin:Vector,
    "maxgustdelay": "20",
    "mingustdelay": "10",
    "maxgust": "250",
    "minwind": "20",
    "gustduration": "5",
    "mingust": "100",
    pub angles:Vector,
    //"hammerid": "38005",
    "maxwind": "50",
    "gustdirchange": "20",
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvDetailController{
    //"hammerid": "207110",
    pub origin:Vector,
    "fademindist": "200",
    "fademaxdist": "350",
    pub angles:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InfoPlayerStart{
	pub angles:Vector,
    pub origin:Vector,
    //"hammerid": "259724",
    "spawnflags": "0"
}

#[derive(Debug, Clone, Deserialize)]
pub struct PropPhysics{
    "pressuredelay": "0",
    "explodedamage": "0",
    "damagetoenablemotion": "0",
    "inertiascale": "1.0",
    "spawnflags": "14",
    "performancemode": "0",
    "forcetoenablemotion": "0",
    "fademindist": "2000",
    "minhealthdmg": "0",
    "disablereceiveshadows": "0",
    "disableshadows": "0",
    "exploderadius": "0",
    "renderfx": "0",
    "mindxlevel": "0",
    "massscale": "0",
    "physdamagescale": "0.1",
    "shadowcastdist": "0",
    pub origin:Vector,
    //"hammerid": "487103",
    "damagetype": "0",
    "fadescale": "1",
    "rendermode": "0",
    "renderamt": "255",
    "skin": "0",
    "model": "models/props_debris/concrete_cynderblock001.mdl",
    "nodamageforces": "1",
    pub angles:Vector,
    "fademaxdist": "2100",
    "rendercolor": "255 255 255",
    "maxdxlevel": "0"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponAk47{
    pub angles:Vector,
    "targetname": "ak1_2",
    "ammo": "1337",
    pub origin:Vector,
    "spawnflags": "1",
    //"hammerid": "155595"
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncMovelinear{
    "disablereceiveshadows": "0",
    "renderfx": "0",
    pub origin:Vector,
    "rendermode": "3",
    "parentname": "ak2_1",
    "rendercolor": "255 255 255",
    "blockdamage": "0",
    "model": "*616",
    "movedir": "0 0 0",
    "speed": "100",
    //"hammerid": "155597",
    "renderamt": "255",
    "spawnflags": "8",
    "startposition": "0",
    "movedistance": "100"
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncPrecipitation{
	//"hammerid": "288365",
    "renderamt": "100",
    "preciptype": "3",
    "rendercolor": "100 100 100",
    "model": "*1"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvLightglow{
    "hdrcolorscale": "0.5",
    pub origin:Vector,
    pub angles:Vector,
    "horizontalglowsize": "8",
    "verticalglowsize": "8",
    "maxdist": "256",
    //"hammerid": "156600",
    "outermaxdist": "0",
    "spawnflags": "0",
    "glowproxysize": "0",
    "mindist": "64",
    "rendercolor": "255 255 255"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSmokestack{
    "rendercolor": "255 255 255",
    "roll": "15.0",
    "windangle": "0",
    "twist": "2",
    "startsize": "20",
    "renderamt": "160",
    "basespread": "125",
    pub angles:Vector,
    "initialstate": "1",
    "jetlength": "200",
    "endsize": "30",
    "windspeed": "4",
    "smokematerial": "particle/smokestack.vmt",
    pub origin:Vector,
    "rate": "20",
    "spreadspeed": "15",
    //"hammerid": "129270",
    "speed": "30"
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncSmokevolume{
    "color2": "192 192 192",
    "particledrawwidth": "192",
    //"hammerid": "31779",
    "particlespacingdistance": "80",
    "color1": "192 192 192",
    "spawnflags": "0",
    "rotationspeed": "10",
    "movementspeed": "10",
    "material": "particle/particle_noisesphere",
    "densityrampspeed": "1",
    "density": "0.5",
    "model": "*73"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvEmbers{
	pub angles:Vector,
    "spawnflags": "1",
    //"hammerid": "363584",
    "model": "*356",
    "speed": "150.0",
    "lifetime": "8",
    "rendercolor": "255 128 0",
    "density": "32"
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncDustcloud{
    "frozen": "0",
    "sizemax": "150",
    "startdisabled": "0",
    //"hammerid": "370009",
    "lifetimemin": "3",
    "lifetimemax": "5",
    "color": "128 128 128",
    "distmax": "1024",
    "alpha": "96",
    "model": "*367",
    "speedmax": "6",
    "sizemin": "50",
    "spawnrate": "500"
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerOnce{
	pub origin:Vector,
    "model": "*398",
    //"hammerid": "387965",
    pub angles:Vector,
    "startdisabled": "0",
    "ontrigger": "servercommand,command,say level 5 unlocked,0,-1",
    "spawnflags": "1"
}

#[derive(Debug, Clone, Deserialize)]
pub struct PointSpotlight{
    "hdrcolorscale": "0.5",
    pub angles:Vector,
    "spotlightlength": "200.0",
    "spotlightwidth": "20.0",
    "renderamt": "255",
    "targetname": "scout_spotlight",
    "spawnflags": "3",
    pub origin:Vector,
    "rendercolor": "255 255 255",
    //"hammerid": "393222"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponTmp{
    "maxdxlevel": "0",
    pub angles:Vector,
    "fadescale": "1.0",
    "renderfx": "0",
    "shadowcastdist": "0",
    //"hammerid": "402780",
    "fademindist": "-1.0",
    "mindxlevel": "0",
    "rendermode": "0",
    "renderamt": "255",
    "rendercolor": "255 255 255",
    "fademaxdist": "0.0",
    pub origin:Vector,
    "spawnflags": "1",
    "nodamageforces": "0",
    "ammo": "120"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponXm1014{
    "renderfx": "0",
    "rendercolor": "255 255 255",
    //"hammerid": "402832",
    "spawnflags": "1",
    "mindxlevel": "0",
    "ammo": "32",
    "nodamageforces": "0",
    "renderamt": "255",
    pub origin:Vector,
    "fademaxdist": "0.0",
    "rendermode": "0",
    "maxdxlevel": "0",
    "fadescale": "1.0",
    "shadowcastdist": "0",
    "fademindist": "-1.0",
    pub angles:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponMac10{
    "spawnflags": "1",
    "rendermode": "0",
    "fademindist": "-1.0",
    "ammo": "100",
    pub angles:Vector,
    "shadowcastdist": "0",
    "rendercolor": "255 255 255",
    "fademaxdist": "0.0",
    //"hammerid": "403143",
    pub origin:Vector,
    "renderamt": "255",
    "renderfx": "0",
    "nodamageforces": "0",
    "mindxlevel": "0",
    "fadescale": "1.0",
    "maxdxlevel": "0"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponUmp45{
    "rendermode": "0",
    "spawnflags": "1",
    "shadowcastdist": "0",
    "renderfx": "0",
    "fademaxdist": "0.0",
    //"hammerid": "403301",
    "ammo": "100",
    "mindxlevel": "0",
    "renderamt": "255",
    pub origin:Vector,
    "nodamageforces": "0",
    "fademindist": "-1.0",
    pub angles:Vector,
    "maxdxlevel": "0",
    "fadescale": "1.0",
    "rendercolor": "255 255 255"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponFamas{
	//"hammerid": "403360",
    "shadowcastdist": "0",
    "renderfx": "0",
    "rendermode": "0",
    "nodamageforces": "0",
    pub origin:Vector,
    "fademaxdist": "0.0",
    "maxdxlevel": "0",
    "renderamt": "255",
    "mindxlevel": "0",
    pub angles:Vector,
    "fadescale": "1.0",
    "spawnflags": "1",
    "rendercolor": "255 255 255",
    "fademindist": "-1.0",
    "ammo": "90"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponG3Sg1{
	pub angles:Vector,
    "nodamageforces": "0",
    "rendermode": "0",
    "renderfx": "0",
    "fademaxdist": "0.0",
    "spawnflags": "1",
    "fadescale": "1.0",
    "ammo": "90",
    //"hammerid": "403518",
    "renderamt": "255",
    "fademindist": "-1.0",
    pub origin:Vector,
    "mindxlevel": "0",
    "rendercolor": "255 255 255",
    "shadowcastdist": "0",
    "maxdxlevel": "0",
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponSg550{
    "renderamt": "255",
    //"hammerid": "403621",
    pub origin:Vector,
    "maxdxlevel": "0",
    "fadescale": "1.0",
    "renderfx": "0",
    "rendermode": "0",
    "spawnflags": "1",
    "nodamageforces": "0",
    "fademaxdist": "0.0",
    "rendercolor": "255 255 255",
    "mindxlevel": "0",
    "ammo": "90",
    pub angles:Vector,
    "fademindist": "-1.0",
    "shadowcastdist": "0"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponFlashbang{
	pub origin:Vector,
    "fademindist": "-1.0",
    "spawnflags": "0",
    //"hammerid": "416160",
    "renderamt": "255",
    "fadescale": "1.0",
    "fademaxdist": "0.0",
    pub angles:Vector,
    "targetname": "flashbang",
    "rendercolor": "255 255 255"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvFireTrail{
    "targetname": "uber_scout_fire",
    //"hammerid": "425179",
    pub origin:Vector,
    "parentname": "uber_scout"
}

#[derive(Debug, Clone, Deserialize)]
pub struct InfoLadder{
    "maxs.y": "3528.00",
    "mins.z": "112.00",
    //"hammerid": "2854",
    "maxs.x": "-9792.00",
    "mins.y": "3520.00",
    "maxs.z": "640.00",
    "mins.x": "-9856.00"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponM3{
    "ammo": "999",
    //"hammerid": "176561",
    pub angles:Vector,
    "spawnflags": "1",
    "targetname": "gun",
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponFiveseven{
	pub origin:Vector,
    "ammo": "999",
    //"hammerid": "177007",
    "targetname": "gun6",
    "spawnflags": "1",
    pub angles:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MoveRope{
    "spawnflags": "0",
    "barbed": "0",
    "slack": "105",
    "nowind": "1",
    pub origin:Vector,
    "subdiv": "4",
    "movespeed": "64",
    "width": "1",
    "positioninterpolator": "2",
    "mindxlevel": "0",
    "ropematerial": "cable/cable",
    "texturescale": "1",
    "collide": "0",
    //"hammerid": "129625",
    pub angles:Vector,
    "breakable": "0",
    "type": "0",
    "maxdxlevel": "0",
    "dangling": "0",
    "targetname": "rope_20",
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvFade{
    "spawnflags": "4",
    "rendercolor": "0 0 0",
    pub origin:Vector,
    //"hammerid": "262982",
    "duration": "3",
    "holdtime": "0.0",
    "renderamt": "255",
    "targetname": "fadelj1"
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncAreaportalwindow{
    "translucencylimit": "0.0",
    "fadestartdist": "1950",
    "fadedist": "2000",
    "portalnumber": "1",
    "target": "brush_a",
    "portalversion": "1",
    //"hammerid": "270454"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvLaser{
    "texturescroll": "35",
    "dissolvetype": "none",
    "lasertarget": "las_target2",
    "width": "4",
    "rendercolor": "0 128 255",
    "renderamt": "100",
    "texture": "sprites/laserbeam.spr",
    "damage": "0",
    //"hammerid": "731916",
    pub origin:Vector,
    "spawnflags": "1"
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogicRelay{
	pub origin:Vector,
    //"hammerid": "805602",
    "ontrigger": "snek_btn_1,kill,,0,-1",
    "targetname": "snake_relay1",
    "spawnflags": "1"
}

#[derive(Debug, Clone, Deserialize)]
pub struct PathTrack{
    "orientationtype": "1",
    pub angles:Vector,
    "spawnflags": "0",
    "target": "vr_track1_2",
    //"hammerid": "30225",
    "targetname": "vr_track1_1",
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncTracktrain{
    "velocitytype": "1",
    "rendermode": "0",
    "orientationtype": "1",
    "movesoundmaxpitch": "200",
    "movesoundmintime": "0",
    "height": "4",
    "target": "vr_track1_1",
    "speed": "100",
    "dmg": "0",
    "renderamt": "255",
    "targetname": "vr_train1",
    "movesound": "ambient/atmosphere/city_beacon_loop1.wav",
    "disablereceiveshadows": "1",
    "movesoundminpitch": "60",
    //"hammerid": "30270",
    "rendercolor": "255 255 255",
    "volume": "10",
    "wheels": "50",
    "bank": "0",
    "model": "*105",
    "spawnflags": "538",
    "movesoundmaxtime": "0",
    "disableshadows": "1",
    pub origin:Vector,
    "renderfx": "0",
    "startspeed": "100"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvSteam{
    "rate": "26",
    "spawnflags": "0",
    "speed": "120",
    "startsize": "10",
    "renderamt": "255",
    "endsize": "25",
    "targetname": "cave_steam_dream01",
    "rollspeed": "8",
    //"hammerid": "65082",
    pub angles:Vector,
    pub origin:Vector,
    "rendercolor": "255 255 255",
    "spreadspeed": "15",
    "jetlength": "80"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvShake{
    "amplitude": "6",
    "targetname": "cave_shake01",
    pub origin:Vector,
    "duration": "4",
    "spawnflags": "0",
    "frequency": "2.5",
    "radius": "375",
    //"hammerid": "105159"
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncRotButton{
    "distance": "90",
    "sounds": "25",
    pub angles:Vector,
    pub origin:Vector,
    "onpressed": "water_movelin01,close,,30,-1",
    "wait": "3",
    "startdisabled": "0",
    "targetname": "rot_button01",
    "speed": "90",
    //"hammerid": "239116",
    "health": "0",
    "spawnflags": "1088",
    "model": "*496",
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerGravity{
	//"hammerid": "259593",
    pub origin:Vector,
    "spawnflags": "1",
    "model": "*562",
    "startdisabled": "0",
    "gravity": ".05",
}

#[derive(Debug, Clone, Deserialize)]
pub struct SkyCamera{
    "fogdir": "1 0 0",
    "scale": "16",
    "fogstart": "500.0",
    "fogend": "2000.0",
    "fogcolor2": "255 255 255",
    //"hammerid": "361080",
    pub angles:Vector,
    "fogcolor": "255 255 255",
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GameUi{
    "fieldofview": "-1.0",
    pub origin:Vector,
    "pressedforward": "push_forward,enable,,0,-1",
    "playeroff": "button_lvl1,addoutput,renderamt 200,0,-1",
    "unpressedforward": "push_forward,disable,,0,-1",
    "spawnflags": "480",
    //"hammerid": "435027",
    "unpressedmoveright": "push_right,disable,,0,-1",
    "unpressedmoveleft": "push_left,disable,,0,-1",
    "unpressedback": "push_backward,disable,,0,-1",
    "pressedmoveright": "push_right,enable,,0,-1",
    "pressedback": "push_backward,enable,,0,-1",
    "pressedmoveleft": "push_left,enable,,0,-1",
    "targetname": "ui"
}

#[derive(Debug, Clone, Deserialize)]
pub struct FuncPhysboxMultiplayer{
    "targetname": "box_lvl1",
    "health": "0",
    "disablereceiveshadows": "1",
    "pressuredelay": "0",
    "damagetype": "0",
    "material": "10",
    "preferredcarryangles": "0 0 0",
    pub origin:Vector,
    "exploderadius": "0",
    "propdata": "0",
    "model": "*614",
    "rendermode": "0",
    "notsolid": "0",
    "explodemagnitude": "0",
    "explosion": "0",
    "gibdir": "0 0 0",
    "renderamt": "255",
    //"hammerid": "435046",
    "disableshadows": "1",
    "performancemode": "0",
    "forcetoenablemotion": "0",
    "_minlight": "2",
    "rendercolor": "255 255 255",
    "renderfx": "0",
    "spawnflags": "524288",
    "damagetoenablemotion": "0",
    "spawnobject": "0",
    "nodamageforces": "0",
    "explodedamage": "0",
    "massscale": "0.01"
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerLook{
    "startdisabled": "0",
    "looktime": "5",
    //"hammerid": "491215",
    "target": "target_look",
    "spawnflags": "128",
    "model": "*748",
    "timeout": "0",
    "fieldofview": "0.9",
    "ontrigger": "door_pic,open,,0,-1",
    pub origin:Vector,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KeyframeRope{
	pub origin:Vector,
    "slack": "25",
    "texturescale": "1",
    "ropematerial": "cable/cable.vmt",
    "movespeed": "64",
    pub angles:Vector,
    //"hammerid": "543370",
    "width": "2",
    "targetname": "cave_rope02",
    "subdiv": "2"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvShooter{
    "m_flvariance": "0.3",
    pub angles:Vector,
    "renderfx": "0",
    "rendermode": "0",
    "m_flvelocity": "772",
    pub origin:Vector,
    "parentname": "wtf_weapon",
    "shootmodel": "models/gibs/agibs.mdl",
    "nogibshadows": "0",
    "spawnflags": "7",
    "massoverride": "0",
    "skin": "0",
    "renderamt": "255",
    "gibangles": "0 0 0",
    "delay": "3",
    "simulation": "0",
    "targetname": "wtf_shoot",
    "m_flgiblife": "1.5",
    "m_igibs": "99999",
    "gibgravityscale": "1",
    "rendercolor": "255 255 255",
    "disablereceiveshadows": "0",
    "shootsounds": "-1",
    //"hammerid": "564233"
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponUsp{
	//"hammerid": "597821",
    pub origin:Vector,
    "spawnflags": "1",
    "targetname": "spusp",
    pub angles:Vector,
    "ammo": "99",
}

#[derive(Debug, Clone, Deserialize)]
pub struct PropRagdoll{
    "fadescale": "1",
    "modelscale": "1.0",
    pub origin:Vector,
    "model": "models/humans/group03/male_06_bloody.mdl",
    "fademindist": "-1",
    "skin": "0",
    "spawnflags": "4",
    pub angles:Vector,
    //"hammerid": "739080",
    "targetname": "ragdoll01"
}

#[derive(Debug, Clone, Deserialize)]
pub struct PhysBallsocket{
    "targetname": "ballsocket",
    pub origin:Vector,
    "attach1": "ragdoll01",
    //"hammerid": "739103"
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvBeam{
	pub origin:Vector,
    "lightningend": "antlion1",
    "texturescroll": "35",
    "hdrcolorscale": "1.0",
    //"hammerid": "774042",
    "spawnflags": "48",
    "life": ".5",
    "decalname": "bigshot",
    "renderamt": "200",
    "boltwidth": "6",
    "noiseamplitude": "12",
    "striketime": "1",
    "texture": "sprites/laserbeam.spr",
    "lightningstart": "start_tesla1",
    "radius": "256",
    "rendercolor": "122 230 252",
    "targetname": "tesla_beam1"
}

#[derive(Debug, Clone, Deserialize)]
pub struct Light{
	pub origin:Vector,
    "_lightscalehdr": "1",
    "_quadratic_attn": "1",
    "_lighthdr": "-1 -1 -1 1",
    "_light": "255 128 64",
    //"hammerid": "823486"
}
