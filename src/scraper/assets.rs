use regex::Regex;

/// Extract assets from HTML
pub struct AssetExtractor;

impl AssetExtractor {
    /// Extract all JavaScript files
    pub fn extract_js_files(html: &str) -> Vec<String> {
        let re = Regex::new(r#"<script[^>]*src="([^"]+\.js[^"]*)""#).unwrap();
        
        re.captures_iter(html)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .collect()
    }
    
    /// Extract all CSS files
    pub fn extract_css_files(html: &str) -> Vec<String> {
        let re = Regex::new(r#"<link[^>]*href="([^"]+\.css[^"]*)""#).unwrap();
        
        re.captures_iter(html)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .collect()
    }
    
    /// Extract all image files
    pub fn extract_image_files(html: &str) -> Vec<String> {
        let re = Regex::new(r#"<img[^>]*src="([^"]+\.(jpg|jpeg|png|gif|svg|webp|ico)[^"]*)""#).unwrap();
        
        re.captures_iter(html)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .collect()
    }
    
    /// Extract all font files
    pub fn extract_font_files(html: &str) -> Vec<String> {
        let re = Regex::new(r#"url\(["']?([^"']+\.(woff|woff2|ttf|eot|otf)[^"']*)["']?\)"#).unwrap();
        
        re.captures_iter(html)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .collect()
    }
    
    /// Extract all video files
    pub fn extract_video_files(html: &str) -> Vec<String> {
        let re = Regex::new(r#"<(video|source)[^>]*src="([^"]+\.(mp4|webm|ogg)[^"]*)""#).unwrap();
        
        re.captures_iter(html)
            .filter_map(|cap| cap.get(2))
            .map(|m| m.as_str().to_string())
            .collect()
    }
    
    /// Extract all assets
    pub fn extract_all_assets(html: &str) -> Vec<String> {
        let mut assets = Vec::new();
        assets.extend(Self::extract_js_files(html));
        assets.extend(Self::extract_css_files(html));
        assets.extend(Self::extract_image_files(html));
        assets.extend(Self::extract_font_files(html));
        assets.extend(Self::extract_video_files(html));
        assets
    }
}
