// use smash::phx::Hash40;
use smash::hash40;
use smash::lua2cpp::L2CAgentBase;
use smash::app::*;
use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;
use crate::system::IS_FUNNY;
// use crate::globals::*;
use crate::commonfuncs::*;

#[acmd_script( agent = "diddy", scripts = ["game_speciallw", "game_specialairlw"], category = ACMD_GAME, low_priority )]
unsafe fn diddy_dspecial(fighter: &mut L2CAgentBase) {
    let rng : i32;
    let randitem : i32;
    if IS_FUNNY[entry_id(fighter.module_accessor)] {
        rng = sv_math::rand(hash40("fighter"), 100);
        if rng > 20 && rng <= 45 {
            randitem = *ITEM_KIND_SENSORBOMB;
        }
        else if rng > 45 && rng <= 60 {
            randitem = *ITEM_KIND_UNIRA;
        }
        else if rng > 60 && rng <= 74 {
            randitem = *ITEM_KIND_HAMMERHEAD;
        }
        else if rng > 74 && rng <= 86 {
            randitem = *ITEM_KIND_BUMPER;
        }
        else if rng > 86 && rng <= 96 {
            randitem = *ITEM_KIND_POWBLOCK;
        }
        else if rng > 96 && rng <= 98 {
            randitem = *ITEM_KIND_SMASHBOMB;
        }
        else if rng > 98 {
            randitem = *ITEM_KIND_SMASHBALL;
        }
        else {
            randitem = *ITEM_KIND_BOMBHEI;
        }
    }
    else {
        rng = 101;
        randitem = 0;
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        if rng == 101 {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_ITEM_BANANA, false, 0);
        }
        else {
            ItemModule::have_item(fighter.module_accessor, ItemKind(randitem), 0, 0, false, false);
        }
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        if rng == 101 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_SPECIAL_LW_FLAG_ITEM_THROW);
        }
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        if rng != 101 {
            let angle : f32;
            let power : f32;
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                angle = 40.0;
                power = 2.0;
            }
            else {
                angle = 75.0;
                power = 2.2;
            }
            ItemModule::throw_item(fighter.module_accessor, angle, power, 1.0, 0, true, 0.0);
        }
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        if rng == 101 {
            ArticleModule::shoot(fighter.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_ITEM_BANANA, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        diddy_dspecial
    );
}