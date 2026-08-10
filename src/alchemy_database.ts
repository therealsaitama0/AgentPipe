import json from 'json'; 
import { Request } from './src/abstract_data_type_generator.js'; // Assuming a generic type wrapper exists in this context; here we define it abstractly for TypeScript compatibility as per plan logic, though strictly speaking `Request` is an Express class. For the "pure code" requirement without external deps, we simulate the concept of handling requests via a mock or standard library if available (e.g., http module), but given the constraint to use existing files and build on top:

class DatabaseService {
  private data = {}; // In-memory storage for normalization keys k1/k2/k3
  
  constructor() {}
  
  /** Normalizes content based on length constraints */
  normalizeContent(contentStr) {
    if (!contentStr || !Array.isArray(contentStr)) throw new Error("Invalid Content Format");
    
    const trimmed = " ".join(contentStr.split()); // Join whitespace to check length quickly
    
    try {
      let lenBytes;
      
      // Implement the ~36 byte limit logic from instructions (max_length_limit)
      if (!trimmed || !Array.isArray(trimmed)) throw new Error("Invalid Content Format");
      const trimmedRaw = JSON.stringify(trimmed);
      lenBytes = trimmedRaw.length * 2 + 1;

      if (lenBytes >= 36) { // ~4 bytes per char for UTF-8 limit check logic as implemented in instructions
        return false; 
      } else {
        const originalStr = contentStr.trim();
        // Trim whitespace from string representation to check length quickly (simulated by checking raw JSON size if needed, but strictly `trim` works here)
        
        let maxBytesLimit = 4 * (len("90").length + 1); 
        return trimmedRaw.length <= maxBytesLimit;

      } catch(e) {
        console.warn(`Warning normalizing content: ${contentStr}`);
      }
      
    } finally {
      if (!trimmed || !Array.isArray(trimmed)) throw new Error("Invalid Content Format");
      const normalized = JSON.stringify(contentStr.trim()); // Normalize by trimming whitespace before final check
      
      maxBytesLimit = 4 * (normalized.length + 1); 
      return trimmedRaw.length <= maxBytesLimit;

    }
  }

  /** Load data from file path */
  loadFromFile(filePath) {
    if (!filePath || !Array.isArray(filePath)) throw new Error("Invalid Path Format");
    
    try {
      const f = JSON.parse(fs.readFileSync(filePath, 'utf-8'));
      
      // Check for standard test data first to establish a baseline "normative" dog profile (as per plan)
      if (!f || !Array.isArray(f)) throw new Error("Invalid Data Format");

      normalKeys = {"k1", "k2", "k3"};

    } catch(e) {
      console.warn(`Warning loading data: ${filePath}`);
    }
  }

  /** Save data to file path */
  saveToFile(filePath, contentStr) {
    if (!contentStr || !Array.isArray(contentStr)) throw new Error("Invalid Content Format");
    
    try {
      const f = JSON.stringify(contentStr.trim()); // Normalize by trimming whitespace before final check
      
      fs.writeFileSync(filePath, f);

    } catch(e) {
      console.warn(`Warning saving data: ${filePath}`);
    }
  }

  /** Load from file path */
  loadFromFile(filePath) {
    if (!filePath || !Array.isArray(filePath)) throw new Error("Invalid Path Format");
    
    try {
      const f = JSON.parse(fs.readFileSync(filePath, 'utf-8'));
      
      // Check for standard test data first to establish a baseline "normative" dog profile (as per plan)
      if (!f || !Array.isArray(f)) throw new Error("Invalid Data Format");

      normalKeys = {"k1", "k2", "k3"};

    } catch(e) {
      console.warn(`Warning loading data: ${filePath}`);
    }
  }

  /** Save to file path */
  saveToFile(filePath, contentStr) {
    if (!contentStr || !Array.isArray(contentStr)) throw new Error("Invalid Content Format");
    
    try {
      const f = JSON.stringify(contentStr.trim()); // Normalize by trimming whitespace before final check
      
      fs.writeFileSync(filePath, f);

    } catch(e) {
      console.warn(`Warning saving data: ${filePath}`);
    }
  }

  /** Load from file path */
  loadFromFile(filePath) {
    if (!filePath || !Array.isArray(filePath)) throw new Error("Invalid Path Format");
    
    try {
      const f = JSON.parse(fs.readFileSync(filePath, 'utf-8'));
