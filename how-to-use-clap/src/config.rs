use anyhow::Result;
use config::File;
use serde::{Deserialize, Serialize};

/// 链配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainConfig {
    pub name: String,
    pub rpc_url: String,
    pub chain_id: u32,
    pub confirmations: u32,
}

/// 消息队列配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageQueueConfig {
    pub broker: String,
    pub topic: String,
}

/// 应用配置，从 YAML 文件加载
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub db_path: String,
    pub chains: Vec<ChainConfig>,
    pub network: String,
    pub api_version: u32,
    pub mq_config: Option<MessageQueueConfig>,

    #[serde(default = "default_true")]
    pub enable_sync: bool,
    #[serde(default = "default_true")]
    pub enable_api: bool,
    #[serde(default = "default_true")]
    pub enable_monitor: bool,

    #[serde(skip)]
    pub config_dir: String,
}

fn default_true() -> bool {
    true
}

#[derive(Deserialize)]
struct ConfigWithIncludes {
    #[serde(default)]
    includes: Vec<String>,
}

impl AppConfig {
    /// 从 YAML 文件加载配置
    ///
    /// 1. 自动补 .yml 后缀
    /// 2. 检查是否有 includes，先加载被引用的文件
    /// 3. 再加载主配置文件（后加载的覆盖先加载的）
    /// 4. 记录 config_dir 方便后续引用相对路径
    pub fn load_from_yaml(path: &str) -> Result<Self> {
        let path_with_yml = if path.ends_with(".yml") {
            path.to_string()
        } else {
            format!("{}.yml", path)
        };

        let config_path = std::path::Path::new(&path_with_yml);
        let config_dir = config_path
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."));

        let yaml_content = std::fs::read_to_string(&path_with_yml)?;
        let include_check: ConfigWithIncludes =
            serde_yaml::from_str(&yaml_content).unwrap_or(ConfigWithIncludes { includes: vec![] });

        let mut builder = config::Config::builder();

        for include_file in &include_check.includes {
            let include_path = config_dir.join(include_file);
            let include_path_str = include_path
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("invalid include path: {:?}", include_path))?;
            let include_path_without_ext = include_path_str.trim_end_matches(".yml");
            builder = builder
                .add_source(File::with_name(include_path_without_ext).format(config::FileFormat::Yaml));
        }

        builder = builder.add_source(File::with_name(path).format(config::FileFormat::Yaml));

        let settings = builder.build()?;
        let mut app_config: AppConfig = settings.try_deserialize()?;
        app_config.config_dir = config_dir.to_string_lossy().to_string();

        Ok(app_config)
    }
}
