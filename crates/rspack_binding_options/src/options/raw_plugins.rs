use crate::RawOption;
use rspack_core::{CompilerOptionsBuilder, Plugins};

pub type RawPlugins = serde_json::value::Value;

impl RawOption<Plugins> for RawPlugins {
  fn to_compiler_option(self, _options: &CompilerOptionsBuilder) -> anyhow::Result<Plugins> {
    let result: Plugins = vec![];

    Ok(result)
    // if self.is_null() {
    //   return Ok(result);
    // }

    // let plugins = &self;

    // if let Some(plugins) = plugins.as_array() {
    //   for (i, plugin) in plugins.iter().enumerate() {
    //     let (target, config) = if let Some(name) = plugin.as_str() {
    //       (Some(name.to_ascii_lowercase()), None)
    //     } else if let Some(name_with_config) = plugin.as_array() {
    //       (
    //         name_with_config
    //           .get(0)
    //           .and_then(|f| f.as_str())
    //           .map(|f| f.to_ascii_lowercase()),
    //         name_with_config.get(1),
    //       )
    //     } else {
    //       return Err(anyhow::format_err!(
    //         "`config.plugins[{i}]`: structure is not recognized."
    //       ));
    //     };
    //     match target.as_deref() {
    //       Some("html") => {
    //         let config: HtmlPluginConfig = match config {
    //           Some(config) => serde_json::from_value::<HtmlPluginConfig>(config.clone())?,
    //           None => Default::default(),
    //         };
    //         result.push(Box::new(rspack_plugin_html::HtmlPlugin::new(config)));
    //       }
    //       _ => {
    //         return Err(anyhow::format_err!(
    //           "`config.plugins[{i}]`: plugin is not found."
    //         ));
    //       }
    //     };
    //   }
    // } else {
    //   return Err(anyhow::format_err!(
    //     "`config.plugins`: structure is not recognized. Found `{:?}`",
    //     plugins
    //   ));
    // }
    // Ok(result)
  }

  fn fallback_value(_options: &CompilerOptionsBuilder) -> Self {
    serde_json::Value::default()
  }
}