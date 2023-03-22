use fltk::{
    app,
    prelude::*,
    window::Window,
    button::*,
    input::Input,
    enums::{Font,Color,FrameType}, group,frame,
};
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::System::Memory::{PAGE_READWRITE},
    Win32::{System::Memory::*},
};


pub fn show_dialog()-> MainDlg{
    MainDlg::default()
}

pub struct MainDlg{
}

impl MainDlg{
    pub fn default()->Self{
        let mut win = Window::default().with_size(400,180)
        .with_label("Basic Sharing");
        let pack = group::Pack::new(0,0,400,80,None);
        frame::Frame::default().with_size(0,40).with_label("Enter content");
        let inp = Input::default().with_size(0,40);
        pack.end();

        frame::Frame::default();
        let mut write = Button::new(100,95,80,40,None).with_label("Write");
        write.set_color(Color::from_rgb(225,225,225));
        let mut read = Button::new(230,95,80,40,None).with_label("Read");
        read.set_color(Color::from_rgb(225,225,225));

        let mut frame = frame::Frame::new(0,145,400,30,None);
        frame.set_frame(FrameType::BorderBox);
        frame.set_color(Color::Green.inactive());

        win.set_color(Color::from_rgb(240,240,240));
        win.end();

        win.show();

        unsafe{
            write.set_callback(move|_|{
                let _result = CreateFileMappingW(INVALID_HANDLE_VALUE, 
                    None, 
                    PAGE_READWRITE, 
                    0,
                     1<<12, w!("MySharedMemory"));
                let mem = match _result{
                    Ok(_result)=> _result,
                    Err(_result)=> panic!("Failed to create file mapping {:?}",_result),
                };
                let buffer = MapViewOfFile(mem, FILE_MAP_WRITE, 0, 0, 0);
                if buffer.is_null(){
                    panic!("failed to map memroy");
                }else{
                    let value = inp.value();
                    println!("value: {:?} {}",buffer,value);
                    let mut index = 0;
                    for b in value.bytes() {
                        std::ptr::write(buffer.add(index) as *mut u8,b);
                        index = index+1;
                    }
                }
                UnmapViewOfFile(buffer);
                CloseHandle(mem);
            });

            read.set_callback( move|_|{
                let _result = CreateFileMappingW(INVALID_HANDLE_VALUE, 
                    None, 
                    PAGE_READWRITE, 
                    0,
                     1<<12, w!("MySharedMemory"));
                let mem = match _result{
                    Ok(_result)=> _result,
                    Err(_result)=> panic!("Failed to create file mapping {:?}",_result),
                };
                let buffer = MapViewOfFile(mem, FILE_MAP_READ, 0, 0, 0);
                if buffer.is_null(){
                    panic!("failed to map memroy");
                }else{
                    frame.set_label("test!");
                }
                UnmapViewOfFile(buffer);
                CloseHandle(mem);
            });

            Self{}
        }
    }
}

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    app::set_font(Font::Times);
    let dlg = show_dialog();
    app.run().unwrap();
}

