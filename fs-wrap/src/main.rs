use libc;
use std::env;
use std::ffi::CString;
use std::io;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let command = args
        .first()
        .unwrap_or_else(|| {
            println!("No command provided");
            std::process::exit(1)
        })
        .as_str();

    match command {
        "stat" => {
            let path = args
                .get(1)
                .unwrap_or_else(|| {
                    println!("No path provided");
                    std::process::exit(1)
                })
                .as_str();

            stat(path);
        }
        _ => println!("Unknown command - {}", command),
    }
}

fn stat(path: &str) {
    let mut buf: libc::stat = unsafe { core::mem::zeroed() };
    let path = CString::new(path).expect("CString::new failed");
    let err = unsafe { libc::stat(path.as_ptr(), &mut buf) };
    if err == -1 {
        println!("{:?}", io::Error::last_os_error());
    } else {
        show_stat(buf);
    }
}

#[derive(Debug)]
struct Stat {
    device: u64,
    inode: u64,
    file_type: u32,
    hard_links: u32,
    uid: u32,
    gid: u32,
    size: i64,
    block_size: i32,
    blocks: i64,
    atime: i64,
    mtime: i64,
    ctime: i64,
}

fn show_stat(buf: libc::stat) {
    let stats = Stat {
        device: buf.st_dev,
        inode: buf.st_ino,
        file_type: buf.st_mode,
        hard_links: buf.st_nlink,
        uid: buf.st_uid,
        gid: buf.st_gid,
        size: buf.st_size,
        block_size: buf.st_blksize,
        blocks: buf.st_blocks,
        atime: buf.st_atime,
        mtime: buf.st_mtime,
        ctime: buf.st_ctime,
    };

    println!("Stats: {:?}", stats);
}
