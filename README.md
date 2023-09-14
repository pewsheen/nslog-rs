# nslog-rs
Just a small lib for using `NSLog` in rust

## Example
This example will print `Hello World!` in terminal

```rs
use nslog_rs::NSLog;
use cocoa::base::nil;
use cocoa::foundation::NSString;

fn main() {
  unsafe {
      let fmt: id = NSString::alloc(nil).init_str("Hello %@!");
      let va_1: id = NSString::alloc(nil).init_str("World");
      NSLog(fmt, va_1);
  }
}
```
