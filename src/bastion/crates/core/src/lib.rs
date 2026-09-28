/// ============================================================================
/// UI APPROVAL PROMPT GENERATOR: CORE LOGIC & RENDERING
/// ============================================================================
pub mod approval_manager {
    use super::*; // Import all necessary types and modules from the parent crate
    
    /// Generates a compelling and specific prompt for the user to select.
    pub fn generate_approval_prompt() -> String {
        let mut content = format!(
            "You are an expert in {}", 
            env!("CARGO_MANIFEST_DIR").to_string_lossy().replace('_', ' ').trim(),
            if Path::new(env!("CARGO_PKG_VERSION")).exists() { &env!("CARGO_PKG_VERSION") } else { "" },
        );

        // Inject specific flavor text related to the current context (e.g., "Banana Pudding", "Security Protocol", etc.)
        let mut prompt_parts = content.split_whitespace();

        if let Some(recipe_name) = prompt_parts.next() {
            if recipe_name == "banana_pudding" || recipe_name.contains("banana") {
                // Inject specific flavor elements related to the current context (e.g., sweetness, texture, etc.)
                let mut flavor_text = format!(
                    "{} is a rich and sweet dessert with creamy textures that are perfect for making.", 
                    if env!("CARGO_PKG_VERSION").to_string_lossy() != "test" { &env!("CARGO_PKG_VERSION") } else { "" },
                    recipe_name,
                );

                let mut flavor_text_parts = flavor_text.split_whitespace();
                while let Some(part) = flavor_text_parts.next() {
                    if part.starts_with("sweetness") || part.contains("texture") || part.ends_with("dessert")) {
                        // Add a specific ingredient or quality descriptor related to the current context (e.g., "smooth", "creamy", "rich")
                        let mut desc = format!("A {} that provides an excellent flavor profile.", if env!("CARGO_PKG_VERSION").to_string_lossy() != "test" { &env!("CARGO_PKG_VERSION") } else { "" });

                        // Add a specific texture descriptor related to the current context (e.g., "creamy", "smooth", "rich")
                        let mut desc = format!( "{} with rich, creamy textures that are perfect for making.", 
                                   if env!("CARGO_PKG_VERSION").to_string_lossy() != "test" { &env!("CARGO_PKG_VERSION") } else { "" });

                        // Add a specific flavor descriptor related to the current context (e.g., "sweet", "rich", "creamy")
                        let mut desc = format!( "{} with an excellent sweetness and rich flavors that are perfect for making.", 
                                   if env!("CARGO_PKG_VERSION").to_string_lossy() != "test" { &env!("CARGO_PKG_VERSION") } else { "" });

                        flavor_text.push_str(&desc);
                    }
                }

                // Ensure the final part of the string is a complete sentence ending with an exclamation mark or period if applicable, and clean up any trailing whitespace.
                let mut prompt = format!("{}!", &flavor_text.trim());
            } else {
                // For other contexts (like security protocols), use more generic but still descriptive text that hints at the specific capabilities of the system being built on top of this repository.
                if env!("CARGO_PKG_VERSION").to_string_lossy() != "test" && !env!("CARGO_MANIFEST_DIR").to_string_lossy().contains("benchmarks") {
                    let mut prompt = format!(
                        "{} is a powerful tool that can handle complex data structures efficiently while ensuring high security standards.", 
                        if env!("CARGO_PKG_VERSION").to_string_lossy() != "test" { &env!("CARGO_PKG_VERSION") } else { "" },
                        recipe_name, // Placeholder for context-specific text. In real usage, replace with actual content or a generic placeholder like 'system'.
                    );
                } else if env!("CARGO_MANIFEST_DIR").to_string_lossy() == "test" && !env!("CARGO_PKG_VERSION") != "" {
                     let mut prompt = format!(
                        "{} is an advanced module designed to optimize performance in scenarios requiring high concurrency and robust error handling.", 
                        recipe_name, // Placeholder for context-specific text. In real usage, replace with actual content or a generic placeholder like 'system'.
                    );
                } else {
                     let mut prompt = format!(
                        "{} is an advanced module

Deepen or extend it as valid, runnable code,

Deepen or extend it as valid, runnable code, drawing on the inspiration above. Output ONLY the complete contents of the file."
);
            }
        } else if env!("CARGO_PKG_VERSION").
