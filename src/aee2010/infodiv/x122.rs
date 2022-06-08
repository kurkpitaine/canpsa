use core::{cmp::Ordering, fmt, time::Duration};

use crate::{Error, Result};

/// A read/write wrapper around an CAN frame buffer.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Frame<T: AsRef<[u8]>> {
    buffer: T,
}

/*
122 ETAT_FMUX_CPT_MOL_FAC_1_HS7_122     // OK
122 ETAT_FMUX_CPT_MOL_FAC_2_HS7_122     // OK
122 ETAT_FMUX_DMD_ADAS_RING_HS7_122
122 ETAT_FMUX_INFO_PUSH_ESP_HS7_122     // OK
122 ETAT_FMUX_PUSH_10_HS7_122           // OK
122 ETAT_FMUX_PUSH_11_HS7_122           // OK
122 ETAT_FMUX_PUSH_12_HS7_122           // OK
122 ETAT_FMUX_PUSH_13_HS7_122           // OK
122 ETAT_FMUX_PUSH_14_HS7_122           // OK
122 ETAT_FMUX_PUSH_15_HS7_122           // OK
122 ETAT_FMUX_PUSH_16_HS7_122           // OK
122 ETAT_FMUX_PUSH_17_HS7_122           // OK
122 ETAT_FMUX_PUSH_18_HS7_122           // OK
122 ETAT_FMUX_PUSH_19_HS7_122           // OK
122 ETAT_FMUX_PUSH_1_HS7_122            // OK
122 ETAT_FMUX_PUSH_20_HS7_122           // OK
122 ETAT_FMUX_PUSH_21_HS7_122           // OK
122 ETAT_FMUX_PUSH_22_HS7_122           // OK
122 ETAT_FMUX_PUSH_23_HS7_122           // OK
122 ETAT_FMUX_PUSH_24_HS7_122           // OK
122 ETAT_FMUX_PUSH_25_HS7_122           // OK
122 ETAT_FMUX_PUSH_26_HS7_122           // OK
122 ETAT_FMUX_PUSH_27_HS7_122           // OK
122 ETAT_FMUX_PUSH_28_HS7_122           // OK
122 ETAT_FMUX_PUSH_29_HS7_122           // OK
122 ETAT_FMUX_PUSH_2_HS7_122            // OK
122 ETAT_FMUX_PUSH_30_HS7_122           // OK
122 ETAT_FMUX_PUSH_31_HS7_122           // OK
122 ETAT_FMUX_PUSH_32_HS7_122           // OK
122 ETAT_FMUX_PUSH_33_HS7_122           // OK
122 ETAT_FMUX_PUSH_34_HS7_122           // OK
122 ETAT_FMUX_PUSH_35_HS7_122           // OK
122 ETAT_FMUX_PUSH_36_HS7_122           // OK
122 ETAT_FMUX_PUSH_37_HS7_122           // OK
122 ETAT_FMUX_PUSH_38_HS7_122           // OK
122 ETAT_FMUX_PUSH_39_HS7_122           // OK
122 ETAT_FMUX_PUSH_3_HS7_122            // OK
122 ETAT_FMUX_PUSH_40_HS7_122           // OK
122 ETAT_FMUX_PUSH_41_HS7_122           // OK
122 ETAT_FMUX_PUSH_42_HS7_122           // OK
122 ETAT_FMUX_PUSH_43_HS7_122           // OK
122 ETAT_FMUX_PUSH_44_HS7_122           // OK
122 ETAT_FMUX_PUSH_4_HS7_122            // OK
122 ETAT_FMUX_PUSH_5_HS7_122            // OK
122 ETAT_FMUX_PUSH_6_HS7_122            // OK
122 ETAT_FMUX_PUSH_7_HS7_122            // OK
122 ETAT_FMUX_PUSH_8_HS7_122            // OK
122 ETAT_FMUX_PUSH_9_HS7_122            // OK
122 ETAT_FMUX_PUSH_BP_HS7_122           // OK
122 ETAT_FMUX_SYNC_MOL_FAC_1_HS7_122    // OK
122 ETAT_FMUX_SYNC_MOL_FAC_2_HS7_122    // OK
*/

mod field {
    /// 8 * 1-bit push buttons state.
    pub const PUSH_BTN_FLAGS_0: usize = 0;
    /// 8 * 1-bit push buttons state.
    pub const PUSH_BTN_FLAGS_1: usize = 1;
    /// 8 * 1-bit push buttons state.
    pub const PUSH_BTN_FLAGS_2: usize = 2;
    /// 8 * 1-bit push buttons state.
    pub const PUSH_BTN_FLAGS_3: usize = 3;
    /// 8 * 1-bit push buttons state.
    pub const PUSH_BTN_FLAGS_4: usize = 4;
    /// 1-bit front panel second wheel sync request flag,
    /// 1-bit front panel first wheel sync request flag,
    /// 1-bit front panel 'BP' button button state,
    /// 1-bit front panel ESP button state,
    /// 4 * 1-bit push buttons state.
    pub const PUSH_BTN_FLAGS_5: usize = 5;
    /// 8-bit front panel first wheel ticks counter.
    pub const WHL_1_TICKS: usize = 6;
    /// 8-bit front panel second wheel ticks counter.
    pub const WHL_2_TICKS: usize = 7;
}

/// Raw x122 CAN frame identifier.
pub const FRAME_ID: u16 = 0x122;
/// Length of a x122 CAN frame.
pub const FRAME_LEN: usize = field::WHL_2_TICKS + 1;

/// Periodicity of a x122 CAN frame.
pub const PERIODICITY: Duration = Duration::from_millis(200);

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

    /// Return the button state in byte B at index I.
    #[inline]
    pub fn read_button_state<const B: usize, const I: u8>(&self) -> bool {
        let data = self.buffer.as_ref();
        (data[B] & (1u8 << I)) != 0
    }

    /// Return the front panel second wheel sync request flag.
    #[inline]
    pub fn fp_second_wheel_sync_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::PUSH_BTN_FLAGS_5] & 0x01 != 0
    }

    /// Return the front panel first wheel sync request flag.
    #[inline]
    pub fn fp_first_wheel_sync_request(&self) -> bool {
        let data = self.buffer.as_ref();
        data[field::PUSH_BTN_FLAGS_5] & 0x02 != 0
    }

    /// Return the front panel first wheel ticks counter.
    #[inline]
    pub fn fp_first_wheel_ticks_counter(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::WHL_1_TICKS]
    }

    /// Return the front panel second wheel ticks counter.
    #[inline]
    pub fn fp_second_wheel_ticks_counter(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::WHL_2_TICKS]
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Frame<T> {
    /// Set the button state in byte B at index I.
    #[inline]
    pub fn write_button_state<const B: usize, const I: u8>(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let mask = 1u8 << I;
        let raw = data[B];
        let raw = if value { raw | mask } else { raw & !mask };
        data[B] = raw;
    }

    /// Set the front panel second wheel sync request flag.
    #[inline]
    pub fn set_fp_second_wheel_sync_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::PUSH_BTN_FLAGS_5];
        let raw = if value { raw | 0x01 } else { raw & !0x01 };
        data[field::PUSH_BTN_FLAGS_5] = raw;
    }

    /// Set the front panel first wheel sync request flag.
    #[inline]
    pub fn set_fp_first_wheel_sync_request(&mut self, value: bool) {
        let data = self.buffer.as_mut();
        let raw = data[field::PUSH_BTN_FLAGS_5];
        let raw = if value { raw | 0x02 } else { raw & !0x02 };
        data[field::PUSH_BTN_FLAGS_5] = raw;
    }

    /// Set the front panel front panel first wheel ticks counter.
    #[inline]
    pub fn set_fp_first_wheel_ticks_counter(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::WHL_1_TICKS] = value;
    }

    /// Set the front panel second wheel ticks counter.
    #[inline]
    pub fn set_fp_second_wheel_ticks_counter(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::WHL_2_TICKS] = value;
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for Frame<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Repr::parse(self) {
            Ok(repr) => write!(f, "{}", repr),
            Err(err) => {
                write!(f, "x122 ({})", err)?;
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

/// A high-level representation of a x122 CAN frame.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Repr {
    pub front_panel_buttons_state: [bool; 44],
    pub front_panel_bp_button_state: bool,
    pub front_panel_esp_button_state: bool,
    pub front_panel_first_wheel_sync_request: bool,
    pub front_panel_second_wheel_sync_request: bool,
    pub front_panel_first_wheel_ticks_counter: u8,
    pub front_panel_second_wheel_ticks_counter: u8,
}

impl Repr {
    pub fn parse<T: AsRef<[u8]> + ?Sized>(frame: &Frame<&T>) -> Result<Repr> {
        frame.check_len()?;

        let mut buttons = [false; 44];

        buttons[0] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_0 }, 7>();
        buttons[1] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_0 }, 6>();
        buttons[2] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_0 }, 5>();
        buttons[3] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_0 }, 4>();
        buttons[4] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_0 }, 3>();
        buttons[5] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_0 }, 2>();
        buttons[6] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_0 }, 1>();
        buttons[7] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_0 }, 0>();
        buttons[8] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_1 }, 7>();
        buttons[9] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_1 }, 6>();
        buttons[10] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_1 }, 5>();
        buttons[11] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_1 }, 4>();
        buttons[12] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_1 }, 3>();
        buttons[13] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_1 }, 2>();
        buttons[14] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_1 }, 1>();
        buttons[15] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_1 }, 0>();
        buttons[16] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_2 }, 7>();
        buttons[17] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_2 }, 6>();
        buttons[18] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_2 }, 5>();
        buttons[19] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_2 }, 4>();
        buttons[20] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_2 }, 3>();
        buttons[21] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_2 }, 2>();
        buttons[22] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_2 }, 1>();
        buttons[23] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_2 }, 0>();
        buttons[24] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_3 }, 7>();
        buttons[25] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_3 }, 6>();
        buttons[26] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_3 }, 5>();
        buttons[27] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_3 }, 4>();
        buttons[28] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_3 }, 3>();
        buttons[29] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_3 }, 2>();
        buttons[30] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_3 }, 1>();
        buttons[31] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_3 }, 0>();
        buttons[32] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_4 }, 7>();
        buttons[33] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_4 }, 6>();
        buttons[34] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_4 }, 5>();
        buttons[35] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_4 }, 4>();
        buttons[36] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_4 }, 3>();
        buttons[37] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_4 }, 2>();
        buttons[38] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_4 }, 1>();
        buttons[39] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_4 }, 0>();
        buttons[40] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_5 }, 7>();
        buttons[41] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_5 }, 6>();
        buttons[42] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_5 }, 5>();
        buttons[43] = frame.read_button_state::<{ field::PUSH_BTN_FLAGS_5 }, 4>();

        Ok(Repr {
            front_panel_buttons_state: buttons,
            front_panel_bp_button_state: frame
                .read_button_state::<{ field::PUSH_BTN_FLAGS_5 }, 2>(),
            front_panel_esp_button_state: frame
                .read_button_state::<{ field::PUSH_BTN_FLAGS_5 }, 3>(),
            front_panel_first_wheel_sync_request: frame.fp_first_wheel_sync_request(),
            front_panel_second_wheel_sync_request: frame.fp_second_wheel_sync_request(),
            front_panel_first_wheel_ticks_counter: frame.fp_first_wheel_ticks_counter(),
            front_panel_second_wheel_ticks_counter: frame.fp_second_wheel_ticks_counter(),
        })
    }

    /// Return the length of a frame that will be emitted from this high-level representation.
    pub fn buffer_len(&self) -> usize {
        FRAME_LEN
    }

    /// Emit a high-level representation into a x122 CAN frame.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, frame: &mut Frame<T>) {
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_0 }, 7>(
            self.front_panel_buttons_state[0],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_0 }, 6>(
            self.front_panel_buttons_state[1],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_0 }, 5>(
            self.front_panel_buttons_state[2],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_0 }, 4>(
            self.front_panel_buttons_state[3],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_0 }, 3>(
            self.front_panel_buttons_state[4],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_0 }, 2>(
            self.front_panel_buttons_state[5],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_0 }, 1>(
            self.front_panel_buttons_state[6],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_0 }, 0>(
            self.front_panel_buttons_state[7],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_1 }, 7>(
            self.front_panel_buttons_state[8],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_1 }, 6>(
            self.front_panel_buttons_state[9],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_1 }, 5>(
            self.front_panel_buttons_state[10],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_1 }, 4>(
            self.front_panel_buttons_state[11],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_1 }, 3>(
            self.front_panel_buttons_state[12],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_1 }, 2>(
            self.front_panel_buttons_state[13],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_1 }, 1>(
            self.front_panel_buttons_state[14],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_1 }, 0>(
            self.front_panel_buttons_state[15],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_2 }, 7>(
            self.front_panel_buttons_state[16],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_2 }, 6>(
            self.front_panel_buttons_state[17],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_2 }, 5>(
            self.front_panel_buttons_state[18],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_2 }, 4>(
            self.front_panel_buttons_state[19],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_2 }, 3>(
            self.front_panel_buttons_state[20],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_2 }, 2>(
            self.front_panel_buttons_state[21],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_2 }, 1>(
            self.front_panel_buttons_state[22],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_2 }, 0>(
            self.front_panel_buttons_state[23],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_3 }, 7>(
            self.front_panel_buttons_state[24],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_3 }, 6>(
            self.front_panel_buttons_state[25],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_3 }, 5>(
            self.front_panel_buttons_state[26],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_3 }, 4>(
            self.front_panel_buttons_state[27],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_3 }, 3>(
            self.front_panel_buttons_state[28],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_3 }, 2>(
            self.front_panel_buttons_state[29],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_3 }, 1>(
            self.front_panel_buttons_state[30],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_3 }, 0>(
            self.front_panel_buttons_state[31],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_4 }, 7>(
            self.front_panel_buttons_state[32],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_4 }, 6>(
            self.front_panel_buttons_state[33],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_4 }, 5>(
            self.front_panel_buttons_state[34],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_4 }, 4>(
            self.front_panel_buttons_state[35],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_4 }, 3>(
            self.front_panel_buttons_state[36],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_4 }, 2>(
            self.front_panel_buttons_state[37],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_4 }, 1>(
            self.front_panel_buttons_state[38],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_4 }, 0>(
            self.front_panel_buttons_state[39],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_5 }, 7>(
            self.front_panel_buttons_state[40],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_5 }, 6>(
            self.front_panel_buttons_state[41],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_5 }, 5>(
            self.front_panel_buttons_state[42],
        );
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_5 }, 4>(
            self.front_panel_buttons_state[43],
        );
        frame
            .write_button_state::<{ field::PUSH_BTN_FLAGS_5 }, 2>(self.front_panel_bp_button_state);
        frame.write_button_state::<{ field::PUSH_BTN_FLAGS_5 }, 3>(
            self.front_panel_esp_button_state,
        );
        frame.set_fp_first_wheel_sync_request(self.front_panel_first_wheel_sync_request);
        frame.set_fp_second_wheel_sync_request(self.front_panel_second_wheel_sync_request);
        frame.set_fp_first_wheel_ticks_counter(self.front_panel_first_wheel_ticks_counter);
        frame.set_fp_second_wheel_ticks_counter(self.front_panel_second_wheel_ticks_counter);
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x122",)?;
        writeln!(f, " front_panel_buttons_state=")?;
        for (btn, val) in self.front_panel_buttons_state.into_iter().enumerate() {
            writeln!(f, "  button_{}={}", btn, val)?;
        }
        writeln!(
            f,
            " front_panel_bp_button_state={}",
            self.front_panel_bp_button_state
        )?;
        writeln!(
            f,
            " front_panel_esp_button_state={}",
            self.front_panel_esp_button_state
        )?;
        writeln!(
            f,
            " front_panel_first_wheel_ticks_counter={}",
            self.front_panel_first_wheel_ticks_counter
        )?;
        writeln!(
            f,
            " front_panel_second_wheel_ticks_counter={}",
            self.front_panel_second_wheel_ticks_counter
        )
    }
}

#[cfg(test)]
mod test {
    use super::{Frame, Repr};

    use crate::Error;

    static REPR_FRAME_BYTES_1: [u8; 8] = [0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x00, 0x00];
    static REPR_FRAME_BYTES_2: [u8; 8] = [0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xff, 0xff];

    fn frame_1_repr() -> Repr {
        let mut buttons = [true; 44];
        for (i, btn) in buttons.iter_mut().enumerate() {
            if (i & 1) == 0 {
                *btn = false;
            }
        }

        Repr {
            front_panel_buttons_state: buttons,
            front_panel_bp_button_state: true,
            front_panel_esp_button_state: false,
            front_panel_first_wheel_sync_request: false,
            front_panel_second_wheel_sync_request: true,
            front_panel_first_wheel_ticks_counter: 0,
            front_panel_second_wheel_ticks_counter: 0,
        }
    }

    fn frame_2_repr() -> Repr {
        let mut buttons = [false; 44];
        for (i, btn) in buttons.iter_mut().enumerate() {
            if (i & 1) == 0 {
                *btn = true;
            }
        }

        Repr {
            front_panel_buttons_state: buttons,
            front_panel_bp_button_state: false,
            front_panel_esp_button_state: true,
            front_panel_first_wheel_sync_request: true,
            front_panel_second_wheel_sync_request: false,
            front_panel_first_wheel_ticks_counter: 255,
            front_panel_second_wheel_ticks_counter: 255,
        }
    }

    #[test]
    fn test_frame_1_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_1);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.fp_second_wheel_sync_request(), true);
        assert_eq!(frame.fp_first_wheel_sync_request(), false);
        assert_eq!(frame.fp_first_wheel_ticks_counter(), 0);
        assert_eq!(frame.fp_second_wheel_ticks_counter(), 0);
    }

    #[test]
    fn test_frame_2_deconstruction() {
        let frame = Frame::new_unchecked(&REPR_FRAME_BYTES_2);
        assert_eq!(frame.check_len(), Ok(()));
        assert_eq!(frame.fp_second_wheel_sync_request(), false);
        assert_eq!(frame.fp_first_wheel_sync_request(), true);
        assert_eq!(frame.fp_first_wheel_ticks_counter(), 255);
        assert_eq!(frame.fp_second_wheel_ticks_counter(), 255);
    }

    #[test]
    fn test_frame_1_construction() {
        let mut bytes = [0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x00, 0x00];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_fp_second_wheel_sync_request(true);
        frame.set_fp_first_wheel_sync_request(false);
        frame.set_fp_first_wheel_ticks_counter(0);
        frame.set_fp_second_wheel_ticks_counter(0);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_1);
    }

    #[test]
    fn test_frame_2_construction() {
        let mut bytes = [0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xff, 0xff];
        let mut frame = Frame::new_unchecked(&mut bytes);

        frame.set_fp_second_wheel_sync_request(false);
        frame.set_fp_first_wheel_sync_request(true);
        frame.set_fp_first_wheel_ticks_counter(255);
        frame.set_fp_second_wheel_ticks_counter(255);

        assert_eq!(frame.into_inner(), &REPR_FRAME_BYTES_2);
    }

    #[test]
    fn test_overlong() {
        let bytes: [u8; 9] = [0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x00, 0x00, 0xff];
        assert_eq!(
            Frame::new_unchecked(&bytes).check_len().unwrap_err(),
            Error::Overlong
        );
    }

    #[test]
    fn test_underlong() {
        let bytes: [u8; 7] = [0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x00];
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
