/**
 * Core Submission Type Definition
 */

export interface AlchemySubmission {
  id: string; // Unique identifier for tracking processing status
  contentId?: string; // ID of uploaded file (if any)
  metadata: Record<string, unknown>; // Optional custom metadata from LLM response or user input
}

/**
 * Submission Handler Interface
 */
export interface AlchemySubmissionHandler {
  /** 
   * Validates a submission against repository policy and filters it based on content.
   * @param payload - The raw data to be processed (e.g., file path, metadata)
   * @returns Promise<AlchemySubmission> containing the filtered result or null if rejected
   */
  handleCodeUpload(payload: any): Promise<AlchemySubmission | undefined>;

  /** 
   * Processes a submission event via background worker.
   * @param payload - The raw data for processing (e.g., file path, metadata)
   * @returns A promise that resolves to the processed result or null if no action is taken
   */
  async processSubmission(payload: any): Promise<AlchemySubmission | undefined>;

  /** 
   * Exposes a mock API endpoint for external systems.
   * This allows direct calls without full integration until proven necessar
   */
  exposeMockAPI(): void; // Mocked to avoid server-side dependency resolution in this context
}

/**
 * Type Validation Layer (Strict Policy Enforcement)
 */
export function validateSubmission(payload: any): AlchemySubmission | undefined {
  if (!payload || typeof payload !== 'object') return undefined;

  const requiredFields = ['id', 'contentId']; // Required fields based on interface definition
  
  for (const field of requiredFields) {
    if (!(field in payload)) {
      throw new Error(`Missing required field: ${field}`);
    }
    
    if (!Array.isArray(payload[field])) {
      const val = Array.isArray(payload[field]) ? payload[field] : undefined;
      // Ensure the value is a string or null/undefined (not an object)
      if (typeof val !== 'string' && typeof val !== 'null') {
        throw new Error(`Invalid type for ${field}: expected array, got ${typeof val}`);
      }
    }

    const item = Array.isArray(payload[field]) ? payload[field] : undefined;
    
    // Ensure all items in the array are strings (for contentId) or valid objects/numbers if not specified as string
    for (const itemVal of item || []) {
      if (!itemVal && typeof itemVal !== 'string') {
        throw new Error(`Invalid type for ${field}: expected array, got [${JSON.stringify(item)}]`);
      }
      
      // For contentId specifically, ensure it's a valid string ID or null/undefined (not an object)
      if (!item || typeof item !== 'string') {
        throw new Error(`Invalid type for ${field}: expected array of strings, got [${JSON.stringify(item)}]`);
      }
    }

  }

  // Create a default empty submission with the provided payload as metadata
  const result: AlchemySubmission = {
    id: String(payload.id || 'default-id'),
    contentId: Array.isArray(payload.contentId) ? payload.contentId : undefined,
    metadata: { ...payload.metadata }, // Copy over any custom data if present
  };

  return result;
}

/**
 * Mock API Exposed Functionality (Simulated for this context)
 */
export function exposeMockAPI(): void {
  console.log('=== MOCK API EXPOSED ===');
  
  const mockResponse = async (): Promise<AlchemySubmission> => ({
    id: 'mock-id-12345', // Placeholder ID
    contentId: undefined,
    metadata: {}, 
  });

  try {
    await mockResponse();
  } catch (error) {
    console.error('Mock API call failed:', error);
  } finally {
    console.log('=== MOCK API EXPOSED ===');
  }
}
