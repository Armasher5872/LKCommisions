use super::*;

unsafe extern "C" fn sonic_special_s_hold_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_hold"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(fighter.module_accessor) {
        sonic_special_s_hold_substatus(fighter);
    }
    fighter.global_table[0x14].assign(&L2CValue::Ptr(sonic_special_s_hold_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(sonic_special_s_hold_main_loop as *const () as _))
}

unsafe extern "C" fn sonic_special_s_hold_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    let special_s_advance_counter = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_HOLD_WORK_FLOAT_ADVANCE_COUNTER);
    let special_s_hold_frame_coeff = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_hold_frame_coeff"));
    let special_s_hold_frame_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_hold_frame_max"));
    let advance = special_s_advance_counter+(1.0/special_s_hold_frame_coeff);
    WorkModule::set_float(fighter.module_accessor, advance, *FIGHTER_SONIC_STATUS_SPECIAL_S_HOLD_WORK_FLOAT_ADVANCE_COUNTER);
    if special_s_hold_frame_max < advance as i32 {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_HOLD_FLAG_MAX_CHARGE);
        fighter.global_table[0x14].assign(&L2CValue::I32(0));
        fighter.global_table[0x15].assign(&L2CValue::I32(0));
    }
    0.into()
}

unsafe extern "C" fn sonic_special_s_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[0x16].get_i32();
    let special_s_advance_counter = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_HOLD_WORK_FLOAT_ADVANCE_COUNTER) as i32;
    let limit_counter = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_HOLD_INT_LIMIT_COUNTER);
    let special_s_hold_frame_min = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_hold_frame_min"));
    let special_s_hold_frame_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_hold_frame_max"));
    let special_s_hold_frame_limit = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_hold_frame_limit"));
    let mut lstack_60 = false;
    let bvar4;
    if special_s_hold_frame_min > special_s_advance_counter {
        bvar4 = false;
    }
    else {
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            bvar4 = ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SMASH);
        }
        else {
            bvar4 = false;
        }
    }
    if bvar4 {
        lstack_60 = true;
    }
    if special_s_hold_frame_limit <= limit_counter {
        lstack_60 = true;
    }
    if lstack_60 {
        fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH.into(), true.into());
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() || fighter.sub_transition_group_check_ground_jump().get_bool() {
            fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD_JUMP.into(), true.into());
            return 1.into();
        }
        if fighter.sub_transition_group_check_ground_guard().get_bool() {
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_ON.into(), false.into());
            return 1.into();
        }
    }
    if fun_7100015020(fighter).get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) && StatusModule::is_situation_changed(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_SONIC_SPECIAL_S_AIR_HOLD);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_SONIC_SPECIAL_S_HOLD);
        }
        if special_s_hold_frame_max < special_s_advance_counter {
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
    }
    0.into()
}

unsafe extern "C" fn fun_7100015020(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status_kind = fighter.global_table[0x9].get_i32();
    let status_kind = fighter.global_table[0xA].get_i32();
    let current_frame = fighter.global_table[0xE].get_i32();
    let situation_kind = fighter.global_table[0x16].get_i32();
    let cmd_cat1 = fighter.global_table[0x20].get_i32();
    let current_jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    let max_jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
    let mut ret = 0;
    if situation_kind != *SITUATION_KIND_GROUND {
        if !fighter.sub_transition_group_check_air_jump_aerial().get_bool() {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                if current_jump_count < max_jump_count {
                    if status_kind == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_HOLD {
                        if current_frame == 0 {
                            KineticModule::clear_speed_all(fighter.module_accessor);
                        }
                        fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), true.into());
                    }
                }
            }
            ret = 0;
        }
    }
    if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON != 0 {
        fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPIN_JUMP.into(), true.into());
    }
    else {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPIN_JUMP.into(), true.into());
        }
        else {
            if prev_status_kind == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH {
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPIN_JUMP.into(), true.into());
                    ret = 1;
                }
            }
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0 {
                if ControlModule::is_enable_flick_jump(fighter.module_accessor) {
                    fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPIN_JUMP.into(), true.into());
                    ret = 1;
                }
            }
        }
    }
    ret.into()
}

pub fn install() {
    Agent::new("sonic")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD, sonic_special_s_hold_main_status)
    .install()
    ;
}