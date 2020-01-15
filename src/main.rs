
use std::thread;

mod tm_tc_interface;
mod tc_constructor;
mod tm_deconstructor;

use tm_tc_interface::TmTcIf;
use tc_constructor::*;

fn main() {
    
    // Test TC pipeline

    // Create the interface
    let mut tm_tc_if = TmTcIf::start();

    // Create the constructor
    let mut tc_cstr = TcConstructor::new(&mut tm_tc_if);

    // Build a heartbeat command
    tc_cstr.build_and_send(TcHeartbeat::new()).unwrap();

    // Try and read some TM data (shouldn't be any)
    match tm_tc_if.get_pending_tm::<TcDisconnect>()  {
        Ok(Some(d)) => println!("Got some TM: {:?}", d),
        Ok(None) => println!("Couldn't get any TM data"),
        Err(e) => eprintln!("{}", e)
    }

    // Wait for a while to see if the heartbeat got sent
    thread::sleep_ms(500);

    // Stop the interface
    tm_tc_if.stop().unwrap();
}
