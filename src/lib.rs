use windows::{core::w};
use windows::core::s;
use windows::Win32::System::LibraryLoader::{LoadLibraryW, GetProcAddress, GetModuleHandleW, GetModuleFileNameW};
use std::path::Path;
use ini::Ini;
use std::collections::HashMap;

#[repr(C)]
struct XinputGamepad
{
    wbuttons: u16,
    blefttrigger: u8,
    brighttrigger: u8,
    sthumblx: i16,
    sthumbly: i16,
    sthumbrx: i16,
    sthumbry: i16,
}

#[repr(C)]
struct XinputState
{
    dw_packet_number: i32,
    gamepad: XinputGamepad
}



use retour::static_detour;
#[unsafe(no_mangle)]
pub extern "C" fn DllMain(_hmodule: u64, reason: u32) -> bool {
    if reason != 1 {
        return true;
    }

    let xinput_module= unsafe{LoadLibraryW(w!("xinput1_4.dll"))}.unwrap();
    let address = unsafe{GetProcAddress(xinput_module, s!("XInputGetState"))}.unwrap();

    let h_module = Some(unsafe{GetModuleHandleW(w!("er_keycustomizer.dll")).unwrap()});
    let mut buffer  = [0u16; 260];
    let len = unsafe{GetModuleFileNameW(h_module, &mut buffer)};
    let dll_path = String::from_utf16_lossy(&buffer[..len as usize]);
    let dll_path = Path::new(&dll_path);
    let config_path = dll_path.parent().unwrap().join("er_key_customization.config.ini");

    static_detour! {
        static XINPUTGETSTATE_HOOK: unsafe extern "C" fn(i32, *mut XinputState) -> u32 ;
    }
    unsafe{
        XINPUTGETSTATE_HOOK.initialize(std::mem::transmute::<u64, unsafe extern "C" fn(i32, *mut XinputState) -> u32>(address as u64), 
        move |dwuserindex, pstate|
        {
            let return_value = XINPUTGETSTATE_HOOK.call(dwuserindex, pstate);

        if !Path::new(&config_path).exists() {
            let mut conf = Ini::new();
            conf.with_section(None::<String>)
                .set("encoding", "utf-8");
            conf.with_section(Some("Settings"))
                .set("X Button", "X Button")
                .set("Square Button", "Square Button")
                .set("Triangle Button", "Triangle Button")
                .set("Circle Button", "Circle Button")
                .set("DPad Up", "DPad Up")
                .set("DPad Down", "DPad Down")
                .set("DPad Left", "DPad Left")
                .set("DPad Right", "DPad Right")
                .set("Start", "Start")
                .set("Back", "Back")
                .set("Left Thumb", "Left Thumb")
                .set("Right Thumb", "Right Thumb")
                .set("Left Shoulder", "Left Shoulder")
                .set("Right Shoulder", "Right Shoulder")
                .set("Left Stick Right", "Left Stick Right")
                .set("Left Stick Left", "Left Stick Left")
                .set("Left Stick Up", "Left Stick Up")
                .set("Left Stick Down", "Left Stick Down")
                .set("Right Stick Right", "Right Stick Right")
                .set("Right Stick Left", "Right Stick Left")
                .set("Right Stick Up", "Right Stick Up")
                .set("Right Stick Down", "Right Stick Down")
                .set("Left Trigger", "Left Trigger")
                .set("Right Trigger", "Right Trigger");
            conf.write_to_file(&config_path).unwrap();
            }


            let conf: Ini = Ini::load_from_file(&config_path).unwrap();

            let settings = conf.section(Some("Settings")).unwrap();

            let x_button = settings.get("X Button").unwrap().trim().to_string();
            let square_button = settings.get("Square Button").unwrap().to_string();
            let circle_button = settings.get("Circle Button").unwrap().to_string();
            let triangle_button = settings.get("Triangle Button").unwrap().to_string();
            let dpad_up = settings.get("DPad Up").unwrap().to_string();
            let dpad_down = settings.get("DPad Down").unwrap().to_string();
            let dpad_left = settings.get("DPad Left").unwrap().to_string();
            let dpad_right = settings.get("DPad Right").unwrap().to_string();
            let start_button = settings.get("Start").unwrap().to_string();
            let back_button = settings.get("Back").unwrap().to_string();
            let left_thumb = settings.get("Left Thumb").unwrap().to_string();
            let right_thumb = settings.get("Right Thumb").unwrap().to_string();
            let left_shoulder = settings.get("Left Shoulder").unwrap().to_string();
            let right_shoulder = settings.get("Right Shoulder").unwrap().to_string();
            let left_stick_right = settings.get("Left Stick Right").unwrap().to_string();
            let left_stick_left = settings.get("Left Stick Left").unwrap().to_string();
            let left_stick_up = settings.get("Left Stick Up").unwrap().to_string();
            let left_stick_down = settings.get("Left Stick Down").unwrap().to_string();
            let right_stick_right = settings.get("Right Stick Right").unwrap().to_string();
            let right_stick_left = settings.get("Right Stick Left").unwrap().to_string();
            let right_stick_up = settings.get("Right Stick Up").unwrap().to_string();
            let right_stick_down = settings.get("Right Stick Down").unwrap().to_string();
            let left_trigger = settings.get("Left Trigger").unwrap().to_string();
            let right_trigger = settings.get("Right Trigger").unwrap().to_string();


            let  xinputstate = &*pstate;
            let wbuttons = xinputstate.gamepad.wbuttons;

            let dpad_up_state = ((wbuttons >> 0) & 1) as f32;
            let dpad_down_state = ((wbuttons >> 1) & 1) as f32;
            let dpad_left_state = ((wbuttons >> 2) & 1) as f32;
            let dpad_right_state = ((wbuttons >> 3) & 1) as f32;
            let start_button_state = ((wbuttons >> 4) & 1) as f32;
            let back_button_state = ((wbuttons >> 5) & 1) as f32;
            let left_thumb_state = ((wbuttons >> 6) & 1)  as f32;
            let right_thumb_state = ((wbuttons >> 7) & 1) as f32;
            let left_shoulder_state = ((wbuttons >> 8) & 1) as f32;
            let right_shoulder_state = ((wbuttons >> 9) & 1) as f32;
            let x_button_state = ((wbuttons >> 12) & 1) as f32;
            let circle_button_state = ((wbuttons >> 13) & 1) as f32;
            let square_button_state = ((wbuttons >> 14) & 1) as f32;
            let triangle_button_state = ((wbuttons >> 15) & 1) as f32;

            let left_stick_x_state = xinputstate.gamepad.sthumblx as f32 / 32767.0f32;
            let left_stick_y_state = xinputstate.gamepad.sthumbly as f32 / 32767.0f32;
            let right_stick_x_state = xinputstate.gamepad.sthumbrx as f32 / 32767.0f32;
            let right_stick_y_state = xinputstate.gamepad.sthumbry as f32 / 32767.0f32;

            let mut left_stick_up_state = 0.0;
            let mut left_stick_down_state = 0.0;

            if left_stick_y_state > 0.0
            {
                left_stick_up_state = left_stick_y_state
            }
            else if left_stick_y_state < 0.0 {
                left_stick_down_state = -left_stick_y_state
            }

            let mut left_stick_right_state  = 0.0f32;
            let mut left_stick_left_state = 0.0f32;

            if left_stick_x_state > 0.0
            {
                left_stick_right_state = left_stick_x_state
            }
            else if left_stick_x_state < 0.0 {
                left_stick_left_state = -left_stick_x_state
            }

            let mut right_stick_up_state = 0.0;
            let mut right_stick_down_state = 0.0;

            if right_stick_y_state > 0.0
            {
                right_stick_up_state = right_stick_y_state
            }
            else if right_stick_y_state < 0.0 {
                right_stick_down_state = -right_stick_y_state
            }

            let mut right_stick_right_state  = 0.0f32;
            let mut right_stick_left_state = 0.0f32;

            if right_stick_x_state > 0.0
            {
                right_stick_right_state = right_stick_x_state
            }
            else if right_stick_x_state < 0.0 {
                right_stick_left_state = -right_stick_x_state
            }

            let right_trigger_state = xinputstate.gamepad.brighttrigger as f32 / 255.0;
            let left_trigger_state = xinputstate.gamepad.blefttrigger as f32 / 255.0;

            let button_string_hashmap = HashMap::from(
                [
                    ("X Button".to_string(), x_button_state),
                    ("Square Button".to_string(), square_button_state),
                    ("Circle Button".to_string(), circle_button_state),
                    ("Triangle Button".to_string(), triangle_button_state),
                    ("DPad Up".to_string(), dpad_up_state),
                    ("DPad Down".to_string(), dpad_down_state),
                    ("DPad Left".to_string(), dpad_left_state),
                    ("DPad Right".to_string(), dpad_right_state),
                    ("Start".to_string(), start_button_state),
                    ("Back".to_string(), back_button_state),
                    ("Left Thumb".to_string(), left_thumb_state),
                    ("Right Thumb".to_string(), right_thumb_state),
                    ("Left Shoulder".to_string(), left_shoulder_state),
                    ("Right Shoulder".to_string(), right_shoulder_state),
                    ("Left Stick Right".to_string(), left_stick_right_state),
                    ("Left Stick Left".to_string(), left_stick_left_state),
                    ("Left Stick Up".to_string(), left_stick_up_state),
                    ("Left Stick Down".to_string(), left_stick_down_state),
                    ("Right Stick Right".to_string(), right_stick_right_state),
                    ("Right Stick Left".to_string(), right_stick_left_state),
                    ("Right Stick Up".to_string(), right_stick_up_state),
                    ("Right Stick Down".to_string(), right_stick_down_state),
                    ("Left Trigger".to_string(), left_trigger_state),
                    ("Right Trigger".to_string(), right_trigger_state)
                ]
            ); 


            let new_x_button_state = button_string_hashmap[&x_button].round() as i32;
            let new_square_button_state = button_string_hashmap[&square_button].round() as i32;
            let new_circle_button_state = button_string_hashmap[&circle_button].round() as i32;
            let new_triangle_button_state = button_string_hashmap[&triangle_button].round() as i32;
            let new_dpad_up_state = button_string_hashmap[&dpad_up].round() as i32;
            let new_dpad_down_state = button_string_hashmap[&dpad_down].round() as i32;
            let new_dpad_left_state = button_string_hashmap[&dpad_left].round() as i32;
            let new_dpad_right_state = button_string_hashmap[&dpad_right].round() as i32;
            let new_start_button_state = button_string_hashmap[&start_button].round() as i32;
            let new_back_button_state = button_string_hashmap[&back_button].round() as i32;
            let new_left_shoulder_state = button_string_hashmap[&left_shoulder].round() as i32;
            let new_right_shoulder_state = button_string_hashmap[&right_shoulder].round() as i32;
            let new_left_thumb_state = button_string_hashmap[&left_thumb].round() as i32;
            let new_right_thumb_state = button_string_hashmap[&right_thumb].round() as i32;

            let new_wbuttons = {
                (new_dpad_up_state as u16) * 0x0001
                + (new_dpad_down_state as u16) * 0x0002
                + (new_dpad_left_state as u16) * 0x0004
                + (new_dpad_right_state as u16) * 0x0008
                + (new_start_button_state as u16) * 0x0010
                + (new_back_button_state as u16) * 0x0020
                + (new_left_thumb_state as u16) * 0x0040
                + (new_right_thumb_state as u16) * 0x0080
                + (new_left_shoulder_state as u16) * 0x0100
                + (new_right_shoulder_state as u16) * 0x0200
                + (new_x_button_state as u16) * 0x1000
                + (new_circle_button_state as u16) * 0x2000
                + (new_square_button_state as u16) * 0x4000
                + (new_triangle_button_state as u16) * 0x8000
            };

            
            let new_blefttrigger: u8 = (button_string_hashmap[&left_trigger]* 255.0) as u8;
            let new_brighttrigger: u8 = (button_string_hashmap[&right_trigger]* 255.0)as u8;

            let new_sthumbly = ((button_string_hashmap[&left_stick_up] - button_string_hashmap[&left_stick_down]) * 32767.0) as i16;
            let new_sthumblx = ((button_string_hashmap[&left_stick_right] - button_string_hashmap[&left_stick_left]) * 32767.0) as i16;

            let new_sthumbry = ((button_string_hashmap[&right_stick_up] - button_string_hashmap[&right_stick_down])* 32767.0) as i16;
            let new_sthumbrx = ((button_string_hashmap[&right_stick_right] - button_string_hashmap[&right_stick_left]) * 32767.0) as i16;            

            let new_xinput_gamepad = XinputGamepad{
                wbuttons: new_wbuttons,
                blefttrigger: new_blefttrigger,
                brighttrigger: new_brighttrigger,
                sthumbly: new_sthumbly,
                sthumblx: new_sthumblx,
                sthumbry: new_sthumbry,
                sthumbrx: new_sthumbrx
            };

            (*pstate).gamepad = new_xinput_gamepad;

            return_value
        }
        ).unwrap().enable().unwrap();
    }
    true
}