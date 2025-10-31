use embedded_hal::delay::DelayNs;

use crate::{
    dcs::{
        EnterNormalMode, ExitSleepMode, InterfaceExt, PixelFormat, SetAddressMode, SetDisplayOn,
        SetInvertMode, SetPixelFormat,
    },
    interface::Interface,
    options::ModelOptions,
};

/// Common init for all ILI934x controllers and color formats.
pub fn init_common<DELAY, DI>(
    di: &mut DI,
    delay: &mut DELAY,
    options: &ModelOptions,
    pixel_format: PixelFormat,
) -> Result<SetAddressMode, DI::Error>
where
    DELAY: DelayNs,
    DI: Interface,
{
    let madctl = SetAddressMode::from(options);

    delay.delay_ms(120);

    // Power control B
    // 00000000
    // 100 power control[3:4] 001
    // 001 DC_ena 0000
    di.write_raw(0xCF, &[
        0x00, // Padding
        0b11000011, // C3 // Default=10100010
        // ESD protection enabled
        0b00110000  // 30 // Default=11110000
    ]).unwrap();
    // Power on sequence control
    di.write_raw(0xED, &[
        // Soft start control
        0x64,
        // Power on sequence control
        0x03,
        0x12,
        // DDVDH Enhance Mode
        0x81
    ]).unwrap();
    // Driver timing control A
    // TODO: different in https://github.com/Bodmer/TFT_ILI9341/blob/master/TFT_ILI9341.cpp
    di.write_raw(0xE8, &[
        // Gate driver non-overlapping timing control
        0x85,
        // EQ timing control
        0x10,
        // Pre-charge timing control
        0x79,
    ]).unwrap();
    // Power control A
    di.write_raw(0xCB, &[
        0x39,
        0x2C,
        0x00,
        // vCore Control
        0x34,
        // DDVDH control
        0x02,
    ]).unwrap();
    //// Pump ratio control
    di.write_raw(0xF7, &[
        // Ratio Control
        0x20
    ]).unwrap();
    // Driver timing control B
    di.write_raw(0xEA, &[
        // Gate driver timing control
        0x00,
        0x00
    ]).unwrap();
    // Power control  VRH[5:0] 
    di.write_raw(0xC0, &[
        0x22 // GVDD level, reference for VCOM and grayscale voltage.
    ]).unwrap();
    // Power control SAP[2:0];BT[3:0] 
    di.write_raw(0xC1, &[
        0x11 // Step-up circuit factor
    ]).unwrap();
    //VCM control
    di.write_raw(0xC5, &[
        0x3a, // VCOMH Voltage
        0x1c // VCOML Voltage
        ]).unwrap();
    //VCM control2 
    di.write_raw(0xC7, &[
        0xa9 // nVM and VCOM offset voltage
    ]).unwrap();
    // Memory Access Control 
    di.write_raw(0x36, &[
        // [MY, MX, MV, ML, BGR, MH, 0, 0]
        0b00000000
    ]).unwrap(); // 0x08
    di.write_raw(0x3A, &[
        //  DBI[2:0] = 101 for 6-bit pixel RGB565 color.

        // [0, DPI[2:0], 0, DBI[2:0]]
        0b01010101
    ]).unwrap();
    //  Frame control in normal mode
    di.write_raw(0xB1, &[
        // DIVA[1:0]
        0b00000000,
        // RTAN[4:0]
        0b00011011 // 70hz
    ]).unwrap();
    // Display Function Control 
    // TODO: very different from https://github.com/Bodmer/TFT_ILI9341/blob/master/TFT_ILI9341.cpp
    di.write_raw(0xB6, &[
        // TODO: document..
        0x0A,
        0xA2
    ]).unwrap();
    
    // Interface control/MADCTL
    // Memory overflow, endianess, RGB interface control
    // TODO: is set for 16-bit color?
    di.write_raw(0xF6, &[
        0x01,
        0x1d
    ]).unwrap();
    // 3Gamma Function Disable 
    di.write_raw(0xF2, &[
        0x00
    ]).unwrap();
    //Gamma curve selected 
    di.write_raw(0x26, &[
        0x01, // 0x01
    ]).unwrap();
    // Set Gamma 
    di.write_raw(0xE0, &[0x0F,0x3F,0x2F,0x0C,0x10,0x0A,0x53,0xD5,0x40,0x0A,0x13,0x03,0x08,0x03,0x00]).unwrap();
    //Set Gamma 
    di.write_raw(0xE1, &[0x00,0x00,0x10,0x03,0x0F,0x05,0x2C,0xA2,0x3F,0x05,0x0E,0x0C,0x37,0x3C,0x0F]).unwrap();
    
    //Display inversion on  
    di.write_raw(0x21, &[]).unwrap();

    // Exit Sleep 
    //
    // Delay required after because controller loads manufacture settings
    // and register values after the sleep exit command is called.
    di.write_raw(0x11, &[]).unwrap();
    delay.delay_ms(120);
    // Display on 
    //
    // Exits out of display-off mode and starts reading from frame memory.
    di.write_raw(0x29, &[]).unwrap();
    delay.delay_ms(50);
    
    // 15.4:  It is necessary to wait 5msec after releasing RESX before sending commands.
    // 8.2.2: It will be necessary to wait 5msec before sending new command following software reset.
    // delay.delay_us(5_000);

    // di.write_command(madctl)?;
    // di.write_raw(0xB4, &[0x0])?;
    // di.write_command(SetInvertMode::new(options.invert_colors))?;
    // di.write_command(SetPixelFormat::new(pixel_format))?;

    // di.write_command(EnterNormalMode)?;

    // // 8.2.12: It will be necessary to wait 120msec after sending Sleep In command (when in Sleep Out mode)
    // //          before Sleep Out command can be sent.
    // // The reset might have implicitly called the Sleep In command if the controller is reinitialized.
    // delay.delay_us(120_000);

    // di.write_command(ExitSleepMode)?;

    // // 8.2.12: It takes 120msec to become Sleep Out mode after SLPOUT command issued.
    // // 13.2 Power ON Sequence: Delay should be 60ms + 80ms
    // delay.delay_us(140_000);

    // di.write_command(SetDisplayOn)?;

    Ok(madctl)
}
