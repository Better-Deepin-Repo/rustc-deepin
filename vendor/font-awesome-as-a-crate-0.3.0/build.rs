use std::{
    env,
    fs::{read_dir, File},
    io::{Read, Write},
    path::Path,
};

fn main() {
    println!("cargo:rustc-cfg=font_awesome_out_dir");
    write_fontawesome_sprite();
}

fn write_fontawesome_sprite() {
    let dest_path = Path::new(&env::var("OUT_DIR").unwrap()).join("fontawesome.rs");
    let mut dest_file = File::create(&dest_path).unwrap();
    dest_file
        .write_all(b"const fn fontawesome_svg(dir:&str,file:&str)->&'static str{match(dir.as_bytes(),file.as_bytes()){")
        .expect("fontawesome fn write");
    for dirname in &["brands", "regular", "solid"] {
        let dir = read_dir(
            Path::new("/usr/share/nodejs/@fortawesome/fontawesome-free/svgs/").join(dirname),
        )
        .expect("fontawesome directory exists");
        let mut data = String::new();
        for file in dir {
            let file = file.expect("fontawesome directory access");
            data.clear();
            let filename = file
                .file_name()
                .into_string()
                .expect("fontawesome filenames are unicode");
            if !filename.ends_with(".svg") {
                continue;
            }
            let mut file = File::open(file.path()).expect("fontawesome file access");
            file.read_to_string(&mut data)
                .expect("fontawesome file read");
            if data.is_empty() {
                panic!("encountered invalid fontawesome data in {}", filename);
            }
            // if this assert goes off, add more hashes here and in the format! below
            assert!(!data.contains("###"), "file {} breaks raw string", filename);
            dest_file
                .write_all(
                    format!(
                        r####"(b"{dirname}",b"{filename}")=>r###"{data}"###,"####,
                        data = data,
                        dirname = dirname,
                        filename = filename.replace(".svg", ""),
                    )
                    .as_bytes(),
                )
                .expect("write fontawesome file");
        }
    }
    dest_file
        .write_all(b"_=>\"\"}}")
        .expect("fontawesome fn write");
}
