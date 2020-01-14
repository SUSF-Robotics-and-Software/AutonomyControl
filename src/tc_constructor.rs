// ---------------------------------------------------------------------------
// TELECOMMAND CONSTRUCTOR
//
// Provides a single interface to the GUI for building telecommands which will
// be sent to the Rover via the TmTcInterface module.
//
// Diferent types of telecommand are defined as structs here.
// ---------------------------------------------------------------------------

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::tm_tc_interface::{TmTcIf, TmTcData};

// ---------------------------------------------------------------------------
// TC CONSTRUCTOR
// ---------------------------------------------------------------------------

pub struct TcConstructor<'a> {

    tm_tc_if: &'a mut TmTcIf,

}

impl<'a> TcConstructor<'a> {

    pub fn new(tm_tc_if: &'a mut TmTcIf) -> Self {
        TcConstructor {
            tm_tc_if: tm_tc_if
        }
    }

    pub fn build_and_send<T>(&mut self, data: T) -> Result<(), String> where
        T: TmTcData {
        
        self.tm_tc_if.add_pending_tc(data)
    }
}

// ---------------------------------------------------------------------------
// TC TYPES
// ---------------------------------------------------------------------------
// Note all the `impl TmTcData for ...` statememts are at the bottom.

// HEARTBEAT
//
// Contains the current time to be sent to the Rover

#[derive(Serialize, Deserialize, Debug)]
pub struct TcHeartbeat {
    current_time_utc: DateTime<Utc>
}

impl TcHeartbeat {
    pub fn new() -> Self {
        TcHeartbeat {
            current_time_utc: chrono::Utc::now()
        }
    }
}

// DISCONNECT
//
// Instructs the rover to disconnect from the control GUI

#[derive(Serialize, Deserialize, Debug)]
pub struct TcDisconnect {}

impl TcDisconnect {
    pub fn new() -> Self {
        TcDisconnect {}
    }
}

// Impl statements

impl TmTcData for TcHeartbeat {}
impl TmTcData for TcDisconnect {}