use core::fmt;

use byteorder::{ByteOrder, NetworkEndian};

use crate::{
    config::SpeedUnit,
    vehicle::{SpeedRegulationMode, SpeedRegulationModeState},
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
    /// 1-bit speed unit flag,
    /// 1-bit cruise-control/speed-limiter/acc try enable flag,
    /// 3-bit cruise-control/speed-limiter/acc activated mode state field,
    /// 2-bit cruise-control/speed-limiter/acc activated mode field.
    pub const FLAGS: usize = 0;
    /// 16-bit cruise-control/speed-limiter/acc speed setting field.
    pub const SPD_INST: Field = 1..3;
    /// 16-bit empty.
    pub const EMPTY: Field = 3..5;
    /// 24-bit partial odometer field, in 0.1 kilometers units.
    pub const ODOMETER: Field = 5..8;
}

/// Length of a x1a8 CAN frame.
pub const FRAME_LEN: usize = field::ODOMETER.end;

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

    /// Return the speed unit flag.
    #[inline]
    pub fn speed_unit(&self) -> SpeedUnit {
        let data = self.buffer.as_ref();
        let raw = (data[field::FLAGS] & 0x02) >> 1;
        SpeedUnit::from(raw)
    }

    /// Return the cruise-control/speed-limiter/acc try enable flag.
    #[inline]
    pub fn try_enable(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FLAGS] & 0x04 != 0
    }

    /// Return the cruise-control/speed-limiter/acc activated mode state field.
    #[inline]
    pub fn speed_regulation_mode_state(&self) -> SpeedRegulationModeState {
        let data = self.buffer.as_ref();
        let raw = (data[field::FLAGS] & 0x38) >> 3;
        SpeedRegulationModeState::from(raw)
    }

    /// Return the cruise-control/speed-limiter/acc activated mode field.
    #[inline]
    pub fn speed_regulation_mode(&self) -> SpeedRegulationMode {
        let data = self.buffer.as_ref();
        let raw = data[field::FLAGS] >> 6;
        SpeedRegulationMode::from(raw)
    }

    /// Return the cruise-control/speed-limiter/acc speed setting field.
    #[inline]
    pub fn speed_setting(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::SPD_INST])
    }

    /// Return the partial odometer field, in 0.1 kilometers units.
    #[inline]
    pub fn partial_odometer(&self) -> u32 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u24(&data[field::ODOMETER])
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the speed unit flag.
    #[inline]
    pub fn set_speed_unit(&mut self, value: SpeedUnit) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS] & !0x02;
        let raw = raw | ((u8::from(value) << 1) & 0x02);
        data[field::FLAGS] = raw;
    }

    /// Set the cruise-control/speed-limiter/acc try enable flag.
    #[inline]
    pub fn set_try_enable(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS];
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
        data[field::FLAGS] = raw;
    }

    /// Set the cruise-control/speed-limiter/acc activated mode state field.
    #[inline]
    pub fn set_speed_regulation_mode_state(&mut self, value: SpeedRegulationModeState) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS] & !0x38;
        let raw = raw | ((u8::from(value) << 3) & 0x38);
        data[field::FLAGS] = raw;
    }

    /// Set the cruise-control/speed-limiter/acc activated mode field.
    #[inline]
    pub fn set_speed_regulation_mode(&mut self, value: SpeedRegulationMode) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS] & !0xc0;
        let raw = raw | (u8::from(value) << 6);
        data[field::FLAGS] = raw;
    }

    /// Set the cruise-control/speed-limiter/acc speed setting field.
    #[inline]
    pub fn set_speed_setting(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::SPD_INST], value);
    }

    /// Set the partial odometer field, in 0.1 kilometers units.
    #[inline]
    pub fn set_partial_odometer(&mut self, value: u32) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u24(&mut data[field::ODOMETER], value);
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x1a8 ({})", err)?;
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

/// A high-level representation of a x1a8 CAN frame.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub speed_unit: SpeedUnit,
    pub try_enable: bool,
    pub speed_regulation_mode_state: SpeedRegulationModeState,
    pub speed_regulation_mode: SpeedRegulationMode,
    pub speed_setting: u16,
    pub partial_odometer: f32,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            speed_unit: frame.speed_unit(),
            try_enable: frame.try_enable(),
            speed_regulation_mode_state: frame.speed_regulation_mode_state(),
            speed_regulation_mode: frame.speed_regulation_mode(),
            speed_setting: frame.speed_setting(),
            partial_odometer: (frame.partial_odometer() as f32 / 10.0),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x1a8 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_speed_unit(self.speed_unit);
        frame.set_try_enable(self.try_enable);
        frame.set_speed_regulation_mode_state(self.speed_regulation_mode_state);
        frame.set_speed_regulation_mode(self.speed_regulation_mode);
        frame.set_speed_setting(self.speed_setting);
        frame.set_partial_odometer((self.partial_odometer * 10.0) as u32);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x1a8")?;
        write!(f, " speed_unit={}", self.speed_unit)?;
        write!(f, " try_enable={}", self.try_enable)?;
        write!(
            f,
            " speed_regulation_mode_state={}",
            self.speed_regulation_mode_state
        )?;
        write!(f, " speed_regulation_mode={}", self.speed_regulation_mode)?;
        write!(f, " speed_setting={}", self.speed_setting)?;
        write!(f, " partial_odometer={}", self.partial_odometer)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};

    use crate::{
        config::SpeedUnit,
        vehicle::{SpeedRegulationMode, SpeedRegulationModeState},
        Error,
    };

    static REPR_FRAME_BYTES_1: [u8; 8] = [0x44, 0x00, 0x82, 0x00, 0x00, 0x00, 0x19, 0x84];
    static REPR_FRAME_BYTES_2: [u8; 8] = [0x92, 0x00, 0x32, 0x00, 0x00, 0x00, 0x0c, 0xb6];

    fn frame_1_repr() -> Repr {
        Repr {
            speed_unit: SpeedUnit::Kph,
            try_enable: true,
            speed_regulation_mode_state: SpeedRegulationModeState::Standby,
            speed_regulation_mode: SpeedRegulationMode::CruiseControl,
            speed_setting: 130,
            partial_odometer: 653.2,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            speed_unit: SpeedUnit::Mph,
            try_enable: false,
            speed_regulation_mode_state: SpeedRegulationModeState::LimiterUpAndRunning,
            speed_regulation_mode: SpeedRegulationMode::SpeedLimiter,
            speed_setting: 50,
            partial_odometer: 325.4,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.speed_unit(), SpeedUnit::Kph);
        assert_eq!(frame.try_enable(), true);
        assert_eq!(
            frame.speed_regulation_mode_state(),
            SpeedRegulationModeState::Standby
        );
        assert_eq!(
            frame.speed_regulation_mode(),
            SpeedRegulationMode::CruiseControl
        );
        assert_eq!(frame.speed_setting(), 130);
        assert_eq!(frame.partial_odometer(), 6532);
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.speed_unit(), SpeedUnit::Mph);
        assert_eq!(frame.try_enable(), false);
        assert_eq!(
            frame.speed_regulation_mode_state(),
            SpeedRegulationModeState::LimiterUpAndRunning
        );
        assert_eq!(
            frame.speed_regulation_mode(),
            SpeedRegulationMode::SpeedLimiter
        );
        assert_eq!(frame.speed_setting(), 50);
        assert_eq!(frame.partial_odometer(), 3254);
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0x00; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_speed_unit(SpeedUnit::Kph);
        frame.set_try_enable(true);
        frame.set_speed_regulation_mode_state(SpeedRegulationModeState::Standby);
        frame.set_speed_regulation_mode(SpeedRegulationMode::CruiseControl);
        frame.set_speed_setting(130);
        frame.set_partial_odometer(6532);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0x00; 8];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_speed_unit(SpeedUnit::Mph);
        frame.set_try_enable(false);
        frame.set_speed_regulation_mode_state(SpeedRegulationModeState::LimiterUpAndRunning);
        frame.set_speed_regulation_mode(SpeedRegulationMode::SpeedLimiter);
        frame.set_speed_setting(50);
        frame.set_partial_odometer(3254);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 9] = [0x44, 0x00, 0x82, 0x00, 0x00, 0x00, 0x19, 0x84, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 7] = [0x44, 0x00, 0x82, 0x00, 0x00, 0x00, 0x19];
        assert_eq!(Frame::new_checked(&bytes).unwrap_err(), Error::Truncated);
    }

    #[test]
    fn test_repr_1_parse_valid() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        let repr = Repr::parse(&frame).unwrap();
        assert_eq!(repr, frame_1_repr());
    }

    #[test]
    fn test_repr_2_parse_valid() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        let repr = Repr::parse(&frame).unwrap();
        assert_eq!(repr, frame_2_repr());
    }

    #[test]
    fn test_basic_repr_1_emit() {
        let mut buf = [0u8; 8];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_1_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_basic_repr_2_emit() {
        let mut buf = [0u8; 8];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_2_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }
}
