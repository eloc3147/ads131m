//! Traits for supporting SPI interfaces

use embedded_hal::spi::FullDuplex;
use nb;

use crate::Error;

/// A type that can make SPI transfers
pub trait Transfer<W> {
    /// Perform a SPI transaction on the interface
    ///
    /// The transaction will be the length of the `send` or `receive` buffer, whichever is longer.
    /// If `send` is shorter than `receive`, the remaining bytes will be sent as zeros.
    /// If `receive` is shorter than `send`, the remaining bytes will be dropped.
    ///
    /// Both buffer lengths MUST be a multiple of two.
    ///
    /// # Errors
    /// Will return `Err` if the underlying SPI interface encounters an error.
    ///
    /// # Panics
    /// Will panic if either buffer length is not a multiple of two.
    fn transfer(&mut self, send: &[u8], receive: &mut [u8]) -> Result<(), Error>;
}

impl<T, W> Transfer<W> for T
where
    T: WordTransfer<W>,
    W: Copy,
{
    fn transfer(&mut self, send: &[u8], receive: &mut [u8]) -> Result<(), Error> {
        debug_assert!(send.len() % 2 == 0);
        debug_assert!(receive.len() % 2 == 0);

        let transfer_len = core::cmp::max(send.len(), receive.len());
        let mut bytes_read = 0;
        let mut bytes_written = 0;
        let mut word_cache: Option<W> = None;

        // Try to keep write buffer full
        while bytes_read < transfer_len || bytes_written < transfer_len {
            // Write first
            if bytes_written < transfer_len {
                // Cache the word that bytes_written currently points to
                let word = word_cache.take().unwrap_or_else(|| {
                    if bytes_written < send.len() {
                        Self::encode_word(&send[bytes_written..(bytes_written + Self::WORD_LEN)])
                    } else {
                        Self::encode_word(&[0, 0][..Self::WORD_LEN])
                    }
                });

                match self.write_word(word) {
                    Ok(()) => bytes_written += Self::WORD_LEN,
                    Err(nb::Error::WouldBlock) => word_cache = Some(word),
                    Err(nb::Error::Other(e)) => return Err(e),
                }
            }

            // Read only if more has been written
            if bytes_written > bytes_read && bytes_read < transfer_len {
                match self.read_word(&mut receive[bytes_read..(bytes_read + Self::WORD_LEN)]) {
                    Ok(()) => bytes_read += Self::WORD_LEN,
                    Err(nb::Error::WouldBlock) => {}
                    Err(nb::Error::Other(e)) => return Err(e),
                }
            }
        }

        Ok(())
    }
}

/// Trait for interacting with an [`FullDuplex`] interface word by word
pub trait WordTransfer<W> {
    /// Length of a word in bytes
    const WORD_LEN: usize;

    /// Attempt to read and decode a word to a buffer
    ///
    /// # Note
    /// A word must be written before calling this
    ///
    /// # Errors
    /// Will return [`Err(WouldBlock)`](nb::Error::WouldBlock) if the read would require blocking.
    ///
    /// Will return [`Err(Other)`](nb::Error::Other) if the underlying SPI interface encounters an error.
    ///
    /// # Panics
    /// Will panic if buf length is not [`WORD_LEN`](Self::WORD_LEN)
    fn read_word(&mut self, buf: &mut [u8]) -> nb::Result<(), Error>;

    /// Encode a word from a buffer
    ///
    /// # Panics
    /// Will panic if buf length is not [`WORD_LEN`](Self::WORD_LEN)
    fn encode_word(buf: &[u8]) -> W;

    /// Attempt to write a word
    ///
    /// # Errors
    /// Will return [`Err(WouldBlock)`](nb::Error::WouldBlock) if the write would require blocking.
    ///
    /// Will return [`Err(Other)`](nb::Error::Other) if the underlying SPI interface encounters an error.
    fn write_word(&mut self, word: W) -> nb::Result<(), Error>;
}

impl<I: FullDuplex<u16>> WordTransfer<u16> for I {
    const WORD_LEN: usize = 2;

    fn read_word(&mut self, buf: &mut [u8]) -> nb::Result<(), Error> {
        debug_assert!(buf.len() == Self::WORD_LEN);
        let word = self.read().map_err(|e| match e {
            nb::Error::WouldBlock => nb::Error::WouldBlock,
            nb::Error::Other(_) => nb::Error::Other(Error::SpiIOError),
        })?;
        buf.copy_from_slice(&word.to_be_bytes());
        Ok(())
    }

    fn encode_word(buf: &[u8]) -> u16 {
        debug_assert!(buf.len() == Self::WORD_LEN);
        u16::from_be_bytes(buf.try_into().expect("buf should be WORD_LEN"))
    }

    fn write_word(&mut self, word: u16) -> nb::Result<(), Error> {
        self.send(word).map_err(|e| match e {
            nb::Error::WouldBlock => nb::Error::WouldBlock,
            nb::Error::Other(_) => nb::Error::Other(Error::SpiIOError),
        })
    }
}

impl<I: FullDuplex<u8>> WordTransfer<u8> for I {
    const WORD_LEN: usize = 1;

    fn read_word(&mut self, buf: &mut [u8]) -> nb::Result<(), Error> {
        debug_assert!(buf.len() == Self::WORD_LEN);
        let word = self.read().map_err(|e| match e {
            nb::Error::WouldBlock => nb::Error::WouldBlock,
            nb::Error::Other(_) => nb::Error::Other(Error::SpiIOError),
        })?;
        buf[0] = word;
        Ok(())
    }

    fn encode_word(buf: &[u8]) -> u8 {
        debug_assert!(buf.len() == Self::WORD_LEN);
        buf[0]
    }

    fn write_word(&mut self, word: u8) -> nb::Result<(), Error> {
        self.send(word).map_err(|e| match e {
            nb::Error::WouldBlock => nb::Error::WouldBlock,
            nb::Error::Other(_) => nb::Error::Other(Error::SpiIOError),
        })
    }
}

// TODO: DMA Interfaces
