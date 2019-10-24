mod root_word;

use std::fs;
use serde_json::json;

use std::collections::HashSet;

use root_word::RootWord;
use root_word::value::Value;
use root_word::affix::Affix;
use root_word::source::Source;

fn main() {
  let data_r:     String
    = fs::read_to_string("data/gismu.txt").expect("err");
  let data_s_en:  String
    = fs::read_to_string("data/lojban-source-words_en.txt").expect("err");
  let data_s_es:  String
    = fs::read_to_string("data/lojban-source-words_es.txt").expect("err");
  let data_s_hi:  String
    = fs::read_to_string("data/lojban-source-words_hi.txt").expect("err");
  let data_s_ru:  String
    = fs::read_to_string("data/lojban-source-words_ru.txt").expect("err");
  let data_s_zh:  String
    = fs::read_to_string("data/lojban-source-words_zh.txt").expect("err");
  let head_r:     Vec<&str> = data_r.split("\n").skip(1).collect();
  let head_s_en:  Vec<&str> = data_s_en.split("\n").skip(1).collect();
  let head_s_es:  Vec<&str> = data_s_es.split("\n").skip(1).collect();
  let head_s_hi:  Vec<&str> = data_s_hi.split("\n").skip(1).collect();
  let head_s_ru:  Vec<&str> = data_s_ru.split("\n").skip(1).collect();
  let head_s_zh:  Vec<&str> = data_s_zh.split("\n").skip(1).collect();
  let tail_r:     usize     = head_r.len() - 1;
  let tail_s_en:  usize     = head_s_en.len() - 1;
  let tail_s_es:  usize     = head_s_es.len() - 1;
  let tail_s_hi:  usize     = head_s_hi.len() - 1;
  let tail_s_ru:  usize     = head_s_ru.len() - 1;
  let tail_s_zh:  usize     = head_s_zh.len() - 1;
  let lines_r:    Vec<&str> = head_r.into_iter().take(tail_r).collect();
  let lines_s_en: Vec<&str> = head_s_en.into_iter().take(tail_s_en).collect();
  let lines_s_es: Vec<&str> = head_s_es.into_iter().take(tail_s_es).collect();
  let lines_s_hi: Vec<&str> = head_s_hi.into_iter().take(tail_s_hi).collect();
  let lines_s_ru: Vec<&str> = head_s_ru.into_iter().take(tail_s_ru).collect();
  let lines_s_zh: Vec<&str> = head_s_zh.into_iter().take(tail_s_zh).collect();

  for line_r in lines_r {
    let value_r:   String        = line_r.chars().skip(1 ).take(5).collect();
    let value_o:   Option<Value> = Value::new(value_r);

    if value_o.is_some() {
      let value                  = value_o.unwrap();
      let affix_0_v: String      = line_r.chars().skip(7 ).take(4).collect();
      let affix_1_v: String      = line_r.chars().skip(11).take(4).collect();
      let affix_2_v: String      = line_r.chars().skip(15).take(5).collect();
      let affix_v_vs             = vec![
        affix_0_v.trim(),
        affix_1_v.trim(),
        affix_2_v.trim()
      ];
      let affix_vs:  Vec<&str> = affix_v_vs.into_iter().filter(|affix_v| {
        value.valid_affix(affix_v)
      }).collect();

      let mut valid_affixes: HashSet<&str> = HashSet::new();
      let mut valid_sources: HashSet<&str> = HashSet::new();

      for affix in affix_vs {
        if value.valid_affix(affix) {
          valid_affixes.insert(affix);
        }
      }

      for line_s_en in &lines_s_en {
        let values_s_en:   Vec<&str> = line_s_en.split("\t").collect();
        let values_s_en_v: &str      = values_s_en[0];
        let values_s_en_s: &str      = values_s_en[2];
        if values_s_en_v == value.value {
          if value.valid_source(values_s_en_s) {
            valid_sources.insert(values_s_en_s);
          }
        }
      }

      for line_s_es in &lines_s_es {
        let values_s_es:   Vec<&str> = line_s_es.split("\t").collect();
        let values_s_es_v: &str      = values_s_es[0];
        let values_s_es_s: &str      = values_s_es[2];
        if values_s_es_v == value.value {
          if value.valid_source(values_s_es_s) {
            valid_sources.insert(values_s_es_s);
          }
        }
      }

      for line_s_hi in &lines_s_hi {
        let values_s_hi:   Vec<&str> = line_s_hi.split("\t").collect();
        let values_s_hi_v: &str      = values_s_hi[0];
        let values_s_hi_s: &str      = values_s_hi[2];
        if values_s_hi_v == value.value {
          if value.valid_source(values_s_hi_s) {
            valid_sources.insert(values_s_hi_s);
          }
        }
      }

      for line_s_ru in &lines_s_ru {
        let values_s_ru:   Vec<&str> = line_s_ru.split("\t").collect();
        let values_s_ru_v: &str      = values_s_ru[0];
        let values_s_ru_s: &str      = values_s_ru[2];
        if values_s_ru_v == value.value {
          if value.valid_source(values_s_ru_s) {
            valid_sources.insert(values_s_ru_s);
          }
        }
      }

      for line_s_zh in &lines_s_zh {
        let values_s_zh:   Vec<&str> = line_s_zh.split("\t").collect();
        let values_s_zh_v: &str      = values_s_zh[0];
        let values_s_zh_s: &str      = values_s_zh[2];
        if values_s_zh_v == value.value {
          if value.valid_source(values_s_zh_s) {
            valid_sources.insert(values_s_zh_s);
          }
        }
      }

      println!("{}", json!({
        "value":    value.value,
        "affixes":  valid_affixes,
        "sources":  valid_sources,
      }));
    }
  }
}
