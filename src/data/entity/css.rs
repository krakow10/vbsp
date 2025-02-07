
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
pub struct Worldspawn
{
    "maxpropscreenwidth": "-1",
    "skyname": "italy",
    "detailmaterial": "detail/detailsprites",
    "classname": "worldspawn",
    "world_mins": "-2737 -1681 -244",
    "world_maxs": "432 944 360",
    "detailvbsp": "detail.vbsp"
},
pub struct InfoPlayerTerrorist
{
    "classname": "info_player_terrorist",
    "origin": "53.6539 -141 80",
    "angles": "0 0 0"
},
pub struct TriggerTeleport
{
    "target": "level1",
    "startdisabled": "0",
    "classname": "trigger_teleport",
    "origin": "200 240 29.5",
    "spawnflags": "1",
    "model": "*2"
},
pub struct InfoTeleportDestination
{
    "origin": "200 -64 72.8837",
    "targetname": "level1",
    "angles": "0 90 0",
    "classname": "info_teleport_destination"
},
pub struct FuncWaterAnalog
{
    "rendercolor": "255 255 255",
    "renderfx": "0",
    "classname": "func_water_analog",
    "model": "*6",
    "waveheight": "3.0",
    "_minlight": "0.0",
    "movedir": "0 0 0",
    "movedistance": "100",
    "speed": "100",
    "disablereceiveshadows": "0",
    "disableshadows": "0",
    "renderamt": "255",
    "origin": "200 240 29.5",
    "rendermode": "0"
},
pub struct LightEnvironment
{
    "origin": "-529 -168 511",
    "_light": "255 255 198 300",
    "classname": "light_environment",
    "angles": "90 0 0",
    "_ambient": "255 255 255 20",
    "_lighthdr": "-1 -1 -1 1",
    "pitch": "-90",
    "_ambienthdr": "-1 -1 -1 1"
},
pub struct GamePlayerEquip
{
    "origin": "200 -680 224",
    "weapon_knife": "1",
    "classname": "game_player_equip"
},
pub struct TriggerHurt
{
    "damagecap": "1000",
    "model": "*76",
    "spawnflags": "1",
    "classname": "trigger_hurt",
    "damagetype": "0",
    "origin": "-1151 -378.5 -273.5",
    "damage": "1000",
    "startdisabled": "0",
    "damagemodel": "0"
},
pub struct FuncButton
{
    "classname": "func_button",
    "movedir": "0 0 0",
    "onpressed": "kill04,disable,,0.5,-1",
    "spawnflags": "1025",
    "unlocked_sentence": "0",
    "speed": "5",
    "wait": "3",
    "sounds": "0",
    "origin": "-1037.5 -412 104",
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
},
pub struct WeaponM4A1
{
    "spawnflags": "1",
    "classname": "weapon_m4a1",
    "angles": "0 0 0",
    "origin": "85 -757.356 221"
},
pub struct WaterLodControl
{
    "cheapwaterenddistance": "2000",
    "cheapwaterstartdistance": "1000",
    "classname": "water_lod_control"
},
pub struct FuncWall
{
    "renderamt": "100",
    "rendercolor": "255 255 255",
    "rendermode": "2",
    "disableshadows": "0",
    "classname": "func_wall",
    "disablereceiveshadows": "0",
    "model": "*1",
    "renderfx": "0"
},
pub struct EnvTonemapController
{
    "origin": "-344 -464 80",
    "classname": "env_tonemap_controller"
},
pub struct EnvSun
{
    "rendercolor": "237 232 216",
    "target": "sun",
    "hdrcolorscale": "1.0",
    "material": "sprites/light_glow02_add_noz",
    "size": "12",
    "origin": "896 320 576",
    "angles": "0 130 0",
    "overlaymaterial": "sprites/light_glow02_add_noz",
    "overlaycolor": "0 0 0",
    "classname": "env_sun",
    "overlaysize": "-1"
},
pub struct InfoTarget
{
    "spawnflags": "0",
    "origin": "-376 -128 80",
    "targetname": "sun",
    "angles": "0 0 0",
    "classname": "info_target"
},
pub struct WeaponAwp
{
    "origin": "-2128.89 -776 76.1248",
    "classname": "weapon_awp",
    "angles": "0 0 0",
    "targetname": "secretw1",
    "spawnflags": "1"
},
pub struct EnvSpritetrail
{
    "endwidth": "1.0",
    "lifetime": "1",
    "classname": "env_spritetrail",
    "parentname": "secretw1",
    "origin": "-2125.78 -776 94.0908",
    "rendercolor": "0 13 168",
    "renderamt": "255",
    "startwidth": "8.0",
    "spritename": "sprites/bluelaser1.vmt",
    "rendermode": "5"
},
pub struct WeaponDeagle
{
    "classname": "weapon_deagle",
    "origin": "-760.106 -167.704 107",
    "angles": "0 0 0",
    "ammo": "31"
},
pub struct ShadowControl
{
    "origin": "-16 168 64",
    "angles": "80 30 0",
    "classname": "shadow_control",
    "distance": "75",
    "color": "128 128 128",
    "hammerid": "2971"
},
pub struct WeaponElite
{
    "classname": "weapon_elite",
    "hammerid": "7248",
    "targetname": "gunmapby",
    "ammo": "31",
    "spawnflags": "1",
    "angles": "0 0 0",
    "origin": "-448 -2 80"
},
pub struct FuncRotating
{
    "spawnflags": "67",
    "volume": "0",
    "model": "*43",
    "solidbsp": "0",
    "dmg": "0",
    "disableshadows": "1",
    "renderamt": "255",
    "parentname": "gunmapby",
    "fanfriction": "0",
    "origin": "-448 0 54.5",
    "classname": "func_rotating",
    "rendercolor": "255 255 255",
    "disablereceiveshadows": "1",
    "renderfx": "0",
    "hammerid": "7383",
    "maxspeed": "50",
    "angles": "0 0 0",
    "rendermode": "0"
},
pub struct FuncBreakable
{
    "gibdir": "0 0 0",
    "rendercolor": "255 255 255",
    "performancemode": "0",
    "explodemagnitude": "0",
    "pressuredelay": "0",
    "origin": "-448 0 110",
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
    "classname": "func_breakable",
    "hammerid": "8344",
    "renderamt": "255",
    "physdamagescale": "1.0",
    "spawnflags": "0",
    "disableshadows": "1",
    "minhealthdmg": "0"
},
pub struct WeaponP90
{
    "ammo": "31",
    "origin": "-960 2240 1696",
    "spawnflags": "1",
    "classname": "weapon_p90",
    "targetname": "p90",
    "hammerid": "15611",
    "angles": "0 0 0"
},
pub struct WeaponGlock
{
    "classname": "weapon_glock",
    "targetname": "glock",
    "hammerid": "19772",
    "origin": "5244 0 84",
    "angles": "0 0 0",
    "spawnflags": "1",
    "ammo": "31"
},
pub struct LogicAuto
{
    "hammerid": "20253",
    "classname": "logic_auto",
    "origin": "5856 160 64",
    "spawnflags": "0",
    "onmapspawn": "d_red_3,open,,0,-1"
},
pub struct FuncWallToggle
{
    "rendercolor": "255 255 255",
    "renderamt": "255",
    "hammerid": "20341",
    "disablereceiveshadows": "1",
    "model": "*210",
    "disableshadows": "1",
    "spawnflags": "0",
    "rendermode": "0",
    "renderfx": "0",
    "classname": "func_wall_toggle",
    "targetname": "wall_to_glock"
},
pub struct WeaponM249
{
    "angles": "0 0 0",
    "targetname": "m249",
    "origin": "6488 1992 -344",
    "hammerid": "24744",
    "spawnflags": "1",
    "ammo": "31",
    "classname": "weapon_m249"
},
pub struct WeaponHegrenade
{
    "origin": "4768 1664 148",
    "hammerid": "28302",
    "targetname": "nade",
    "angles": "0 0 0",
    "classname": "weapon_hegrenade",
    "spawnflags": "1"
},
pub struct PointViewcontrol
{
    "wait": "10",
    "classname": "point_viewcontrol",
    "angles": "0 0 0",
    "spawnflags": "1",
    "acceleration": "500",
    "target": "cam1",
    "deceleration": "500",
    "origin": "6456 1992 -240",
    "hammerid": "36815"
},
pub struct WeaponScout
{
    "spawnflags": "0",
    "targetname": "scout1",
    "origin": "808 160 96",
    "classname": "weapon_scout",
    "angles": "0 270 0"
},
pub struct PointTemplate
{
    "template01": "scout1",
    "spawnflags": "2",
    "classname": "point_template",
    "targetname": "template1",
    "origin": "800 160 128"
},
pub struct FuncDoorRotating
{
    "unlocked_sentence": "0",
    "renderamt": "255",
    "rendermode": "0",
    "classname": "func_door_rotating",
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
    "origin": "64 15228 2172",
    "lip": "0",
    "speed": "60",
    "disablereceiveshadows": "0",
    "loopmovesound": "0",
    "angles": "0 0 0",
    "wait": "3",
    "distance": "135",
    "disableshadows": "0"
},
pub struct TriggerPush
{
    "alternateticksfix": "0",
    "speed": "1000",
    "model": "*456",
    "origin": "224 15424 2272",
    "spawnflags": "1",
    "pushdir": "0 90 0",
    "startdisabled": "0",
    "classname": "trigger_push"
},
pub struct FuncPhysbox
{
    "preferredcarryangles": "0 0 0",
    "spawnobject": "0",
    "disableshadows": "0",
    "spawnflags": "0",
    "explodedamage": "0",
    "performancemode": "0",
    "gibdir": "0 0 0",
    "classname": "func_physbox",
    "propdata": "0",
    "damagetoenablemotion": "0",
    "disablereceiveshadows": "0",
    "renderfx": "0",
    "massscale": "0",
    "nodamageforces": "0",
    "origin": "9026 4015 172",
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
},
pub struct WeaponSmokegrenade
{
    "classname": "weapon_smokegrenade",
    "origin": "7760 4256 144",
    "angles": "0 0 0",
    "spawnflags": "1",
    "targetname": "smoke1"
},
pub struct FilterDamageType
{
    "origin": "-719.625 -413.753 1793",
    "classname": "filter_damage_type",
    "negated": "1",
    "targetname": "falldmg",
    "damagetype": "32"
},
pub struct EnvFogController
{
    "fogend": "4000",
    "fogcolor": "0 0 0",
    "origin": "1820.26 -346.925 1025",
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
    "angles": "0 0 0",
    "fogmaxdensity": "1",
    "classname": "env_fog_controller"
},
pub struct FuncConveyor
{
    "spawnflags": "0",
    "classname": "func_conveyor",
    "disablereceiveshadows": "0",
    "rendercolor": "255 255 255",
    "rendermode": "0",
    "angles": "339 107 0",
    "renderfx": "0",
    "disableshadows": "0",
    "speed": "300",
    "renderamt": "255",
    "model": "*60",
    "movedir": "0 45 0"
},
pub struct FilterActivatorName
{
    "origin": "-98.1297 -512 1751.28",
    "negated": "allow entities that match criteria",
    "classname": "filter_activator_name",
    "targetname": "filter_activator",
    "filtername": "activator"
},
pub struct TriggerMultiple
{
    "model": "*86",
    "startdisabled": "0",
    "wait": "0.01",
    "spawnflags": "1",
    "ontrigger": "!activator,addoutput,targetname default,0.13,-1",
    "origin": "512 512 -528",
    "classname": "trigger_multiple"
},
pub struct FilterMulti
{
    "targetname": "1multi",
    "filter02": "filt_2",
    "origin": "82 158 142",
    "negated": "0",
    "classname": "filter_multi",
    "filter01": "filt_1",
    "hammerid": "49342",
    "filtertype": "1"
},
pub struct PropPhysicsOverride
{
    "origin": "156 280 160.679",
    "spawnflags": "524",
    "skin": "0",
    "nodamageforces": "0",
    "performancemode": "0",
    "minhealthdmg": "0",
    "fadescale": "1",
    "exploderadius": "0",
    "explodedamage": "0",
    "damagetype": "0",
    "angles": "0 0 0",
    "classname": "prop_physics_override",
    "hammerid": "89676",
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
},
pub struct PointServercommand
{
    "classname": "point_servercommand",
    "origin": "60 210 142",
    "targetname": "servcommand",
    "hammerid": "90371"
},
pub struct PointClientcommand
{
    "targetname": "clientcommand",
    "classname": "point_clientcommand",
    "hammerid": "90377",
    "origin": "38 210 142"
},
pub struct GameWeaponManager
{
    "weaponname": "weapon_mp5navy",
    "maxpieces": "0",
    "origin": "48 544 136",
    "ammomod": "1",
    "classname": "game_weapon_manager",
    "hammerid": "90603"
},
pub struct FilterActivatorClass
{
    "origin": "-56 210 142",
    "targetname": "hegrenade",
    "negated": "allow entities that match criteria",
    "filterclass": "hegrenade_projectile",
    "hammerid": "91384",
    "classname": "filter_activator_class"
},
pub struct PlayerSpeedmod
{
    "hammerid": "91426",
    "origin": "82 236 142",
    "spawnflags": "0",
    "classname": "player_speedmod",
    "targetname": "speed"
},
pub struct EnvFire
{
    "spawnflags": "31",
    "classname": "env_fire",
    "startdisabled": "0",
    "ignitionpoint": "32",
    "health": "1",
    "origin": "-688 -4200 866",
    "damagescale": "1.0",
    "hammerid": "101536",
    "firesize": "64",
    "firetype": "0",
    "fireattack": "4"
},
pub struct PlayerWeaponstrip
{
    "targetname": "strip",
    "hammerid": "107702",
    "origin": "16 236 142",
    "classname": "player_weaponstrip"
},
pub struct FuncBuyzone
{
    "classname": "func_buyzone",
    "teamnum": "2",
    "model": "*542",
    "hammerid": "113521"
},
pub struct EnvSoundscape
{
    "startdisabled": "0",
    "hammerid": "133638",
    "origin": "-256 2048 224",
    "radius": "848",
    "classname": "env_soundscape",
    "soundscape": "lego1"
},
pub struct EnvSprite
{
    "glowproxysize": "2.0",
    "rendermode": "0",
    "mindxlevel": "0",
    "framerate": "10.0",
    "hammerid": "134683",
    "maxdxlevel": "0",
    "hdrcolorscale": "1.0",
    "origin": "192 176 176",
    "renderamt": "255",
    "disablereceiveshadows": "0",
    "model": "lego/longjumps.vmt",
    "spawnflags": "1",
    "renderfx": "0",
    "rendercolor": "255 255 255",
    "classname": "env_sprite"
},
pub struct MathCounter
{
    "startdisabled": "0",
    "targetname": "counter",
    "max": "1",
    "origin": "82 258 142",
    "onhitmax": "!self,disable,,0,1",
    "hammerid": "134898",
    "classname": "math_counter",
    "startvalue": "0",
    "min": "0"
},
pub struct WeaponKnife
{
    "classname": "weapon_knife",
    "angles": "0 0 0",
    "hammerid": "176620",
    "spawnflags": "1",
    "targetname": "noobknife",
    "origin": "-80 592 192"
},
pub struct EnvEntityMaker
{
    "entitytemplate": "noobtemplate",
    "angles": "0 0 0",
    "hammerid": "176738",
    "spawnflags": "0",
    "classname": "env_entity_maker",
    "postspawndirection": "0 0 0",
    "origin": "320 1008 263.068",
    "targetname": "noobmaker",
    "postspawninheritangles": "0",
    "postspawndirectionvariance": "0.15",
    "postspawnspeed": "0"
},
pub struct EnvSoundscapeTriggerable
{
    "soundscape": "lego2",
    "radius": "128",
    "hammerid": "179655",
    "startdisabled": "0",
    "targetname": "first",
    "classname": "env_soundscape_triggerable",
    "origin": "-48 -200 208"
},
pub struct EnvHudhint
{
    "origin": "82 280 142",
    "classname": "env_hudhint",
    "spawnflags": "0",
    "hammerid": "183578",
    "targetname": "quickrestart",
    "message": "press e to restart the map"
},
pub struct PropDynamic
{
    "solid": "0",
    "mindxlevel": "0",
    "classname": "prop_dynamic",
    "rendercolor": "255 255 255",
    "disableshadows": "0",
    "hammerid": "190009",
    "randomanimation": "0",
    "explodedamage": "0",
    "angles": "0 90 0",
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
    "origin": "2312 -9464 1010.54",
    "minanimtime": "5",
    "maxdxlevel": "0",
    "startdisabled": "0"
},
pub struct Infodecal
{
    "texture": "decals/decalgraffiti044a",
    "hammerid": "3739",
    "classname": "infodecal",
    "angles": "0 0 0",
    "origin": "-15520 -12604.2 411.861"
},
pub struct TriggerSoundscape
{
    "startdisabled": "0",
    "soundscape": "nuke.abomb",
    "model": "*407",
    "origin": "-15528 -11616 96",
    "spawnflags": "1",
    "classname": "trigger_soundscape",
    "hammerid": "7740"
},
pub struct EnvSpark
{
    "spawnflags": "0",
    "classname": "env_spark",
    "targetname": "spark1",
    "maxdelay": "0",
    "magnitude": "1",
    "angles": "0 0 0",
    "hammerid": "34153",
    "origin": "-15534 -12064 153",
    "traillength": "1"
},
pub struct LogicTimer
{
    "spawnflags": "0",
    "userandomtime": "1",
    "lowerrandombound": "12",
    "startdisabled": "0",
    "hammerid": "34188",
    "classname": "logic_timer",
    "ontimer": "spark1,sparkonce,,0,-1",
    "upperrandombound": "20",
    "origin": "-15509 -12044 144.947"
},
pub struct PropPhysicsMultiplayer
{
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
    "angles": "0 0 0",
    "performancemode": "0",
    "mindxlevel": "0",
    "renderamt": "255",
    "hammerid": "35774",
    "skin": "0",
    "rendermode": "0",
    "minhealthdmg": "0",
    "maxdxlevel": "0",
    "forcetoenablemotion": "0",
    "physdamagescale": "0.1",
    "exploderadius": "0",
    "damagetype": "0",
    "model": "models/props_c17/oildrum001.mdl",
    "origin": "-13984 -12248 545",
    "pressuredelay": "0",
    "nodamageforces": "0",
    "fademindist": "1000",
    "damagetoenablemotion": "0",
    "classname": "prop_physics_multiplayer"
},
pub struct EnvWind
{
    "origin": "-14656 -11816 368",
    "maxgustdelay": "20",
    "mingustdelay": "10",
    "maxgust": "250",
    "minwind": "20",
    "gustduration": "5",
    "mingust": "100",
    "angles": "-57 305 0",
    "hammerid": "38005",
    "maxwind": "50",
    "gustdirchange": "20",
    "classname": "env_wind"
},
pub struct EnvDetailController
{
    "classname": "env_detail_controller",
    "hammerid": "207110",
    "origin": "-14656 -11768 352",
    "fademindist": "200",
    "fademaxdist": "350",
    "angles": "0 0 0"
},
pub struct InfoPlayerStart
{
    "angles": "0 0 0",
    "origin": "-15648 -11672 40",
    "classname": "info_player_start",
    "hammerid": "259724",
    "spawnflags": "0"
},
pub struct PropPhysics
{
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
    "origin": "-15461 -12749 552",
    "hammerid": "487103",
    "damagetype": "0",
    "fadescale": "1",
    "rendermode": "0",
    "classname": "prop_physics",
    "renderamt": "255",
    "skin": "0",
    "model": "models/props_debris/concrete_cynderblock001.mdl",
    "nodamageforces": "1",
    "angles": "0 330 0",
    "fademaxdist": "2100",
    "rendercolor": "255 255 255",
    "maxdxlevel": "0"
},
pub struct WeaponAk47
{
    "classname": "weapon_ak47",
    "angles": "0 180 0",
    "targetname": "ak1_2",
    "ammo": "1337",
    "origin": "-12927.6 9798.86 46.2675",
    "spawnflags": "1",
    "hammerid": "155595"
},
pub struct FuncMovelinear
{
    "disablereceiveshadows": "0",
    "renderfx": "0",
    "origin": "-12649.3 9902 90.26",
    "rendermode": "3",
    "parentname": "ak2_1",
    "rendercolor": "255 255 255",
    "blockdamage": "0",
    "classname": "func_movelinear",
    "model": "*616",
    "movedir": "0 0 0",
    "speed": "100",
    "hammerid": "155597",
    "renderamt": "255",
    "spawnflags": "8",
    "startposition": "0",
    "movedistance": "100"
},
pub struct FuncPrecipitation
{
    "hammerid": "288365",
    "renderamt": "100",
    "preciptype": "3",
    "classname": "func_precipitation",
    "rendercolor": "100 100 100",
    "model": "*1"
},
pub struct EnvLightglow
{
    "hdrcolorscale": "0.5",
    "origin": "-1120 -1120 2432",
    "classname": "env_lightglow",
    "angles": "0 0 0",
    "horizontalglowsize": "8",
    "verticalglowsize": "8",
    "maxdist": "256",
    "hammerid": "156600",
    "outermaxdist": "0",
    "spawnflags": "0",
    "glowproxysize": "0",
    "mindist": "64",
    "rendercolor": "255 255 255"
},
pub struct EnvSmokestack
{
    "rendercolor": "255 255 255",
    "roll": "15.0",
    "windangle": "0",
    "twist": "2",
    "startsize": "20",
    "renderamt": "160",
    "basespread": "125",
    "angles": "0 0 0",
    "initialstate": "1",
    "jetlength": "200",
    "endsize": "30",
    "windspeed": "4",
    "smokematerial": "particle/smokestack.vmt",
    "classname": "env_smokestack",
    "origin": "-6984 -2192 2224",
    "rate": "20",
    "spreadspeed": "15",
    "hammerid": "129270",
    "speed": "30"
},
pub struct FuncSmokevolume
{
    "color2": "192 192 192",
    "particledrawwidth": "192",
    "classname": "func_smokevolume",
    "hammerid": "31779",
    "particlespacingdistance": "80",
    "color1": "192 192 192",
    "spawnflags": "0",
    "rotationspeed": "10",
    "movementspeed": "10",
    "material": "particle/particle_noisesphere",
    "densityrampspeed": "1",
    "density": "0.5",
    "model": "*73"
},
pub struct EnvEmbers
{
    "angles": "0 90 0",
    "classname": "env_embers",
    "spawnflags": "1",
    "hammerid": "363584",
    "model": "*356",
    "speed": "150.0",
    "lifetime": "8",
    "rendercolor": "255 128 0",
    "density": "32"
},
pub struct FuncDustcloud
{
    "frozen": "0",
    "sizemax": "150",
    "startdisabled": "0",
    "hammerid": "370009",
    "classname": "func_dustcloud",
    "lifetimemin": "3",
    "lifetimemax": "5",
    "color": "128 128 128",
    "distmax": "1024",
    "alpha": "96",
    "model": "*367",
    "speedmax": "6",
    "sizemin": "50",
    "spawnrate": "500"
},
pub struct TriggerOnce
{
    "origin": "-2048 -6656 -6648",
    "model": "*398",
    "hammerid": "387965",
    "angles": "0 0 0",
    "classname": "trigger_once",
    "startdisabled": "0",
    "ontrigger": "servercommand,command,say level 5 unlocked,0,-1",
    "spawnflags": "1"
},
pub struct PointSpotlight
{
    "hdrcolorscale": "0.5",
    "angles": "-90 0 0",
    "spotlightlength": "200.0",
    "spotlightwidth": "20.0",
    "renderamt": "255",
    "targetname": "scout_spotlight",
    "spawnflags": "3",
    "classname": "point_spotlight",
    "origin": "8192 -12288 13568",
    "rendercolor": "255 255 255",
    "hammerid": "393222"
},
pub struct WeaponTmp
{
    "maxdxlevel": "0",
    "angles": "0 270 0",
    "fadescale": "1.0",
    "renderfx": "0",
    "shadowcastdist": "0",
    "hammerid": "402780",
    "fademindist": "-1.0",
    "mindxlevel": "0",
    "rendermode": "0",
    "classname": "weapon_tmp",
    "renderamt": "255",
    "rendercolor": "255 255 255",
    "fademaxdist": "0.0",
    "origin": "992 8968 12064",
    "spawnflags": "1",
    "nodamageforces": "0",
    "ammo": "120"
},
pub struct WeaponXm1014
{
    "renderfx": "0",
    "rendercolor": "255 255 255",
    "hammerid": "402832",
    "spawnflags": "1",
    "mindxlevel": "0",
    "ammo": "32",
    "classname": "weapon_xm1014",
    "nodamageforces": "0",
    "renderamt": "255",
    "origin": "-1536 5192 12528",
    "fademaxdist": "0.0",
    "rendermode": "0",
    "maxdxlevel": "0",
    "fadescale": "1.0",
    "shadowcastdist": "0",
    "fademindist": "-1.0",
    "angles": "0 270 0"
},
pub struct WeaponMac10
{
    "spawnflags": "1",
    "rendermode": "0",
    "fademindist": "-1.0",
    "ammo": "100",
    "angles": "0 0 0",
    "shadowcastdist": "0",
    "rendercolor": "255 255 255",
    "fademaxdist": "0.0",
    "hammerid": "403143",
    "origin": "-3080 -4608 9248",
    "renderamt": "255",
    "renderfx": "0",
    "nodamageforces": "0",
    "classname": "weapon_mac10",
    "mindxlevel": "0",
    "fadescale": "1.0",
    "maxdxlevel": "0"
},
pub struct WeaponUmp45
{
    "rendermode": "0",
    "spawnflags": "1",
    "shadowcastdist": "0",
    "renderfx": "0",
    "fademaxdist": "0.0",
    "hammerid": "403301",
    "ammo": "100",
    "classname": "weapon_ump45",
    "mindxlevel": "0",
    "renderamt": "255",
    "origin": "3920 6304 72",
    "nodamageforces": "0",
    "fademindist": "-1.0",
    "angles": "0 270 0",
    "maxdxlevel": "0",
    "fadescale": "1.0",
    "rendercolor": "255 255 255"
},
pub struct WeaponFamas
{
    "hammerid": "403360",
    "shadowcastdist": "0",
    "classname": "weapon_famas",
    "renderfx": "0",
    "rendermode": "0",
    "nodamageforces": "0",
    "origin": "448 -952 -88",
    "fademaxdist": "0.0",
    "maxdxlevel": "0",
    "renderamt": "255",
    "mindxlevel": "0",
    "angles": "0 270 0",
    "fadescale": "1.0",
    "spawnflags": "1",
    "rendercolor": "255 255 255",
    "fademindist": "-1.0",
    "ammo": "90"
},
pub struct WeaponG3Sg1
{
    "angles": "0 180 0",
    "nodamageforces": "0",
    "rendermode": "0",
    "renderfx": "0",
    "fademaxdist": "0.0",
    "spawnflags": "1",
    "fadescale": "1.0",
    "ammo": "90",
    "hammerid": "403518",
    "renderamt": "255",
    "fademindist": "-1.0",
    "origin": "-808 -5232 -4984",
    "mindxlevel": "0",
    "rendercolor": "255 255 255",
    "shadowcastdist": "0",
    "maxdxlevel": "0",
    "classname": "weapon_g3sg1"
},
pub struct WeaponSg550
{
    "renderamt": "255",
    "hammerid": "403621",
    "origin": "-14040 -9200 -7096",
    "maxdxlevel": "0",
    "fadescale": "1.0",
    "renderfx": "0",
    "rendermode": "0",
    "spawnflags": "1",
    "nodamageforces": "0",
    "fademaxdist": "0.0",
    "rendercolor": "255 255 255",
    "mindxlevel": "0",
    "classname": "weapon_sg550",
    "ammo": "90",
    "angles": "0 270 0",
    "fademindist": "-1.0",
    "shadowcastdist": "0"
},
pub struct WeaponFlashbang
{
    "origin": "11136 -11648 13440",
    "fademindist": "-1.0",
    "spawnflags": "0",
    "hammerid": "416160",
    "renderamt": "255",
    "classname": "weapon_flashbang",
    "fadescale": "1.0",
    "fademaxdist": "0.0",
    "angles": "0 270 0",
    "targetname": "flashbang",
    "rendercolor": "255 255 255"
},
pub struct EnvFireTrail
{
    "targetname": "uber_scout_fire",
    "hammerid": "425179",
    "classname": "env_fire_trail",
    "origin": "-2816 -7680 15616",
    "parentname": "uber_scout"
},
pub struct InfoLadder
{
    "maxs.y": "3528.00",
    "mins.z": "112.00",
    "hammerid": "2854",
    "classname": "info_ladder",
    "maxs.x": "-9792.00",
    "mins.y": "3520.00",
    "maxs.z": "640.00",
    "mins.x": "-9856.00"
},
pub struct WeaponM3
{
    "ammo": "999",
    "hammerid": "176561",
    "classname": "weapon_m3",
    "angles": "0 0 0",
    "spawnflags": "1",
    "targetname": "gun",
    "origin": "-5408 8800 1390.15"
},
pub struct WeaponFiveseven
{
    "origin": "-11616 -96 -13841.8",
    "ammo": "999",
    "classname": "weapon_fiveseven",
    "hammerid": "177007",
    "targetname": "gun6",
    "spawnflags": "1",
    "angles": "0 0 0"
},
pub struct MoveRope
{
    "spawnflags": "0",
    "barbed": "0",
    "slack": "105",
    "nowind": "1",
    "origin": "-8952 2328 840",
    "subdiv": "4",
    "movespeed": "64",
    "width": "1",
    "positioninterpolator": "2",
    "mindxlevel": "0",
    "ropematerial": "cable/cable",
    "texturescale": "1",
    "collide": "0",
    "hammerid": "129625",
    "angles": "0 0 0",
    "breakable": "0",
    "type": "0",
    "maxdxlevel": "0",
    "dangling": "0",
    "targetname": "rope_20",
    "classname": "move_rope"
},
pub struct EnvFade
{
    "spawnflags": "4",
    "classname": "env_fade",
    "rendercolor": "0 0 0",
    "origin": "5651.92 -8211.48 1000.12",
    "hammerid": "262982",
    "duration": "3",
    "holdtime": "0.0",
    "renderamt": "255",
    "targetname": "fadelj1"
},
pub struct FuncAreaportalwindow
{
    "translucencylimit": "0.0",
    "fadestartdist": "1950",
    "classname": "func_areaportalwindow",
    "fadedist": "2000",
    "portalnumber": "1",
    "target": "brush_a",
    "portalversion": "1",
    "hammerid": "270454"
},
pub struct EnvLaser
{
    "texturescroll": "35",
    "dissolvetype": "none",
    "lasertarget": "las_target2",
    "width": "4",
    "rendercolor": "0 128 255",
    "renderamt": "100",
    "texture": "sprites/laserbeam.spr",
    "damage": "0",
    "classname": "env_laser",
    "hammerid": "731916",
    "origin": "8656 -8360 4468",
    "spawnflags": "1"
},
pub struct LogicRelay
{
    "origin": "-1680 -1586.07 -52",
    "hammerid": "805602",
    "ontrigger": "snek_btn_1,kill,,0,-1",
    "targetname": "snake_relay1",
    "classname": "logic_relay",
    "spawnflags": "1"
},
pub struct PathTrack
{
    "orientationtype": "1",
    "angles": "0 0 0",
    "spawnflags": "0",
    "target": "vr_track1_2",
    "hammerid": "30225",
    "targetname": "vr_track1_1",
    "classname": "path_track",
    "origin": "-11920 -8164 132"
},
pub struct FuncTracktrain
{
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
    "classname": "func_tracktrain",
    "movesoundminpitch": "60",
    "hammerid": "30270",
    "rendercolor": "255 255 255",
    "volume": "10",
    "wheels": "50",
    "bank": "0",
    "model": "*105",
    "spawnflags": "538",
    "movesoundmaxtime": "0",
    "disableshadows": "1",
    "origin": "-11920 -8164 144",
    "renderfx": "0",
    "startspeed": "100"
},
pub struct EnvSteam
{
    "rate": "26",
    "spawnflags": "0",
    "speed": "120",
    "startsize": "10",
    "renderamt": "255",
    "endsize": "25",
    "targetname": "cave_steam_dream01",
    "rollspeed": "8",
    "hammerid": "65082",
    "angles": "0 180 0",
    "origin": "6176 -7335.32 5440",
    "rendercolor": "255 255 255",
    "spreadspeed": "15",
    "classname": "env_steam",
    "jetlength": "80"
},
pub struct EnvShake
{
    "amplitude": "6",
    "classname": "env_shake",
    "targetname": "cave_shake01",
    "origin": "5840 -5280 5170.94",
    "duration": "4",
    "spawnflags": "0",
    "frequency": "2.5",
    "radius": "375",
    "hammerid": "105159"
},
pub struct FuncRotButton
{
    "distance": "90",
    "sounds": "25",
    "angles": "0 0 0",
    "origin": "6382 7480 144",
    "onpressed": "water_movelin01,close,,30,-1",
    "wait": "3",
    "startdisabled": "0",
    "targetname": "rot_button01",
    "speed": "90",
    "hammerid": "239116",
    "health": "0",
    "spawnflags": "1088",
    "model": "*496",
    "classname": "func_rot_button"
},
pub struct TriggerGravity
{
    "hammerid": "259593",
    "origin": "8632 4504 -280",
    "spawnflags": "1",
    "model": "*562",
    "startdisabled": "0",
    "gravity": ".05",
    "classname": "trigger_gravity"
},
pub struct SkyCamera
{
    "fogdir": "1 0 0",
    "scale": "16",
    "fogstart": "500.0",
    "fogend": "2000.0",
    "fogcolor2": "255 255 255",
    "classname": "sky_camera",
    "hammerid": "361080",
    "angles": "0 0 0",
    "fogcolor": "255 255 255",
    "origin": "-8969 6066 388"
},
pub struct GameUi
{
    "fieldofview": "-1.0",
    "origin": "272 5584 934.249",
    "pressedforward": "push_forward,enable,,0,-1",
    "playeroff": "button_lvl1,addoutput,renderamt 200,0,-1",
    "unpressedforward": "push_forward,disable,,0,-1",
    "spawnflags": "480",
    "hammerid": "435027",
    "unpressedmoveright": "push_right,disable,,0,-1",
    "unpressedmoveleft": "push_left,disable,,0,-1",
    "unpressedback": "push_backward,disable,,0,-1",
    "pressedmoveright": "push_right,enable,,0,-1",
    "pressedback": "push_backward,enable,,0,-1",
    "pressedmoveleft": "push_left,enable,,0,-1",
    "classname": "game_ui",
    "targetname": "ui"
},
pub struct FuncPhysboxMultiplayer
{
    "targetname": "box_lvl1",
    "health": "0",
    "disablereceiveshadows": "1",
    "classname": "func_physbox_multiplayer",
    "pressuredelay": "0",
    "damagetype": "0",
    "material": "10",
    "preferredcarryangles": "0 0 0",
    "origin": "0 6064 410.25",
    "exploderadius": "0",
    "propdata": "0",
    "model": "*614",
    "rendermode": "0",
    "notsolid": "0",
    "explodemagnitude": "0",
    "explosion": "0",
    "gibdir": "0 0 0",
    "renderamt": "255",
    "hammerid": "435046",
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
},
pub struct TriggerLook
{
    "startdisabled": "0",
    "looktime": "5",
    "hammerid": "491215",
    "target": "target_look",
    "spawnflags": "128",
    "model": "*748",
    "timeout": "0",
    "fieldofview": "0.9",
    "classname": "trigger_look",
    "ontrigger": "door_pic,open,,0,-1",
    "origin": "9824 -16 8168"
},
pub struct KeyframeRope
{
    "origin": "5780 -2060 5304",
    "slack": "25",
    "texturescale": "1",
    "ropematerial": "cable/cable.vmt",
    "movespeed": "64",
    "angles": "0 90 0",
    "hammerid": "543370",
    "classname": "keyframe_rope",
    "width": "2",
    "targetname": "cave_rope02",
    "subdiv": "2"
},
pub struct EnvShooter
{
    "m_flvariance": "0.3",
    "angles": "-90 270 0",
    "renderfx": "0",
    "rendermode": "0",
    "m_flvelocity": "772",
    "origin": "13504 -1352 8252",
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
    "classname": "env_shooter",
    "shootsounds": "-1",
    "hammerid": "564233"
},
pub struct WeaponUsp
{
    "hammerid": "597821",
    "origin": "11584 1860 8219",
    "spawnflags": "1",
    "targetname": "spusp",
    "angles": "0 270 0",
    "ammo": "99",
    "classname": "weapon_usp"
},
pub struct PropRagdoll
{
    "fadescale": "1",
    "modelscale": "1.0",
    "origin": "352 -9824 368",
    "model": "models/humans/group03/male_06_bloody.mdl",
    "fademindist": "-1",
    "skin": "0",
    "spawnflags": "4",
    "angles": "0 300 0",
    "classname": "prop_ragdoll",
    "hammerid": "739080",
    "targetname": "ragdoll01"
},
pub struct PhysBallsocket
{
    "classname": "phys_ballsocket",
    "targetname": "ballsocket",
    "origin": "355.96 -9816 488",
    "attach1": "ragdoll01",
    "hammerid": "739103"
},
pub struct EnvBeam
{
    "origin": "5816 -8847.58 4030.12",
    "lightningend": "antlion1",
    "texturescroll": "35",
    "hdrcolorscale": "1.0",
    "hammerid": "774042",
    "spawnflags": "48",
    "life": ".5",
    "decalname": "bigshot",
    "renderamt": "200",
    "boltwidth": "6",
    "noiseamplitude": "12",
    "classname": "env_beam",
    "striketime": "1",
    "texture": "sprites/laserbeam.spr",
    "lightningstart": "start_tesla1",
    "radius": "256",
    "rendercolor": "122 230 252",
    "targetname": "tesla_beam1"
},
pub struct Light
{
    "origin": "9536 -304.963 7948",
    "_lightscalehdr": "1",
    "_quadratic_attn": "1",
    "_lighthdr": "-1 -1 -1 1",
    "_light": "255 128 64",
    "classname": "light",
    "hammerid": "823486"
}
