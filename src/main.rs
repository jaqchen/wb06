// Created by yejq.jiaqiang@gmail.com
// Simple Wubi-2008 table Lookup utility
// 2025/11/08

use phf::phf_map;

struct Wubima {
	bianma_1:           [u8; 1],
	bianma_2:           [u8; 2],
	bianma_3:           [u8; 3],
	bianma_4:           [u8; 4],
}

impl Wubima {
	fn dump(&self, word: &str) {
		println!("五笔编码：  {} =>", word);
		if self.bianma_1[0] != 0 {
			println!("\t{}", char::from_u32(self.bianma_1[0] as u32).unwrap());
		}
		if self.bianma_2[0] != 0 {
			println!("\t{}", std::str::from_utf8(&self.bianma_2).unwrap());
		}
		if self.bianma_3[0] != 0 {
			println!("\t{}", std::str::from_utf8(&self.bianma_3).unwrap());
		}
		if self.bianma_4[0] != 0 {
			println!("\t{}", std::str::from_utf8(&self.bianma_4).unwrap());
		}
	}
}

include!(concat!(env!("OUT_DIR"), "/wubi2008_map.rs"));

fn main() {
	// 从命令行收集需要查询的词组:
	let words: Vec<String> = std::env::args().skip(1).collect();
	if words.is_empty() {
		eprintln!("错误！您没有在命令行上指定您希望查词的词组。");
		std::process::exit(1);
	}

	words.iter().for_each(|word| {
		let word: &str = word.as_str();
		if let Some(wbm) = WUBIMA_TABLE.get(word) {
			wbm.dump(word);
		} else {
			eprintln!("失败！未找到词组的五笔编码： {}", word);
		}
	});
}
