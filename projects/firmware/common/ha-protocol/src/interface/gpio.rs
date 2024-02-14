/// Interface type definitions and tools for GPIO interface implementation

/// Gpio direction
#[derive(Debug)]
pub enum GpioDir {
   /// Floating input
   FloatingInput,

   /// Pull down input
   PullDownInput,

   /// Pull up input
   PullUpInput,

   /// Push pull output
   PushPullOutput,

   /// Open drain output
   ODOutput,
}

/// GPIO request codes
enum Request {
   /// Set GPIO direction
   GpioDirSet,

   /// Get GPIO direction
   GpioDirGet,

   /// Write GPIO value
   GpioWrite,

   /// Read GPIO value
   GpioRead,
}

/// GPIO response codes
enum Response {
   /// GPIO value
   GpioValue,

   /// Gpio direction response
   GpioDir,
}