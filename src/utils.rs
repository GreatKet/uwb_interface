pub fn get_params_payload() {
    let params = [
        (1u8, 0u8, 1usize), // (App.DeviceType, args.device),
        (1, 0x11, 1),       // (App.DeviceRole, 0 if args.controlee else 1),
        (1, 0x3, 0),        // (App.MultiNodeMode, args.node),
        (1, 0x1, 2),        // (App.RangingRoundUsage, args.round),
        (2, 0x6, 1),        // (App.DeviceMacAddress, args.mac),
        // # Additional config:
        (1, 0x4, 9),              // (App.ChannelNumber, args.channel),
        (1, 0x22, 1),             // (App.ScheduleMode, args.schedule),
        (1, 0x2, 0),              // (App.StsConfig, args.sts),
        (1, 0x12, 3),             // (App.RframeConfig, args.frame),
        (1, 0x2e, 0xb),           // (App.ResultReportConfig, args.report),
        (2, 0x27, 0x708),         // (App.VendorId, args.vendor_id),
        (6, 0x28, 0x60504030201), // (App.StaticStsIv, args.static_sts),
        (1, 0xd, 1),              // (App.AoaResultReq, args.aoa_report),
        (8, 0x2b, 0),             // (App.UwbInitiationTime, args.init_time),
        (1, 0x14, 0xa),           // (App.PreambleCodeIndex, args.preamble_idx),
        (1, 0x15, 0x2),           // (App.SfdId, args.sfd),
        (2, 0x8, 0x960),          // (App.SlotDuration, args.slot_span),
        (4, 0x9, 0xc8),           // (App.RangingInterval, args.ranging_span),
        (1, 0x1b, 0x19),          // (App.SlotsPerRr, args.slots_per_rr),
        (2, 0x32, 0),             // (App.MaxNumberOfMeasurements, args.meas_max),
        (1, 0x2c, 0),             // (App.HoppingMode, args.hopping_mode),
        (1, 0x13, 0),             // (App.RssiReporting, 1 if args.en_rssi else 0),
        (1, 0x2d, 0),             // (App.BlockStrideLength, args.block_stride_length),
    ];
}
