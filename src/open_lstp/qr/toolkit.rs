use qrcode::QrCode;
use quircs::Quirc;
use image::Luma;

pub fn save_qr(data: &Vec<u8>, output_file: String) {
    let code = QrCode::new(data).unwrap();
    let img = code.render::<Luma<u8>>().build();
    img.save(output_file).unwrap();
}

pub fn parse_qr(input_file: String) -> Vec<u8> {
    let img = image::open(input_file).expect("failed to open image");
    let img_gray = img.into_luma8();
    let mut decoder = Quirc::default();
    let mut codes = decoder.identify(
        img_gray.width() as usize,
        img_gray.height() as usize, &img_gray
    );
    let code = codes.next().unwrap();
    let code = code.expect("failed to extract qr code");
    let decoded = code.decode().expect("failed to decode qr code");
    return decoded.payload;
}
