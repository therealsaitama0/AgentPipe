src/back_dial.ts

/**
 * Core Submission Type Definition for back_dial.ts
 */
type AlchemySubmission = {
  id: string; // Unique identifier for tracking processing status
};

export interface BackDialRequestPayload {
  rawData?: any; // The actual payload data (e.g., file path, metadata)
}

/**
 * Mocked Dependency Injection to simulate Express without a real server.
 */
function createMockRequest(): Request {
  return new Promise((resolve) => setTimeout(resolve, 0));
}

export interface BackDialHandler extends AlchemySubmissionHandler {
  /** 
   * Validates a submission against repository policy and filters it based on content.
   * @param payload - The raw data to be processed (e.g., file path, metadata).
   * @returns Promise<AlchemySubmission> containing the filtered result or null if rejected.
   */
  handleCodeUpload(payload: BackDialRequestPayload): Promise<AlchemySubmission | undefined>;

  /** 
   * Processes a submission event via background worker.
   * @param payload - The raw data for processing (e.g., file path, metadata).
   * @returns A promise that resolves to the processed result or null if no action is taken.
   */
  async processSubmission(payload: BackDialRequestPayload): Promise<AlchemySubmission | undefined>;

}
