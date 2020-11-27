#![feature(asm)]
use std::cmp::Ordering;
use std::str::from_utf8_unchecked;

fn main() {
    let s = "Enter you guess: ";

    let mut buf = [0u8; 16];
    let mut bytes_written: usize;

    let mut rando_buf = [0u8; 8];

    let fd: u32; // fd for /dev/urandom
    let filename = "/dev/urandom\0";

    unsafe {
        asm! {
            // Open /dev/urandom
            "syscall",
            in("rax") 2,
            in("rdi") filename.as_ptr(),
            in("rsi") 0,
            lateout("rax") fd,
        };

        asm! {
            // Read from /dev/urandom
            "syscall",
            in("rax") 0,
            in("rdi") fd,
            in("rsi") rando_buf.as_mut_ptr(),
            in("rdx") 8,
        };
    }

    loop {
        unsafe {
            asm! {
                // Print string
                "syscall",
                in("rax") 1,
                in("rdi") 1,
                in("rsi") s.as_ptr(),
                in("rdx") s.len(),
            };

            asm! {
                // Read string
                "syscall",
                in("rax") 0,
                in("rdi") 0,
                in("rsi") buf.as_mut_ptr(),
                in("rdx") buf.len(),
                lateout("rax") bytes_written,
            };

            let guess = from_utf8_unchecked(&buf[..bytes_written])
                .trim()
                .parse::<u64>()
                .unwrap();
            let secret = u64::from_ne_bytes(rando_buf) % 255;

            use Ordering::*;
            match guess.cmp(&secret) {
                Less => eprintln!("Too small"),
                Greater => eprintln!("Too big"),
                Equal => {
                    eprintln!("You done gone and done it");
                    break;
                }
            }
        }
    }

    // Cleanup and close fd
}
