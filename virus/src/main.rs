use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};

fn main() {
    println!("Hello, world!");
}
fn send(data: String){
    let file = File::open("pass.txt").expect("file not found!");
    let reader = BufReader::new(file);
    let lines = reader.lines();
    
    let (user, password, email, provider, port) = lines;
    let smtp_credentials = Credentials::new(user.to_string(), password.to_string());

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(provider)?
        .credentials(smtp_credentials)
        .build();

    let from = user+ " <"+email+">";
    let to = user+ " <"+email+">";
    let subject = "Hello World";
    let body = data.to_string();

    send_email_smtp(&mailer, from, to, subject, body)
    
}

fn send_email_smtp(
    mailer: &AsyncSmtpTransport<Tokio1Executor>,
    from: &str,
    to: &str,
    subject: &str,
    body: String,
){
    let email = Message::builder()
        .from(from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body.to_string())?;

    mailer.send(email);

}
