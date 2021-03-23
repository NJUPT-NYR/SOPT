use strsim::jaro_winkler;
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    pub static ref TORRENT_SEARCH_ENGINE: RwLock<SearchEngine> = RwLock::new(SearchEngine::new(0.8));
}

pub struct SearchEngine {
    threshold: f64,
    forward: HashMap<i64, Vec<String>>,
    reverse: HashMap<String, Vec<i64>>,
}

impl SearchEngine {
    pub fn new(threshold: f64) -> Self {
        SearchEngine {
            threshold,
            forward: HashMap::new(),
            reverse: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: i64, tokens: Vec<String>) {
        self.delete(id);
        let tokens = Self::tokenize(tokens);
        for token in tokens.clone() {
            self.reverse.entry(token).or_insert_with(|| Vec::with_capacity(1)).push(id);
        }
        self.forward.insert(id, tokens);
    }

    pub fn search(&self, patterns: Vec<String>) -> Vec<i64> {
        // TODO: show suggestions on typo
        let patterns = Self::tokenize(patterns);
        let mut scores: HashMap<&str, f64> = HashMap::new();
        for pattern in patterns {
            for token in self.reverse.keys() {
                let score = jaro_winkler(&pattern, token);
                if score > self.threshold {
                    scores.insert(token, score);
                }
            }
        }

        let mut ret: HashMap<i64, f64> = HashMap::new();
        for (token, score) in scores.drain() {
            for id in &self.reverse[token] {
                *ret.entry(*id).or_insert(0.) += score;
            }
        }

        let mut ret: Vec<(i64, f64)> = ret.drain().collect();
        ret.sort_by(|lhs, rhs| rhs.1.partial_cmp(&lhs.1).unwrap());
        let result_ids = ret.iter()
            .map(|(id, _)| *id )
            .collect();

        result_ids
    }

    pub fn delete(&mut self, id: i64) {
        for token in self.forward.entry(id).or_default() {
            self.reverse.get_mut(token).unwrap().retain(|i| *i != id);
        }
        self.forward.remove(&id);
    }

    fn tokenize(tokens: Vec<String>) -> Vec<String> {
        let mut ret: Vec<String> = tokens.iter()
            .flat_map(|token| token.split_ascii_whitespace())
            .map(|token| token.to_string()).collect();

        ret = ret.iter()
            .flat_map(|token| token.split_terminator("["))
            .flat_map(|token| token.split_terminator("]"))
            .map(|token| token.to_string())
            .collect();

        ret.retain(|token| !token.is_empty());
        ret.sort();
        ret.dedup();
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_exact_keyword_works() {
        let mut engine = SearchEngine::new(0.8);
        engine.insert(1, vec!["[喵萌奶茶屋][回转企鹅罐 Mawaru Penguindrum][BDRIP][720P][X264-10bit_AAC]".to_string()]);
        engine.insert(2, vec!["[MagicStar][回转企鹅罐 Mawaru Penguindrum][BDRIP][720P][X264-10bit_AAC]".to_string()]);
        engine.insert(3, vec!["[桜都字幕组][回转企鹅罐 Mawaru Penguindrum][BDRIP][720P][X264-10bit_AAC]".to_string()]);

        let ids = engine.search(vec!["桜都字幕组".to_string()]);
        assert_eq!(ids[0], 3);
    }

    #[test]
    fn search_typo_keyword_works() {
        let mut engine = SearchEngine::new(0.8);
        engine.insert(1, vec!["[喵萌奶茶屋][回转企鹅罐 Mawaru Penguindrum][BDRIP][720P][X264-10bit_AAC]".to_string()]);

        let ids = engine.search(vec!["mawru".to_string(), "BD".to_string()]);
        assert_ne!(ids.is_empty(), true);
    }

    #[test]
    fn multiple_insert_works() {
        let mut engine = SearchEngine::new(0.8);
        engine.insert(1, vec!["[喵萌奶茶屋][回转企鹅罐 Mawaru Penguindrum][BDRIP][720P][X264-10bit_AAC]".to_string()]);
        engine.insert(1, vec!["[桜都字幕组][回转企鹅罐 Mawaru Penguindrum][BDRIP][720P][X264-10bit_AAC]".to_string()]);

        let ids = engine.search(vec!["喵萌奶茶屋".to_string()]);
        assert_eq!(ids.is_empty(), true);
    }
}
