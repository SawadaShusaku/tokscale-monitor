export interface OpenCodeMessage {
  id: string;
  session_id: string;
  model: string;
  provider: string;
  cost: number;
  created: number; // Unix timestamp in milliseconds
}

export interface OpenCodeData {
  messages: OpenCodeMessage[];
}

export interface UnifiedMessage {
  id: string;
  client: string;
  model_id: string;
  cost: number;
  timestamp: number; // Unix timestamp in milliseconds
}

export interface UnifiedData {
  messages: UnifiedMessage[];
}
