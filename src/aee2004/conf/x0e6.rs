use core::{cmp::Ordering, fmt, time::Duration};

use byteorder::{ByteOrder, NetworkEndian};

use crate::{
    vehicle::{SlopeType, StopAndStartBrakeRequirement},
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
    /// 1-bit ABS failure lamp ON request flag,
    /// 1-bit low level brake fluid alert flag,
    /// 1-bit worn brake pad flag,
    /// 1-bit Electronic Brakeforce Distribution in regulation flag,
    /// 1-bit Automatic hazard warning lamps managed by brake control unit flag.
    /// 1-bit ABS in regulation flag,
    /// 1-bit ABS failure flag,
    /// 1-bit Electronic Brakeforce Distribution failure lamp ON request flag.
    pub const FLAGS_1: usize = 0;
    /// 15-bit rear left wheel counter field,
    /// 1-bit rear left wheel counter failure flag.
    pub const CNT_REAR_LEFT: Field = 1..3;
    /// 15-bit rear right wheel counter field,
    /// 1-bit rear right wheel counter failure flag.
    pub const CNT_REAR_RIGHT: Field = 3..5;
    /// 8-bit battery voltage in 0.1 volt unit field.
    pub const BAT_VOLTAGE: usize = 5;
    /// 2-bit unknown,
    /// 2-bit slope type field,
    /// 2-bit Stop & Start braking request field,
    /// 1-bit 'GEE' failure flag,
    /// 1-bit Emergency Braking Warning managed by brake control unit flag.
    pub const FLAGS_2: usize = 6;
}

/// Length of a x0e6 CAN frame.
pub const FRAME_LEN: usize = field::FLAGS_2 + 1;

/// Periodicity of a x0e6 CAN frame.
pub const PERIODICITY: Duration = Duration::from_millis(100);

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
        match len.cmp(&FRAME_LEN) {
            Ordering::Less => Err(Error::Truncated),
            Ordering::Greater => Err(Error::Overlong),
            Ordering::Equal => Ok(()),
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

    /// Return the ABS failure lamp ON request flag.
    #[inline]
    pub fn abs_failure_lamp_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FLAGS_1] & 0x01 != 0
    }

    /// Return the low level brake fluid alert flag.
    #[inline]
    pub fn low_level_brake_fluid(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FLAGS_1] & 0x02 != 0
    }

    /// Return the worn brake pad flag.
    #[inline]
    pub fn worn_brake_pad(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FLAGS_1] & 0x04 != 0
    }

    /// Return the Electronic Brakeforce Distribution in regulation flag.
    #[inline]
    pub fn ebd_in_regulation(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FLAGS_1] & 0x08 != 0
    }

    /// Return the Automatic hazard warning lamps managed by brake control unit flag.
    #[inline]
    pub fn auto_hazard_lamps_managed_by_bcu(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FLAGS_1] & 0x10 != 0
    }

    /// Return the ABS in regulation flag.
    #[inline]
    pub fn abs_in_regulation(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FLAGS_1] & 0x20 != 0
    }

    /// Return the ABS failure flag.
    #[inline]
    pub fn abs_failure(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FLAGS_1] & 0x40 != 0
    }

    /// Return the Electronic Brakeforce Distribution failure lamp ON request flag.
    #[inline]
    pub fn ebd_failure_lamp_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FLAGS_1] & 0x80 != 0
    }

    /// Return the rear left wheel counter field.
    #[inline]
    pub fn rear_left_wheel_counter(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::CNT_REAR_LEFT]) & 0x7fff
    }

    /// Return the rear left wheel counter failure flag.
    #[inline]
    pub fn rear_left_wheel_counter_failure(&self) -> bool {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::CNT_REAR_LEFT]);
        raw & !0x7fff != 0
    }

    /// Return the rear right wheel counter field.
    #[inline]
    pub fn rear_right_wheel_counter(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::CNT_REAR_RIGHT]) & 0x7fff
    }

    /// Return the rear right wheel counter failure flag.
    #[inline]
    pub fn rear_right_wheel_counter_failure(&self) -> bool {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::CNT_REAR_RIGHT]);
        raw & !0x7fff != 0
    }

    /// Return the battery voltage in 0.1 volt unit field.
    #[inline]
    pub fn battery_voltage(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::BAT_VOLTAGE]
    }

    /// Return the slope type field.
    #[inline]
    pub fn slope_type(&self) -> SlopeType {
        let data = self.buffer.as_ref();
        let raw = (data[field::FLAGS_2] & 0x0c) >> 2;
        SlopeType::from(raw)
    }

    /// Return the Stop & Start braking request field.
    #[inline]
    pub fn stop_start_brake_req(&self) -> StopAndStartBrakeRequirement {
        let data = self.buffer.as_ref();
        let raw = (data[field::FLAGS_2] & 0x30) >> 4;
        StopAndStartBrakeRequirement::from(raw)
    }

    /// Return the 'GEE' failure flag.
    #[inline]
    pub fn gee_failure(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FLAGS_2] & 0x40 != 0
    }

    /// Return the Emergency Braking Warning managed by brake control unit flag.
    #[inline]
    pub fn ebw_managed_by_bcu(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::FLAGS_2] & 0x80 != 0
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the ABS failure lamp ON request flag.
    #[inline]
    pub fn set_abs_failure_lamp_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_1] & !0x01;
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::FLAGS_1] = raw;
    }

    /// Set the low level brake fluid alert flag.
    #[inline]
    pub fn set_low_level_brake_fluid(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_1] & !0x02;
        let raw = if value { raw | 0x02 } else { raw & !0x02 };
        data[field::FLAGS_1] = raw;
    }

    /// Set the worn brake pad flag.
    #[inline]
    pub fn set_worn_brake_pad(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_1] & !0x04;
        let raw = if value { raw | 0x04 } else { raw & !0x04 };
        data[field::FLAGS_1] = raw;
    }

    /// Set the Electronic Brakeforce Distribution in regulation flag.
    #[inline]
    pub fn set_ebd_in_regulation(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_1] & !0x08;
        let raw = if value { raw | 0x08 } else { raw & !0x08 };
        data[field::FLAGS_1] = raw;
    }

    /// Set the Automatic hazard warning lamps managed by brake control unit flag.
    #[inline]
    pub fn set_auto_hazard_lamps_managed_by_bcu(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_1] & !0x10;
        let raw = if value { raw | 0x10 } else { raw & !0x10 };
        data[field::FLAGS_1] = raw;
    }

    /// Set the ABS in regulation flag.
    #[inline]
    pub fn set_abs_in_regulation(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_1] & !0x20;
        let raw = if value { raw | 0x20 } else { raw & !0x20 };
        data[field::FLAGS_1] = raw;
    }

    /// Set the ABS failure flag.
    #[inline]
    pub fn set_abs_failure(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_1] & !0x40;
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::FLAGS_1] = raw;
    }

    /// Set the Electronic Brakeforce Distribution failure lamp ON request flag.
    #[inline]
    pub fn set_ebd_failure_lamp_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_1] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::FLAGS_1] = raw;
    }

    /// Set the rear left wheel counter field.
    #[inline]
    pub fn set_rear_left_wheel_counter(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u16(&data[field::CNT_REAR_LEFT]);
        let raw = raw | (value & 0x7fff);
        NetworkEndian::write_u16(&mut data[field::CNT_REAR_LEFT], raw);
    }

    /// Set the rear left wheel counter failure flag.
    #[inline]
    pub fn set_rear_left_wheel_counter_failure(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u16(&data[field::CNT_REAR_LEFT]);
        let raw = if value { raw | 0x8000 } else { raw & !0x8000 };
        NetworkEndian::write_u16(&mut data[field::CNT_REAR_LEFT], raw);
    }

    /// Set the rear right wheel counter field.
    #[inline]
    pub fn set_rear_right_wheel_counter(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u16(&data[field::CNT_REAR_RIGHT]);
        let raw = raw | (value & 0x7fff);
        NetworkEndian::write_u16(&mut data[field::CNT_REAR_RIGHT], raw);
    }

    /// Set the rear right wheel counter failure flag.
    #[inline]
    pub fn set_rear_right_wheel_counter_failure(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u16(&data[field::CNT_REAR_RIGHT]);
        let raw = if value { raw | 0x8000 } else { raw & !0x8000 };
        NetworkEndian::write_u16(&mut data[field::CNT_REAR_RIGHT], raw);
    }

    /// Set the battery voltage in 0.1 volt unit field.
    #[inline]
    pub fn set_battery_voltage(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::BAT_VOLTAGE] = value;
    }

    /// Set the slope type field.
    #[inline]
    pub fn set_slope_type(&mut self, value: SlopeType) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_2] & !0x0c;
        let raw = raw | ((u8::from(value) << 2) & 0x0c);
        data[field::FLAGS_2] = raw;
    }

    /// Set the Stop & Start braking request field.
    #[inline]
    pub fn set_stop_start_brake_req(&mut self, value: StopAndStartBrakeRequirement) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_2] & !0x30;
        let raw = raw | ((u8::from(value) << 4) & 0x30);
        data[field::FLAGS_2] = raw;
    }

    /// Set the 'GEE' failure flag.
    #[inline]
    pub fn set_gee_failure(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_2] & !0x40;
        let raw = if value { raw | 0x40 } else { raw & !0x40 };
        data[field::FLAGS_2] = raw;
    }

    /// Set the Emergency Braking Warning managed by brake control unit flag.
    #[inline]
    pub fn set_ebw_managed_by_bcu(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::FLAGS_2] & !0x80;
        let raw = if value { raw | 0x80 } else { raw & !0x80 };
        data[field::FLAGS_2] = raw;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x0e6 ({})", err)?;
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

/// A high-level representation of a x0e6 CAN frame.
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub abs_failure_lamp_request: bool,
    pub low_level_brake_fluid: bool,
    pub worn_brake_pad: bool,
    pub ebd_in_regulation: bool,
    pub auto_hazard_lamps_managed_by_bcu: bool,
    pub abs_in_regulation: bool,
    pub abs_failure: bool,
    pub ebd_failure_lamp_request: bool,
    pub rear_left_wheel_counter: u16,
    pub rear_left_wheel_counter_failure: bool,
    pub rear_right_wheel_counter: u16,
    pub rear_right_wheel_counter_failure: bool,
    #[cfg(feature = "float")]
    pub battery_voltage: f32,
    #[cfg(not(feature = "float"))]
    pub battery_voltage: u8,
    pub slope_type: SlopeType,
    pub stop_start_brake_req: StopAndStartBrakeRequirement,
    pub gee_failure: bool,
    pub ebw_managed_by_bcu: bool,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        Ok(Repr {
            abs_failure_lamp_request: frame.abs_failure_lamp_request(),
            low_level_brake_fluid: frame.low_level_brake_fluid(),
            worn_brake_pad: frame.worn_brake_pad(),
            ebd_in_regulation: frame.ebd_in_regulation(),
            auto_hazard_lamps_managed_by_bcu: frame.auto_hazard_lamps_managed_by_bcu(),
            abs_in_regulation: frame.abs_in_regulation(),
            abs_failure: frame.abs_failure(),
            ebd_failure_lamp_request: frame.ebd_failure_lamp_request(),
            rear_left_wheel_counter: frame.rear_left_wheel_counter(),
            rear_left_wheel_counter_failure: frame.rear_left_wheel_counter_failure(),
            rear_right_wheel_counter: frame.rear_right_wheel_counter(),
            rear_right_wheel_counter_failure: frame.rear_right_wheel_counter_failure(),
            #[cfg(feature = "float")]
            battery_voltage: (frame.battery_voltage() as f32) / 10.0,
            #[cfg(not(feature = "float"))]
            battery_voltage: frame.battery_voltage(),
            slope_type: frame.slope_type(),
            stop_start_brake_req: frame.stop_start_brake_req(),
            gee_failure: frame.gee_failure(),
            ebw_managed_by_bcu: frame.ebw_managed_by_bcu(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x0e6 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.set_abs_failure_lamp_request(self.abs_failure_lamp_request);
        frame.set_low_level_brake_fluid(self.low_level_brake_fluid);
        frame.set_worn_brake_pad(self.worn_brake_pad);
        frame.set_ebd_in_regulation(self.ebd_in_regulation);
        frame.set_auto_hazard_lamps_managed_by_bcu(self.auto_hazard_lamps_managed_by_bcu);
        frame.set_abs_in_regulation(self.abs_in_regulation);
        frame.set_abs_failure(self.abs_failure);
        frame.set_ebd_failure_lamp_request(self.ebd_failure_lamp_request);
        frame.set_rear_left_wheel_counter(self.rear_left_wheel_counter);
        frame.set_rear_left_wheel_counter_failure(self.rear_left_wheel_counter_failure);
        frame.set_rear_right_wheel_counter(self.rear_right_wheel_counter);
        frame.set_rear_right_wheel_counter_failure(self.rear_right_wheel_counter_failure);
        #[cfg(feature = "float")]
        frame.set_battery_voltage((self.battery_voltage * 10.0) as u8);
        #[cfg(not(feature = "float"))]
        frame.set_battery_voltage(self.battery_voltage);
        frame.set_slope_type(self.slope_type);
        frame.set_stop_start_brake_req(self.stop_start_brake_req);
        frame.set_gee_failure(self.gee_failure);
        frame.set_ebw_managed_by_bcu(self.ebw_managed_by_bcu);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x0e6")?;
        writeln!(
            f,
            " abs_failure_lamp_request={}",
            self.abs_failure_lamp_request
        )?;
        writeln!(f, " low_level_brake_fluid={}", self.low_level_brake_fluid)?;
        writeln!(f, " worn_brake_pad={}", self.worn_brake_pad)?;
        writeln!(f, " ebd_in_regulation={}", self.ebd_in_regulation)?;
        writeln!(
            f,
            " auto_hazard_lamps_managed_by_bcu={}",
            self.auto_hazard_lamps_managed_by_bcu
        )?;
        writeln!(f, " abs_in_regulation={}", self.abs_in_regulation)?;
        writeln!(f, " abs_failure={}", self.abs_failure)?;
        writeln!(
            f,
            " ebd_failure_lamp_request={}",
            self.ebd_failure_lamp_request
        )?;
        writeln!(
            f,
            " rear_left_wheel_counter={}",
            self.rear_left_wheel_counter
        )?;
        writeln!(
            f,
            " rear_left_wheel_counter_failure={}",
            self.rear_left_wheel_counter_failure
        )?;
        writeln!(
            f,
            " rear_right_wheel_counter={}",
            self.rear_right_wheel_counter
        )?;
        writeln!(
            f,
            " rear_right_wheel_counter_failure={}",
            self.rear_right_wheel_counter_failure
        )?;
        writeln!(f, " battery_voltage={}", self.battery_voltage)?;
        writeln!(f, " slope_type={}", self.slope_type)?;
        writeln!(f, " stop_start_brake_req={}", self.stop_start_brake_req)?;
        writeln!(f, " gee_failure={}", self.gee_failure)?;
        writeln!(f, " ebw_managed_by_bcu={}", self.ebw_managed_by_bcu)
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::{
        vehicle::{SlopeType, StopAndStartBrakeRequirement},
        Error,
    };

    static REPR_FRAME_BYTES_1: [u8; 7] = [0x55, 0x2c, 0x15, 0x82, 0x26, 0x86, 0x80];
    static REPR_FRAME_BYTES_2: [u8; 7] = [0xaa, 0x82, 0x0e, 0x21, 0x71, 0x8d, 0x64];

    fn frame_1_repr() -> Repr {
        Repr {
            abs_failure_lamp_request: true,
            low_level_brake_fluid: false,
            worn_brake_pad: true,
            ebd_in_regulation: false,
            auto_hazard_lamps_managed_by_bcu: true,
            abs_in_regulation: false,
            abs_failure: true,
            ebd_failure_lamp_request: false,
            rear_left_wheel_counter: 11285,
            rear_left_wheel_counter_failure: false,
            rear_right_wheel_counter: 550,
            rear_right_wheel_counter_failure: true,
            battery_voltage: 13.4,
            slope_type: SlopeType::Light,
            stop_start_brake_req: StopAndStartBrakeRequirement::Nothing,
            gee_failure: false,
            ebw_managed_by_bcu: true,
        }
    }

    fn frame_2_repr() -> Repr {
        Repr {
            abs_failure_lamp_request: false,
            low_level_brake_fluid: true,
            worn_brake_pad: false,
            ebd_in_regulation: true,
            auto_hazard_lamps_managed_by_bcu: false,
            abs_in_regulation: true,
            abs_failure: false,
            ebd_failure_lamp_request: true,
            rear_left_wheel_counter: 526,
            rear_left_wheel_counter_failure: true,
            rear_right_wheel_counter: 8561,
            rear_right_wheel_counter_failure: false,
            battery_voltage: 14.1,
            slope_type: SlopeType::SteepUpward,
            stop_start_brake_req: StopAndStartBrakeRequirement::Restart,
            gee_failure: true,
            ebw_managed_by_bcu: false,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.abs_failure_lamp_request(), true);
        assert_eq!(frame.low_level_brake_fluid(), false);
        assert_eq!(frame.worn_brake_pad(), true);
        assert_eq!(frame.ebd_in_regulation(), false);
        assert_eq!(frame.auto_hazard_lamps_managed_by_bcu(), true);
        assert_eq!(frame.abs_in_regulation(), false);
        assert_eq!(frame.abs_failure(), true);
        assert_eq!(frame.ebd_failure_lamp_request(), false);
        assert_eq!(frame.rear_left_wheel_counter(), 11285);
        assert_eq!(frame.rear_left_wheel_counter_failure(), false);
        assert_eq!(frame.rear_right_wheel_counter(), 550);
        assert_eq!(frame.rear_right_wheel_counter_failure(), true);
        assert_eq!(frame.battery_voltage(), 134);
        assert_eq!(frame.slope_type(), SlopeType::Light);
        assert_eq!(
            frame.stop_start_brake_req(),
            StopAndStartBrakeRequirement::Nothing
        );
        assert_eq!(frame.gee_failure(), false);
        assert_eq!(frame.ebw_managed_by_bcu(), true);
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.abs_failure_lamp_request(), false);
        assert_eq!(frame.low_level_brake_fluid(), true);
        assert_eq!(frame.worn_brake_pad(), false);
        assert_eq!(frame.ebd_in_regulation(), true);
        assert_eq!(frame.auto_hazard_lamps_managed_by_bcu(), false);
        assert_eq!(frame.abs_in_regulation(), true);
        assert_eq!(frame.abs_failure(), false);
        assert_eq!(frame.ebd_failure_lamp_request(), true);
        assert_eq!(frame.rear_left_wheel_counter(), 526);
        assert_eq!(frame.rear_left_wheel_counter_failure(), true);
        assert_eq!(frame.rear_right_wheel_counter(), 8561);
        assert_eq!(frame.rear_right_wheel_counter_failure(), false);
        assert_eq!(frame.battery_voltage(), 141);
        assert_eq!(frame.slope_type(), SlopeType::SteepUpward);
        assert_eq!(
            frame.stop_start_brake_req(),
            StopAndStartBrakeRequirement::Restart
        );
        assert_eq!(frame.gee_failure(), true);
        assert_eq!(frame.ebw_managed_by_bcu(), false);
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0x00; 7];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_abs_failure_lamp_request(true);
        frame.set_low_level_brake_fluid(false);
        frame.set_worn_brake_pad(true);
        frame.set_ebd_in_regulation(false);
        frame.set_auto_hazard_lamps_managed_by_bcu(true);
        frame.set_abs_in_regulation(false);
        frame.set_abs_failure(true);
        frame.set_ebd_failure_lamp_request(false);
        frame.set_rear_left_wheel_counter(11285);
        frame.set_rear_left_wheel_counter_failure(false);
        frame.set_rear_right_wheel_counter(550);
        frame.set_rear_right_wheel_counter_failure(true);
        frame.set_battery_voltage(134);
        frame.set_slope_type(SlopeType::Light);
        frame.set_stop_start_brake_req(StopAndStartBrakeRequirement::Nothing);
        frame.set_gee_failure(false);
        frame.set_ebw_managed_by_bcu(true);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0x00; 7];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_abs_failure_lamp_request(false);
        frame.set_low_level_brake_fluid(true);
        frame.set_worn_brake_pad(false);
        frame.set_ebd_in_regulation(true);
        frame.set_auto_hazard_lamps_managed_by_bcu(false);
        frame.set_abs_in_regulation(true);
        frame.set_abs_failure(false);
        frame.set_ebd_failure_lamp_request(true);
        frame.set_rear_left_wheel_counter(526);
        frame.set_rear_left_wheel_counter_failure(true);
        frame.set_rear_right_wheel_counter(8561);
        frame.set_rear_right_wheel_counter_failure(false);
        frame.set_battery_voltage(141);
        frame.set_slope_type(SlopeType::SteepUpward);
        frame.set_stop_start_brake_req(StopAndStartBrakeRequirement::Restart);
        frame.set_gee_failure(true);
        frame.set_ebw_managed_by_bcu(false);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 8] = [0x55, 0x2c, 0x15, 0x82, 0x26, 0x86, 0x80, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 6] = [0x55, 0x2c, 0x15, 0x82, 0x26, 0x86];
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
        let mut buf = [0u8; 7];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_1_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_basic_repr_2_emit() {
        let mut buf = [0u8; 7];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_2_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }
}
