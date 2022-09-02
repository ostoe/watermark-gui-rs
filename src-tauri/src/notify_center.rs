use crossbeam_channel::Receiver;
use tauri::Window;

use crate::banner_unit::Notification;


// the payload type must implement `Se&rialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
    state_code: u32,
}


pub fn notification_thread(main_window: Window, notify_front_rt: Receiver<Notification> ) {


// let id = main_window.listen("front-backend", |event| {
//     println!("got window event-name with payload {:?}", event.payload());
// });
// std::thread::sleep(std::time::Duration::from_secs(2));
// splashscreen_window.close().unwrap();
// main_window.show().unwrap();
loop {
    let opt_result = notify_front_rt.recv().unwrap();
    match opt_result {
        Notification::Single(fname) => {
            println!("----{:?}", fname);
            windows_send_msg(&main_window, "front-backend", &fname, 200);
        }
        Notification::Complated => {
            windows_send_msg(&main_window, "front-backend", "", 200);
        }
        Notification::Error(e) => {
            windows_send_msg(&main_window, "front-backend", &e, 500);
        }
        Notification::SkipFile(fname) => {
            windows_send_msg(&main_window, "front-backend", &fname, 300);
        }
    }
}
}



pub fn windows_send_msg(window: &Window, event: &str, msg: &str, code: u32) {
    window
        .emit(
            event,
            Payload {
                message: String::from(msg),
                state_code: code,
            },
        )
        .unwrap();
}