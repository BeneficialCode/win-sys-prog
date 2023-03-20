use std::{env};
use windows::{imp::{heap_free, GetLastError}, Win32::System::Diagnostics::Debug::*};
use windows::core::PWSTR;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ShowError <number>");
        return;
    }

    let message:u32 = args[1].parse().unwrap();

    let mut text = HeapString(std::ptr::null_mut());
    unsafe{
        let chars = FormatMessageW(
            FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            None, message, 
            0,
            PWSTR(&mut text.0 as *mut _ as *mut _),
            0,None);
        if chars > 0 {
            let parts = std::slice::from_raw_parts(text.0, chars as _);
            println!("Message {}: {:?}",message,String::from_utf16(parts).unwrap());
        }
        else{
            let errcode = GetLastError();
            println!("No such error exists error: {}",errcode);
        }
    }

}

#[derive(Debug)]
struct HeapString(*mut u16);

impl Drop for HeapString{
    fn drop(&mut self) {
        if !self.0.is_null(){
            unsafe{
                heap_free(self.0 as _);
            }
        }
    }
}