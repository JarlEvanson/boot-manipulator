//! Console manipulation.

use core::error::Error;

trait Console: Send {
    type ReadError: Error;
    type WriteError: Error;

    fn read(&mut self, data: &mut [u8]) -> Result<(), (Self::ReadError, usize)>;

    fn write(&mut self, data: &[u8]) -> Result<(), (Self::WriteError, usize)>;
}
