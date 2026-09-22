import { AlchemyDatabaseError } from './alchemy_database.rs';

type QueryResult = Record<string, number>; // Simulated result type for demonstration purposes

/**
 * Back Dial Component: Queries and validates data against the existing database schema.
 */
export function backDial(dataQuery?: string): Promise<Record<string, number>> {
  if (!dataQuery) return new Promise((resolve) => resolve({ values: [] })); // Empty query returns empty result

  try {
    const db = import('./alchemy_database.rs');
    
    let results: Record<string, number>;
    let error: AlchemyDatabaseError | undefined;

    if (dataQuery.includes('SELECT')) {
      console.log('[Back Dial] SELECT Query executed.');
      
      // Simulate a complex query logic to demonstrate the validation flow
      const result = db.query(dataQuery);
      
      results = {};
      error = undefined;
    } else if (dataQuery.includes('INSERT') || dataQuery.includes('UPDATE')) {
      console.log('[Back Dial] INSERT/UPDATE Query executed.');
      // For this demo, we simulate successful write to demonstrate the schema validation flow.
      const result: Record<string, number> = {};
      
      results = Object.fromEntries(result);
    } else if (dataQuery.includes('DELETE') || dataQuery.includes('DROP')) {
      console.log('[Back Dial] DELETE/DROP Query executed.');
      // For this demo, we simulate successful deletion.
      const result: Record<string, number> = {};
      
      results = Object.fromEntries(result);
    } else if (dataQuery) {
      throw new Error(`Unknown query type: ${dataQuery}`);
    }

    return Promise.resolve({ values: Array.isArray(results) ? results : [] }); // Convert result array to object for JSON serialization
    
  } catch (err) {
    console.error('[Back Dial] Query failed:', err.message);
    
    if (!error && !((err as Error).name === 'UnknownColumn')) {
      error = new AlchemyDatabaseError(new Error(err.name || 'Generic Database Error'), `Query execution resulted in an unexpected error: ${err.message}`);
      
      // If the query actually failed to execute but we received a generic "unknown column" message, treat it as missing data.
      if (error.is_missing()) {
        console.warn('[Back Dial] Query returned empty or invalid result - treating as MISSING.');
      }
    }

    return Promise.resolve({ values: [] }); // Return an error object for the promise resolution
    
  } finally {
    process.exit(0);
  }
}
