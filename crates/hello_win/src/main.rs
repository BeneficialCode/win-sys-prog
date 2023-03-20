use windows::{Win32::System::SystemInformation::*};

fn main() {
    let mut si: SYSTEM_INFO = Default::default();
    unsafe{
        GetNativeSystemInfo(&mut si);
    }
    println!("Number of Logical Processors: {}",si.dwNumberOfProcessors);
    println!("Page size: {} Bytes", si.dwPageSize);
	println!("Processor Mask: {:x}", si.dwActiveProcessorMask);
	println!("Minimum process address: {:?}", si.lpMinimumApplicationAddress);
	println!("Maximum process address: {:?}", si.lpMaximumApplicationAddress);
}
