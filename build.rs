// Created by yejq.jiaqiang@gmail.com
// Simple Wubi-2008 table Lookup utility
// 2025/11/08

use std::collections::HashMap;
use std::io::{Read, Write, BufRead, BufReader};

struct Wubima {
	word:               String,
	bm1:                [u8; 1],
	bm2:                [u8; 2],
	bm3:                [u8; 3],
	bm4:                [u8; 4],
}

impl Wubima {
	fn new(w: &str) -> Self {
		Self {
			word:          w.to_string(),
			bm1:      [0u8, ],
			bm2:      [0u8, 0u8, ],
			bm3:      [0u8, 0u8, 0u8, ],
			bm4:      [0u8, 0u8, 0u8, 0u8, ],
		}
	}

	fn dump(&self, file: &mut std::fs::File) -> std::io::Result<usize> {
		let mut res = file.write("Wubima {\n".as_bytes())?;
		write!(file, "\t\tbm1: [{}, ],\n", self.bm1[0])?;
		write!(file, "\t\tbm2: [{}, {}, ],\n", self.bm2[0], self.bm2[1])?;
		write!(file, "\t\tbm3: [{}, {}, {}, ],\n", self.bm3[0], self.bm3[1], self.bm3[2])?;
		write!(file, "\t\tbm4: [{}, {}, {}, {}, ],\n", self.bm4[0], self.bm4[1], self.bm4[2], self.bm4[3])?;
		res += file.write("\t},\n".as_bytes())?;
		Ok(res)
	}

	fn update_bm(&mut self, bm: &str) -> bool {
		let bmc: &[u8] = bm.as_bytes();
		match bm.len() {
			1 => {
				if self.bm1[0] != 0 {
					eprintln!("警告：词组已存在一级编码：{} => {}",
						self.word, char::from_u32(self.bm1[0] as u32).unwrap());
				}
				self.bm1[0] = bmc[0];
			},
			2 => {
				if self.bm2[0] != 0 {
					eprintln!("警告：词组已存在二级编码：{} => {}",
						self.word, std::str::from_utf8(&self.bm2).unwrap());
				}
				// TODO: use slice copy method
				self.bm2[0] = bmc[0];
				self.bm2[1] = bmc[1];
			},
			3 => {
				if self.bm3[0] != 0 {
					eprintln!("警告：词组已存在三级编码：{} => {}",
						self.word, std::str::from_utf8(&self.bm3).unwrap());
				}
				self.bm3[0] = bmc[0];
				self.bm3[1] = bmc[1];
				self.bm3[2] = bmc[2];
			},
			4 => {
				if self.bm4[0] != 0 {
					eprintln!("警告：词组已存在终级编码：{} => {}",
						self.word, std::str::from_utf8(&self.bm4).unwrap());
				}
				self.bm4[0] = bmc[0];
				self.bm4[1] = bmc[1];
				self.bm4[2] = bmc[2];
				self.bm4[3] = bmc[3];
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
	// 生成五笔码表代码文件的两个依赖：
	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-changed=新世纪五笔词库");

	// 获得编译输出文件夹：
	let outdir: String = match std::env::var("OUT_DIR") {
		Ok(dir) => dir,
		Err(err) => {
			eprintln!("错误！环境变量未定义：OUT_DIR => {:?}", err);
			std::process::exit(1);
		},
	};

	// 码表文件的路径为工程根目录：
	let mut dbpath = std::env::current_dir().unwrap();
	dbpath.push("新世纪五笔词库");

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
		}
	}

	// 释放之前创建的BufReader对象:
	drop(wdb);

	println!("共处理 {} 个词组。", wordmap.len());

	// 创建码表代码文件，wubi2008_map.rs
	let mut wbfile = std::fs::OpenOptions::new()
		.read(false)
		.write(true)
		.create(true)
		.truncate(true)
		.open(std::path::Path::new(&outdir).join("wubi2008_map.rs"))
		.unwrap();

	// 写入五笔码表的定义：
	let res = wbfile.write_all("static WUBIMA_TABLE: phf::Map<&'static str, Wubima> = phf_map! {\n".as_bytes());
	if let Err(err) = res {
		eprintln!("错误！写入 phf 预定义表失败：{:?}", err);
		std::process::exit(3);
	}

	wordmap.iter().for_each(|(word, wbm)| {
		let word: &str = word.as_str();
		let define = format!("\t\"{}\" => ", word);
		let mut res = wbfile.write(define.as_bytes());
		if res.is_ok() {
			res = wbm.dump(&mut wbfile);
		}

		if let Err(err) = res {
			eprintln!("错误！写入五笔码表文件失败： {:?}", err);
			std::process::exit(4);
		}
	});

	wbfile.write_all("};\n".as_bytes()).unwrap();
	wbfile.sync_all().unwrap();
}
