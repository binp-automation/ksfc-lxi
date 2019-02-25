use std::error;
use std::fmt;
use std::collections::HashMap;


static ERROR_LIST: [(i32, &'static str); 225] = [
    // Command Errors
    (-100, "Command error"),
    (-101, "Invalid character"),
    (-102, "Syntax error"),
    (-103, "Invalid separator"),
    (-104, "Data type error"),
    (-108, "Parameter not allowed"),
    (-109, "Missing parameter"),
    (-110, "Command header error"),
    (-111, "Header separator error"),
    (-112, "Program mnemonic too long"),
    (-113, "Undefined header"),
    (-114, "Header suffix out of range"),
    (-120, "Numeric data error"),
    (-121, "Invalid character in number"),
    (-123, "Exponent too large"),
    (-123, "Numeric overflow"),
    (-124, "Too many digits"),
    (-128, "Numeric data not allowed"),
    (-130, "Suffix error"),
    (-131, "Invalid suffix"),
    (-134, "Suffix too long"),
    (-138, "Suffix not allowed"),
    (-140, "Character data error"),
    (-141, "Invalid character data"),
    (-144, "Character data too long"),
    (-148, "Character data not allowed"),
    (-150, "String data error"),
    (-151, "Invalid string data"),
    (-158, "String data not allowed"),
    (-160, "Block data error"),
    (-161, "Invalid block data"),
    (-168, "Block data not allowed"),
    (-170, "Expression error"),
    (-171, "Invalid expression"),
    (-178, "Expression data not allowed"),

    // Execution Errors
    (-203, "Command protected; pulsed measurements require option 150"),
    (-213, "INIT ignored"),
    (-221, "Settings conflict"),
    (-221, "Settings conflict; *TRG when TRIG:SOUR BUS not selected; trigger ignored"),
    (-221, "Settings conflict; CALC:SCAL:REF 0.0 not compatible with CALC:SCAL:FUNC PCT|PPM|PPB; CALC:SCAL:FUNC set to NULL"),
    (-221, "Settings conflict; CALC:STATe or CALC:AVER:STATe OFF set"),
    (-221, "Settings conflict; CALC2:TRAN:HIST:STATe OFF set"),
    (-221, "Settings conflict; GATE:STAR:DEL:SOUR EVENts on channel 3; GATE:STAR:DEL:SOUR IMM set"),
    (-221, "Settings conflict; GATE:STOP:HOLD:SOUR EVENts on channel 3; GATE:STOP:HOLD:SOUR TIM set"),
    (-221, "Settings conflict; ROSC:EXT:CHEC can only be sent when ROSC:SOUR EXT selectd;AUTO OFF set"),
    (-221, "Settings conflict; SAMP:COUN limited to 100,000 in time stamp function with SENS:TST:RATE SLOW; SAMP:COUN set to 100,000"),
    (-221, "Settings conflict; SENS:FREQ:GATE:SOUR on measurement channel; SENS:FREQ:GATE:SOUR set to EXT"),
    (-221, "Settings conflict; SENS:FREQ:MODE CONT only valid for frequency/period; SENS:FREQ:MODE AUTO set"),
    (-221, "Settings conflict; SENS:GATE:EXT:SOUR on measurement channel; SENS:GATE:EXT:SOUR set to EXT"),
    (-221, "Settings conflict; SENS:TINT:GATE:SOUR on measurement channel; SENS:TINT:GATE:SOUR set to EXT"),
    (-221, "Settings conflict; SENS:TOT:GATE:SOUR on measurement channel; SENS:TOT:GATE:SOUR set to EXT"),
    (-221, "Settings conflict; cannot auto-level input channel used as gate; INP:LEV set to 0V, auto-level off"),
    (-221, "Settings conflict; cannot delete state selected and enabled for automatic power-on recall"),
    (-221, "Settings conflict; cannot gate time interval-type measurement with baseband channel; SENS:GATE:EXT:SOUR set to BNC"),
    (-221, "Settings conflict; cannot have immediate, no-holdoff gate stop for frequency or totalize meas- urements; GATE:STOP:HOLD:SOUR TIME set"),
    (-221, "Settings conflict; cannot use READ? with continuous totalize"),
    (-221, "Settings conflict; external gating not compatible with gate output; gate output disabled"),
    (-221, "Settings conflict; histogram bin width is 0.0; CALC2:TRAN:HIST:RANG:AUTO ON set"),
    (-221, "Settings conflict; histogram lower range > upper range; CALC2:TRAN:HIST:RANG:AUTO ON set"),
    (-221, "Settings conflict; infinite stop holdoff time for frequency; SENS:FREQ:GATE:SOUR set to TIME"),
    (-221, "Settings conflict; infinite stop holdoff time for time interval; SENS:TINT:GATE:SOUR set to IMM"),
    (-221, "Settings conflict; input range not compatible with input probe factor; INP:RANG set to 50V range"),
    (-221, "Settings conflict; input threshold voltage > input range; threshold clipped to range"),
    (-221, "Settings conflict; low reference >= high reference"),
    (-221, "Settings conflict; low reference >= high reference; reference values changed to defaults"),
    (-221, "Settings conflict; lower limit > upper limit; CALC:LIM:UPP set to CALC:LIM:LOW value"),
    (-221, "Settings conflict; lower reference and upper reference have different units"),
    (-221, "Settings conflict; stop holdoff < minimum gate time for frequency or totalize; SENSe:GATE:STOP:HOLD:TIME set to minimum"),
    (-221, "Settings conflict; trigger source is BUS"),
    (-222, "Data out of range"),
    (-222, "Data out of range; value clipped to lower limit"),
    (-222, "Data out of range; value clipped to upper limit"),
    (-223, "Too much data"),
    (-224, "Illegal parameter value"),
    (-225, "Out of memory; measurement data overrun"),
    (-230, "Data corrupt or stale"),
    (-240, "Hardware error; CPU board initialization failed"),
    (-240, "Hardware error; GPIB interface failed"),
    (-240, "Hardware error; cannot communicate with channel 3 hardware"),
    (-240, "Hardware error; cannot communicate with measurement hardware"),
    (-240, "Hardware error; channel 3 operation failed"),
    (-240, "Hardware error; measurement hardware initialization failed"),
    (-240, "Hardware error; measurement operation failed"),
    (-240, "Hardware error; failed to program measurement FPGA security EEProm"),
    (-241, "Hardware missing"),
    (-250, "Mass storage error: file read/write error"),
    (-252, "Missing media"),
    (-254, "Media full"),
    (-256, "File or folder name not found"),
    (-257, "File name error; invalid character in name"),
    (-257, "File name error; relative path not allowed"),
    (-257, "File name error; path too long"),
    (-257, "File name error; path is a folder name"),
    (-257, "File name error; not a folder name"),
    (-257, "File name error; drive name missing or not recognized"),
    (-257, "File name error; path name missing"),
    (-257, "File name error; file or folder already exists"),
    (-257, "File name error; folder not empty"),
    (-257, "File name error; folder is default folder"),
    (-257, "File name error; access denied"),
    (-257, "File name error"),
    (-257, "File name error; file too large"),
    (-257, "File name error; unknown file extension"),

    // Device-Specific Errors
    (-310, "System error; internal software error"),
    (-310, "System error; software initialization failed"),
    (-310, "System error; out of memory"),
    (-310, "System error; failed to erase calibration data in PIC EEProm"),
    (-310, "System error; failed to erase system information in PIC EEProm"),
    (-310, "System error; failed to read calibration information from PIC EEProm"),
    (-310, "System error; failed to read system information from PIC EEProm"),
    (-310, "System error; failed to write calibration information to PIC EEProm"),
    (-310, "System error; failed to write system data to PIC EEProm"),
    (-310, "System error; I2C Comms Failure, PIC:Ac Power Detect"),
    (-310, "System error; I2C Comms Failure, PIC:BatteryFuelGauge"),
    (-310, "System error; I2C Comms Failure, PIC:BatteryInfo"),
    (-310, "System error; I2C Comms Failure, PIC:OCXO"),
    (-310, "System error; I2C Comms Failure, PIC:PwrCondition"),
    (-310, "System error; I2C Comms Failure, PIC:PwrOverVolt"),
    (-310, "System error; I2C Comms Failure, PIC:PwrUnderVolt"),
    (-310, "System error; I2C Comms Failure, PIC:SetOCXOStanby"),
    (-310, "System error; I2C Comms Failure, PIC:Temperature"),
    (-310, "System error; I2C Comms Failure, PIC:clearPwrCondition"),
    (-310, "System error; I2C Comms Failure, PIC:cyclePower"),
    (-310, "System error; I2C Comms Failure, PIC:finishPowerdown"),
    (-310, "System error; I2C Comms Failure, PIC:picCommunication"),
    (-310, "System error; I2C Comms Failure, PIC:setBattStorage"),
    (-310, "System error; I2C Comms Failure, PIC:setBatteryPresent"),
    (-310, "System error; PIC EEProm access failed"),
    (-310, "System error; PIC EEProm failed waiting for unbusy"),
    (-311, "Internal software error"),
    (-313, "Calibration memory lost"),
    (-313, "Calibration memory lost; memory corruption detected"),
    (-313, "Calibration memory lost; due to firmware revision change"),
    (-314, "Save/recall memory lost; memory corruption detected"),
    (-314, "Save/recall memory lost; due to firmware revision change"),
    (-315, "Configuration memory lost; memory corruption detected"),
    (-315, "Configuration memory lost; due to firmware revision change"),
    (-330, "Self-test failed"),
    (-350, "Error queue overflow"),

    // Query Errors
    (-410, "Query INTERRUPTED"),
    (-420, "Query UNTERMINATED"),
    (-430, "Query DEADLOCKED"),
    (-440, "Query UNTERMINATED after indefinite response"),

    // Instrument Errors
    (100, "Network Error"),
    (110, "LXI mDNS Error"),
    (201, "Memory lost: stored state"),
    (202, "Memory lost: power-on state"),
    (203, "Memory lost: stored measurements"),
    (263, "Not able to execute while instrument is measuring"),
    (291, "Not able to recall state: it is empty"),
    (292, "State file size error"),
    (293, "State file corrupt"),
    (294, "Preference file size error"),
    (295, "Preference file corrupt"),
    (301, "Input termination protection relay opened"),
    (302, "Cannot reset input protection; high voltage present"),
    (305, "Not able to perform requested operation"),
    (310, "Channel 3 pulse width too short"),
    (311, "Channel 3 pulse width too long"),
    (312, "Channel 3 pulse width could not be measured"),
    (313, "Channel 3 burst frequency could not be measured"),
    (314, "Channel 3 pulse ended before gate closed"),
    (315, "Channel 3 power too high for operation"),
    (316, "Channel 3 power too low for operation"),
    (317, "Channel 3 power changed during measurement"),
    (318, "Channel 3 input is not pulsed signal"),
    (319, "Channel 3 frequency shift detected during measurement"),
    (320, "Input signal frequency shift caused internal counter overflow"),
    (321, "Measurement timeout occurred"),
    (322, "Measurement overflow occurred"),
    (514, "Not allowed"),
    (514, "Not allowed; Instrument locked by another I/O session"),
    (521, "Communications: input buffer overflow"),
    (522, "Communications: output buffer overflow"),
    (532, "Not able to achieve requested resolution"),
    (540, "Cannot use overload as math reference"),
    (541, "Cannot use zero as math reference for PCT, PPM, or PPB scaling functions"),
    (550, "Not able to execute command in local mode"),
    (580, "No valid external timebase"),
    (600, "Internal licensing error"),
    (601, "License file corrupt or empty"),
    (602, "No valid licenses found for this instrument"),
    (603, "Some licenses could not be installed"),
    (604, "License not found"),
    (800, "Nonvolatile memory write failure"),
    (810, "State has not been stored"),
    (820, "Model and Serial Numbers not restored"),
    (821, "Controller and measurement board model numbers do not match"),
    (822, "Controller and measurement board serial numbers do not match"),

    // Self-Test Errors
    (901, "Self Test failed; auto-calibration failure"),
    (902, "Self-test failed; main CPU power supply out of range"),
    (903, "Self-test failed; real time clock settings lost"),
    (904, "Self-test failed; main CPU error accessing boot environment"),
    (905, "Self-test failed; failed to read FPGA revision"),
    (906, "Self-test failed; FPGA revision is less than expected"),
    (907, "Self-test failed; PIC communication failure"),
    (908, "Self-test failed; battery test failed"),
    (909, "Self-test failed; GPIB test failed"),
    (910, "Self-test failed; channel 3 test failed"),
    (911, "Self-test failed; front panel revision check failed"),
    (912, "Self-test failed; measurement board test failed"),

    // Calibration Errors
    (701, "Calibration error; security defeated"),
    (702, "Calibration error; calibration memory is secured"),
    (703, "Calibration error; secure code provided was invalid"),
    (704, "Calibration error: secure code too long"),
    (705, "Calibration error; calibration aborted"),
    (706, "Calibration error: provided value out of range"),
    (707, "Calibration error: computed correction factor out of range"),
    (708, "Calibration error: signal measurement out of range"),
    (709, "Calibration error: no calibration for this function"),
    (710, "Calibration error: full scale correction out of range"),
    (711, "Calibration error: calibration string too long"),
    (712, "Calibration failed"),
    (713, "Channel 3 calibration signal not detected"),
    (714, "Channel 3 calibration signal power level error"),
    (715, "Channel 3 calibration signal frequency error"),
    (716, "Channel 3 calibration signal is not CW"),
    (717, "Channel 3 calibration timeout occurred"),
    (720, "Auto-calibration failed; input signal detected"),
    (740, "Calibration data lost: secure state"),
    (740, "Calibration information lost: count, security state, security code, string"),
    (741, "Calibration data lost: string data"),
    (742, "Calibration data lost: corrections"),
    (748, "Calibration memory write failure"),
    (750, "Calibration data not restored"),
];

lazy_static! {
    static ref ERROR_MAP: HashMap<i32, &'static str> = {
        let mut m = HashMap::with_capacity(ERROR_LIST.len());
        for (code, text) in ERROR_LIST.iter() {
            m.insert(*code, *text);
        }
        m.shrink_to_fit();
        m
    };
}

#[derive(Clone, Eq, PartialEq)]
pub struct Error {
	code: i32,
}

impl Error {
	pub fn new(code: i32) -> Option<Self> {
        if ERROR_MAP.contains_key(&code) {
            Some(Error { code })
        } else {
            None
        }
	}

    pub fn code(&self) -> i32 {
        self.code
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: '{}'", self.code, ERROR_MAP.get(&self.code).unwrap())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ERROR_MAP.get(&self.code).unwrap())
    }
}

impl error::Error for Error {}
