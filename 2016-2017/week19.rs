extern crate reqwest;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

use std::collections::{HashMap, BTreeMap};

#[derive(Serialize, Deserialize)]
struct Data {
    data: Vec<Entry>,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    week: u32,
    language: String,
}

fn main() {
    println!("{}", &serde_json::to_string(
        &reqwest::get("http://potw.quinnftw.com/api/solution_languages")
            .and_then(|mut resp| resp.json::<Data>())
            .map(|data| data.data)
            .map(|entries| {
                let mut g_avg = HashMap::new();
                let g_total = entries.len() as f32;

                entries.into_iter()
                    .fold(BTreeMap::new(), |mut l_avg, entry| {
                        *g_avg.entry(entry.language.clone())
                            .or_insert(0.0) += 1.0 / g_total;

                        {
                            let &mut (ref mut l_total, ref mut l_langs) = l_avg.entry(entry.week)
                                .or_insert((0.0, HashMap::new()));

                            *l_total += 1.0;
                            *l_langs.entry(entry.language)
                                .or_insert(0.0f32) += 1.0;
                        }

                        l_avg
                    })
                    .into_iter()
                    .map(|(week, (l_total, langs))| {
                        langs.into_iter()
                            .max_by(|&(ref l1, ref c1), &(ref l2, ref c2)| {
                                let avg1 = (c1 / l_total) / g_avg[l1];
                                let avg2 = (c2 / l_total) / g_avg[l2];
                                avg1.partial_cmp(&avg2).unwrap()
                            })
                            .map(|(lang, _)| Entry { week: week, language: lang })
                            .unwrap()

                    })
                    .collect::<Vec<_>>()
            })
            .map(|trending| Data { data: trending })
            .expect("Error expecting response data.")
        )
        .expect("Error converting JSON."));
}