mod enums;

use enums::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("Hello, world!");
    let path = "/dev/ttyACM0";
    let port = tokio_serial::new(path, 115200).open_native_async().unwrap();

    let cancel = CancellationToken::new();
    let read_cancel = cancel.child_token();
    let write_cancel = cancel.child_token();

    let (rx, tx) = tokio::io::split(port);
    let (resp_tx, resp_rx) = mpsc::channel::<Response>(5);

    let mut reader = tokio::spawn(read_loop(rx, resp_tx, read_cancel));
    let mut writer = tokio::spawn(write_loop(tx, resp_rx, write_cancel));

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            println!("Ctrl+C received, shutting down.");
            cancel.cancel();

        }
        _ = &mut reader => { println!("reader task ended"); }
        _ = &mut writer => { println!("writer task ended"); }
    }
    Ok(())
    // send_msg(Mt::Command, Gid::Ranging(OidRanging::Start), &p, 300);
}
async fn read_loop(
    mut rx: ReadHalf<SerialStream>,
    mut resp_tx: Sender<Response>,
    cancel: CancellationToken,
) -> Result<(), ()> {
    loop {
        let mut header_buf = [0u8; 4];
        tokio::select! {
        res = rx.read_exact(&mut header_buf) => {
            println!("Rx: {:?}", header_buf);
            let mt = (header_buf[0] &0xe0) >> 5;
            let pl: usize = {
                if mt == Mt::DataPacket as u8 {usize::from_le_bytes(header_buf[2..4].try_into().unwrap())} else {header_buf[3] as usize}
            };
            let mut payload = vec![0u8; pl];
            rx.read_exact(&mut payload).await.unwrap();
            parse_msg(header_buf,&mut resp_tx, payload).await;
        }
        _ = cancel.cancelled() => break}
    }
    Ok(())
}
async fn write_loop(
    tx: WriteHalf<SerialStream>,
    mut resp_rx: Receiver<Response>,
    cancel: CancellationToken,
) -> Result<(), ()> {
    println!("Initializing session");
    let sid: u32 = 42;
    let mut payload = Vec::with_capacity(5);
    payload.extend_from_slice(&sid.to_le_bytes());
    payload.push(SessionType::Ranging as u8);
    println!("{:?}", payload);
    send_msg(tx, Mt::Command, Gid::Session(OidSession::Init), &payload, 5).await;
    let res = resp_rx.recv().await.unwrap();
    if let Response::SessionInit([status, handle]) = res {
        println!("Status: {}, session handle: {}", status, handle);
    } else {
        panic!("Not matching response")
    }
    cancel.cancelled().await;
    Ok(())
}
async fn parse_msg(header: [u8; 4], resp_tx: &mut Sender<Response>, payload: Vec<u8>) {
    let mt = Mt::try_from((header[0] & 0xe0) >> 5).unwrap();
    let gid = header[0] & 0xf;
    let oid = header[1];
    let gid_oid = match gid {
        0 => Gid::Core(OidCore::try_from(oid).unwrap()),
        1 => Gid::Session(OidSession::try_from(oid).unwrap()),
        2 => Gid::Ranging(OidRanging::try_from(oid).unwrap()),
        _ => Gid::Unknown,
    };
    println!("MT: {:?}, OID: {:?}, payload: {:?}", mt, gid_oid, payload);
    if mt == Mt::Response {
        match (gid, oid) {
            (1, 0) => {
                let handle = u32::from_le_bytes(payload[1..5].try_into().unwrap()) as u8;
                resp_tx
                    .send(Response::SessionInit([payload[0], handle]))
                    .await
                    .unwrap();
            }
            _ => {
                println!("Not covered type of response")
            }
        }
    }
}
fn construct_msg(mt: Mt, gid: Gid, pbf: bool, payload: &[u8]) -> Result<Vec<u8>, &'static str> {
    let mut gid_id: u8 = 0xf;
    if gid_id > 15 {
        return Err("GID is too big");
    }
    let mut oid_id: u8 = 0xf;
    match gid {
        Gid::Core(oid) => {
            gid_id = 0;
            oid_id = oid as u8;
        }
        Gid::Session(oid) => {
            gid_id = 1;
            oid_id = oid as u8;
        }
        Gid::Ranging(oid) => {
            gid_id = 2;
            oid_id = oid as u8;
        }
        Gid::Test(oid) => {
            gid_id = 0xd;
            oid_id = oid as u8;
        }
        Gid::Unknown => {
            gid_id = 0xf;
            oid_id = 0xf
        }
    }
    let mt_id = mt as u8;
    if mt_id > 7 {
        return Err("MT is too big");
    }
    let header = [
        (mt_id << 5) | ((pbf as u8) << 4) | (gid_id & 0xf),
        oid_id,
        0u8,
        payload.len() as u8,
    ];
    let mut msg = Vec::from(header);
    msg.extend_from_slice(payload);
    Ok(msg)
}

async fn send_msg(
    mut tx: WriteHalf<SerialStream>,
    mt: Mt,
    gid: Gid,
    payload: &[u8],
    length: usize,
) {
    let mut n = 0;
    while n != length {
        let mut pbf = false;
        let mut size = length - n;
        if size > 250 {
            size = 250;
            pbf = true;
        }
        let msg = construct_msg(mt, gid, pbf, &payload[n..(n + size)]).unwrap();
        tx.write_all(&msg).await.unwrap();
        println!("{:?}", msg);
        n += size;
    }
}
