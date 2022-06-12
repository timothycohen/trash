use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};
use uuid::Uuid;

#[derive(Debug)]
pub struct TrashNames {
    pub trash_file_name: PathBuf,
    pub trash_info_name: PathBuf,
}

// #[derive(Debug)]
// pub enum StripError {
//     TooShort,
//     InvalidUuid,
//     InvalidDelimiter,
// }

impl TrashNames {
    pub fn from_base_name(file_basename: PathBuf) -> TrashNames {
        let mut trash_file_name = file_basename.file_name().expect("not empty").to_owned();
        trash_file_name.push(".");
        trash_file_name.push(Uuid::new_v4().to_string());
        Self::from_trash_file_name(PathBuf::from(trash_file_name))
    }

    // pub fn from_trash_info_name(trash_info_name: PathBuf) -> TrashNames {
    //     TrashNames {
    //         trash_file_name: PathBuf::from(trash_info_name.file_stem().expect("not empty")),
    //         trash_info_name: PathBuf::from(trash_info_name.file_name().expect("not empty")),
    //     }
    // }

    pub fn from_trash_file_name(trash_file_name: PathBuf) -> TrashNames {
        let trash_file_name = PathBuf::from(trash_file_name.file_name().expect("not empty"));
        let trash_info_name = Self::append_ext(&trash_file_name, ".trashinfo");
        TrashNames {
            trash_file_name,
            trash_info_name,
        }
    }

    fn append_ext(trash_file_name: &Path, ext: &str) -> PathBuf {
        let mut trash_info_name: OsString = trash_file_name.into();
        trash_info_name.push(ext);
        PathBuf::from(trash_info_name)
    }

    // pub fn strip_uuid(trash_file_name: PathBuf) -> Result<(PathBuf, Uuid), StripError> {
    //     let trash_file_name = match trash_file_name.extension().ok_or(StripError::InvalidDelimiter)? == "trashinfo" {
    //         true => trash_file_name.file_stem().expect("not empty"),
    //         false => trash_file_name.file_name().expect("not empty"),
    //     };

    //     // check that it has at least a 36 char uuid and a `.` in the final spot
    //     println!("{:?}", trash_file_name);
    //     let len = trash_file_name.len();
    //     if len <= 37 {
    //         return Err(StripError::TooShort);
    //     }

    //     // TODO check that this doesn't affect 한글 or anything
    //     let trash_file_name = trash_file_name.to_string_lossy();
    //     let (mut file_basename, uuid) = trash_file_name.split_at(len - 36);

    //     file_basename = file_basename.strip_suffix('.').ok_or(StripError::InvalidDelimiter)?;
    //     let uuid = Uuid::parse_str(uuid).map_err(|_| StripError::InvalidUuid)?;

    //     Ok((PathBuf::from(file_basename), uuid))
    // }
}

// #[test]
// fn from_base() {
//     let trash_names = TrashNames::from_base_name(
//         PathBuf::from("~/dev/a/foo.txt"),
//         Uuid::parse_str("23f9089d-62b2-4102-80ae-de95ee4d66d0").unwrap(),
//     );
//     assert_eq!(
//         PathBuf::from("foo.txt.23f9089d-62b2-4102-80ae-de95ee4d66d0"),
//         trash_names.trash_file_name
//     );
//     assert_eq!(
//         PathBuf::from("foo.txt.23f9089d-62b2-4102-80ae-de95ee4d66d0.trashinfo"),
//         trash_names.trash_info_name
//     );
// }

#[test]
fn from_trash_file() {
    let trash_names =
        TrashNames::from_trash_file_name(PathBuf::from("~/dev/b.txt.23f9089d-62b2-4102-80ae-de95ee4d66d0"));

    assert_eq!(
        PathBuf::from("b.txt.23f9089d-62b2-4102-80ae-de95ee4d66d0"),
        trash_names.trash_file_name
    );
    assert_eq!(
        PathBuf::from("b.txt.23f9089d-62b2-4102-80ae-de95ee4d66d0.trashinfo"),
        trash_names.trash_info_name
    );
}

// #[test]
// fn from_trash_info() {
//     let trash_names = TrashNames::from_trash_info_name(PathBuf::from(
//         "~/dev/c.txt.23f9089d-62b2-4102-80ae-de95ee4d66d0.trashinfo",
//     ));
//     assert_eq!(
//         PathBuf::from("c.txt.23f9089d-62b2-4102-80ae-de95ee4d66d0"),
//         trash_names.trash_file_name
//     );
//     assert_eq!(
//         PathBuf::from("c.txt.23f9089d-62b2-4102-80ae-de95ee4d66d0.trashinfo"),
//         trash_names.trash_info_name
//     );
// }

// #[test]
// fn strip_uuid() {
//     let abs = "~/dev/";
//     let file = "foo";
//     let ex1 = ".txt";
//     let ex2 = ".bz";
//     let uuid = ".23f9089d-62b2-4102-80ae-de95ee4d66d0";
//     let suffix = ".trashinfo";

//     let asserter = |path: String, ex2| {
//         assert_eq!(
//             TrashNames::strip_uuid(PathBuf::from(path)).unwrap().0,
//             PathBuf::from(format!("foo.txt{}", ex2))
//         )
//     };
//     asserter(format!("{}{}{}{}{}{}", abs, file, ex1, ex2, uuid, suffix), ".bz");
//     asserter(format!("{}{}{}{}{}", abs, file, ex1, ex2, uuid), ".bz");
//     asserter(format!("{}{}{}{}{}", file, ex1, ex2, uuid, suffix), ".bz");
//     asserter(format!("{}{}{}{}", file, ex1, ex2, uuid), ".bz");
//     asserter(format!("{}{}{}{}{}", abs, file, ex1, uuid, suffix), "");
//     asserter(format!("{}{}{}{}", abs, file, ex1, uuid), "");
//     asserter(format!("{}{}{}{}", file, ex1, uuid, suffix), "");
//     asserter(format!("{}{}{}", file, ex1, uuid), "");
// }
