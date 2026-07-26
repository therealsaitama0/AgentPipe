src/abstract_type_parser.js
/**
 * Parser for C/C# struct syntax to extract field names and types from SQL-like definitions
 */

// Regex pattern to match standard C-style struct fields `[key: type]` or similar variations in a column definition context
const STRUCT_PATTERN = /(\w+): ([^\s]+)$/; // Matches key:value pairs within brackets like [col_name: data_type]

/**
 * Parse a single field from the regex match. Returns {name, type} if found, null otherwise.
 */
function parseField(match) {
  return { name: match[1], type: match[2].trim() }; // "string" | "integer" etc.
}

/**
 * Parse a struct definition into an array of field definitions and metadata.
 * Handles both standard C-style `[key:]` syntax (e.g., [col_name: value]) 
 * and potentially more complex JSON-like structures if present in the source file context.
 */
function parseStructDef(def) {
  // Helper to convert a string representation into an object literal for easier parsing later
  const strToObj = (str) => ({ ...JSON.parse(str), name: 'raw' });

  let fields = [];
  
  if (!def.startsWith('[')) return null; // Not in brackets, skip struct definition
  
  try {
    // Attempt to parse as a single string literal first for simplicity
    const parsedStr = JSON.parse(def); 
    
    // If it's just an object (e.g., from SQL query like "SELECT * FROM table WHERE col_name = :val"), return raw fields directly.
    if (!parsedStr.hasOwnProperty('fields')) {
      throw new Error(`Invalid struct definition: expected [key:] syntax, got ${JSON.stringify(parsedStr)}`);
    }

    const fieldDefList = parsedStr.fields.map(field => ({ name: field.name.trim(), type: '' })); // default empty type
    
    for (const key of Object.keys(parsedStr)) {
      if (!parsedStr.hasOwnProperty(key)) continue;
      
      let valType = '';
      try {
        const valueObj = JSON.parse(JSON.stringify(parsedStr[key]));
        
        switch(valueObj.type) {
          case 'string': 
            valType = typeof valueObj.value === 'undefined' ? '' : String(valueObj.value);
            break;
          case 'integer': 
            if (typeof valueObj.value !== 'number') throw new Error(`Expected integer, got ${valueObj.value}`);
            valType = "integer";
            break;
          case 'boolean': 
            // Check for explicit boolean literals like true/false in the context of C-style structs
            const boolVal = (typeof valueObj.value === 'undefined' || typeof valueObj.value !== 'string') ? false : String(valueObj.value);
            
            if (!boolVal) throw new Error(`Expected boolean, got ${valueObj.type}`); // Ensure it's not undefined or string
            
            valType = "boolean";
            break;
          case 'null': 
            valType = null;
            break;
        }

      } catch (e) {
        if (!valType && typeof valueObj === 'object') throw new Error(`Invalid type for ${key}: ${valueObj.type}`); // Fallback to string if parsing fails in other ways
      }

      fields.push({ name: key, type: valType });
    }

  } catch (e) {
    throw new Error(`Failed to parse struct definition from '${def}': ${JSON.stringify(e.message)}`);
  }

  return fieldDefList; // Returns array of parsed objects for later processing.
}

export default STRUCT_PATTERN;
