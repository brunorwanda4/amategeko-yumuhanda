//
//  gpui_ios.h
//  GPUI iOS FFI Header
//
//  This header declares the C-compatible functions exported by the GPUI Rust library
//  for use in iOS Objective-C / Swift code.
//

#ifndef GPUI_IOS_H
#define GPUI_IOS_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/// Initialize the GPUI iOS application.
void* gpui_ios_initialize(void);

/// Called when the iOS app has finished launching.
void gpui_ios_did_finish_launching(void* app_ptr);

/// Called when the iOS app will enter the foreground.
void gpui_ios_will_enter_foreground(void* app_ptr);

/// Called when the iOS app did become active.
void gpui_ios_did_become_active(void* app_ptr);

/// Called when the iOS app will resign active.
void gpui_ios_will_resign_active(void* app_ptr);

/// Called when the iOS app did enter the background.
void gpui_ios_did_enter_background(void* app_ptr);

/// Called when the iOS app will terminate.
void gpui_ios_will_terminate(void* app_ptr);

/// Called when a touch event occurs.
void gpui_ios_handle_touch(void* window_ptr, void* touch_ptr, void* event_ptr);

/// Give GPUI a frame. Returns whether GPUI wants another one.
bool gpui_ios_request_frame(void* window_ptr);

/// Register the host callback that resumes frame delivery.
void gpui_ios_set_frame_waker(void* window_ptr, void (*waker)(void* context), void* context);

/// Get the most recently created GPUI window pointer.
void* gpui_ios_get_window(void);

/// Register the application root view with the GPUI platform.
void gpui_ios_register_app(void);

/// Run the GPUI iOS application.
void gpui_ios_run_demo(void);

/// Show the software keyboard.
void gpui_ios_show_keyboard(void* window_ptr);

/// Hide the software keyboard.
void gpui_ios_hide_keyboard(void* window_ptr);

/// Handle text input from the software keyboard.
void gpui_ios_handle_text_input(void* window_ptr, void* text_ptr);

/// Handle a key event from an external keyboard.
void gpui_ios_handle_key_event(void* window_ptr, uint32_t key_code, uint32_t modifiers, _Bool is_key_down);

/// Called when the app receives a URL to open (deep link).
void gpui_ios_handle_open_url(void* url_ptr);

#ifdef __cplusplus
}
#endif

#endif /* GPUI_IOS_H */
