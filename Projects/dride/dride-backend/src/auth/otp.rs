use rand::Rng;

pub fn generate_otp() -> String {
    let mut rng = rand::thread_rng();
    let code: u32 = rng.gen_range(100_000..1_000_000);
    code.to_string()
}

pub async fn send_otp_sms(
    _phone: &str,
    _code: &str,
    _twilio_sid: &str,
    _twilio_token: &str,
    _twilio_from: &str,
) -> Result<(), anyhow::Error> {
    // TODO: integrate Twilio SMS API
    // For dev/MVP we log the OTP instead of sending SMS
    tracing::info!("OTP code generated (dev mode — SMS not sent)");
    Ok(())
}
