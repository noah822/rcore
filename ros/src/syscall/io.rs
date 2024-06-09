const STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize){
    match fd {
        STDOUT => {
           let data_slice = unsafe {core::slice::from_raw_parts(buf, len)};
           let str = core::str::from_utf8(data_slice).unwrap();
           print!("{}", str);

        },
        _ => println!("[kernel] invalid fd")
    }

}