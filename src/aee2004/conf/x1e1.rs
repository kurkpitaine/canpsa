use core::fmt;

use crate::{
    vehicle::{PAXWheelState, UnderInflationSystemState, WheelState},
    Error, Result,
};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    use crate::field::Field;
    /// 1-bit empty,
    /// 2-bit front left wheel PAX state field,
    /// 5-bit front left wheel state field.
    pub const FRONT_LEFT: usize = 0;
    /// 1-bit empty,
    /// 2-bit front right wheel PAX state field,
    /// 5-bit front right wheel state field.
    pub const FRONT_RIGHT: usize = 1;
    /// 1-bit empty,
    /// 2-bit rear left wheel PAX state field,
    /// 5-bit rear left wheel state field.
    pub const REAR_LEFT: usize = 2;
    /// 1-bit empty,
    /// 2-bit rear right wheel PAX state field,
    /// 5-bit rear right wheel state field.
    pub const REAR_RIGHT: usize = 3;
    /// 3-bit empty,
    /// 5-bit spare wheel state field.
    pub const SPARE_WHEEL: usize = 4;
    /// 5-bit empty,
    /// 3-bit under-inflation system state field.
    pub const SYSTEM_STATE: usize = 5;
    /// 16-bit empty field.
    pub const EMPTY: Field = 6..8;
}

/// Length of a x1e1 CAN frame.
pub const FRAME_LEN: usize = field::EMPTY.end;

impl<T: AsRef<[u8]>> Frame<T> {
    /// Create a raw octet buffer with a CAN frame structure.
    #[inline]
    pub fn new_unchecked(buffer: T) -> Frame<T> {
        Frame { buffer }
    }

    /// Shorthand for a combination of [new_unchecked] and [check_len].
    ///
    /// [new_unchecked]: #method.new_unchecked
    /// [check_len]: #method.check_len
    #[inline]
    pub fn new_checked(buffer: T) -> Result<Frame<T>> {
        let packet = Self::new_unchecked(buffer);
        packet.check_len()?;
        Ok(packet)
    }

    /// Ensure that no accessor method will panic if called.
    /// Returns `Err(Error::Truncated)` if the buffer is too short.
    ///
    /// The result of this check is invalidated by calling [set_payload_len].
    ///
    /// [set_payload_len]: #method.set_payload_len
    #[inline]
    pub fn check_len(&self) -> Result<()> {
        let len = self.buffer.as_ref().len();
        if len < (FRAME_LEN) {
            Err(Error::Truncated)
        } else if len > (FRAME_LEN) {
            Err(Error::Overlong)
        } else {
            Ok(())
        }
    }

    /// Consume the frame, returning the underlying buffer.
    #[inline]
    pub fn into_inner(self) -> T {
        self.buffer
    }

    /// Return the frame length.
    #[inline]
    pub fn frame_len(&self) -> usize {
        FRAME_LEN
    }

    /// Return the front left wheel PAX state field.
    #[inline]
    pub fn front_left_wheel_pax_state(&self) -> PAXWheelState {
        let data = self.buffer.as_ref();
        let raw = (data[field::FRONT_LEFT] & 0x06) >> 1;
        PAXWheelState::from(raw)
    }

    /// Return the front left wheel state field.
    #[inline]
    pub fn front_left_wheel_state(&self) -> WheelState {
        let data = self.buffer.as_ref();
        WheelState::from(data[field::FRONT_LEFT] >> 3)
    }

    /// Return the front right wheel PAX state field.
    #[inline]
    pub fn front_right_wheel_pax_state(&self) -> PAXWheelState {
        let data = self.buffer.as_ref();
        let raw = (data[field::FRONT_RIGHT] & 0x06) >> 1;
        PAXWheelState::from(raw)
    }

    /// Return the front right wheel state field.
    #[inline]
    pub fn front_right_wheel_state(&self) -> WheelState {
        let data = self.buffer.as_ref();
        WheelState::from(data[field::FRONT_RIGHT] >> 3)
    }

    /// Return the rear left wheel PAX state field.
    #[inline]
    pub fn rear_left_wheel_pax_state(&self) -> PAXWheelState {
        let data = self.buffer.as_ref();
        let raw = (data[field::REAR_LEFT] & 0x06) >> 1;
        PAXWheelState::from(raw)
    }

    /// Return the rear left wheel state field.
    #[inline]
    pub fn rear_left_wheel_state(&self) -> WheelState {
        let data = self.buffer.as_ref();
        WheelState::from(data[field::REAR_LEFT] >> 3)
    }

    /// Return the rear right wheel PAX state field.
    #[inline]
    pub fn rear_right_wheel_pax_state(&self) -> PAXWheelState {
        let data = self.buffer.as_ref();
        let raw = (data[field::REAR_RIGHT] & 0x06) >> 1;
        PAXWheelState::from(raw)
    }

    /// Return the rear right wheel state field.
    #[inline]
    pub fn rear_right_wheel_state(&self) -> WheelState {
        let data = self.buffer.as_ref();
        WheelState::from(data[field::REAR_RIGHT] >> 3)
    }

    /// Return the spare wheel state field.
    #[inline]
    pub fn spare_wheel_state(&self) -> WheelState {
        let data = self.buffer.as_ref();
        WheelState::from(data[field::SPARE_WHEEL] >> 3)
    }

    /// Return the under-inflation system state field.
    #[inline]
    pub fn system_state(&self) -> UnderInflationSystemState {
        let data = self.buffer.as_ref();
        UnderInflationSystemState::from(data[field::SYSTEM_STATE] >> 5)
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the front left wheel PAX state field.
    #[inline]
    pub fn set_front_left_wheel_pax_state(&mut self, value: PAXWheelState) {
        let data = self.buffer.as_mut();
        let raw = data[field::FRONT_LEFT] & !0x06;
        let raw = raw | (u8::from(value) << 1);
        data[field::FRONT_LEFT] = raw;
    }

    /// Set the front left wheel state field.
    #[inline]
    pub fn set_front_left_wheel_state(&mut self, value: WheelState) {
        let data = self.buffer.as_mut();
        let raw = data[field::FRONT_LEFT] & !0xf8;
        let raw = raw | (u8::from(value) << 3);
        data[field::FRONT_LEFT] = raw;
    }

    /// Set the front right wheel state field.
    #[inline]
    pub fn set_front_right_wheel_pax_state(&mut self, value: PAXWheelState) {
        let data = self.buffer.as_mut();
        let raw = data[field::FRONT_RIGHT] & !0x06;
        let raw = raw | (u8::from(value) << 1);
        data[field::FRONT_RIGHT] = raw;
    }

    /// Set the front right wheel state field.
    #[inline]
    pub fn set_front_right_wheel_state(&mut self, value: WheelState) {
        let data = self.buffer.as_mut();
        let raw = data[field::FRONT_RIGHT] & !0xf8;
        let raw = raw | (u8::from(value) << 3);
        data[field::FRONT_RIGHT] = raw;
    }

    /// Set the rear left wheel PAX state field.
    #[inline]
    pub fn set_rear_left_wheel_pax_state(&mut self, value: PAXWheelState) {
        let data = self.buffer.as_mut();
        let raw = data[field::REAR_LEFT] & !0x06;
        let raw = raw | (u8::from(value) << 1);
        data[field::REAR_LEFT] = raw;
    }

    /// Set the rear left wheel state field.
    #[inline]
    pub fn set_rear_left_wheel_state(&mut self, value: WheelState) {
        let data = self.buffer.as_mut();
        let raw = data[field::REAR_LEFT] & !0xf8;
        let raw = raw | (u8::from(value) << 3);
        data[field::REAR_LEFT] = raw;
    }

    /// Set the rear right wheel PAX state field.
    #[inline]
    pub fn set_rear_right_wheel_pax_state(&mut self, value: PAXWheelState) {
        let data = self.buffer.as_mut();
        let raw = data[field::REAR_RIGHT] & !0x06;
        let raw = raw | (u8::from(value) << 1);
        data[field::REAR_RIGHT] = raw;
    }

    /// Set the rear right wheel state field.
    #[inline]
    pub fn set_rear_right_wheel_state(&mut self, value: WheelState) {
        let data = self.buffer.as_mut();
        let raw = data[field::REAR_RIGHT] & !0xf8;
        let raw = raw | (u8::from(value) << 3);
        data[field::REAR_RIGHT] = raw;
    }

    /// Set the spare wheel state field.
    #[inline]
    pub fn set_spare_wheel_state(&mut self, value: WheelState) {
        let data = self.buffer.as_mut();
        let raw = data[field::SPARE_WHEEL] & !0xf8;
        let raw = raw | (u8::from(value) << 3);
        data[field::SPARE_WHEEL] = raw;
    }

    /// Set the under-inflation system state field.
    #[inline]
    pub fn set_system_state(&mut self, value: UnderInflationSystemState) {
        let data = self.buffer.as_mut();
        let raw = data[field::SYSTEM_STATE] & !0xe0;
        let raw = raw | (u8::from(value) << 5);
        data[field::SYSTEM_STATE] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x1e1 ({})", err)?;
                Ok(())
            }
        }
    }
}

impl<T: AsRef<[u8]>> AsRef<[u8]> for Frame<T> {
    fn as_ref(&self) -> &[u8] {
        self.buffer.as_ref()
    }
}

/// A high-level representation of a x1e1 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub front_left_wheel_pax_state: PAXWheelState,
    pub front_left_wheel_state: WheelState,
    pub front_right_wheel_pax_state: PAXWheelState,
    pub front_right_wheel_state: WheelState,
    pub rear_left_wheel_pax_state: PAXWheelState,
    pub rear_left_wheel_state: WheelState,
    pub rear_right_wheel_pax_state: PAXWheelState,
    pub rear_right_wheel_state: WheelState,
    pub spare_wheel_state: WheelState,
    pub system_state: UnderInflationSystemState,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            front_left_wheel_pax_state: frame.front_left_wheel_pax_state(),
            front_left_wheel_state: frame.front_left_wheel_state(),
            front_right_wheel_pax_state: frame.front_right_wheel_pax_state(),
            front_right_wheel_state: frame.front_right_wheel_state(),
            rear_left_wheel_pax_state: frame.rear_left_wheel_pax_state(),
            rear_left_wheel_state: frame.rear_left_wheel_state(),
            rear_right_wheel_pax_state: frame.rear_right_wheel_pax_state(),
            rear_right_wheel_state: frame.rear_right_wheel_state(),
            spare_wheel_state: frame.spare_wheel_state(),
            system_state: frame.system_state(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x1e1 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_front_left_wheel_pax_state(self.front_left_wheel_pax_state);
        frame.set_front_left_wheel_state(self.front_left_wheel_state);
        frame.set_front_right_wheel_pax_state(self.front_right_wheel_pax_state);
        frame.set_front_right_wheel_state(self.front_right_wheel_state);
        frame.set_rear_left_wheel_pax_state(self.rear_left_wheel_pax_state);
        frame.set_rear_left_wheel_state(self.rear_left_wheel_state);
        frame.set_rear_right_wheel_pax_state(self.rear_right_wheel_pax_state);
        frame.set_rear_right_wheel_state(self.rear_right_wheel_state);
        frame.set_spare_wheel_state(self.spare_wheel_state);
        frame.set_system_state(self.system_state);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x1e1")?;
        writeln!(f, " front_left_wheel_state={}", self.front_left_wheel_state)?;
        writeln!(
            f,
            " front_right_wheel_state={}",
            self.front_right_wheel_state
        )?;
        writeln!(f, " rear_left_wheel_state={}", self.rear_left_wheel_state)?;
        writeln!(f, " rear_right_wheel_state={}", self.rear_right_wheel_state)?;
        writeln!(f, " spare_wheel_state={}", self.spare_wheel_state)?;
        writeln!(
            f,
            " front_left_wheel_pax_state={}",
            self.front_left_wheel_pax_state
        )?;
        writeln!(
            f,
            " front_right_wheel_pax_state={}",
            self.front_right_wheel_pax_state
        )?;
        writeln!(
            f,
            " rear_left_wheel_pax_state={}",
            self.rear_left_wheel_pax_state
        )?;
        writeln!(
            f,
            " rear_right_wheel_pax_state={}",
            self.rear_right_wheel_pax_state
        )?;
        writeln!(f, " spare_wheel_state={}", self.spare_wheel_state)?;
        writeln!(f, " system_state={}", self.system_state)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};

    use crate::{
        vehicle::{PAXWheelState, UnderInflationSystemState, WheelState},
        Error,
    };

    static REPR_FRAME_BYTES: [u8; 8] = [0x80, 0x22, 0x14, 0x08, 0x40, 0xe0, 0x00, 0x00];

    fn frame_repr() -> Repr {
        Repr {
            front_left_wheel_state: WheelState::Normal,
            front_right_wheel_state: WheelState::HighlyDeflated,
            rear_left_wheel_state: WheelState::LightlyDeflated,
            rear_right_wheel_state: WheelState::Puncture,
            spare_wheel_state: WheelState::NotMonitored,
            front_left_wheel_pax_state: PAXWheelState::Normal,
            front_right_wheel_pax_state: PAXWheelState::Puncture,
            rear_left_wheel_pax_state: PAXWheelState::Unavailable,
            rear_right_wheel_pax_state: PAXWheelState::Normal,
            system_state: UnderInflationSystemState::Ok,
        }
    }

    #[test]
    fn test_frame_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.front_left_wheel_state(), WheelState::Normal);
        assert_eq!(frame.front_right_wheel_state(), WheelState::HighlyDeflated);
        assert_eq!(frame.rear_left_wheel_state(), WheelState::LightlyDeflated);
        assert_eq!(frame.rear_right_wheel_state(), WheelState::Puncture);
        assert_eq!(frame.spare_wheel_state(), WheelState::NotMonitored);
        assert_eq!(frame.front_left_wheel_pax_state(), PAXWheelState::Normal);
        assert_eq!(frame.front_right_wheel_pax_state(), PAXWheelState::Puncture);
        assert_eq!(
            frame.rear_left_wheel_pax_state(),
            PAXWheelState::Unavailable
        );
        assert_eq!(frame.rear_right_wheel_pax_state(), PAXWheelState::Normal);
        assert_eq!(frame.system_state(), UnderInflationSystemState::Ok);
    }

    #[test]
    fn test_frame_construction() {
        let mut bytes = [0u8; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_front_left_wheel_state(WheelState::Normal);
        frame.set_front_right_wheel_state(WheelState::HighlyDeflated);
        frame.set_rear_left_wheel_state(WheelState::LightlyDeflated);
        frame.set_rear_right_wheel_state(WheelState::Puncture);
        frame.set_spare_wheel_state(WheelState::NotMonitored);
        frame.set_front_left_wheel_pax_state(PAXWheelState::Normal);
        frame.set_front_right_wheel_pax_state(PAXWheelState::Puncture);
        frame.set_rear_left_wheel_pax_state(PAXWheelState::Unavailable);
        frame.set_rear_right_wheel_pax_state(PAXWheelState::Normal);
        frame.set_system_state(UnderInflationSystemState::Ok);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 9] = [0x80, 0x22, 0x14, 0x08, 0x40, 0xe0, 0x00, 0x00, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 7] = [0x80, 0x22, 0x14, 0x08, 0x40, 0xe0, 0x00];
        assert_eq!(Frame::new_checked(&bytes).unwrap_err(), Error::Truncated);
    }

    #[test]
    fn test_repr_parse_valid() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES);
        let repr = Repr::parse(&frame).unwrap();
        assert_eq!(repr, frame_repr());
    }

    #[test]
    fn test_basic_repr_emit() {
        let mut buf = [0u8; 8];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }
}
