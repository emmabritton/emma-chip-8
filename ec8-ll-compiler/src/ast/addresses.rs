use crate::ast::Program;
use std::collections::HashMap;
use ec8_common::PROG_START_ADDRESS;
use crate::ast::data::Data;
use crate::parser::line::tokens::Param;

impl Program {
    pub fn set_addresses(&mut self) {
        let mut labels = HashMap::new();
        let data_start = (self.asm_lines.len() * 2) as u16;
        let datas: HashMap<&String, &Data> = self.datas.iter().map(|data| (&data.name, data)).collect();

        for (i, line) in self.asm_lines.iter().enumerate() {
            for lbl in &line.labels {
                labels.insert(lbl.to_string(), (i * 2) as u16);
            }
        }

        for line in &mut self.asm_lines {
            let mut addrs = HashMap::new();
            for (i,param) in line.params.iter().enumerate() {
                match param {
                    Param::Label(txt) => {
                        let addr = *labels.get(txt).unwrap_or_else(|| panic!("Label {txt} not found, please raise an issue"));
                        addrs.insert(i, Param::Addr(addr + PROG_START_ADDRESS));
                    },
                    Param::Data(txt) => {
                        let dat = datas.get(txt).unwrap_or_else(|| panic!("Data {txt} not found, please raise an issue"));
                        addrs.insert(i, Param::Addr(dat.addr + data_start + PROG_START_ADDRESS));
                    }
                    Param::Unknown(txt) => {
                        let addr = labels.get(txt).map(|num| *num).unwrap_or_else(|| datas.get(txt).map(|dat| dat.addr + data_start ).unwrap_or_else(|| panic!("{txt} not found as data or label, please raise an issue")));
                        addrs.insert(i, Param::Addr(addr + PROG_START_ADDRESS));
                    }
                    _ => {}
                }
            }
            for (i, addr) in addrs {
                line.params.remove(i);
                line.params.insert(i, addr);
            }
        }
    }
}
