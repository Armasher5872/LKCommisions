use {
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                frame,
                wait
            },
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::Hash40
    },
    smash_script::{
        macros::*,
        *
    },
    smashline::{
        Priority::*,
        *
    },
};

mod cloud;
mod donkey;
mod fox;
mod luigi;
mod mario;
mod marth;
mod miifighter;
mod sheik;
mod sonic;

pub fn install() {
    cloud::install();
    donkey::install();
    fox::install();
    luigi::install();
    mario::install();
    marth::install();
    miifighter::install();
    sheik::install();
    sonic::install();
}