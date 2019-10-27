mod root_word;

use std::fs;
use std::collections::HashSet;

use root_word::RootWord;
use root_word::value::Value;

fn main() {
  let cultural_values: Vec<&str> = vec![
    "baxso","bengo","bemro","bindo","brazo","brito","budjo","centi","cmavo",
    "dadjo","decti","dekto","delno","dotco","dzipo","femti","filso","fraso",
    "friko","gento","gigdo","glico","jegvo","jerxo","jordo","jungo","kadno",
    "kelvo","ketco","kilto","kisto","latmo","libjo","lojbo","lubno","lujvo",
    "megdo","mekso","meljo","merko","mexco","mikri","milti","misro","molro",
    "morko","muslo","nanvi","petso","picti","polno","ponjo","porto","radno",
    "rakso","ropno","rusko","sadjo","semto","sinso","sirxo","skoto","softo",
    "spano","sralo","srito","stero","tanjo","terto","xampo","xatsi","xazdo",
    "xebro","xecto","xelso","xexso","xindo","xispo","xrabo","xriso","xurdo",
    "mexno","slovo","vukro",
    "zepti","zetro","gocti","gotro",
    "broda","brode","brodi","brodo","brodu",
    "sorgu",
    "gismu","tanru","rafsi","sumti", //"cmavo","lujvo","bridi"
    "traji"
  ];
  let cultural: Vec<String> = cultural_values.into_iter().map(|value| {
    value.to_string()
  }).collect();

  let data_r:     String
    = fs::read_to_string("data/gismu.txt").expect("err");
  let data_m:     String
    = fs::read_to_string("data/simple_gismu.txt").expect("err");
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
  let head_m:     Vec<&str> = data_m.split("\n").collect();
  let head_s_en:  Vec<&str> = data_s_en.split("\n").collect();
  let head_s_es:  Vec<&str> = data_s_es.split("\n").collect();
  let head_s_hi:  Vec<&str> = data_s_hi.split("\n").collect();
  let head_s_ru:  Vec<&str> = data_s_ru.split("\n").collect();
  let head_s_zh:  Vec<&str> = data_s_zh.split("\n").collect();
  let tail_r:     usize     = head_r.len() - 1;
  let tail_m:     usize     = head_m.len() - 1;
  let tail_s_en:  usize     = head_s_en.len() - 1;
  let tail_s_es:  usize     = head_s_es.len() - 1;
  let tail_s_hi:  usize     = head_s_hi.len() - 1;
  let tail_s_ru:  usize     = head_s_ru.len() - 1;
  let tail_s_zh:  usize     = head_s_zh.len() - 1;
  let lines_r:    Vec<&str> = head_r.into_iter().take(tail_r).collect();
  let lines_m:    Vec<&str> = head_m.into_iter().take(tail_m).collect();
  let lines_s_en: Vec<&str> = head_s_en.into_iter().take(tail_s_en).collect();
  let lines_s_es: Vec<&str> = head_s_es.into_iter().take(tail_s_es).collect();
  let lines_s_hi: Vec<&str> = head_s_hi.into_iter().take(tail_s_hi).collect();
  let lines_s_ru: Vec<&str> = head_s_ru.into_iter().take(tail_s_ru).collect();
  let lines_s_zh: Vec<&str> = head_s_zh.into_iter().take(tail_s_zh).collect();

  for line_r in lines_r {
    let value_r:   String        = line_r.chars().skip(1 ).take(5).collect();
    let value_o:   Option<Value> = Value::new(value_r);

    if value_o.is_some() {
      let value = value_o.unwrap();
      if ! cultural.contains(&value.value) {
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

        let mut affixes: HashSet<&str>     = HashSet::new();
        let mut sources: HashSet<[&str;3]> = HashSet::new();

        for affix in affix_vs {
          affixes.insert(affix);
        }

        let line_m_o: Option<&&str> = lines_m.iter().find(|&line| {
          let line_v: String = line.chars().take(5).collect();
          line_v == value.value
        });

        let meaning: String = match line_m_o {
          Some(line)  => line.chars().skip(6).collect(),
          None        => String::from("")
        };

        let lines_s_langs = vec![
          ("en", &lines_s_en),
          ("es", &lines_s_es),
          ("hi", &lines_s_hi),
          ("ru", &lines_s_ru),
          ("zh", &lines_s_zh)
        ];

        for (lang, lines_s) in lines_s_langs {
          for line_s in lines_s {
            let values_s:   Vec<&str> = line_s.split("\t").collect();
            let values_s_r: &str      = values_s[0];
            if values_s_r == value.value {
              let values_s_v: &str = values_s[2];
              let values_s_s: &str = values_s[3];
              sources.insert([values_s_v, values_s_s, lang]);
            }
          }
        }

        let root_word = RootWord::new(
          value,
          meaning,
          affixes,
          sources
        );

        println!("{}", root_word);
      }
    }
  }
}
