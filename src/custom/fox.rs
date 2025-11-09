#![allow(improper_ctypes_definitions)]
use super::*;

//Fox Shine ACMD
unsafe extern "C" fn fox_shine_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 1, 100, 80, 0, 8.0, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
}

unsafe extern "C" fn fox_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[0x16].get_i32();
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    let reflector_air_stop_y_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("reflector_air_stop_y_frame"));
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_LANDING) {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_INT_STOP_Y_FRAME);
        if situation_kind == *SITUATION_KIND_AIR {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
        }
    }
    else {
        WorkModule::set_int(fighter.module_accessor, reflector_air_stop_y_frame, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    }
    else {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_BUTTON);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_NEXT);
    }
    fox_special_lw_motion_helper(fighter, "special_lw_start", "special_air_lw_start", "special_lw_start_l", "special_air_lw_start_l");
    fighter.sub_shift_status_main(L2CValue::Ptr(fox_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn fox_special_lw_motion_helper(fighter: &mut L2CFighterCommon, ground_motion: &str, air_motion: &str, ground_flip_motion: &str, air_flip_motion: &str) {
    let situation_kind = fighter.global_table[0x16].get_i32();
    let lr = PostureModule::lr(fighter.module_accessor);
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE) {
            if lr == -1.0 {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new(air_flip_motion), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new(air_motion), -1.0, 1.0, 0.0, false, false);
            }
        }
        if lr == -1.0 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new(air_flip_motion), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new(air_motion), 0.0, 1.0, false, 0.0, false, false);
        }
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE) {
            if lr == -1.0 {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new(ground_flip_motion), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new(ground_motion), -1.0, 1.0, 0.0, false, false);
            }
        }
        if lr == -1.0 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new(ground_flip_motion), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new(ground_motion), 0.0, 1.0, false, 0.0, false, false);
        }
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_CONTINUE);
    }
}

unsafe extern "C" fn fox_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[0xE].get_f32();
    let situation_kind = fighter.global_table[0x16].get_i32();
    if current_frame > 2.0 {
        if situation_kind == *SITUATION_KIND_GROUND {
            if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() || fighter.sub_transition_group_check_ground_jump().get_bool() {
                return 0.into();
            }
        }
        else {
            if fighter.sub_transition_group_check_air_jump_aerial().get_bool() {
                return 0.into();
            }
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) && StatusModule::is_situation_changed(fighter.module_accessor) {
        fox_special_lw_motion_helper(fighter, "special_lw_start", "special_air_lw_start", "special_lw_start_l", "special_air_lw_start_l");
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn fox_special_lw_loop_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[0x16].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    }
    else {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_BUTTON);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_NEXT);
    }
    fox_special_lw_motion_helper(fighter, "special_lw_loop", "special_air_lw_loop", "special_lw_loop_l", "special_air_lw_loop_l");
    fighter.sub_shift_status_main(L2CValue::Ptr(fox_special_lw_loop_main_loop as *const () as _))
}

unsafe extern "C" fn fox_special_lw_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[0xE].get_f32();
    let situation_kind = fighter.global_table[0x16].get_i32();
    let motion_frame = MotionModule::motion_kind(fighter.module_accessor) as i32;
    let current_hitstop_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME);
    let reflector_init_keep_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("reflector_init_keep_frame"));
    let reflector_attack_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("reflector_attack_frame"));
    if current_hitstop_frame <= 0 {
        if situation_kind == *SITUATION_KIND_GROUND {
            if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() || fighter.sub_transition_group_check_ground_jump().get_bool() {
                return 0.into();
            }
        }
        else {
            if fighter.sub_transition_group_check_air_jump_aerial().get_bool() {
                return 0.into();
            }
        }
    }
    if current_frame > reflector_init_keep_frame || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_HIT_TO_RESTART) {
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END.into(), false.into());
            return 0.into();
        }
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_SET_ATTACK) {
        if reflector_attack_frame <= motion_frame {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2ad70b3640));
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_REFLECTOR_STATUS_WORK_ID_FLAG_SET_ATTACK);
        }
    }
    else {
        AttackModule::clear_all(fighter.module_accessor);
    }
    if !StatusModule::is_changing(fighter.module_accessor) && StatusModule::is_situation_changed(fighter.module_accessor) {
        fox_special_lw_motion_helper(fighter, "special_lw_loop", "special_air_lw_loop", "special_lw_loop_l", "special_air_lw_loop_l");
    }
    0.into()
}

pub fn install() {
    Agent::new("fox")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallwstart", fox_shine_acmd, Low)
    .game_acmd("game_specialairlwstart", fox_shine_acmd, Low)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, fox_special_lw_main_status)
    .status(Main, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP, fox_special_lw_loop_main_status)
    .install()
    ;
}