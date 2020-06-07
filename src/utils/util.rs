use rand::Rng;
use slog::{Drain, o, info, Logger};
const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789";
const PASSWORD_LEN: usize = 8;
    
pub fn generate_randop_otp(logger: &Logger) -> String {
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    // println!("{:?}", password);
    info!(logger, "{:?}", password);
    password
}