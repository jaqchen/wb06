// Created by yejq.jiaqiang@gmail.com
// Simple Wubi-2008 table Lookup utility
// 2025/11/08

use std::io::{Write, BufRead, BufReader};
use std::os::raw::{c_void, c_char, c_int, c_uint};
use std::collections::HashMap;

use phf::phf_map;
use lazy_static::lazy_static;

struct Wubima {
	bm1:                [u8; 1],
	bm2:                [u8; 2],
	bm3:                [u8; 3],
	bm4:                [u8; 4],
}

impl Wubima {
	fn dump(&self, word: &str) -> String {
		let mut res = String::with_capacity(512);
		let line = format!("结果：  {} =>\n", word);
		res.push_str(line.as_str());
		if self.bm1[0] != 0 {
			let line = format!("\t{}\n", char::from_u32(self.bm1[0] as u32).unwrap());
			res.push_str(line.as_str());
		}
		if self.bm2[0] != 0 {
			let line = format!("\t{}\n", std::str::from_utf8(&self.bm2).unwrap());
			res.push_str(line.as_str());
		}
		if self.bm3[0] != 0 {
			let line = format!("\t{}\n", std::str::from_utf8(&self.bm3).unwrap());
			res.push_str(line.as_str());
		}
		if self.bm4[0] != 0 {
			let line = format!("\t{}\n", std::str::from_utf8(&self.bm4).unwrap());
			res.push_str(line.as_str());
		}
		res
	}
}

include!(concat!(env!("OUT_DIR"), "/wubi2008_map.rs"));

include!("../wubiform/wubiform.rs");

struct Wubihist {
	wbidx:        usize,
	wbhist:       Vec<String>,
	wbset:        HashMap<String, usize>,
}

impl Wubihist {
	fn new() -> Self {
		Self {
			wbidx:      0usize,
			wbhist:     Vec::with_capacity(256),
			wbset:      HashMap::new(),
		}
	}

	fn current(&self, s: &mut String) -> Option<usize> {
		let wlen = self.wbhist.len();
		if wlen == 0 {
			return None;
		}

		let idx = if self.wbidx >= wlen { 0usize } else { self.wbidx };
		s.push_str(self.wbhist[idx].as_str());
		Some(idx)
	}

	fn next(&mut self, s: &mut String) -> Option<usize> {
		let wlen = self.wbhist.len();
		if wlen == 0 {
			return None;
		}

		let mut idx = self.wbidx + 1;
		if idx >= wlen {
			idx = 0usize;
		}
		self.wbidx = idx;
		s.push_str(self.wbhist[idx].as_str());
		Some(idx)
	}

	fn load_file(&mut self, filp: &str) -> String {
		let hist = std::fs::OpenOptions::new()
			.read(true)
			.write(false)
			.create(false)
			.open(filp);
		if let Err(err) = hist {
			return format!("打开历史文件失败：{:?}", err);
		}

		let mut wnum = 0usize;
		let mut line = String::with_capacity(256);
		let mut hist = BufReader::new(hist.unwrap());
		self.wbidx = 0;
		self.wbset.clear();
		self.wbhist.clear();

		loop {
			line.clear();
			let ret = hist.read_line(&mut line);
			if let Err(err) = ret {
				return format!("读取历史文件失败：{:?}", err);
			}

			let ret = ret.unwrap();
			if ret == 0 {
				break;
			}

			let word: &str = line.trim();
			if WUBIMA_TABLE.contains_key(word) && !self.wbset.contains_key(word) {
				self.wbset.insert(word.to_string(), wnum);
				self.wbhist.push(word.to_string());
				wnum += 1;
			}
		}

		let first: &str = if wnum >= 1 { self.wbhist[0].as_str() } else { "<無>" };
		format!("历史文件加载成功，共{}个词组。\n第零个词组：\n\t{}", wnum, first)
	}

	fn store_file(&self, filp: &str) -> String {
		let hist = std::fs::OpenOptions::new()
			.read(false)
			.write(true)
			.create(true)
			.truncate(true)
			.open(filp);
		if let Err(err) = hist {
			return format!("创建历史文件失败：{:?}", err);
		}

		let mut wnum = 0usize;
		let mut hist = hist.unwrap();
		for hanzi in self.wbhist.iter() {
			let hanzi: &str = hanzi.as_str();
			if let Err(err) = writeln!(hist, "{}", hanzi) {
				return format!("历史文件写入失败：{:?}", err);
			}
			wnum += 1;
		}

		let mut word = String::with_capacity(256);
		if let Some(_) = self.current(&mut word) {
			format!("历史文件写入成功，共 {} 个词组。\n当前词组：\n\t{}", wnum, word)
		} else {
			format!("历史文件写入成功，共 {} 个词组。", wnum)
		}
	}

	fn add_word(&mut self, w: &str) -> bool {
		if !self.wbset.contains_key(w) {
			let wnum = self.wbhist.len();
			self.wbidx = wnum;
			self.wbset.insert(w.to_string(), wnum);
			self.wbhist.push(w.to_string());
			true
		} else { false }
	}
}

lazy_static! {
	static ref WUBI_HIST: std::sync::RwLock<Wubihist> = {
		std::sync::RwLock::new(Wubihist::new())
	};
}

extern "C" fn query_wubima(form: WubiForm,
	utf8p: *const c_char, utf8l: c_uint) -> c_int {
	let utf8d: &[u8] = unsafe { std::slice::from_raw_parts(utf8p as *const u8, utf8l as usize) };
	let utf8str = String::from_utf8_lossy(utf8d);

	let mut ret: c_int = 0;
	let msg: String = if let Some(wbm) = WUBIMA_TABLE.get(&utf8str) {
		if let Ok(mut history) = WUBI_HIST.write() {
			let _ = history.add_word(&utf8str);
		}
		wbm.dump(&utf8str)
	} else {
		ret = -1;
		format!("抱歉！五笔编码未找到：\n\t{}\n", utf8str)
	};

	let msgptr: &[u8] = msg.as_bytes();
	unsafe { wform_push_result(form, msgptr.as_ptr() as *const c_char, msgptr.len() as c_uint) };
	ret
}

extern "C" fn load_wubima(form: WubiForm,
	utf8p: *const c_char, utf8l: c_uint) -> c_int {
	let utf8d: &[u8] = unsafe { std::slice::from_raw_parts(utf8p as *const u8, utf8l as usize) };
	let utf8str = String::from_utf8_lossy(utf8d);

	let msg = if let Ok(mut history) = WUBI_HIST.write() {
		history.load_file(&utf8str)
	} else { "给历史记录加读写锁失败!".to_string() };

	let err = msg.as_str();
	unsafe { wform_push_result(form, err.as_ptr() as *const c_char, err.len() as c_uint) }
}

extern "C" fn store_wubima(form: WubiForm,
	utf8p: *const c_char, utf8l: c_uint) -> c_int {
	let utf8d: &[u8] = unsafe { std::slice::from_raw_parts(utf8p as *const u8, utf8l as usize) };
	let utf8str = String::from_utf8_lossy(utf8d);

	let msg = if let Ok(history) = WUBI_HIST.read() {
		history.store_file(&utf8str)
	} else { "给历史记录加读写锁失败!".to_string() };

	let err = msg.as_str();
	unsafe { wform_push_result(form, err.as_ptr() as *const c_char, err.len() as c_uint) }
}

extern "C" fn next_wubima(form: WubiForm,
	utf8p: *const c_char, utf8l: c_uint) -> c_int {
	let utf8d: &[u8] = unsafe { std::slice::from_raw_parts(utf8p as *const u8, utf8l as usize) };
	let utf8str: String = String::from_utf8_lossy(utf8d).into_owned();

	let Ok(mut history) = WUBI_HIST.write() else {
		let err = "获取当前历史词组索引读写锁失败！";
		return unsafe { wform_push_result(form, err.as_ptr() as *const c_char, err.len() as c_uint) };
	};

	let mut word = String::with_capacity(256);
	let Some(idx) = history.current(&mut word) else {
		let err = "错误！当前历史记录为空！";
		return unsafe { wform_push_result(form, err.as_ptr() as *const c_char, err.len() as c_uint) };
	};

	let msg: String = if utf8str == word {
		word.clear();
		if let Some(nidx) = history.next(&mut word) {
			format!("对比成功！下一个词组，序号 {}：\n\t{}\n", nidx, word)
		} else {
			"对比成功！但找不到下一个词组！".to_string()
		}
	} else {
		format!("对比不一致！请重新输入，序号 {}：\n\t{}\n", idx, word)
	};
	return unsafe { wform_push_result(form, msg.as_bytes().as_ptr() as *const c_char, msg.len() as c_uint) };
}

fn main() {
	let ret = unsafe {
		let form = wform_init_lib();
		wform_regiter_fun(form, WFB_QUERY, Some(query_wubima));
		wform_regiter_fun(form, WFB_LOAD, Some(load_wubima));
		wform_regiter_fun(form, WFB_STORE, Some(store_wubima));
		wform_regiter_fun(form, WFB_NEXT, Some(next_wubima));
		wform_looping(form)
	};
	std::process::exit(ret as i32);
}
