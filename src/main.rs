// Created by yejq.jiaqiang@gmail.com
// Simple Wubi-2008 table Lookup utility
// 2025/11/08

use std::collections::HashMap;
use std::io::{Read, BufRead, BufReader};

struct Wubima {
	word:               String,
	bianma_1:           [u8; 1],
	bianma_2:           [u8; 2],
	bianma_3:           [u8; 3],
	bianma_4:           [u8; 4],
}

impl Wubima {
	fn new(w: &str) -> Self {
		Self {
			word:          w.to_string(),
			bianma_1:      [0u8, ],
			bianma_2:      [0u8, 0u8, ],
			bianma_3:      [0u8, 0u8, 0u8, ],
			bianma_4:      [0u8, 0u8, 0u8, 0u8, ],
		}
	}

	fn dump(&self) {
		println!("词组的五笔编码：  {} =>", self.word);
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

	fn update_bm(&mut self, bm: &str) -> bool {
		let bmc: &[u8] = bm.as_bytes();
		match bm.len() {
			1 => {
				if self.bianma_1[0] != 0 {
					eprintln!("警告：词组已存在一级编码：{} => {}",
						self.word, char::from_u32(self.bianma_1[0] as u32).unwrap());
				}
				self.bianma_1[0] = bmc[0];
			},
			2 => {
				if self.bianma_2[0] != 0 {
					eprintln!("警告：词组已存在二级编码：{} => {}",
						self.word, std::str::from_utf8(&self.bianma_2).unwrap());
				}
				// TODO: use slice copy method
				self.bianma_2[0] = bmc[0];
				self.bianma_2[1] = bmc[1];
			},
			3 => {
				if self.bianma_3[0] != 0 {
					eprintln!("警告：词组已存在三级编码：{} => {}",
						self.word, std::str::from_utf8(&self.bianma_3).unwrap());
				}
				self.bianma_3[0] = bmc[0];
				self.bianma_3[1] = bmc[1];
				self.bianma_3[2] = bmc[2];
			},
			4 => {
				if self.bianma_4[0] != 0 {
					eprintln!("警告：词组已存在终级编码：{} => {}",
						self.word, std::str::from_utf8(&self.bianma_4).unwrap());
				}
				self.bianma_4[0] = bmc[0];
				self.bianma_4[1] = bmc[1];
				self.bianma_4[2] = bmc[2];
				self.bianma_4[3] = bmc[3];
			},
			_ => {
				eprintln!("错误！不能为词组更新五笔编码： {} => {}",
					self.word, bm);
				return false;
			},
		}
		return true;
	}

	fn validate_bm(bm: &str) -> bool {
		let b: &[u8] = bm.as_bytes();
		match b.len() {
			1 => b[0] >= b'a' && b[0] <= b'z',
			2 => b[0] >= b'a' && b[0] <= b'z' && b[1] >= b'a' && b[1] <= b'z',
			3 => b[0] >= b'a' && b[0] <= b'z' && b[1] >= b'a' && b[1] <= b'z' && b[2] >= b'a' && b[2] <= b'z',
			4 => b[0] >= b'a' && b[0] <= b'z' && b[1] >= b'a' && b[1] <= b'z' && b[2] >= b'a' && b[2] <= b'z' && b[3] >= b'a' && b[3] <= b'z',
			_ => false,
		}
	}

	fn new_bm(wtab: &mut HashMap<String, Wubima>, bm: &str, wlist: &[&str]) {
		wlist.iter().for_each(|word| {
			let word: &str = *word;
			if let Some(wbm) = wtab.get_mut(word) {
				wbm.update_bm(bm);
			} else {
				let mut wbm = Wubima::new(word);
				wbm.update_bm(bm);
				wtab.insert(word.to_string(), wbm);
			}
		});
	}
}


fn main() {
	// 从命令行收集需要查询的词组:
	let words: Vec<String> = std::env::args().skip(1).collect();
	if words.is_empty() {
		eprintln!("错误！您没有在命令行上指定您希望查词的词组。");
		std::process::exit(1);
	}

	// 码表名称：
	let dbfile: &str = "新世纪五笔词库";
	// 确认码表文件的路径：
	let mut dbpath = if std::fs::exists(dbfile).unwrap_or(false) {
		std::path::PathBuf::with_capacity(256)
	} else {
		std::env::current_exe().unwrap()
	};
	dbpath.set_file_name(dbfile);

	// 打开2008版五笔词库文件：
	let wdb = match std::fs::OpenOptions::new()
		.read(true)
		.write(false)
		.create(false)
		.create_new(false)
		.open(&dbpath) {
		Ok(mut file) => {
			// 跳过前三字节：0xef 0xbb 0xbf
			let mut bcs = [0u8; 3];
			let _ = file.read_exact(&mut bcs);
			file
		},
		Err(err) => {
			eprintln!("打开文件 《新世纪五笔词库》失败: {:?}", err);
			std::process::exit(2);
		},
	};

	// 以行的方式读取五笔词库：
	let mut wdb = BufReader::new(wdb);
	// TODO: 使用编译时生成的查询表，compile-time lookup table
	// 创建简单的哈稀表，以词组为键值，Wubima为键值：
	let mut wordmap: HashMap<String, Wubima> = HashMap::new();

	let mut wline = String::with_capacity(1024);
	// 循环读取五笔编码：
	loop {
		wline.clear();
		let rline = wdb.read_line(&mut wline);
		if let Err(err) = rline {
			eprintln!("错误！读取五笔词库文件失败：{:?}", err);
			break;
		}

		let rline = rline.unwrap();
		if rline == 0 {
			// 文件末尾，循环结束。
			break;
		}

		let cline: &str = wline.trim();
		let words: Vec<&str> = cline.split_whitespace().collect();
		if words.len() <= 1 {
			eprintln!("错误！分割行失败，忽略行： {}", cline);
			continue;
		}

		if Wubima::validate_bm(words[0]) {
			Wubima::new_bm(&mut wordmap, words[0], &words[1..]);
		} else {
			eprintln!("错误！无效的五笔词组编码： {}", cline);
			continue;
		}
	}

	// 释放之前创建的BufReader对象:
	drop(wdb);

	println!("共处理 {} 个词组。", wordmap.len());
	words.iter().for_each(|word| {
		let word: &str = word.as_str();
		if let Some(wbm) = wordmap.get(word) {
			wbm.dump();
		} else {
			eprintln!("失败！未找到词组的五笔编码： {}", word);
		}
	});
}
