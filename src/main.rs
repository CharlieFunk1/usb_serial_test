use serialport::{available_ports, SerialPortType, SerialPort};
use std::io::{Write, IoSlice};

fn main() {
    let r:u8 = 255;
    let g:u8 = 0;
    let b:u8 = 0;
    let mut data = String::new();//: String = ' '.to_string();
    //println!("fresh");
    let mut port = serialport::new("/dev/ttyUSB0", 230_400)
	.timeout(std::time::Duration::from_millis(10))
	.open().expect("Failed to open port");
    loop {
	let mut i:u8 = 0;
	while i < 150 {
	    data.push_str(&r.to_string());
	    //println!("{:?}", data);
	    data.push_str(&':'.to_string());
	    //println!("{:?}", data);
	    data.push_str(&g.to_string());
	    //println!("{:?}", data);
	    data.push_str(&':'.to_string());
	    //println!("{:?}", data);
	    data.push_str(&(b + i).to_string());
	    //println!("{:?}", data);
	    data.push_str(&';'.to_string());
	    //println!("{:?}", data);
	    i += 1;
	    //println!("i = {:?}", i);
	}
	data.push_str(&'/'.to_string());
	//println!("done with strip");
	println!("{:?}", data);
	port.write(&data.as_bytes()).expect("Couldn't write to serial");
	port.flush().unwrap();
	//std::thread::sleep(std::time::Duration::from_millis(50));
	println!("Write complete");
	let data = String::new();
    }
}
