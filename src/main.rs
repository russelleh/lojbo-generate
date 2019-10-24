mod root_word;

use std::fs;
use std::collections::HashSet;

use root_word::RootWord;
use root_word::value::Value;

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
        affix_v.len() > 0
      }).collect();

      let lines_s_langs = vec![
        &lines_s_en,
        &lines_s_es,
        &lines_s_hi,
        &lines_s_ru,
        &lines_s_zh,
      ];

      let mut affixes: HashSet<&str> = HashSet::new();
      let mut sources: HashSet<&str> = HashSet::new();

      for affix in affix_vs {
        affixes.insert(affix);
      }

      for lines_s in lines_s_langs {
        for line_s in lines_s {
          let values_s:   Vec<&str> = line_s.split("\t").collect();
          let values_s_v: &str      = values_s[0];
          if values_s_v == value.value {
            let values_s_s: &str = values_s[2];
            sources.insert(values_s_s);
          }
        }
      }

      let root_word = RootWord::new(
        value,
        affixes,
        sources
      );

      println!("{}", root_word);
    }
  }
}
