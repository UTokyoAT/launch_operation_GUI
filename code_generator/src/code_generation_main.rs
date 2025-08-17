use crate::code_generation_context::CodeGenerationContext;
use crate::code_generator;
use crate::config_parser;
use crate::template_parser::{self, TypeToString};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

/// マニフェストディレクトリを取得するヘルパー関数
fn get_manifest_dir() -> PathBuf {
    std::env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .unwrap()
}

/// プロジェクトルートからの相対パスを絶対パスに変換
fn resolve_project_path<P: AsRef<Path>>(relative_path: P) -> PathBuf {
    get_manifest_dir().join(relative_path)
}

fn sub_dir(path: &Path) -> Vec<String> {
    let mut sub_dirs = Vec::new();
    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_str().unwrap().to_string();
            sub_dirs.push(dir_name);
        }
    }
    sub_dirs
}

pub fn generate(config_path: Box<Path>, output_path: Box<Path>) {
    let template_path = resolve_project_path("config/template");
    //config/template内にあるディレクトリを列挙
    let languages = sub_dir(&template_path);
    for language in languages {
        let language_config_path = template_path.join(&language);
        let language_output_path = output_path.join(&language);
        let type_to_string =
            TypeToString::read_json(language_config_path.join("types.json").into_boxed_path());
        let template_path = language_config_path.join("template.txt");
        let filename_extension =
            fs::read_to_string(language_config_path.join("file_name_extension.txt"))
                .expect("Unable to read file");
        let template = template_parser::read_template(template_path.into_boxed_path());
        let config = fs::read_to_string(config_path.clone()).expect("Unable to read file");
        let config = config_parser::parse_config(&config);
        fs::create_dir_all(&language_output_path).expect("Unable to create directory");
        for data_definition in config {
            let output_path =
                language_output_path.join(data_definition.name.clone() + "." + &filename_extension);
            let code_generation_context = CodeGenerationContext::new(data_definition);
            let type_to_string = type_to_string.clone().to_fn();
            let result = code_generator::render_template(
                &template,
                Box::new(type_to_string),
                code_generation_context,
            );
            fs::write(output_path, result).expect("Unable to write file");
        }
    }
}
