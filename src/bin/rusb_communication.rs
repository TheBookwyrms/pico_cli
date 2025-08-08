use rusb;
use rusb::UsbContext;
use rusb::Error;
use rusb::Device;


const PICO_VID : u16 = 49374;
const PICO_PID : u16 = 51966;


fn main() {


    //    for device in rusb::devices().unwrap().iter() {
    //    let device_desc = device.device_descriptor().unwrap();
    //    let device_handle = device.open();
    //
    //    println!("name {:?} Bus {:03} Device {:03} ID {:}:{:} {:?}",
    //        device_desc.manufacturer_string_index(),
    //        device.bus_number(),
    //        device.address(),
    //        device_desc.vendor_id(),
    //        device_desc.product_id(),
    //        device_handle,);
    //}


    let context_result = rusb::Context::new();
    //match context_result {
    //    Ok(ref v) => println!("working with version: {v:?}"),
    //    Err(e) => println!("error parsing header: {e:?}")
    //}
    let context = context_result.unwrap();
    let pico_option = context.open_device_with_vid_pid(PICO_VID, PICO_PID);
    let pico_handle = pico_option.unwrap();
    let mut pico_device = pico_handle.device();

    
    let device_desc = pico_device.device_descriptor().unwrap();
    println!("{:?}", device_desc);

    println!("");

    for n in 0..device_desc.num_configurations() {
        let config_desc = pico_device.config_descriptor(n).unwrap();
        println!("{:?}", config_desc);

        println!("");

        for interface in config_desc.interfaces() {
            for interface_desc in interface.descriptors() {
                println!("{:?}", interface_desc);
                for endpoint_desc in interface_desc.endpoint_descriptors() {
                    println!("{:?}", endpoint_desc);
                    println!("{:?}", endpoint_desc.direction());
                }
                println!("");
            }
        }
    }

    let a = pico_handle.claim_interface(0);
    println!("{:?}", a);
    // interface 0
    // endpoints { 1 Out } { 129, In }

    
    let input_end = 1;
    let output_end = 129;
    let time = std::time::Duration::from_millis(1);
    let time_read = std::time::Duration::from_secs(3);
    //let buffer : &mut [u8] = &mut [];
    let buffer: [u8; 4096] = [0; 4096];

    
        //let a = pico_handle.read_bulk(end, buffer, time);
        let a = pico_handle.write_bulk(input_end, &buffer, time);
        match a {
            Ok(n) => println!("wrote {:?}", n),
            //Ok(n) => print!(""),
            Err(n) => print!("didn't write {:?}", n),
        }
        
        let mut buf: [u8; 4096] = [0; 4096];

        //loop {
        let b = pico_handle.read_bulk(output_end, &mut buf, time_read);
        match b {
            //Ok(n) => println!("received {:?}", str::from_utf8(&buf)),
            Ok(n) => {
                println!("received data");
                let formatted_received = str::from_utf8(&buf).unwrap();
                let a = formatted_received.replace("\0", "");
                println!("formatted to : {:?}", a);
            },
            //Ok(n) => print!(""),
            //Err(n) => print!("didn't receive {:?}", n),
            Err(n) => print!(""),
        }

        //}


    //println!("{:?}", pico_device.speed());



}