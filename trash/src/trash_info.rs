use chrono::{DateTime, Utc};
use humansize::{file_size_opts as options, FileSize};
use std::path::{Path, PathBuf};

pub struct TrashInfo {
    header: String,
    trash_file_name: PathBuf,
    source_path: PathBuf,
    deletion_date: DateTime<Utc>,
    file_size: String,
}

impl TrashInfo {
    pub fn new(source_path: PathBuf, trash_file_name: PathBuf) -> Self {
        let file_size = std::fs::metadata(&source_path).expect("TODO").len();
        let file_size = file_size.file_size(options::CONVENTIONAL).unwrap();

        TrashInfo {
            header: Self::format_header(),
            trash_file_name,
            source_path,
            deletion_date: std::time::SystemTime::now().into(),
            file_size,
        }
    }

    fn format_header() -> String {
        "[Trash Info]".to_string()
    }

    fn format_file_name(&self) -> String {
        format!("FileName={}", self.trash_file_name.display())
    }

    pub fn format_source_path(source_path: &Path) -> String {
        format!("Path={}", source_path.display())
    }

    fn format_deletion_date(&self) -> String {
        format!("DeletionDate={}", self.deletion_date.to_rfc3339())
    }

    fn format_file_size(&self) -> String {
        format!("FileSize={}", self.file_size)
    }

    pub fn content(&self) -> String {
        format!(
            "{}\n{}\n{}\n{}\n{}",
            self.header,
            self.format_file_name(),
            Self::format_source_path(&self.source_path),
            self.format_deletion_date(),
            self.format_file_size()
        )
    }

    pub fn read_to_std(trash_info_path: &Path) {
        if trash_info_path.is_file() {
            let file = std::fs::read_to_string(&trash_info_path).ok().unwrap();
            for path_str in file.lines() {
                println!("{}", path_str);
            }
        }
    }
}
