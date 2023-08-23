mod open_lstp;

use open_lstp::qr::save_qr;
use open_lstp::qr::parse_qr;


fn main() {
    let inp_data: &[u8] = b"012adkrdgrjkhdrlhglh3452adkrdg\
                            rjkhdrlhglh3452adkrdgrjkhdrldd\
                            rjkhdrlhglh3452adkrdgrjkhdrldd";
    let inp_vec: Vec<u8> = inp_data.into();
    save_qr(&inp_vec, "/tmp/qrcode.png".to_string());
    let out_vec: Vec<u8> = parse_qr("/tmp/qrcode.png".to_string());
    if inp_vec == out_vec {
        println!("OK")
    } else {
        println!("ERR!")
    }
}
