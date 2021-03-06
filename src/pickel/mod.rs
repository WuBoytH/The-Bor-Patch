use smash::phx::Hash40;
use smash::lua2cpp::{L2CAgentBase/*, L2CFighterCommon*/};
use smash::app::*;
use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;
// use crate::system::IS_FUNNY;
// use crate::commonfuncs::*;

#[acmd_script( agent = "pickel_forge", script = "game_wait", category = ACMD_GAME, low_priority )]
unsafe fn pickel_forge_wait(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 70, 78, 0, 58, 6.4, 0.0, 4.6, 0.0, Some(0.0), Some(4.8), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 5.0, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 10, true, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new_raw(0x1778c298b0), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
}

#[acmd_script( agent = "pickel", script = "game_specialsride", category = ACMD_GAME, low_priority )]
unsafe fn pickel_sspecialride(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, 60192);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 350, 100, 30, 10, 3.0, 0.0, 8.0, 4.5, Some(0.0), Some(4.0), Some(4.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    macros::FT_MOTION_RATE(fighter, 1.5);
    frame(fighter.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "pickel", script = "game_specialairsride", category = ACMD_GAME, low_priority )]
unsafe fn pickel_sspecialrideair(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, 60192);
    }
    macros::FT_MOTION_RATE(fighter, 1.5);
    frame(fighter.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}

pub fn install() {
    smashline::install_acmd_scripts!(
        pickel_forge_wait,
        pickel_sspecialride,
        pickel_sspecialrideair
    );
}