/*!邮件服务
 */
use lettre::smtp::authentication::Credentials;
use lettre::smtp::error::Error as SmtpError;
use lettre::{SmtpClient, SmtpTransport, Transport};
use lettre_email::{error::Error as LettreError, Email, EmailBuilder, Mailbox};
use std::error;

// 邮件对象 结构
#[allow(dead_code)]
#[derive(Debug)]
pub struct EmailImpl {
    host: String,           // 邮箱服务器地址
    username: String,       // 邮箱账号
    password: String,       // 密码,需要使用授权码
    receivers: Vec<String>, // 账号
}

impl EmailImpl {
    // 生成邮件对象
    pub fn new(host: String, username: String, password: String, receivers: Vec<String>) -> Self {
        EmailImpl {
            host,
            username,
            password,
            receivers,
        }
    }

    // 设置凭证
    #[allow(dead_code)]
    fn set_creds(&self) -> Credentials {
        return Credentials::new(self.username.clone(), self.password.clone());
    }

    // 连接邮件服务器
    // 163的邮箱: smtp.163.com
    // 126的邮箱: smtp.126.com
    #[allow(dead_code)]
    fn connect(&self, creds: Credentials) -> Result<SmtpTransport, SmtpError> {
        let mailer = SmtpClient::new_simple(&self.host)?
            .credentials(creds)
            // .hello_name(ClientId::Domain("localhost".to_string()))
            .smtp_utf8(true)
            // .authentication_mechanism(Mechanism::Plain)
            // .connection_reuse(ConnectionReuseParameters::ReuseUnlimited).build()
            .transport();
        Ok(mailer)
    }

    // 编译HTML邮件内容
    #[allow(dead_code)]
    fn build_html(&self, subject: &str, body: &str) -> Result<Email, LettreError> {
        let mut builder = EmailBuilder::new();
        // 多个接收者邮箱
        for to in self.receivers.iter() {
            builder = builder.clone().to(Mailbox::new(to.to_string()));
        }

        let email = builder
            .from(self.username.clone())
            // .to(Mailbox::new("xxx@163.com".to_string())) // 单个接收者邮箱
            .subject(subject) //邮件标题
            .html(body)
            // .text(content.text) // 文本内容
            // .body(content.body) //邮件内容
            // .attachment(Path::new("Cargo.toml"), None, &mime::TEXT_PLAIN).unwrap() // 附件
            .build()?;
        Ok(email)
    }

    pub fn send(&self, subject: &str, body: &str) -> Result<(), Box<dyn error::Error>> {
        let creds = self.set_creds();
        let mut mailer = self.connect(creds)?;
        let email_content = self.build_html(subject, body)?;
        let rsp = mailer.send(email_content.into());

        if let Err(err) = rsp {
            return Err(Box::new(err));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_send_email() {
        let obj = EmailImpl::new(
            "smtp.163.com".to_string(),
            "xxx@163.com".to_string(),
            "xxx".to_string(),
            vec!["xxx@qq.com".to_string(), "xxx@163.com".to_string()],
        );
        let rsp = obj.send("this is test title", "<h1>this is test html</h1>");
        if let Err(err) = &rsp {
            assert!(rsp.is_err(), "{}", err)
        }
        assert!(rsp.is_err())
    }
}
