pub fn get_params_payload(handle: u8, dst_mac: u64, is_controller: bool) -> Vec<u8> {
    let params = [
        (1u8, 0u8, if is_controller { 1 } else { 0 }), // (App.DeviceType, args.device),
        (1, 0x11, if is_controller { 1 } else { 0 }), // (App.DeviceRole, 0 if args.controlee else 1),
        (1, 0x3, 0),                                  // (App.MultiNodeMode, args.node),
        (1, 0x1, 2),                                  // (App.RangingRoundUsage, args.round),
        (2, 0x6, if is_controller { 0 } else { dst_mac }), // (App.DeviceMacAddress, args.mac),
        // # Additional config:
        (1, 0x4, 9),                                 // (App.ChannelNumber, args.channel),
        (1, 0x22, 1),                                // (App.ScheduleMode, args.schedule),
        (1, 0x2, 0),                                 // (App.StsConfig, args.sts),
        (1, 0x12, 3),                                // (App.RframeConfig, args.frame),
        (1, 0x2e, 0xb),                              // (App.ResultReportConfig, args.report),
        (2, 0x27, 0x708),                            // (App.VendorId, args.vendor_id),
        (6, 0x28, 0x60504030201),                    // (App.StaticStsIv, args.static_sts),
        (1, 0xd, 1),                                 // (App.AoaResultReq, args.aoa_report),
        (8, 0x2b, 0),                                // (App.UwbInitiationTime, args.init_time),
        (1, 0x14, 0xa),                              // (App.PreambleCodeIndex, args.preamble_idx),
        (1, 0x15, 0x2),                              // (App.SfdId, args.sfd),
        (2, 0x8, 0x960),                             // (App.SlotDuration, args.slot_span),
        (4, 0x9, 0xc8),                              // (App.RangingInterval, args.ranging_span),
        (1, 0x1b, 0x19),                             // (App.SlotsPerRr, args.slots_per_rr),
        (2, 0x32, 0), // (App.MaxNumberOfMeasurements, args.meas_max),
        (1, 0x2c, 0), // (App.HoppingMode, args.hopping_mode),
        (1, 0x13, 0), // (App.RssiReporting, 1 if args.en_rssi else 0),
        (1, 0x2d, 0), // (App.BlockStrideLength, args.block_stride_length),
        (1, 0x5, if is_controller { 1 } else { 0 }), // NumberOfControlees
        (2, 0x7, dst_mac), // DstMacAddress
        (1, 0x24, 0), // KeyRotationRate
        (1, 0x35, 1), // StsLength
    ];
    let mut payload = Vec::<u8>::new();
    let sid: u32 = handle as u32;
    payload.extend_from_slice(&sid.to_le_bytes());
    payload.push(params.len() as u8);
    for set in params {
        payload.push(set.1);
        payload.push(set.0);
        let mut v = set.2;
        for _ in 0..set.0 {
            payload.push((v & 0xff) as u8);
            v >>= 8;
        }
    }
    // println!("{:?}", payload);
    payload
}
#[derive(Debug)]
pub struct Ranging {
    pub n_meas: u8,
    pub mac_add: u16,
    pub status: u8,
    pub nlos: bool,
    pub distance_cm: f32,        // u16 LE, in cm
    pub aoa_theta_deg: f32,      // Q8.7 -> i16 / 128.0
    pub aoa_theta_fom: u8,       // %
    pub aoa_phi_deg: f32,        // Q8.7
    pub aoa_phi_fom: u8,         // %
    pub aoa_dest_theta_deg: f32, // Q8.7
    pub aoa_dest_theta_fom: u8,  // %
    pub aoa_dest_phi_deg: f32,   // Q8.7
    pub aoa_dest_phi_fom: u8,    // %
    pub slot_in_error: u8,
    pub rssi_dbm: f32, // Q7.1 (byte / 2, negated)
}
pub fn parse_ranging(full_payload: &[u8]) -> Ranging {
    let i = 25;
    let payload = &full_payload[i..i + 31];
    let u16le = |lo: usize| -> u16 { u16::from_le_bytes([payload[lo], payload[lo + 1]]) };
    let i16le = |lo: usize| -> i16 { i16::from_le_bytes([payload[lo], payload[lo + 1]]) };
    Ranging {
        n_meas: full_payload[0],
        mac_add: u16::from_le_bytes(payload[0..2].try_into().unwrap()),
        status: payload[2],
        nlos: payload[3] != 0,
        distance_cm: u16le(4) as f32,
        aoa_theta_deg: i16le(6) as f32 / 128.0,
        aoa_theta_fom: payload[8],
        aoa_phi_deg: i16le(9) as f32 / 128.0,
        aoa_phi_fom: payload[11],
        aoa_dest_theta_deg: i16le(12) as f32 / 128.0,
        aoa_dest_theta_fom: payload[14],
        aoa_dest_phi_deg: i16le(15) as f32 / 128.0,
        aoa_dest_phi_fom: payload[17],
        slot_in_error: payload[18],
        rssi_dbm: -(payload[19] as f32 / 2.0),
    }
}
