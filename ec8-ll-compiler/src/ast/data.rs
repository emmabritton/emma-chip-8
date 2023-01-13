use crate::parser::line::Line;
use ec8_common::MAX_PROG_SIZE;

#[derive(Debug, Clone)]
pub struct Data {
    pub name: String,
    pub bytes: Vec<u8>,
    pub addr: u16
}

impl Data {
    pub fn new(name: String, addr: u16, bytes: Vec<u8>) -> Self {
        Self { name, addr, bytes }
    }
}

pub fn extract_data(lines: &[Line]) -> Result<Vec<Data>, String> {
    let mut datas = vec![];

    let max = MAX_PROG_SIZE - 10;
    let total = total_data(lines);
    if total >= max {
        return Err(format!(
            "Too many data bytes, max is {max}b, found {total}b"
        ));
    }

    let mut addr = 0;
    for line in lines {
        if let Line::Data {
            line: _,
            name,
            data,
        } = line
        {
            datas.push(Data::new(name.clone(), addr, data.clone()));
            addr += data.len() as u16;
        }
    }
    Ok(datas)
}

fn total_data(lines: &[Line]) -> usize {
    let mut total = 0;
    for line in lines {
        if let Line::Data {
            line: _,
            name: _,
            data,
        } = line
        {
            total += data.len();
        }
    }
    total
}
