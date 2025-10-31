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

    di.write_raw(0xCF, &[0x00, 0xc3, 0x30]).unwrap();
    di.write_raw(0xED, &[0x64, 0x03, 0x12, 0x81]).unwrap();
    di.write_raw(0xE8, &[0x85, 0x10, 0x79]).unwrap();
    di.write_raw(0xCB, &[0x39, 0x2C, 0x00, 0x34, 0x02]).unwrap();
    di.write_raw(0xF7, &[0x20]).unwrap();
    di.write_raw(0xEA, &[0x00, 0x00]).unwrap();
    // Power control  VRH[5:0] 
    di.write_raw(0xC0, &[0x22]).unwrap();
    // Power control SAP[2:0];BT[3:0] 
    di.write_raw(0xC1, &[0x11]).unwrap();
    //VCM control
    di.write_raw(0xC5, &[0x3a, 0x1c]).unwrap();
    //VCM control2 
    di.write_raw(0xC7, &[0xa9]).unwrap();
    // Memory Access Control 
    di.write_raw(0x36, &[0x00]).unwrap(); // 0x08
    di.write_raw(0x3A, &[0x55]).unwrap();
    //  //70hz 
    di.write_raw(0xB1, &[0x00, 0x1B]).unwrap();
    // Display Function Control 
    di.write_raw(0xB6, &[0x0A, 0xA2]).unwrap();
    
    di.write_raw(0xF6, &[0x01, 0x1d]).unwrap();
    // 3Gamma Function Disable 
    di.write_raw(0xF2, &[0x00]).unwrap();
    //Gamma curve selected 
    di.write_raw(0x26, &[0x01]).unwrap();
    //Set Gamma 
    di.write_raw(0xE0, &[0x0F,0x3F,0x2F,0x0C,0x10,0x0A,0x53,0xD5,0x40,0x0A,0x13,0x03,0x08,0x03,0x00]).unwrap();
    //Set Gamma 
    di.write_raw(0xE1, &[0x00,0x00,0x10,0x03,0x0F,0x05,0x2C,0xA2,0x3F,0x05,0x0E,0x0C,0x37,0x3C,0x0F]).unwrap();
    
    //Display inversino on  
    di.write_raw(0x21, &[]).unwrap();
    //Exit Sleep 
    di.write_raw(0x11, &[]).unwrap();
    
    delay.delay_ms(120);
    //Display on 
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
