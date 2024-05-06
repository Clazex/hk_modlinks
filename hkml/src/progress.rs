use std::io::{self, prelude::*, BufReader};

use lazy_static::lazy_static;

use indicatif::{ProgressBar, ProgressStyle};

use crate::{Result, DEFAULT_BUF_SIZE};

const PROGRESS_BAR_TEMPLATE: &str =
    "{prefix:>16.cyan.bold} [{elapsed_precise}] [{bar:40}] {spinner} {percent:>2}% ({total_bytes:9}) {binary_bytes_per_sec:>11.green.bold} ETA: {eta} {msg}";
const SPINNER_TEMPLATE: &str =
    "{prefix:>16.cyan.bold} [{elapsed_precise}] {spinner} {binary_bytes_per_sec:>11.green.bold} {msg}";

lazy_static! {
    static ref PROGRESS_BAR_STYLE: ProgressStyle = ProgressStyle::default_bar()
        .template(PROGRESS_BAR_TEMPLATE)
        .unwrap()
        .progress_chars("=> ");
    static ref SPINNER_STYLE: ProgressStyle = ProgressStyle::default_spinner()
        .template(SPINNER_TEMPLATE)
        .unwrap();
}

pub fn copy_pb_buf_read<R: Read, W: Write>(
    r: &mut R,
    w: &mut W,
    size: Option<usize>,
    action: &'static str,
) -> Result {
    match size {
        Some(size) => {
            let pb = ProgressBar::new(size as u64)
                .with_style(PROGRESS_BAR_STYLE.clone())
                .with_prefix(action);

            copy_pb_buf_read_inner(r, w, &pb)
                .and_then(|copied| {
                    if copied == size {
                        Ok(())
                    } else {
                        Err(io::Error::other(format!(
                            "Failed to copy data when {action}"
                        )))
                    }
                })
                .map_err(|e| {
                    pb.abandon_with_message(format!("Failed when {action}"));
                    e
                })?;
        }
        None => {
            let pb = ProgressBar::new_spinner()
                .with_style(SPINNER_STYLE.clone())
                .with_prefix(action);

            copy_pb_buf_read_inner(r, w, &pb).map_err(|e| {
                pb.abandon_with_message(format!("Failed when {action}"));
                e
            })?;
        }
    }

    Ok(())
}

pub fn copy_pb_slice<W: Write>(slice: &[u8], w: &mut W, action: &'static str) -> Result {
    let size = slice.len();
    let pb = ProgressBar::new(size as u64)
        .with_style(PROGRESS_BAR_STYLE.clone())
        .with_prefix(action);

    let mut bytes_written = 0;

    // Write chunks until size exceeds
    if size > DEFAULT_BUF_SIZE {
        loop {
            match w.write(&slice[bytes_written..bytes_written + DEFAULT_BUF_SIZE]) {
                Ok(0) => Err(io::Error::other(format!("Failed to write when {action}")))?,
                Ok(n) => {
                    bytes_written += n;
                    pb.inc(n as u64);

                    if bytes_written + DEFAULT_BUF_SIZE > size {
                        break;
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => Err(e)?,
            }
        }
    }

    // Write the rest
    if bytes_written < size {
        loop {
            match w.write(&slice[bytes_written..]) {
                Ok(0) => Err(io::Error::other(format!(
                    "Failed to write last part when {action}"
                )))?,
                Ok(n) => {
                    bytes_written += n;
                    pb.inc(n as u64);

                    if bytes_written >= size {
                        break;
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => Err(e)?,
            }
        }
    }

    Ok(())
}

fn copy_pb_buf_read_inner<R: Read, W: Write>(
    r: &mut R,
    w: &mut W,
    pb: &ProgressBar,
) -> io::Result<usize> {
    let mut buf_read = BufReader::with_capacity(DEFAULT_BUF_SIZE, r);
    let mut bytes_read = 0;

    loop {
        let buf = buf_read.fill_buf()?;
        let len = buf.len();

        if len == 0 {
            break;
        }

        w.write_all(buf)?;
        buf_read.consume(len);

        bytes_read += len;
        pb.inc(len as u64);
    }

    Ok(bytes_read)
}
