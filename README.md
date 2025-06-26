# Todo GTK Application

This is a simple Todo application built with Rust and GTK. It allows users to manage their tasks in a graphical user interface.

## How to Run

To run this application, you need to have Rust and GTK development libraries installed on your system.

1.  **Clone the repository (if you haven't already):**
    ```bash
    git clone https://github.com/your-repo/todo-gtk.git
    cd todo-gtk
    ```
    *(Note: Replace `https://github.com/your-repo/todo-gtk.git` with the actual repository URL if this project is hosted.)*

2.  **Install GTK Development Libraries:**
    The installation steps for GTK development libraries vary depending on your operating system.

    *   **Debian/Ubuntu:**
        ```bash
        sudo apt update
        sudo apt install libgtk-4-dev
        ```
    *   **Fedora:**
        ```bash
        sudo dnf install gtk4-devel
        ```
    *   **Arch Linux:**
        ```bash
        sudo pacman -S gtk4
        ```
    *   **macOS (using Homebrew):**
        ```bash
        brew install gtk4
        ```
    *   **Windows:**
        Refer to the official GTK documentation for setting up GTK on Windows, typically involving MSYS2.

3.  **Build and Run the Application:**
    Navigate to the project's root directory and use Cargo to build and run the application:
    ```bash
    cargo run
    ```

    This command will compile the project and then execute the application.
