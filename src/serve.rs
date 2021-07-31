// This file contains the universal logic for a serving process, regardless of framework

use std::fs;
use serde::{Serialize, Deserialize};
use crate::errors::*;
use crate::render_cfg::{RenderCfg, RenderOpt};
use crate::config_manager::ConfigManager;
use crate::template::TemplateMap;
use sycamore::prelude::SsrNode;

/// Represents the data necessary to render a page.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PageData {
    /// Prerendered HTML content.
    pub content: String,
    /// The state for hydration. This is kept as a string for ease of typing. Some pages may not need state or generate it in another way,
    /// so this might be `None`.
    pub state: Option<String>
}

/// Gets the configuration of how to render each page.
pub fn get_render_cfg() -> Result<RenderCfg> {
    let content = fs::read_to_string("../app/dist/render_conf.json")?;
    let cfg = serde_json::from_str::<RenderCfg>(&content)?;

    Ok(cfg)
}

/// Gets the HTML/JSON data for the given page path. This will call SSG/SSR/etc., whatever is needed for that page.
// TODO let this function take a request struct of some form
pub fn get_page(
    path: &str,
    render_cfg: &RenderCfg,
    templates: &TemplateMap<SsrNode>,
    config_manager: &impl ConfigManager
) -> Result<PageData> {
    // Remove `/` from the path by encoding it as a URL (that's what we store)
    let path_encoded = urlencoding::encode(path).to_string();

    // Match the path to one of the templates
    let mut template_name = String::new();
    // We'll try a direct match first
    if let Some(template_root_path) = render_cfg.pages.get(path) {
        template_name = template_root_path.to_string();
    }
    // Next, an ISR match (more complex)
    // We progressively look for more and more specificity of the path, adding each segment
    // That way, we're searching forwards rather than backwards, which is more efficient
    let path_segments: Vec<&str> = path.split('/').collect();
    for (idx, _) in path_segments.iter().enumerate() {
        // Make a path out of this and all the previous segments
        // For some reason, [0..0] gives nothing, so we need to `match` here
        let path_to_try = match idx {
            0 => path_segments[0].to_string(),
            _ => path_segments[0..idx].join("/")
        } + "/*";

        // If we find something, keep going until we don't (maximise specificity)
        if let Some(template_root_path) = render_cfg.pages.get(&path_to_try) {
            template_name = template_root_path.to_string();
        } else {
            break;
        }
    }
    if template_name.is_empty() {
        bail!(ErrorKind::PageNotFound(path.to_string()))
    }

    // Get the render options of the template
    let render_opts = render_cfg.templates.get(&template_name);
    let render_opts = match render_opts {
        Some(render_opts) => render_opts,
        None => bail!(ErrorKind::PageNotFound(path.to_string()))
    };

    let html: String;
    let state: Option<String>;

    // Handle each different type of rendering (static paths have already been done though, so we don't need to deal with them)
    if render_opts.contains(&RenderOpt::StaticProps) {
        // Get the static HTML
        html = config_manager.read(&format!("../app/dist/static/{}.html", path_encoded))?;
        // Get the static JSON
        state = match config_manager.read(&format!("../app/dist/static/{}.json", path_encoded)) {
            Ok(state) => Some(state),
            Err(_) => None
        };
    }  else {
        bail!(ErrorKind::NoRenderOpts(template_name));
    }
    // TODO support SSR, revalidation, and ISR

    // Combine everything into one JSON object
    let res = PageData {
        content: html,
        state,
    };

    Ok(res)
}
