# nslog-rs

Just a small lib for using `NSLog` in rust

## Example

This example will print `Hello World!` in terminal

```rs
use nslog_rs::NSLog;
use cocoa_foundation::base::nil;
use cocoa_foundation::foundation::{NSAutoreleasePool, NSString};

fn main() {
  unsafe {
      let fmt: id = NSString::alloc(nil).init_str("Hello %@!").autorelease();
      let va_1: id = NSString::alloc(nil).init_str("World").autorelease();
      NSLog(fmt, va_1);
  }
}
```
