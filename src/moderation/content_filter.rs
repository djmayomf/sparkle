use rust_bert::pipelines::text_classification::{TextClassificationModel, TextClassificationConfig};
use whatlang::{Lang, Script, detect};
use rust_bert::RustBertError;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;

lazy_static! {
    static ref ML_MODEL: RwLock<TextClassificationModel> = RwLock::new(
        TextClassificationModel::new(TextClassificationConfig::default())
            .expect("Failed to load ML model")
    );

    // Language-specific inappropriate patterns
    static ref INAPPROPRIATE_PATTERNS: HashMap<Lang, HashSet<String>> = {
        let mut map = HashMap::new();
        
        // English patterns
        map.insert(Lang::Eng, {
            let mut set = HashSet::new();
            set.insert("nsfw".to_string());
            set.insert("adult content".to_string());
            set.extend(load_wordlist("en/inappropriate.txt"));
            set
        });
        
        // Japanese patterns
        map.insert(Lang::Jpn, {
            let mut set = HashSet::new();
            set.insert("アダルト".to_string());
            set.insert("エッチ".to_string());
            set.extend(load_wordlist("jp/inappropriate.txt"));
            set
        });
        
        map
    };

    // Language-specific harmful patterns
    static ref HARMFUL_PATTERNS: HashMap<Lang, HashSet<String>> = {
        let mut map = HashMap::new();
        
        // English harmful patterns
        map.insert(Lang::Eng, {
            let mut set = HashSet::new();
            set.insert("hate speech".to_string());
            set.insert("harassment".to_string());
            set.extend(load_wordlist("en/harmful.txt"));
            set
        });
        
        // Japanese harmful patterns
        map.insert(Lang::Jpn, {
            let mut set = HashSet::new();
            set.insert("ヘイト".to_string());
            set.insert("いじめ".to_string());
            set.extend(load_wordlist("jp/harmful.txt"));
            set
        });
        
        map
    };
}

pub struct ContentFilter {
    ml_confidence_threshold: f32,
    language_detection: bool,
    strict_mode: bool,
}

impl ContentFilter {
    pub fn new() -> Self {
        Self {
            ml_confidence_threshold: 0.85,
            language_detection: true,
            strict_mode: true,
        }
    }

    pub fn with_config(confidence: f32, detect_language: bool, strict: bool) -> Self {
        Self {
            ml_confidence_threshold: confidence,
            language_detection: detect_language,
            strict_mode: strict,
        }
    }

    pub async fn check_content(&self, content: &str) -> Result<ContentCheckResult, FilterError> {
        // Detect language if enabled
        let lang = if self.language_detection {
            detect_language(content)
        } else {
            Lang::Eng // Default to English
        };

        // Run ML classification
        let ml_result = self.run_ml_classification(content).await?;

        // Check against language-specific patterns
        let pattern_matches = self.check_language_patterns(content, lang);

        // Combine results
        let final_result = self.combine_results(ml_result, pattern_matches);

        Ok(final_result)
    }

    async fn run_ml_classification(&self, content: &str) -> Result<MLClassification, FilterError> {
        let model = ML_MODEL.read().await;
        let output = model.predict(&[content])?;

        Ok(MLClassification {
            inappropriate_confidence: output[0].get("inappropriate").unwrap_or(&0.0).clone(),
            harmful_confidence: output[0].get("harmful").unwrap_or(&0.0).clone(),
            safe_confidence: output[0].get("safe").unwrap_or(&0.0).clone(),
        })
    }

    fn check_language_patterns(&self, content: &str, lang: Lang) -> PatternMatches {
        let inappropriate = INAPPROPRIATE_PATTERNS
            .get(&lang)
            .map(|patterns| patterns.iter().any(|p| content.contains(p)))
            .unwrap_or(false);

        let harmful = HARMFUL_PATTERNS
            .get(&lang)
            .map(|patterns| patterns.iter().any(|p| content.contains(p)))
            .unwrap_or(false);

        PatternMatches {
            inappropriate,
            harmful,
        }
    }

    fn combine_results(&self, ml: MLClassification, patterns: PatternMatches) -> ContentCheckResult {
        let is_inappropriate = ml.inappropriate_confidence > self.ml_confidence_threshold || 
                             (self.strict_mode && patterns.inappropriate);
                             
        let is_harmful = ml.harmful_confidence > self.ml_confidence_threshold ||
                        (self.strict_mode && patterns.harmful);

        ContentCheckResult {
            is_inappropriate,
            is_harmful,
            ml_confidence: MLConfidence {
                inappropriate: ml.inappropriate_confidence,
                harmful: ml.harmful_confidence,
                safe: ml.safe_confidence,
            },
            pattern_matches: patterns,
        }
    }
}

#[derive(Debug)]
struct MLClassification {
    inappropriate_confidence: f32,
    harmful_confidence: f32,
    safe_confidence: f32,
}

#[derive(Debug)]
struct PatternMatches {
    inappropriate: bool,
    harmful: bool,
}

#[derive(Debug)]
pub struct ContentCheckResult {
    pub is_inappropriate: bool,
    pub is_harmful: bool,
    pub ml_confidence: MLConfidence,
    pub pattern_matches: PatternMatches,
}

#[derive(Debug)]
pub struct MLConfidence {
    pub inappropriate: f32,
    pub harmful: f32,
    pub safe: f32,
}

#[derive(Debug, thiserror::Error)]
pub enum FilterError {
    #[error("ML model error: {0}")]
    ModelError(#[from] RustBertError),
    #[error("Language detection error")]
    LanguageError,
}

fn detect_language(content: &str) -> Lang {
    detect(content)
        .map(|info| info.lang())
        .unwrap_or(Lang::Eng)
}

fn load_wordlist(path: &str) -> HashSet<String> {
    // Load word list from file
    // This would typically load from a resource file
    HashSet::new() // Placeholder
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_content_filter() {
        let filter = ContentFilter::new();

        // Test safe content
        let safe_result = filter.check_content("Hello world!").await.unwrap();
        assert!(!safe_result.is_inappropriate);
        assert!(!safe_result.is_harmful);

        // Test inappropriate content
        let inappropriate_result = filter.check_content("nsfw content").await.unwrap();
        assert!(inappropriate_result.is_inappropriate);

        // Test harmful content
        let harmful_result = filter.check_content("hate speech").await.unwrap();
        assert!(harmful_result.is_harmful);

        // Test Japanese content
        let jp_result = filter.check_content("こんにちは").await.unwrap();
        assert!(!jp_result.is_inappropriate);
        assert!(!jp_result.is_harmful);
    }
} 