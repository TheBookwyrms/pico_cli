use rusb;
use rusb::devices;
use rusb::DeviceHandle;
use rusb::UsbContext;
use rusb::Error;
use rusb::Device;
use rusb::Context;


const PICO_VID : u16 = 49374;
const PICO_PID : u16 = 51966;
const IFACE_0_END_OUT : u8 = 129;
const IFACE_0_END_IN : u8 = 1;


fn display_device_info(device : &Device<Context>) {
    let device_desc = device.device_descriptor().unwrap();
    println!("{:?}", device_desc);

    println!("");

    for n in 0..device_desc.num_configurations() {
        let config_desc = device.config_descriptor(n).unwrap();
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

}


fn get_pico_prepared() -> DeviceHandle<Context>{
    let context = rusb::Context::new().expect("failed to get rusb context");
    let pico_handle = context
            .open_device_with_vid_pid(PICO_VID, PICO_PID)
            .expect("failed to open pico with VID and PID");

    let interface_claim = pico_handle.claim_interface(0)
                                         .expect("failed to claim communication interface");

    pico_handle
}


fn get_pico_device(pico_handle : &DeviceHandle<Context>) -> Device<Context> {
    pico_handle.device()
}

fn list_devices() {
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        let device_handle = device.open();
    
        println!("name {:?} Bus {:03} Device {:03} ID {:}:{:} {:?}",
            device_desc.manufacturer_string_index(),
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id(),
            device_handle,);
    }
}


pub fn write_bulk(data:&[u8]) {

    let pico_handle = get_pico_prepared();

    let time_write = std::time::Duration::from_millis(1);

    //let a = pico_handle.read_bulk(end, buffer, time);
    let write_result = pico_handle.write_bulk(IFACE_0_END_IN, data, time_write);
    match write_result {
        Ok(n) => println!("sent data {:?}", str::from_utf8(data).unwrap()),
        Err(n) => print!("didn't write {:?}", n),
    }
}


pub fn read_bulk() {

    let pico_handle = get_pico_prepared();

    let time_read = std::time::Duration::from_secs(3);
        
    let mut read_buf: [u8; 4096] = [0; 4096];
    

    let read_result = pico_handle.read_bulk(IFACE_0_END_OUT, &mut read_buf, time_read);
    

    //let a = &read_buf[0..4];
    //println!("test {:?}", a);

    
    //let a = [112, 105, 99, 111, 32, 115, 101, 110, 116, 32, 97, 110, 100, 32, 114, 101, 99, 101, 105, 118, 101, 100, 33, 33, 33];
    //println!("a {:?}", str::from_utf8(&a));

    match read_result {
        Ok(n) => {
            //println!("received data");
            let formatted_received = str::from_utf8(&read_buf).unwrap();
            let cleaned = formatted_received.replace("\0", "");
            println!("received data, formatted to : {:?}", cleaned);
        },
        Err(n) => print!("didn't read {:?}", n),
    }
}



fn rusb_demo() {

    
    // list_devices();

    let pico_handle = get_pico_prepared();
    
    //display_device_info(&get_pico_device(&pico_handle));


    let data = "pico sent and received!!!".as_bytes();
    let data = "pico!!!".as_bytes();
    
    
    //
    //
    //
    // REMINDER
    // ADD START/END CHARACTER U8
    // THIS WAY FIXES FILTERING
    // REMINDER
    //
    //
    
    
    
    
    write_bulk(data);//, &pico_handle);
    read_bulk();//&pico_handle);
}


fn main() {
    //rusb_demo();
}