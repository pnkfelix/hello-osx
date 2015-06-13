Hello OS X. :)

First we pull in the `cooca` crate.

```rust
extern crate cocoa;
```

Note: The `cocoa` crate is specified in our `Cargo.toml` file as an external dependency,
like so:
```toml
[dependencies.cocoa]
git = "https://github.com/servo/cocoa-rs"
```

Next, we need to import some names from the `cocoa` crate for use in our demo program.

These `base` imports are used frequently enough that we'll pull them
in rather than referencing them via `base::IDENT` all the time.

```rust
use cocoa::base::{self, selector, nil, YES, NO};

use cocoa::foundation::{self, NSString};
use cocoa::appkit;

// These are traits that must be pulled into scope in order for their
// methods to be visible.
use cocoa::foundation::NSAutoreleasePool as NSAutoreleasePoolTrait;
use cocoa::foundation::NSProcessInfo as NSProcessInfoTrait;
use cocoa::appkit::NSApplication as NSApplicationTrait;
use cocoa::appkit::NSMenu as NSMenuTrait;
use cocoa::appkit::NSMenuItem as NSMenuItemTrait;
use cocoa::appkit::NSWindow as NSWindowTrait;

fn main() {
    unsafe {
        let _pool = foundation::NSAutoreleasePool::new(nil);

        let app = appkit::NSApp();
        app.setActivationPolicy_(appkit::NSApplicationActivationPolicyRegular);

        let menubar = appkit::NSMenu::new(nil).autorelease();
        let app_menu_item = appkit::NSMenuItem::new(nil).autorelease();
        menubar.addItem_(app_menu_item);
        app.setMainMenu_(menubar);

        // create Application menu
        let app_menu = appkit::NSMenu::new(nil).autorelease();
        let quit_prefix = NSString::alloc(nil).init_str("Quit ");
        let quit_title = quit_prefix.stringByAppendingString_(
            foundation::NSProcessInfo::processInfo(nil).processName()
                );
        let quit_action = selector("terminate:");
        let quit_key = NSString::alloc(nil).init_str("q");
        let quit_item = appkit::NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
            quit_title,
            quit_action,
            quit_key
                ).autorelease();
        app_menu.addItem_(quit_item);

        app_menu_item.setSubmenu_(app_menu);

        // create Window
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

        app.activateIgnoringOtherApps_(YES);
        app.run();
    }
    println!("Hello World 2");
}
```
