use std::{borrow::Cow, io::BufRead, result};

pub type Result<T> = result::Result<T, Cow<'static, str>>;

pub fn get_order(input: &mut impl BufRead, line_buf: &mut String) -> Result<Vec<(u8, u8)>> {
    let mut order = Vec::with_capacity(128);

    let res = loop {
        line_buf.clear();
        if let 1.. = input.read_line(line_buf).map_err(|e| Cow::Owned(e.to_string()))? {

            if let Some((a, b)) = line_buf.trim_ascii().split_once('|') {

                if let [Ok(a), Ok(b)] = [a, b].map(str::parse::<u8>) {
                    order.push((a, b));
                } else {
                    break Err(Cow::Owned(format!("Couldn't parse rule line: {line_buf}")))
                }
            } else {
                break Ok(())
            }
        } else {
            break Err(Cow::Borrowed("No update lines found"))
        }
    };
    line_buf.clear();
    res.and(Ok(order))
}
