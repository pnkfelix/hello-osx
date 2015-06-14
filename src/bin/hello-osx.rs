//@ Hello OS X. :)
//@
//@ This code is adapted from the `cocoa-rs` [hello_world.rs example]
//@
//@ [hello_world.rs example]: https://github.com/servo/cocoa-rs/blob/master/examples/hello_world.rs
//@
//@ I have added notes along the way as I dissected what the various pieces. Some of the
//@ annotations were based on my reading of [Minimalist Cocoa programming], which from what
//@ I can see could well be the basis from which `hello_world.rs` was written, it seems.
//@
//@ [Minimalist Cocoa programming]: http://www.cocoawithlove.com/2010/09/minimalist-cocoa-programming.html
//@
//@ First we pull in the `cooca` crate.

extern crate cocoa;

//@ Note: The `cocoa` crate is specified in our `Cargo.toml` file as an external dependency,
//@ like so:
//@ ```toml
//@ [dependencies.cocoa]
//@ git = "https://github.com/servo/cocoa-rs"
//@ ```
//@
//@ Next, we need to import some names from the `cocoa` crate for use in our demo program.
//@
//@ These `base` imports are used frequently enough that we'll pull them
//@ in rather than referencing them via `base::IDENT` all the time.

use cocoa::base::{selector, nil, YES, NO};

use cocoa::foundation::{self, NSString};
use cocoa::appkit;

//@ There are also traits that must be pulled into scope in order for
//@ their methods to be visible. Usually one folds these imports into
//@ the name imports above, but I want to continue using prefixes
//@ when I reference names explicitly below, so I instead rename each
//@ trait that I import.
//@
//@ (Idea: `use path as _;` for importing a traits methods without adding
//@ it to the namespace.)

use cocoa::foundation::NSAutoreleasePool as NSAutoreleasePoolTrait;
use cocoa::foundation::NSProcessInfo as NSProcessInfoTrait;
use cocoa::appkit::NSApplication as NSApplicationTrait;
use cocoa::appkit::NSMenu as NSMenuTrait;
use cocoa::appkit::NSMenuItem as NSMenuItemTrait;
use cocoa::appkit::NSWindow as NSWindowTrait;

//@ The original hello world demo managed to avoid doing any objective-c
//@ style message sends itself, but I am not so sure I will be so lucky.
//@ Let us import the macro to make that easy.

#[macro_use]
extern crate objc;

//@ Okay, now we can jump into the code!
//@
//@ First, I have put in a tiny class for instumenting the control-flow:
//@ `let _s = DropLoud("hi")` prints `make DropLoud("hi")` when it is
//@ first evaluated, and then print `drop DropLoud("hi")` when we hit the
//@ end of the scope for the binding `_s`.

use std::convert::{Into};
use std::borrow::{Cow};

#[derive(Debug)]
struct DropLoud { s: Cow<'static, str> }
#[allow(non_snake_case)]
fn DropLoud<S: Into<Cow<'static, str>>>(s: S) -> DropLoud {
    let s = s.into();
    println!("make DropLoud({})", s);
    DropLoud { s: s.into() }
}  
impl Drop for DropLoud {
    fn drop(&mut self) {
        println!("drop DropLoud({})", self.s);
    }
}

//@ Now, we are going to have all of our code inside of one giant `unsafe` block
//@ within `fn main`.  (Yes, I know its evil of me to write a code
//@ fragment where the parentheses don't match up.  Maybe I'll fix that
//@ later in some way, e.g. by factoring the code into individual
//@ functions or macros. I really do not want to replicate WEB's named
//@ extensible code-blocks in Tango.)

fn main() {
    let _start_main = DropLoud("start_main");
    unsafe {

//@ First we create an auto-release pool. Such a pool acts like a
//@ Tofte-style stack-based "region" for `NSObjects` -- you create the
//@ pool, and then you invoke `autorelease` on the object to register it
//@ to be released when the pool itself is drained. (You could also think
//@ of the auto-release pool as an "arena", though often arenas are
//@ implemented by allocating their objects from a contiguous backing
//@ store; that certainly is not what is going on here.)
//@
//@ According to my reading of [Using Autorelease Pool Blocks], we do not
//@ *have* to call the `drain` method ourselves on the pool at the end of
//@ its scope; it will automatically get called when the thread exits. But
//@ then again, it seems like better programming practice to do so (or
//@ rather, to use a `Drop` implementation to do it. I will explore that
//@ later).
//@
//@ [Using Autorelease Pool Blocks]: https://developer.apple.com/library/ios/documentation/Cocoa/Conceptual/MemoryMgmt/Articles/mmAutoreleasePools.html

        let _pool = foundation::NSAutoreleasePool::new(nil);

//@ Next we need to extract the shared app instance. (I misunderstood this
//@ when I first saw it; I thought it was *creating* an instance of some
//@ class named `NSApp`, but it is in fact accessing a global constant.
//@ According to the [`NSApp` doc], it is the same as sending the
//@ `NSApplication` class the method `sharedApplication` message.
//@
//@ [`NSApp` doc]: https://developer.apple.com/library/mac//documentation/Cocoa/Reference/ApplicationKit/Classes/NSApplication_Class/index.html#//apple_ref/doc/constant_group/NSApp

        let app = appkit::NSApp();

//@ Now we get into some nitty-gritty that comes up when you develop
//@ outside of XCode and the Interface Builder.
//@
//@ "In Snow Leopard" (alone?) "Programs without application bundles and Info.plist files don't get a menubar
//@ and can't be brought to the front unless the presentation option is changed:"

        app.setActivationPolicy_(appkit::NSApplicationActivationPolicyRegular);

//@ Here we create the menubar itself, which will be rendered along the
//@ top of the current desktop when this application is in the foreground.
//@
//@ This is passing `nil` as the argument to menu-item; I think that is to
//@ illustrate that the first menu-item automatically gets its name from
//@ the application's own name.

        let menubar = appkit::NSMenu::new(nil).autorelease();
        let app_menu_item = appkit::NSMenuItem::new(nil).autorelease();
        menubar.addItem_(app_menu_item);
        app.setMainMenu_(menubar);

//@ Our menu will have just the "Quit" command. Many demos and discussions
//@ implement "Quit" by calling the `terminate:` method, but from what I
//@ can tell, the `quit:` method is a better match for what we are likely
//@ to want. (At least, depending on whether we can shut ourselves down
//@ efficiently.)
//@
//@ TODO: What is the distinction between the `app_menu_item` and the `app_menu`?

        // create Application menu
        let app_menu = appkit::NSMenu::new(nil).autorelease();
        let quit_prefix = NSString::alloc(nil).init_str("Quit ");
        let quit_title = quit_prefix.stringByAppendingString_(
            foundation::NSProcessInfo::processInfo(nil).processName()
                );

        // let quit_action = selector("terminate:");
        let quit_action = selector("stop:");

        let quit_key = NSString::alloc(nil).init_str("q");
        let quit_item = appkit::NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            quit_title,
            quit_action,
            quit_key
                ).autorelease();
        app_menu.addItem_(quit_item);

        app_menu_item.setSubmenu_(app_menu);

//@ Now we create the window and activate the application. The first five
//@ lines here seem unfortunate; the corresponding Objective-C code is
//@ only three lines. This is partly a consequence of my own choice to
//@ keep the prefixes like `appkit::` and `foundation::` in the paths, but
//@ still, a nice wrapper library that tries to use types/traits rather
//@ than passing `id` everywhere and shortens up some of these code
//@ sequences seems like it could be a good thing.

        let window = appkit::NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
            foundation::NSRect::new(foundation::NSPoint::new(0., 0.), foundation::NSSize::new(200., 200.)),
            appkit::NSTitledWindowMask as foundation::NSUInteger,
            appkit::NSBackingStoreBuffered,
            NO
                ).autorelease();
        window.cascadeTopLeftFromPoint_(foundation::NSPoint::new(20., 20.));
        window.center();
        let title = NSString::alloc(nil).init_str("Hello World");
        window.setTitle_(title);
        window.makeKeyAndOrderFront_(nil);

        println!("about to activate");
        app.activateIgnoringOtherApps_(YES);

//@ Okay, we have made our window and activated it. It is even
//@ showing at this point (you can demonstrate this by adding
//@ some more instrumentation and well-placed pauses).
//@
//@ But we are not quite done yet: Our window, while showing, is not going
//@ to respond to any user-actions (e.g. Cmd-Q) until we fire up the
//@ event-loop. We can do that by calling the equivalent of `[NSApp
//@ run]`. (I have left in some instrumentation that illustrates when we
//@ reach this point in control-flow, and when we hit the end of the
//@ scope.)

        let _a = DropLoud(Cow::Borrowed("app_run"));
        app.run();
    }
    println!("Hello World 2");
}
