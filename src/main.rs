use qrcode::QrCode;

fn main() {
    let code = QrCode::new(
        b"012adkrdgrjkhdrlhglh3452adkrdg\
          rjkhdrlhglh3452adkrdgrjkhdrldd\
          rjkhdrlhglh3452adkrdgrjkhdrldd"
    ).unwrap();

    let string = code.render()
        .light_color('#')
        .dark_color(' ')
        .build();
    println!("{}", string);
}
