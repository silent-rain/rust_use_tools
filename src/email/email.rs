use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::{EmailBuilder, Mailbox};



fn main2() {
    let email = EmailBuilder::new()
        .from(Mailbox::new("发送者的邮箱地址".to_string()))
        //.from(Mailbox::new("xiaoming@163.com".to_string())) //发送者：xiaoming@163.com
        .to(Mailbox::new("接收者邮箱地址".to_string()))
        //.to(Mailbox::new("xiaohong@126.com".to_string())) //接收者：xiaohong@126.com
        .subject("Test") //邮件标题
        .body("This is a test email!") //邮件内容
        .build()
        .unwrap();

    //for example: xiaoming@163.com, password: 123456
    //let creds = Credentials::new("xiaoming".to_string(), "123456".to_string());
    let creds = Credentials::new("你的邮箱用户名".to_string(), "你的邮箱密码".to_string());

    //如163的邮箱就是smtp.163.com, 126的邮箱就是smtp.126.com
    let mut mailer = SmtpClient::new_simple("邮箱服务器地址")
        .unwrap()
        .credentials(creds)
        .transport();

    let result = mailer.send(email.into());

    if result.is_ok() {
        println!("Email sent");
    } else {
        println!("Could not send email: {:?}", result);
    }

    assert!(result.is_ok());
}

// username: str, password: str, host="smtp.163.com", port=465,
// 163: smtp.163.com   465/994(ssl)   25
// qq: smtp.qq.com     465(ssl)
// 端口： 25 不安全，推荐使用安全协议的邮件

// :param username: 账号
// :param password: 密码,需要使用授权码
// :param host: 邮箱服务器地址
// :param port: 邮箱服务器端口
// :param email_use_ssl:  使用安全连接

// receivers
