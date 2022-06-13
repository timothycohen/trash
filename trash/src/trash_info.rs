use ansi_term::Colour;
use chrono::{DateTime, Utc};
use humansize::{file_size_opts as options, FileSize};
use std::path::{Path, PathBuf};

pub struct TrashInfo {
    pub trash_file_name: PathBuf,
    pub source_path: PathBuf,
    pub is_dir: bool,
    pub deletion_date: DateTime<Utc>,
    pub file_size: String,
}

impl TrashInfo {
    const HEADER: &'static str = "[Trash Info]";
    const FILENAME: &'static str = "FileName=";
    const PATH: &'static str = "Path=";
    const ISDIR: &'static str = "IsDir=";
    const DELETIONDATE: &'static str = "DeletionDate=";
    const FILESIZE: &'static str = "FileSize=";

    pub fn new(source_path: PathBuf, trash_file_name: PathBuf) -> Self {
        let file_size = std::fs::metadata(&source_path).expect("TODO").len();
        let file_size = file_size.file_size(options::CONVENTIONAL).unwrap();
        let is_dir = source_path.is_dir();

        TrashInfo {
            trash_file_name,
            source_path,
            is_dir,
            deletion_date: std::time::SystemTime::now().into(),
            file_size,
        }
    }

    pub fn content(&self) -> String {
        self.color_content(false)
    }

    pub fn format_source_path(source_path: &Path) -> String {
        format!("{}{}", TrashInfo::PATH, source_path.display())
    }
    fn color_format_source_path(source_path: &Path) -> String {
        format!("{}{}", Colour::Fixed(244).paint(TrashInfo::PATH), source_path.display())
    }

    fn color_content(&self, color: bool) -> String {
        match color {
            true => {
                format!(
                    "{}\n{}{}\n{}\n{}{}\n{}{}\n{}{}",
                    Colour::Yellow.paint(TrashInfo::HEADER),
                    Colour::Fixed(244).paint(TrashInfo::FILENAME),
                    self.trash_file_name.display(),
                    Self::color_format_source_path(&self.source_path),
                    Colour::Fixed(244).paint(TrashInfo::ISDIR),
                    self.is_dir,
                    Colour::Fixed(244).paint(TrashInfo::DELETIONDATE),
                    self.deletion_date.to_rfc3339(),
                    Colour::Fixed(244).paint(TrashInfo::FILESIZE),
                    self.file_size,
                )
            }
            false => {
                format!(
                    "{}\n{}{}\n{}\n{}{}\n{}{}\n{}{}",
                    TrashInfo::HEADER,
                    TrashInfo::FILENAME,
                    self.trash_file_name.display(),
                    Self::format_source_path(&self.source_path),
                    TrashInfo::ISDIR,
                    self.is_dir,
                    TrashInfo::DELETIONDATE,
                    self.deletion_date.to_rfc3339(),
                    TrashInfo::FILESIZE,
                    self.file_size,
                )
            }
        }
    }

    pub fn read_to_std(trash_info_path: &Path) {
        if trash_info_path.is_file() {
            let trash_info = TrashInfo::from_file(trash_info_path);
            if let Err(e) = trash_info {
                eprintln!("{}", e);
                return;
            }
            println!("\n{}", trash_info.unwrap().color_content(true));
        }
    }

    pub fn from_file(source_path: &Path) -> Result<TrashInfo, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let file = std::fs::read_to_string(&source_path)?;
        let mut lines = file.lines();
        if lines.next().is_none() {
            Err("missing header")?
        }
        let trash_file_name: PathBuf = lines
            .next()
            .ok_or("missing trash_file_name")?
            .trim_start_matches(&TrashInfo::FILENAME)
            .into();
        let source_path: PathBuf = lines
            .next()
            .ok_or("missing source_path")?
            .trim_start_matches(&TrashInfo::PATH)
            .into();
        let is_dir: bool = match lines
            .next()
            .ok_or("missing is_dir")?
            .trim_start_matches(&TrashInfo::ISDIR)
        {
            "true" => true,
            "false" => false,
            string => Err(format!("expected bool, found {}", string))?,
        };
        let deletion_date = DateTime::parse_from_rfc3339(
            lines
                .next()
                .ok_or("missing deletion date")?
                .trim_start_matches(&TrashInfo::DELETIONDATE),
        )?
        .with_timezone(&chrono::Utc);

        let file_size = lines.next().unwrap().trim_start_matches(&TrashInfo::FILESIZE).into();

        Ok(TrashInfo {
            trash_file_name,
            source_path,
            is_dir,
            deletion_date,
            file_size,
        })
    }
}
