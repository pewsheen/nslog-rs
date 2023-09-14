use cocoa::base::id;

#[cfg(target_os = "macos")]
#[link(name = "Foundation", kind = "framework")]
extern "C" {
    pub fn NSLog(format: id, ...);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "macos")]
    fn it_works() {
        use cocoa::base::nil;
        use cocoa::foundation::NSString;
        unsafe {
            let fmt: id = NSString::alloc(nil).init_str("Hello %@!");
            let va_1: id = NSString::alloc(nil).init_str("World");
            NSLog(fmt, va_1);
        }
    }
}
