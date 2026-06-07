use std::fs::{self};
use std::path::Path;

fn main() -> std::io::Result<()> {
    let target_dir = "./"; 
    println!("🚀 [AXIOM ORGANON] Iniciando purga y ordenación de archivos...");

    for entry in fs::read_dir(target_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
                let folder_name = match extension.to_lowercase().as_str() {
                    "rs" | "py" | "js" | "cpp" | "json" | "html" | "css" => "Projects_Source",
                    "jpg" | "jpeg" | "png" | "gif" | "mp4" | "mov" | "mp3" => "Media_Files",
                    "pdf" | "docx" | "txt" | "xlsx" | "pptx" => "Documents",
                    "zip" | "tar" | "gz" | "rar" | "7z" => "Archives",
                    "exe" | "msi" | "deb" | "dmg" => "Installers",
                    _ => "Other_Garbage",
                };

                let dest_dir = Path::new(target_dir).join(folder_name);
                fs::create_dir_all(&dest_dir)?;
                
                let dest_path = dest_dir.join(path.file_name().unwrap());
                fs::rename(&path, &dest_path)?;
                println!("📦 Organizado: {:?} -> {:?}", path.file_name().unwrap(), folder_name);
            }
        }
    }
    println!("✨ ¡Entorno limpio, general! Organización completada.");
    Ok(())
}
