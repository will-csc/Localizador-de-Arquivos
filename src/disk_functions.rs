use sysinfo::Disks;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde_json;
use slint::SharedString;
use lazy_static::lazy_static;
use std::sync::Mutex;

const CACHE_FILE: &str = "cache.txt";

lazy_static! {
    static ref DRIVE_INDEX: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
}

/// Obtém a lista de partições do sistema.
pub fn get_disks() -> Vec<String> {
    let mut disks = Disks::new();
    disks.refresh_list();

    disks
        .list()
        .iter()
        .map(|disk| disk.mount_point().to_string_lossy().to_string())
        .collect()
}

/// Função para salvar os resultados da busca em cache
pub fn save_cache(file_name: &str, results: &[String]) {
    let cache: HashMap<String, Vec<String>> = [(file_name.to_string(), results.to_vec())].iter().cloned().collect();
    
    // Converte os resultados em uma string e salva no cache
    let content = serde_json::to_string(&cache).expect("Falha ao serializar os dados de cache");
    fs::write(CACHE_FILE, content).expect("Falha ao salvar o cache");
}

/// Função para carregar os resultados da busca do cache
pub fn load_cache(file_name: &str) -> Option<Vec<String>> {
    if Path::new(CACHE_FILE).exists() {
        if let Ok(content) = fs::read_to_string(CACHE_FILE) {
            if content.trim().is_empty() {
                return None;
            }

            if let Ok(cache) = serde_json::from_str::<HashMap<String, Vec<String>>>(&content) {
                return cache.get(file_name).cloned();
            }
        }
    }
    None
}
/// Função para indexar o drive inteiro e armazenar os caminhos em um HashMap
pub fn index_drive(drive: &str) {
    let mut index = DRIVE_INDEX.lock().unwrap();
    let path = Path::new(drive);

    let mut file_list = Vec::new();
    let mut folder_list = Vec::new();

    if path.exists() {
        index_directory_recursive(path, &mut file_list, &mut folder_list);
    }

    index.insert("files".to_string(), file_list);
    index.insert("folders".to_string(), folder_list);
}

/// Função recursiva para indexar todos os arquivos e diretórios
fn index_directory_recursive(path: &Path, files: &mut Vec<String>, folders: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            let entry_str = entry_path.to_string_lossy().to_string();

            if entry_path.is_file() {
                files.push(entry_str);
            } else if entry_path.is_dir() {
                folders.push(entry_str);
                index_directory_recursive(&entry_path, files, folders);
            }
        }
    }
}

/// Busca dentro do HashMap indexado
pub fn find_in(file_name: &str, option_folder: bool, option_file: bool) -> SharedString {
    {
        let index = DRIVE_INDEX.lock().expect("Falha ao acessar o índice do drive");
        if index.is_empty() {
            index_drive("D:\\"); // Indexa automaticamente o drive D:
        }
    }

    let index = DRIVE_INDEX.lock().expect("Falha ao acessar o índice do drive");
    let mut results = Vec::new();

    if option_file {
        if let Some(files) = index.get("files") {
            results.extend(files.iter().filter(|f| f.contains(file_name)).cloned());
        }
    }

    if option_folder {
        if let Some(folders) = index.get("folders") {
            results.extend(folders.iter().filter(|f| f.contains(file_name)).cloned());
        }
    }

    SharedString::from(results.join("\n"))
}
