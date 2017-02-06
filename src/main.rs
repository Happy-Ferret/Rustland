#![feature(box_syntax, box_patterns)]
#[allow(inaccessible_extern_crate)]

use std::sync::RwLock;
use std::io::Write;
use std::process::Command;

#[macro_use]
pub extern crate lazy_static;

pub extern crate rustwlc;
use rustwlc::*;

mod layout;
mod handlers;
mod definitions;

pub mod wmstate;
use wmstate::WM_STATE;
use handlers::input::*;
use layout::arrangement::*;

/*
1. Get find_first_empty_container to work
2. Set up workspaces to hide views that belong to unactivated workspaces
3. Setup new fmt (debugging ) implementations and move over from tree
*/

fn main() {
    callback::compositor_ready(compositor_ready);
    callback::view_created(layout::window::on_view_created);
    callback::view_destroyed(layout::window::on_view_destroyed);
    callback::view_focus(layout::window::on_view_focus);
    callback::view_request_move(layout::window::on_view_request_move);
    callback::view_request_resize(layout::window::on_view_request_resize);
    callback::output_resolution(layout::on_output_resolution);

    // The default log handler will print wlc logs to stdout
    rustwlc::log_set_default_handler();
    let run_fn = rustwlc::init().expect("Unable to initialize!");

    run_fn();
    
}

pub extern fn compositor_ready()
{
    let root = WlcView::root();
    
    if let Ok(mut wm_state) =  WM_STATE.lock()
    {
        wm_state.input_dev = InputDevice::init();
        
        wm_state.tree.arrange();
    }
}