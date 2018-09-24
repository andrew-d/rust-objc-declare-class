#[macro_use]
extern crate objc;
extern crate objc_foundation;

use std::os::raw::c_void;

use objc::declare::{ClassDecl, MethodImplementation};
use objc::runtime::{Class, Object, Sel};
use objc_foundation::{NSString, NSDictionary, NSObject};


fn main() {
    let superclass = Class::get("NEPacketTunnelProvider").unwrap();
    let mut decl = ClassDecl::new("MyNumber", superclass).unwrap();

    unsafe {
        decl.add_method(
            sel!(startTunnelWithOptions:completionHandler:),
            startTunnelWithOptions as extern fn(&Object, Sel, *const c_void, *const c_void));
        decl.add_method(
            sel!(stopTunnelWithReason:completionHandler:),
            stopTunnelWithReason as extern fn(&Object, Sel, usize, *const c_void));
    }

    println!("registering...");
    decl.register();
    println!("registered");
}

// TODO: options is of type: NSDictionary<NSString, NSObject>

extern fn startTunnelWithOptions(this: &Object, _sel: Sel, options: *const c_void, _completion_handler: *const c_void) {
    println!("startTunnelWithOptions called");
}

extern fn stopTunnelWithReason(this: &Object, _sel: Sel, reason: usize, _completion_handler: *const c_void) {
    println!("stopTunnelWithReason called");
}
