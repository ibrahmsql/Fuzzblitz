pub struct TakeoverDetector;

impl TakeoverDetector {
    pub fn vulnerable_cnames() -> Vec<(&'static str, &'static str)> {
        vec![
            ("amazonaws.com", "NoSuchBucket"),
            ("herokuapp.com", "No such app"),
            ("github.io", "There isn't a GitHub Pages site here"),
            ("shopify.com", "Sorry, this shop is currently unavailable"),
            ("tumblr.com", "Whatever you were looking for doesn't currently exist"),
            ("wordpress.com", "Do you want to register"),
            ("ghost.io", "The thing you were looking for is no longer here"),
            ("bitbucket.io", "Repository not found"),
            ("azure.com", "404 Web Site not found"),
            ("zendesk.com", "Help Center Closed"),
        ]
    }
    
    pub fn check_indicators(body: &str, cname: &str) -> bool {
        for (service, indicator) in Self::vulnerable_cnames() {
            if cname.contains(service) && body.contains(indicator) {
                return true;
            }
        }
        false
    }
}
