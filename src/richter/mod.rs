use smash::phx::Hash40;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::*;
use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;
use crate::commonfuncs::*;

pub static mut RICHTER_SPECIAL_HI : [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_RICHTER )]
fn richter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH {
            RICHTER_SPECIAL_HI[entry_id(fighter.module_accessor)] = false;
        }
        if sv_information::is_ready_go() == false {
            RICHTER_SPECIAL_HI[entry_id(fighter.module_accessor)] = false;
        }

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_LANDING {
            RICHTER_SPECIAL_HI[entry_id(fighter.module_accessor)] = true;
        }
        else if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
        || is_damage_check(fighter.module_accessor) {
            RICHTER_SPECIAL_HI[entry_id(fighter.module_accessor)] = false;
        }
    }
}

#[acmd_script( agent = "richter", scripts = ["game_specialn", "game_specialairn"], category = ACMD_GAME, low_priority )]
unsafe fn richter_nspecial(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 1.3);
    if macros::is_excute(fighter){
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, false, 0);
    }
    frame(fighter.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
    }
}

#[acmd_script( agent = "richter", scripts = ["game_specials1", "game_specialairs1"], category = ACMD_GAME, low_priority )]
unsafe fn richter_sspecial(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.56);
    if macros::is_excute(fighter){
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_CROSS, false, 0);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_S_FLAG_FALL);
    }
    frame(fighter.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_CROSS, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_S_FLAG_FALL);
    }
}

#[acmd_script( agent = "richter", scripts = ["game_speciallw", "game_specialairlw"], category = ACMD_GAME, low_priority )]
unsafe fn richter_dspecial(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 1.3);
    if macros::is_excute(fighter){
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_LW_FLAG_GENERATE_HOLYWATER);
    }
    frame(fighter.lua_state_agent, 18.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_LW_FLAG_SHOOT_HOLYWATER);
    }
}

#[acmd_script( agent = "richter", scripts = ["game_specialhi", "game_specialairhi"], category = ACMD_GAME, low_priority )]
unsafe fn richter_uspecial(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 95, 100, 155, 0, 8.5, 0.0, 9.5, 10.5, None, None, None, 1.4, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 80, 0, 6.0, 0.0, 26.0, 9.5, Some(0.0), Some(6.0), Some(7.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
        smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 6.0, 61, 86, 0, 85, 6.0, 0.0, 26.0, 9.5, Some(0.0), Some(6.0), Some(7.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_HI_FLAG_MOVE);
    }
    frame(fighter.lua_state_agent, 47.0);
    if macros::is_excute(fighter) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        richter_frame
    );
    smashline::install_acmd_scripts!(
        richter_nspecial,
        richter_sspecial,
        richter_dspecial,
        richter_uspecial
    );
}