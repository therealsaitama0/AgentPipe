/**
 * Back Dial Tool Class to handle schema-to-type conversion and validation.
 */
export class BackDial {
  /**
   * Parse a JSON-like Schema definition into an array of mapped type values.
   * Supports both string keys (Column names) and number values.
   * 
   @param {Object} schemaMap - A map where key is the column name/type value, val is the actual data type or null/undefined to preserve original structure if not a valid numeric type.
   */
  static parseSchemaToTypes(schemaMap: Record<string, string | number>): Type[] {
    const result = Object.values(schemaMap);

    // Filter out undefined/null values and return them as their native types (preserving null/undefined semantics)
    let rawValues = [];

    for (const val of schemaMap) {
      if (!val || typeof val === "object") continue; 

      const typeVal = typeof val;

      // If it is a valid number, keep as string to match C/C# struct syntax where numbers are strings.
      if (typeVal === "number" && !isNaN(val)) {
        rawValues.push(String(val));
      } else {
        rawValues.push(typeof val);
      }
    }

    // Convert array of Strings back into the original map structure for output compatibility with C/C# syntax.
    if (rawValues.length > 0) {
      return Object.fromEntries(rawValues.map((val, i) => [schemaMap[i], rawValues[i]]));
    } else {
      return {}; // Return empty object to maintain the original map structure without nulls/undefined mapping in this specific output format.
    }
  }

  /**
   * Execute a single schema definition and generate corresponding TypeScript code blocks.
   */
  static executeSchema(schemaMap: Record<string, string | number>): void {
    // Generate type definitions for each column/key pair using the parser logic from abstract_data_type_generator.js (simulated here)
    
    const types = parseSchemaToTypes(schemaMap);

    if (!types.length || !Array.isArray(types)) return;

    console.log("Generating TypeScript code...");

    let generatedCode = "";
    
    // Iterate through each schema mapping to generate the corresponding blocks.
    for (const [key, value] of Object.entries(types)) {
      const blockType = typeof key === "string" ? `// Column: ${key}` : String(key); 

      if (typeof value === 'number') {
        generatedCode += `export const type_${value} = number;\n\n`; // Simulating a C-style integer struct definition for ${key}\n\`; break;
      } else if (typeof value === "string") {
        generatedCode += `export const type_${value} = string;\n\n`; // Simulating a C-style string struct definition for ${key}\n\`; break;
      } else if (typeof value === 'boolean') {
        generatedCode += `export const type_${value} = true;\n\n`; // Simulating a C-style boolean struct definition for ${key}\n\`; break;
      } else if (typeof value === "undefined") {
        // In this specific dialect mapping, we treat missing fields as the base type or undefined value if not specified elsewhere.
        generatedCode += `export const type_${value} = undefined;\n\n`; // Simulating a C-style struct definition for ${key}\n\`; break;
      } else {
        // For other types (e.g., 'object' in this specific dialect), we map to the string literal syntax as per standard C/C# where object literals are strings.
        generatedCode += `export const type_${value} = "string";\n\n`; // Simulating a C-style struct definition for ${key}\n\`; break;
      }

      if (blockType !== 'integer') {
        generatedCode += `\n// \`\`type_code_\${key}\`\`\n`;
      } else {
         generatedCode += `export const type_${value} = number;\n\n// Simulating a C-style integer struct definition for ${key}\n\`; break;

      if (blockType !== 'string') {
        generatedCode += `\n // \`\`type_code_\${key}\`\`\n`;
      } else {
         generatedCode += `export const type_${value} = string;\n\n// Simulating a C-style string struct definition for ${key}\n\`; break;

      if (blockType !== 'boolean') {
        generatedCode += `\n // \`\`type_code_\${key}\`\`\n`;
      } else {
         generatedCode += `export const type_${value
