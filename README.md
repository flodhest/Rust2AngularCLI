# Rust2AngularCLI

Automate Angular project setup with Rust!

<details>
  <summary>🌲 **Project Structure Tree**</summary>
<pre>
project-name
├── src
│   ├── app
│   │   ├── components
│   │   │   ├── PlaceholderComponent1
│   │   │   │   ├── PlaceholderComponent1.html
│   │   │   │   ├── PlaceholderComponent1.scss
│   │   │   │   └── PlaceholderComponent1.ts
│   │   │   └── PlaceholderComponent2
│   │   │       ├── PlaceholderComponent2.html
│   │   │       ├── PlaceholderComponent2.scss
│   │   │       └── PlaceholderComponent2.ts
│   │   ├── models
│   │   │   └── PlaceholderModel.ts
│   │   ├── service
│   │   │   └── Service.ts
│   │   ├── app.component.html
│   │   ├── app.component.scss
│   │   ├── app.component.ts
│   │   ├── app.module.ts
│   │   ├── app-routing.module.ts
│   │   └── main.ts
│   ├── assets
│   ├── environments
│   │   ├── env.development
│   │   └── env.production
│   ├── index.html
│   ├── ngsw-config.json
│   ├── styles.scss
│   └── README.md
├── .eslintrc.json
├── .gitignore
├── angular.json
├── package.json
├── README.md
├── tsconfig.base.json
└── tsconfig.json
</pre>
</details>

**Angular CLI Rust Setup** is a handy Rust CLI tool that streamlines Angular project creation, adding powerful features effortlessly.

## Features

- **Angular Material Integration:** Enhance your UI with Material Design components and icons.

- **PWA Service Worker Configuration:** Enable PWA features with service worker support for offline caching and push notifications.

- **RxJS Integration:** The project incorporates RxJS for reactive programming, enabling efficient handling of asynchronous tasks. Leverage the power of observables and operators to manage data flow and events in your Angular application.

- **Browser Routing:** Implement smooth navigation between pages with browser routing.

- **Unit Testing Configuration:** Easily configure unit testing using Karma and Jasmine.

- **Linting and Code Formatting:** Ensure code consistency with ESLint and Prettier integration.

- **Environment Configuration:** Customize environment variables for different environments (dev, staging, production).

- **Ahead-of-Time Compilation (AOT):** Improve Angular app performance and security with AOT compilation.

## 🛠 Prerequisites

Before using Rust2ReactCLI, ensure that you have Node.js version >=16.20.2 installed on your machine. [Download Node.js](https://nodejs.org/dist/latest-v16.x/)

## 🏁 Getting Started

1. **Clone the Rust2AngularCLI repository to your local machine:**
   ```bash
   git clone https://github.com/flodhest/Rust2AngularCLI.git
   cd Rust2AngularCLI
   ```

2. **Ensure Rust is Installed:**
   Before proceeding, make sure you have Rust installed on your machine. If not, you can install Rust by running:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   Follow the instructions to complete the installation.

3. **Build and Run Rust2AngularCLI:**
   ```bash
   cargo run
   ```
   Follow the prompts to enter the project name; the project will be generated in `Rust2AngularCLI/src`. Run the terminal from the new project's folder.

4. **Project Setup:**
   Once completed, your Angular app project will be set up with the specified structure and files.

5. **Navigate to Your Angular App:**
   ```bash
   cd your_project_name
   ```

6. **Install Node.js Dependencies:**
   ```bash
   npm i -g yarn
   yarn install
   ```
   This command installs the necessary Node.js dependencies for your Angular app.

7. **Run Your Angular App:**
   ```bash
   yarn start
   ```

## 🚧 Additional Resources

- [Node.js Installation](https://nodejs.org/dist/latest-v16.x/): Download and install Node.js version >16.20.2 manually if needed.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
