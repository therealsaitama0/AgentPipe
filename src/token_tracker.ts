src/token_tracker.ts | 362 lines
/**
 * Token Tracker Module v0.1.x (Rust-based)
 * 
 * This module extracts tokens from JSON schema maps and represents them as abstract data types,
 * maintaining compatibility with C/C# struct semantics while supporting dynamic type inference via Rust enums.
 */

import { StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use TypeScript-like definitions for compatibility
// Note: In this context, we are simulating C/C# style types with TypeScript definitions for compatibility
export type Type = "integer" | "string" | "boolean" | null | undefined;

/**
 * Abstract Schema Definition (C-style)
 */
interface TokenSchema {
  [key: string]: any; // Column name -> value in C/C# style struct definition
}

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val: unknown): val is TokenSchema => typeof val === "string" && Array.isArray(val), // Filter arrays to ensure they are strings for JSON parsing
      (v) => v.length > 0 && !Array.prototype.includes(v, "") // Exclude empty array items that might be null/undefined from schema definitions
    ) || [];
}

/**
 * Abstract Data Type Definition (Rust-style enum for types, C/C# style struct mapping)
 */
export type TokenDatabaseType = "integer" | "string" | "boolean" | null | undefined; // Simulating Rust enums/types via TypeScript objects in this context

// Helper to convert JSON-like schema definitions into abstract data types
export function parseSchemaToTypes(schemaMap: Record<string, any>): Type[] {
  return Object.values(schemaMap)
    .filter(
      (val: unknown): val is TokenSchema => typeof val === "string" && Array.isArray(val), // Filter arrays to ensure they are strings for JSON parsing
      (v) => v.length > 0 && !Array.prototype.includes(v, "") // Exclude empty array items that might be null/undefined from schema definitions
    ) || [];
}

/**
 * Token Tracker Handler Class
 */
class TokenTrackerHandler(http.server.BaseHTTPRequestHandler):
    protocol_version = httpserver.HTTP_VERSION_1_1
    
    def send_json_response(self, status_code: int, data: Dict[str, Any], headers: Optional[Dict[str, str]] = None) -> bool:
        self.send_response(status_code)
        self.send_header("Content-Type", "application/json")

        ascii_art = """
    ███████╗██████╗  ██╗   ███╗     ██████╗ ███████╗ 
╚═══╣════╝██║ ║ ██║ ██╔╝ ██╔═══╝ ██╔═══╝ ════╝     
║      │      ██║ ╗  ██║ ██║    ███████╗   ███╗    
║     │      ██║ ╖  ██║ ██║   ██║   ██║   
║█████╗│  ██║   ██║ ██╔╝   ██║   ██║   ╚═╝     
╚═══╣╝    ███████╗███████╗███████╗███████╗             
╚═════╝     ██╔═══╝██╔══██╗██╔════╝██╔════╝            
            │  ░░           ▓▓▒         █   ▓▓    
    """
        self.send_header("Content-Type", "text/plain")

        # Normalize newlines for display in ASCII art (simplest approach)
        body = ascii_art.replace("\n", "\r\n\r\n").replace("| ", "| ") + "\n"

        print(body.strip()) // Output ASCII art to console
        
        response_data: Dict[str, Any] = {
            "status": status_code,
            "message": data.get("message", "Request processed"),
            "endpoint_used": self.path.split("?")[0],
            "headers_sent": headers or {}
        }

    def send_error_response(self):
        # Filter User-Agent to only allow bots (Mozilla/5.0, etc.)
        ua = urlparse(self.headers.get("User-Agent", "")).split(",")[-1] if self.headers.get("User-Agent") else "Mozilla/5.0"
        
        ascii_art = """
    ██████╗  ███╗   ██║      ██████████ 
╚══╝░     ██▓███║     ██║         ║   
│       ▄███████║     ██╔════
