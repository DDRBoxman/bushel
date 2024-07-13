use objc2::rc::Retained;
use objc2::runtime::{AnyObject, NSObject};
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, mutability, ClassType};
use objc2_foundation::{ns_string, NSString};

fn main() {
    let application: Retained<MusicApplication> =
        SBApplication::new_with_bundle_identifier(ns_string!("com.apple.Music"));
    println!("{:?}", application);
    //application.activate();

    let application_class = application.class();

    println!("{:?}", application_class.name());

    application_class
        .instance_methods()
        .iter()
        .for_each(|method| {
            println!("{:?}", method.name());
        });

    application_class
        .instance_variables()
        .iter()
        .for_each(|ivar| {
            println!("{:?}", ivar.name());
        });

    // SBSApplication calls do work
    println!("{:?}", application.is_running());

    // Makes a raw call to playpause with the event codes
    // 0x686F6F6B, 0x506C5073, 0
    // works as expected
    application.sendEvent();

    // invalid message send to -[SBScriptableApplication playpause]: method not found
    application.playpause();
}

#[link(name = "ScriptingBridge", kind = "framework")]
extern "C" {}

extern_class!(
    /// An example description.
    #[derive(PartialEq, Eq, Hash, Debug)] // Uses the superclass' implementation
                                          // Specify the class and struct name to be used
    pub struct SBObject;

    // Specify the superclass, in this case `NSObject`
    unsafe impl ClassType for SBObject {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

impl SBObject {
    pub fn propertyWithCode(&self)-> Retained<SBObject>{
        unsafe { msg_send_id![self, propertyWithCode: 0x506C5073 as u32] }
    }

    pub fn sendEvent(&self) -> Option<Retained<AnyObject>> {
        unsafe { msg_send_id![self, sendEvent: 0x686F6F6B as u32 id: 0x506C5073 as u32 parameters: 0 as u32] }
    }

}

extern_class!(
    /// An example description.
    #[derive(PartialEq, Eq, Hash, Debug)] // Uses the superclass' implementation
                                          // Specify the class and struct name to be used
    pub struct SBApplication;

    // Specify the superclass, in this case `NSObject`
    unsafe impl ClassType for SBApplication {
        type Super = SBObject;
        type Mutability = mutability::InteriorMutable;
    }
);

impl SBApplication {
    pub(crate) fn new_with_bundle_identifier(s: &NSString) -> Retained<MusicApplication> {
        unsafe { msg_send_id![Self::class(), applicationWithBundleIdentifier: s] }
    }
}

extern_methods!(
    unsafe impl SBApplication {
        #[method(activate)]
        pub fn activate(&self);

        #[method(isRunning)]
        pub fn is_running(&self) -> bool;
    }
);


extern_class!(
    /// An example description.
    #[derive(PartialEq, Eq, Hash, Debug)] // Uses the superclass' implementation
                                          // Specify the class and struct name to be used
    pub struct MusicApplication;

    // Specify the superclass, in this case `NSObject`
    unsafe impl ClassType for MusicApplication {
        type Super = SBApplication;
        type Mutability = mutability::InteriorMutable;
    }
);

impl MusicApplication {
    pub fn playpause(&self) {
        unsafe { msg_send![self, playpause] }
    }
}