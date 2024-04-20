# Lowbull(s): Low Cost Business and UI Logic Separator
>*low* cost *b*usiness and *u*i *l*ogic *s*eperator

Lowbull(s) is a Rust library that abstracts business logic away from UI logic, enabling developers to separate and reuse these components independently across different UI frameworks or applications. This project offers a flexible and lightweight solution to manage core logic separately from user interface implementation.

## Overview

In software projects, especially those with graphical user interfaces (GUIs), cleanly separating business logic (how the application works) from UI logic (how it's presented) enhances code readability, maintainability, and flexibility.

Lowbull(s) lets you define and manage core application logic independently of any specific UI framework, making it easy to integrate with different UI libraries like egui.

## Features

- **Business Logic Abstraction**: Define and implement application's business rules using Lowbull(s).
- **UI Logic Decoupling**: Develop UI components with any framework while keeping business logic separate.
- **Flexibility**: Swap or update UI without impacting application behavior.
- **Minimal Overhead**: Lightweight and efficient design.

## Getting Started

### Prerequisites

- Rust (latest stable version recommended)
- Cargo (Rust's package manager)

### Installation

To use Lowbull(s) in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
lowbulls = "0.1.0"
```

## Usage
- **Define Your Business LogicStart** by defining your business logic, which includes structs, enums, functions, and any data manipulation or rules specific to your application's functionality.
- **Implement Business Logic with Lowbull(s)** Integrate Lowbull(s) into your business logic implementation. Use Lowbull(s) to encapsulate and manage the core functionality of your application.
- **Develop UI Components** Independently develop your UI components using a UI framework of your choice (e.g., egui).
- **Connect UI to Business Logic** Use Lowbull(s) to bridge the gap between your UI components and the underlying business logic. Implement event handling and data synchronization between the two layers.

### Example
Here's a simple example demonstrating how you might use Lowbull(s) with egui:

```rust
extern crate lowbulls;

use lowbulls::BusinessLogic;

// Define your business logic
struct Calculator {
    value: f32,
}

impl BusinessLogic for Calculator {
    fn new() -> Self {
        Calculator { value: 0.0 }
    }

    fn add(&mut self, x: f32) {
        self.value += x;
    }

    fn subtract(&mut self, x: f32) {
        self.value -= x;
    }

    fn get_result(&self) -> f32 {
        self.value
    }
}

// UI logic (using egui)
fn ui_logic(calc: &mut Calculator) {
    egui::CentralPanel::default().show(egui::Window::new("Calculator").show(ui(|ui| {
        ui.label(format!("Result: {}", calc.get_result()));

        if ui.button("Add").clicked() {
            calc.add(1.0);
        }

        if ui.button("Subtract").clicked() {
            calc.subtract(1.0);
        }
    })));
}

fn main() {
    let mut calculator = Calculator::new();

    loop {
        // Update UI with business logic
        ui_logic(&mut calculator);

        // Render UI (e.g., egui)
        // your_render_function();

        // Handle events and other application logic
        // your_event_handling_function();
    }
}
```

## Contribution
Contributions to Lowbull(s) are welcome! If you have suggestions, feature requests, or want to report a bug, please open an issue on GitHub.

## License
This project is licensed under the MIT License - see the LICENSE file for details.
