use dialoguer::Input;
use std::env;
use std::fs;

fn main() {
    // Prompt the user to enter the project name
    let project_name: String = Input::new()
        .with_prompt("Enter the project name")
        .interact()
        .expect("Failed to get project name");

    // Call the function to set up project directories and files
    setup_project_directories_and_files(&project_name);
}

// Function to set up project directories and files
fn setup_project_directories_and_files(project_name: &str) {
    // Get the current directory and join it with the project name
    let angular_app_path = env::current_dir()
        .unwrap()
        .join(&project_name)
        .to_string_lossy()
        .to_string();

    // Create necessary folders: Service, Models, components, styles, src, public
    create_directories(
        &angular_app_path,
        &[
            "src/app",
            "src/assets",
            "src",
            "src/components",
            "src/service",
            "src/models",
        
        ],
    );

    // Create separate folders for each component inside src/components
    create_directories(
        &angular_app_path,
        &[
        
            "src/components/PlaceholderComponent1",
            "src/components/PlaceholderComponent2",
        ],
    );

    // Read and write PlaceholderComponent1 content
    copy_template_file(
        "src/PlaceholderComponent1.ts.txt",
        &angular_app_path,
        "src/components/PlaceholderComponent1/PlaceholderComponent1.ts",
    );
    copy_template_file(
        "src/PlaceholderComponent1.html.txt",
        &angular_app_path,
        "src/components/PlaceholderComponent1/PlaceholderComponent1.html",
    );
    generate_empty_scss_file(
        &angular_app_path,
        "src/components/PlaceholderComponent1/PlaceholderComponent1.scss",
    );

    // Read and write PlaceholderComponent2 content
    copy_template_file(
        "src/PlaceholderComponent2.ts.txt",
        &angular_app_path,
        "src/components/PlaceholderComponent2/PlaceholderComponent2.ts",
    );
    copy_template_file(
        "src/PlaceholderComponent2.html.txt",
        &angular_app_path,
        "src/components/PlaceholderComponent2/PlaceholderComponent2.html",
    );
    generate_empty_scss_file(
        &angular_app_path,
        "src/components/PlaceholderComponent2/PlaceholderComponent2.scss",
    );

    // Read and write BackendService boilerplate content
    copy_template_file(
        "src/backend_service_boilerplate.txt",
        &angular_app_path,
        "src/Service/Service.ts",
    );

    // Read and write PlaceholderModel content
    copy_template_file(
        "src/models.txt",
        &angular_app_path,
        "src/Models/PlaceholderModel.ts",
    );

    // Read and write app.component.html content
    copy_template_file(
        "src/app.component.html.txt",
        &angular_app_path,
        "src/app/app.component.html",
    );

    // Read and write app.component.scss content
    copy_template_file(
        "src/app.component.scss.txt",
        &angular_app_path,
        "src/app/app.component.scss",
    );

    // Read and write app.component.ts content
    copy_template_file(
        "src/app.component.ts.txt",
        &angular_app_path,
        "src/app/app.component.ts",
    );

    // Read and write app.module.ts content
    copy_template_file(
        "src/app.module.ts.txt",
        &angular_app_path,
        "src/app/app.module.ts",
    );

    // Read and write app-routing.module.ts content
    copy_template_file(
        "src/app-routing.module.ts.txt",
        &angular_app_path,
        "src/app/app-routing.module.ts",
    );

    // Read and write main.ts content
    copy_template_file("src/main.ts.txt", &angular_app_path, "src/main.ts");

    // Read and write index.html content
    copy_template_file("src/index.html.txt", &angular_app_path, "src/index.html");

    // Read and write styles.scss content
    copy_template_file("src/styles.scss.txt", &angular_app_path, "src/styles.scss");

    // Write development and production environment files
    create_directories(&angular_app_path, &["enviroment"]);  // Create enviroment directory
    fs::write(
        format!("{}/enviroment/env.development", &angular_app_path,),
        "REACT_APP_ENV=development",
    )
    .expect("Failed to create .env.development file");
    fs::write(
        format!("{}/enviroment/env.production", &angular_app_path,),
        "REACT_APP_ENV=production",
    )
    .expect("Failed to create .env.production file");
    

    // Read and write tsconfig.json template content
    copy_template_file(
        "src/tsconfig_json_template.txt",
        &angular_app_path,
        "tsconfig.json",
    );
    copy_template_file(
        "src/tsconfig.app.json.txt",
        &angular_app_path,
        "tsconfig.app.json",
    );
    copy_template_file(
        "src/tsconfig.spec.json.txt",
        &angular_app_path,
        " tsconfig.spec.json",
    );
    copy_template_file("src/readme_template.txt", &angular_app_path, "README.md");
    copy_template_file("src/eslint_template.txt", &angular_app_path, ".eslint.json");
    copy_template_file(
        "src/angular_json_template.txt",
        &angular_app_path,
        "angular.json",
    );

    // Read and write .gitignore template content
    copy_template_file(
        "src/gitignore_template.txt",
        &angular_app_path,
        ".gitignore",
    );

    // Read and write package.json template content
    copy_template_file(
        "src/package_json_template.txt",
        &angular_app_path,
        "package.json",
    );

    println!("Project setup completed successfully!");
}

// Function to create directories based on the provided base path and directory names
fn create_directories(base_path: &str, directories: &[&str]) {
    for dir in directories {
        fs::create_dir_all(format!("{}/{}", base_path, dir))
            .expect(&format!("Failed to create {} directory", dir));
    }
}

// Function to read a template file and write its content to another file
fn copy_template_file(template_file: &str, base_path: &str, target_file: &str) {
    let content =
        fs::read_to_string(template_file).expect(&format!("Failed to read {}", template_file));
    fs::write(format!("{}/{}", base_path, target_file), content)
        .expect(&format!("Failed to add {}", target_file));
}

// Function to generate an empty SCSS file
fn generate_empty_scss_file(base_path: &str, target_file: &str) {
    fs::write(format!("{}/{}", base_path, target_file), "")
        .expect(&format!("Failed to generate {}", target_file));
}
