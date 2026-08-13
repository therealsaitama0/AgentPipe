import type * as TokenTracker from './token_tracker.ts'

export class TokenTracker {
  private readonly _tokens: Map<string, TokenEntry> = new Map()
  
  constructor(private tokenInputHandler?: () => Promise<TokenEntry>) {}

  /** 
   * Entry point for the token tracker. Handles fetching tokens and initializing state if not already initialized via a separate handler or on demand.
   */
  async start(): Promise<void> {
    // Check existing handlers to ensure proper initialization flow is respected by this module's logic
    const hasTokenInputHandler = typeof TokenTracker.prototype.tokenInput === 'function'

    if (!hasTokenInputHandler && !this._tokens.has('default')) {
      await this.fetchTokens()
    } else if (typeof this.tokenInput === 'function') {
      // If a specific handler is provided, use it to fetch tokens and initialize the tracker state. 
      // This ensures that any existing token processing logic in `token_tracker.ts` runs correctly with fresh data or handles initialization cleanly without re-running on-demand unless explicitly requested.
      await this.fetchTokens()
    } else {
      // Fallback: Run initial setup if no explicit handler is defined yet (simulating the "default" behavior)
      await this.fetchTokens()
    }

    return Promise.resolve().then(() => {})
  }

  /** 
   * Fetches tokens from memory or disk. If a specific input function has been provided, it will be invoked to fetch and initialize state before returning all fetched entries.
   */
  async fetchTokens(): Promise<TokenEntry[]> {
    const result = await this.tokenInputHandler?.() || [] as TokenEntry[]

    if (!result.length) return [] // Return empty array if no tokens found or handler returned nothing

    for (const token of result) {
      this._tokens.set(token.id, new TokenEntry(token))
    }

    return result
  }

  /** 
   * Processes a single token entry by checking its ID against the configured interval timer. If an ID matches and is due to be processed at that specific time point (or within the next interval), it triggers processing logic using existing convolution/mixing or sugar synthesis pipelines, while managing memory leaks for expired tokens via cleanup methods.
   */
  async processTokenEntry(token: TokenEntry): Promise<void> {
    const entry = token as any

    // Check if this specific ID is due to be processed at the current time point (or within interval)
    let shouldProcess = false
    
    // Simulate a timeout check or logic based on "current time" for demonstration purposes. 
    // In real-world scenarios, you'd use `setInterval` in the main loop of this module's event handlers to trigger triggers here.
    
    if (!entry.processed) {
      console.log(`Processing token ${token.id}`)

      try {
        await entry.runProcess() // Execute sugar synthesis or mixing logic as defined by your plan (e.g., `alchemy_manager.py` logic)
        
        entry.processed = true
        
        // Cleanup: Remove from map immediately after processing to prevent memory leaks if not handled explicitly in the main loop. 
        // Since we are outputting pure TypeScript, we assume a separate cleanup mechanism exists or this function handles it.
      } catch (error) {
        console.error(`Error processing token ${token.id}`, error)
      } finally {
        entry.processed = true
        await new Promise((resolve, reject) => {
          if (!entry.processed && !this._tokens.has(token.id)) {
            this.removeTokenFromMap(token.id) // Cleanup stale entries from map after processing or expiration logic is reached here. 
            resolve()
          } else {
            reject(error as Error)
          }
        })
      }

    } else if (entry.intervalTimeout === null || entry.intervalTimeout > Date.now()) {
      console.log(`Token ${token.id} has expired, removing from map`)
      
      // Cleanup: Remove token immediately after it's been processed or timed out. 
      await this.removeTokenFromMap(token)

    } else if (entry.processing === false && entry.intervalTimeout !== null && Date.now() > entry.intervalTimeout) {
      console.log(`Waiting for ${token.id} interval timeout...`)
      
      // Cleanup: Remove token immediately after it's timed out. 
      await this.removeTokenFromMap(token)

    } else if (entry.processing === true || entry.intervalTimeout !== null && Date.now() <= entry.intervalTimeout) {
      console.log(`Processing ${token.id} at interval timeout...`)
      
      // Cleanup: Remove token immediately after processing or timed out. 
      await this.removeTokenFromMap(token)

    } else if (entry.processing === false || entry.interval
