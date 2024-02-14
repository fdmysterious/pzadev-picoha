/// Interface type definitions for common aspect of all interfaces

/// Interface types
pub enum InterfaceType {
   /// GPIO interface
   Gpio,

   /// Watchdog measurement interface
   WdMeasure,
}

impl InterfaceType {
   pub fn as_u16(&self) -> u16 {
      match self {
         Gpio      => 0x0000u16,
         WdMeasure => 
      }
   }
}

/// Common request codes
enum Request{
   /// Basic ping
   Ping,

   /// Get interface type
   ItfType,

   /// Get interface version
   Version,

   /// Get interface ID
   IdGet,
}

/// Common response codes
enum Response {
   /// Good, request is OK
   Good,

   /// An error has occured (generic code), with optional text description
   ErrGeneric,

   /// CRC error
   ErrCRC,

   /// Code is unknown (not supported)
   ErrUnknownCode,

   /// Invalid arguments for request
   ErrInvalidArgs,

   /// Interface is busy
   ErrBusy,

   // ---------------------------------- //

   /// Version response
   VersionResp,

   /// Interface type response
   ItfTypeResp,

   /// ID response
   IdResp,
}