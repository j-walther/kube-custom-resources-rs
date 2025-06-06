// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/apecloud/kubeblocks/apps.kubeblocks.io/v1alpha1/configconstraints.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
}
use self::prelude::*;

/// ConfigConstraintSpec defines the desired state of ConfigConstraint
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "apps.kubeblocks.io", version = "v1alpha1", kind = "ConfigConstraint", plural = "configconstraints")]
#[kube(status = "ConfigConstraintStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct ConfigConstraintSpec {
    /// Specifies the top-level key in the 'configurationSchema.cue' that organizes the validation rules for parameters.
    /// This key must exist within the CUE script defined in 'configurationSchema.cue'.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cfgSchemaTopLevelName")]
    pub cfg_schema_top_level_name: Option<String>,
    /// Defines a list of parameters including their names, default values, descriptions,
    /// types, and constraints (permissible values or the range of valid values).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configurationSchema")]
    pub configuration_schema: Option<ConfigConstraintConfigurationSchema>,
    /// Specifies a list of actions to execute specified commands based on Pod labels.
    /// 
    /// 
    /// It utilizes the K8s Downward API to mount label information as a volume into the pod.
    /// The 'config-manager' sidecar container watches for changes in the role label and dynamically invoke
    /// registered commands (usually execute some SQL statements) when a change is detected.
    /// 
    /// 
    /// It is designed for scenarios where:
    /// 
    /// 
    /// - Replicas with different roles have different configurations, such as Redis primary & secondary replicas.
    /// - After a role switch (e.g., from secondary to primary), some changes in configuration are needed
    ///   to reflect the new role.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "downwardAPIOptions")]
    pub downward_api_options: Option<Vec<ConfigConstraintDownwardApiOptions>>,
    /// Indicates whether to consolidate dynamic reload and restart actions into a single restart.
    /// 
    /// 
    /// - If true, updates requiring both actions will result in only a restart, merging the actions.
    /// - If false, updates will trigger both actions executed sequentially: first dynamic reload, then restart.
    /// 
    /// 
    /// This flag allows for more efficient handling of configuration changes by potentially eliminating
    /// an unnecessary reload step.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dynamicActionCanBeMerged")]
    pub dynamic_action_can_be_merged: Option<bool>,
    /// List dynamic parameters.
    /// Modifications to these parameters trigger a configuration reload without requiring a process restart.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dynamicParameters")]
    pub dynamic_parameters: Option<Vec<String>>,
    /// Specifies the format of the configuration file and any associated parameters that are specific to the chosen format.
    /// Supported formats include `ini`, `xml`, `yaml`, `json`, `hcl`, `dotenv`, `properties`, and `toml`.
    /// 
    /// 
    /// Each format may have its own set of parameters that can be configured.
    /// For instance, when using the `ini` format, you can specify the section name.
    /// 
    /// 
    /// Example:
    /// ```text
    /// formatterConfig:
    ///  format: ini
    ///  iniConfig:
    ///    sectionName: mysqld
    /// ```
    #[serde(rename = "formatterConfig")]
    pub formatter_config: ConfigConstraintFormatterConfig,
    /// Lists the parameters that cannot be modified once set.
    /// Attempting to change any of these parameters will be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "immutableParameters")]
    pub immutable_parameters: Option<Vec<String>>,
    /// Specifies the dynamic reload action supported by the engine.
    /// When set, the controller executes the method defined here to execute hot parameter updates.
    /// 
    /// 
    /// Dynamic reloading is triggered only if both of the following conditions are met:
    /// 
    /// 
    /// 1. The modified parameters are listed in the `dynamicParameters` field.
    ///    If `reloadStaticParamsBeforeRestart` is set to true, modifications to `staticParameters`
    ///    can also trigger a reload.
    /// 2. `reloadOptions` is set.
    /// 
    /// 
    /// If `reloadOptions` is not set or the modified parameters are not listed in `dynamicParameters`,
    /// dynamic reloading will not be triggered.
    /// 
    /// 
    /// Example:
    /// ```text
    /// reloadOptions:
    ///  tplScriptTrigger:
    ///    namespace: kb-system
    ///    scriptConfigMapRef: mysql-reload-script
    ///    sync: true
    /// ```
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reloadOptions")]
    pub reload_options: Option<ConfigConstraintReloadOptions>,
    /// Configures whether the dynamic reload specified in `reloadOptions` applies only to dynamic parameters or
    /// to all parameters (including static parameters).
    /// 
    /// 
    /// - false (default): Only modifications to the dynamic parameters listed in `dynamicParameters`
    ///   will trigger a dynamic reload.
    /// - true: Modifications to both dynamic parameters listed in `dynamicParameters` and static parameters
    ///   listed in `staticParameters` will trigger a dynamic reload.
    ///   The "true" option is for certain engines that require static parameters to be set
    ///   via SQL statements before they can take effect on restart.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reloadStaticParamsBeforeRestart")]
    pub reload_static_params_before_restart: Option<bool>,
    /// A list of ScriptConfig Object.
    /// 
    /// 
    /// Each ScriptConfig object specifies a ConfigMap that contains script files that should be mounted inside the pod.
    /// The scripts are mounted as volumes and can be referenced and executed by the dynamic reload
    /// and DownwardAction to perform specific tasks or configurations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scriptConfigs")]
    pub script_configs: Option<Vec<ConfigConstraintScriptConfigs>>,
    /// Used to match labels on the pod to determine whether a dynamic reload should be performed.
    /// 
    /// 
    /// In some scenarios, only specific pods (e.g., primary replicas) need to undergo a dynamic reload.
    /// The `selector` allows you to specify label selectors to target the desired pods for the reload process.
    /// 
    /// 
    /// If the `selector` is not specified or is nil, all pods managed by the workload will be considered for the dynamic
    /// reload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<ConfigConstraintSelector>,
    /// List static parameters.
    /// Modifications to any of these parameters require a restart of the process to take effect.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "staticParameters")]
    pub static_parameters: Option<Vec<String>>,
    /// Specifies the tools container image used by ShellTrigger for dynamic reload.
    /// If the dynamic reload action is triggered by a ShellTrigger, this field is required.
    /// This image must contain all necessary tools for executing the ShellTrigger scripts.
    /// 
    /// 
    /// Usually the specified image is referenced by the init container,
    /// which is then responsible for copy the tools from the image to a bin volume.
    /// This ensures that the tools are available to the 'config-manager' sidecar.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "toolsImageSpec")]
    pub tools_image_spec: Option<ConfigConstraintToolsImageSpec>,
}

/// Defines a list of parameters including their names, default values, descriptions,
/// types, and constraints (permissible values or the range of valid values).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigConstraintConfigurationSchema {
    /// Hold a string that contains a script written in CUE language that defines a list of configuration items.
    /// Each item is detailed with its name, default value, description, type (e.g. string, integer, float),
    /// and constraints (permissible values or the valid range of values).
    /// 
    /// 
    /// CUE (Configure, Unify, Execute) is a declarative language designed for defining and validating
    /// complex data configurations.
    /// It is particularly useful in environments like K8s where complex configurations and validation rules are common.
    /// 
    /// 
    /// This script functions as a validator for user-provided configurations, ensuring compliance with
    /// the established specifications and constraints.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cue: Option<String>,
    /// Generated from the 'cue' field and transformed into a JSON format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<serde_json::Value>,
}

/// DownwardAPIChangeTriggeredAction defines an action that triggers specific commands in response to changes in Pod labels.
/// For example, a command might be executed when the 'role' label of the Pod is updated.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigConstraintDownwardApiOptions {
    /// Specifies the command to be triggered when changes are detected in Downward API volume files.
    /// It relies on the inotify mechanism in the config-manager sidecar to monitor file changes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// Represents a list of files under the Downward API volume.
    pub items: Vec<ConfigConstraintDownwardApiOptionsItems>,
    /// Specifies the mount point of the Downward API volume.
    #[serde(rename = "mountPoint")]
    pub mount_point: String,
    /// Specifies the name of the field. It must be a string of maximum length 63.
    /// The name should match the regex pattern `^[a-z0-9]([a-z0-9\.\-]*[a-z0-9])?$`.
    pub name: String,
    /// ScriptConfig object specifies a ConfigMap that contains script files that should be mounted inside the pod.
    /// The scripts are mounted as volumes and can be referenced and executed by the DownwardAction to perform specific tasks or configurations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scriptConfig")]
    pub script_config: Option<ConfigConstraintDownwardApiOptionsScriptConfig>,
}

/// DownwardAPIVolumeFile represents information to create the file containing the pod field
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigConstraintDownwardApiOptionsItems {
    /// Required: Selects a field of the pod: only annotations, labels, name and namespace are supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldRef")]
    pub field_ref: Option<ConfigConstraintDownwardApiOptionsItemsFieldRef>,
    /// Optional: mode bits used to set permissions on this file, must be an octal value
    /// between 0000 and 0777 or a decimal value between 0 and 511.
    /// YAML accepts both octal and decimal values, JSON requires decimal values for mode bits.
    /// If not specified, the volume defaultMode will be used.
    /// This might be in conflict with other options that affect the file
    /// mode, like fsGroup, and the result can be other mode bits set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
    /// Required: Path is  the relative path name of the file to be created. Must not be absolute or contain the '..' path. Must be utf-8 encoded. The first item of the relative path must not start with '..'
    pub path: String,
    /// Selects a resource of the container: only resources limits and requests
    /// (limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceFieldRef")]
    pub resource_field_ref: Option<ConfigConstraintDownwardApiOptionsItemsResourceFieldRef>,
}

/// Required: Selects a field of the pod: only annotations, labels, name and namespace are supported.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigConstraintDownwardApiOptionsItemsFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Path of the field to select in the specified API version.
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

/// Selects a resource of the container: only resources limits and requests
/// (limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigConstraintDownwardApiOptionsItemsResourceFieldRef {
    /// Container name: required for volumes, optional for env vars
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerName")]
    pub container_name: Option<String>,
    /// Specifies the output format of the exposed resources, defaults to "1"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<IntOrString>,
    /// Required: resource to select
    pub resource: String,
}

/// ScriptConfig object specifies a ConfigMap that contains script files that should be mounted inside the pod.
/// The scripts are mounted as volumes and can be referenced and executed by the DownwardAction to perform specific tasks or configurations.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigConstraintDownwardApiOptionsScriptConfig {
    /// Specifies the namespace for the ConfigMap.
    /// If not specified, it defaults to the "default" namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specifies the reference to the ConfigMap containing the scripts.
    #[serde(rename = "scriptConfigMapRef")]
    pub script_config_map_ref: String,
}

/// Specifies the format of the configuration file and any associated parameters that are specific to the chosen format.
/// Supported formats include `ini`, `xml`, `yaml`, `json`, `hcl`, `dotenv`, `properties`, and `toml`.
/// 
/// 
/// Each format may have its own set of parameters that can be configured.
/// For instance, when using the `ini` format, you can specify the section name.
/// 
/// 
/// Example:
/// ```text
/// formatterConfig:
///  format: ini
///  iniConfig:
///    sectionName: mysqld
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintFormatterConfig {
    /// The config file format. Valid values are `ini`, `xml`, `yaml`, `json`,
    /// `hcl`, `dotenv`, `properties` and `toml`. Each format has its own characteristics and use cases.
    /// 
    /// 
    /// - ini: is a text-based content with a structure and syntax comprising key–value pairs for properties, reference wiki: https://en.wikipedia.org/wiki/INI_file
    /// - xml: refers to wiki: https://en.wikipedia.org/wiki/XML
    /// - yaml: supports for complex data types and structures.
    /// - json: refers to wiki: https://en.wikipedia.org/wiki/JSON
    /// - hcl: The HashiCorp Configuration Language (HCL) is a configuration language authored by HashiCorp, reference url: https://www.linode.com/docs/guides/introduction-to-hcl/
    /// - dotenv: is a plain text file with simple key–value pairs, reference wiki: https://en.wikipedia.org/wiki/Configuration_file#MS-DOS
    /// - properties: a file extension mainly used in Java, reference wiki: https://en.wikipedia.org/wiki/.properties
    /// - toml: refers to wiki: https://en.wikipedia.org/wiki/TOML
    /// - props-plus: a file extension mainly used in Java, supports CamelCase(e.g: brokerMaxConnectionsPerIp)
    pub format: ConfigConstraintFormatterConfigFormat,
    /// Holds options specific to the 'ini' file format.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "iniConfig")]
    pub ini_config: Option<ConfigConstraintFormatterConfigIniConfig>,
}

/// Specifies the format of the configuration file and any associated parameters that are specific to the chosen format.
/// Supported formats include `ini`, `xml`, `yaml`, `json`, `hcl`, `dotenv`, `properties`, and `toml`.
/// 
/// 
/// Each format may have its own set of parameters that can be configured.
/// For instance, when using the `ini` format, you can specify the section name.
/// 
/// 
/// Example:
/// ```text
/// formatterConfig:
///  format: ini
///  iniConfig:
///    sectionName: mysqld
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConfigConstraintFormatterConfigFormat {
    #[serde(rename = "xml")]
    Xml,
    #[serde(rename = "ini")]
    Ini,
    #[serde(rename = "yaml")]
    Yaml,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "hcl")]
    Hcl,
    #[serde(rename = "dotenv")]
    Dotenv,
    #[serde(rename = "toml")]
    Toml,
    #[serde(rename = "properties")]
    Properties,
    #[serde(rename = "redis")]
    Redis,
    #[serde(rename = "props-plus")]
    PropsPlus,
}

/// Holds options specific to the 'ini' file format.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigConstraintFormatterConfigIniConfig {
    /// A string that describes the name of the ini section.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sectionName")]
    pub section_name: Option<String>,
}

/// Specifies the dynamic reload action supported by the engine.
/// When set, the controller executes the method defined here to execute hot parameter updates.
/// 
/// 
/// Dynamic reloading is triggered only if both of the following conditions are met:
/// 
/// 
/// 1. The modified parameters are listed in the `dynamicParameters` field.
///    If `reloadStaticParamsBeforeRestart` is set to true, modifications to `staticParameters`
///    can also trigger a reload.
/// 2. `reloadOptions` is set.
/// 
/// 
/// If `reloadOptions` is not set or the modified parameters are not listed in `dynamicParameters`,
/// dynamic reloading will not be triggered.
/// 
/// 
/// Example:
/// ```text
/// reloadOptions:
///  tplScriptTrigger:
///    namespace: kb-system
///    scriptConfigMapRef: mysql-reload-script
///    sync: true
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigConstraintReloadOptions {
    /// Automatically perform the reload when specified conditions are met.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoTrigger")]
    pub auto_trigger: Option<ConfigConstraintReloadOptionsAutoTrigger>,
    /// Allows to execute a custom shell script to reload the process.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "shellTrigger")]
    pub shell_trigger: Option<ConfigConstraintReloadOptionsShellTrigger>,
    /// Enables reloading process using a Go template script.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tplScriptTrigger")]
    pub tpl_script_trigger: Option<ConfigConstraintReloadOptionsTplScriptTrigger>,
    /// Used to trigger a reload by sending a specific Unix signal to the process.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unixSignalTrigger")]
    pub unix_signal_trigger: Option<ConfigConstraintReloadOptionsUnixSignalTrigger>,
}

/// Automatically perform the reload when specified conditions are met.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigConstraintReloadOptionsAutoTrigger {
    /// The name of the process.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "processName")]
    pub process_name: Option<String>,
}

/// Allows to execute a custom shell script to reload the process.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigConstraintReloadOptionsShellTrigger {
    /// Specifies a Go template string for formatting batch input data.
    /// It's used when `batchReload` is 'True' to format data passed into STDIN of the script.
    /// The template accesses key-value pairs of updated parameters via the '$' variable.
    /// This allows for custom formatting of the input data.
    /// 
    /// 
    /// Example template:
    /// 
    /// 
    /// ```text
    /// batchParamsFormatterTemplate: |-
    /// {{- range $pKey, $pValue := $ }}
    /// {{ printf "%s:%s" $pKey $pValue }}
    /// {{- end }}
    /// ```
    /// 
    /// 
    /// This example generates batch input data in a key:value format, sorted by keys.
    /// ```text
    /// key1:value1
    /// key2:value2
    /// key3:value3
    /// ```
    /// 
    /// 
    /// If not specified, the default format is key=value, sorted by keys, for each updated parameter.
    /// ```text
    /// key1=value1
    /// key2=value2
    /// key3=value3
    /// ```
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "batchParamsFormatterTemplate")]
    pub batch_params_formatter_template: Option<String>,
    /// Controls whether parameter updates are processed individually or collectively in a batch:
    /// 
    /// 
    /// - 'True': Processes all changes in one batch reload.
    /// - 'False': Processes each change individually.
    /// 
    /// 
    /// Defaults to 'False' if unspecified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "batchReload")]
    pub batch_reload: Option<bool>,
    /// Specifies the command to execute in order to reload the process. It should be a valid shell command.
    pub command: Vec<String>,
    /// ScriptConfig object specifies a ConfigMap that contains script files that should be mounted inside the pod.
    /// The scripts are mounted as volumes and can be referenced and executed by the dynamic reload.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scriptConfig")]
    pub script_config: Option<ConfigConstraintReloadOptionsShellTriggerScriptConfig>,
    /// Determines the synchronization mode of parameter updates with "config-manager".
    /// 
    /// 
    /// - 'True': Executes reload actions synchronously, pausing until completion.
    /// - 'False': Executes reload actions asynchronously, without waiting for completion.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync: Option<bool>,
    /// Specifies the tools container image used by ShellTrigger for dynamic reload.
    /// If the dynamic reload action is triggered by a ShellTrigger, this field is required.
    /// This image must contain all necessary tools for executing the ShellTrigger scripts.
    /// 
    /// 
    /// Usually the specified image is referenced by the init container,
    /// which is then responsible for copy the tools from the image to a bin volume.
    /// This ensures that the tools are available to the 'config-manager' sidecar.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "toolsSetup")]
    pub tools_setup: Option<ConfigConstraintReloadOptionsShellTriggerToolsSetup>,
}

/// ScriptConfig object specifies a ConfigMap that contains script files that should be mounted inside the pod.
/// The scripts are mounted as volumes and can be referenced and executed by the dynamic reload.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigConstraintReloadOptionsShellTriggerScriptConfig {
    /// Specifies the namespace for the ConfigMap.
    /// If not specified, it defaults to the "default" namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specifies the reference to the ConfigMap containing the scripts.
    #[serde(rename = "scriptConfigMapRef")]
    pub script_config_map_ref: String,
}

/// Specifies the tools container image used by ShellTrigger for dynamic reload.
/// If the dynamic reload action is triggered by a ShellTrigger, this field is required.
/// This image must contain all necessary tools for executing the ShellTrigger scripts.
/// 
/// 
/// Usually the specified image is referenced by the init container,
/// which is then responsible for copy the tools from the image to a bin volume.
/// This ensures that the tools are available to the 'config-manager' sidecar.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigConstraintReloadOptionsShellTriggerToolsSetup {
    /// Specifies the directory path in the container where the tools-related files are to be copied.
    /// This field is typically used with an emptyDir volume to ensure a temporary, empty directory is provided at pod creation.
    #[serde(rename = "mountPoint")]
    pub mount_point: String,
    /// Specifies a list of settings of init containers that prepare tools for dynamic reload.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "toolConfigs")]
    pub tool_configs: Option<Vec<ConfigConstraintReloadOptionsShellTriggerToolsSetupToolConfigs>>,
}

/// ToolConfig specifies the settings of an init container that prepare tools for dynamic reload.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigConstraintReloadOptionsShellTriggerToolsSetupToolConfigs {
    /// Indicates whether the tool image should be used as the container image for a sidecar.
    /// This is useful for large tool images, such as those for C++ tools, which may depend on
    /// numerous libraries (e.g., *.so files).
    /// 
    /// 
    /// If enabled, the tool image is deployed as a sidecar container image.
    /// 
    /// 
    /// Examples:
    /// ```text
    ///  toolsSetup::
    ///    mountPoint: /kb_tools
    ///    toolConfigs:
    ///      - name: kb-tools
    ///        asContainerImage: true
    ///        image:  apecloud/oceanbase:4.2.0.0-100010032023083021
    /// ```
    /// 
    /// 
    /// generated containers:
    /// ```text
    /// initContainers:
    ///  - name: install-config-manager-tool
    ///    image: apecloud/kubeblocks-tools:${version}
    ///    command:
    ///    - cp
    ///    - /bin/config_render
    ///    - /opt/tools
    ///    volumemounts:
    ///    - name: kb-tools
    ///      mountpath: /opt/tools
    /// 
    /// 
    /// containers:
    ///  - name: config-manager
    ///    image: apecloud/oceanbase:4.2.0.0-100010032023083021
    ///    imagePullPolicy: IfNotPresent
    /// 	  command:
    ///    - /opt/tools/reloader
    ///    - --log-level
    ///    - info
    ///    - --operator-update-enable
    ///    - --tcp
    ///    - "9901"
    ///    - --config
    ///    - /opt/config-manager/config-manager.yaml
    ///    volumemounts:
    ///    - name: kb-tools
    ///      mountpath: /opt/tools
    /// ```
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "asContainerImage")]
    pub as_container_image: Option<bool>,
    /// Specifies the command to be executed by the init container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// Specifies the tool container image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Specifies the name of the init container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Enables reloading process using a Go template script.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigConstraintReloadOptionsTplScriptTrigger {
    /// Specifies the namespace for the ConfigMap.
    /// If not specified, it defaults to the "default" namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specifies the reference to the ConfigMap containing the scripts.
    #[serde(rename = "scriptConfigMapRef")]
    pub script_config_map_ref: String,
    /// Determines whether parameter updates should be synchronized with the "config-manager".
    /// Specifies the controller's reload strategy:
    /// 
    /// 
    /// - If set to 'True', the controller executes the reload action in synchronous mode,
    ///   pausing execution until the reload completes.
    /// - If set to 'False', the controller executes the reload action in asynchronous mode,
    ///   updating the ConfigMap without waiting for the reload process to finish.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync: Option<bool>,
}

/// Used to trigger a reload by sending a specific Unix signal to the process.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigConstraintReloadOptionsUnixSignalTrigger {
    /// Identifies the name of the process to which the Unix signal will be sent.
    #[serde(rename = "processName")]
    pub process_name: String,
    /// Specifies a valid Unix signal to be sent.
    /// For a comprehensive list of all Unix signals, see: ../../pkg/configuration/configmap/handler.go:allUnixSignals
    pub signal: ConfigConstraintReloadOptionsUnixSignalTriggerSignal,
}

/// Used to trigger a reload by sending a specific Unix signal to the process.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConfigConstraintReloadOptionsUnixSignalTriggerSignal {
    #[serde(rename = "SIGHUP")]
    Sighup,
    #[serde(rename = "SIGINT")]
    Sigint,
    #[serde(rename = "SIGQUIT")]
    Sigquit,
    #[serde(rename = "SIGILL")]
    Sigill,
    #[serde(rename = "SIGTRAP")]
    Sigtrap,
    #[serde(rename = "SIGABRT")]
    Sigabrt,
    #[serde(rename = "SIGBUS")]
    Sigbus,
    #[serde(rename = "SIGFPE")]
    Sigfpe,
    #[serde(rename = "SIGKILL")]
    Sigkill,
    #[serde(rename = "SIGUSR1")]
    Sigusr1,
    #[serde(rename = "SIGSEGV")]
    Sigsegv,
    #[serde(rename = "SIGUSR2")]
    Sigusr2,
    #[serde(rename = "SIGPIPE")]
    Sigpipe,
    #[serde(rename = "SIGALRM")]
    Sigalrm,
    #[serde(rename = "SIGTERM")]
    Sigterm,
    #[serde(rename = "SIGSTKFLT")]
    Sigstkflt,
    #[serde(rename = "SIGCHLD")]
    Sigchld,
    #[serde(rename = "SIGCONT")]
    Sigcont,
    #[serde(rename = "SIGSTOP")]
    Sigstop,
    #[serde(rename = "SIGTSTP")]
    Sigtstp,
    #[serde(rename = "SIGTTIN")]
    Sigttin,
    #[serde(rename = "SIGTTOU")]
    Sigttou,
    #[serde(rename = "SIGURG")]
    Sigurg,
    #[serde(rename = "SIGXCPU")]
    Sigxcpu,
    #[serde(rename = "SIGXFSZ")]
    Sigxfsz,
    #[serde(rename = "SIGVTALRM")]
    Sigvtalrm,
    #[serde(rename = "SIGPROF")]
    Sigprof,
    #[serde(rename = "SIGWINCH")]
    Sigwinch,
    #[serde(rename = "SIGIO")]
    Sigio,
    #[serde(rename = "SIGPWR")]
    Sigpwr,
    #[serde(rename = "SIGSYS")]
    Sigsys,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigConstraintScriptConfigs {
    /// Specifies the namespace for the ConfigMap.
    /// If not specified, it defaults to the "default" namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specifies the reference to the ConfigMap containing the scripts.
    #[serde(rename = "scriptConfigMapRef")]
    pub script_config_map_ref: String,
}

/// Used to match labels on the pod to determine whether a dynamic reload should be performed.
/// 
/// 
/// In some scenarios, only specific pods (e.g., primary replicas) need to undergo a dynamic reload.
/// The `selector` allows you to specify label selectors to target the desired pods for the reload process.
/// 
/// 
/// If the `selector` is not specified or is nil, all pods managed by the workload will be considered for the dynamic
/// reload.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigConstraintSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ConfigConstraintSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigConstraintSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Specifies the tools container image used by ShellTrigger for dynamic reload.
/// If the dynamic reload action is triggered by a ShellTrigger, this field is required.
/// This image must contain all necessary tools for executing the ShellTrigger scripts.
/// 
/// 
/// Usually the specified image is referenced by the init container,
/// which is then responsible for copy the tools from the image to a bin volume.
/// This ensures that the tools are available to the 'config-manager' sidecar.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigConstraintToolsImageSpec {
    /// Specifies the directory path in the container where the tools-related files are to be copied.
    /// This field is typically used with an emptyDir volume to ensure a temporary, empty directory is provided at pod creation.
    #[serde(rename = "mountPoint")]
    pub mount_point: String,
    /// Specifies a list of settings of init containers that prepare tools for dynamic reload.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "toolConfigs")]
    pub tool_configs: Option<Vec<ConfigConstraintToolsImageSpecToolConfigs>>,
}

/// ToolConfig specifies the settings of an init container that prepare tools for dynamic reload.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigConstraintToolsImageSpecToolConfigs {
    /// Indicates whether the tool image should be used as the container image for a sidecar.
    /// This is useful for large tool images, such as those for C++ tools, which may depend on
    /// numerous libraries (e.g., *.so files).
    /// 
    /// 
    /// If enabled, the tool image is deployed as a sidecar container image.
    /// 
    /// 
    /// Examples:
    /// ```text
    ///  toolsSetup::
    ///    mountPoint: /kb_tools
    ///    toolConfigs:
    ///      - name: kb-tools
    ///        asContainerImage: true
    ///        image:  apecloud/oceanbase:4.2.0.0-100010032023083021
    /// ```
    /// 
    /// 
    /// generated containers:
    /// ```text
    /// initContainers:
    ///  - name: install-config-manager-tool
    ///    image: apecloud/kubeblocks-tools:${version}
    ///    command:
    ///    - cp
    ///    - /bin/config_render
    ///    - /opt/tools
    ///    volumemounts:
    ///    - name: kb-tools
    ///      mountpath: /opt/tools
    /// 
    /// 
    /// containers:
    ///  - name: config-manager
    ///    image: apecloud/oceanbase:4.2.0.0-100010032023083021
    ///    imagePullPolicy: IfNotPresent
    /// 	  command:
    ///    - /opt/tools/reloader
    ///    - --log-level
    ///    - info
    ///    - --operator-update-enable
    ///    - --tcp
    ///    - "9901"
    ///    - --config
    ///    - /opt/config-manager/config-manager.yaml
    ///    volumemounts:
    ///    - name: kb-tools
    ///      mountpath: /opt/tools
    /// ```
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "asContainerImage")]
    pub as_container_image: Option<bool>,
    /// Specifies the command to be executed by the init container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// Specifies the tool container image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Specifies the name of the init container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// ConfigConstraintStatus represents the observed state of a ConfigConstraint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigConstraintStatus {
    /// Provides descriptions for abnormal states.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Refers to the most recent generation observed for this ConfigConstraint. This value is updated by the API Server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// Specifies the status of the configuration template.
    /// When set to CCAvailablePhase, the ConfigConstraint can be referenced by ClusterDefinition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<ConfigConstraintStatusPhase>,
}

/// ConfigConstraintStatus represents the observed state of a ConfigConstraint.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConfigConstraintStatusPhase {
    Available,
    Unavailable,
    Deleting,
}

