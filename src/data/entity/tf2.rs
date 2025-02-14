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
    #[serde(rename = "base_boss")]
    #[serde(borrow)]
    BaseBoss(BaseBoss<'a>),
    #[serde(rename = "bot_action_point")]
    #[serde(borrow)]
    BotActionPoint(BotActionPoint<'a>),
    #[serde(rename = "bot_generator")]
    #[serde(borrow)]
    BotGenerator(BotGenerator<'a>),
    #[serde(rename = "bot_hint_engineer_nest")]
    #[serde(borrow)]
    BotHintEngineerNest(BotHintEngineerNest<'a>),
    #[serde(rename = "bot_hint_sentrygun")]
    #[serde(borrow)]
    BotHintSentrygun(BotHintSentrygun<'a>),
    #[serde(rename = "bot_hint_sniper_spot")]
    #[serde(borrow)]
    BotHintSniperSpot(BotHintSniperSpot<'a>),
    #[serde(rename = "bot_hint_teleporter_exit")]
    #[serde(borrow)]
    BotHintTeleporterExit(BotHintTeleporterExit<'a>),
    #[serde(rename = "bot_roster")]
    #[serde(borrow)]
    BotRoster(BotRoster<'a>),
    #[serde(rename = "color_correction")]
    #[serde(borrow)]
    ColorCorrection(ColorCorrection<'a>),
    #[serde(rename = "dispenser_touch_trigger")]
    #[serde(borrow)]
    DispenserTouchTrigger(DispenserTouchTrigger<'a>),
    #[serde(rename = "editor_text")]
    #[serde(borrow)]
    EditorText(EditorText<'a>),
    #[serde(rename = "entity_spawn_manager")]
    #[serde(borrow)]
    EntitySpawnManager(EntitySpawnManager<'a>),
    #[serde(rename = "entity_spawn_point")]
    #[serde(borrow)]
    EntitySpawnPoint(EntitySpawnPoint<'a>),
    #[serde(rename = "env_beam")]
    #[serde(borrow)]
    EnvBeam(EnvBeam<'a>),
    #[serde(rename = "env_bubbles")]
    #[serde(borrow)]
    EnvBubbles(EnvBubbles<'a>),
    #[serde(rename = "env_entity_maker")]
    #[serde(borrow)]
    EnvEntityMaker(EnvEntityMaker<'a>),
    #[serde(rename = "env_explosion")]
    #[serde(borrow)]
    EnvExplosion(EnvExplosion<'a>),
    #[serde(rename = "env_fade")]
    #[serde(borrow)]
    EnvFade(EnvFade<'a>),
    #[serde(rename = "env_fog_controller")]
    #[serde(borrow)]
    EnvFogController(EnvFogController<'a>),
    #[serde(rename = "env_glow")]
    #[serde(borrow)]
    EnvGlow(EnvGlow<'a>),
    #[serde(rename = "env_laser")]
    #[serde(borrow)]
    EnvLaser(EnvLaser<'a>),
    #[serde(rename = "env_lightglow")]
    #[serde(borrow)]
    EnvLightglow(EnvLightglow<'a>),
    #[serde(rename = "env_physexplosion")]
    #[serde(borrow)]
    EnvPhysexplosion(EnvPhysexplosion<'a>),
    #[serde(rename = "env_screenoverlay")]
    #[serde(borrow)]
    EnvScreenoverlay(EnvScreenoverlay<'a>),
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
    #[serde(rename = "env_soundscape_proxy")]
    #[serde(borrow)]
    EnvSoundscapeProxy(EnvSoundscapeProxy<'a>),
    #[serde(rename = "env_soundscape_triggerable")]
    #[serde(borrow)]
    EnvSoundscapeTriggerable(EnvSoundscapeTriggerable<'a>),
    #[serde(rename = "env_spark")]
    #[serde(borrow)]
    EnvSpark(EnvSpark<'a>),
    #[serde(rename = "env_sprite")]
    #[serde(borrow)]
    EnvSprite(EnvSprite<'a>),
    #[serde(rename = "env_steam")]
    #[serde(borrow)]
    EnvSteam(EnvSteam<'a>),
    #[serde(rename = "env_sun")]
    #[serde(borrow)]
    EnvSun(EnvSun<'a>),
    #[serde(rename = "env_texturetoggle")]
    #[serde(borrow)]
    EnvTexturetoggle(EnvTexturetoggle<'a>),
    #[serde(rename = "env_tonemap_controller")]
    #[serde(borrow)]
    EnvTonemapController(EnvTonemapController<'a>),
    #[serde(rename = "env_wind")]
    #[serde(borrow)]
    EnvWind(EnvWind<'a>),
    #[serde(rename = "eyeball_boss")]
    #[serde(borrow)]
    EyeballBoss(EyeballBoss<'a>),
    #[serde(rename = "filter_activator_class")]
    #[serde(borrow)]
    FilterActivatorClass(FilterActivatorClass<'a>),
    #[serde(rename = "filter_activator_name")]
    #[serde(borrow)]
    FilterActivatorName(FilterActivatorName<'a>),
    #[serde(rename = "filter_activator_team")]
    #[serde(borrow)]
    FilterActivatorTeam(FilterActivatorTeam<'a>),
    #[serde(rename = "filter_activator_tfteam")]
    #[serde(borrow)]
    FilterActivatorTfteam(FilterActivatorTfteam<'a>),
    #[serde(rename = "filter_base")]
    #[serde(borrow)]
    FilterBase(FilterBase<'a>),
    #[serde(rename = "filter_damage_type")]
    #[serde(borrow)]
    FilterDamageType(FilterDamageType<'a>),
    #[serde(rename = "filter_multi")]
    #[serde(borrow)]
    FilterMulti(FilterMulti<'a>),
    #[serde(rename = "filter_tf_bot_has_tag")]
    #[serde(borrow)]
    FilterTfBotHasTag(FilterTfBotHasTag<'a>),
    #[serde(rename = "filter_tf_class")]
    #[serde(borrow)]
    FilterTfClass(FilterTfClass<'a>),
    #[serde(rename = "filter_tf_condition")]
    #[serde(borrow)]
    FilterTfCondition(FilterTfCondition<'a>),
    #[serde(rename = "filter_tf_damaged_by_weapon_in_slot")]
    #[serde(borrow)]
    FilterTfDamagedByWeaponInSlot(FilterTfDamagedByWeaponInSlot<'a>),
    #[serde(rename = "func_achievement")]
    #[serde(borrow)]
    FuncAchievement(FuncAchievement<'a>),
    #[serde(rename = "func_areaportal")]
    #[serde(borrow)]
    FuncAreaportal(FuncAreaportal<'a>),
    #[serde(rename = "func_areaportalwindow")]
    #[serde(borrow)]
    FuncAreaportalwindow(FuncAreaportalwindow<'a>),
    #[serde(rename = "func_breakable")]
    #[serde(borrow)]
    FuncBreakable(FuncBreakable<'a>),
    #[serde(rename = "func_brush")]
    #[serde(borrow)]
    FuncBrush(FuncBrush<'a>),
    #[serde(rename = "func_button")]
    #[serde(borrow)]
    FuncButton(FuncButton<'a>),
    #[serde(rename = "func_capturezone")]
    #[serde(borrow)]
    FuncCapturezone(FuncCapturezone<'a>),
    #[serde(rename = "func_clip_vphysics")]
    #[serde(borrow)]
    FuncClipVphysics(FuncClipVphysics<'a>),
    #[serde(rename = "func_croc")]
    #[serde(borrow)]
    FuncCroc(FuncCroc<'a>),
    #[serde(rename = "func_detail_blocker")]
    #[serde(borrow)]
    FuncDetailBlocker(FuncDetailBlocker<'a>),
    #[serde(rename = "func_door")]
    #[serde(borrow)]
    FuncDoor(FuncDoor<'a>),
    #[serde(rename = "func_door_rotating")]
    #[serde(borrow)]
    FuncDoorRotating(FuncDoorRotating<'a>),
    #[serde(rename = "func_dustcloud")]
    #[serde(borrow)]
    FuncDustcloud(FuncDustcloud<'a>),
    #[serde(rename = "func_dustmotes")]
    #[serde(borrow)]
    FuncDustmotes(FuncDustmotes<'a>),
    #[serde(rename = "func_fish_pool")]
    #[serde(borrow)]
    FuncFishPool(FuncFishPool<'a>),
    #[serde(rename = "func_flag_alert")]
    #[serde(borrow)]
    FuncFlagAlert(FuncFlagAlert<'a>),
    #[serde(rename = "func_flagdetectionzone")]
    #[serde(borrow)]
    FuncFlagdetectionzone(FuncFlagdetectionzone<'a>),
    #[serde(rename = "func_illusionary")]
    #[serde(borrow)]
    FuncIllusionary(FuncIllusionary<'a>),
    #[serde(rename = "func_lod")]
    #[serde(borrow)]
    FuncLod(FuncLod<'a>),
    #[serde(rename = "func_monitor")]
    #[serde(borrow)]
    FuncMonitor(FuncMonitor<'a>),
    #[serde(rename = "func_movelinear")]
    #[serde(borrow)]
    FuncMovelinear(FuncMovelinear<'a>),
    #[serde(rename = "func_nav_avoid")]
    #[serde(borrow)]
    FuncNavAvoid(FuncNavAvoid<'a>),
    #[serde(rename = "func_nav_blocker")]
    #[serde(borrow)]
    FuncNavBlocker(FuncNavBlocker<'a>),
    #[serde(rename = "func_nav_prefer")]
    #[serde(borrow)]
    FuncNavPrefer(FuncNavPrefer<'a>),
    #[serde(rename = "func_nav_prerequisite")]
    #[serde(borrow)]
    FuncNavPrerequisite(FuncNavPrerequisite<'a>),
    #[serde(rename = "func_nobuild")]
    #[serde(borrow)]
    FuncNobuild(FuncNobuild<'a>),
    #[serde(rename = "func_nogrenades")]
    #[serde(borrow)]
    FuncNogrenades(FuncNogrenades<'a>),
    #[serde(rename = "func_occluder")]
    #[serde(borrow)]
    FuncOccluder(FuncOccluder<'a>),
    #[serde(rename = "func_passtime_goal")]
    #[serde(borrow)]
    FuncPasstimeGoal(FuncPasstimeGoal<'a>),
    #[serde(rename = "func_passtime_goalie_zone")]
    #[serde(borrow)]
    FuncPasstimeGoalieZone(FuncPasstimeGoalieZone<'a>),
    #[serde(rename = "func_passtime_no_ball_zone")]
    #[serde(borrow)]
    FuncPasstimeNoBallZone(FuncPasstimeNoBallZone<'a>),
    #[serde(rename = "func_physbox")]
    #[serde(borrow)]
    FuncPhysbox(FuncPhysbox<'a>),
    #[serde(rename = "func_physbox_multiplayer")]
    #[serde(borrow)]
    FuncPhysboxMultiplayer(FuncPhysboxMultiplayer<'a>),
    #[serde(rename = "func_powerupvolume")]
    #[serde(borrow)]
    FuncPowerupvolume(FuncPowerupvolume<'a>),
    #[serde(rename = "func_precipitation")]
    #[serde(borrow)]
    FuncPrecipitation(FuncPrecipitation<'a>),
    #[serde(rename = "func_regenerate")]
    #[serde(borrow)]
    FuncRegenerate(FuncRegenerate<'a>),
    #[serde(rename = "func_respawnflag")]
    #[serde(borrow)]
    FuncRespawnflag(FuncRespawnflag<'a>),
    #[serde(rename = "func_respawnroom")]
    #[serde(borrow)]
    FuncRespawnroom(FuncRespawnroom<'a>),
    #[serde(rename = "func_respawnroomvisualizer")]
    #[serde(borrow)]
    FuncRespawnroomvisualizer(FuncRespawnroomvisualizer<'a>),
    #[serde(rename = "func_rotating")]
    #[serde(borrow)]
    FuncRotating(FuncRotating<'a>),
    #[serde(rename = "func_smokevolume")]
    #[serde(borrow)]
    FuncSmokevolume(FuncSmokevolume<'a>),
    #[serde(rename = "func_suggested_build")]
    #[serde(borrow)]
    FuncSuggestedBuild(FuncSuggestedBuild<'a>),
    #[serde(rename = "func_tanktrain")]
    #[serde(borrow)]
    FuncTanktrain(FuncTanktrain<'a>),
    #[serde(rename = "func_tfbot_hint")]
    #[serde(borrow)]
    FuncTfbotHint(FuncTfbotHint<'a>),
    #[serde(rename = "func_tracktrain")]
    #[serde(borrow)]
    FuncTracktrain(FuncTracktrain<'a>),
    #[serde(rename = "func_upgradestation")]
    #[serde(borrow)]
    FuncUpgradestation(FuncUpgradestation<'a>),
    #[serde(rename = "func_wall")]
    #[serde(borrow)]
    FuncWall(FuncWall<'a>),
    #[serde(rename = "func_wall_toggle")]
    #[serde(borrow)]
    FuncWallToggle(FuncWallToggle<'a>),
    #[serde(rename = "func_water_analog")]
    #[serde(borrow)]
    FuncWaterAnalog(FuncWaterAnalog<'a>),
    #[serde(rename = "game_end")]
    #[serde(borrow)]
    GameEnd(GameEnd<'a>),
    #[serde(rename = "game_forcerespawn")]
    #[serde(borrow)]
    GameForcerespawn(GameForcerespawn<'a>),
    #[serde(rename = "game_intro_viewpoint")]
    #[serde(borrow)]
    GameIntroViewpoint(GameIntroViewpoint<'a>),
    #[serde(rename = "game_round_win")]
    #[serde(borrow)]
    GameRoundWin(GameRoundWin<'a>),
    #[serde(rename = "game_text")]
    #[serde(borrow)]
    GameText(GameText<'a>),
    #[serde(rename = "game_text_tf")]
    #[serde(borrow)]
    GameTextTf(GameTextTf<'a>),
    #[serde(rename = "halloween_fortune_teller")]
    #[serde(borrow)]
    HalloweenFortuneTeller(HalloweenFortuneTeller<'a>),
    #[serde(rename = "halloween_zapper")]
    #[serde(borrow)]
    HalloweenZapper(HalloweenZapper<'a>),
    #[serde(rename = "headless_hatman")]
    #[serde(borrow)]
    HeadlessHatman(HeadlessHatman<'a>),
    #[serde(rename = "info_camera_link")]
    #[serde(borrow)]
    InfoCameraLink(InfoCameraLink<'a>),
    #[serde(rename = "info_intermission")]
    #[serde(borrow)]
    InfoIntermission(InfoIntermission<'a>),
    #[serde(rename = "info_landmark")]
    #[serde(borrow)]
    InfoLandmark(InfoLandmark<'a>),
    #[serde(rename = "info_null")]
    #[serde(borrow)]
    InfoNull(InfoNull<'a>),
    #[serde(rename = "info_observer_point")]
    #[serde(borrow)]
    InfoObserverPoint(InfoObserverPoint<'a>),
    #[serde(rename = "info_overlay_accessor")]
    #[serde(borrow)]
    InfoOverlayAccessor(InfoOverlayAccessor<'a>),
    #[serde(rename = "info_particle_system")]
    #[serde(borrow)]
    InfoParticleSystem(InfoParticleSystem<'a>),
    #[serde(rename = "info_passtime_ball_spawn")]
    #[serde(borrow)]
    InfoPasstimeBallSpawn(InfoPasstimeBallSpawn<'a>),
    #[serde(rename = "info_player_start")]
    InfoPlayerStart(InfoPlayerStart),
    #[serde(rename = "info_player_teamspawn")]
    #[serde(borrow)]
    InfoPlayerTeamspawn(InfoPlayerTeamspawn<'a>),
    #[serde(rename = "info_powerup_spawn")]
    InfoPowerupSpawn(InfoPowerupSpawn),
    #[serde(rename = "info_target")]
    #[serde(borrow)]
    InfoTarget(InfoTarget<'a>),
    #[serde(rename = "info_teleport_destination")]
    #[serde(borrow)]
    InfoTeleportDestination(InfoTeleportDestination<'a>),
    #[serde(rename = "infodecal")]
    #[serde(borrow)]
    Infodecal(Infodecal<'a>),
    #[serde(rename = "item_ammopack_full")]
    #[serde(borrow)]
    ItemAmmopackFull(ItemAmmopackFull<'a>),
    #[serde(rename = "item_ammopack_medium")]
    #[serde(borrow)]
    ItemAmmopackMedium(ItemAmmopackMedium<'a>),
    #[serde(rename = "item_ammopack_small")]
    #[serde(borrow)]
    ItemAmmopackSmall(ItemAmmopackSmall<'a>),
    #[serde(rename = "item_healthammokit")]
    ItemHealthammokit(ItemHealthammokit),
    #[serde(rename = "item_healthkit_full")]
    #[serde(borrow)]
    ItemHealthkitFull(ItemHealthkitFull<'a>),
    #[serde(rename = "item_healthkit_medium")]
    #[serde(borrow)]
    ItemHealthkitMedium(ItemHealthkitMedium<'a>),
    #[serde(rename = "item_healthkit_small")]
    #[serde(borrow)]
    ItemHealthkitSmall(ItemHealthkitSmall<'a>),
    #[serde(rename = "item_teamflag")]
    #[serde(borrow)]
    ItemTeamflag(ItemTeamflag<'a>),
    #[serde(rename = "keyframe_rope")]
    #[serde(borrow)]
    KeyframeRope(KeyframeRope<'a>),
    #[serde(rename = "light")]
    #[serde(borrow)]
    Light(Light<'a>),
    #[serde(rename = "light_environment")]
    #[serde(borrow)]
    LightEnvironment(LightEnvironment<'a>),
    #[serde(rename = "light_spot")]
    #[serde(borrow)]
    LightSpot(LightSpot<'a>),
    #[serde(rename = "logic_auto")]
    #[serde(borrow)]
    LogicAuto(LogicAuto<'a>),
    #[serde(rename = "logic_branch")]
    #[serde(borrow)]
    LogicBranch(LogicBranch<'a>),
    #[serde(rename = "logic_branch_listener")]
    #[serde(borrow)]
    LogicBranchListener(LogicBranchListener<'a>),
    #[serde(rename = "logic_case")]
    #[serde(borrow)]
    LogicCase(LogicCase<'a>),
    #[serde(rename = "logic_collision_pair")]
    #[serde(borrow)]
    LogicCollisionPair(LogicCollisionPair<'a>),
    #[serde(rename = "logic_compare")]
    #[serde(borrow)]
    LogicCompare(LogicCompare<'a>),
    #[serde(rename = "logic_measure_movement")]
    #[serde(borrow)]
    LogicMeasureMovement(LogicMeasureMovement<'a>),
    #[serde(rename = "logic_relay")]
    #[serde(borrow)]
    LogicRelay(LogicRelay<'a>),
    #[serde(rename = "logic_script")]
    #[serde(borrow)]
    LogicScript(LogicScript<'a>),
    #[serde(rename = "logic_timer")]
    #[serde(borrow)]
    LogicTimer(LogicTimer<'a>),
    #[serde(rename = "mapobj_cart_dispenser")]
    #[serde(borrow)]
    MapobjCartDispenser(MapobjCartDispenser<'a>),
    #[serde(rename = "material_modify_control")]
    MaterialModifyControl(MaterialModifyControl),
    #[serde(rename = "math_counter")]
    #[serde(borrow)]
    MathCounter(MathCounter<'a>),
    #[serde(rename = "math_remap")]
    #[serde(borrow)]
    MathRemap(MathRemap<'a>),
    #[serde(rename = "merasmus")]
    #[serde(borrow)]
    Merasmus(Merasmus<'a>),
    #[serde(rename = "momentary_rot_button")]
    #[serde(borrow)]
    MomentaryRotButton(MomentaryRotButton<'a>),
    #[serde(rename = "move_rope")]
    #[serde(borrow)]
    MoveRope(MoveRope<'a>),
    #[serde(rename = "obj_dispenser")]
    #[serde(borrow)]
    ObjDispenser(ObjDispenser<'a>),
    #[serde(rename = "obj_sentrygun")]
    #[serde(borrow)]
    ObjSentrygun(ObjSentrygun<'a>),
    #[serde(rename = "obj_teleporter")]
    #[serde(borrow)]
    ObjTeleporter(ObjTeleporter<'a>),
    #[serde(rename = "passtime_logic")]
    #[serde(borrow)]
    PasstimeLogic(PasstimeLogic<'a>),
    #[serde(rename = "path_corner")]
    #[serde(borrow)]
    PathCorner(PathCorner<'a>),
    #[serde(rename = "path_track")]
    #[serde(borrow)]
    PathTrack(PathTrack<'a>),
    #[serde(rename = "phys_constraint")]
    #[serde(borrow)]
    PhysConstraint(PhysConstraint<'a>),
    #[serde(rename = "phys_hinge")]
    #[serde(borrow)]
    PhysHinge(PhysHinge<'a>),
    #[serde(rename = "phys_ragdollmagnet")]
    #[serde(borrow)]
    PhysRagdollmagnet(PhysRagdollmagnet<'a>),
    #[serde(rename = "phys_thruster")]
    #[serde(borrow)]
    PhysThruster(PhysThruster<'a>),
    #[serde(rename = "point_camera")]
    #[serde(borrow)]
    PointCamera(PointCamera<'a>),
    #[serde(rename = "point_clientcommand")]
    #[serde(borrow)]
    PointClientcommand(PointClientcommand<'a>),
    #[serde(rename = "point_devshot_camera")]
    #[serde(borrow)]
    PointDevshotCamera(PointDevshotCamera<'a>),
    #[serde(rename = "point_hurt")]
    #[serde(borrow)]
    PointHurt(PointHurt<'a>),
    #[serde(rename = "point_populator_interface")]
    #[serde(borrow)]
    PointPopulatorInterface(PointPopulatorInterface<'a>),
    #[serde(rename = "point_push")]
    #[serde(borrow)]
    PointPush(PointPush<'a>),
    #[serde(rename = "point_servercommand")]
    #[serde(borrow)]
    PointServercommand(PointServercommand<'a>),
    #[serde(rename = "point_spotlight")]
    #[serde(borrow)]
    PointSpotlight(PointSpotlight<'a>),
    #[serde(rename = "point_teleport")]
    #[serde(borrow)]
    PointTeleport(PointTeleport<'a>),
    #[serde(rename = "point_template")]
    #[serde(borrow)]
    PointTemplate(PointTemplate<'a>),
    #[serde(rename = "point_tesla")]
    #[serde(borrow)]
    PointTesla(PointTesla<'a>),
    #[serde(rename = "point_viewcontrol")]
    #[serde(borrow)]
    PointViewcontrol(PointViewcontrol<'a>),
    #[serde(rename = "prop_door_rotating")]
    #[serde(borrow)]
    PropDoorRotating(PropDoorRotating<'a>),
    #[serde(rename = "prop_dynamic")]
    #[serde(borrow)]
    PropDynamic(PropDynamic<'a>),
    #[serde(rename = "prop_dynamic_ornament")]
    #[serde(borrow)]
    PropDynamicOrnament(PropDynamicOrnament<'a>),
    #[serde(rename = "prop_dynamic_override")]
    #[serde(borrow)]
    PropDynamicOverride(PropDynamicOverride<'a>),
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
    #[serde(rename = "prop_soccer_ball")]
    #[serde(borrow)]
    PropSoccerBall(PropSoccerBall<'a>),
    #[serde(rename = "shadow_control")]
    #[serde(borrow)]
    ShadowControl(ShadowControl<'a>),
    #[serde(rename = "sky_camera")]
    #[serde(borrow)]
    SkyCamera(SkyCamera<'a>),
    #[serde(rename = "team_control_point")]
    #[serde(borrow)]
    TeamControlPoint(TeamControlPoint<'a>),
    #[serde(rename = "team_control_point_master")]
    #[serde(borrow)]
    TeamControlPointMaster(TeamControlPointMaster<'a>),
    #[serde(rename = "team_control_point_round")]
    #[serde(borrow)]
    TeamControlPointRound(TeamControlPointRound<'a>),
    #[serde(rename = "team_round_timer")]
    #[serde(borrow)]
    TeamRoundTimer(TeamRoundTimer<'a>),
    #[serde(rename = "team_train_watcher")]
    #[serde(borrow)]
    TeamTrainWatcher(TeamTrainWatcher<'a>),
    #[serde(rename = "tf_base_minigame")]
    TfBaseMinigame(TfBaseMinigame),
    #[serde(rename = "tf_gamerules")]
    #[serde(borrow)]
    TfGamerules(TfGamerules<'a>),
    #[serde(rename = "tf_generic_bomb")]
    #[serde(borrow)]
    TfGenericBomb(TfGenericBomb<'a>),
    #[serde(rename = "tf_glow")]
    #[serde(borrow)]
    TfGlow(TfGlow<'a>),
    #[serde(rename = "tf_halloween_gift_spawn_location")]
    #[serde(borrow)]
    TfHalloweenGiftSpawnLocation(TfHalloweenGiftSpawnLocation<'a>),
    #[serde(rename = "tf_halloween_minigame")]
    #[serde(borrow)]
    TfHalloweenMinigame(TfHalloweenMinigame<'a>),
    #[serde(rename = "tf_halloween_minigame_falling_platforms")]
    #[serde(borrow)]
    TfHalloweenMinigameFallingPlatforms(TfHalloweenMinigameFallingPlatforms<'a>),
    #[serde(rename = "tf_halloween_pickup")]
    #[serde(borrow)]
    TfHalloweenPickup(TfHalloweenPickup<'a>),
    #[serde(rename = "tf_logic_arena")]
    #[serde(borrow)]
    TfLogicArena(TfLogicArena<'a>),
    #[serde(rename = "tf_logic_competitive")]
    #[serde(borrow)]
    TfLogicCompetitive(TfLogicCompetitive<'a>),
    #[serde(rename = "tf_logic_cp_timer")]
    #[serde(borrow)]
    TfLogicCpTimer(TfLogicCpTimer<'a>),
    #[serde(rename = "tf_logic_holiday")]
    #[serde(borrow)]
    TfLogicHoliday(TfLogicHoliday<'a>),
    #[serde(rename = "tf_logic_hybrid_ctf_cp")]
    TfLogicHybridCtfCp(TfLogicHybridCtfCp),
    #[serde(rename = "tf_logic_koth")]
    #[serde(borrow)]
    TfLogicKoth(TfLogicKoth<'a>),
    #[serde(rename = "tf_logic_mann_vs_machine")]
    #[serde(borrow)]
    TfLogicMannVsMachine(TfLogicMannVsMachine<'a>),
    #[serde(rename = "tf_logic_mannpower")]
    TfLogicMannpower(TfLogicMannpower),
    #[serde(rename = "tf_logic_medieval")]
    #[serde(borrow)]
    TfLogicMedieval(TfLogicMedieval<'a>),
    #[serde(rename = "tf_logic_minigames")]
    #[serde(borrow)]
    TfLogicMinigames(TfLogicMinigames<'a>),
    #[serde(rename = "tf_logic_multiple_escort")]
    TfLogicMultipleEscort(TfLogicMultipleEscort),
    #[serde(rename = "tf_logic_on_holiday")]
    #[serde(borrow)]
    TfLogicOnHoliday(TfLogicOnHoliday<'a>),
    #[serde(rename = "tf_logic_player_destruction")]
    #[serde(borrow)]
    TfLogicPlayerDestruction(TfLogicPlayerDestruction<'a>),
    #[serde(rename = "tf_logic_robot_destruction")]
    #[serde(borrow)]
    TfLogicRobotDestruction(TfLogicRobotDestruction<'a>),
    #[serde(rename = "tf_logic_training_mode")]
    #[serde(borrow)]
    TfLogicTrainingMode(TfLogicTrainingMode<'a>),
    #[serde(rename = "tf_point_nav_interface")]
    #[serde(borrow)]
    TfPointNavInterface(TfPointNavInterface<'a>),
    #[serde(rename = "tf_point_weapon_mimic")]
    #[serde(borrow)]
    TfPointWeaponMimic(TfPointWeaponMimic<'a>),
    #[serde(rename = "tf_pumpkin_bomb")]
    TfPumpkinBomb(TfPumpkinBomb),
    #[serde(rename = "tf_robot_destruction_robot_spawn")]
    #[serde(borrow)]
    TfRobotDestructionRobotSpawn(TfRobotDestructionRobotSpawn<'a>),
    #[serde(rename = "tf_robot_destruction_spawn_group")]
    #[serde(borrow)]
    TfRobotDestructionSpawnGroup(TfRobotDestructionSpawnGroup<'a>),
    #[serde(rename = "tf_spell_pickup")]
    #[serde(borrow)]
    TfSpellPickup(TfSpellPickup<'a>),
    #[serde(rename = "tf_teleport_location")]
    #[serde(borrow)]
    TfTeleportLocation(TfTeleportLocation<'a>),
    #[serde(rename = "tf_zombie")]
    #[serde(borrow)]
    TfZombie(TfZombie<'a>),
    #[serde(rename = "tf_zombie_spawner")]
    #[serde(borrow)]
    TfZombieSpawner(TfZombieSpawner<'a>),
    #[serde(rename = "training_annotation")]
    #[serde(borrow)]
    TrainingAnnotation(TrainingAnnotation<'a>),
    #[serde(rename = "training_prop_dynamic")]
    #[serde(borrow)]
    TrainingPropDynamic(TrainingPropDynamic<'a>),
    #[serde(rename = "trigger_add_or_remove_tf_player_attributes")]
    #[serde(borrow)]
    TriggerAddOrRemoveTfPlayerAttributes(TriggerAddOrRemoveTfPlayerAttributes<'a>),
    #[serde(rename = "trigger_add_tf_player_condition")]
    #[serde(borrow)]
    TriggerAddTfPlayerCondition(TriggerAddTfPlayerCondition<'a>),
    #[serde(rename = "trigger_apply_impulse")]
    #[serde(borrow)]
    TriggerApplyImpulse(TriggerApplyImpulse<'a>),
    #[serde(rename = "trigger_brush")]
    #[serde(borrow)]
    TriggerBrush(TriggerBrush<'a>),
    #[serde(rename = "trigger_capture_area")]
    #[serde(borrow)]
    TriggerCaptureArea(TriggerCaptureArea<'a>),
    #[serde(rename = "trigger_catapult")]
    #[serde(borrow)]
    TriggerCatapult(TriggerCatapult<'a>),
    #[serde(rename = "trigger_gravity")]
    #[serde(borrow)]
    TriggerGravity(TriggerGravity<'a>),
    #[serde(rename = "trigger_hurt")]
    #[serde(borrow)]
    TriggerHurt(TriggerHurt<'a>),
    #[serde(rename = "trigger_ignite")]
    #[serde(borrow)]
    TriggerIgnite(TriggerIgnite<'a>),
    #[serde(rename = "trigger_ignite_arrows")]
    #[serde(borrow)]
    TriggerIgniteArrows(TriggerIgniteArrows<'a>),
    #[serde(rename = "trigger_look")]
    #[serde(borrow)]
    TriggerLook(TriggerLook<'a>),
    #[serde(rename = "trigger_multiple")]
    #[serde(borrow)]
    TriggerMultiple(TriggerMultiple<'a>),
    #[serde(rename = "trigger_once")]
    #[serde(borrow)]
    TriggerOnce(TriggerOnce<'a>),
    #[serde(rename = "trigger_passtime_ball")]
    #[serde(borrow)]
    TriggerPasstimeBall(TriggerPasstimeBall<'a>),
    #[serde(rename = "trigger_player_respawn_override")]
    #[serde(borrow)]
    TriggerPlayerRespawnOverride(TriggerPlayerRespawnOverride<'a>),
    #[serde(rename = "trigger_push")]
    #[serde(borrow)]
    TriggerPush(TriggerPush<'a>),
    #[serde(rename = "trigger_rd_vault_trigger")]
    #[serde(borrow)]
    TriggerRdVaultTrigger(TriggerRdVaultTrigger<'a>),
    #[serde(rename = "trigger_remove_tf_player_condition")]
    #[serde(borrow)]
    TriggerRemoveTfPlayerCondition(TriggerRemoveTfPlayerCondition<'a>),
    #[serde(rename = "trigger_soundscape")]
    #[serde(borrow)]
    TriggerSoundscape(TriggerSoundscape<'a>),
    #[serde(rename = "trigger_stun")]
    #[serde(borrow)]
    TriggerStun(TriggerStun<'a>),
    #[serde(rename = "trigger_teleport")]
    #[serde(borrow)]
    TriggerTeleport(TriggerTeleport<'a>),
    #[serde(rename = "trigger_teleport_relative")]
    #[serde(borrow)]
    TriggerTeleportRelative(TriggerTeleportRelative<'a>),
    #[serde(rename = "trigger_timer_door")]
    #[serde(borrow)]
    TriggerTimerDoor(TriggerTimerDoor<'a>),
    #[serde(rename = "trigger_vphysics_motion")]
    #[serde(borrow)]
    TriggerVphysicsMotion(TriggerVphysicsMotion<'a>),
    #[serde(rename = "water_lod_control")]
    #[serde(borrow)]
    WaterLodControl(WaterLodControl<'a>),
    #[serde(rename = "wheel_of_doom")]
    #[serde(borrow)]
    WheelOfDoom(WheelOfDoom<'a>),
    #[serde(rename = "worldspawn")]
    #[serde(borrow)]
    Worldspawn(Worldspawn<'a>),
}
#[derive(Debug, Clone, Deserialize)]
pub struct AmbientGeneric<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub cspinup: bool,
    #[serde(default)]
    pub fadeinsecs: Option<f32>,
    #[serde(default)]
    pub fadeoutsecs: Option<f32>,
    pub health: f32,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub lfomodpitch: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub lfomodvol: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub lforate: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub lfotype: bool,
    #[serde(default)]
    pub message: Option<&'a str>,
    #[serde(default)]
    pub onuser1: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub pitch: u8,
    pub pitchstart: u8,
    #[serde(default)]
    pub preset: Option<u8>,
    pub radius: i32,
    #[serde(default)]
    pub sourceentityname: Option<&'a str>,
    pub spawnflags: u8,
    #[serde(default)]
    pub spindown: Option<u8>,
    #[serde(default)]
    pub spinup: Option<u8>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub volstart: Option<f32>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct BaseBoss<'a> {
    pub angles: Angles,
    pub health: u8,
    pub model: &'a str,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub playbackrate: bool,
    pub skin: u8,
    pub speed: u8,
    pub targetname: &'a str,
    pub vscripts: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct BotActionPoint<'a> {
    #[serde(default)]
    pub command: Option<&'a str>,
    pub desired_distance: u8,
    #[serde(default)]
    pub next_action_point: Option<&'a str>,
    #[serde(default)]
    pub onbotreached: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub stay_time: Option<f32>,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct BotGenerator<'a> {
    #[serde(default)]
    pub action_point: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    pub actionondeath: bool,
    pub angles: Angles,
    pub class: &'a str,
    pub count: i32,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub difficulty: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disabledodge: bool,
    #[serde(default)]
    pub initial_command: Option<&'a str>,
    pub interval: u8,
    pub maxactive: u8,
    #[serde(default)]
    pub onbotkilled: Option<&'a str>,
    #[serde(default)]
    pub onspawned: Option<&'a str>,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub removeondeath: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub retainbuildings: bool,
    #[serde(default)]
    pub spawnflags: Option<u16>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spectateondeath: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub suppressfire: bool,
    pub targetname: &'a str,
    pub team: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub useteamspawnpoint: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct BotHintEngineerNest<'a> {
    pub angles: Angles,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(default)]
    pub startdisabled: Option<&'a str>,
    pub targetname: &'a str,
    pub teamnum: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct BotHintSentrygun<'a> {
    pub angles: Angles,
    pub origin: Vector,
    #[serde(default)]
    pub rangehelper: Option<u16>,
    #[serde(default)]
    pub sequence: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(default)]
    pub startdisabled: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub sticky: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub teamnum: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct BotHintSniperSpot<'a> {
    pub angles: Angles,
    pub origin: Vector,
    pub radius: u16,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    pub targetname: &'a str,
    pub teamnum: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct BotHintTeleporterExit<'a> {
    pub angles: Angles,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(default)]
    pub startdisabled: Option<&'a str>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub teamnum: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct BotRoster<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    pub allowclasschanges: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub allowdemoman: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub allowengineer: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub allowheavy: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub allowmedic: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub allowpyro: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub allowscout: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub allowsniper: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub allowsoldier: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub allowspy: bool,
    pub origin: Vector,
    pub targetname: &'a str,
    pub team: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ColorCorrection<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub fadeinduration: f32,
    pub fadeoutduration: f32,
    #[serde(default)]
    pub filename: Option<&'a str>,
    pub maxfalloff: f32,
    pub maxweight: f32,
    pub minfalloff: f32,
    #[serde(default)]
    pub onmultinewround: Option<&'a str>,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct DispenserTouchTrigger<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub filtername: Option<&'a str>,
    pub model: &'a str,
    #[serde(default)]
    pub origin: Option<Vector>,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub solid: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EditorText<'a> {
    pub angles: Angles,
    pub color: Color,
    pub message: &'a str,
    pub origin: Vector,
    pub textsize: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EntitySpawnManager<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub drop_to_ground: bool,
    #[serde(default)]
    pub entity_count: Option<u8>,
    #[serde(default)]
    pub entity_name: Option<&'a str>,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub random_rotation: bool,
    #[serde(default)]
    pub respawn_time: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EntitySpawnPoint<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub origin: Vector,
    pub spawn_manager_name: &'a str,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvBeam<'a> {
    pub boltwidth: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub damage: bool,
    pub decalname: &'a str,
    #[serde(default)]
    pub dissolvetype: Option<i32>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub framerate: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub framestart: bool,
    pub hdrcolorscale: f32,
    pub life: f32,
    #[serde(default)]
    pub lightningend: Option<&'a str>,
    pub lightningstart: &'a str,
    pub noiseamplitude: u8,
    pub origin: Vector,
    pub radius: u16,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub rendermode: bool,
    pub spawnflags: u16,
    pub striketime: f32,
    pub targetname: &'a str,
    pub texture: &'a str,
    pub texturescroll: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub touchtype: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvBubbles<'a> {
    pub current: u8,
    pub density: u8,
    pub frequency: u8,
    pub model: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvEntityMaker<'a> {
    pub angles: Angles,
    pub entitytemplate: &'a str,
    #[serde(default)]
    pub model: Option<&'a str>,
    #[serde(default)]
    pub onentityspawned: Option<&'a str>,
    #[serde(default)]
    pub onuser1: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub postspawndirection: Vector,
    pub postspawndirectionvariance: f32,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub postspawninheritangles: bool,
    #[serde(default)]
    pub postspawnspeed: Option<u16>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvExplosion<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub fireballsprite: &'a str,
    #[serde(default)]
    pub ignoredentity: Option<&'a str>,
    pub imagnitude: u16,
    pub iradiusoverride: u16,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub rendermode: u8,
    pub spawnflags: u16,
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
pub struct EnvFogController<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablex360: bool,
    pub farz: i32,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub fogblend: bool,
    pub fogcolor: &'a str,
    pub fogcolor2: &'a str,
    pub fogdir: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub fogenable: bool,
    pub fogend: f32,
    #[serde(default)]
    pub foglerptime: Option<f32>,
    pub fogmaxdensity: f32,
    pub fogstart: f32,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub hdrcolorscale: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub maxcpulevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub maxgpulevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub mincpulevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub mindxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub mingpulevel: bool,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub use_angles: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub zoomfogscale: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvGlow<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub frame: bool,
    pub framerate: f32,
    pub glowproxysize: u8,
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
    pub renderfx: bool,
    pub rendermode: u8,
    pub scale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvLaser<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub damage: u16,
    pub dissolvetype: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(default)]
    pub endsprite: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub framestart: bool,
    #[serde(default)]
    pub hdrcolorscale: Option<f32>,
    pub lasertarget: &'a str,
    pub noiseamplitude: u8,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub rendermode: bool,
    pub spawnflags: u8,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub texture: &'a str,
    pub texturescroll: u8,
    pub width: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvLightglow<'a> {
    #[serde(default)]
    pub _light: Option<LightColor>,
    pub angles: Angles,
    pub glowproxysize: f32,
    pub hdrcolorscale: f32,
    pub horizontalglowsize: u8,
    pub maxdist: f32,
    pub mindist: i32,
    pub origin: Vector,
    #[serde(default)]
    pub outermaxdist: Option<u16>,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(default)]
    pub renderamt: Option<u8>,
    pub rendercolor: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub verticalglowsize: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvPhysexplosion<'a> {
    pub inner_radius: u16,
    pub magnitude: u32,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub radius: bool,
    pub spawnflags: u8,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvScreenoverlay<'a> {
    pub origin: Vector,
    pub overlayname1: &'a str,
    #[serde(default)]
    pub overlayname10: Option<&'a str>,
    #[serde(default)]
    pub overlayname2: Option<&'a str>,
    #[serde(default)]
    pub overlayname3: Option<&'a str>,
    #[serde(default)]
    pub overlayname4: Option<&'a str>,
    #[serde(default)]
    pub overlayname5: Option<&'a str>,
    #[serde(default)]
    pub overlayname6: Option<&'a str>,
    #[serde(default)]
    pub overlayname7: Option<&'a str>,
    #[serde(default)]
    pub overlayname8: Option<&'a str>,
    #[serde(default)]
    pub overlayname9: Option<&'a str>,
    pub overlaytime1: f32,
    pub overlaytime10: f32,
    pub overlaytime2: f32,
    pub overlaytime3: f32,
    pub overlaytime4: f32,
    pub overlaytime5: f32,
    pub overlaytime6: f32,
    pub overlaytime7: f32,
    pub overlaytime8: f32,
    pub overlaytime9: f32,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvShake<'a> {
    pub amplitude: f32,
    #[serde(default)]
    pub angles: Option<Angles>,
    pub duration: f32,
    pub frequency: f32,
    #[serde(default)]
    pub onuser1: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub radius: i32,
    pub spawnflags: u8,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvShooter<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    pub delay: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    pub gibangles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    pub gibanglevelocity: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub gibgravityscale: bool,
    pub m_flgiblife: u8,
    pub m_flvariance: f32,
    pub m_flvelocity: u16,
    pub m_igibs: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub massoverride: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub nogibshadows: bool,
    pub origin: Vector,
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
    pub initialstate: bool,
    pub jetlength: u16,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub rate: u8,
    pub renderamt: u8,
    pub rendercolor: Color,
    pub roll: u8,
    pub smokematerial: &'a str,
    pub speed: u8,
    pub spreadspeed: u8,
    pub startsize: u8,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub twist: u8,
    #[serde(default)]
    pub windangle: Option<i32>,
    #[serde(default)]
    pub windspeed: Option<u8>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSoundscape<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub origin: Vector,
    #[serde(default)]
    pub position0: Option<&'a str>,
    #[serde(default)]
    pub position1: Option<&'a str>,
    #[serde(default)]
    pub position2: Option<&'a str>,
    #[serde(default)]
    pub position3: Option<&'a str>,
    #[serde(default)]
    pub position4: Option<&'a str>,
    #[serde(default)]
    pub position5: Option<&'a str>,
    #[serde(default)]
    pub position6: Option<&'a str>,
    #[serde(default)]
    pub position7: Option<&'a str>,
    pub radius: i32,
    pub soundscape: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSoundscapeProxy<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub mainsoundscapename: Option<&'a str>,
    pub origin: Vector,
    pub radius: i32,
    #[serde(default)]
    pub sides: Option<&'a str>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSoundscapeTriggerable<'a> {
    pub origin: Vector,
    #[serde(default)]
    pub position0: Option<&'a str>,
    #[serde(default)]
    pub position1: Option<&'a str>,
    #[serde(default)]
    pub position2: Option<&'a str>,
    #[serde(default)]
    pub position3: Option<&'a str>,
    #[serde(default)]
    pub position4: Option<&'a str>,
    #[serde(default)]
    pub position5: Option<&'a str>,
    #[serde(default)]
    pub position6: Option<&'a str>,
    pub radius: i32,
    pub soundscape: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSpark<'a> {
    pub angles: Angles,
    pub magnitude: u8,
    #[serde(default)]
    pub maxdelay: Option<f32>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub spawnflags: u16,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub traillength: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSprite<'a> {
    #[serde(default)]
    pub _cone: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub _inner_cone: bool,
    #[serde(default)]
    pub _light: Option<LightColor>,
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(default)]
    pub fademaxdist: Option<u16>,
    #[serde(default)]
    pub fademindist: Option<u16>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub frame: bool,
    pub framerate: f32,
    pub glowproxysize: f32,
    pub hdrcolorscale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(default)]
    pub mindxlevel: Option<u8>,
    pub model: &'a str,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub renderamt: u8,
    pub rendercolor: &'a str,
    #[serde(default)]
    pub renderfx: Option<u8>,
    pub rendermode: u8,
    #[serde(default)]
    pub scale: Option<f32>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSteam<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    pub endsize: u16,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub initialstate: bool,
    pub jetlength: u16,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub r#type: bool,
    pub rate: u8,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub rendermode: bool,
    pub rollspeed: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub speed: u8,
    pub spreadspeed: u8,
    pub startsize: u16,
    #[serde(default)]
    pub targetname: Option<&'a str>,
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
    pub pitch: Option<f32>,
    pub rendercolor: Color,
    pub size: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub use_angles: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvTexturetoggle<'a> {
    pub origin: Vector,
    pub target: &'a str,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvTonemapController<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvWind<'a> {
    pub angles: Angles,
    pub gustdirchange: u8,
    pub gustduration: u8,
    pub maxgust: u16,
    pub maxgustdelay: u16,
    pub maxwind: u16,
    pub mingust: u8,
    pub mingustdelay: u16,
    pub minwind: u8,
    pub origin: Vector,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EyeballBoss<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub origin: Vector,
    #[serde(default)]
    pub physdamagescale: Option<f32>,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(default)]
    pub spawnflags: Option<u16>,
    pub targetname: &'a str,
    #[serde(default)]
    pub team: Option<u8>,
    pub teamnum: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterActivatorClass<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub filterclass: &'a str,
    #[serde(default)]
    pub negated: Option<&'a str>,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterActivatorName<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub filtername: Option<&'a str>,
    #[serde(default)]
    pub negated: Option<&'a str>,
    #[serde(default)]
    pub onpass: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterActivatorTeam<'a> {
    pub filterteam: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub negated: bool,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterActivatorTfteam<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub controlpoint: Option<&'a str>,
    #[serde(default)]
    pub linedivider1: Option<&'a str>,
    #[serde(default)]
    pub negated: Option<&'a str>,
    #[serde(default)]
    pub onfail: Option<&'a str>,
    #[serde(default)]
    pub onpass: Option<&'a str>,
    pub origin: Vector,
    pub targetname: &'a str,
    #[serde(default)]
    pub teamnum: Option<u8>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterBase<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    pub negated: bool,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterDamageType<'a> {
    pub damagetype: u16,
    pub negated: &'a str,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterMulti<'a> {
    #[serde(default)]
    pub filter01: Option<&'a str>,
    #[serde(default)]
    pub filter02: Option<&'a str>,
    #[serde(default)]
    pub filter03: Option<&'a str>,
    #[serde(default)]
    pub filter04: Option<&'a str>,
    #[serde(default)]
    pub filter05: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    pub filtertype: bool,
    #[serde(default)]
    pub negated: Option<&'a str>,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterTfBotHasTag<'a> {
    pub negated: &'a str,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub require_all_tags: bool,
    pub tags: &'a str,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterTfClass<'a> {
    pub negated: &'a str,
    pub origin: Vector,
    pub targetname: &'a str,
    pub tfclass: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterTfCondition<'a> {
    pub condition: u8,
    #[serde(default)]
    pub negated: Option<&'a str>,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterTfDamagedByWeaponInSlot<'a> {
    pub negated: &'a str,
    pub origin: Vector,
    #[serde(default)]
    pub spawnflags: Option<u8>,
    pub targetname: &'a str,
    pub weaponslot: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncAchievement<'a> {
    pub model: &'a str,
    #[serde(default)]
    pub origin: Option<Vector>,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    pub teamnum: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub zone_id: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncAreaportal<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub origin: Option<Vector>,
    pub portalnumber: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub portalversion: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub startopen: bool,
    #[serde(default)]
    pub target: Option<&'a str>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncAreaportalwindow<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub backgroundbmodel: Option<&'a str>,
    pub fadedist: u16,
    pub fadestartdist: u16,
    #[serde(default)]
    pub origin: Option<Vector>,
    pub portalnumber: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub portalversion: bool,
    #[serde(default)]
    pub target: Option<&'a str>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub translucencylimit: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncBreakable<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub explodedamage: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub explodemagnitude: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub exploderadius: bool,
    pub explosion: u8,
    pub gibdir: Vector,
    pub health: i32,
    pub material: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub minhealthdmg: bool,
    pub model: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub nodamageforces: bool,
    #[serde(default)]
    pub onbreak: Option<&'a str>,
    pub origin: Vector,
    pub performancemode: u8,
    pub physdamagescale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub pressuredelay: bool,
    pub propdata: u8,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub rendermode: u8,
    pub spawnflags: u16,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnobject: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub texframeindex: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub vrad_brush_cast_shadows: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncBrush<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub _minlight: bool,
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(default)]
    pub inputfilter: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub invert_exclusion: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub mindxlevel: bool,
    #[serde(default)]
    pub model: Option<&'a str>,
    #[serde(default)]
    pub onfullyopen: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(default)]
    pub renderfx: Option<u8>,
    #[serde(default)]
    pub rendermode: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub solid: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub solidbsp: bool,
    #[serde(default)]
    pub solidity: Option<&'a str>,
    pub spawnflags: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub texframeindex: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub vrad_brush_cast_shadows: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncButton<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub damagefilter: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub health: bool,
    pub lip: i32,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub locked_sentence: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub locked_sound: bool,
    pub model: &'a str,
    pub movedir: Vector,
    pub ondamaged: &'a str,
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
    #[serde(default)]
    pub sounds: Option<u8>,
    pub spawnflags: u16,
    pub speed: u16,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub texframeindex: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub unlocked_sentence: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub unlocked_sound: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub vrad_brush_cast_shadows: bool,
    pub wait: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncCapturezone<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub capture_delay: Option<f32>,
    #[serde(default)]
    pub capture_delay_offset: Option<f32>,
    #[serde(deserialize_with = "bool_from_int")]
    pub capturepoint: bool,
    pub model: &'a str,
    #[serde(default)]
    pub oncapteam1: Option<&'a str>,
    #[serde(default)]
    pub oncapteam1_pd: Option<&'a str>,
    #[serde(default)]
    pub oncapteam2: Option<&'a str>,
    #[serde(default)]
    pub oncapteam2_pd: Option<&'a str>,
    #[serde(default)]
    pub oncapture: Option<&'a str>,
    #[serde(default)]
    pub origin: Option<Vector>,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub shouldblock: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub teamnum: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncClipVphysics<'a> {
    #[serde(default)]
    pub filtername: Option<&'a str>,
    pub model: &'a str,
    #[serde(default)]
    pub origin: Option<Vector>,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncCroc<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub croc_model: Option<&'a str>,
    #[serde(default)]
    pub filtername: Option<&'a str>,
    #[serde(default)]
    pub kill_icon: Option<&'a str>,
    pub model: &'a str,
    #[serde(default)]
    pub oneat: Option<&'a str>,
    #[serde(default)]
    pub origin: Option<Vector>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub teamnum: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncDetailBlocker<'a> {
    pub model: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncDoor<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub _minlight: bool,
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub closesound: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(default)]
    pub dmg: Option<u32>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(default)]
    pub filtername: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    pub forceclosed: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub health: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub ignoredebris: bool,
    #[serde(default)]
    pub lip: Option<i32>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub locked_sentence: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub loopmovesound: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub mindxlevel: bool,
    pub model: &'a str,
    pub movedir: Vector,
    #[serde(default)]
    pub noise1: Option<&'a str>,
    #[serde(default)]
    pub noise2: Option<&'a str>,
    #[serde(default)]
    pub onclose: Option<&'a str>,
    #[serde(default)]
    pub onfullyclosed: Option<&'a str>,
    #[serde(default)]
    pub onfullyopen: Option<&'a str>,
    #[serde(default)]
    pub onopen: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub rendermode: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub solid: bool,
    pub spawnflags: u16,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnpos: bool,
    pub speed: f32,
    #[serde(default)]
    pub startclosesound: Option<&'a str>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub texframeindex: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub unlocked_sentence: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub vrad_brush_cast_shadows: bool,
    pub wait: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncDoorRotating<'a> {
    pub angles: Angles,
    #[serde(default)]
    pub closesound: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    pub distance: f32,
    pub dmg: u32,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub forceclosed: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub health: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub ignoredebris: bool,
    #[serde(default)]
    pub lip: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub locked_sentence: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub loopmovesound: bool,
    pub model: &'a str,
    #[serde(default)]
    pub noise1: Option<&'a str>,
    #[serde(default)]
    pub noise2: Option<&'a str>,
    #[serde(default)]
    pub onclose: Option<&'a str>,
    #[serde(default)]
    pub onfullyclosed: Option<&'a str>,
    #[serde(default)]
    pub onfullyopen: Option<&'a str>,
    #[serde(default)]
    pub onopen: Option<&'a str>,
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
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnpos: bool,
    pub speed: f32,
    #[serde(default)]
    pub startclosesound: Option<&'a str>,
    pub targetname: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub texframeindex: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub unlocked_sentence: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub vrad_brush_cast_shadows: bool,
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
    pub spawnrate: u8,
    pub speedmax: u16,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncDustmotes<'a> {
    pub alpha: u8,
    #[serde(default)]
    pub angles: Option<Angles>,
    pub color: &'a str,
    pub distmax: u16,
    #[serde(default)]
    pub fallspeed: Option<i32>,
    #[serde(deserialize_with = "bool_from_int")]
    pub frozen: bool,
    pub lifetimemax: f32,
    pub lifetimemin: f32,
    pub model: &'a str,
    #[serde(default)]
    pub origin: Option<Vector>,
    pub sizemax: u16,
    pub sizemin: u8,
    pub spawnrate: u16,
    pub speedmax: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncFishPool<'a> {
    pub fish_count: u8,
    pub max_range: u8,
    pub model: &'a str,
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncFlagAlert<'a> {
    pub alert_delay: u8,
    pub model: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub playsound: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub targetname: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub teamnum: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncFlagdetectionzone<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub alarm: bool,
    pub model: &'a str,
    #[serde(default)]
    pub onendtouchflag: Option<&'a str>,
    pub onstarttouchflag: &'a str,
    #[serde(default)]
    pub origin: Option<Vector>,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub teamnum: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncIllusionary<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(default)]
    pub fademaxdist: Option<i32>,
    #[serde(default)]
    pub fademindist: Option<i32>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub mindxlevel: bool,
    #[serde(default)]
    pub model: Option<&'a str>,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(default)]
    pub rendermode: Option<u8>,
    #[serde(default)]
    pub spawnflags: Option<u8>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub texframeindex: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub vrad_brush_cast_shadows: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncLod<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub disappeardist: u16,
    #[serde(default)]
    pub model: Option<&'a str>,
    #[serde(default)]
    pub origin: Option<Vector>,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub solid: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub vrad_brush_cast_shadows: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncMonitor<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub effects: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub invert_exclusion: bool,
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
    #[serde(deserialize_with = "bool_from_int")]
    pub solidity: bool,
    pub spawnflags: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub target: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub texframeindex: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub vrad_brush_cast_shadows: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncMovelinear<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub blockdamage: u16,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    pub model: &'a str,
    pub movedir: Vector,
    pub movedistance: u16,
    #[serde(default)]
    pub onfullyclosed: Option<&'a str>,
    pub onfullyopen: &'a str,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub rendermode: u8,
    pub spawnflags: u8,
    pub speed: u16,
    #[serde(deserialize_with = "bool_from_int")]
    pub startposition: bool,
    #[serde(default)]
    pub startsound: Option<&'a str>,
    #[serde(default)]
    pub stopsound: Option<&'a str>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncNavAvoid<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub avoid_groups: Option<&'a str>,
    pub model: &'a str,
    #[serde(default)]
    pub origin: Option<Vector>,
    #[serde(deserialize_with = "bool_from_int")]
    pub start_disabled: bool,
    #[serde(default)]
    pub tags: Option<&'a str>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub team: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncNavBlocker<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    pub affectsflow: bool,
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    pub model: &'a str,
    #[serde(default)]
    pub origin: Option<Vector>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    pub teamtoblock: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncNavPrefer<'a> {
    pub model: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub start_disabled: bool,
    pub tags: &'a str,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub team: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncNavPrerequisite<'a> {
    pub entity: &'a str,
    pub filtername: &'a str,
    pub model: &'a str,
    pub origin: Vector,
    pub spawnflags: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub start_disabled: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub targetname: &'a str,
    pub task: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub value: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncNobuild<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub allowdispenser: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub allowsentry: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub allowteleporters: bool,
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub destroybuildings: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    pub model: &'a str,
    #[serde(default)]
    pub onendtouchall: Option<&'a str>,
    #[serde(default)]
    pub onstarttouch: Option<&'a str>,
    #[serde(default)]
    pub origin: Option<Vector>,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(default)]
    pub spawnflags: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub teamnum: Option<u8>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncNogrenades<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub filtername: Option<&'a str>,
    pub model: &'a str,
    #[serde(default)]
    pub origin: Option<Vector>,
    #[serde(default)]
    pub spawnflags: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub teamnum: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncOccluder<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub model: &'a str,
    pub occludernumber: u8,
    #[serde(default)]
    pub origin: Option<Vector>,
    #[serde(deserialize_with = "bool_from_int")]
    pub startactive: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncPasstimeGoal<'a> {
    pub model: &'a str,
    pub onscoreblu: &'a str,
    pub onscorered: &'a str,
    pub origin: Vector,
    pub points: u8,
    pub spawnflags: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub teamnum: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncPasstimeGoalieZone<'a> {
    pub model: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub teamnum: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncPasstimeNoBallZone<'a> {
    pub model: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncPhysbox<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
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
    pub gibdir: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub health: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub massscale: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub material: bool,
    pub model: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub notsolid: bool,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub performancemode: bool,
    #[serde(default)]
    pub physdamagescale: Option<f32>,
    pub preferredcarryangles: Angles,
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
pub struct FuncPhysboxMultiplayer<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "bool_from_int")]
    pub damagetoenablemotion: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub damagetype: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
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
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub forcetoenablemotion: bool,
    pub gibdir: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub health: bool,
    pub massscale: u8,
    pub material: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub minhealthdmg: bool,
    pub model: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub nodamageforces: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub notsolid: bool,
    #[serde(default)]
    pub ondamaged: Option<&'a str>,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub performancemode: bool,
    #[serde(default)]
    pub physdamagescale: Option<f32>,
    pub preferredcarryangles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub pressuredelay: bool,
    pub propdata: u8,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub rendermode: bool,
    pub spawnflags: u32,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnobject: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub texframeindex: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub vrad_brush_cast_shadows: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncPowerupvolume<'a> {
    #[serde(default)]
    pub filtername: Option<&'a str>,
    pub model: &'a str,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub targetname: &'a str,
    pub teamnum: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncPrecipitation<'a> {
    pub model: &'a str,
    pub preciptype: u8,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncRegenerate<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub associatedmodel: Option<&'a str>,
    pub model: &'a str,
    #[serde(default)]
    pub onendtouchall: Option<&'a str>,
    #[serde(default)]
    pub origin: Option<Vector>,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub solid: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub teamnum: Option<u8>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncRespawnflag<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub model: &'a str,
    #[serde(default)]
    pub origin: Option<Vector>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncRespawnroom<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub filtername: Option<&'a str>,
    pub model: &'a str,
    #[serde(default)]
    pub onstarttouch: Option<&'a str>,
    #[serde(default)]
    pub origin: Option<Vector>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub rendermode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub solid: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub teamnum: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncRespawnroomvisualizer<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(default)]
    pub inputfilter: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub invert_exclusion: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub mindxlevel: bool,
    pub model: &'a str,
    #[serde(default)]
    pub onclose: Option<&'a str>,
    #[serde(default)]
    pub onopen: Option<&'a str>,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub rendermode: u8,
    #[serde(default)]
    pub respawnroomname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub solid: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub solid_to_enemies: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub solidbsp: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub solidity: bool,
    pub spawnflags: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub teamnum: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub texframeindex: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub vrad_brush_cast_shadows: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncRotating<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(default)]
    pub dmg: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    pub fanfriction: u8,
    pub maxspeed: f32,
    #[serde(default)]
    pub message: Option<&'a str>,
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
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub texframeindex: bool,
    pub volume: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub vrad_brush_cast_shadows: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncSmokevolume<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub color1: Color,
    pub color2: Color,
    pub density: f32,
    pub densityrampspeed: u8,
    pub material: &'a str,
    pub model: &'a str,
    pub movementspeed: u8,
    pub particledrawwidth: u8,
    pub particlespacingdistance: u8,
    pub rotationspeed: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncSuggestedBuild<'a> {
    #[serde(default)]
    pub face_entity: Option<&'a str>,
    pub face_entity_fov: u8,
    pub model: &'a str,
    pub object_type: u8,
    #[serde(default)]
    pub onbuildingupgraded: Option<&'a str>,
    pub onbuildinsidearea: &'a str,
    #[serde(default)]
    pub onbuildnotfacing: Option<&'a str>,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub targetname: &'a str,
    pub teamnum: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncTanktrain<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    pub bank: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    pub dmg: u16,
    pub health: u32,
    pub height: u8,
    pub model: &'a str,
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
    pub rendermode: u8,
    pub spawnflags: u16,
    pub speed: u16,
    pub startspeed: u16,
    pub target: &'a str,
    pub targetname: &'a str,
    pub velocitytype: u8,
    pub volume: u8,
    pub wheels: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncTfbotHint<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    pub hint: bool,
    pub model: &'a str,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub team: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncTracktrain<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub _minlight: bool,
    #[serde(default)]
    pub angles: Option<Angles>,
    pub bank: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(default)]
    pub dmg: Option<u32>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    pub height: f32,
    #[serde(default)]
    pub manualaccelspeed: Option<u8>,
    #[serde(default)]
    pub manualdecelspeed: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub manualspeedchanges: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub mindxlevel: bool,
    pub model: &'a str,
    #[serde(default)]
    pub movesound: Option<&'a str>,
    pub movesoundmaxpitch: u16,
    #[serde(deserialize_with = "bool_from_int")]
    pub movesoundmaxtime: bool,
    pub movesoundminpitch: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub movesoundmintime: bool,
    #[serde(default)]
    pub onnext: Option<&'a str>,
    #[serde(default)]
    pub onstart: Option<&'a str>,
    #[serde(default)]
    pub onuser1: Option<&'a str>,
    #[serde(default)]
    pub onuser2: Option<&'a str>,
    #[serde(default)]
    pub onuser3: Option<&'a str>,
    #[serde(default)]
    pub onuser4: Option<&'a str>,
    pub orientationtype: u8,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    pub rendermode: u8,
    pub spawnflags: u16,
    pub speed: u8,
    #[serde(default)]
    pub startsound: Option<&'a str>,
    pub startspeed: u16,
    #[serde(default)]
    pub stopsound: Option<&'a str>,
    #[serde(default)]
    pub target: Option<&'a str>,
    pub targetname: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub texframeindex: bool,
    pub velocitytype: u8,
    pub volume: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub vrad_brush_cast_shadows: bool,
    pub wheels: u16,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncUpgradestation<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub model: &'a str,
    #[serde(default)]
    pub onendtouch: Option<&'a str>,
    #[serde(default)]
    pub onstarttouch: Option<&'a str>,
    #[serde(default)]
    pub origin: Option<Vector>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
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
    #[serde(deserialize_with = "bool_from_int")]
    pub rendermode: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncWallToggle<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    pub model: &'a str,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub rendermode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncWaterAnalog<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub blockdamage: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    pub model: &'a str,
    pub movedir: Vector,
    pub movedistance: u16,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub rendermode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    pub speed: u8,
    pub startposition: f32,
    #[serde(default)]
    pub startsound: Option<&'a str>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub texframeindex: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub vrad_brush_cast_shadows: bool,
    #[serde(default)]
    pub waveheight: Option<f32>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GameEnd<'a> {
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GameForcerespawn<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GameIntroViewpoint<'a> {
    pub angles: Angles,
    #[serde(default)]
    pub event_data_int: Option<u8>,
    pub event_delay: u8,
    #[serde(default)]
    pub event_to_fire: Option<&'a str>,
    #[serde(default)]
    pub fov: Option<u8>,
    #[serde(default)]
    pub hint_message: Option<&'a str>,
    pub origin: Vector,
    pub step_number: u8,
    #[serde(default)]
    pub teamnum: Option<u8>,
    pub time_delay: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GameRoundWin<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "bool_from_int")]
    pub force_map_reset: bool,
    #[serde(default)]
    pub onroundwin: Option<&'a str>,
    #[serde(default)]
    pub onuser1: Option<&'a str>,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub switch_teams: bool,
    pub targetname: &'a str,
    pub teamnum: u8,
    #[serde(default)]
    pub win_reason: Option<u8>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GameText<'a> {
    pub channel: u8,
    pub color: Color,
    pub color2: Color,
    pub effect: u8,
    pub fadein: f32,
    pub fadeout: f32,
    pub fxtime: f32,
    pub holdtime: f32,
    #[serde(default)]
    pub message: Option<&'a str>,
    pub origin: Vector,
    pub spawnflags: u8,
    pub targetname: &'a str,
    pub x: f32,
    pub y: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GameTextTf<'a> {
    #[serde(default)]
    pub background: Option<u8>,
    #[serde(default)]
    pub display_to_team: Option<u8>,
    #[serde(default)]
    pub icon: Option<&'a str>,
    pub message: &'a str,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct HalloweenFortuneTeller<'a> {
    pub angles: Angles,
    pub blue_teleport: &'a str,
    pub onfortunecurse: &'a str,
    pub onfortuneend: &'a str,
    pub onfortunewarning: &'a str,
    pub origin: Vector,
    pub red_teleport: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub skin: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct HalloweenZapper<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub origin: Vector,
    pub particleeffect: &'a str,
    pub targetname: &'a str,
    pub touch_trigger: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub zappertype: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct HeadlessHatman<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub ignoreunseenenemies: bool,
    pub origin: Vector,
    #[serde(default)]
    pub physdamagescale: Option<f32>,
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
    pub sleepstate: bool,
    pub spawnflags: u16,
    pub targetname: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub wakeradius: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoCameraLink<'a> {
    pub angles: Angles,
    pub origin: Vector,
    pub pointcamera: &'a str,
    pub target: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoIntermission<'a> {
    pub origin: Vector,
    pub target: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoLandmark<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoNull<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub origin: Vector,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoObserverPoint<'a> {
    pub angles: &'a str,
    #[serde(default)]
    pub associated_team_entity: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub defaultwelcome: bool,
    #[serde(default)]
    pub fov: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub match_summary: bool,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub target: Option<&'a str>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub teamnum: Option<u8>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoOverlayAccessor<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub basisnormal: Vector,
    pub basisorigin: Vector,
    pub basisu: Vector,
    pub basisv: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub endu: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub endv: bool,
    pub fademindist: i32,
    pub material: &'a str,
    pub origin: Vector,
    pub overlayid: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub renderorder: bool,
    pub sides: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub startu: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub startv: bool,
    pub targetname: &'a str,
    pub uv0: Vector,
    pub uv1: Vector,
    pub uv2: Vector,
    pub uv3: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoParticleSystem<'a> {
    #[serde(default)]
    pub _light: Option<LightColor>,
    pub angles: Angles,
    #[serde(default)]
    pub cpoint1: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub cpoint1_parent: bool,
    #[serde(default)]
    pub cpoint2: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub cpoint2_parent: bool,
    #[serde(default)]
    pub cpoint3: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub cpoint3_parent: bool,
    #[serde(default)]
    pub cpoint4: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub cpoint4_parent: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub cpoint5_parent: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub cpoint6_parent: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub cpoint7_parent: bool,
    #[serde(default)]
    pub effect_name: Option<&'a str>,
    #[serde(default)]
    pub fademaxdist: Option<u16>,
    #[serde(default)]
    pub fademindist: Option<u16>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub flag_as_weather: bool,
    #[serde(default)]
    pub onuser1: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub start_active: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoPasstimeBallSpawn<'a> {
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoPlayerStart {
    pub angles: Angles,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoPlayerTeamspawn<'a> {
    #[serde(default)]
    pub activate_goal_no: Option<u16>,
    pub angles: Angles,
    #[serde(default)]
    pub controlpoint: Option<&'a str>,
    #[serde(default)]
    pub goal_activation: Option<u8>,
    #[serde(default)]
    pub goal_result: Option<u8>,
    #[serde(default)]
    pub goal_state: Option<u8>,
    #[serde(default)]
    pub group_no: Option<u8>,
    #[serde(default)]
    pub matchsummary: Option<u8>,
    #[serde(default)]
    pub model: Option<&'a str>,
    #[serde(default)]
    pub modelscale: Option<f32>,
    pub origin: Vector,
    #[serde(default)]
    pub round_bluespawn: Option<&'a str>,
    #[serde(default)]
    pub round_redspawn: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnarmor: bool,
    #[serde(default)]
    pub spawnflags: Option<u16>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawngrenades: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnmode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub team_no: Option<u8>,
    #[serde(default)]
    pub teamnum: Option<u8>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoPowerupSpawn {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    pub team: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoTarget<'a> {
    pub angles: Angles,
    #[serde(default)]
    pub model: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub vscripts: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoTeleportDestination<'a> {
    pub angles: Angles,
    #[serde(default)]
    pub model: Option<&'a str>,
    #[serde(default)]
    pub modelscale: Option<f32>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
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
pub struct ItemAmmopackFull<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub automaterialize: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub body: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(default)]
    pub fademaxdist: Option<u16>,
    #[serde(default)]
    pub fademindist: Option<i32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    #[serde(default)]
    pub modelscale: Option<f32>,
    #[serde(default)]
    pub onplayertouch: Option<&'a str>,
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
    pub screenspacefade: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub setbodygroup: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub shadowcastdist: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub skin: bool,
    #[serde(default)]
    pub solid: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub teamnum: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub texframeindex: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemAmmopackMedium<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub automaterialize: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub body: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(default)]
    pub fademaxdist: Option<u16>,
    #[serde(default)]
    pub fademindist: Option<i32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    #[serde(default)]
    pub modelscale: Option<f32>,
    #[serde(default)]
    pub onplayertouch: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub powerup_model: Option<&'a str>,
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
    pub screenspacefade: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub setbodygroup: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub shadowcastdist: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub skin: bool,
    #[serde(default)]
    pub solid: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub teamnum: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub texframeindex: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemAmmopackSmall<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub automaterialize: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub body: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(default)]
    pub fademaxdist: Option<u16>,
    #[serde(default)]
    pub fademindist: Option<i32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    #[serde(default)]
    pub modelscale: Option<f32>,
    pub origin: Vector,
    #[serde(default)]
    pub powerup_model: Option<&'a str>,
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
    pub screenspacefade: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub setbodygroup: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub shadowcastdist: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub skin: bool,
    #[serde(default)]
    pub solid: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub teamnum: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub texframeindex: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemHealthammokit {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    pub automaterialize: bool,
    pub fademindist: i32,
    pub fadescale: f32,
    pub modelscale: f32,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemHealthkitFull<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub automaterialize: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub body: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(default)]
    pub fademaxdist: Option<u16>,
    #[serde(default)]
    pub fademindist: Option<i32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    #[serde(default)]
    pub modelscale: Option<f32>,
    pub origin: Vector,
    #[serde(default)]
    pub powerup_model: Option<&'a str>,
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
    pub screenspacefade: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub shadowcastdist: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub skin: bool,
    #[serde(default)]
    pub solid: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub teamnum: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub texframeindex: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemHealthkitMedium<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub automaterialize: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub body: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(default)]
    pub fademaxdist: Option<u16>,
    #[serde(default)]
    pub fademindist: Option<i32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub mindxlevel: bool,
    #[serde(default)]
    pub modelscale: Option<f32>,
    #[serde(default)]
    pub onplayertouch: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub powerup_model: Option<&'a str>,
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
    pub screenspacefade: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub setbodygroup: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub shadowcastdist: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub skin: bool,
    #[serde(default)]
    pub solid: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub teamnum: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub texframeindex: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemHealthkitSmall<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub automaterialize: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub body: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(default)]
    pub fademaxdist: Option<u16>,
    #[serde(default)]
    pub fademindist: Option<i32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    #[serde(default)]
    pub modelscale: Option<f32>,
    #[serde(default)]
    pub onplayertouch: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub powerup_model: Option<&'a str>,
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
    pub screenspacefade: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub setbodygroup: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub shadowcastdist: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub skin: bool,
    #[serde(default)]
    pub solid: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub teamnum: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub texframeindex: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemTeamflag<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablebonefollowers: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub explodedamage: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub exploderadius: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub fademaxdist: bool,
    #[serde(default)]
    pub fademindist: Option<i32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    #[serde(default)]
    pub flag_icon: Option<&'a str>,
    #[serde(default)]
    pub flag_model: Option<&'a str>,
    #[serde(default)]
    pub flag_paper: Option<&'a str>,
    #[serde(default)]
    pub flag_trail: Option<&'a str>,
    #[serde(default)]
    pub gametype: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub health: bool,
    #[serde(default)]
    pub maxanimtime: Option<u8>,
    #[serde(default)]
    pub minanimtime: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub minhealthdmg: bool,
    #[serde(default)]
    pub model: Option<&'a str>,
    #[serde(default)]
    pub modelscale: Option<f32>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub neutraltype: bool,
    #[serde(default)]
    pub oncapteam1: Option<&'a str>,
    #[serde(default)]
    pub oncapteam2: Option<&'a str>,
    pub oncapture: &'a str,
    #[serde(default)]
    pub oncapture1: Option<&'a str>,
    #[serde(default)]
    pub ondrop: Option<&'a str>,
    #[serde(default)]
    pub ondrop1: Option<&'a str>,
    #[serde(default)]
    pub onpickup: Option<&'a str>,
    #[serde(default)]
    pub onpickup1: Option<&'a str>,
    #[serde(default)]
    pub onpickupteam1: Option<&'a str>,
    #[serde(default)]
    pub onpickupteam2: Option<&'a str>,
    #[serde(default)]
    pub onreturn: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub performancemode: bool,
    #[serde(default)]
    pub physdamagescale: Option<f32>,
    #[serde(default)]
    pub pointvalue: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub pressuredelay: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub randomanimation: bool,
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
    pub returnbetweenwaves: bool,
    pub returntime: u16,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub scoringtype: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub screenspacefade: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub shotclockmode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub skin: bool,
    #[serde(default)]
    pub solid: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub tags: Option<&'a str>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub teamnum: Option<u8>,
    #[serde(default)]
    pub trail_effect: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub visiblewhendisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct KeyframeRope<'a> {
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
    #[serde(default)]
    pub mindxlevel: Option<u8>,
    pub movespeed: u16,
    #[serde(default)]
    pub nextkey: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub nowind: bool,
    pub origin: Vector,
    #[serde(default)]
    pub positioninterpolator: Option<u8>,
    #[serde(default)]
    pub r#type: Option<u8>,
    pub ropematerial: &'a str,
    pub slack: u16,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub solid: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    pub subdiv: f32,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub texturescale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub usewind: bool,
    pub width: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Light<'a> {
    #[serde(default)]
    pub _constant_attn: Option<f32>,
    #[serde(default)]
    pub _distance: Option<u16>,
    #[serde(default)]
    pub _fifty_percent_distance: Option<i32>,
    #[serde(default)]
    pub _hardfalloff: Option<u16>,
    pub _light: &'a str,
    pub _lighthdr: &'a str,
    pub _lightscalehdr: f32,
    #[serde(default)]
    pub _linear_attn: Option<f32>,
    pub _quadratic_attn: f32,
    #[serde(default)]
    pub _zero_percent_distance: Option<i32>,
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub basisnormal: Option<Vector>,
    #[serde(default)]
    pub basisorigin: Option<Vector>,
    #[serde(default)]
    pub basisu: Option<Vector>,
    #[serde(default)]
    pub basisv: Option<Vector>,
    #[serde(default)]
    pub defaultstyle: Option<u8>,
    #[serde(default)]
    pub linedivider1: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub pattern: Option<&'a str>,
    #[serde(default)]
    pub spawnflags: Option<u8>,
    #[serde(default)]
    pub style: Option<u8>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub uv0: Option<Vector>,
    #[serde(default)]
    pub uv1: Option<Vector>,
    #[serde(default)]
    pub uv2: Option<Vector>,
    #[serde(default)]
    pub uv3: Option<Vector>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LightEnvironment<'a> {
    pub _ambient: &'a str,
    pub _ambienthdr: &'a str,
    pub _ambientscalehdr: u8,
    pub _light: LightColor,
    pub _lighthdr: &'a str,
    pub _lightscalehdr: f32,
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub defaultstyle: bool,
    pub origin: Vector,
    pub pitch: i32,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub style: bool,
    #[serde(default)]
    pub sunspreadangle: Option<f32>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LightSpot<'a> {
    pub _cone: u8,
    #[serde(default)]
    pub _cone2: Option<u8>,
    #[serde(default)]
    pub _constant_attn: Option<&'a str>,
    #[serde(default)]
    pub _distance: Option<u16>,
    pub _exponent: f32,
    #[serde(default)]
    pub _fifty_percent_distance: Option<u16>,
    #[serde(default)]
    pub _hardfalloff: Option<u16>,
    pub _inner_cone: u8,
    pub _light: &'a str,
    pub _lighthdr: &'a str,
    pub _lightscalehdr: f32,
    #[serde(default)]
    pub _linear_attn: Option<&'a str>,
    pub _quadratic_attn: u16,
    #[serde(default)]
    pub _zero_percent_distance: Option<u16>,
    pub angles: Angles,
    #[serde(default)]
    pub comment: Option<&'a str>,
    #[serde(default)]
    pub defaultstyle: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub fademaxdist: bool,
    #[serde(default)]
    pub fademindist: Option<i32>,
    #[serde(default)]
    pub linedivider1: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub pattern: Option<&'a str>,
    #[serde(default)]
    pub pitch: Option<f32>,
    #[serde(default)]
    pub spawnflags: Option<u8>,
    #[serde(default)]
    pub style: Option<u8>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicAuto<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub globalstate: Option<&'a str>,
    #[serde(default)]
    pub linedivider1: Option<&'a str>,
    #[serde(default)]
    pub onloadgame: Option<&'a str>,
    pub onmapspawn: &'a str,
    #[serde(default)]
    pub onmultinewmap: Option<&'a str>,
    pub onmultinewround: &'a str,
    #[serde(default)]
    pub onnewgame: Option<&'a str>,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicBranch<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub initialvalue: bool,
    pub onfalse: &'a str,
    pub ontrue: &'a str,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicBranchListener<'a> {
    pub branch01: &'a str,
    #[serde(default)]
    pub branch02: Option<&'a str>,
    #[serde(default)]
    pub branch03: Option<&'a str>,
    pub onallfalse: &'a str,
    pub onalltrue: &'a str,
    pub onmixed: &'a str,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicCase<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub case01: Option<&'a str>,
    #[serde(default)]
    pub case02: Option<&'a str>,
    #[serde(default)]
    pub case03: Option<&'a str>,
    #[serde(default)]
    pub case04: Option<&'a str>,
    #[serde(default)]
    pub case05: Option<&'a str>,
    #[serde(default)]
    pub case06: Option<&'a str>,
    #[serde(default)]
    pub case07: Option<&'a str>,
    #[serde(default)]
    pub case08: Option<&'a str>,
    #[serde(default)]
    pub case09: Option<&'a str>,
    #[serde(default)]
    pub case10: Option<u8>,
    #[serde(default)]
    pub case11: Option<u8>,
    #[serde(default)]
    pub case12: Option<u8>,
    #[serde(default)]
    pub case13: Option<u8>,
    #[serde(default)]
    pub case14: Option<u8>,
    #[serde(default)]
    pub case15: Option<u8>,
    #[serde(default)]
    pub case16: Option<u8>,
    pub oncase01: &'a str,
    pub oncase02: &'a str,
    pub oncase03: &'a str,
    #[serde(default)]
    pub oncase04: Option<&'a str>,
    #[serde(default)]
    pub oncase05: Option<&'a str>,
    #[serde(default)]
    pub oncase06: Option<&'a str>,
    #[serde(default)]
    pub oncase07: Option<&'a str>,
    #[serde(default)]
    pub oncase08: Option<&'a str>,
    #[serde(default)]
    pub oncase09: Option<&'a str>,
    #[serde(default)]
    pub oncase10: Option<&'a str>,
    #[serde(default)]
    pub oncase11: Option<&'a str>,
    #[serde(default)]
    pub oncase12: Option<&'a str>,
    #[serde(default)]
    pub oncase13: Option<&'a str>,
    #[serde(default)]
    pub oncase14: Option<&'a str>,
    #[serde(default)]
    pub oncase15: Option<&'a str>,
    #[serde(default)]
    pub oncase16: Option<&'a str>,
    #[serde(default)]
    pub ondefault: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub spawnflags: Option<u8>,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicCollisionPair<'a> {
    pub attach1: &'a str,
    pub attach2: &'a str,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicCompare<'a> {
    #[serde(default)]
    pub comparevalue: Option<f32>,
    #[serde(default)]
    pub initialvalue: Option<f32>,
    #[serde(default)]
    pub onequalto: Option<&'a str>,
    pub ongreaterthan: &'a str,
    #[serde(default)]
    pub onlessthan: Option<&'a str>,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicMeasureMovement<'a> {
    pub measurereference: &'a str,
    pub measuretarget: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub measuretype: bool,
    pub origin: Vector,
    #[serde(default)]
    pub spawnflags: Option<u8>,
    pub target: &'a str,
    pub targetname: &'a str,
    pub targetreference: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub targetscale: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicRelay<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(default)]
    pub onspawn: Option<&'a str>,
    pub ontrigger: &'a str,
    #[serde(default)]
    pub onuser1: Option<&'a str>,
    #[serde(default)]
    pub onuser2: Option<&'a str>,
    #[serde(default)]
    pub onuser4: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub spawnflags: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicScript<'a> {
    pub origin: Vector,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub thinkfunction: Option<&'a str>,
    pub vscripts: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicTimer<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub lowerrandombound: Option<f32>,
    pub ontimer: &'a str,
    #[serde(default)]
    pub ontimerhigh: Option<&'a str>,
    #[serde(default)]
    pub ontimerlow: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub refiretime: Option<f32>,
    #[serde(default)]
    pub spawnflags: Option<u16>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub upperrandombound: Option<f32>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub userandomtime: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MapobjCartDispenser<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub defaultupgrade: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(default)]
    pub helper_trigger_maxs: Option<Vector>,
    #[serde(default)]
    pub helper_trigger_mins: Option<Vector>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub solid: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub solidtoplayer: bool,
    pub spawnflags: u8,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub teamnum: u8,
    pub touch_trigger: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MaterialModifyControl {
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MathCounter<'a> {
    #[serde(default)]
    pub max: Option<u8>,
    #[serde(default)]
    pub min: Option<u8>,
    #[serde(default)]
    pub ongetvalue: Option<&'a str>,
    pub onhitmax: &'a str,
    #[serde(default)]
    pub onhitmin: Option<&'a str>,
    #[serde(default)]
    pub onuser1: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub outvalue: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub startvalue: Option<u8>,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MathRemap<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub in1: i32,
    pub in2: u8,
    pub origin: Vector,
    pub out1: i32,
    pub out2: u8,
    pub outvalue: &'a str,
    pub spawnflags: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Merasmus<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MomentaryRotButton<'a> {
    pub angles: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    pub distance: f32,
    pub model: &'a str,
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
    pub returnspeed: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub solidbsp: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub sounds: bool,
    pub spawnflags: u16,
    pub speed: f32,
    pub startdirection: &'a str,
    pub startposition: f32,
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
    #[serde(default)]
    pub mindxlevel: Option<u8>,
    pub movespeed: u16,
    #[serde(default)]
    pub nextkey: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub nowind: bool,
    pub origin: Vector,
    pub positioninterpolator: u8,
    #[serde(default)]
    pub r#type: Option<u8>,
    pub ropematerial: &'a str,
    pub slack: u16,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub solid: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    pub subdiv: f32,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub texturescale: f32,
    pub width: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ObjDispenser<'a> {
    pub angles: Angles,
    pub defaultupgrade: u8,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub targetname: &'a str,
    pub teamnum: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ObjSentrygun<'a> {
    pub angles: Angles,
    pub defaultupgrade: u8,
    pub ondestroyed: &'a str,
    pub origin: Vector,
    pub spawnflags: u8,
    pub targetname: &'a str,
    pub teamnum: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ObjTeleporter<'a> {
    pub angles: Angles,
    pub defaultupgrade: u8,
    pub matchingteleporter: &'a str,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub targetname: &'a str,
    pub teamnum: u8,
    pub teleportertype: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PasstimeLogic<'a> {
    pub ball_spawn_countdown: u8,
    #[serde(default)]
    pub max_pass_range: Option<u16>,
    #[serde(deserialize_with = "bool_from_int")]
    pub num_sections: bool,
    pub onballfree: &'a str,
    pub onballgetblu: &'a str,
    pub onballgetred: &'a str,
    pub onballpowerdown: &'a str,
    pub onballpowerup: &'a str,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PathCorner<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub speed: bool,
    pub targetname: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub wait: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub yaw_speed: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PathTrack<'a> {
    #[serde(default)]
    pub altpath: Option<&'a str>,
    pub angles: Angles,
    #[serde(default)]
    pub comment: Option<&'a str>,
    #[serde(default)]
    pub model: Option<&'a str>,
    #[serde(default)]
    pub onpass: Option<&'a str>,
    #[serde(default)]
    pub onteleport: Option<&'a str>,
    pub orientationtype: u8,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub radius: bool,
    #[serde(default)]
    pub sides: Option<&'a str>,
    #[serde(default)]
    pub spawnflags: Option<u8>,
    #[serde(default)]
    pub speed: Option<u16>,
    #[serde(default)]
    pub target: Option<&'a str>,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysConstraint<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub attach1: &'a str,
    pub attach2: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub forcelimit: bool,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub targetname: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub teleportfollowdistance: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub torquelimit: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysHinge<'a> {
    pub attach1: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub forcelimit: bool,
    pub hingeaxis: Vector,
    pub hingefriction: u8,
    pub maxsoundthreshold: u8,
    pub minsoundthreshold: u8,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub reversalsoundthresholdlarge: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub reversalsoundthresholdmedium: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub reversalsoundthresholdsmall: bool,
    pub spawnflags: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub systemloadscale: bool,
    pub targetname: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub teleportfollowdistance: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub torquelimit: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysRagdollmagnet<'a> {
    pub angles: Angles,
    pub axis: Vector,
    pub force: i32,
    pub origin: Vector,
    pub radius: u16,
    #[serde(default)]
    pub spawnflags: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysThruster<'a> {
    pub angles: Angles,
    pub attach1: &'a str,
    pub force: u32,
    pub forcetime: f32,
    pub origin: Vector,
    pub spawnflags: u8,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointCamera<'a> {
    pub angles: Angles,
    pub fogcolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub fogenable: bool,
    pub fogend: u16,
    #[serde(deserialize_with = "bool_from_int")]
    pub fogmaxdensity: bool,
    pub fogstart: i32,
    pub fov: u8,
    pub origin: Vector,
    pub parentname: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    pub targetname: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub usescreenaspectratio: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointClientcommand<'a> {
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointDevshotCamera<'a> {
    pub angles: Angles,
    #[serde(default)]
    pub cameraname: Option<&'a str>,
    pub fov: u8,
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointHurt<'a> {
    pub damage: u32,
    #[serde(deserialize_with = "bool_from_int")]
    pub damagedelay: bool,
    pub damageradius: u16,
    pub damagetarget: &'a str,
    pub damagetype: u8,
    pub origin: Vector,
    pub spawnflags: u8,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointPopulatorInterface<'a> {
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointPush<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    pub enabled: bool,
    #[serde(default)]
    pub influence_cone: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub inner_radius: bool,
    pub magnitude: i32,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub radius: u16,
    pub spawnflags: u8,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointServercommand<'a> {
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointSpotlight<'a> {
    #[serde(default)]
    pub _light: Option<LightColor>,
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub fademaxdist: bool,
    #[serde(default)]
    pub fademindist: Option<i32>,
    pub hdrcolorscale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub ignoresolid: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(default)]
    pub mindxlevel: Option<u8>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(default)]
    pub pitch: Option<f32>,
    pub renderamt: u8,
    pub rendercolor: &'a str,
    #[serde(default)]
    pub renderfx: Option<u8>,
    #[serde(default)]
    pub rendermode: Option<u8>,
    pub spawnflags: &'a str,
    pub spotlightlength: f32,
    pub spotlightwidth: f32,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointTeleport<'a> {
    pub angles: Angles,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    #[serde(default)]
    pub target: Option<&'a str>,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointTemplate<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub onentityspawned: Option<&'a str>,
    pub origin: Vector,
    pub spawnflags: u8,
    pub targetname: &'a str,
    #[serde(default)]
    pub template01: Option<&'a str>,
    #[serde(default)]
    pub template02: Option<&'a str>,
    #[serde(default)]
    pub template03: Option<&'a str>,
    #[serde(default)]
    pub template04: Option<&'a str>,
    #[serde(default)]
    pub template05: Option<&'a str>,
    #[serde(default)]
    pub template06: Option<&'a str>,
    #[serde(default)]
    pub template07: Option<&'a str>,
    #[serde(default)]
    pub template08: Option<&'a str>,
    #[serde(default)]
    pub template09: Option<&'a str>,
    #[serde(default)]
    pub template10: Option<&'a str>,
    #[serde(default)]
    pub template11: Option<&'a str>,
    #[serde(default)]
    pub template12: Option<&'a str>,
    #[serde(default)]
    pub template13: Option<&'a str>,
    #[serde(default)]
    pub template14: Option<&'a str>,
    #[serde(default)]
    pub template15: Option<&'a str>,
    #[serde(default)]
    pub template16: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointTesla<'a> {
    pub beamcount_max: u8,
    pub beamcount_min: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub interval_max: bool,
    pub interval_min: f32,
    pub lifetime_max: f32,
    pub lifetime_min: f32,
    pub m_color: Color,
    pub m_flradius: u16,
    pub m_soundname: &'a str,
    pub origin: Vector,
    pub targetname: &'a str,
    pub texture: &'a str,
    pub thick_max: u8,
    pub thick_min: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointViewcontrol<'a> {
    #[serde(default)]
    pub _frustum_far: Option<u16>,
    pub acceleration: u16,
    pub angles: Angles,
    pub deceleration: u16,
    pub origin: Vector,
    pub spawnflags: u8,
    #[serde(default)]
    pub speed: Option<u8>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub wait: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropDoorRotating<'a> {
    pub ajarangles: Angles,
    pub angles: Angles,
    pub axis: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    pub distance: u8,
    pub dmg: u16,
    #[serde(deserialize_with = "bool_from_int")]
    pub forceclosed: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub hardware: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub health: bool,
    pub model: &'a str,
    pub modelscale: f32,
    pub opendir: u8,
    pub origin: Vector,
    pub returndelay: i32,
    #[serde(deserialize_with = "bool_from_int")]
    pub skin: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnpos: bool,
    pub speed: u8,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropDynamic<'a> {
    pub angles: Angles,
    #[serde(default)]
    pub area_time_to_cap: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub body: bool,
    #[serde(default)]
    pub breakmodelmessage: Option<&'a str>,
    #[serde(default)]
    pub damagefilter: Option<&'a str>,
    #[serde(default)]
    pub defaultanim: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablebonefollowers: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableselfshadowing: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablevertexlighting: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(default)]
    pub explodedamage: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub explodemagnitude: bool,
    #[serde(default)]
    pub exploderadius: Option<u8>,
    #[serde(default)]
    pub fademaxdist: Option<f32>,
    pub fademindist: f32,
    pub fadescale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub generatelightmaps: bool,
    #[serde(default)]
    pub health: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub ignorenormals: bool,
    #[serde(default)]
    pub lightingorigin: Option<&'a str>,
    #[serde(default)]
    pub lightmapresolutionx: Option<u8>,
    #[serde(default)]
    pub lightmapresolutiony: Option<u8>,
    #[serde(default)]
    pub linedivider1: Option<&'a str>,
    #[serde(default)]
    pub linedivider10: Option<&'a str>,
    #[serde(default)]
    pub linedivider3: Option<&'a str>,
    #[serde(default)]
    pub linedivider4: Option<&'a str>,
    #[serde(default)]
    pub linedivider8: Option<&'a str>,
    pub maxanimtime: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub maxdxlevel: bool,
    pub minanimtime: &'a str,
    #[serde(default)]
    pub mindxlevel: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub minhealthdmg: bool,
    #[serde(default)]
    pub model: Option<&'a str>,
    #[serde(default)]
    pub modelscale: Option<&'a str>,
    #[serde(default)]
    pub onanimationbegun: Option<&'a str>,
    #[serde(default)]
    pub onanimationdone: Option<&'a str>,
    #[serde(default)]
    pub ontakedamage: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub performancemode: bool,
    #[serde(default)]
    pub physdamagescale: Option<f32>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub pressuredelay: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub randomanimation: bool,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(default)]
    pub rendermode: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub screenspacefade: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub setbodygroup: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub shadowcastdist: bool,
    #[serde(default)]
    pub skin: Option<&'a str>,
    pub solid: &'a str,
    #[serde(default)]
    pub spawnflags: Option<u16>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub texframeindex: bool,
    #[serde(default)]
    pub thinkfunction: Option<&'a str>,
    #[serde(default)]
    pub vscripts: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropDynamicOrnament<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablebonefollowers: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub explodedamage: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub exploderadius: bool,
    pub fademindist: i32,
    #[serde(deserialize_with = "bool_from_int")]
    pub fadescale: bool,
    pub initialowner: &'a str,
    pub maxanimtime: u8,
    pub minanimtime: u8,
    pub model: &'a str,
    pub modelscale: f32,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub performancemode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub pressuredelay: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub randomanimation: bool,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub rendermode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub setbodygroup: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub skin: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub solid: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropDynamicOverride<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub body: bool,
    #[serde(default)]
    pub defaultanim: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablebonefollowers: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub explodedamage: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub exploderadius: bool,
    #[serde(default)]
    pub fademaxdist: Option<f32>,
    pub fademindist: f32,
    pub fadescale: f32,
    #[serde(default)]
    pub health: Option<u8>,
    #[serde(default)]
    pub lightingorigin: Option<&'a str>,
    pub maxanimtime: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub maxdxlevel: bool,
    pub minanimtime: u8,
    #[serde(default)]
    pub mindxlevel: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub minhealthdmg: bool,
    pub model: &'a str,
    #[serde(default)]
    pub modelscale: Option<f32>,
    #[serde(default)]
    pub ontakedamage: Option<&'a str>,
    #[serde(default)]
    pub onuser1: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub performancemode: bool,
    #[serde(default)]
    pub physdamagescale: Option<f32>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub pressuredelay: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
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
    #[serde(default)]
    pub screenspacefade: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub setbodygroup: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub shadowcastdist: bool,
    #[serde(default)]
    pub skin: Option<u8>,
    pub solid: u8,
    #[serde(default)]
    pub spawnflags: Option<u16>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub texframeindex: bool,
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
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub explodedamage: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub exploderadius: bool,
    #[serde(default)]
    pub fademaxdist: Option<u16>,
    pub fademindist: i32,
    #[serde(deserialize_with = "bool_from_int")]
    pub fadescale: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub forcetoenablemotion: bool,
    pub inertiascale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub massscale: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(default)]
    pub mindxlevel: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    pub minhealthdmg: bool,
    pub model: &'a str,
    pub modelscale: f32,
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
    #[serde(default)]
    pub skin: bool,
    pub spawnflags: u16,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropPhysicsMultiplayer<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub body: bool,
    #[serde(default)]
    pub damagefilter: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub damagetoenablemotion: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub damagetype: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub explodedamage: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub explodemagnitude: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub exploderadius: bool,
    #[serde(default)]
    pub fademaxdist: Option<f32>,
    pub fademindist: f32,
    pub fadescale: f32,
    #[serde(default)]
    pub forcetoenablemotion: Option<f32>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub health: bool,
    pub inertiascale: f32,
    #[serde(default)]
    pub massscale: Option<f32>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(default)]
    pub mindxlevel: Option<u8>,
    #[serde(default)]
    pub minhealthdmg: Option<u16>,
    pub model: &'a str,
    #[serde(default)]
    pub modelscale: Option<f32>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub nodamageforces: bool,
    #[serde(default)]
    pub onawakened: Option<&'a str>,
    #[serde(default)]
    pub ontakedamage: Option<&'a str>,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub performancemode: bool,
    pub physdamagescale: f32,
    #[serde(default)]
    pub physicsmode: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
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
    #[serde(default)]
    pub screenspacefade: bool,
    #[serde(default)]
    pub shadowcastdist: Option<u8>,
    #[serde(default)]
    pub skin: Option<u8>,
    #[serde(default)]
    pub solid: Option<u8>,
    pub spawnflags: u16,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub texframeindex: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropPhysicsOverride<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub damagetoenablemotion: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub damagetype: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub explodedamage: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub exploderadius: bool,
    #[serde(default)]
    pub fademaxdist: Option<f32>,
    pub fademindist: f32,
    pub fadescale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub forcetoenablemotion: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub health: bool,
    pub inertiascale: f32,
    pub massscale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(default)]
    pub mindxlevel: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub minhealthdmg: bool,
    pub model: &'a str,
    #[serde(default)]
    pub modelscale: Option<f32>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub nodamageforces: bool,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub performancemode: bool,
    pub physdamagescale: f32,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
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
    #[serde(default)]
    pub shadowcastdist: bool,
    pub skin: u8,
    #[serde(default)]
    pub solid: Option<u8>,
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
    pub startdisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropSoccerBall<'a> {
    pub model: &'a str,
    pub modelscale: f32,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub skin: bool,
    pub targetname: &'a str,
    #[serde(default)]
    pub trigger_name: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ShadowControl<'a> {
    pub angles: Angles,
    pub color: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableallshadows: bool,
    pub distance: u16,
    pub origin: Vector,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct SkyCamera<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub fogblend: bool,
    pub fogcolor: &'a str,
    pub fogcolor2: &'a str,
    pub fogdir: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub fogenable: bool,
    pub fogend: f32,
    #[serde(default)]
    pub fogmaxdensity: Option<f32>,
    pub fogstart: f32,
    pub origin: Vector,
    pub scale: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub skin: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub use_angles: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub use_angles_for_sky: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TeamControlPoint<'a> {
    pub angles: Angles,
    #[serde(default)]
    pub area_time_to_cap: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub fademaxdist: bool,
    #[serde(default)]
    pub fademindist: Option<i32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    #[serde(default)]
    pub linedivider1: Option<&'a str>,
    #[serde(default)]
    pub model: Option<&'a str>,
    #[serde(default)]
    pub modelscale: Option<f32>,
    #[serde(default)]
    pub oncapreset: Option<&'a str>,
    #[serde(default)]
    pub oncapteam1: Option<&'a str>,
    pub oncapteam2: &'a str,
    #[serde(default)]
    pub onownerchangedtoteam2: Option<&'a str>,
    #[serde(default)]
    pub onroundstartownedbyteam1: Option<&'a str>,
    #[serde(default)]
    pub onroundstartownedbyteam2: Option<&'a str>,
    #[serde(default)]
    pub onunlocked: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(default)]
    pub point_default_owner: Option<u8>,
    #[serde(default)]
    pub point_group: Option<u8>,
    #[serde(default)]
    pub point_index: Option<u8>,
    pub point_printname: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub point_start_locked: bool,
    #[serde(default)]
    pub point_warn_on_cap: Option<u8>,
    pub point_warn_sound: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub random_owner_on_restart: bool,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(default)]
    pub rendermode: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub screenspacefade: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub shadowcastdist: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub skin: bool,
    #[serde(default)]
    pub solid: Option<u8>,
    pub spawnflags: u8,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    pub targetname: &'a str,
    pub team_bodygroup_0: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub team_bodygroup_2: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub team_bodygroup_3: bool,
    #[serde(default)]
    pub team_capsound_0: Option<&'a str>,
    #[serde(default)]
    pub team_capsound_2: Option<&'a str>,
    #[serde(default)]
    pub team_capsound_3: Option<&'a str>,
    pub team_icon_0: &'a str,
    pub team_icon_2: &'a str,
    pub team_icon_3: &'a str,
    pub team_model_0: &'a str,
    pub team_model_2: &'a str,
    pub team_model_3: &'a str,
    #[serde(default)]
    pub team_overlay_0: Option<&'a str>,
    #[serde(default)]
    pub team_overlay_2: Option<&'a str>,
    #[serde(default)]
    pub team_overlay_3: Option<&'a str>,
    #[serde(default)]
    pub team_previouspoint_2_0: Option<&'a str>,
    #[serde(default)]
    pub team_previouspoint_2_1: Option<&'a str>,
    #[serde(default)]
    pub team_previouspoint_2_2: Option<&'a str>,
    #[serde(default)]
    pub team_previouspoint_3_0: Option<&'a str>,
    #[serde(default)]
    pub team_previouspoint_3_1: Option<&'a str>,
    #[serde(default)]
    pub team_previouspoint_3_2: Option<&'a str>,
    #[serde(default)]
    pub team_timedpoints_2: Option<f32>,
    #[serde(default)]
    pub team_timedpoints_3: Option<f32>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TeamControlPointMaster<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub caplayout: Option<&'a str>,
    #[serde(default)]
    pub cpm_restrict_team_cap_win: Option<u8>,
    #[serde(default)]
    pub custom_position_x: Option<f32>,
    #[serde(default)]
    pub custom_position_y: Option<f32>,
    #[serde(default)]
    pub linedivider1: Option<&'a str>,
    #[serde(default)]
    pub onwonbyteam1: Option<&'a str>,
    #[serde(default)]
    pub onwonbyteam2: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub partial_cap_points_rate: Option<f32>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub play_all_rounds: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub score_style: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub switch_teams: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub team_base_icon_2: &'a str,
    pub team_base_icon_3: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TeamControlPointRound<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub cpr_cp_names: &'a str,
    #[serde(default)]
    pub cpr_priority: Option<u8>,
    #[serde(default)]
    pub cpr_restrict_team_cap_win: Option<u8>,
    #[serde(default)]
    pub onend: Option<&'a str>,
    pub onstart: &'a str,
    #[serde(default)]
    pub onwonbyteam1: Option<&'a str>,
    #[serde(default)]
    pub onwonbyteam2: Option<&'a str>,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TeamRoundTimer<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub auto_countdown: bool,
    #[serde(default)]
    pub max_length: Option<u16>,
    #[serde(default)]
    pub on10secremain: Option<&'a str>,
    #[serde(default)]
    pub on1minremain: Option<&'a str>,
    #[serde(default)]
    pub on1secremain: Option<&'a str>,
    #[serde(default)]
    pub on2minremain: Option<&'a str>,
    #[serde(default)]
    pub on2secremain: Option<&'a str>,
    #[serde(default)]
    pub on30secremain: Option<&'a str>,
    #[serde(default)]
    pub on3secremain: Option<&'a str>,
    #[serde(default)]
    pub on4secremain: Option<&'a str>,
    #[serde(default)]
    pub on5minremain: Option<&'a str>,
    #[serde(default)]
    pub on5secremain: Option<&'a str>,
    pub onfinished: &'a str,
    #[serde(default)]
    pub onroundstart: Option<&'a str>,
    pub onsetupfinished: &'a str,
    #[serde(default)]
    pub onsetupstart: Option<&'a str>,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub reset_time: bool,
    #[serde(default)]
    pub setup_length: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    pub show_in_hud: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub show_time_remaining: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub start_paused: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub teamnum: Option<u8>,
    #[serde(default)]
    pub thinkfunction: Option<&'a str>,
    pub timer_length: u16,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TeamTrainWatcher<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub env_spark_name: Option<&'a str>,
    #[serde(default)]
    pub goal_node: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub handle_train_movement: bool,
    pub hud_min_speed_level_1: u8,
    pub hud_min_speed_level_2: u8,
    pub hud_min_speed_level_3: u8,
    #[serde(default)]
    pub linked_cp_1: Option<&'a str>,
    #[serde(default)]
    pub linked_cp_2: Option<&'a str>,
    #[serde(default)]
    pub linked_cp_3: Option<&'a str>,
    #[serde(default)]
    pub linked_cp_4: Option<&'a str>,
    #[serde(default)]
    pub linked_cp_5: Option<&'a str>,
    #[serde(default)]
    pub linked_pathtrack_1: Option<&'a str>,
    #[serde(default)]
    pub linked_pathtrack_2: Option<&'a str>,
    #[serde(default)]
    pub linked_pathtrack_3: Option<&'a str>,
    #[serde(default)]
    pub linked_pathtrack_4: Option<&'a str>,
    #[serde(default)]
    pub linked_pathtrack_5: Option<&'a str>,
    #[serde(default)]
    pub ontrainstartrecede: Option<&'a str>,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub speed_forward_modifier: bool,
    #[serde(default)]
    pub start_node: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    pub targetname: &'a str,
    #[serde(default)]
    pub teamnum: Option<u8>,
    pub train: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub train_can_recede: bool,
    #[serde(default)]
    pub train_recede_time: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub uphill_speed_modifier: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfBaseMinigame {
    #[serde(deserialize_with = "bool_from_int")]
    pub inrandompool: bool,
    pub maxscore: u8,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub scoretype: bool,
    pub suddendeathtime: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfGamerules<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "bool_from_int")]
    pub ctf_overtime: bool,
    #[serde(default)]
    pub hud_type: Option<u8>,
    #[serde(default)]
    pub linedivider1: Option<&'a str>,
    #[serde(default)]
    pub onbluescorechanged: Option<&'a str>,
    #[serde(default)]
    pub onpowerupimbalancemeasuresover: Option<&'a str>,
    #[serde(default)]
    pub onpowerupimbalanceteam1: Option<&'a str>,
    #[serde(default)]
    pub onpowerupimbalanceteam2: Option<&'a str>,
    #[serde(default)]
    pub ontruceend: Option<&'a str>,
    #[serde(default)]
    pub ontrucestart: Option<&'a str>,
    #[serde(default)]
    pub onwonbyteam1: Option<&'a str>,
    #[serde(default)]
    pub onwonbyteam2: Option<&'a str>,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub ropes_holiday_lights_allowed: bool,
    pub targetname: &'a str,
    #[serde(default)]
    pub vscripts: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfGenericBomb<'a> {
    pub angles: Angles,
    pub damage: u16,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    pub explode_particle: &'a str,
    #[serde(default)]
    pub fademindist: Option<i32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub friendlyfire: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub health: bool,
    pub model: &'a str,
    pub modelscale: f32,
    pub ondetonate: &'a str,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub passactivator: bool,
    pub radius: u16,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "bool_from_int")]
    pub skin: bool,
    #[serde(default)]
    pub sound: Option<&'a str>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfGlow<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub glowcolor: LightColor,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub mode: bool,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    pub target: &'a str,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfHalloweenGiftSpawnLocation<'a> {
    pub angles: Angles,
    pub origin: Vector,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfHalloweenMinigame<'a> {
    pub bluespawn: &'a str,
    #[serde(default)]
    pub enemy_team_score_sound: Option<&'a str>,
    pub hud_res_file: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub inrandompool: bool,
    pub maxscore: u8,
    pub minigametype: u8,
    pub onallbluedead: &'a str,
    pub onallreddead: &'a str,
    pub onbluehitmaxscore: &'a str,
    pub onredhitmaxscore: &'a str,
    #[serde(default)]
    pub onsuddendeathstart: Option<&'a str>,
    pub onteleporttominigame: &'a str,
    pub origin: Vector,
    pub redspawn: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub scoretype: bool,
    pub suddendeathtime: i32,
    pub targetname: &'a str,
    #[serde(default)]
    pub your_team_score_sound: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfHalloweenMinigameFallingPlatforms<'a> {
    pub bluespawn: &'a str,
    pub hud_res_file: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub inrandompool: bool,
    pub maxscore: u8,
    pub minigametype: u8,
    pub onallbluedead: &'a str,
    pub onallreddead: &'a str,
    pub onteleporttominigame: &'a str,
    pub origin: Vector,
    pub outputremoveplatform: &'a str,
    pub outputsafeplatform: &'a str,
    pub redspawn: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub scoretype: bool,
    pub suddendeathtime: i32,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfHalloweenPickup<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    pub automaterialize: bool,
    pub fademindist: i32,
    pub origin: Vector,
    pub pickup_particle: &'a str,
    pub pickup_sound: &'a str,
    pub powerup_model: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub targetname: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub teamnum: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicArena<'a> {
    pub capenabledelay: u8,
    pub onarenaroundstart: &'a str,
    pub oncapenabled: &'a str,
    pub origin: Vector,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicCompetitive<'a> {
    pub onspawnroomdoorsshouldlock: &'a str,
    pub onspawnroomdoorsshouldunlock: &'a str,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicCpTimer<'a> {
    pub controlpoint: &'a str,
    pub oncountdownend: &'a str,
    pub oncountdownstart: &'a str,
    pub origin: Vector,
    pub targetname: &'a str,
    pub timer_length: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicHoliday<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub allowhaunting: bool,
    #[serde(default)]
    pub angles: Option<Angles>,
    pub holiday_type: u8,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub tauntinhell: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicHybridCtfCp {
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicKoth<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub linedivider1: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub timer_length: u16,
    pub unlock_point: u16,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicMannVsMachine<'a> {
    pub origin: Vector,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicMannpower {
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicMedieval<'a> {
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicMinigames<'a> {
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicMultipleEscort {
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicOnHoliday<'a> {
    #[serde(default)]
    pub isaprilfools: Option<&'a str>,
    #[serde(default)]
    pub ishalloween: Option<&'a str>,
    #[serde(default)]
    pub isnothing: Option<&'a str>,
    #[serde(default)]
    pub issmissmas: Option<&'a str>,
    #[serde(default)]
    pub istfbirthday: Option<&'a str>,
    #[serde(default)]
    pub isvalentines: Option<&'a str>,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicPlayerDestruction<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub blue_respawn_time: u8,
    pub finale_length: f32,
    pub flag_reset_delay: u8,
    pub heal_distance: u16,
    #[serde(default)]
    pub max_points: Option<u8>,
    pub min_points: u8,
    pub onbluehitmaxpoints: &'a str,
    pub onbluescorechanged: &'a str,
    #[serde(default)]
    pub oncountdowntimerexpired: Option<&'a str>,
    pub onredhitmaxpoints: &'a str,
    pub onredscorechanged: &'a str,
    pub origin: Vector,
    pub points_per_player: u8,
    #[serde(default)]
    pub prop_drop_sound: Option<&'a str>,
    pub prop_model_name: &'a str,
    #[serde(default)]
    pub prop_pickup_sound: Option<&'a str>,
    pub red_respawn_time: u8,
    pub res_file: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub score_interval: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub skin: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicRobotDestruction<'a> {
    pub blue_respawn_time: u8,
    pub finale_length: u8,
    pub loser_respawn_bonus_per_bot: f32,
    pub max_points: u16,
    pub onbluefirstflagstolen: &'a str,
    pub onblueflagstolen: &'a str,
    pub onbluehaspoints: &'a str,
    pub onbluehitmaxpoints: &'a str,
    pub onbluelastflagreturned: &'a str,
    pub onredfirstflagstolen: &'a str,
    pub onredflagstolen: &'a str,
    pub onredhaspoints: &'a str,
    pub onredhitmaxpoints: &'a str,
    pub onredlastflagreturned: &'a str,
    pub origin: Vector,
    pub red_respawn_time: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub score_interval: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicTrainingMode<'a> {
    pub onbotdied: &'a str,
    #[serde(default)]
    pub onbuildoutsidearea: Option<&'a str>,
    #[serde(default)]
    pub onplayerdetonatebuilding: Option<&'a str>,
    pub onplayerdied: &'a str,
    #[serde(default)]
    pub onplayerspawnasdemoman: Option<&'a str>,
    #[serde(default)]
    pub onplayerspawnasengineer: Option<&'a str>,
    pub onplayerspawnassoldier: &'a str,
    pub onplayerspawnasspy: &'a str,
    #[serde(default)]
    pub onplayerswappedtobuilding: Option<&'a str>,
    #[serde(default)]
    pub onplayerswappedtomelee: Option<&'a str>,
    #[serde(default)]
    pub onplayerswappedtopda: Option<&'a str>,
    #[serde(default)]
    pub onplayerswappedtoprimary: Option<&'a str>,
    #[serde(default)]
    pub onplayerswappedtosecondary: Option<&'a str>,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfPointNavInterface<'a> {
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfPointWeaponMimic<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    pub crits: bool,
    pub damage: u8,
    pub modelscale: f32,
    pub origin: Vector,
    pub parentname: &'a str,
    pub speedmax: u16,
    pub speedmin: u16,
    pub splashradius: u8,
    pub spreadangle: u8,
    pub targetname: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub weapontype: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfPumpkinBomb {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    pub automaterialize: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub fademaxdist: bool,
    pub fademindist: i32,
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfRobotDestructionRobotSpawn<'a> {
    pub angles: Angles,
    pub gibs: u8,
    pub health: u16,
    pub origin: Vector,
    pub r#type: u8,
    pub spawngroup: &'a str,
    pub startpath: &'a str,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfRobotDestructionSpawnGroup<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub group_number: u8,
    pub hud_icon: &'a str,
    pub origin: Vector,
    pub respawn_reduction_scale: f32,
    #[serde(default)]
    pub respawn_time: Option<u8>,
    pub targetname: &'a str,
    pub team_number: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfSpellPickup<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    pub automaterialize: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(default)]
    pub fademaxdist: Option<u16>,
    pub fademindist: i32,
    #[serde(default)]
    pub fadescale: Option<f32>,
    #[serde(default)]
    pub modelscale: Option<f32>,
    #[serde(default)]
    pub onplayertouch: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub powerup_model: Option<&'a str>,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(default)]
    pub rendermode: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub teamnum: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub tier: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfTeleportLocation<'a> {
    pub angles: Angles,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfZombie<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub effects: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub fademaxdist: bool,
    #[serde(default)]
    pub fademindist: Option<i32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    #[serde(default)]
    pub health: Option<u16>,
    #[serde(default)]
    pub max_health: Option<u16>,
    #[serde(default)]
    pub modelscale: Option<f32>,
    pub ondeath: &'a str,
    pub onuser1: &'a str,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub rendermode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub screenspacefade: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub shadowcastdist: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub skin: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    pub targetname: &'a str,
    pub teamnum: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfZombieSpawner<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub infinite_zombies: bool,
    pub max_zombies: u8,
    pub origin: Vector,
    pub targetname: &'a str,
    #[serde(default)]
    pub teamnum: Option<u8>,
    #[serde(default)]
    pub zombie_lifetime: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub zombie_scale: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub zombie_type: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TrainingAnnotation<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub display_text: &'a str,
    pub lifetime: i32,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub offset: bool,
    pub origin: Vector,
    pub targetname: &'a str,
    #[serde(default)]
    pub team_filter: Option<i32>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TrainingPropDynamic<'a> {
    pub angles: Angles,
    pub damagefilter: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub disableshadows: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub explodedamage: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub exploderadius: bool,
    pub fademindist: i32,
    #[serde(deserialize_with = "bool_from_int")]
    pub fadescale: bool,
    pub maxanimtime: u8,
    pub minanimtime: u8,
    pub model: &'a str,
    pub ontakedamage: &'a str,
    pub origin: Vector,
    pub parentname: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub performancemode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub pressuredelay: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub randomanimation: bool,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "bool_from_int")]
    pub renderfx: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub rendermode: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub setbodygroup: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub skin: bool,
    pub solid: u8,
    pub spawnflags: u16,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerAddOrRemoveTfPlayerAttributes<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    pub add_or_remove: bool,
    #[serde(default)]
    pub angles: Option<Angles>,
    pub attribute_name: &'a str,
    pub duration: i32,
    #[serde(default)]
    pub filtername: Option<&'a str>,
    pub model: &'a str,
    #[serde(default)]
    pub onstarttouch: Option<&'a str>,
    pub origin: Vector,
    pub spawnflags: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub value: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerAddTfPlayerCondition<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub condition: u8,
    pub duration: f32,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(default)]
    pub filtername: Option<&'a str>,
    pub model: &'a str,
    #[serde(default)]
    pub oncatapulted: Option<&'a str>,
    #[serde(default)]
    pub onendtouch: Option<&'a str>,
    #[serde(default)]
    pub onendtouchall: Option<&'a str>,
    #[serde(default)]
    pub onstarttouch: Option<&'a str>,
    #[serde(default)]
    pub onstarttouchall: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub solid: bool,
    pub spawnflags: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerApplyImpulse<'a> {
    #[serde(default)]
    pub filtername: Option<&'a str>,
    pub force: u16,
    pub impulse_dir: Vector,
    pub model: &'a str,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerBrush<'a> {
    pub angles: Angles,
    pub model: &'a str,
    pub onuse: &'a str,
    pub origin: Vector,
    pub spawnflags: u8,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerCaptureArea<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub area_cap_point: &'a str,
    pub area_time_to_cap: f32,
    #[serde(default)]
    pub filtername: Option<&'a str>,
    #[serde(default)]
    pub linedivider1: Option<&'a str>,
    #[serde(default)]
    pub linedivider2: Option<&'a str>,
    pub model: &'a str,
    #[serde(default)]
    pub onbreakcap: Option<&'a str>,
    #[serde(default)]
    pub onbreakteam1: Option<&'a str>,
    #[serde(default)]
    pub onbreakteam2: Option<&'a str>,
    #[serde(default)]
    pub oncapteam1: Option<&'a str>,
    pub oncapteam2: &'a str,
    #[serde(default)]
    pub onendcap: Option<&'a str>,
    #[serde(default)]
    pub onendtouchall: Option<&'a str>,
    #[serde(default)]
    pub onnumcapperschanged: Option<&'a str>,
    #[serde(default)]
    pub onnumcapperschanged2: Option<&'a str>,
    #[serde(default)]
    pub onstartcap: Option<&'a str>,
    #[serde(default)]
    pub onstartteam1: Option<&'a str>,
    #[serde(default)]
    pub onstartteam2: Option<&'a str>,
    #[serde(default)]
    pub onstarttouchall: Option<&'a str>,
    #[serde(default)]
    pub onuser1: Option<&'a str>,
    #[serde(default)]
    pub origin: Option<Vector>,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub solid: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    pub team_cancap_2: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub team_cancap_3: bool,
    pub team_numcap_2: u8,
    pub team_numcap_3: u8,
    #[serde(default)]
    pub team_spawn_2: Option<i32>,
    #[serde(default)]
    pub team_spawn_3: Option<i32>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub team_startcap_2: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub team_startcap_3: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerCatapult<'a> {
    pub airctrlsupressiontime: f32,
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "bool_from_int")]
    pub applyangularimpulse: bool,
    pub entryangletolerance: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub exactvelocitychoicetype: bool,
    #[serde(default)]
    pub filtername: Option<&'a str>,
    pub launchdirection: Vector,
    #[serde(default)]
    pub launchtarget: Option<&'a str>,
    pub lowerthreshold: f32,
    pub model: &'a str,
    #[serde(default)]
    pub oncapteam1_pd: Option<&'a str>,
    #[serde(default)]
    pub oncapteam2_pd: Option<&'a str>,
    pub oncatapulted: &'a str,
    pub onlyvelocitycheck: &'a str,
    #[serde(default)]
    pub onstarttouch: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub physicsspeed: i32,
    pub playerspeed: i32,
    pub spawnflags: u16,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub upperthreshold: f32,
    #[serde(deserialize_with = "bool_from_int")]
    pub useexactvelocity: bool,
    pub usethresholdcheck: &'a str,
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
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerHurt<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub damage: f32,
    pub damagecap: u32,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub damagemodel: bool,
    pub damagetype: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub effects: bool,
    #[serde(default)]
    pub filtername: Option<&'a str>,
    #[serde(default)]
    pub globalname: Option<&'a str>,
    #[serde(default)]
    pub model: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub nodmgforce: bool,
    #[serde(default)]
    pub onendtouch: Option<&'a str>,
    #[serde(default)]
    pub onendtouchall: Option<&'a str>,
    #[serde(default)]
    pub onhurtplayer: Option<&'a str>,
    #[serde(default)]
    pub onstarttouch: Option<&'a str>,
    #[serde(default)]
    pub onstarttouchall: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub spawnflags: u16,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub teamnum: bool,
    #[serde(default)]
    pub thinkfunction: Option<&'a str>,
    #[serde(default)]
    pub vscripts: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerIgnite<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub burn_duration: u8,
    pub damage_percent_per_second: u8,
    #[serde(default)]
    pub filtername: Option<&'a str>,
    pub model: &'a str,
    #[serde(default)]
    pub onstarttouch: Option<&'a str>,
    #[serde(default)]
    pub onstarttouchall: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub spawnflags: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerIgniteArrows<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub model: &'a str,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerLook<'a> {
    pub fieldofview: f32,
    pub looktime: f32,
    pub model: &'a str,
    pub ontrigger: &'a str,
    pub origin: Vector,
    pub spawnflags: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub target: &'a str,
    pub targetname: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub timeout: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerMultiple<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub duration: Option<u8>,
    #[serde(default)]
    pub filtername: Option<&'a str>,
    pub model: &'a str,
    #[serde(default)]
    pub onendtouch: Option<&'a str>,
    #[serde(default)]
    pub onendtouchall: Option<&'a str>,
    #[serde(default)]
    pub onnottouching: Option<&'a str>,
    #[serde(default)]
    pub onstarttouch: Option<&'a str>,
    #[serde(default)]
    pub onstarttouchall: Option<&'a str>,
    #[serde(default)]
    pub ontouching: Option<&'a str>,
    #[serde(default)]
    pub ontrigger: Option<&'a str>,
    #[serde(default)]
    pub onuser1: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub solid: bool,
    pub spawnflags: u16,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub wait: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerOnce<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub filtername: Option<&'a str>,
    pub model: &'a str,
    pub onstarttouch: &'a str,
    pub ontrigger: &'a str,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub spawnflags: u16,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerPasstimeBall<'a> {
    pub model: &'a str,
    pub onballenter: &'a str,
    pub onballexit: &'a str,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerPlayerRespawnOverride<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub filtername: Option<&'a str>,
    pub model: &'a str,
    pub origin: Vector,
    pub respawntime: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub wait: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerPush<'a> {
    #[serde(deserialize_with = "bool_from_int")]
    pub alternateticksfix: bool,
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub filtername: Option<&'a str>,
    #[serde(default)]
    pub model: Option<&'a str>,
    #[serde(default)]
    pub onendtouch: Option<&'a str>,
    #[serde(default)]
    pub onstarttouch: Option<&'a str>,
    #[serde(default)]
    pub onstarttouchall: Option<&'a str>,
    #[serde(default)]
    pub onuser1: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub pushdir: Vector,
    pub spawnflags: u16,
    pub speed: u16,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub teamnum: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerRdVaultTrigger<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub model: &'a str,
    #[serde(default)]
    pub onpointsendstealing: Option<&'a str>,
    #[serde(default)]
    pub onpointsstartstealing: Option<&'a str>,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub teamnum: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerRemoveTfPlayerCondition<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub condition: u8,
    pub model: &'a str,
    #[serde(default)]
    pub onstarttouch: Option<&'a str>,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
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
    #[serde(default)]
    pub onendtouch: Option<&'a str>,
    #[serde(default)]
    pub onstarttouch: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub soundscape: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerStun<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub filtername: Option<&'a str>,
    pub model: &'a str,
    pub move_speed_reduction: f32,
    #[serde(default)]
    pub onfullyclosed: Option<&'a str>,
    #[serde(default)]
    pub onstarttouch: Option<&'a str>,
    #[serde(default)]
    pub onstarttouchall: Option<&'a str>,
    #[serde(default)]
    pub onstunplayer: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub spawnflags: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub stun_duration: u16,
    #[serde(deserialize_with = "bool_from_int")]
    pub stun_effects: bool,
    pub stun_type: u8,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub trigger_delay: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerTeleport<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub filtername: Option<&'a str>,
    #[serde(default)]
    pub landmark: Option<&'a str>,
    pub model: &'a str,
    #[serde(default)]
    pub oncapteam1_pd: Option<&'a str>,
    #[serde(default)]
    pub oncapteam2_pd: Option<&'a str>,
    #[serde(default)]
    pub onendtouch: Option<&'a str>,
    pub onstarttouch: &'a str,
    #[serde(default)]
    pub onstarttouchall: Option<&'a str>,
    #[serde(default)]
    pub ontouching: Option<&'a str>,
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
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerTeleportRelative<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub model: &'a str,
    pub origin: Vector,
    #[serde(deserialize_with = "bool_from_int")]
    pub spawnflags: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub targetname: &'a str,
    pub teleportoffset: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerTimerDoor<'a> {
    pub area_cap_point: &'a str,
    pub area_time_to_cap: u8,
    pub door_name: &'a str,
    pub filtername: &'a str,
    pub model: &'a str,
    #[serde(default)]
    pub onbreakteam2: Option<&'a str>,
    pub oncapteam2: &'a str,
    pub onendtouchall: &'a str,
    pub onstarttouchall: &'a str,
    pub origin: Vector,
    pub spawnflags: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub targetname: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    pub team_cancap_2: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub team_cancap_3: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub team_numcap_2: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub team_numcap_3: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub team_startcap_2: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub team_startcap_3: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerVphysicsMotion<'a> {
    pub filtername: &'a str,
    pub model: &'a str,
    pub origin: Vector,
    pub parentname: &'a str,
    pub particletrailendsize: u8,
    pub particletraillifetime: u8,
    pub particletrailstartsize: u8,
    pub setadditionalairdensity: u16,
    pub setangvelocitylimit: f32,
    pub setangvelocityscale: f32,
    pub setgravityscale: f32,
    pub setlinearforce: f32,
    pub setlinearforceangles: Angles,
    pub setvelocitylimit: f32,
    pub setvelocitylimitdelta: u32,
    pub setvelocityscale: f32,
    pub spawnflags: u16,
    #[serde(deserialize_with = "bool_from_int")]
    pub startdisabled: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WaterLodControl<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub cheapwaterenddistance: u32,
    pub cheapwaterstartdistance: u32,
    #[serde(default)]
    pub origin: Option<Vector>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WheelOfDoom<'a> {
    pub angles: Angles,
    pub effect_duration: u8,
    #[serde(deserialize_with = "bool_from_int")]
    pub has_spiral: bool,
    pub oneffectapplied: &'a str,
    pub oneffectexpired: &'a str,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Worldspawn<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(default)]
    pub author: Option<&'a str>,
    #[serde(default)]
    pub chaptertitle: Option<&'a str>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub coldworld: bool,
    #[serde(default)]
    pub comment: Option<&'a str>,
    pub detailmaterial: &'a str,
    pub detailvbsp: &'a str,
    #[serde(default)]
    pub fogcolor: Option<Color>,
    #[serde(default)]
    pub fogcolor2: Option<Color>,
    #[serde(default)]
    pub fogdir: Option<Vector>,
    #[serde(default)]
    pub fogend: Option<f32>,
    #[serde(default)]
    pub fogstart: Option<f32>,
    pub mapversion: u16,
    #[serde(default)]
    pub maxoccludeearea: Option<f32>,
    pub maxpropscreenwidth: i32,
    #[serde(default)]
    pub maxrange: Option<u16>,
    #[serde(default)]
    pub message: Option<&'a str>,
    #[serde(default)]
    pub minoccluderarea: Option<f32>,
    #[serde(default)]
    pub minoccluderarea_x360: Option<u8>,
    #[serde(default)]
    pub minpropscreenwidth: Option<u8>,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub newunit: bool,
    pub skyname: &'a str,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub sounds: bool,
    #[serde(deserialize_with = "bool_from_int")]
    #[serde(default)]
    pub spawnflags: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub world_maxs: Vector,
    pub world_mins: Vector,
}
