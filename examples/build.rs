use const_gen::*;
use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};
use std::{env, fs, path::Path};
use tokio::runtime::Runtime;
use viam::gen::proto::app::v1::{
    robot_service_client::RobotServiceClient, AgentInfo, CertificateRequest, ConfigRequest,
    RobotConfig,
};
use viam_rust_utils::rpc::dial::{DialOptions, RPCCredentials};

struct ComponentConfig(viam::gen::proto::app::v1::ComponentConfig);
struct Attributes(prost_types::Struct);
struct Kind(prost_types::value::Kind);
struct StaticRobotConfig {
    components: Vec<ComponentConfig>,
}

impl const_gen::CompileConst for StaticRobotConfig {
    fn const_type() -> String {
        String::from("RobotConfigStatic")
    }
    fn const_val(&self) -> String {
        let mut obj = String::new();
        if self.components.len() > 0 {
            obj.push_str(&format!("Some({})", self.components.const_val()));
        } else {
            obj.push_str("None");
        }
        format!("RobotConfigStatic {{components: {}}}", obj)
    }
}

impl const_gen::CompileConst for Kind {
    fn const_type() -> String {
        String::from("Kind")
    }
    fn const_val(&self) -> String {
        let mut obj = String::new();
        match self.0.clone() {
            prost_types::value::Kind::NumberValue(v) => {
                obj.push_str(&format!("NumberValue({})", v.const_val()));
            }
            prost_types::value::Kind::NullValue(v) => {
                obj.push_str(&format!("NullValue({})", v.const_val()));
            }
            prost_types::value::Kind::StringValue(v) => {
                obj.push_str(&format!("StringValueStatic({})", v.const_val()));
            }
            prost_types::value::Kind::ListValue(v) => {
                obj.push_str(&format!(
                    "ListValueStatic({})",
                    v.values
                        .into_iter()
                        .filter(|a| a.kind.is_some())
                        .map(|a| Kind(a.kind.unwrap()))
                        .collect::<Vec<Kind>>()
                        .const_val()
                ));
            }
            prost_types::value::Kind::StructValue(v) => {
                obj.push_str(&format!("StructValueStatic({})", Attributes(v).const_val()));
            }
            prost_types::value::Kind::BoolValue(v) => {
                obj.push_str(&format!("BoolValue({})", v.const_val()));
            }
        }
        format!("Kind::{}", obj)
    }
}

impl const_gen::CompileConst for ComponentConfig {
    fn const_type() -> String {
        String::from("StaticComponentConfig")
    }
    fn const_val(&self) -> String {
        let mut obj = String::new();
        obj.push_str(&format!("name: {},", &self.0.name.const_val()));
        obj.push_str(&format!("namespace: {},", self.0.namespace.const_val()));
        obj.push_str(&format!("r#type: {},", self.0.r#type.const_val()));
        obj.push_str(&format!("model: {},", self.0.model.const_val()));
        match self.0.attributes.clone() {
            Some(attrs) => obj.push_str(&format!(
                "attributes: Some({}),",
                Attributes(attrs).const_val()
            )),
            None => obj.push_str("None"),
        };
        format!("StaticComponentConfig {{{}}}", obj)
    }
}

impl const_gen::CompileConst for Attributes {
    fn const_type() -> String {
        format!("phf::Map<{}, {}>", "&'static str", "Kind")
    }
    fn const_val(&self) -> String {
        self.0
            .fields
            .clone()
            .into_iter()
            .filter(|(_, v)| v.kind.is_some())
            .map(|(k, v)| (k, Kind(v.kind.unwrap())))
            .collect::<std::collections::HashMap<_, _>>()
            .const_val()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub cloud: Cloud,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Cloud {
    pub id: String,
    pub secret: String,
    #[serde(default)]
    pub location_secret: String,
    #[serde(default)]
    managed_by: String,
    #[serde(default)]
    fqdn: String,
    #[serde(default)]
    local_fqdn: String,
    #[serde(default)]
    signaling_address: String,
    #[serde(default)]
    signaling_insecure: bool,
    #[serde(default)]
    path: String,
    #[serde(default)]
    log_path: String,
    app_address: String,
    #[serde(default)]
    refresh_interval: String,
    #[serde(default)]
    pub tls_certificate: String,
    #[serde(default)]
    pub tls_private_key: String,
}

fn main() -> anyhow::Result<()> {
    if env::var("TARGET").unwrap() == "xtensa-esp32-espidf" {
        if std::env::var_os("IDF_PATH").is_none() {
            return Err(anyhow::anyhow!(
                "You need to run IDF's export.sh before building"
            ));
        }
        if std::env::var_os("MICRO_RDK_WIFI_SSID").is_none() {
            std::env::set_var("MICRO_RDK_WIFI_SSID", "Viam-2G");
            println!("cargo:rustc-env=MICRO_RDK_WIFI_SSID=Viam-2G");
        }
        if std::env::var_os("MICRO_RDK_WIFI_PASSWORD").is_none() {
            return Err(anyhow::anyhow!(
                "please set the password for WiFi {}",
                std::env::var_os("MICRO_RDK_WIFI_PASSWORD")
                    .unwrap()
                    .to_str()
                    .unwrap()
            ));
        }
        embuild::build::LinkArgs::output_propagated("ESP_IDF")?;
    }

    let (robot_cfg, cfg) = if let Ok(content) = std::fs::read_to_string("viam.json") {
        let mut cfg: Config = serde_json::from_str(content.as_str()).map_err(anyhow::Error::msg)?;

        let rt = Runtime::new()?;
        let robot_cfg = rt.block_on(read_cloud_config(&mut cfg))?;
        rt.block_on(read_certificates(&mut cfg))?;
        (robot_cfg, cfg)
    } else {
        (RobotConfig::default(), Config::default())
    };

    let cloud_cfg = robot_cfg.cloud.unwrap_or_default();
    let robot_name = cloud_cfg.local_fqdn.split('.').next().unwrap_or("");
    let local_fqdn = cloud_cfg.local_fqdn.replace('.', "-");
    let fqdn = cloud_cfg.fqdn.replace('.', "-");

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("ca.crt");
    let ca_cert = String::from(&cfg.cloud.tls_certificate) + "\0";
    std::fs::write(dest_path, ca_cert)?;
    let dest_path = std::path::Path::new(&out_dir).join("key.key");
    let key = String::from(&cfg.cloud.tls_private_key) + "\0";
    std::fs::write(dest_path, key)?;

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("robot_secret.rs");
    let robot_decl = vec![
        const_declaration!(
            #[allow(clippy::redundant_static_lifetimes, dead_code)]
            ROBOT_ID = cfg.cloud.id.as_str()
        ),
        const_declaration!(
            #[allow(clippy::redundant_static_lifetimes, dead_code)]
            ROBOT_SECRET = cfg.cloud.secret.as_str()
        ),
        const_declaration!(
            #[allow(clippy::redundant_static_lifetimes, dead_code)]
            LOCAL_FQDN = local_fqdn.as_str()
        ),
        const_declaration!(
            #[allow(clippy::redundant_static_lifetimes, dead_code)]
            FQDN = fqdn.as_str()
        ),
        const_declaration!(
            #[allow(clippy::redundant_static_lifetimes, dead_code)]
            ROBOT_NAME = robot_name
        ),
    ]
    .join("\n");
    fs::write(&dest_path, robot_decl).unwrap();

    let components_config = robot_cfg
        .components
        .into_iter()
        .map(ComponentConfig)
        .collect::<Vec<ComponentConfig>>();
    let robot_config = StaticRobotConfig {
        components: components_config,
    };
    let dest_path = Path::new(&out_dir).join("robot_config.rs");
    let conf_decl = if robot_config.components.len() > 0 {
        vec![const_declaration!(
            #[allow(clippy::redundant_static_lifetimes, dead_code)]
            STATIC_ROBOT_CONFIG = Some(robot_config)
        )]
    } else {
        vec![const_declaration!(
            #[allow(clippy::redundant_static_lifetimes, dead_code)]
            STATIC_ROBOT_CONFIG = None::<StaticRobotConfig>
        )]
    }
    .join("\n");
    fs::write(&dest_path, conf_decl).unwrap();

    Ok(())
}

async fn read_certificates(config: &mut Config) -> anyhow::Result<()> {
    let creds = RPCCredentials::new(
        Some(config.cloud.id.clone()),
        "robot-secret".to_string(),
        config.cloud.secret.clone(),
    );
    let cert_req = CertificateRequest {
        id: config.cloud.id.clone(),
    };
    let dial = DialOptions::builder()
        .uri(config.cloud.app_address.clone().as_str())
        .with_credentials(creds)
        .disable_webrtc()
        .connect()
        .await?;
    let mut app_service = RobotServiceClient::new(dial);
    let certs = app_service.certificate(cert_req).await?.into_inner();
    config.cloud.tls_certificate = certs.tls_certificate;
    config.cloud.tls_private_key = certs.tls_private_key;
    Ok(())
}

async fn read_cloud_config(config: &mut Config) -> anyhow::Result<RobotConfig> {
    let creds = RPCCredentials::new(
        Some(config.cloud.id.clone()),
        "robot-secret".to_string(),
        config.cloud.secret.clone(),
    );
    let agent = AgentInfo {
        os: "esp32-build".to_string(),
        host: gethostname::gethostname().to_str().unwrap().to_string(),
        ips: vec![local_ip().unwrap().to_string()],
        version: "0.0.1".to_string(),
        git_revision: "".to_string(),
    };
    let cfg_req = ConfigRequest {
        agent_info: Some(agent),
        id: config.cloud.id.clone(),
    };
    let dial = DialOptions::builder()
        .uri(config.cloud.app_address.clone().as_str())
        .with_credentials(creds)
        .disable_webrtc()
        .connect()
        .await?;
    let mut app_service = RobotServiceClient::new(dial);
    let cfg = app_service.config(cfg_req).await?.into_inner();
    match cfg.config {
        Some(cfg) => Ok(cfg),
        None => anyhow::bail!("no config for robot"),
    }
}
