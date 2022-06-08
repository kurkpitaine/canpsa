use core::{cmp::Ordering, fmt, time::Duration};

use byteorder::{ByteOrder, NetworkEndian};
use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time};

use crate::{
    config::{ClockFormat, DisplayMode},
    Error, Result, YEAR_OFFSET,
};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

/*
276 DONNEES_VSM_LENTES_3_AFFICHAGE_HORLOGE_HS7_276          // OK
276 DONNEES_VSM_LENTES_3_ANNEE_HORLOGE_HS7_276              // OK
276 DONNEES_VSM_LENTES_3_AUTONOMIE_UREE_HS7_276             // OK
276 DONNEES_VSM_LENTES_3_DMD_AFF_AUTON_UREE_HS7_276         // OK
276 DONNEES_VSM_LENTES_3_ETAT_CONDA_COFFRE_HS7_276
276 DONNEES_VSM_LENTES_3_ETAT_CONDA_HS7_276
276 DONNEES_VSM_LENTES_3_ETAT_CONDA_OUV_AR_HS7_276
276 DONNEES_VSM_LENTES_3_ETAT_SELECTIVITE_CAB_ZC_HS7_276
276 DONNEES_VSM_LENTES_3_ETAT_SELECTIVITE_COND_HS7_276
276 DONNEES_VSM_LENTES_3_ETAT_SELECTIVITE_HS7_276
276 DONNEES_VSM_LENTES_3_ETAT_SELECTIVITE_OUV_AR_HS7_276
276 DONNEES_VSM_LENTES_3_HEURE_HORLOGE_HS7_276              // OK
276 DONNEES_VSM_LENTES_3_JOUR_HORLOGE_HS7_276               // OK
276 DONNEES_VSM_LENTES_3_MINUTE_HORLOGE_HS7_276             // OK
276 DONNEES_VSM_LENTES_3_MODE_HEURE_CLIENT_HS7_276          // OK
276 DONNEES_VSM_LENTES_3_MOIS_HORLOGE_HS7_276               // OK
*/

mod field {
    use crate::field::*;
    /// 7-bit clock year,
    /// 1-bit clock format.
    pub const YEAR_CLK_FMT: usize = 0;
    /// 4-bit clock month,
    /// 1-bit clock display mode,
    /// 3-bit empty.
    pub const MONTH_CLOCK_DISP_MODE: usize = 1;
    /// 6-bit clock day,
    /// 2-bit empty.
    pub const DAY: usize = 2;
    /// 5-bit clock hour,
    /// 3-bit empty.
    pub const HOUR: usize = 3;
    /// 6-bit clock minute,
    /// 2-bit empty.
    pub const MINUTE: usize = 4;
    /// 14-bit AdBlue autonomy,
    /// 1-bit empty,
    /// 1-bit AdBlue autonomy display request.
    pub const FLAGS_ADBLUE_AUTONOMY: Field = 5..7;
}

/// Raw x276 CAN frame identifier.
pub const FRAME_ID: u16 = 0x276;
/// Length of a x276 CAN frame.
pub const FRAME_LEN: usize = field::FLAGS_ADBLUE_AUTONOMY.end;

/// Periodicity of a x276 CAN frame.
pub const PERIODICITY: Duration = Duration::from_millis(1000);

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

    /// Return the clock format field.
    #[inline]
    pub fn clock_format(&self) -> ClockFormat {
        let data = self.buffer.as_ref();
        let raw = (data[field::YEAR_CLK_FMT] & 0x80) >> 7;
        ClockFormat::from(raw)
    }

    /// Return the year field.
    #[inline]
    pub fn year(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::YEAR_CLK_FMT] & 0x7f
    }

    /// Return the month field.
    #[inline]
    pub fn month(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::MONTH_CLOCK_DISP_MODE] & 0x0f
    }

    /// Return the clock display mode field.
    #[inline]
    pub fn clock_display_mode(&self) -> DisplayMode {
        let data = self.buffer.as_ref();
        let raw = (data[field::MONTH_CLOCK_DISP_MODE] & 0x10) >> 4;
        DisplayMode::from(raw)
    }

    /// Return the day field.
    #[inline]
    pub fn day(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::DAY] & 0x3f
    }

    /// Return the hour field.
    #[inline]
    pub fn hour(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::HOUR] & 0x1f
    }

    /// Return the minute field.
    #[inline]
    pub fn minute(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::MINUTE] & 0x3f
    }

    /// Return the Adblue autonomy field.
    #[inline]
    pub fn adblue_autonomy(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::FLAGS_ADBLUE_AUTONOMY]) & 0x3fff
    }

    /// Return the AdBlue autonomy display request field.
    #[inline]
    pub fn adblue_autonomy_display_request(&self) -> bool {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::FLAGS_ADBLUE_AUTONOMY]);
        raw & !0x7fff != 0
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the clock format field.
    #[inline]
    pub fn set_clock_format(&mut self, value: ClockFormat) {
        let data = self.buffer.as_mut();
        let raw = data[field::YEAR_CLK_FMT] & !0x80;
        let raw = raw | (u8::from(value) << 7);
        data[field::YEAR_CLK_FMT] = raw;
    }

    /// Set the year field.
    #[inline]
    pub fn set_year(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::YEAR_CLK_FMT] & !0x7f;
        let raw = raw | (value & 0x7f);
        data[field::YEAR_CLK_FMT] = raw
    }

    /// Set the month field.
    #[inline]
    pub fn set_month(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::MONTH_CLOCK_DISP_MODE] & !0x0f;
        let raw = raw | value & 0x0f;
        data[field::MONTH_CLOCK_DISP_MODE] = raw;
    }

    /// Set the clock display mode field.
    #[inline]
    pub fn set_clock_display_mode(&mut self, value: DisplayMode) {
        let data = self.buffer.as_mut();
        let raw = data[field::MONTH_CLOCK_DISP_MODE] & !0x10;
        let raw = raw | ((u8::from(value) << 4) & 0x10);
        data[field::MONTH_CLOCK_DISP_MODE] = raw;
    }

    /// Set the day field.
    #[inline]
    pub fn set_day(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::DAY] & !0x3f;
        let raw = raw | value & 0x3f;
        data[field::DAY] = raw;
    }

    /// Set the hour field.
    #[inline]
    pub fn set_hour(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::HOUR] & !0x1f;
        let raw = raw | value & 0x1f;
        data[field::HOUR] = raw;
    }

    /// Set the minute field.
    #[inline]
    pub fn set_minute(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        let raw = data[field::MINUTE] & !0x3f;
        let raw = raw | value & 0x3f;
        data[field::MINUTE] = raw;
    }

    /// Set the Adblue autonomy field.
    #[inline]
    pub fn set_adblue_autonomy(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u16(&data[field::FLAGS_ADBLUE_AUTONOMY]);
        let raw = raw | (value & 0x3fff);
        NetworkEndian::write_u16(&mut data[field::FLAGS_ADBLUE_AUTONOMY], raw);
    }

    /// Set the AdBlue autonomy display request field.
    #[inline]
    pub fn set_adblue_autonomy_display_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = NetworkEndian::read_u16(&data[field::FLAGS_ADBLUE_AUTONOMY]);
        let raw = if value { raw | 0x8000 } else { raw & !0x8000 };
        NetworkEndian::write_u16(&mut data[field::FLAGS_ADBLUE_AUTONOMY], raw);
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x276 ({})", err)?;
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

/// A high-level representation of a x276 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub clock_format: ClockFormat,
    pub clock_disp_mode: DisplayMode,
    pub utc_datetime: OffsetDateTime,
    pub adblue_autonomy: u16,
    pub adblue_autonomy_display_request: bool,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        // month values:
        //  - 0x00 and 0x0d are not used
        //  - 0x0e means unavailable
        //  - 0x0f means invalid value.

        let date = Date::from_calendar_date(
            YEAR_OFFSET + (frame.year() as i32),
            Month::try_from(frame.month()).map_err(|_| Error::Illegal)?,
            frame.day(),
        )
        .map_err(|_| Error::Illegal)?;

        let time = Time::from_hms(frame.hour(), frame.minute(), 0).map_err(|_| Error::Illegal)?;
        let date_time = PrimitiveDateTime::new(date, time);
        let utc_datetime = OffsetDateTime::from_unix_timestamp(0).map_err(|_| Error::Illegal)?;

        Ok(Repr {
            clock_format: frame.clock_format(),
            clock_disp_mode: frame.clock_display_mode(),
            utc_datetime: utc_datetime.replace_date_time(date_time),
            adblue_autonomy: frame.adblue_autonomy(),
            adblue_autonomy_display_request: frame.adblue_autonomy_display_request(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x276 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        let can_year = self.utc_datetime.year() - YEAR_OFFSET;
        frame.set_clock_format(self.clock_format);
        frame.set_year(can_year as u8);
        frame.set_month(self.utc_datetime.month().into());
        frame.set_clock_display_mode(self.clock_disp_mode);
        frame.set_day(self.utc_datetime.day());
        frame.set_hour(self.utc_datetime.hour());
        frame.set_minute(self.utc_datetime.minute());
        frame.set_adblue_autonomy(self.adblue_autonomy);
        frame.set_adblue_autonomy_display_request(self.adblue_autonomy_display_request);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x276 clock_format={}", self.clock_format)?;
        writeln!(f, " clock_disp_mode={}", self.clock_disp_mode)?;
        writeln!(f, " utc_datetime={}", self.utc_datetime)?;
        writeln!(f, " adblue_autonomy={}", self.adblue_autonomy)?;
        writeln!(
            f,
            " adblue_autonomy_display_request={}",
            self.adblue_autonomy_display_request
        )
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};
    use crate::{
        config::{ClockFormat, DisplayMode},
        Error,
    };

    use time::macros::datetime;

    static REPR_FRAME_BYTES: [u8; 7] = [0x96, 0x11, 0x0a, 0x0f, 0x1d, 0x3f, 0xfe];

    fn frame_repr() -> Repr {
        Repr {
            clock_format: ClockFormat::H24,
            clock_disp_mode: DisplayMode::Blinking,
            utc_datetime: datetime!(2022-01-10 15:29 UTC),
            adblue_autonomy: 16382,
            adblue_autonomy_display_request: false,
        }
    }

    #[test]
    fn test_frame_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.clock_format(), ClockFormat::H24);
        assert_eq!(frame.year(), 0x16);
        assert_eq!(frame.month(), 0x01);
        assert_eq!(frame.clock_display_mode(), DisplayMode::Blinking);
        assert_eq!(frame.day(), 0x0a);
        assert_eq!(frame.hour(), 0x0f);
        assert_eq!(frame.minute(), 0x1d);
        assert_eq!(frame.adblue_autonomy(), 0x3ffe);
        assert_eq!(frame.adblue_autonomy_display_request(), false);
    }

    #[test]
    fn test_frame_construction() {
        let mut bytes = [0u8; 7];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_clock_format(ClockFormat::H24);
        frame.set_year(0x16);
        frame.set_month(0x01);
        frame.set_clock_display_mode(DisplayMode::Blinking);
        frame.set_day(0x0a);
        frame.set_hour(0x0f);
        frame.set_minute(0x1d);
        frame.set_adblue_autonomy(0x3ffe);
        frame.set_adblue_autonomy_display_request(false);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 8] = [0x96, 0x11, 0x0a, 0x0f, 0x1d, 0x3f, 0xfe, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 5] = [0x96, 0x11, 0x0a, 0x0f, 0x1d];
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
        let mut buf = [0u8; 7];
        let mut frame = Frame::new_unchecked(&mut buf);
        let repr = frame_repr();
        repr.emit(&mut frame);
        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES);
    }
}
