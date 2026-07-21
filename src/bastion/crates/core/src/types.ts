src/bastion/crates/core/src/types.ts
// ============================================================================
// Core Types Definition for Alchemy Submission & Processing Workflow
// ============================================================================

/**
 * Represents a raw file upload payload sent to backend validation logic.
 */
export interface UploadedFile {
  /** The unique identifier of the uploaded file (e.g., filename). */
  name: string;
  
  // Optional metadata describing the content type or source context if needed by downstream modules.
  contentType?: string | null; 
}

/**
 * Represents a processed submission that has passed validation and is ready for consumption.
 */
export interface AlchemySubmission {
  /** Unique identifier for tracking processing status (e.g., 'pending', 'processing', 'completed'). */
  id: string; 
  
  // Optional metadata describing the content or source context if needed by downstream modules.
  contentId?: string | null; 

  // Metadata from LLM response or user input, which may be optional depending on current logic flow.
  metadata?: Record<string, unknown>; 
}

/**
 * Represents a processed result that has passed validation and is ready for consumption.
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

}
