use libc::{self, S_IFDIR, S_IFLNK, S_IFMT, S_IFREG, S_IFSOCK};
use pico_args::Arguments;
use std::ffi::CString;
use std::io;

const STAT_HELP: &str = "\
fs-wrap stat

USAGE:
  fs-wrap stat [OPTIONS] [PATH]

FLAGS:
  -h, --help       Prints help information

OPTIONS:
  --extended       Show extended info (calls statx)

ARGS:
  <PATH>           Path to file.
";

const ASK_FOR_HELP: &str = "Get help: fs-wrap stat --help";

#[derive(Debug)]
struct AppArgs {
    extended: bool,
    path: String,
}

type PicoError = pico_args::Error;

fn main() {
    let mut pargs = Arguments::from_env();

    let command = ok_or_exit(pargs.subcommand());

    match command.as_deref() {
        Some("stat") => {
            let args = ok_or_exit(parse_args(&mut pargs));

            if args.path.is_empty() {
                println!("No path provided");
                print!("{}", ASK_FOR_HELP);
                std::process::exit(1);
            }

            if args.extended {
                println!("Extended Stats");
            }

            stat(&args.path)
        }
        Some(other) => {
            eprintln!("Unknown command - {}", other);
            print!("{}", ASK_FOR_HELP);
        }
        None => {
            eprintln!("No commands provided");
            print!("{}", ASK_FOR_HELP);
        }
    }
}

fn ok_or_exit<T>(result: Result<T, PicoError>) -> T {
    match result {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    }
}

fn parse_args(pargs: &mut Arguments) -> Result<AppArgs, PicoError> {
    if pargs.contains(["-h", "--help"]) {
        print!("{}", STAT_HELP);
        std::process::exit(0);
    }

    let args = AppArgs {
        extended: pargs.contains("--extended"),
        path: pargs.free_from_str()?,
    };

    Ok(args)
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
#[allow(dead_code)]
struct Stat {
    device: u64, // Identified by Major and Minor IDs
    inode: u64,
    file_type: FileType,
    hard_links: u32,
    uid: u32,
    gid: u32,
    size: i64,       // Bytes
    block_size: i32, // Optimal block size for reading
    blocks: i64,     // 512-byte blocks
    atime: i64,
    mtime: i64,
    ctime: i64,
}

#[derive(Debug)]
enum FileType {
    Directory,
    RegularFile,
    SymLink,
    Socket,
    Other,
}

fn file_type(st_mode: u32) -> FileType {
    match st_mode & S_IFMT {
        S_IFDIR => FileType::Directory,
        S_IFREG => FileType::RegularFile,
        S_IFLNK => FileType::SymLink,
        S_IFSOCK => FileType::Socket,
        _ => FileType::Other,
    }
}

fn show_stat(buf: libc::stat) {
    let stats = Stat {
        device: buf.st_dev,
        inode: buf.st_ino,
        file_type: file_type(buf.st_mode),
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

    println!("{:#?}", stats);
}
