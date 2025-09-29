pub fn get_params_payload() {
    let params = [
        (1u8, 0u8, 1u8), // (App.DeviceType, args.device),
        (1, 0x11, 1),    // (App.DeviceRole, 0 if args.controlee else 1),
        (1, 0x3, 0),     // (App.MultiNodeMode, args.node),
        (1, 0x1, 2),     // (App.RangingRoundUsage, args.round),
                         //             (App.DeviceMacAddress, args.mac),
                         //             # Additional config:
                         //             (App.ChannelNumber, args.channel),
                         //             (App.ScheduleMode, args.schedule),
                         //             (App.StsConfig, args.sts),
                         //             (App.RframeConfig, args.frame),
                         //             (App.ResultReportConfig, args.report),
                         //             (App.VendorId, args.vendor_id),
                         //             (App.StaticStsIv, args.static_sts),
                         //             (App.AoaResultReq, args.aoa_report),
                         //             (App.UwbInitiationTime, args.init_time),
                         //             (App.PreambleCodeIndex, args.preamble_idx),
                         //             (App.SfdId, args.sfd),
                         //             (App.SlotDuration, args.slot_span),
                         //             (App.RangingInterval, args.ranging_span),
                         //             (App.SlotsPerRr, args.slots_per_rr),
                         //             (App.MaxNumberOfMeasurements, args.meas_max),
                         //             (App.HoppingMode, args.hopping_mode),
                         //             (App.RssiReporting, 1 if args.en_rssi else 0),
                         //             (App.BlockStrideLength, args.block_stride_length),
    ];
}
