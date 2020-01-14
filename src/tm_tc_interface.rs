// ---------------------------------------------------------------------------
// TELEMETRY AND TELECOMMAND INTERFACE
// 
// Allows for a nice asynchronous interface between the TC constructor and
// TM deconstructor modules and the rover's AutonomyManager via a TCP/IP link.
//
// All TMs/TCs are encoded as JSON with the layout of the `AutoTmTc` struct.
//
// Each packet can contain arbitrary data stored as a `String` object.
// ---------------------------------------------------------------------------

use serde::{Serialize, Deserialize, de::DeserializeOwned};
use serde_json;
use chrono::{DateTime, Utc};
use std::thread;
use std::sync::{Arc, mpsc::{channel, Receiver, Sender, TryRecvError}};
use std::sync::atomic::{Ordering, AtomicBool};

// ---------------------------------------------------------------------------
// AUTOTMTC PACKET
// ---------------------------------------------------------------------------

// Trait for data that can be used in an AutoTmTc packet
pub trait TmTcData: Serialize + DeserializeOwned {}

// The structure of a TM or TC packet for sending to the AutonomyManager. 
// Should be sent using JSON.
#[derive(Serialize, Deserialize, Clone, Debug)]
struct AutoTmTc {
    // Frame count of this packet, starting from 0.
    frame_counter: Option<u32>,

    // The time at which this packet was sent, as a UTC DateTime object.
    send_time_utc: Option<DateTime<Utc>>,

    // The typename of the data stored in this packet
    data_type_name: String,

    // The data to be sent as a UTF-8 string
    data: String
}

impl AutoTmTc {

    // Construct the packet back from a JSON string, returns string on error.
    fn from_json(json_str: &str) -> Result<Self, String> {
        match serde_json::from_str(json_str) {
            Ok(a) => Ok(a),
            Err(e) => Err(format!("Cannot parse JSON string: {}", e))
        }
    }

    // Serialise the packet into a JSON string
    fn to_json(&self) -> Result<String, String> {
        match serde_json::to_string(&self) {
            Ok(s) => Ok(s),
            Err(e) => Err(format!("Cannot serialise the packet: {}", e))
        }
    }

}

// ---------------------------------------------------------------------------
// TMTCIF MODULE
// ---------------------------------------------------------------------------

struct TmTcIfBackend {

    // A vector of pending TCs to be sent in the next send cycle
    tc_queue: Vec<AutoTmTc>,

    // Receiver for the TC queue (i.e. how data gets into the interface from 
    // the GUI)
    tc_rx: Receiver<AutoTmTc>,

    // Vector of pending TMs
    tm_queue: Vec<AutoTmTc>,

    // Sender for the TM queue (i.e. how data gets from the interface to the 
    // GUI)
    tm_tx: Sender<AutoTmTc>,

    // Keep running bool
    run: Arc<AtomicBool>
}

impl TmTcIfBackend {

    fn start(
        chan_tc_rx: Receiver<AutoTmTc>, 
        chan_tm_tx: Sender<AutoTmTc>,
        backround_run: Arc<AtomicBool>) -> thread::JoinHandle<()> {

        let mut backend = TmTcIfBackend {
            tc_queue: vec![],
            tc_rx: chan_tc_rx,

            tm_queue: vec![],
            tm_tx: chan_tm_tx,

            run: backround_run
        };

        thread::spawn(move || {
            loop {
                // Run the cyclic activity
                match backend.cyclic_activity() {
                    None => (),
                    Some(e) => {
                        eprintln!("TmTcIfBackend exiting due to error: {}", e);
                        break
                    }
                }

                // If the backend has been requested to stop
                if !(backend.run.load(Ordering::SeqCst)) {
                    println!("TmTcIfBackend exiting on stop request");
                    break;
                }
            }

            // TODO: Cleanup
        })
    }

    // Check for new TCs coming from the GUI and new TMs coming from the Rover
    fn cyclic_activity(&mut self) -> Option<String> {
        
        // Check for new TCs by reading from the TC receiver
        for tc in self.tc_rx.try_iter() {
            // For now just print them out 
            // TODO: send to rover
            println!("TC: {:?}", tc);
        }

        None
    }

}

// The TMTC interface module state. Instatiate using the ::start function.
pub struct TmTcIf {

    // Sender for the TC queue (i.e. how data gets into the interface from the 
    // GUI)
    tc_tx: Sender<AutoTmTc>,

    // Receiver for the TM queue (i.e. how data gets from the interface to the 
    // GUI)
    tm_rx: Receiver<AutoTmTc>,

    // Handle to the backend thread
    backend_thread_handle: thread::JoinHandle<()>,

    // Stop atomic bool used to stop the backend when a stop() func is called
    backend_run: Arc<AtomicBool>

}

impl TmTcIf {

    // Start the interface processing, and return the interface structure that
    // you can call `add_pending_tc` and `get_pending_tm` on.
    pub fn start() -> Self {

        // Create channels
        let (chan_tc_tx, chan_tc_rx) = channel::<AutoTmTc>();
        let (chan_tm_tx, chan_tm_rx) = channel::<AutoTmTc>();

        // Create atomic run bool
        let backend_run_bool = Arc::new(AtomicBool::new(true));

        // Start the backend
        let backend_handle = TmTcIfBackend::start(
            chan_tc_rx, chan_tm_tx, backend_run_bool.clone());

        // Create the front end interface
        let tm_tc_if = TmTcIf {
            tc_tx: chan_tc_tx,
            tm_rx: chan_tm_rx,
            backend_thread_handle: backend_handle,
            backend_run: backend_run_bool
        };

        tm_tc_if
    }

    // Stop the execution of the interface, including disconnnecting from the 
    // rover and sending the disconnect TC
    pub fn stop(self) -> Result<(), String> {

        // TODO: Disconnect

        // Store a false in the backend run bool
        self.backend_run.store(false, Ordering::SeqCst);

        // Wait for the background thread to join
        match self.backend_thread_handle.join() {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to join backend thread: {:?}", e))
        }
    }

    // Add a new piece of data to the TC queue to be sent to the rover's 
    // AutonomyManager.
    pub fn add_pending_tc<T>(&mut self, data: T) -> Result<(), String> where 
        T: TmTcData {
        
        let json_str = match serde_json::to_string(&data) {
            Ok(s) => s,
            Err(e) => return Err(format!("Failed to serialise data: {}", e))
        };

        match self.tc_tx.send(AutoTmTc {
            frame_counter: None,
            send_time_utc: None,
            data_type_name: std::any::type_name::<T>().to_string(),
            data: json_str
        }) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to add packet to queue: {}", e))
        }
    }

    // Get a pending TM from the buffer, or return None if none available
    pub fn get_pending_tm<T>(&self) -> Result<Option<T>, String> where
        T: TmTcData {
        
        // Read from the TM reciever
        match self.tm_rx.try_recv() {
            Ok(tm) => match serde_json::from_str(&tm.data) {
                Ok(d) => Ok(Some(d)),
                Err(e) => Err(format!("Failed to parse incoming data: {}", e))
            },
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(format!("Internal channel disconnected"))
        }
    }
}