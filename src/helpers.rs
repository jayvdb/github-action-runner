use std::{env, fs};
use std::fs::File;
use std::io::{BufReader, Cursor, Read};
use zip::ZipArchive;
use std::sync::Arc;
use std::time::Duration;
use indicatif::ProgressBar;
use rodio::{Decoder, OutputStream, Sink};
use tokio::sync::Mutex;

pub fn unzip_and_concatenate(data_bytes: Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
    let cursor = Cursor::new(data_bytes);
    let mut archive = ZipArchive::new(cursor)?;

    let mut result = String::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.name().to_string();

        // Пропустить файлы в поддиректориях, пока не обработаем все файлы в корне
        if file_name.contains("/") {
            continue;
        }

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        result.push_str(&contents);
    }

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.name().to_string();

        // Теперь обрабатываем только файлы в поддиректориях
        if !file_name.contains("/") {
            continue;
        }

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        result.push_str("\n");
        result.push_str("--------------\n");
        result.push_str(&file_name);
        result.push_str("\n--------------\n");
        result.push_str(&contents);
    }

    Ok(result)
}

pub(crate) async fn update_progress_bar(pb: Arc<Mutex<ProgressBar>>) {
    loop {
        {
            let pb = pb.lock().await;
            pb.tick();
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

pub(crate) fn install_zsh_autocompletion() -> Result<(), Box<dyn std::error::Error>> {
    // Define the source and destination paths
    let current_dir = env::current_dir()?;
    let source_path = current_dir.join("completions/zsh");
    let dest_dir = dirs::home_dir().ok_or("Could not get home directory")?.join(".oh-my-zsh/plugins/gar/");

    // Ensure the destination directory exists
    fs::create_dir_all(&dest_dir)?;

    let dest_path = dest_dir.join("_gar");

    // Copy the file
    fs::copy(source_path, &dest_path)?;

    println!("Zsh autocompletion installed at {:?}", dest_path);

    let zsh_config = dirs::home_dir().ok_or("Could not get home directory")?.join(".zsh");

    println!("you need add plugin `gar` to your zsh config {:?}", zsh_config);

    Ok(())
}

pub(crate) fn beep(count: u8) {
    // Получаем устройство для воспроизведения
    let (_stream, handle) = OutputStream::try_default().unwrap();

    // Воспроизводим звук три раза
    for _ in 0..count {
        let file = File::open("beep.mp3").unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        let sink = Sink::try_new(&handle).unwrap();

        // Добавляем звук в Sink для воспроизведения
        sink.append(source);

        // Используем sleep, чтобы дать время звуку для воспроизведения
        // Предположим, что длительность beep.mp3 - 1 секунда
        std::thread::sleep(Duration::from_millis(300));
    }
}